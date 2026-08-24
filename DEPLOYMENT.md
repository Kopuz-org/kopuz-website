# Homeserver deployment

Pushes to `master` run the Rust checks, connect a GitHub-hosted runner to the tailnet, synchronize the source tree, build natively on the homeserver, and restart the systemd service. A failed health check restores the previous release.

The deployment lives under `/home/temidaradev/Projects/kopuz-website`:

```text
kopuz-website/
├── current -> releases/<git-sha>
├── releases/
├── shared/
│   ├── kopuz-website.env
│   └── sponsors_state.json
└── source/
```

## 1. Prepare the homeserver

Run the setup as `temidaradev`. The server needs `git`, `rsync`, `curl`, `flock` from `util-linux`, a C toolchain, Rust, and `cargo-leptos` 0.3.6. Install the OS packages using the server's package manager, then run:

```bash
rustup toolchain install 1.96.0 --profile minimal \
  --target wasm32-unknown-unknown
cargo +1.96.0 install --version 0.3.6 --locked cargo-leptos

install -d -m 0755 /home/temidaradev/Projects/kopuz-website
git clone https://github.com/Kopuz-org/kopuz-website.git \
  /home/temidaradev/Projects/kopuz-website/source
```

Seed `current` with the running deployment so the first automated release can roll back:

```bash
deploy_root=/home/temidaradev/Projects/kopuz-website
legacy="$deploy_root/releases/legacy"
install -d -m 0755 "$legacy"
install -d -m 0700 "$deploy_root/shared"
install -m 0755 \
  /mnt/musics/kopuz-website/target/release/kopuz-website \
  "$legacy/kopuz-website"
cp -a /mnt/musics/kopuz-website/target/site "$legacy/site"
printf 'legacy\n' > "$legacy/REVISION"
ln -s releases/legacy "$deploy_root/current.new"
mv -Tf "$deploy_root/current.new" "$deploy_root/current"
```

Perform the state cutover and install the new unit. This briefly restarts the existing binary from the SSD-backed path:

```bash
deploy_root=/home/temidaradev/Projects/kopuz-website
sudo systemctl stop kopuz-website.service

if [[ -f /mnt/musics/kopuz-website/kopuz-website.env ]]; then
  install -m 0600 \
    /mnt/musics/kopuz-website/kopuz-website.env \
    "$deploy_root/shared/kopuz-website.env"
fi
if [[ -f /mnt/musics/kopuz-website/sponsors_state.json ]]; then
  install -m 0600 \
    /mnt/musics/kopuz-website/sponsors_state.json \
    "$deploy_root/shared/sponsors_state.json"
fi

cd "$deploy_root/source"
sudo install -m 0644 systemd/kopuz-website.service \
  /etc/systemd/system/kopuz-website.service
sudo systemctl daemon-reload
sudo systemctl enable --now kopuz-website.service
curl --fail http://127.0.0.1:8090/
```

The deployment script verifies that the installed unit matches the repository. Reinstall the unit and run `systemctl daemon-reload` whenever it changes.

The deployment user needs non-interactive permission for the single restart command used by CI. Confirm the path printed by `command -v systemctl`, then create `/etc/sudoers.d/kopuz-website-deploy` with:

```sudoers
temidaradev ALL=(root) NOPASSWD: /usr/bin/systemctl restart kopuz-website.service
```

Validate it with:

```bash
sudo chmod 0440 /etc/sudoers.d/kopuz-website-deploy
sudo visudo -cf /etc/sudoers.d/kopuz-website-deploy
```

## 2. Configure Tailscale

The workflow joins the tailnet as an ephemeral `tag:ci` device. Create its credentials in the Tailscale admin console:

1. Make sure `tag:ci` exists in the tailnet policy and can reach TCP port 22 on the homeserver.
2. Open **Trust credentials**, select **Credential**, then **OAuth**.
3. Grant **Auth keys** (`auth_keys`) **Write** access and select `tag:ci`.
4. Select **Generate credential**.
5. Copy both values before closing the page. The client secret is only shown once.

