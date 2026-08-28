#!/usr/bin/env python3
"""Build embedded Noto Sans SC subsets for orbien-desktop.

  python3 scripts/gen-desktop-font-subset.py              # i18n + GB2312 L1 (~1.8 MB)
  python3 scripts/gen-desktop-font-subset.py --regular-only
  python3 scripts/gen-desktop-font-subset.py --full       # full SC regional (~16 MB)
"""

from __future__ import annotations

import argparse
import json
import sys
import tempfile
import urllib.request
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DESKTOP = ROOT / "desktop"
FONTS = DESKTOP / "assets" / "fonts"
SOURCE = FONTS / "source"
TOOLS = FONTS / ".tools"
I18N_ZH = DESKTOP / "i18n" / "zh_CN.properties"
MANIFEST = FONTS / "subset-manifest.json"

CDN = "https://cdn.jsdelivr.net/gh/notofonts/noto-cjk@main/Sans/SubsetOTF/SC"
SRC_REGULAR = f"{CDN}/NotoSansSC-Regular.otf"
SRC_BOLD = f"{CDN}/NotoSansSC-Bold.otf"
OFL_URL = "https://cdn.jsdelivr.net/gh/google/fonts@main/ofl/notosanssc/OFL.txt"
FONTTOOLS_WHL = (
    "https://files.pythonhosted.org/packages/py3/f/fonttools/"
    "fonttools-4.59.0-py3-none-any.whl"
)

OUT_REGULAR = "NotoSansSC-Regular.subset.otf"
OUT_BOLD = "NotoSansSC-Bold.subset.otf"
FAMILY = "Noto Sans SC"


def download(url: str, dest: Path) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    print(f"fetch {dest.name}")
    with urllib.request.urlopen(url, timeout=180) as resp:
        dest.write_bytes(resp.read())


def ensure_fonttools() -> None:
    try:
        import fontTools

        return
    except ImportError:
        pass

    TOOLS.mkdir(parents=True, exist_ok=True)
    if not (TOOLS / "fontTools").is_dir():
        whl = TOOLS / "fonttools.whl"
        if not whl.exists():
            download(FONTTOOLS_WHL, whl)
        with zipfile.ZipFile(whl) as zf:
            zf.extractall(TOOLS)
    sys.path.insert(0, str(TOOLS))
    import fontTools


def i18n_chars() -> set[str]:
    chars: set[str] = set()
    for line in I18N_ZH.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        value = line.split("=", 1)[1].strip().replace("\\n", "\n").replace("\\t", "\t")
        chars.update(value)
    return chars


def gb2312_level1() -> set[str]:
    out: set[str] = set()
    for zone in range(0xB0, 0xD8):
        for point in range(0xA1, 0xFF):
            try:
                ch = bytes([zone, point]).decode("gb2312")
            except UnicodeDecodeError:
                continue
            if len(ch) == 1:
                out.add(ch)
    return out


def target_chars() -> set[str]:
    chars = set(chr(c) for c in range(0x20, 0x7F))
    chars.update("，、。（）…·—「」『』【】《》！？：；")
    return chars | i18n_chars() | gb2312_level1()


def write_manifest(mode: str, chars: set[str], files: dict[str, int]) -> None:
    listed = sorted(c for c in chars if ord(c) > 0x7F)
    MANIFEST.write_text(
        json.dumps(
            {
                "font_family": FAMILY,
                "license": "SIL Open Font License 1.1 (see OFL.txt)",
                "mode": mode,
                "char_count": len(chars),
                "files": files,
                "chars": listed,
            },
            indent=2,
            ensure_ascii=False,
        )
        + "\n",
        encoding="utf-8",
    )


def ensure_sources() -> tuple[Path, Path]:
    SOURCE.mkdir(parents=True, exist_ok=True)
    regular = SOURCE / "NotoSansSC-Regular.otf"
    bold = SOURCE / "NotoSansSC-Bold.otf"
    if not regular.exists():
        download(SRC_REGULAR, regular)
    if not bold.exists():
        download(SRC_BOLD, bold)
    return regular, bold


def subset(src: Path, dest: Path, unicodes_file: Path) -> int:
    from fontTools import subset as ftsubset

    ftsubset.main(
        [
            str(src),
            f"--output-file={dest}",
            f"--unicodes-file={unicodes_file}",
            "--layout-features=kern,liga",
            "--name-IDs=*",
            "--name-legacy",
            "--name-languages=*",
            "--notdef-outline",
            "--recommended-glyphs",
            "--recalc-bounds",
        ]
    )
    return dest.stat().st_size


def build_subset(regular_only: bool) -> None:
    ensure_fonttools()
    regular_src, bold_src = ensure_sources()
    chars = target_chars()
    print(f"subsetting {len(chars)} codepoints")

    with tempfile.NamedTemporaryFile("w", encoding="utf-8", suffix=".txt", delete=False) as f:
        for c in sorted(chars, key=ord):
            f.write(f"U+{ord(c):04X}\n")
        unicodes = Path(f.name)

    files: dict[str, int] = {}
    try:
        regular_out = FONTS / OUT_REGULAR
        bold_out = FONTS / OUT_BOLD
        files[OUT_REGULAR] = subset(regular_src, regular_out, unicodes)
        print(f"  {OUT_REGULAR}: {files[OUT_REGULAR] / 1024:.0f} KB")

        if regular_only:
            bold_out.write_bytes(regular_out.read_bytes())
            files[OUT_BOLD] = bold_out.stat().st_size
            mode = "i18n-gb2312-level1-regular-only"
            print(f"  {OUT_BOLD}: copy of Regular")
        else:
            files[OUT_BOLD] = subset(bold_src, bold_out, unicodes)
            mode = "i18n-gb2312-level1"
            print(f"  {OUT_BOLD}: {files[OUT_BOLD] / 1024:.0f} KB")
    finally:
        unicodes.unlink(missing_ok=True)

    ofl = FONTS / "OFL.txt"
    if not ofl.exists():
        download(OFL_URL, ofl)

    write_manifest(mode, chars, files)
    print(f"total {sum(files.values()) / 1024 / 1024:.2f} MB")


def build_full() -> None:
    files: dict[str, int] = {}
    for name, url in [(OUT_REGULAR, SRC_REGULAR), (OUT_BOLD, SRC_BOLD)]:
        dest = FONTS / name
        download(url, dest)
        files[name] = dest.stat().st_size
    download(OFL_URL, FONTS / "OFL.txt")
    write_manifest("noto-cjk-sc-regional", i18n_chars(), files)


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--full", action="store_true", help="full SC regional (~16 MB)")
    p.add_argument("--regular-only", action="store_true", help="one weight only (~0.9 MB)")
    args = p.parse_args()

    if args.full:
        build_full()
    else:
        build_subset(regular_only=args.regular_only)
    print(f"wrote {MANIFEST}")


if __name__ == "__main__":
    main()
