#!/usr/bin/env bash
set -Eeuo pipefail

revision="${1:-}"
deploy_root="${DEPLOY_ROOT:-$HOME/Projects/kopuz-website}"
source_dir="${DEPLOY_SOURCE_DIR:-$deploy_root/source}"
service_name="${DEPLOY_SERVICE:-kopuz-website.service}"
healthcheck_url="${DEPLOY_HEALTHCHECK_URL:-http://127.0.0.1:8090/}"
releases_to_keep="${DEPLOY_RELEASES_TO_KEEP:-5}"
healthcheck_attempts="${DEPLOY_HEALTHCHECK_ATTEMPTS:-30}"
healthcheck_delay="${DEPLOY_HEALTHCHECK_DELAY:-2}"
expected_cargo_leptos_version="cargo-leptos 0.3.6"
expected_wasm_bindgen_version="wasm-bindgen 0.2.127"
releases_dir="$deploy_root/releases"
shared_dir="$deploy_root/shared"
current_link="$deploy_root/current"
installed_unit="${DEPLOY_UNIT_PATH:-/etc/systemd/system/$service_name}"
stage_dir=""
previous_target=""
rollback_armed=0

fail() {
  printf 'deploy: %s\n' "$*" >&2
  exit 1
}

require_command() {
  command -v "$1" >/dev/null 2>&1 || fail "required command not found: $1"
}

cleanup_stage() {
  if [[ -n "$stage_dir" && -d "$stage_dir" ]]; then
    rm -rf -- "$stage_dir"
  fi
}

set_current() {
  local target="$1"
  local next_link="${current_link}.new.$$"

  rm -f -- "$next_link"
  ln -s "$target" "$next_link"
  mv -Tf -- "$next_link" "$current_link"
}

restart_service() {
  sudo -n systemctl restart "$service_name"
}

wait_for_healthcheck() {
  local attempt
  for ((attempt = 1; attempt <= healthcheck_attempts; attempt += 1)); do
    if curl --fail --silent --show-error --max-time 3 "$healthcheck_url" >/dev/null 2>&1; then
      return 0
    fi
    sleep "$healthcheck_delay"
  done
  return 1
}

rollback() {
  rollback_armed=0
  if [[ -z "$previous_target" ]]; then
    printf 'No previous release is available for rollback\n' >&2
    return 1
  fi

  printf 'Restoring previous release %s\n' "$previous_target" >&2
  if ! set_current "$previous_target"; then
    printf 'Could not restore the previous current symlink\n' >&2
    return 1
  fi
  if ! restart_service; then
    printf 'Could not restart %s after restoring the symlink\n' "$service_name" >&2
    return 1
  fi
  if ! wait_for_healthcheck; then
    printf 'The restored release did not pass %s\n' "$healthcheck_url" >&2
    return 1
  fi

  printf 'Rollback passed %s\n' "$healthcheck_url" >&2
}

on_exit() {
  local status="$?"
  trap - EXIT INT TERM
  cleanup_stage

  if [[ "$status" -ne 0 && "$rollback_armed" -eq 1 ]]; then
    rollback || printf 'Automatic rollback failed; inspect %s\n' "$service_name" >&2
  fi

  exit "$status"
}
trap on_exit EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

[[ "$revision" =~ ^[0-9a-f]{40,64}$ ]] || fail "expected a full Git revision"
[[ "$releases_to_keep" =~ ^[1-9][0-9]*$ ]] || fail "DEPLOY_RELEASES_TO_KEEP must be a positive integer"
[[ "$healthcheck_attempts" =~ ^[1-9][0-9]*$ ]] || fail "DEPLOY_HEALTHCHECK_ATTEMPTS must be a positive integer"
[[ "$healthcheck_delay" =~ ^[0-9]+([.][0-9]+)?$ ]] || fail "DEPLOY_HEALTHCHECK_DELAY must be a non-negative number"
[[ "$service_name" =~ ^[A-Za-z0-9@_.-]+$ ]] || fail "invalid systemd service name"
[[ -f "$source_dir/Cargo.toml" ]] || fail "source tree not found at $source_dir"
[[ -f "$source_dir/Cargo.lock" ]] || fail "Cargo.lock is missing from the source tree"
[[ -f "$source_dir/systemd/$service_name" ]] || fail "service unit is missing from the source tree"
[[ -f "$installed_unit" ]] || fail "installed service unit not found at $installed_unit"

