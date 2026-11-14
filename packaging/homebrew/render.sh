#!/usr/bin/env bash
# Render the Homebrew formula for a features-cli release.
#
# Usage:
#   packaging/homebrew/render.sh <version> <checksums.txt> [output]
#
# <version>        Release version without the leading "v" (e.g. 0.14.0).
# <checksums.txt>  sha256sum output for the release assets, one
#                  "<sha256>  <asset-name>" line per asset (the checksums.txt
#                  file attached to each GitHub release).
# [output]         Where to write the formula (default: stdout).
#
# Example (render from a published release):
#   gh release download v0.14.0 -p checksums.txt -D /tmp/features
#   packaging/homebrew/render.sh 0.14.0 /tmp/features/checksums.txt features.rb
set -euo pipefail

if [[ $# -lt 2 || $# -gt 3 ]]; then
  sed -n '2,17p' "$0" | sed 's/^# \{0,1\}//' >&2
  exit 64
fi

version="${1#v}"
checksums="$2"
output="${3:-/dev/stdout}"
template="$(dirname "$0")/features.rb.tmpl"

if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-+][0-9A-Za-z.-]+)?$ ]]; then
  echo "error: invalid version '$version'" >&2
  exit 65
fi

sha_for() {
  local asset="$1" sha
  sha="$(awk -v name="$asset" '{ n = $2; sub(/^\*/, "", n) } n == name { print $1; exit }' "$checksums")"
  if [[ ! "$sha" =~ ^[0-9a-f]{64}$ ]]; then
    echo "error: no sha256 for '$asset' in $checksums" >&2
    exit 65
  fi
  printf '%s' "$sha"
}

# Resolve every checksum first so a missing asset aborts before any output.
darwin_arm64="$(sha_for features-darwin-arm64)"
darwin_x64="$(sha_for features-darwin-x64)"
linux_arm64_musl="$(sha_for features-linux-arm64-musl)"
linux_x64_musl="$(sha_for features-linux-x64-musl)"

sed \
  -e "s/@VERSION@/$version/g" \
  -e "s/@SHA256_DARWIN_ARM64@/$darwin_arm64/g" \
  -e "s/@SHA256_DARWIN_X64@/$darwin_x64/g" \
  -e "s/@SHA256_LINUX_ARM64_MUSL@/$linux_arm64_musl/g" \
  -e "s/@SHA256_LINUX_X64_MUSL@/$linux_x64_musl/g" \
  "$template" > "$output"
