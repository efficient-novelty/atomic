#!/usr/bin/env python3
"""Verify and optionally isolate-build the oracle-free law-v2 closure."""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib


REPO = pathlib.Path(__file__).resolve().parents[1]
ISOLATION_LOCKFILE = REPO / "scripts" / "law-v2-isolation.Cargo.lock"
PRODUCTION_ROOTS = ("pen-law", "pen-engine", "pen-gf2", "pen-gf2-agda")
ALLOWED_WORKSPACE_PACKAGES = frozenset(
    (*PRODUCTION_ROOTS, "pen-kernel", "pen-demand")
)
FORBIDDEN_DEPENDENCIES = frozenset(
    {
        "pen-oracle",
        "pen-search",
        "pen-eval",
        "pen-schema",
        "pen-type",
        "pen-cli",
        "pen-agda",
        "pen-store",
    }
)
FORBIDDEN_SOURCE_FRAGMENTS = (
    "telescope::reference",
    "all_reference_telescopes",
    "until_step",
    "minimal_positive_overshoot",
    "structuraldebt",
    "structuralfamily",
    "expected_kappa",
    "expected_nu",
    "expected_trace",
    "expected_hash",
    "accepted_trace",
    "future_viability",
    "stage_index",
    "step_index",
    "target_stage",
    "target_length",
    "stage4",
    "stage_4",
    "stage-4",
    "step4",
    "step_4",
    "step-4",
    "requires_temporal_shell_package",
    "circle",
    "hopf",
    "cohesion",
    "curvature",
    "hilbert",
    "dct",
)
FORBIDDEN_SOURCE_PATTERNS = (
    (re.compile(rb"(?<![a-z0-9_])15(?![a-z0-9_])"), "literal target count 15"),
    (
        re.compile(rb"\b(if|match)\s+[^{}\r\n]*(stage|step)", re.IGNORECASE),
        "stage-index control flow",
    ),
)
ISOLATION_TEST_COMMAND = (
    "cargo",
    "test",
    "--locked",
    "--workspace",
    "--all-targets",
)
ISOLATION_RUN_COMMAND = (
    "cargo",
    "run",
    "--locked",
    "--quiet",
    "-p",
    "pen-engine",
    "--bin",
    "pen-law-v2",
)


