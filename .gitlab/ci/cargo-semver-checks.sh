#!/bin/sh

set -e

readonly version="0.33.0"
readonly sha256sum="3438c1c155c1ad76ac7a0b7b04fa96cec48bf820720dd492e62f4552369fb03e"
readonly filename="cargo-semver-checks-x86_64-unknown-linux-musl.tar.gz"

cd .gitlab

echo "$sha256sum  $filename" > cargo-semver-checks.sha256sum
curl -OL "https://github.com/obi1kenobi/cargo-semver-checks/releases/download/v$version/$filename"
sha256sum --check cargo-semver-checks.sha256sum
tar -xf "$filename"
