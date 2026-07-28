from __future__ import annotations

import pathlib
import tempfile
import unittest

import check_law_v2_firewall as firewall


class FirewallTests(unittest.TestCase):
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


if __name__ == "__main__":
    unittest.main()
