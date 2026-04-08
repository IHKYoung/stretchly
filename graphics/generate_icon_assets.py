#!/usr/bin/env python3

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
GRAPHICS = ROOT / "graphics"
TAURIICONS = ROOT / "apps" / "desktop" / "src-tauri" / "icons"
BUILD = ROOT / "build"
APPX = BUILD / "appx"
ELECTRON_ICONS = ROOT / "app" / "images" / "app-icons"

APP_ICON_SVG = GRAPHICS / "app-icon.svg"
TRAY_ICON_SVG = GRAPHICS / "tray-icon.svg"

SRGB_PROFILE = "/System/Library/ColorSync/Profiles/sRGB Profile.icc"


def run(cmd: list[str]) -> None:
    subprocess.run(cmd, check=True)


def render_svg(svg_path: Path, out_path: Path, size: int) -> None:
    run(["rsvg-convert", "-w", str(size), "-h", str(size), str(svg_path), "-o", str(out_path)])


def ensure_srgb(png_path: Path, out_path: Path | None = None) -> None:
    target = out_path or png_path
    srgb = Path(SRGB_PROFILE)
    if srgb.exists():
        run(["sips", "-m", str(srgb), str(png_path), "--out", str(target)])
    else:
        run(["magick", str(png_path), "-colorspace", "sRGB", "-type", "TrueColorAlpha", str(target)])


def ensure_rgba(src: Path, dst: Path) -> None:
    if src == dst:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmp = Path(tmpdir) / dst.name
            run([
                "ffmpeg", "-y", "-i", str(src), "-vf", "format=rgba", "-frames:v", "1", "-update", "1", "-f", "image2",
                "-vcodec", "png", str(tmp),
            ])
            shutil.copy2(tmp, dst)
        return

    run(["ffmpeg", "-y", "-i", str(src), "-vf", "format=rgba", "-frames:v", "1", "-update", "1", "-f", "image2", "-vcodec", "png", str(dst)])


def generate_packaging_icons() -> None:
    if not APP_ICON_SVG.exists():
        raise FileNotFoundError(f"Source SVG not found: {APP_ICON_SVG}")

    TAURIICONS.mkdir(parents=True, exist_ok=True)
    BUILD.mkdir(parents=True, exist_ok=True)
    APPX.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory() as tmpdir:
        master = Path(tmpdir) / "icon_1024.png"
        render_svg(APP_ICON_SVG, master, 1024)

        shutil.copy2(master, ROOT / "Pauza.png")
        ensure_srgb(ROOT / "Pauza.png")

        tauri_icon = TAURIICONS / "icon.png"
        shutil.copy2(master, tauri_icon)
        ensure_rgba(tauri_icon, tauri_icon)

        icon128 = Path(tmpdir) / "icon_128.png"
        run(["magick", str(master), "-colorspace", "sRGB", "-resize", "128x128", str(icon128)])
        shutil.copy2(icon128, TAURIICONS / "128x128.png")
        ensure_rgba(TAURIICONS / "128x128.png", TAURIICONS / "128x128.png")
        shutil.copy2(icon128, ROOT / "pauza_128x128.png")
        ensure_srgb(ROOT / "pauza_128x128.png")

        run([
            "magick", str(master), "-colorspace", "sRGB",
            "-define", "icon:auto-resize=256,128,64,48,32,16",
            str(BUILD / "icon.ico"),
        ])
        shutil.copy2(BUILD / "icon.ico", TAURIICONS / "icon.ico")

        iconset = Path(tmpdir) / "pauza.iconset"
        iconset.mkdir()
        icon_sizes = {
            "icon_16x16.png": "16x16",
            "icon_16x16@2x.png": "32x32",
            "icon_32x32.png": "32x32",
            "icon_32x32@2x.png": "64x64",
            "icon_128x128.png": "128x128",
            "icon_128x128@2x.png": "256x256",
            "icon_256x256.png": "256x256",
            "icon_256x256@2x.png": "512x512",
            "icon_512x512.png": "512x512",
            "icon_512x512@2x.png": "1024x1024",
        }
        for name, size in icon_sizes.items():
            run(["magick", str(master), "-colorspace", "sRGB", "-resize", size, str(iconset / name)])
        for png in iconset.glob("*.png"):
            ensure_srgb(png)
        run(["iconutil", "-c", "icns", str(iconset), "-o", str(BUILD / "icon.icns")])
        shutil.copy2(BUILD / "icon.icns", TAURIICONS / "icon.icns")

        run(["magick", str(master), "-colorspace", "sRGB", "-resize", "150x150", str(APPX / "Square150x150Logo.png")])
        run(["magick", str(master), "-colorspace", "sRGB", "-resize", "50x50", str(APPX / "StoreLogo.png")])
        run(["magick", str(master), "-colorspace", "sRGB", "-resize", "44x44", str(APPX / "Square44x44Logo.png")])
        run([
            "magick", "-size", "310x150", "xc:none",
            "(", str(master), "-resize", "120x120", ")",
            "-gravity", "center", "-compose", "over", "-composite",
            str(APPX / "Wide310x150Logo.png"),
        ])


