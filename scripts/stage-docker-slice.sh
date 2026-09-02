#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ARCH=""
BIN_DIR=""
UPX_BIN=""

usage() {
  cat <<EOF
Usage: $0 --arch amd64|arm64 --bin-dir <path> [--upx <upx-binary>]

Example:
  $0 --arch arm64 --bin-dir target/aarch64-unknown-linux-musl/release
EOF
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --arch) ARCH="$2"; shift 2 ;;
    --bin-dir) BIN_DIR="$2"; shift 2 ;;
    --upx) UPX_BIN="$2"; shift 2 ;;
    -h|--help) usage ;;
    *) echo "unknown arg: $1" >&2; usage ;;
  esac
done

[[ -n "$ARCH" && -n "$BIN_DIR" ]] || usage
[[ "$ARCH" == "amd64" || "$ARCH" == "arm64" ]] || {
  echo "unsupported arch: $ARCH (want amd64 or arm64)" >&2
  exit 1
}

ORBien="${BIN_DIR}/orbien"
SERVER="${BIN_DIR}/orbien-server"
[[ -f "$ORBien" && -f "$SERVER" ]] || {
  echo "missing binaries under ${BIN_DIR}" >&2
  exit 1
}

compress_bin() {
  local bin="$1"
  if [[ -n "$UPX_BIN" && -x "$UPX_BIN" ]]; then
    "$UPX_BIN" --lzma --best "$bin" || true
  fi
}

OUT="${ROOT}/docker_slice"
mkdir -p "${OUT}/orbien-linux-${ARCH}" "${OUT}/orbien-server-linux-${ARCH}"

install -m755 "$ORBien" "${OUT}/orbien-linux-${ARCH}/orbien"
install -m755 "$SERVER" "${OUT}/orbien-server-linux-${ARCH}/orbien-server"

compress_bin "${OUT}/orbien-linux-${ARCH}/orbien"
compress_bin "${OUT}/orbien-server-linux-${ARCH}/orbien-server"

echo "staged docker slice for ${ARCH} -> ${OUT}"
