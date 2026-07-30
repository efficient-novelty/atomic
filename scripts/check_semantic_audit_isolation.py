#!/usr/bin/env python3
"""Fail-closed isolation check for the proposed semantic-audit workspace."""

from __future__ import annotations

import argparse
import json
import pathlib
import sys
import tomllib


REPO = pathlib.Path(__file__).resolve().parents[1]
ROOT_MANIFEST = REPO / "Cargo.toml"
ROOT_LOCK = REPO / "Cargo.lock"
AUDIT_ROOT = REPO / "crates" / "pen-semantic-audit"
AUDIT_MANIFEST = AUDIT_ROOT / "Cargo.toml"
AUDIT_LOCK = AUDIT_ROOT / "Cargo.lock"

ALLOWED_DEPENDENCIES = {
    "pen-kernel",
    "pen-kernel-synthesis",
    "serde",
    "thiserror",
}
FORBIDDEN_SOURCE_MARKERS = {
    "law_v2_h3_inductive_completion_v1",
    "law_v2_h4_continuation_v1",
    "law_v2a_registered_bootstrap",
    "pen_oracle",
    "pen-oracle",
    "archived semantic vector",
}


def load_toml(path: pathlib.Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def audit() -> dict:
    errors: list[str] = []
    if not AUDIT_MANIFEST.is_file():
        return {"status": "invalid", "errors": ["missing nested audit manifest"]}
    if not AUDIT_LOCK.is_file():
        errors.append("missing nested audit lockfile")

    root_manifest = load_toml(ROOT_MANIFEST)
    root_members = root_manifest.get("workspace", {}).get("members", [])
    if "crates/pen-semantic-audit" in root_members:
        errors.append("semantic-audit crate must not be a root workspace member")

    root_lock_text = ROOT_LOCK.read_text(encoding="utf-8")
    if 'name = "pen-semantic-audit"' in root_lock_text:
        errors.append("root lockfile must not contain pen-semantic-audit")

    manifest = load_toml(AUDIT_MANIFEST)
    if "workspace" not in manifest:
        errors.append("audit manifest must declare its own nested workspace")
    package = manifest.get("package", {})
    if package.get("name") != "pen-semantic-audit":
        errors.append("unexpected audit package name")
    metadata = package.get("metadata", {}).get("law-v2", {})
    if metadata.get("role") != "proposed-generic-audit":
        errors.append("audit package must remain proposed-generic-audit")
    if metadata.get("live_profile_a_access") is not False:
        errors.append("live Profile A access must be explicitly false")
    if metadata.get("oracle_access") is not False:
        errors.append("oracle access must be explicitly false")

    dependencies = set(manifest.get("dependencies", {}))
    unexpected = sorted(dependencies - ALLOWED_DEPENDENCIES)
    if unexpected:
        errors.append(f"unexpected dependencies: {', '.join(unexpected)}")
    kernel = manifest.get("dependencies", {}).get("pen-kernel")
    if not isinstance(kernel, dict) or kernel.get("path") != "../pen-kernel":
        errors.append("pen-kernel must be the exact outgoing ../pen-kernel path dependency")
    synthesis = manifest.get("dependencies", {}).get("pen-kernel-synthesis")
    if (
        not isinstance(synthesis, dict)
        or synthesis.get("path") != "../pen-kernel-synthesis"
    ):
        errors.append(
            "pen-kernel-synthesis must be the exact outgoing "
            "../pen-kernel-synthesis path dependency"
        )

    source_files = sorted((AUDIT_ROOT / "src").glob("*.rs"))
    if not source_files:
        errors.append("audit crate has no Rust source files")
    for source in source_files:
        text = source.read_text(encoding="utf-8").lower()
        for marker in FORBIDDEN_SOURCE_MARKERS:
            if marker in text:
                errors.append(f"{source.relative_to(REPO)} contains forbidden marker {marker!r}")

    return {
        "status": "valid" if not errors else "invalid",
        "root_workspace_member": False if not errors else None,
        "nested_lockfile": AUDIT_LOCK.is_file(),
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
        print("semantic-audit isolation: valid")
    else:
        for error in result["errors"]:
            print(f"semantic-audit isolation: {error}", file=sys.stderr)
    return 0 if result["status"] == "valid" else 1


if __name__ == "__main__":
    raise SystemExit(main())
