#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import shlex
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Sequence


ASSET_NAME_RE = re.compile(r"^Pauza_(?P<version>\d+\.\d+\.\d+)_aarch64\.dmg$")
TARGETS_URL_RE = re.compile(
    r"fallbackUrl:\s*'https://github\.com/[^']+/releases/download/v[^']+/Pauza_[^']+_aarch64\.dmg'"
)
INDEX_URL_RE = re.compile(
    r'(<a[\s\S]*?class="download-button"[\s\S]*?href=")https://github\.com/[^"]+/releases/download/v[^"]+/Pauza_[^"]+_aarch64\.dmg(")',
    re.MULTILINE,
)


@dataclass(frozen=True)
class ReleaseContext:
    asset_path: Path
    asset_name: str
    version: str
    tag: str
    title: str
    notes: str
    repo: str
    target: str
    site_root: Path

    @property
    def release_url(self) -> str:
        return f"https://github.com/{self.repo}/releases/download/{self.tag}/{self.asset_name}"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Update site pinned download URLs and create/upload a GitHub release asset."
    )
    parser.add_argument("--asset", required=True, help="Absolute or relative path to Pauza_<version>_aarch64.dmg")
    parser.add_argument("--repo", default="IHKYoung/Pauza", help="GitHub repo in owner/name format")
    parser.add_argument("--target", default="baseline", help="Git target for a new release tag")
    parser.add_argument("--tag", default="", help="Explicit release tag, default derives from asset version")
    parser.add_argument("--title", default="", help="Explicit release title, default matches tag")
    parser.add_argument("--notes", default="", help="Inline release notes")
    parser.add_argument("--notes-file", default="", help="Path to a markdown file for release notes")
    parser.add_argument("--site-root", default=".", help="Site repo root, defaults to current directory")
    parser.add_argument("--skip-site-update", action="store_true", help="Do not touch index.html or download/targets.js")
    parser.add_argument("--skip-release", action="store_true", help="Do not call gh release create/upload")
    parser.add_argument("--dry-run", action="store_true", help="Print intended changes and commands without executing them")
    return parser.parse_args()


def build_context(args: argparse.Namespace) -> ReleaseContext:
    asset_path = Path(args.asset).expanduser().resolve()
    if not asset_path.is_file():
        raise SystemExit(f"Asset not found: {asset_path}")

    match = ASSET_NAME_RE.match(asset_path.name)
    if not match:
        raise SystemExit(
            "Asset name must match Pauza_<version>_aarch64.dmg, "
            f"got: {asset_path.name}"
        )

    version = match.group("version")
    tag = args.tag.strip() or f"v{version}"
    title = args.title.strip() or tag
    notes = args.notes
    if args.notes_file:
        notes = Path(args.notes_file).expanduser().read_text(encoding="utf-8")
    if not notes.strip():
        notes = f"Pauza {version} release"

    return ReleaseContext(
        asset_path=asset_path,
        asset_name=asset_path.name,
        version=version,
        tag=tag,
        title=title,
        notes=notes,
        repo=args.repo.strip(),
        target=args.target.strip(),
        site_root=Path(args.site_root).expanduser().resolve(),
    )


def run_command(cmd: Sequence[str], *, dry_run: bool, check: bool = True) -> subprocess.CompletedProcess[str]:
    print(f"$ {shlex.join(cmd)}")
    if dry_run:
        return subprocess.CompletedProcess(cmd, 0, "", "")

    result = subprocess.run(cmd, text=True, capture_output=True, check=False)
    if result.stdout.strip():
        print(result.stdout.strip())
    if result.returncode != 0 and check:
        detail = result.stderr.strip() or result.stdout.strip() or "command failed"
        raise SystemExit(detail)
    return result


def require_gh_auth(dry_run: bool) -> None:
    status = run_command(["gh", "auth", "status"], dry_run=dry_run, check=False)
    if dry_run:
        return
    if status.returncode != 0:
        detail = status.stderr.strip() or status.stdout.strip() or "gh auth status failed"
        raise SystemExit(f"`gh` is not ready: {detail}")


def update_targets_file(ctx: ReleaseContext, dry_run: bool) -> bool:
    path = ctx.site_root / "download" / "targets.js"
    text = path.read_text(encoding="utf-8")
    replacement = f"fallbackUrl: '{ctx.release_url}'"
    updated, count = TARGETS_URL_RE.subn(replacement, text, count=1)
    if count != 1:
        raise SystemExit(f"Could not update fallbackUrl in {path}")
    if updated == text:
        print(f"[OK] {path.relative_to(ctx.site_root)} already points to {ctx.tag}")
        return False
    print(f"[UPDATE] {path.relative_to(ctx.site_root)} -> {ctx.release_url}")
    if not dry_run:
        path.write_text(updated, encoding="utf-8")
    return True


def update_index_file(ctx: ReleaseContext, dry_run: bool) -> bool:
    path = ctx.site_root / "index.html"
    text = path.read_text(encoding="utf-8")
    updated, count = INDEX_URL_RE.subn(rf"\1{ctx.release_url}\2", text, count=1)
    if count != 1:
        raise SystemExit(f"Could not update download button href in {path}")
    if updated == text:
        print(f"[OK] {path.relative_to(ctx.site_root)} already points to {ctx.tag}")
        return False
    print(f"[UPDATE] {path.relative_to(ctx.site_root)} -> {ctx.release_url}")
    if not dry_run:
        path.write_text(updated, encoding="utf-8")
    return True


def update_site_files(ctx: ReleaseContext, dry_run: bool) -> None:
    changed = False
    changed |= update_targets_file(ctx, dry_run)
    changed |= update_index_file(ctx, dry_run)
    if not changed:
        print("[OK] Site pinned release URLs are already up to date.")


def release_exists(ctx: ReleaseContext, dry_run: bool) -> bool:
    result = run_command(
        ["gh", "release", "view", ctx.tag, "--repo", ctx.repo],
        dry_run=dry_run,
        check=False,
    )
    return result.returncode == 0


def publish_release(ctx: ReleaseContext, dry_run: bool) -> None:
    require_gh_auth(dry_run)
    if release_exists(ctx, dry_run):
        print(f"[INFO] Release {ctx.tag} already exists; uploading asset with --clobber.")
        run_command(
            [
                "gh",
                "release",
                "upload",
                ctx.tag,
                str(ctx.asset_path),
                "--repo",
                ctx.repo,
                "--clobber",
            ],
            dry_run=dry_run,
        )
        return

    run_command(
        [
            "gh",
            "release",
            "create",
            ctx.tag,
            str(ctx.asset_path),
            "--repo",
            ctx.repo,
            "--title",
            ctx.title,
            "--notes",
            ctx.notes,
            "--target",
            ctx.target,
        ],
        dry_run=dry_run,
    )


def main() -> int:
    args = parse_args()
    ctx = build_context(args)

    print(f"[INFO] asset   : {ctx.asset_path}")
    print(f"[INFO] version : {ctx.version}")
    print(f"[INFO] tag     : {ctx.tag}")
    print(f"[INFO] url     : {ctx.release_url}")

    if not args.skip_site_update:
        update_site_files(ctx, args.dry_run)
    else:
        print("[SKIP] Site pinned URL update skipped.")

    if not args.skip_release:
        publish_release(ctx, args.dry_run)
    else:
        print("[SKIP] GitHub release create/upload skipped.")

    if args.dry_run:
        print("[DONE] Dry run completed.")
    else:
        print("[DONE] Release workflow completed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