The generated client ID is `TS_OAUTH_CLIENT_ID`; the generated client secret is `TS_OAUTH_SECRET`. See the official [OAuth client setup](https://tailscale.com/docs/features/oauth-clients#setting-up-an-oauth-client) and [GitHub Action guide](https://tailscale.com/docs/integrations/github/github-action#using-an-oauth-client).

The workflow uses the official [`tailscale/github-action`](https://github.com/tailscale/github-action) and removes its ephemeral node after each run.

## 3. Configure SSH

Create a dedicated unencrypted deployment key and add its public key to `/home/temidaradev/.ssh/authorized_keys` on the server:

```bash
ssh-keygen -t ed25519 -N '' -f ~/.ssh/kopuz-website-ci -C kopuz-website-ci
ssh-copy-id -i ~/.ssh/kopuz-website-ci.pub \
  temidaradev@YOUR_SERVER_TAILSCALE_NAME
```

Collect the server's Ed25519 host key using the same Tailscale hostname that CI will use. Compare the two `SHA256:` fingerprints before storing the scan:

```bash
# Run on the server.
sudo ssh-keygen -lf /etc/ssh/ssh_host_ed25519_key.pub

# Run on the client.
ssh-keyscan -t ed25519 YOUR_SERVER_TAILSCALE_NAME \
  > kopuz-website-known-hosts
ssh-keygen -lf kopuz-website-known-hosts
```

The two SSH secret values come from these local files:

| Secret | Source |
| --- | --- |
| `DEPLOY_SSH_PRIVATE_KEY_B64` | Single-line Base64 encoding of `~/.ssh/kopuz-website-ci` (`base64 -w 0 ~/.ssh/kopuz-website-ci`) |
| `DEPLOY_SSH_KNOWN_HOSTS` | Entire verified contents of `kopuz-website-known-hosts` |

Keep the private key unencrypted because the Actions runner cannot answer a passphrase prompt. The matching `.pub` file belongs only in the server's `authorized_keys`.

## 4. Configure GitHub

In `Kopuz-org/kopuz-website`, open **Settings > Secrets and variables > Actions**.

On the **Variables** tab, add:

| Variable | Value |
| --- | --- |
| `DEPLOY_HOST` | The same Tailscale MagicDNS name or Tailscale IP used by `ssh-keyscan` |

On the **Secrets** tab, add:

| Secret | Value comes from |
| --- | --- |
| `TS_OAUTH_CLIENT_ID` | Client ID shown after generating the Tailscale OAuth credential |
| `TS_OAUTH_SECRET` | Client secret shown once after generating that credential |
| `DEPLOY_SSH_PRIVATE_KEY_B64` | Base64-encoded `~/.ssh/kopuz-website-ci` |
| `DEPLOY_SSH_KNOWN_HOSTS` | `kopuz-website-known-hosts` |

From this repository, the equivalent GitHub CLI commands are:

```bash
gh variable set DEPLOY_HOST --body 'YOUR_SERVER_TAILSCALE_NAME'
gh secret set TS_OAUTH_CLIENT_ID
gh secret set TS_OAUTH_SECRET
base64 -w 0 ~/.ssh/kopuz-website-ci | gh secret set DEPLOY_SSH_PRIVATE_KEY_B64
gh secret set DEPLOY_SSH_KNOWN_HOSTS < kopuz-website-known-hosts
```

The first two `gh secret set` commands prompt for the copied Tailscale values without printing them. GitHub will list a secret's name afterward but will not reveal its stored value. See [GitHub's repository secret instructions](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/use-secrets#creating-secrets-for-a-repository).

Push to `master`, or run **CI and deploy** manually from the Actions page. The first successful automated deployment activates a release named after its Git commit.

## 5. Remove the old deployment

After the workflow passes, verify the active path and local health check on the server:

```bash
readlink -f /home/temidaradev/Projects/kopuz-website/current
curl --fail http://127.0.0.1:8090/
systemctl status kopuz-website.service
```

Then remove the old copy:

```bash
rm -rf /mnt/musics/kopuz-website
```
