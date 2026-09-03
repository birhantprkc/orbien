#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILD=0
UPX=0
ARCH=""

usage() {
  cat <<EOF
Usage: $0 [--build] [--upx] [--arch amd64|arm64]

  --build   cargo build linux musl binaries (Linux hosts only)
  --upx     compress binaries with UPX when available
  --arch    target arch (default: auto from uname)

Local macOS/Windows: build elsewhere, then run:
  $0 --arch arm64   # uses target/aarch64-unknown-linux-musl/release if present
EOF
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --build) BUILD=1; shift ;;
    --upx) UPX=1; shift ;;
    --arch) ARCH="$2"; shift 2 ;;
    -h|--help) usage ;;
    *) echo "unknown arg: $1" >&2; usage ;;
  esac
done

if [[ -z "$ARCH" ]]; then
  case "$(uname -m)" in
    x86_64|amd64) ARCH=amd64 ;;
    arm64|aarch64) ARCH=arm64 ;;
    *) echo "unsupported host arch: $(uname -m)" >&2; exit 1 ;;
  esac
fi

case "$ARCH" in
  amd64) TARGET=x86_64-unknown-linux-musl ;;
  arm64) TARGET=aarch64-unknown-linux-musl ;;
  *) echo "unsupported arch: $ARCH" >&2; exit 1 ;;
esac

BIN_DIR="${ROOT}/target/${TARGET}/release"

if [[ "$BUILD" -eq 1 ]]; then
  if [[ "$(uname -s)" != "Linux" ]]; then
    echo "cross-build on $(uname -s) is not supported; use CI artifacts or Linux VM" >&2
    exit 1
  fi
  if ! command -v musl-gcc >/dev/null 2>&1; then
    echo "musl-gcc not found (install musl-tools)" >&2
    exit 1
  fi
  export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc
  export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc
  cd "$ROOT"
  npm ci --prefix server-ui
  npm run build --prefix server-ui
  cargo build --release --locked -p orbien-server -p orbien-client --target "$TARGET"
fi

[[ -d "$BIN_DIR" ]] || {
  echo "binaries not found: ${BIN_DIR}" >&2
  echo "run on Linux with --build, or copy musl binaries into target/${TARGET}/release/" >&2
  exit 1
}

UPX_BIN=""
if [[ "$UPX" -eq 1 ]]; then
  if command -v upx >/dev/null 2>&1; then
    UPX_BIN="$(command -v upx)"
  else
    echo "warning: upx not found, skipping compression" >&2
  fi
fi

rm -rf "${ROOT}/docker_slice" "${ROOT}/docker_context"
STAGE_ARGS=(--arch "$ARCH" --bin-dir "$BIN_DIR")
[[ -n "$UPX_BIN" ]] && STAGE_ARGS+=(--upx "$UPX_BIN")
"${ROOT}/scripts/stage-docker-slice.sh" "${STAGE_ARGS[@]}"

mkdir -p "${ROOT}/docker_context"
cp -a "${ROOT}/docker_slice/"* "${ROOT}/docker_context/"
echo "docker context ready: ${ROOT}/docker_context (arch=${ARCH})"
