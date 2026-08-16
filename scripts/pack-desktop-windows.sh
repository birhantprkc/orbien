#!/usr/bin/env bash

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN=""
VERSION=""
OUTDIR="${ROOT}/dist"
ARCH_LABEL="amd64"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bin) BIN="$2"; shift 2 ;;
    --version) VERSION="$2"; shift 2 ;;
    --outdir) OUTDIR="$2"; shift 2 ;;
    --arch) ARCH_LABEL="$2"; shift 2 ;;
    *) echo "unknown arg: $1" >&2; exit 1 ;;
  esac
done

if [[ -z "$VERSION" ]]; then
  VERSION="$(
    cargo metadata --no-deps --format-version 1 --manifest-path "${ROOT}/Cargo.toml" \
      | python3 -c '
import json, sys
meta = json.load(sys.stdin)
for p in meta["packages"]:
    if p["name"] == "orbien-desktop":
        print(p["version"])
        break
else:
    sys.exit("orbien-desktop not found in cargo metadata")
'
  )"
fi

if [[ -z "$BIN" ]]; then
  if [[ -f "${ROOT}/target/release/orbien-desktop.exe" ]]; then
    BIN="${ROOT}/target/release/orbien-desktop.exe"
  elif [[ -f "${ROOT}/target/release/orbien-desktop" ]]; then
    BIN="${ROOT}/target/release/orbien-desktop"
  else
    echo "binary not found; pass --bin" >&2
    exit 1
  fi
fi

if [[ ! -f "$BIN" ]]; then
  echo "binary not found: $BIN" >&2
  exit 1
fi

mkdir -p "$OUTDIR"
STAGE="${OUTDIR}/.win-stage-$$"
rm -rf "$STAGE"
mkdir -p "$STAGE"

EXE_NAME="Orbien-Desktop.exe"
cp "$BIN" "${STAGE}/${EXE_NAME}"
STANDALONE="${OUTDIR}/orbien-desktop_${VERSION}_windows_${ARCH_LABEL}.exe"
cp "$BIN" "$STANDALONE"

cat > "${STAGE}/README.txt" <<EOF
Orbien Desktop ${VERSION}

1. Double-click Orbien-Desktop.exe to run.
2. No installer required (portable).
3. Config defaults to %USERPROFILE%\\.config\\orbien\\orbien.toml
   (or the path set in the Config page).
EOF

ZIP_NAME="orbien-desktop_${VERSION}_windows_${ARCH_LABEL}.zip"
ZIP_PATH="${OUTDIR}/${ZIP_NAME}"
rm -f "$ZIP_PATH"

(
  cd "$STAGE"
  if command -v zip >/dev/null 2>&1; then
    zip -q -r "$ZIP_PATH" .
  elif command -v powershell.exe >/dev/null 2>&1; then
    powershell.exe -NoProfile -Command \
      "Compress-Archive -Path * -DestinationPath '$ZIP_PATH' -Force"
  elif command -v powershell >/dev/null 2>&1; then
    powershell -NoProfile -Command \
      "Compress-Archive -Path * -DestinationPath '$ZIP_PATH' -Force"
  else
    echo "need zip or powershell to create archive" >&2
    exit 1
  fi
)

rm -rf "$STAGE"
echo "wrote ${STANDALONE}"
echo "wrote ${ZIP_PATH}"
ls -lh "$STANDALONE" "$ZIP_PATH"
