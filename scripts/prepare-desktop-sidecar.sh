#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN_DIR="$ROOT/desktop/src-tauri/binaries"
mkdir -p "$BIN_DIR"

TARGET="${1:-}"
if [[ -z "$TARGET" ]]; then
  TARGET="$(rustc --print host-tuple 2>/dev/null || rustc -Vv | awk '/^host:/{print $2}')"
fi

EXT=""
case "$TARGET" in
  *windows*) EXT=".exe" ;;
esac

LOCKED_ARGS=()
if [[ "${CI:-}" == "true" || "${CARGO_LOCKED:-}" == "1" ]]; then
  LOCKED_ARGS+=(--locked)
fi

HOST="$(rustc --print host-tuple 2>/dev/null || rustc -Vv | awk '/^host:/{print $2}')"
echo "building orbien sidecar for target=${TARGET} (host=${HOST})"

build_sidecar() {
  if [[ "$TARGET" == "$HOST" ]]; then
    cargo build --release ${LOCKED_ARGS[@]+"${LOCKED_ARGS[@]}"} \
      -p orbien-client --manifest-path "$ROOT/Cargo.toml"
    SRC="$ROOT/target/release/orbien${EXT}"
  else
    cargo build --release ${LOCKED_ARGS[@]+"${LOCKED_ARGS[@]}"} \
      -p orbien-client --target "$TARGET" --manifest-path "$ROOT/Cargo.toml"
    SRC="$ROOT/target/${TARGET}/release/orbien${EXT}"
  fi
}
build_sidecar

if [[ ! -f "$SRC" ]]; then
  echo "sidecar binary not found: $SRC" >&2
  exit 1
fi

DEST="$BIN_DIR/orbien-${TARGET}${EXT}"
cp "$SRC" "$DEST"
chmod +x "$DEST" 2>/dev/null || true
ls -lh "$DEST"
echo "sidecar ready: $DEST"
