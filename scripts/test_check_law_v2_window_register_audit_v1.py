from __future__ import annotations

import copy
import unittest

import check_law_v2_window_register_audit_v1 as audit_checker


class WindowRegisterAuditTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.audit = audit_checker.load_json(audit_checker.DEFAULT_AUDIT)
        cls.h3 = audit_checker.load_json(audit_checker.DEFAULT_H3)
        cls.h4 = audit_checker.load_json(audit_checker.DEFAULT_H4)
        cls.bootstrap = audit_checker.load_json(audit_checker.DEFAULT_BOOTSTRAP)
        cls.registry = audit_checker.load_json(audit_checker.DEFAULT_REGISTRY)

    def test_issued_undefined_audit_replays(self) -> None:
        audit_checker.verify_inputs(
            self.audit, self.h3, self.h4, self.bootstrap, self.registry
        )
        complete_acts, residual_count = audit_checker.verify_audit_logic(self.audit)
        self.assertEqual(complete_acts, 0)
        self.assertEqual(residual_count, 20)

    def test_family_binding_tamper_is_rejected(self) -> None:
        audit = copy.deepcopy(self.audit)
        audit["acts"][3]["observed_demand_families"][0]["family_id"] = (
            "blake3:" + "0" * 64
        )
        with self.assertRaises(audit_checker.AuditError):
            audit_checker.verify_inputs(
                audit, self.h3, self.h4, self.bootstrap, self.registry
            )

    def test_incomplete_act_cannot_launder_a_numeric_value(self) -> None:
        audit = copy.deepcopy(self.audit)
        audit["acts"][0]["kappa_sem"] = 1
        with self.assertRaises(audit_checker.AuditError):
            audit_checker.verify_audit_logic(audit)

    def test_proposed_profile_cannot_gain_result_authority(self) -> None:
        registry = copy.deepcopy(self.registry)
        registry["profiles"][1]["termination"] = "Unknown"
        with self.assertRaises(audit_checker.AuditError):
            audit_checker.verify_inputs(
                self.audit, self.h3, self.h4, self.bootstrap, registry
            )

    def test_missing_global_prerequisite_cannot_be_laundered(self) -> None:
        audit = copy.deepcopy(self.audit)
        audit["global_prerequisite"]["status"] = "present"
        with self.assertRaises(audit_checker.AuditError):
            audit_checker.verify_audit_logic(audit)

    def test_undefined_audit_cannot_authorize_a_contextual_live_run(self) -> None:
        audit = copy.deepcopy(self.audit)
        audit["next_phase_authority"]["contextual_live_run_authorized"] = True
        with self.assertRaises(audit_checker.AuditError):
            audit_checker.verify_audit_logic(audit)

    def test_unknown_audit_field_is_rejected(self) -> None:
        audit = copy.deepcopy(self.audit)
        audit["nu_override"] = 3
        with self.assertRaises(audit_checker.AuditError):
            audit_checker.verify_audit_logic(audit)

    def test_residuals_cannot_be_collapsed(self) -> None:
        audit = copy.deepcopy(self.audit)
        for obligation in audit["acts"][0]["audit_obligations"].values():
            obligation["residual_id"] = "COLLAPSED"
        audit["acts"][0]["residual_gaps"] = [
            {"id": "COLLAPSED", "disposition": "open_missing_certificate"}
        ]
        with self.assertRaises(audit_checker.AuditError):
            audit_checker.verify_audit_logic(audit)


if __name__ == "__main__":
    unittest.main()
