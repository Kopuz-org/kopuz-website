<p align="center">
  <img src="public/logo.svg" width="88" alt="Kopuz logo">
</p>

<h1 align="center">Kopuz Website</h1>

<p align="center">
  The official website for <a href="https://github.com/Kopuz-org/kopuz">Kopuz</a>, a fast, cross-platform music player for local libraries and connected music services.
</p>

<p align="center">
  <a href="https://kopuz.moe">Live site</a> ·
  <a href="https://github.com/Kopuz-org/kopuz">Kopuz source</a> ·
  <a href="https://github.com/Kopuz-org/kopuz/releases/latest">Latest release</a>
</p>

## About

This repository contains the server-rendered site at [kopuz.moe](https://kopuz.moe). It provides the product overview, feature reference, downloads, setup guides, support information, privacy policy, localized content, and a browser handoff route for Kopuz share links.

The site is built with:

- [Leptos](https://leptos.dev/) for server rendering and WebAssembly hydration
- [Axum](https://github.com/tokio-rs/axum) for the HTTP server and sponsor webhook
- SCSS for the light, dark, and `?moe` themes
- [Fluent](https://projectfluent.org/) for localization
- [Playwright](https://playwright.dev/) for end-to-end browser tests

The music player itself lives in the [Kopuz application repository](https://github.com/Kopuz-org/kopuz).

## Requirements

- Rust 1.96.0 with the `wasm32-unknown-unknown` target, pinned by `rust-toolchain.toml`
- [`cargo-leptos` 0.3.6](https://github.com/leptos-rs/cargo-leptos)
- `wasm-bindgen-cli` 0.2.127
- Dart Sass available as `sass`
- Node.js and npm for browser tests

Install the Rust build tools with:

```bash
cargo install --version 0.3.6 --locked cargo-leptos
cargo install --version 0.2.127 --locked wasm-bindgen-cli
```

## Development

```bash
git clone https://github.com/Kopuz-org/kopuz-website.git
cd kopuz-website
cargo leptos watch
```

The development server listens on <http://127.0.0.1:3000>. Rust, SCSS, and hydration changes rebuild automatically.

User-facing strings live in `locales/*/main.ftl`. English is the fallback locale for keys that have not been translated yet.

## Checks

Run formatting, server tests, an explicit hydration compile, and the production build:

```bash
cargo fmt --all -- --check
cargo test --locked --all-targets --features ssr
cargo check --locked --lib --no-default-features --features hydrate
cargo leptos build --release \
  --lib-cargo-args='--locked' \
  --bin-cargo-args='--locked'
```

For browser tests, keep the site running in one terminal, then run:

```bash
cd end2end
npm ci
npx playwright install chromium
npx playwright test --project=chromium --project=mobile-chromium
```

Set `PLAYWRIGHT_BASE_URL` to test another server. Set `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` to use an existing Chromium binary.

## Production build

```bash
cargo leptos build --release \
  --lib-cargo-args='--locked' \
  --bin-cargo-args='--locked'

LEPTOS_SITE_ADDR=127.0.0.1:3000 \
LEPTOS_SITE_ROOT=target/site \
  target/release/kopuz-website
```

The build produces the server at `target/release/kopuz-website` and browser assets under `target/site`.

The sponsor webhook reads `GITHUB_SPONSORS_WEBHOOK_SECRET`. Persistent sponsor data defaults to `sponsors_state.json` and can be moved with `SPONSORS_STATE_PATH`.

## Repository layout

| Path | Purpose |
| --- | --- |
| `src/app.rs` | Shared UI, routing shell, release data, themes, and reusable sections |
| `src/pages.rs` | Top-level feature, download, guide, and support pages |
| `src/main.rs` | Axum server, static assets, and runtime configuration |
| `src/sponsors.rs` | Sponsor state and signed GitHub webhook handling |
| `locales/` | Fluent translations and language metadata |
| `style/main.scss` | Responsive site styles and themes |
| `public/` | Images, icons, manifests, and other static files |
| `end2end/` | Playwright configuration and browser tests |
| `scripts/`, `systemd/` | Atomic homeserver deployment and service files |

## Deployment

CI checks pull requests and every push to `master`. Repository maintainers can deploy successful `master` builds through the configured Tailscale and SSH workflow. See [DEPLOYMENT.md](DEPLOYMENT.md) for server setup, secrets, rollback behavior, and operations.

## License

Released into the public domain under the [Unlicense](LICENSE).
