import copy
import importlib.util
import pathlib
import tempfile
import unittest


SCRIPT = pathlib.Path(__file__).with_name("check_generative_audit_isolation.py")
SPEC = importlib.util.spec_from_file_location("generative_audit_isolation", SCRIPT)
assert SPEC and SPEC.loader
CHECKER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKER)


class GenerativeAuditIsolationTests(unittest.TestCase):
    def test_repository_layout_and_all_kernel_graphs_are_isolated(self):
        result = CHECKER.audit()
        self.assertEqual(result["errors"], [])
        self.assertEqual(result["status"], "valid")
        self.assertEqual(result["verified_kernel_graphs"], 3)

    def test_sealed_history_authority_metadata_flip_is_rejected(self):
        spec = CHECKER.crate_specs()[0]
        manifest = CHECKER.load_toml(spec["root"] / "Cargo.toml")
        manifest["package"]["metadata"]["law-v2"]["eof_authority"] = True

        errors = CHECKER.validate_manifest(spec, manifest)

        self.assertIn(
            "pen-sealed-history: metadata 'eof_authority' must equal False",
            errors,
        )

    def test_complete_through_head_authority_flip_is_rejected(self):
        spec = CHECKER.crate_specs()[1]
        manifest = CHECKER.load_toml(spec["root"] / "Cargo.toml")
        manifest["package"]["metadata"]["law-v2"][
            "complete_through_producer_finalized_head_authority"
        ] = False

        errors = CHECKER.validate_manifest(spec, manifest)

        self.assertIn(
            "pen-generative-audit: metadata "
            "'complete_through_producer_finalized_head_authority' must equal True",
            errors,
        )

    def test_global_or_current_authority_flip_is_rejected(self):
        spec = CHECKER.crate_specs()[1]
        for key in (
            "globally_latest_history_authority",
            "actual_current_branch_authority",
            "eof_authority",
        ):
            with self.subTest(key=key):
                manifest = CHECKER.load_toml(spec["root"] / "Cargo.toml")
                manifest["package"]["metadata"]["law-v2"][key] = True

                errors = CHECKER.validate_manifest(spec, manifest)

                self.assertIn(
                    f"pen-generative-audit: metadata '{key}' must equal False",
                    errors,
                )

    def test_jg2b2_entry_protocol_grammar_flip_is_rejected(self):
        spec = CHECKER.crate_specs()[1]
        manifest = CHECKER.load_toml(spec["root"] / "Cargo.toml")
        manifest["package"]["metadata"]["law-v2"][
            "jg2b2_entry_protocol_grammar"
        ] = False

        errors = CHECKER.validate_manifest(spec, manifest)

        self.assertIn(
            "pen-generative-audit: metadata "
            "'jg2b2_entry_protocol_grammar' must equal True",
            errors,
        )

    def test_jg2b2b0_structural_occurrence_grammar_flip_is_rejected(self):
        spec = CHECKER.crate_specs()[1]
        manifest = CHECKER.load_toml(spec["root"] / "Cargo.toml")
        manifest["package"]["metadata"]["law-v2"][
            "jg2b2b0_structural_occurrence_grammar"
        ] = False

        errors = CHECKER.validate_manifest(spec, manifest)

        self.assertIn(
            "pen-generative-audit: metadata "
            "'jg2b2b0_structural_occurrence_grammar' must equal True",
            errors,
        )

    def test_jg2b2b1_particular_substitution_authority_flip_is_rejected(self):
        spec = CHECKER.crate_specs()[1]
        manifest = CHECKER.load_toml(spec["root"] / "Cargo.toml")
        manifest["package"]["metadata"]["law-v2"][
            "particular_open_typed_substitution_authority"
        ] = False

        errors = CHECKER.validate_manifest(spec, manifest)

        self.assertIn(
            "pen-generative-audit: metadata "
            "'particular_open_typed_substitution_authority' must equal True",
            errors,
        )

    def test_jg2b2_unearned_factual_authority_flips_are_rejected(self):
        spec = CHECKER.crate_specs()[1]
        for key in (
            "derived_public_occurrence_authority",
            "complete_admissible_substitution_universe_authority",
            "full_kernel_typed_substitution_metatheory",
            "generic_identity_composition_lifting_authority",
            "generic_typing_equality_preservation_authority",
            "normalization_reindexing_compatibility_authority",
            "executable_indexed_interface_authority",
            "indexed_interface_functor_law_authority",
            "universal_naturality_authority",
        ):
            with self.subTest(key=key):
                manifest = CHECKER.load_toml(spec["root"] / "Cargo.toml")
                manifest["package"]["metadata"]["law-v2"][key] = True

                errors = CHECKER.validate_manifest(spec, manifest)

                self.assertIn(
                    f"pen-generative-audit: metadata '{key}' must equal False",
                    errors,
                )

    def test_forbidden_direct_dependency_is_rejected(self):
        spec = CHECKER.crate_specs()[1]
        manifest = CHECKER.load_toml(spec["root"] / "Cargo.toml")
        manifest["dependencies"]["pen-oracle"] = {"path": "../pen-oracle"}

        errors = CHECKER.validate_manifest(spec, manifest)

        self.assertIn(
            "pen-generative-audit: production dependencies are not the exact allowlist",
            errors,
        )

    def test_transitive_checksum_drift_is_rejected(self):
        reviewed = CHECKER.parse_reviewed_graph()
        changed_packages = dict(reviewed.packages)
        identity = sorted(changed_packages)[0]
        changed_packages[identity] = "0" * 64
        actual = CHECKER.KernelGraph(
            reviewed.roots,
            changed_packages,
            reviewed.edges,
        )

        errors = CHECKER.graph_difference_errors("fixture", actual, reviewed)

        self.assertTrue(any("checksum mismatch" in error for error in errors))

    def test_transitive_edge_drift_is_rejected(self):
        reviewed = CHECKER.parse_reviewed_graph()
        changed_edges = set(reviewed.edges)
        changed_edges.remove(sorted(changed_edges)[0])
        actual = CHECKER.KernelGraph(
            reviewed.roots,
            copy.copy(reviewed.packages),
            frozenset(changed_edges),
        )

        errors = CHECKER.graph_difference_errors("fixture", actual, reviewed)

        self.assertTrue(any("missing reviewed kernel edge" in error for error in errors))

    def test_forbidden_source_include_is_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            crate = pathlib.Path(temporary)
            (crate / "src").mkdir()
            (crate / "src" / "lib.rs").write_text(
                'include_str!("../../../forbidden.txt");\n', encoding="utf-8"
            )

            errors, count = CHECKER.validate_sources("fixture", crate)

        self.assertEqual(count, 1)
        self.assertTrue(any("include_str!(" in error for error in errors))

    def test_jg2b2b1_source_allows_only_the_b0_occurrence_grammar_prerequisite(self):
        with tempfile.TemporaryDirectory() as temporary:
            crate = pathlib.Path(temporary)
            (crate / "src").mkdir()
            (crate / "src" / CHECKER.PARTICULAR_OPEN_TYPED_SUBSTITUTION_SOURCE).write_text(
                "use crate::VerifiedGenerativeStructuralOccurrenceGrammarV1;\n"
                "use std::collections::BTreeSet;\n"
                "#[cfg(test)]\n"
                "mod tests {\n"
                "    use crate::{\n"
                "        proposed_generative_structural_occurrence_grammar_v1,\n"
                "        verify_generative_structural_occurrence_grammar_v1,\n"
                "    };\n"
                "}\n",
                encoding="utf-8",
            )

            errors, count = CHECKER.validate_sources("fixture", crate)

        self.assertEqual(count, 1)
        self.assertEqual(errors, [])

    def test_jg2b2b1_source_rejects_history_and_occurrence_surfaces(self):
        with tempfile.TemporaryDirectory() as temporary:
            crate = pathlib.Path(temporary)
            (crate / "src").mkdir()
            source = crate / "src" / CHECKER.PARTICULAR_OPEN_TYPED_SUBSTITUTION_SOURCE
            source.write_text(
                "use crate::{\n"
                "    VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1,\n"
                "    ProposedGenerativeStructuralDeclarationTraversalV1,\n"
                "    GenerativeStructuralOccurrenceRootV1,\n"
                "    proposed_generative_structural_occurrence_definition_v1,\n"
                "};\n",
                encoding="utf-8",
            )

            errors, count = CHECKER.validate_sources("fixture", crate)

        self.assertEqual(count, 1)
        self.assertTrue(any("historythroughheadv1" in error for error in errors))
        self.assertTrue(any("occurrence surface" in error for error in errors))
        self.assertTrue(any("structural_occurrence" in error for error in errors))

    def test_jg2b2b1_cfg_test_boundary_cannot_hide_later_production_authority(self):
        with tempfile.TemporaryDirectory() as temporary:
            crate = pathlib.Path(temporary)
            (crate / "src").mkdir()
            source = crate / "src" / CHECKER.PARTICULAR_OPEN_TYPED_SUBSTITUTION_SOURCE
            source.write_text(
                "#[cfg(test)]\n"
                "mod tests {\n"
                "    use crate::proposed_generative_structural_occurrence_grammar_v1;\n"
                "}\n"
                "use crate::VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1;\n",
                encoding="utf-8",
            )

            errors, count = CHECKER.validate_sources("fixture", crate)

        self.assertEqual(count, 1)
        self.assertTrue(any("historythroughheadv1" in error for error in errors))

    def test_jg2b2b1_source_rejects_std_io_and_environment_authorities(self):
        for module in CHECKER.JG2B2B1_FORBIDDEN_STD_MODULES:
            variants = {
                "direct": f"type Forbidden = std::{module}::Forbidden;\n",
                "nested use": (
                    "use std::{collections::{BTreeMap, BTreeSet}, "
                    f"{module}::Forbidden}};\n"
                ),
            }
            for label, source_text in variants.items():
                with (
                    self.subTest(module=module, variant=label),
                    tempfile.TemporaryDirectory() as temporary,
                ):
                    crate = pathlib.Path(temporary)
                    (crate / "src").mkdir()
                    source = (
                        crate
                        / "src"
                        / CHECKER.PARTICULAR_OPEN_TYPED_SUBSTITUTION_SOURCE
                    )
                    source.write_text(source_text, encoding="utf-8")

                    errors, count = CHECKER.validate_sources("fixture", crate)

                    self.assertEqual(count, 1)
                    self.assertTrue(
                        any(f"std::{module}" in error for error in errors), errors
                    )

    def test_jg2b2b1_source_rejects_namespace_alias_and_glob_bypasses(self):
        bypasses = {
            "std alias": "use std as platform;\n",
            "absolute std alias": "use ::std as platform;\n",
            "std self alias": "use std::{self as platform};\n",
            "nested std self alias": (
                "use std::{collections::{BTreeMap, BTreeSet}, self as platform};\n"
            ),
            "std glob": "use std :: *;\n",
            "std braced glob": "use std::{collections::BTreeSet, *};\n",
            "crate alias": "use crate as authority;\n",
            "crate self alias": "use crate::{self as authority};\n",
            "crate glob": "use crate :: *;\n",
            "crate braced glob": "use crate::{VerifiedThing, *};\n",
            "extern self alias": "extern crate self as authority;\n",
        }
        for label, source_text in bypasses.items():
            with self.subTest(label=label), tempfile.TemporaryDirectory() as temporary:
                crate = pathlib.Path(temporary)
                (crate / "src").mkdir()
                source = crate / "src" / CHECKER.PARTICULAR_OPEN_TYPED_SUBSTITUTION_SOURCE
                source.write_text(source_text, encoding="utf-8")

                errors, count = CHECKER.validate_sources("fixture", crate)

                self.assertEqual(count, 1)
                self.assertTrue(errors, f"{label} bypass was not rejected")


if __name__ == "__main__":
    unittest.main()
