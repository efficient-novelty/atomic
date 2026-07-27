#!/usr/bin/env python3
"""Reproduce the numerical calculations for Critical Boundary Actualization.

Conventions:
  * g1 is GUT-normalized: g1 = sqrt(5/3) gY.
  * One-loop Standard Model beta functions retain only the top Yukawa.
  * The high-order Planck benchmark uses the interpolation formulae in
    Buttazzo et al., arXiv:1307.3536, eqs. (61a-e), at alpha_s(MZ)=0.1184.
  * Pole/direct masses quoted for comparison are PDG 2026 values.

This script is an audit, not a precision spectrum calculator. The one-loop
run from the electroweak equal-trace crossing is deliberately labelled a
one-loop diagnostic.
"""
from __future__ import annotations

import json
import math
from pathlib import Path

from scipy.integrate import solve_ivp

PI = math.pi
V_EW_GEV = 246.22


def beta_one_loop(_t: float, y: list[float]) -> list[float]:
    g1, g2, g3, yt, lam = y
    f = 1.0 / (16.0 * PI**2)
    return [
        f * (41.0 / 10.0) * g1**3,
        f * (-19.0 / 6.0) * g2**3,
        f * (-7.0) * g3**3,
        f
        * yt
        * (
            (9.0 / 2.0) * yt**2
            - (17.0 / 20.0) * g1**2
            - (9.0 / 4.0) * g2**2
            - 8.0 * g3**2
        ),
        f
        * (
            24.0 * lam**2
            - 6.0 * yt**4
            + (12.0 * yt**2 - 9.0 * g2**2 - (9.0 / 5.0) * g1**2) * lam
            + (27.0 / 200.0) * g1**4
            + (9.0 / 20.0) * g1**2 * g2**2
            + (9.0 / 8.0) * g2**4
        ),
    ]


def critical_yt(g1: float, g2: float) -> float:
    """Solve beta_lambda=0 at lambda=0 in the top-dominance approximation."""
    rhs = (
        (27.0 / 200.0) * g1**4
        + (9.0 / 20.0) * g1**2 * g2**2
        + (9.0 / 8.0) * g2**4
    ) / 6.0
    if rhs <= 0:
        raise ValueError("Critical top-Yukawa fourth power is not positive")
    return rhs**0.25


def buttazzo_planck_values(mt_gev: float, mh_gev: float, alpha_s: float = 0.1184) -> tuple[float, float]:
    da = (alpha_s - 0.1184) / 0.0007
    yt = 0.3825 + 0.0051 * (mt_gev - 173.34) - 0.0021 * da
    lam = (
        -0.0143
        - 0.0066 * (mt_gev - 173.34)
        + 0.0018 * da
        + 0.0029 * (mh_gev - 125.15)
    )
    return yt, lam


def predict_pole_masses_from_planck_criticality(
    yt_star: float, alpha_s: float = 0.1184
) -> tuple[float, float]:
    """Invert Buttazzo eqs. (61d,e) imposing yt(MPl)=yt_star, lambda(MPl)=0."""
    da = (alpha_s - 0.1184) / 0.0007
    mt = 173.34 + (yt_star - 0.3825 + 0.0021 * da) / 0.0051
    mh = 125.15 + (
        0.0143 + 0.0066 * (mt - 173.34) - 0.0018 * da
    ) / 0.0029
    return mt, mh


def threshold_record(mt_gev: float, mh_gev: float, yt_star: float) -> dict[str, float]:
    yt_sm, lam_sm = buttazzo_planck_values(mt_gev, mh_gev)
    return {
        "mt_input_GeV": mt_gev,
        "mh_input_GeV": mh_gev,
        "yt_SM_MPl": yt_sm,
        "lambda_SM_MPl": lam_sm,
        "delta_yt_required": yt_star - yt_sm,
        "delta_lambda_required": -lam_sm,
        "relative_delta_yt": (yt_star - yt_sm) / yt_sm,
        "critical_stability_higgs_GeV": 129.6 + 2.0 * (mt_gev - 173.34),
    }


def main() -> None:
    # Exact electroweak-equal-trace relation.
    yt_over_g_equal = (57.0 / 200.0) ** 0.25
    mt_over_mw_equal = math.sqrt(2.0) * yt_over_g_equal

    # Existing two-loop equal-trace crossing in the manuscript.
    mu_x = 1.08939e13
    g_x = 0.545701
    g3_x = 0.580413
    yt_x = critical_yt(g_x, g_x)
    sol = solve_ivp(
        beta_one_loop,
        (math.log(mu_x), math.log(173.0)),
        (g_x, g_x, g3_x, yt_x, 0.0),
        rtol=1e-11,
        atol=1e-13,
    )
    if not sol.success:
        raise RuntimeError(sol.message)
    g1_ir, g2_ir, g3_ir, yt_ir, lam_ir = map(float, sol.y[:, -1])
    mt_ms_tree = yt_ir * V_EW_GEV / math.sqrt(2.0)
    mh_tree = math.sqrt(max(0.0, 2.0 * lam_ir)) * V_EW_GEV

    # Planck benchmark using central high-order extrapolated gauge couplings.
    g1_pl = 0.6154
    g2_pl = 0.5055
    yt_pl_crit = critical_yt(g1_pl, g2_pl)
    mt_pred, mh_pred = predict_pole_masses_from_planck_criticality(yt_pl_crit)

    pdg_direct = threshold_record(172.60, 125.13, yt_pl_crit)
    pdg_pole_xsec = threshold_record(172.10, 125.13, yt_pl_crit)

    results = {
        "exact_equal_trace_relation": {
            "yt_over_g": yt_over_g_equal,
            "mt_over_mW_same_scale": mt_over_mw_equal,
        },
        "equal_trace_crossing_one_loop_diagnostic": {
            "mu_star_GeV": mu_x,
            "g1_star": g_x,
            "g2_star": g_x,
            "g3_star": g3_x,
            "yt_star": yt_x,
            "lambda_star": 0.0,
            "at_173_GeV": {
                "g1": g1_ir,
                "g2": g2_ir,
                "g3": g3_ir,
                "yt": yt_ir,
                "lambda": lam_ir,
                "mt_MSbar_tree_GeV": mt_ms_tree,
                "mh_tree_GeV": mh_tree,
            },
        },
        "planck_no_threshold_benchmark": {
            "g1_MPl": g1_pl,
            "g2_MPl": g2_pl,
            "yt_critical_MPl": yt_pl_crit,
            "lambda_critical_MPl": 0.0,
            "predicted_top_pole_GeV": mt_pred,
            "predicted_higgs_pole_GeV": mh_pred,
            "predicted_higgs_to_top_ratio": mh_pred / mt_pred,
        },
        "threshold_targets_at_alpha_s_0p1184": {
            "using_PDG_direct_top": pdg_direct,
            "using_PDG_cross_section_pole_top": pdg_pole_xsec,
        },
        "PDG_2026_comparisons": {
            "higgs_GeV": 125.13,
            "top_direct_GeV": 172.60,
            "top_cross_section_pole_GeV": 172.10,
            "higgs_to_top_direct_ratio": 125.13 / 172.60,
            "higgs_to_top_pole_ratio": 125.13 / 172.10,
        },
    }

    out = Path("/mnt/data/critical_boundary_actualization_results.json")
    out.write_text(json.dumps(results, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(results, indent=2, sort_keys=True))
    print(f"\nWrote {out}")


if __name__ == "__main__":
    main()