if [[ -f "$HOME/.cargo/env" ]]; then
  # rustup adds Cargo to PATH through this file.
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi
export PATH="$HOME/.cargo/bin:$PATH"

for command_name in cargo cmp curl flock sudo systemctl wasm-bindgen; do
  require_command "$command_name"
done

actual_cargo_leptos_version="$(cargo leptos --version 2>/dev/null || true)"
[[ "$actual_cargo_leptos_version" == "$expected_cargo_leptos_version" ]] || \
  fail "expected $expected_cargo_leptos_version, found ${actual_cargo_leptos_version:-nothing}"
actual_wasm_bindgen_version="$(wasm-bindgen --version 2>/dev/null || true)"
[[ "$actual_wasm_bindgen_version" == "$expected_wasm_bindgen_version" ]] || \
  fail "expected $expected_wasm_bindgen_version, found ${actual_wasm_bindgen_version:-nothing}"
cmp -s "$source_dir/systemd/$service_name" "$installed_unit" || \
  fail "$installed_unit differs from the repository; reinstall it and run systemctl daemon-reload"

install -d -m 0755 "$deploy_root" "$source_dir" "$releases_dir"
install -d -m 0700 "$shared_dir"

exec 9>"$deploy_root/.deploy.lock"
flock -w 900 9 || fail "another deployment still holds $deploy_root/.deploy.lock"

release_dir="$releases_dir/$revision"
if [[ ! -x "$release_dir/kopuz-website" || ! -d "$release_dir/site" ]]; then
  if [[ -e "$release_dir" ]]; then
    rm -rf -- "$release_dir"
  fi

  printf 'Building %s on the homeserver\n' "$revision"
  cd "$source_dir"

  if command -v rustup >/dev/null 2>&1 && \
    ! rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
    rustup target add wasm32-unknown-unknown
  fi

  rm -f \
    "$source_dir/target/server/release/kopuz-website" \
    "$source_dir/target/release/kopuz-website"
  rm -rf "$source_dir/target/site"
  cargo leptos build --release \
    --lib-cargo-args='--locked' \
    --bin-cargo-args='--locked'

  server_binary=""
  for candidate in \
    "$source_dir/target/server/release/kopuz-website" \
    "$source_dir/target/release/kopuz-website"; do
    if [[ -x "$candidate" ]]; then
      server_binary="$candidate"
      break
    fi
  done

  [[ -n "$server_binary" ]] || fail "cargo-leptos did not produce the server binary"
  [[ -d "$source_dir/target/site" ]] || fail "cargo-leptos did not produce target/site"

  stage_dir="$(mktemp -d "$releases_dir/.${revision}.XXXXXX")"
  install -m 0755 "$server_binary" "$stage_dir/kopuz-website"
  install -d -m 0755 "$stage_dir/site"
  cp -a "$source_dir/target/site/." "$stage_dir/site/"
  printf '%s\n' "$revision" > "$stage_dir/REVISION"
  mv -- "$stage_dir" "$release_dir"
  stage_dir=""
else
  printf 'Reusing existing release %s\n' "$revision"
fi

if [[ -L "$current_link" ]]; then
  previous_target="$(readlink "$current_link")"
elif [[ -e "$current_link" ]]; then
  fail "$current_link exists but is not a symbolic link"
fi

rollback_armed=1
set_current "releases/$revision"
restart_service || fail "systemd could not restart $service_name"
wait_for_healthcheck || fail "$healthcheck_url did not become healthy"
rollback_armed=0

prune_releases() {
  local active_path
  local retained=0
  local path
  local -a release_paths

  active_path="$(readlink -f "$current_link")"
  mapfile -t release_paths < <(
    find "$releases_dir" -mindepth 1 -maxdepth 1 -type d ! -name '.*' \
      -printf '%T@ %p\n' | sort -nr | cut -d ' ' -f 2-
  )

  for path in "${release_paths[@]}"; do
    if [[ "$(readlink -f "$path")" == "$active_path" || "$retained" -lt "$releases_to_keep" ]]; then
      retained=$((retained + 1))
      continue
    fi
    rm -rf -- "$path"
  done
}

prune_releases
printf 'Activated %s and passed %s\n' "$revision" "$healthcheck_url"
