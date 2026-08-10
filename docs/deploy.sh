#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

pnpm run clear
USE_SSH=true pnpm run deploy
