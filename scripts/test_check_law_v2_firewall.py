from __future__ import annotations

import pathlib
import tempfile
import unittest

import check_law_v2_firewall as firewall


class FirewallTests(unittest.TestCase):
    def test_isolation_commands_enforce_the_committed_lockfile(self) -> None:
        self.assertIn("--locked", firewall.ISOLATION_TEST_COMMAND)
        self.assertIn("--locked", firewall.ISOLATION_RUN_COMMAND)
        self.assertTrue(firewall.ISOLATION_LOCKFILE.is_file())
        self.assertNotEqual(firewall.ISOLATION_LOCKFILE, firewall.REPO / "Cargo.lock")
        locked_names = {
            package["name"]
            for package in firewall.load_toml(firewall.ISOLATION_LOCKFILE)["package"]
        }
        self.assertTrue(
            firewall.ALLOWED_WORKSPACE_PACKAGES.issubset(locked_names),
            "the isolation lock must cover every lawful workspace package",
        )
        self.assertFalse(firewall.FORBIDDEN_DEPENDENCIES & locked_names)
        full_workspace_packages = firewall.load_toml(
            firewall.REPO / "Cargo.lock"
        )["package"]
        for package in firewall.load_toml(firewall.ISOLATION_LOCKFILE)["package"]:
            self.assertIn(
                package,
                full_workspace_packages,
                f"{package['name']} must use the full workspace's locked resolution",
            )

    def test_toolchain_pin_is_exact_and_host_portable(self) -> None:
        channel = firewall.load_toml(
            firewall.REPO / "rust-toolchain.toml"
        )["toolchain"]["channel"]
        self.assertEqual(channel, "1.88.0")
        self.assertNotIn("windows", channel)
        workspace_rust_version = firewall.load_toml(
            firewall.REPO / "Cargo.toml"
        )["workspace"]["package"]["rust-version"]
        self.assertEqual(workspace_rust_version, "1.88")

    def test_finite_fragment_contract_is_scanned_and_allowlisted(self) -> None:
        self.assertIn("pen-gf2", firewall.PRODUCTION_ROOTS)
        self.assertIn("pen-gf2", firewall.ALLOWED_WORKSPACE_PACKAGES)
        self.assertIn("pen-gf2-agda", firewall.PRODUCTION_ROOTS)
        self.assertIn("pen-gf2-agda", firewall.ALLOWED_WORKSPACE_PACKAGES)

    def test_target_specific_and_build_dependencies_are_discovered(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            manifest = pathlib.Path(temporary) / "Cargo.toml"
            manifest.write_text(
                """
[package]
name = "probe"
version = "0.0.0"

[target.'cfg(windows)'.dependencies]
pen-oracle = { path = "../pen-oracle" }

[target.'cfg(unix)'.build-dependencies]
pen-search = { path = "../pen-search" }
""".strip(),
                encoding="utf-8",
            )
            self.assertEqual(
                firewall.dependency_names(manifest),
                {"pen-oracle", "pen-search"},
            )

    def test_build_scripts_and_assets_are_scanned(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            crate = pathlib.Path(temporary)
            (crate / "src").mkdir()
            (crate / "src" / "lib.rs").write_text("", encoding="utf-8")
            (crate / "build.rs").write_text("", encoding="utf-8")
            (crate / "grammar.json").write_text("{}", encoding="utf-8")
            (crate / "Cargo.toml").write_text("", encoding="utf-8")

            candidates = firewall.candidate_input_files(crate)
            self.assertIn(crate / "build.rs", candidates)
            self.assertIn(crate / "grammar.json", candidates)
            self.assertIn(crate / "Cargo.toml", candidates)

    def test_target_count_and_stage_control_flow_are_rejected(self) -> None:
        violations = firewall.byte_violations(
            b"if stage_index == 15 { choose_expected_hash(); }"
        )
        self.assertTrue(any("literal target count" in item for item in violations))
        self.assertTrue(any("stage-index control flow" in item for item in violations))
        self.assertTrue(any("expected_hash" in item for item in violations))

    def test_stage_four_names_are_rejected_from_the_generic_production_lane(self) -> None:
        for spelling in (b"Stage4Cone", b"stage_4_status", b"step-4-result"):
            with self.subTest(spelling=spelling):
                violations = firewall.byte_violations(spelling)
                self.assertTrue(
                    any("stage4" in item or "stage_4" in item or "stage-4" in item
                        or "step4" in item or "step_4" in item or "step-4" in item
                        for item in violations)
                )


if __name__ == "__main__":
    unittest.main()
