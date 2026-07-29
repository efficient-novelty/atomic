import importlib.util
import pathlib
import tempfile
import unittest
from unittest import mock


SCRIPT = pathlib.Path(__file__).with_name("check_kernel_synthesis_isolation.py")
SPEC = importlib.util.spec_from_file_location("kernel_synthesis_isolation", SCRIPT)
assert SPEC and SPEC.loader
CHECKER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKER)


class KernelSynthesisIsolationTests(unittest.TestCase):
    def test_repository_layout_is_isolated(self):
        self.assertEqual(CHECKER.audit()["status"], "valid")

    def test_root_membership_is_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            synthesis_root = root / "crates" / "pen-kernel-synthesis"
            (synthesis_root / "src").mkdir(parents=True)
            (root / "Cargo.toml").write_text(
                '[workspace]\nmembers = ["crates/pen-kernel-synthesis"]\n',
                encoding="utf-8",
            )
            (root / "Cargo.lock").write_text("", encoding="utf-8")
            (synthesis_root / "Cargo.toml").write_text(
                """
[package]
name = "pen-kernel-synthesis"
[package.metadata.law-v2]
role = "proposed-kernel-synthesis-successor"
oracle_access = false
diagnostic_access = false
live_profile_a_access = false
[workspace]
[dependencies]
pen-kernel = { path = "../pen-kernel" }
thiserror = "2"
""",
                encoding="utf-8",
            )
            (synthesis_root / "Cargo.lock").write_text("", encoding="utf-8")
            (synthesis_root / "src" / "lib.rs").write_text("", encoding="utf-8")
            with (
                mock.patch.object(CHECKER, "REPO", root),
                mock.patch.object(CHECKER, "ROOT_MANIFEST", root / "Cargo.toml"),
                mock.patch.object(CHECKER, "ROOT_LOCK", root / "Cargo.lock"),
                mock.patch.object(CHECKER, "SYNTHESIS_ROOT", synthesis_root),
                mock.patch.object(
                    CHECKER,
                    "SYNTHESIS_MANIFEST",
                    synthesis_root / "Cargo.toml",
                ),
                mock.patch.object(
                    CHECKER,
                    "SYNTHESIS_LOCK",
                    synthesis_root / "Cargo.lock",
                ),
            ):
                result = CHECKER.audit()
            self.assertEqual(result["status"], "invalid")
            self.assertIn(
                "kernel-synthesis crate must not be a root workspace member",
                result["errors"],
            )


if __name__ == "__main__":
    unittest.main()
