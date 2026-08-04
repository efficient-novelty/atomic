#!/usr/bin/env python3
"""Fail-closed isolation and kernel-lock checks for the generative lane."""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import subprocess
import sys
import tomllib
from typing import NamedTuple


REPO = pathlib.Path(__file__).resolve().parents[1]
ROOT_MANIFEST = REPO / "Cargo.toml"
ROOT_LOCK = REPO / "Cargo.lock"
KERNEL_MANIFEST = REPO / "crates" / "pen-kernel" / "Cargo.toml"
REVIEWED_GRAPH = (
    REPO / "crates" / "pen-kernel" / "production-dependency-graph.lock"
)

REVIEWED_GRAPH_SCHEMA = "schema\tpen-kernel-production-dependency-graph-v1"
SERDE_JSON_REVIEWED_PIN = "=1.0.149"

PackageId = tuple[str, str, str]
DependencyEdge = tuple[PackageId, PackageId]


class KernelGraph(NamedTuple):
    roots: frozenset[PackageId]
    packages: dict[PackageId, str]
    edges: frozenset[DependencyEdge]


FORBIDDEN_SOURCE_MARKERS = frozenset(
    {
        "pen_law",
        "pen-law",
        "pen_store",
        "pen-store",
        "pen_engine",
        "pen-engine",
        "pen_oracle",
        "pen-oracle",
        "pen_search",
        "pen-search",
        "pen_eval",
        "pen-eval",
        "pen_semantic_audit",
        "pen_contextual_completion",
        "pen-contextual-completion",
        "law_v2_h3_inductive_completion_v1",
        "law_v2_h4_continuation_v1",
        "law_v2a_registered_bootstrap",
        "telescope::reference",
        "all_reference_telescopes",
        "include!(",
        "include_bytes!(",
        "include_str!(",
        "#[path",
    }
)

PARTICULAR_OPEN_TYPED_SUBSTITUTION_SOURCE = "particular_open_typed_substitution.rs"

# JG2b2b1 is a history-independent, particular-substitution substrate.  Its
# only structural-occurrence dependency is the opaque JG2b2b0 grammar token;
# it must not consume a history/census authority or the unverified traversal
# surface.  These checks are deliberately scoped to the one b1 source module
# so the earlier history-replay modules can retain their required authorities.
JG2B2B1_ALLOWED_OCCURRENCE_IDENTIFIERS = frozenset(
    {
        "VerifiedGenerativeStructuralOccurrenceGrammarV1",
    }
)
JG2B2B1_OCCURRENCE_IDENTIFIER = re.compile(
    r"\b[A-Z][A-Za-z0-9_]*Occurrence[A-Za-z0-9_]*\b"
)
JG2B2B1_FORBIDDEN_AUTHORITY_MARKERS = frozenset(
    {
        "verifiedcompletetargetneutralgenerativehistorythroughheadv1",
        "verify_complete_target_neutral_generative_history_through_head_v1",
        "closedgenerativesealedlogv1",
        "verifiedcontiguousgenerativestagechainv1",
        "verifiedcontiguousgenerativestageeventv1",
        "verifiedgenerativedeclarationbirthv1",
        "generativehistoryeventidv1",
        "verifiedgenerativecapabilitystagesurfacev1",
        "complete_sealed_history",
        "contiguous_stage_chain",
        "linear_append_log",
        "stage_surface",
        "structural_occurrence",
        "proposedgenerativestructuraldeclarationtraversalv1",
        "proposed_generative_structural_declaration_traversal_v1",
        "generativestructuraltraversalnodev1",
    }
)
JG2B2B1_FORBIDDEN_STD_MODULES = ("fs", "net", "process", "env", "time")
JG2B2B1_STD_BRACED_IMPORT = re.compile(
    r"\buse\s+(?:::)?std\s*::\s*\{(?P<body>[^;]*)\}\s*;", re.DOTALL
)
JG2B2B1_STD_ALIAS = re.compile(
    r"\b(?:use\s+(?:::)?std|extern\s+crate\s+std)\s+as\b"
)
JG2B2B1_STD_GLOB = re.compile(r"\buse\s+(?:::)?std\s*::\s*\*")
JG2B2B1_CRATE_BRACED_IMPORT = re.compile(
    r"\buse\s+crate\s*::\s*\{(?P<body>[^;]*)\}\s*;", re.DOTALL
)
JG2B2B1_CRATE_ALIAS = re.compile(
    r"\b(?:use\s+crate|extern\s+crate\s+self)\s+as\b"
)
JG2B2B1_CRATE_GLOB = re.compile(r"\buse\s+crate\s*::\s*\*")
JG2B2B1_BRACED_SELF_ALIAS = re.compile(r"\bself\s+as\b")
JG2B2B1_BRACED_BARE_GLOB = re.compile(r"(?:^|,)\s*\*(?=\s*(?:,|$))")
JG2B2B1_TEST_MODULE = re.compile(
    r"(?m)^\s*#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*\r?\n\s*mod\s+tests\s*\{"
)
JG2B2B1_TEST_ONLY_OCCURRENCE_IDENTIFIERS = (
    "proposed_generative_structural_occurrence_grammar_v1",
    "verify_generative_structural_occurrence_grammar_v1",
)


