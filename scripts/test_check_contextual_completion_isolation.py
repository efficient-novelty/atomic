import importlib.util
import pathlib
import tempfile
import unittest
from unittest import mock


SCRIPT = pathlib.Path(__file__).with_name("check_contextual_completion_isolation.py")
SPEC = importlib.util.spec_from_file_location("contextual_completion_isolation", SCRIPT)
assert SPEC and SPEC.loader
CHECKER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKER)


class ContextualCompletionIsolationTests(unittest.TestCase):
    def test_repository_layout_is_isolated(self):
        self.assertEqual(CHECKER.audit()["status"], "valid")

    def test_root_membership_is_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            contextual = root / "crates" / "pen-contextual-completion"
            (contextual / "src").mkdir(parents=True)
            (root / "Cargo.toml").write_text(
                '[workspace]\nmembers = ["crates/pen-contextual-completion"]\n',
                encoding="utf-8",
            )
            (root / "Cargo.lock").write_text("", encoding="utf-8")
            (contextual / "Cargo.toml").write_text(
                """
[package]
name = "pen-contextual-completion"
[package.metadata.law-v2]
role = "research-prototype"
authority = "proposed-generic-only"
live_profile_a_access = false
registered_prefix_access = false
candidate_generation = false
production_payment_authorized = false
[workspace]
[dependencies]
pen-kernel = { path = "../pen-kernel" }
serde = "1"
serde_json = "1"
thiserror = "2"
""",
                encoding="utf-8",
            )
            (contextual / "Cargo.lock").write_text("", encoding="utf-8")
            (contextual / "src" / "lib.rs").write_text("", encoding="utf-8")
            with (
                mock.patch.object(CHECKER, "REPO", root),
                mock.patch.object(CHECKER, "ROOT_MANIFEST", root / "Cargo.toml"),
                mock.patch.object(CHECKER, "ROOT_LOCK", root / "Cargo.lock"),
                mock.patch.object(CHECKER, "CONTEXTUAL_ROOT", contextual),
                mock.patch.object(
                    CHECKER, "CONTEXTUAL_MANIFEST", contextual / "Cargo.toml"
                ),
                mock.patch.object(CHECKER, "CONTEXTUAL_LOCK", contextual / "Cargo.lock"),
            ):
                result = CHECKER.audit()
            self.assertEqual(result["status"], "invalid")
            self.assertIn(
                "contextual crate must not be a root workspace member",
                result["errors"],
            )


if __name__ == "__main__":
    unittest.main()
