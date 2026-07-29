import importlib.util
import pathlib
import tempfile
import unittest
from unittest import mock


SCRIPT = pathlib.Path(__file__).with_name("check_semantic_audit_isolation.py")
SPEC = importlib.util.spec_from_file_location("semantic_audit_isolation", SCRIPT)
assert SPEC and SPEC.loader
CHECKER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKER)


class SemanticAuditIsolationTests(unittest.TestCase):
    def test_repository_layout_is_isolated(self):
        self.assertEqual(CHECKER.audit()["status"], "valid")

    def test_root_membership_is_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            (root / "crates" / "pen-semantic-audit" / "src").mkdir(parents=True)
            (root / "scripts").mkdir()
            (root / "Cargo.toml").write_text(
                '[workspace]\nmembers = ["crates/pen-semantic-audit"]\n',
                encoding="utf-8",
            )
            (root / "Cargo.lock").write_text("", encoding="utf-8")
            audit_root = root / "crates" / "pen-semantic-audit"
            (audit_root / "Cargo.toml").write_text(
                """
[package]
name = "pen-semantic-audit"
[package.metadata.law-v2]
role = "proposed-generic-audit"
live_profile_a_access = false
oracle_access = false
[workspace]
[dependencies]
pen-kernel = { path = "../pen-kernel" }
serde = "1"
thiserror = "2"
""",
                encoding="utf-8",
            )
            (audit_root / "Cargo.lock").write_text("", encoding="utf-8")
            (audit_root / "src" / "lib.rs").write_text("", encoding="utf-8")
            with (
                mock.patch.object(CHECKER, "REPO", root),
                mock.patch.object(CHECKER, "ROOT_MANIFEST", root / "Cargo.toml"),
                mock.patch.object(CHECKER, "ROOT_LOCK", root / "Cargo.lock"),
                mock.patch.object(CHECKER, "AUDIT_ROOT", audit_root),
                mock.patch.object(CHECKER, "AUDIT_MANIFEST", audit_root / "Cargo.toml"),
                mock.patch.object(CHECKER, "AUDIT_LOCK", audit_root / "Cargo.lock"),
            ):
                result = CHECKER.audit()
            self.assertEqual(result["status"], "invalid")
            self.assertIn(
                "semantic-audit crate must not be a root workspace member",
                result["errors"],
            )


if __name__ == "__main__":
    unittest.main()