def crate_specs() -> tuple[dict, ...]:
    return (
        {
            "name": "pen-sealed-history",
            "root": REPO / "crates" / "pen-sealed-history",
            "dependencies": {
                "pen-kernel": {"path": "../pen-kernel"},
            },
            "metadata": {
                "role": "jg2b1b0-target-neutral-sealed-log-protocol-grammar",
                "oracle_access": False,
                "diagnostic_access": False,
                "law_profile_access": False,
                "live_profile_access": False,
                "storage_access": False,
                "producer_authority": False,
                "history_authority": False,
                "eof_authority": False,
                "finalization_authority": False,
                "capability_carrier_authority": False,
                "generative_gain_authority": False,
                "selection_authority": False,
            },
        },
        {
            "name": "pen-generative-audit",
            "root": REPO / "crates" / "pen-generative-audit",
            "dependencies": {
                "pen-kernel": {"path": "../pen-kernel"},
                "pen-sealed-history": {"path": "../pen-sealed-history"},
            },
            "metadata": {
                "role": "jg1-jg2b2b1-pre-exposure-generative-capacity-foundations",
                "oracle_access": False,
                "diagnostic_access": False,
                "live_profile_access": False,
                "process_local_log_producer": True,
                "complete_through_producer_finalized_head_authority": True,
                "jg2b2_entry_protocol_grammar": True,
                "jg2b2b0_structural_occurrence_grammar": True,
                "derived_public_occurrence_authority": False,
                "particular_open_typed_substitution_authority": True,
                "complete_admissible_substitution_universe_authority": False,
                "full_kernel_typed_substitution_metatheory": False,
                "generic_identity_composition_lifting_authority": False,
                "generic_typing_equality_preservation_authority": False,
                "normalization_reindexing_compatibility_authority": False,
                "executable_indexed_interface_authority": False,
                "indexed_interface_functor_law_authority": False,
                "universal_naturality_authority": False,
                "globally_latest_history_authority": False,
                "actual_current_branch_authority": False,
                "eof_authority": False,
                "capability_carrier_authority": False,
                "generative_gain_authority": False,
                "selection_authority": False,
            },
        },
    )


