#!/usr/bin/env bash
# Build the React client (nextgen-client) and copy it into wwwroot/, which the
# Rust server serves. Can be run from any directory.
set -euo pipefail

repo="$(cd "$(dirname "$0")" && pwd)"

cd "$repo/nextgen-client"
[ -d node_modules ] || npm ci
npm run build

# Empty wwwroot (keeping .gitkeep) and copy the whole build, including static/.
find "$repo/wwwroot" -mindepth 1 ! -name .gitkeep -exec rm -rf {} +
cp -r build/. "$repo/wwwroot/"
