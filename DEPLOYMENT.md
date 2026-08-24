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

Run the setup as `temidaradev`. The server needs `git`, `rsync`, `curl`, `flock` from `util-linux`, a C toolchain, Rust, `cargo-leptos` 0.3.6, and `wasm-bindgen-cli` 0.2.127. Install the OS packages using the server's package manager, then run:

```bash
rustup toolchain install 1.96.0 --profile minimal \
  --target wasm32-unknown-unknown
cargo +1.96.0 install --version 0.3.6 --locked cargo-leptos
cargo +1.96.0 install --version 0.2.127 --locked wasm-bindgen-cli

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

Create a Tailscale OAuth client with the writable `auth_keys` scope and permission to create ephemeral nodes tagged `tag:ci`. The tailnet policy must allow `tag:ci` to reach TCP port 22 on the homeserver.

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

## 4. Configure GitHub

Add this Actions variable under **Settings > Secrets and variables > Actions > Variables**:

| Variable | Value |
| --- | --- |
| `DEPLOY_HOST` | The server's Tailscale MagicDNS name or Tailscale IP |

Add these Actions secrets:

| Secret | Value |
| --- | --- |
| `TS_OAUTH_CLIENT_ID` | Tailscale OAuth client ID |
| `TS_OAUTH_SECRET` | Tailscale OAuth client secret |
| `DEPLOY_SSH_PRIVATE_KEY` | Contents of `~/.ssh/kopuz-website-ci` |
| `DEPLOY_SSH_KNOWN_HOSTS` | Verified contents of `kopuz-website-known-hosts` |

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