def load_toml(path: pathlib.Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def parse_reviewed_graph(path: pathlib.Path = REVIEWED_GRAPH) -> KernelGraph:
    lines = path.read_text(encoding="utf-8").splitlines()
    if not lines or lines[0] != REVIEWED_GRAPH_SCHEMA:
        raise ValueError("unexpected reviewed kernel dependency-graph schema")

    roots: set[PackageId] = set()
    packages: dict[PackageId, str] = {}
    edges: set[DependencyEdge] = set()
    for line_number, line in enumerate(lines[1:], start=2):
        fields = line.split("\t")
        record = fields[0] if fields else ""
        if record == "root" and len(fields) == 4:
            identity = (fields[1], fields[2], fields[3])
            if identity in roots:
                raise ValueError(f"duplicate reviewed root at line {line_number}")
            roots.add(identity)
        elif record == "package" and len(fields) == 5:
            identity = (fields[1], fields[2], fields[3])
            if identity in packages:
                raise ValueError(f"duplicate reviewed package at line {line_number}")
            packages[identity] = fields[4]
        elif record == "dependency" and len(fields) == 7:
            edge = (
                (fields[1], fields[2], fields[3]),
                (fields[4], fields[5], fields[6]),
            )
            if edge in edges:
                raise ValueError(f"duplicate reviewed dependency at line {line_number}")
            edges.add(edge)
        else:
            raise ValueError(f"malformed reviewed graph record at line {line_number}")

    package_ids = set(packages)
    if not roots <= package_ids:
        raise ValueError("reviewed graph root is absent from its package set")
    edge_ids = {identity for edge in edges for identity in edge}
    if not edge_ids <= package_ids:
        raise ValueError("reviewed graph edge references an absent package")
    if not packages:
        raise ValueError("reviewed kernel dependency graph is empty")
    return KernelGraph(frozenset(roots), packages, frozenset(edges))


def _cargo_metadata(manifest: pathlib.Path) -> dict:
    command = (
        "cargo",
        "metadata",
        "--manifest-path",
        str(manifest),
        "--locked",
        "--format-version",
        "1",
    )
    try:
        completed = subprocess.run(
            command,
            cwd=REPO,
            capture_output=True,
            check=False,
            text=True,
            timeout=120,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        raise RuntimeError(f"cannot execute cargo metadata: {error}") from error
    if completed.returncode != 0:
        detail = completed.stderr.strip() or completed.stdout.strip()
        raise RuntimeError(f"cargo metadata --locked failed: {detail}")
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(f"cargo metadata returned invalid JSON: {error}") from error


def _lock_packages(lock: pathlib.Path) -> dict[PackageId, dict]:
    parsed = load_toml(lock)
    records = parsed.get("package")
    if not isinstance(records, list) or not records:
        raise ValueError(f"{lock} has no Cargo.lock package records")
    packages: dict[PackageId, dict] = {}
    for record in records:
        if not isinstance(record, dict):
            raise ValueError(f"{lock} contains a malformed package record")
        name = record.get("name")
        version = record.get("version")
        source = record.get("source", "")
        if not all(isinstance(value, str) for value in (name, version, source)):
            raise ValueError(f"{lock} contains an incomplete package identity")
        identity = (name, version, source)
        if identity in packages:
            raise ValueError(f"{lock} contains duplicate package {identity!r}")
        packages[identity] = record
    return packages


def _metadata_identity(package: dict) -> PackageId:
    name = package.get("name")
    version = package.get("version")
    source = package.get("source") or ""
    if not all(isinstance(value, str) for value in (name, version, source)):
        raise ValueError("cargo metadata contains an incomplete package identity")
    return (name, version, source)


def _is_normal_or_build_dependency(dependency: dict) -> bool:
    kinds = dependency.get("dep_kinds")
    if not isinstance(kinds, list) or not kinds:
        raise ValueError("cargo metadata dependency has no dependency kinds")
    for kind in kinds:
        if not isinstance(kind, dict) or "kind" not in kind:
            raise ValueError("cargo metadata dependency kind is malformed")
        if kind["kind"] in (None, "build"):
            return True
    return False


def resolved_kernel_graph(manifest: pathlib.Path, lock: pathlib.Path) -> KernelGraph:
    metadata = _cargo_metadata(manifest)
    package_records = metadata.get("packages")
    resolve = metadata.get("resolve")
    if not isinstance(package_records, list) or not isinstance(resolve, dict):
        raise ValueError("cargo metadata omitted packages or the resolved graph")

    packages_by_id: dict[str, dict] = {}
    for package in package_records:
        package_id = package.get("id") if isinstance(package, dict) else None
        if not isinstance(package_id, str) or package_id in packages_by_id:
            raise ValueError("cargo metadata contains a missing or duplicate package id")
        packages_by_id[package_id] = package

    node_records = resolve.get("nodes")
    if not isinstance(node_records, list):
        raise ValueError("cargo metadata omitted resolved dependency nodes")
    nodes_by_id: dict[str, dict] = {}
    for node in node_records:
        node_id = node.get("id") if isinstance(node, dict) else None
        if not isinstance(node_id, str) or node_id in nodes_by_id:
            raise ValueError("cargo metadata contains a missing or duplicate node id")
        nodes_by_id[node_id] = node

    expected_kernel_manifest = KERNEL_MANIFEST.resolve()
    kernel_ids = []
    for package_id, package in packages_by_id.items():
        manifest_path = package.get("manifest_path")
        if not isinstance(manifest_path, str):
            continue
        if pathlib.Path(manifest_path).resolve() == expected_kernel_manifest:
            kernel_ids.append(package_id)
    if len(kernel_ids) != 1:
        raise ValueError(
            f"expected one resolved local pen-kernel package, found {len(kernel_ids)}"
        )
    kernel_id = kernel_ids[0]
    if packages_by_id[kernel_id].get("name") != "pen-kernel":
        raise ValueError("the expected kernel path resolved to a different package")

    roots: set[PackageId] = set()
    edges: set[DependencyEdge] = set()
    visited = {kernel_id}
    frontier = [kernel_id]
    while frontier:
        parent_id = frontier.pop()
        node = nodes_by_id.get(parent_id)
        if node is None:
            raise ValueError(f"resolved package {parent_id!r} has no dependency node")
        dependencies = node.get("deps")
        if not isinstance(dependencies, list):
            raise ValueError(f"resolved package {parent_id!r} has malformed dependencies")
        parent_identity = _metadata_identity(packages_by_id[parent_id])
        for dependency in dependencies:
            if not isinstance(dependency, dict):
                raise ValueError("cargo metadata contains a malformed dependency")
            if not _is_normal_or_build_dependency(dependency):
                continue
            child_id = dependency.get("pkg")
            if not isinstance(child_id, str) or child_id not in packages_by_id:
                raise ValueError("cargo metadata dependency has no resolved package")
            child_identity = _metadata_identity(packages_by_id[child_id])
            if parent_id == kernel_id:
                roots.add(child_identity)
            else:
                edges.add((parent_identity, child_identity))
            if child_id not in visited:
                visited.add(child_id)
                frontier.append(child_id)

    locked = _lock_packages(lock)
    graph_packages: dict[PackageId, str] = {}
    for package_id in visited - {kernel_id}:
        identity = _metadata_identity(packages_by_id[package_id])
        record = locked.get(identity)
        if record is None:
            raise ValueError(f"resolved package {identity!r} is absent from {lock}")
        checksum = record.get("checksum")
        if not isinstance(checksum, str):
            checksum = "<missing>"
        graph_packages[identity] = checksum

    return KernelGraph(frozenset(roots), graph_packages, frozenset(edges))


def _format_identity(identity: PackageId) -> str:
    return f"{identity[0]} {identity[1]} ({identity[2] or 'local'})"


def graph_difference_errors(
    label: str, actual: KernelGraph, reviewed: KernelGraph
) -> list[str]:
    errors: list[str] = []
    missing_roots = sorted(reviewed.roots - actual.roots)
    extra_roots = sorted(actual.roots - reviewed.roots)
    missing_packages = sorted(set(reviewed.packages) - set(actual.packages))
    extra_packages = sorted(set(actual.packages) - set(reviewed.packages))
    missing_edges = sorted(reviewed.edges - actual.edges)
    extra_edges = sorted(actual.edges - reviewed.edges)

    for identity in missing_roots:
        errors.append(f"{label}: missing reviewed kernel root {_format_identity(identity)}")
    for identity in extra_roots:
        errors.append(f"{label}: unexpected kernel root {_format_identity(identity)}")
    for identity in missing_packages:
        errors.append(
            f"{label}: missing reviewed kernel package {_format_identity(identity)}"
        )
    for identity in extra_packages:
        errors.append(
            f"{label}: unexpected kernel package {_format_identity(identity)}"
        )
    for identity in sorted(set(reviewed.packages) & set(actual.packages)):
        if actual.packages[identity] != reviewed.packages[identity]:
            errors.append(
                f"{label}: checksum mismatch for {_format_identity(identity)}"
            )
    for parent, child in missing_edges:
        errors.append(
            f"{label}: missing reviewed kernel edge "
            f"{_format_identity(parent)} -> {_format_identity(child)}"
        )
    for parent, child in extra_edges:
        errors.append(
            f"{label}: unexpected kernel edge "
            f"{_format_identity(parent)} -> {_format_identity(child)}"
        )
    return errors


def validate_manifest(spec: dict, manifest: dict) -> list[str]:
    name = spec["name"]
    errors: list[str] = []
    workspace = manifest.get("workspace")
    if not isinstance(workspace, dict):
        errors.append(f"{name}: manifest must declare its own nested workspace")
    elif workspace.get("resolver") != "2":
        errors.append(f"{name}: nested workspace resolver must equal '2'")

    package = manifest.get("package")
    if not isinstance(package, dict) or package.get("name") != name:
        errors.append(f"{name}: unexpected package identity")
        package = {} if not isinstance(package, dict) else package
    if "build" in package or "links" in package:
        errors.append(f"{name}: build-script authority is forbidden")

    metadata = package.get("metadata", {}).get("law-v2", {})
    expected_metadata = spec["metadata"]
    if not isinstance(metadata, dict):
        errors.append(f"{name}: missing law-v2 metadata")
    else:
        for key, expected in expected_metadata.items():
            if metadata.get(key) != expected:
                errors.append(f"{name}: metadata {key!r} must equal {expected!r}")
        unexpected_metadata = sorted(set(metadata) - set(expected_metadata))
        if unexpected_metadata:
            errors.append(
                f"{name}: unexpected law-v2 metadata: "
                f"{', '.join(unexpected_metadata)}"
            )

    dependencies = manifest.get("dependencies", {})
    if dependencies != spec["dependencies"]:
        errors.append(f"{name}: production dependencies are not the exact allowlist")
    if manifest.get("build-dependencies", {}) != {}:
        errors.append(f"{name}: build dependencies are forbidden")
    if manifest.get("dev-dependencies", {}) != {
        "serde_json": SERDE_JSON_REVIEWED_PIN
    }:
        errors.append(
            f"{name}: dev dependencies must contain only the reviewed serde_json pin"
        )
    for forbidden_table in ("patch", "replace"):
        if manifest.get(forbidden_table):
            errors.append(f"{name}: [{forbidden_table}] overrides are forbidden")

    targets = manifest.get("target", {})
    if not isinstance(targets, dict):
        errors.append(f"{name}: malformed target dependency table")
    else:
        for target, table in targets.items():
            if not isinstance(table, dict):
                errors.append(f"{name}: malformed target table {target!r}")
                continue
            for dependency_kind in (
                "dependencies",
                "build-dependencies",
                "dev-dependencies",
            ):
                if table.get(dependency_kind):
                    errors.append(
                        f"{name}: target-specific {dependency_kind} are forbidden"
                    )
    return errors


def _source_files(crate_root: pathlib.Path) -> list[pathlib.Path]:
    files: set[pathlib.Path] = set()
    for directory in ("src", "tests", "benches", "examples"):
        root = crate_root / directory
        if root.is_dir():
            files.update(root.rglob("*.rs"))
    build_script = crate_root / "build.rs"
    if build_script.exists():
        files.add(build_script)
    return sorted(files)


def validate_particular_open_typed_substitution_source(
    name: str, source: pathlib.Path, text: str
) -> list[str]:
    """Enforce the narrower authority boundary of the JG2b2b1 module."""

    errors: list[str] = []
    test_module = JG2B2B1_TEST_MODULE.search(text)
    # Unit tests must mint the opaque b0 prerequisite in order to exercise b1.
    # Remove only those two exact bootstrap helper names after the cfg(test)
    # boundary. Do not discard the suffix: a later production item must remain
    # visible to the authority denylist.
    authority_text = text
    if test_module is not None:
        test_suffix = text[test_module.start() :]
        for identifier in JG2B2B1_TEST_ONLY_OCCURRENCE_IDENTIFIERS:
            test_suffix = re.sub(rf"\b{re.escape(identifier)}\b", "", test_suffix)
        authority_text = text[: test_module.start()] + test_suffix
    lowered = authority_text.lower()
    std_braced_imports = tuple(JG2B2B1_STD_BRACED_IMPORT.finditer(text))
    crate_braced_imports = tuple(JG2B2B1_CRATE_BRACED_IMPORT.finditer(text))
    for marker in sorted(JG2B2B1_FORBIDDEN_AUTHORITY_MARKERS):
        if marker in lowered:
            errors.append(
                f"{name}: {source} contains forbidden JG2b2b1 authority marker "
                f"{marker!r}"
            )

    for match in JG2B2B1_OCCURRENCE_IDENTIFIER.finditer(authority_text):
        identifier = match.group(0)
        if identifier not in JG2B2B1_ALLOWED_OCCURRENCE_IDENTIFIERS:
            errors.append(
                f"{name}: {source} contains forbidden JG2b2b1 occurrence surface "
                f"{identifier!r}"
            )

    for module in JG2B2B1_FORBIDDEN_STD_MODULES:
        direct = re.compile(rf"\bstd\s*::\s*{re.escape(module)}\b")
        if direct.search(text):
            errors.append(
                f"{name}: {source} contains forbidden JG2b2b1 std authority "
                f"'std::{module}'"
            )
        for braced in std_braced_imports:
            if re.search(rf"\b{re.escape(module)}\b", braced.group("body")):
                errors.append(
                    f"{name}: {source} imports forbidden JG2b2b1 std authority "
                    f"'std::{module}' through a braced path"
                )

    if JG2B2B1_STD_ALIAS.search(text) or any(
        JG2B2B1_BRACED_SELF_ALIAS.search(item.group("body"))
        for item in std_braced_imports
    ):
        errors.append(f"{name}: {source} aliases std inside the JG2b2b1 boundary")
    if JG2B2B1_STD_GLOB.search(text) or any(
        JG2B2B1_BRACED_BARE_GLOB.search(item.group("body"))
        for item in std_braced_imports
    ):
        errors.append(f"{name}: {source} glob-imports std inside the JG2b2b1 boundary")
    if JG2B2B1_CRATE_ALIAS.search(text) or any(
        JG2B2B1_BRACED_SELF_ALIAS.search(item.group("body"))
        for item in crate_braced_imports
    ):
        errors.append(f"{name}: {source} aliases crate inside the JG2b2b1 boundary")
    if JG2B2B1_CRATE_GLOB.search(text) or any(
        JG2B2B1_BRACED_BARE_GLOB.search(item.group("body"))
        for item in crate_braced_imports
    ):
        errors.append(f"{name}: {source} glob-imports crate inside the JG2b2b1 boundary")
    return errors


def validate_sources(name: str, crate_root: pathlib.Path) -> tuple[list[str], int]:
    errors: list[str] = []
    source_files = _source_files(crate_root)
    if not source_files:
        errors.append(f"{name}: crate has no Rust source files")
    if (crate_root / "build.rs").exists():
        errors.append(f"{name}: build.rs is forbidden")
    if (crate_root / ".cargo").exists():
        errors.append(f"{name}: nested .cargo configuration is forbidden")
    for source in source_files:
        if source.is_symlink():
            errors.append(f"{name}: symlinked source is forbidden: {source}")
            continue
        text = source.read_text(encoding="utf-8")
        lowered = text.lower()
        for marker in sorted(FORBIDDEN_SOURCE_MARKERS):
            if marker in lowered:
                errors.append(f"{name}: {source} contains forbidden marker {marker!r}")
        if source.name == PARTICULAR_OPEN_TYPED_SUBSTITUTION_SOURCE:
            errors.extend(
                validate_particular_open_typed_substitution_source(name, source, text)
            )
    return errors, len(source_files)


def audit() -> dict:
    errors: list[str] = []
    specs = crate_specs()
    source_file_count = 0

    try:
        root_manifest = load_toml(ROOT_MANIFEST)
        root_lock = load_toml(ROOT_LOCK)
    except (OSError, tomllib.TOMLDecodeError) as error:
        return {"status": "invalid", "errors": [f"cannot read root Cargo state: {error}"]}

    members = root_manifest.get("workspace", {}).get("members", [])
    if not isinstance(members, list):
        errors.append("root workspace members are malformed")
        members = []
    normalized_members = {str(member).replace("\\", "/") for member in members}
    root_package_names = {
        package.get("name")
        for package in root_lock.get("package", [])
        if isinstance(package, dict)
    }

    graph_inputs: list[tuple[str, pathlib.Path, pathlib.Path]] = [
        ("repository root", ROOT_MANIFEST, ROOT_LOCK)
    ]
    for spec in specs:
        name = spec["name"]
        crate_root = spec["root"]
        manifest_path = crate_root / "Cargo.toml"
        lock_path = crate_root / "Cargo.lock"
        relative_root = crate_root.relative_to(REPO).as_posix()
        if relative_root in normalized_members:
            errors.append(f"{name}: crate must not be a root workspace member")
        if name in root_package_names:
            errors.append(f"{name}: root lockfile must not contain the nested crate")
        if not manifest_path.is_file():
            errors.append(f"{name}: missing nested manifest")
            continue
        if not lock_path.is_file():
            errors.append(f"{name}: missing nested lockfile")
            continue
        try:
            manifest = load_toml(manifest_path)
        except (OSError, tomllib.TOMLDecodeError) as error:
            errors.append(f"{name}: cannot read nested manifest: {error}")
            continue
        errors.extend(validate_manifest(spec, manifest))
        source_errors, count = validate_sources(name, crate_root)
        errors.extend(source_errors)
        source_file_count += count
        graph_inputs.append((name, manifest_path, lock_path))

    try:
        reviewed = parse_reviewed_graph()
    except (OSError, ValueError) as error:
        errors.append(f"cannot read reviewed kernel dependency graph: {error}")
        reviewed = None

    verified_graphs = 0
    if reviewed is not None:
        for label, manifest, lock in graph_inputs:
            try:
                actual = resolved_kernel_graph(manifest, lock)
            except (OSError, ValueError, RuntimeError, tomllib.TOMLDecodeError) as error:
                errors.append(f"{label}: cannot resolve kernel dependency graph: {error}")
                continue
            differences = graph_difference_errors(label, actual, reviewed)
            errors.extend(differences)
            if not differences:
                verified_graphs += 1

    return {
        "status": "valid" if not errors else "invalid",
        "crate_count": len(specs),
        "source_file_count": source_file_count,
        "reviewed_package_count": 0 if reviewed is None else len(reviewed.packages),
        "verified_kernel_graphs": verified_graphs,
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
        print("generative-audit isolation: valid")
    else:
        for error in result["errors"]:
            print(f"generative-audit isolation: {error}", file=sys.stderr)
    return 0 if result["status"] == "valid" else 1


if __name__ == "__main__":
    raise SystemExit(main())
