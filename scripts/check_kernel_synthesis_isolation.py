#!/usr/bin/env python3
"""Fail-closed isolation check for the proof-carrying synthesis successor."""

from __future__ import annotations

import argparse
import json
import pathlib
import sys
import tomllib


REPO = pathlib.Path(__file__).resolve().parents[1]
ROOT_MANIFEST = REPO / "Cargo.toml"
ROOT_LOCK = REPO / "Cargo.lock"
SYNTHESIS_ROOT = REPO / "crates" / "pen-kernel-synthesis"
SYNTHESIS_MANIFEST = SYNTHESIS_ROOT / "Cargo.toml"
SYNTHESIS_LOCK = SYNTHESIS_ROOT / "Cargo.lock"

ALLOWED_DEPENDENCIES = {"pen-kernel", "thiserror"}
FORBIDDEN_SOURCE_MARKERS = {
    "law_v2_h3_inductive_completion_v1",
    "law_v2_h4_continuation_v1",
    "law_v2a_registered_bootstrap",
    "pen-semantic-audit",
    "pen_oracle",
    "pen-oracle",
    "archived semantic vector",
}


def load_toml(path: pathlib.Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def audit() -> dict:
    errors: list[str] = []
    if not SYNTHESIS_MANIFEST.is_file():
        return {"status": "invalid", "errors": ["missing synthesis manifest"]}
    if not SYNTHESIS_LOCK.is_file():
        errors.append("missing synthesis lockfile")

    root_manifest = load_toml(ROOT_MANIFEST)
    root_members = root_manifest.get("workspace", {}).get("members", [])
    if "crates/pen-kernel-synthesis" in root_members:
        errors.append("kernel-synthesis crate must not be a root workspace member")
    if 'name = "pen-kernel-synthesis"' in ROOT_LOCK.read_text(encoding="utf-8"):
        errors.append("root lockfile must not contain pen-kernel-synthesis")

    manifest = load_toml(SYNTHESIS_MANIFEST)
    if "workspace" not in manifest:
        errors.append("synthesis manifest must declare its own nested workspace")
    package = manifest.get("package", {})
    if package.get("name") != "pen-kernel-synthesis":
        errors.append("unexpected synthesis package name")
    metadata = package.get("metadata", {}).get("law-v2", {})
    required_metadata = {
        "role": "proposed-kernel-synthesis-successor",
        "oracle_access": False,
        "diagnostic_access": False,
        "live_profile_a_access": False,
    }
    for key, expected in required_metadata.items():
        if metadata.get(key) != expected:
            errors.append(f"synthesis metadata {key!r} must equal {expected!r}")

    dependencies = set(manifest.get("dependencies", {}))
    unexpected = sorted(dependencies - ALLOWED_DEPENDENCIES)
    if unexpected:
        errors.append(f"unexpected dependencies: {', '.join(unexpected)}")
    kernel = manifest.get("dependencies", {}).get("pen-kernel")
    if not isinstance(kernel, dict) or kernel.get("path") != "../pen-kernel":
        errors.append("pen-kernel must be the exact ../pen-kernel path dependency")

    source_files = sorted((SYNTHESIS_ROOT / "src").glob("*.rs"))
    if not source_files:
        errors.append("synthesis crate has no Rust source files")
    for source in source_files:
        text = source.read_text(encoding="utf-8").lower()
        for marker in FORBIDDEN_SOURCE_MARKERS:
            if marker in text:
                errors.append(
                    f"{source.relative_to(REPO)} contains forbidden marker {marker!r}"
                )

    return {
        "status": "valid" if not errors else "invalid",
        "root_workspace_member": False if not errors else None,
        "nested_lockfile": SYNTHESIS_LOCK.is_file(),
        "source_file_count": len(source_files),
        "errors": errors,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()
    result = audit()
    if args.json:
        print(json.dumps(result, sort_keys=True))
    elif result["status"] == "valid":
        print("kernel-synthesis isolation: valid")
    else:
        for error in result["errors"]:
            print(f"kernel-synthesis isolation: {error}", file=sys.stderr)
    return 0 if result["status"] == "valid" else 1


if __name__ == "__main__":
    raise SystemExit(main())
