#!/usr/bin/env bash
set -euo pipefail

# For building the simulator to run locally use:
# cargo build

# webapp build:
wasm-pack build
cd ./site/
npm link divein
npm install && npm run serve