def generate_tray_icons() -> None:
    if not TRAY_ICON_SVG.exists():
        raise FileNotFoundError(f"Tray SVG not found: {TRAY_ICON_SVG}")

    TAURIICONS.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory() as tmpdir:
        raw18 = Path(tmpdir) / "tray18_raw.png"
        raw36 = Path(tmpdir) / "tray36_raw.png"
        render_svg(TRAY_ICON_SVG, raw18, 18)
        render_svg(TRAY_ICON_SVG, raw36, 36)
        ensure_srgb(raw18, TAURIICONS / "tray-iconTemplate.png")
        ensure_srgb(raw36, TAURIICONS / "tray-iconTemplate@2x.png")


def generate_electron_tray_icons() -> None:
    if not TRAY_ICON_SVG.exists():
        raise FileNotFoundError(f"Tray SVG not found: {TRAY_ICON_SVG}")

    ELECTRON_ICONS.mkdir(parents=True, exist_ok=True)
    out = ELECTRON_ICONS

    with tempfile.TemporaryDirectory() as tmpdir:
        r16 = Path(tmpdir) / "tray16.png"
        r32 = Path(tmpdir) / "tray32.png"
        render_svg(TRAY_ICON_SVG, r16, 16)
        render_svg(TRAY_ICON_SVG, r32, 32)

        variants = {
            "trayMacMonochromeTemplate.png": (r16, None),
            "trayMacMonochromeTemplate@2x.png": (r32, None),
            "trayMacMonochromePausedTemplate.png": (r16, "0.4"),
            "trayMacMonochromePausedTemplate@2x.png": (r32, "0.4"),
            "trayMac.png": (r16, None),
            "trayMac@2x.png": (r32, None),
            "trayMacPaused.png": (r16, "0.4"),
            "trayMacPaused@2x.png": (r32, "0.4"),
            "tray.png": (r32, None),
            "trayPaused.png": (r32, "0.4"),
            "trayMonochrome.png": (r32, None),
            "trayMonochromePaused.png": (r32, "0.4"),
        }

        for name, (src, alpha) in variants.items():
            dst = out / name
            if alpha:
                run(["magick", str(src), "-channel", "A", "-evaluate", "Multiply", alpha, "+channel", str(dst)])
            else:
                shutil.copy2(src, dst)

        dark_variants = {
            "trayMacDark.png": (out / "trayMac.png", None),
            "trayMacDark@2x.png": (out / "trayMac@2x.png", None),
            "trayMacPausedDark.png": (out / "trayMac.png", "0.4"),
            "trayMacPausedDark@2x.png": (out / "trayMac@2x.png", "0.4"),
            "trayDark.png": (out / "tray.png", None),
            "trayPausedDark.png": (out / "tray.png", "0.4"),
            "trayMonochromeInverted.png": (out / "trayMonochrome.png", None),
            "trayMonochromeInvertedPaused.png": (out / "trayMonochrome.png", "0.4"),
        }

        for name, (src, alpha) in dark_variants.items():
            dst = out / name
            if alpha:
                run(["magick", str(src), "-channel", "RGB", "-negate", "+channel",
                     "-channel", "A", "-evaluate", "Multiply", alpha, "+channel", str(dst)])
            else:
                run(["magick", str(src), "-channel", "RGB", "-negate", "+channel", str(dst)])


def main() -> None:
    generate_packaging_icons()
    generate_tray_icons()
    generate_electron_tray_icons()
    print("generated icon assets")


if __name__ == "__main__":
    main()