def load_toml(path: pathlib.Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def workspace_packages() -> dict[str, pathlib.Path]:
    root = load_toml(REPO / "Cargo.toml")
    packages: dict[str, pathlib.Path] = {}
    for member in root["workspace"]["members"]:
        manifest = REPO / member / "Cargo.toml"
        if not manifest.is_file():
            continue
        package = load_toml(manifest)["package"]["name"]
        packages[package] = manifest
    return packages


def dependency_names(manifest: pathlib.Path) -> set[str]:
    parsed = load_toml(manifest)
    names: set[str] = set()

    def visit(value: object) -> None:
        if not isinstance(value, dict):
            return
        for key, nested in value.items():
            if key in ("dependencies", "build-dependencies") and isinstance(nested, dict):
                for declared, declaration in nested.items():
                    if isinstance(declaration, dict):
                        names.add(declaration.get("package", declared))
                    else:
                        names.add(declared)
            else:
                visit(nested)

    visit(parsed)
    return names


def dependency_closure(packages: dict[str, pathlib.Path]) -> set[str]:
    closure: set[str] = set()
    frontier = list(PRODUCTION_ROOTS)
    while frontier:
        package = frontier.pop()
        if package in closure:
            continue
        if package not in packages:
            raise RuntimeError(f"production package {package!r} is not a workspace member")
        closure.add(package)
        for dependency in dependency_names(packages[package]):
            if dependency in packages:
                frontier.append(dependency)
    return closure


def source_violations(packages: dict[str, pathlib.Path], closure: set[str]) -> list[str]:
    violations: list[str] = []
    for package in sorted(closure):
        crate = packages[package].parent
        metadata = load_toml(packages[package]).get("package", {}).get("metadata", {})
        law_metadata = metadata.get("law-v2", {})
        if law_metadata.get("role") != "production":
            violations.append(f"{crate.name}: missing law-v2 production role")
        if law_metadata.get("oracle_access") is not False:
            violations.append(f"{crate.name}: oracle_access must be false")
        if law_metadata.get("diagnostic_access") is not False:
            violations.append(f"{crate.name}: diagnostic_access must be false")

        for path in sorted(candidate_input_files(crate)):
            raw = path.read_bytes()
            relative = path.relative_to(REPO)
            violations.extend(
                f"{relative}: {violation}" for violation in byte_violations(raw)
            )
    return violations


def candidate_input_files(crate: pathlib.Path) -> set[pathlib.Path]:
    """Return every local file that can plausibly affect a package build."""
    ignored_parts = {"target", ".git", "__pycache__"}
    files = {
        path
        for path in crate.rglob("*")
        if path.is_file() and not any(part in ignored_parts for part in path.parts)
    }
    manifest = crate / "Cargo.toml"
    files.add(manifest)
    build_script = crate / "build.rs"
    if build_script.is_file():
        files.add(build_script)
    return files


def byte_violations(raw: bytes) -> list[str]:
    lowered = raw.lower()
    violations = [
        f"forbidden fragment {fragment!r}"
        for fragment in FORBIDDEN_SOURCE_FRAGMENTS
        if fragment.encode("ascii") in lowered
    ]
    violations.extend(
        f"forbidden pattern {label!r}"
        for pattern, label in FORBIDDEN_SOURCE_PATTERNS
        if pattern.search(lowered)
    )
    return violations


def isolated_build(packages: dict[str, pathlib.Path], closure: set[str]) -> None:
    with tempfile.TemporaryDirectory(prefix="pen-law-v2-firewall-") as temporary:
        isolated = pathlib.Path(temporary)
        members: list[str] = []
        for package in sorted(closure):
            source = packages[package].parent
            target = isolated / "crates" / package
            shutil.copytree(source, target)
            members.append(f"crates/{package}")

        root = load_toml(REPO / "Cargo.toml")
        workspace_package = root["workspace"]["package"]
        workspace_dependencies = root["workspace"]["dependencies"]
        lines = [
            "[workspace]",
            "resolver = \"2\"",
            "members = [" + ", ".join(json.dumps(member) for member in members) + "]",
            "",
            "[workspace.package]",
        ]
        for key, value in workspace_package.items():
            lines.append(f"{key} = {json.dumps(value)}")
        lines.extend(("", "[workspace.dependencies]"))
        for key in ("blake3", "schemars", "serde", "serde_json", "thiserror"):
            value = workspace_dependencies[key]
            if isinstance(value, str):
                lines.append(f"{key} = {json.dumps(value)}")
            else:
                fields = ", ".join(
                    f"{field} = {json.dumps(setting)}" for field, setting in value.items()
                )
                lines.append(f"{key} = {{ {fields} }}")
        (isolated / "Cargo.toml").write_text("\n".join(lines) + "\n", encoding="utf-8")
        # The full-workspace lock contains non-lawful workspace members. Cargo
        # would have to prune those members in this reduced workspace, so the
        # isolation lane has its own reviewed lock and can run with --locked.
        shutil.copy2(ISOLATION_LOCKFILE, isolated / "Cargo.lock")
        # Production provenance embeds the reviewed isolation lock at its
        # repository-relative path as well as using it as the temporary
        # workspace lock.
        (isolated / "scripts").mkdir()
        shutil.copy2(
            ISOLATION_LOCKFILE,
            isolated / "scripts" / ISOLATION_LOCKFILE.name,
        )
        shutil.copy2(REPO / "rust-toolchain.toml", isolated / "rust-toolchain.toml")
        (isolated / ".cargo").mkdir()
        shutil.copy2(REPO / ".cargo" / "config.toml", isolated / ".cargo" / "config.toml")

        subprocess.run(
            ISOLATION_TEST_COMMAND,
            cwd=isolated,
            check=True,
        )
        completed = subprocess.run(
            ISOLATION_RUN_COMMAND,
            cwd=isolated,
            check=True,
            capture_output=True,
            text=True,
        )
        outcome = json.loads(completed.stdout)
        if outcome.get("status") != "unknown":
            raise RuntimeError("firewall milestone must fail closed with status 'unknown'")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--isolation-build",
        action="store_true",
        help="copy only the lawful closure to a temporary workspace and test it",
    )
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()

    packages = workspace_packages()
    closure = dependency_closure(packages)
    violations = source_violations(packages, closure)
    forbidden = sorted(closure & FORBIDDEN_DEPENDENCIES)
    unexpected = sorted(closure - ALLOWED_WORKSPACE_PACKAGES)
    if forbidden:
        violations.append(f"forbidden production dependencies: {', '.join(forbidden)}")
    if unexpected:
        violations.append(f"unreviewed production dependencies: {', '.join(unexpected)}")

    if not violations and args.isolation_build:
        isolated_build(packages, closure)

    report = {
        "status": "pass" if not violations else "fail",
        "production_roots": list(PRODUCTION_ROOTS),
        "workspace_dependency_closure": sorted(closure),
        "violations": violations,
        "isolation_build": bool(args.isolation_build and not violations),
    }
    if args.json:
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        print(f"law-v2 oracle firewall: {report['status']}")
        print("closure:", ", ".join(report["workspace_dependency_closure"]))
        for violation in violations:
            print("violation:", violation)
    return 0 if not violations else 1


if __name__ == "__main__":
    sys.exit(main())
