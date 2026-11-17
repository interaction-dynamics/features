#!/bin/sh
# Install features-cli, the Features command-line tool.
#
#   curl -fsSL https://<website>/install.sh | sh
#
# Downloads the prebuilt binary for this machine from the GitHub release,
# checks it against its published SHA-256 checksum and installs it as
# `features`. Supports macOS and Linux on x64 and arm64. Linux uses the
# statically linked musl build, so it runs whatever the glibc version.
#
# Environment variables:
#   FEATURES_VERSION      Version to install, e.g. 0.15.0 (default: latest).
#   FEATURES_INSTALL_DIR  Where to put the binary (default: ~/.local/bin).
#
# Pass them to `sh`, not to `curl`:
#   curl -fsSL https://<website>/install.sh | FEATURES_VERSION=0.15.0 sh
#
# Source: https://github.com/interaction-dynamics/features/blob/master/projects/website/public/install.sh

set -eu

repository="https://github.com/interaction-dynamics/features"

say() {
  printf '%s\n' "$*"
}

fail() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

# Everything runs inside main, called on the last line, so a download cut
# short by `curl | sh` never runs half a script.
main() {
  platform=$(detect_platform)
  version="${FEATURES_VERSION:-}"
  version="${version#v}"
  install_dir="${FEATURES_INSTALL_DIR:-$HOME/.local/bin}"

  if [ -n "$version" ]; then
    base_url="$repository/releases/download/v$version"
  else
    base_url="$repository/releases/latest/download"
  fi
  asset="features-$platform"

  tmp_dir=$(mktemp -d 2>/dev/null || mktemp -d -t features)
  trap 'rm -rf "$tmp_dir"' EXIT INT TERM

  say "Downloading $asset (${version:-latest})..."
  download "$base_url/$asset" "$tmp_dir/$asset" ||
    fail "could not download $base_url/$asset. Check that the version exists: $repository/releases"
  download "$base_url/$asset.sha256" "$tmp_dir/$asset.sha256" ||
    fail "could not download the checksum $base_url/$asset.sha256"

  verify_checksum "$tmp_dir/$asset" "$tmp_dir/$asset.sha256"

  mkdir -p "$install_dir" || fail "could not create $install_dir"
  chmod 755 "$tmp_dir/$asset"
  mv "$tmp_dir/$asset" "$install_dir/features" ||
    fail "could not write to $install_dir. Set FEATURES_INSTALL_DIR to a folder you own"

  say "Installed $("$install_dir/features" --version 2>/dev/null || echo features) to $install_dir/features"

  case ":${PATH}:" in
    *":$install_dir:"*)
      say "Run: features serve ./src"
      ;;
    *)
      say ""
      say "$install_dir is not in your PATH. Add it to your shell profile:"
      say "  export PATH=\"$install_dir:\$PATH\""
      say "Then run: features serve ./src"
      ;;
  esac
}

detect_platform() {
  os=$(uname -s)
  arch=$(uname -m)

  case "$arch" in
    x86_64 | amd64) arch=x64 ;;
    arm64 | aarch64) arch=arm64 ;;
    *) fail "unsupported architecture: $arch. Install with npm instead: npm install -g features-cli" ;;
  esac

  case "$os" in
    Darwin)
      # A shell running under Rosetta reports x86_64 on Apple silicon.
      if [ "$arch" = x64 ] && [ "$(sysctl -n sysctl.proc_translated 2>/dev/null)" = 1 ]; then
        arch=arm64
      fi
      echo "darwin-$arch"
      ;;
    Linux)
      echo "linux-$arch-musl"
      ;;
    MINGW* | MSYS* | CYGWIN* | Windows_NT)
      fail "Windows is not supported by this script. Install with npm instead: npm install -g features-cli"
      ;;
    *)
      fail "unsupported operating system: $os. Install with npm or cargo instead: $repository#installation"
      ;;
  esac
}

download() {
  if command -v curl >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -fsSL "$1" -o "$2"
  elif command -v wget >/dev/null 2>&1; then
    wget --https-only -q "$1" -O "$2"
  else
    fail "curl or wget is required"
  fi
}

verify_checksum() {
  expected=$(cut -d ' ' -f 1 <"$2")
  if command -v sha256sum >/dev/null 2>&1; then
    actual=$(sha256sum "$1" | cut -d ' ' -f 1)
  elif command -v shasum >/dev/null 2>&1; then
    actual=$(shasum -a 256 "$1" | cut -d ' ' -f 1)
  else
    fail "sha256sum or shasum is required to verify the download"
  fi
  [ -n "$expected" ] && [ "$expected" = "$actual" ] ||
    fail "checksum mismatch for $(basename "$1"): expected $expected, got $actual"
}

main "$@"
