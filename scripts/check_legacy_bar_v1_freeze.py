#!/usr/bin/env python3
"""Replay the byte-level bindings in the partial legacy testimony freeze."""

from __future__ import annotations

import hashlib
import json
import pathlib
import subprocess
import sys
import tomllib


REPO = pathlib.Path(__file__).resolve().parents[1]
MANIFEST_PATH = REPO / "configs" / "legacy_bar_v1.freeze.json"


def sha256(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    manifest = json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))
    violations: list[str] = []

    bindings = {
        manifest["historical_law"]["path"]: manifest["historical_law"]["sha256"],
        manifest["profile"]["path"]: manifest["profile"]["sha256"],
        manifest["oracle_quarantine"]["path"]: manifest["oracle_quarantine"]["sha256"],
        manifest["oracle_quarantine"]["reference_trajectory_path"]: manifest[
            "oracle_quarantine"
        ]["reference_trajectory_sha256"],
    }
    for relative, expected in bindings.items():
        path = REPO / relative
        if not path.is_file():
            violations.append(f"missing bound input: {relative}")
            continue
        actual = sha256(path)
        if actual != expected:
            violations.append(f"digest drift: {relative}: expected {expected}, found {actual}")

    profile_path = REPO / manifest["profile"]["path"]
    if profile_path.is_file():
        with profile_path.open("rb") as handle:
            profile = tomllib.load(handle)
        expected_fields = {
            ("mode", "search_profile"): manifest["profile"]["search_profile"],
            ("search", "until_step"): manifest["profile"]["configured_endpoint"],
            ("objective", "window_depth"): manifest["profile"]["window_depth"],
            ("objective", "selector"): manifest["profile"]["selector"],
        }
        for (section, field), expected in expected_fields.items():
            actual = profile.get(section, {}).get(field)
            if actual != expected:
                violations.append(
                    f"profile drift: {section}.{field}: expected {expected!r}, found {actual!r}"
                )

    source_commit = manifest["source"]["commit"]
    commit_exists = subprocess.run(
        ["git", "cat-file", "-e", f"{source_commit}^{{commit}}"],
        cwd=REPO,
        capture_output=True,
    )
    if commit_exists.returncode != 0:
        violations.append(f"missing frozen source commit: {source_commit}")
    else:
        for relative, expected in manifest["source"]["tracked_baseline_blobs"].items():
            completed = subprocess.run(
                ["git", "rev-parse", f"{source_commit}:{relative}"],
                cwd=REPO,
                capture_output=True,
                text=True,
            )
            if completed.returncode != 0:
                violations.append(f"missing frozen source blob: {relative}")
                continue
            actual = f"git-sha1:{completed.stdout.strip()}"
            if actual != expected:
                violations.append(
                    f"source blob drift: {relative}: expected {expected}, found {actual}"
                )

    report = {
        "status": "pass" if not violations else "fail",
        "freeze_status": manifest["status"],
        "manifest": str(MANIFEST_PATH.relative_to(REPO)),
        "bound_input_count": len(bindings),
        "violations": violations,
        "incomplete_requirements": manifest["incomplete_requirements"],
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if not violations else 1


if __name__ == "__main__":
    sys.exit(main())
