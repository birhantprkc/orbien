#!/usr/bin/env bash
set -euo pipefail

NAME=""
VERSION=""
OS=""
ARCH=""
LIBC=""
OUTDIR="dist/release"
BIN=""
CONFIG=""
ASSETS=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --name) NAME="$2"; shift 2 ;;
    --version) VERSION="$2"; shift 2 ;;
    --os) OS="$2"; shift 2 ;;
    --arch) ARCH="$2"; shift 2 ;;
    --libc) LIBC="$2"; shift 2 ;;
    --outdir) OUTDIR="$2"; shift 2 ;;
    --bin) BIN="$2"; shift 2 ;;
    --config) CONFIG="$2"; shift 2 ;;
    --assets) ASSETS="$2"; shift 2 ;;
    *) echo "unknown arg: $1" >&2; exit 1 ;;
  esac
done

if [[ -z "$NAME" || -z "$VERSION" || -z "$OS" || -z "$ARCH" || -z "$BIN" ]]; then
  echo "missing required args" >&2
  exit 1
fi
if [[ ! -f "$BIN" ]]; then
  echo "binary not found: $BIN" >&2
  exit 1
fi
if [[ "$OS" == "linux" && -z "$LIBC" ]]; then
  echo "linux packages require --libc gnu|musl" >&2
  exit 1
fi
if [[ -n "$LIBC" && "$LIBC" != "gnu" && "$LIBC" != "musl" ]]; then
  echo "invalid --libc: $LIBC (expected gnu|musl)" >&2
  exit 1
fi

STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT

BIN_BASENAME="$(basename "$BIN")"
case "$OS" in
  windows)
    case "$NAME" in
      orbien) DEST_BIN="orbien.exe" ;;
      orbien-server) DEST_BIN="orbien-server.exe" ;;
      orbien-desktop) DEST_BIN="orbien-desktop.exe" ;;
      *) DEST_BIN="${BIN_BASENAME}" ;;
    esac
    ;;
  *)
    case "$NAME" in
      orbien) DEST_BIN="orbien" ;;
      orbien-server) DEST_BIN="orbien-server" ;;
      orbien-desktop) DEST_BIN="orbien-desktop" ;;
      *) DEST_BIN="${BIN_BASENAME}" ;;
    esac
    ;;
esac

cp "$BIN" "${STAGE}/${DEST_BIN}"
chmod +x "${STAGE}/${DEST_BIN}" || true

if [[ -n "$CONFIG" ]]; then
  if [[ ! -f "$CONFIG" ]]; then
    echo "config not found: $CONFIG" >&2
    exit 1
  fi
  cp "$CONFIG" "${STAGE}/$(basename "$CONFIG")"
fi

if [[ -n "$ASSETS" ]]; then
  if [[ ! -d "$ASSETS" ]]; then
    echo "assets dir not found: $ASSETS" >&2
    exit 1
  fi
  mkdir -p "${STAGE}/assets"
  cp -R "${ASSETS}/." "${STAGE}/assets/"
fi

mkdir -p "$OUTDIR"
if [[ -n "$LIBC" ]]; then
  ARCHIVE="${OUTDIR}/${NAME}_${VERSION}_${OS}_${ARCH}_${LIBC}.tar.gz"
else
  ARCHIVE="${OUTDIR}/${NAME}_${VERSION}_${OS}_${ARCH}.tar.gz"
fi
tar -C "$STAGE" -czf "$ARCHIVE" .
echo "wrote ${ARCHIVE}"
ls -lh "$ARCHIVE"
