#!/usr/bin/env python3
"""Reproduce the numerical diagnostics for Canonical Spectral Closure.

Conventions
-----------
* g1 is GUT-normalized: g1 = sqrt(5/3) gY.
* The RG diagnostic uses one-loop SM beta functions with only the top Yukawa retained.
* Weak-scale inputs use the interpolation formulas in Buttazzo et al.,
  arXiv:1307.3536, evaluated at Mt=172.60 GeV, Mh=125.13 GeV,
  alpha_s(MZ)=0.1184, and MW=80.384 GeV.
* The manuscript's two-loop electroweak pair crossing is used independently for the
  gauge-threshold record: mu_X=1.08939e13 GeV,
  g1=g2=0.545701, g3=0.580413.

This is an audit and boundary-condition diagnostic, not a precision multi-loop
spectrum calculation.
"""
from __future__ import annotations

import json
import math
from pathlib import Path
from typing import Callable

import numpy as np
from scipy.integrate import solve_ivp
from scipy.optimize import brentq, minimize_scalar

PI = math.pi
MT_GEV = 172.60
MH_GEV = 125.13
ALPHA_S_MZ = 0.1184
MW_GEV = 80.384


def weak_scale_inputs() -> tuple[float, float, float, float, float]:
    """Return (g1,g2,g3,yt,lambda) at mu=Mt using Buttazzo interpolations."""
    lam = 0.12604 + 0.00206 * (MH_GEV - 125.15) - 0.00004 * (MT_GEV - 173.34)
    yt = 0.93690 + 0.00556 * (MT_GEV - 173.34) - 0.00042 * (
        (ALPHA_S_MZ - 0.1184) / 0.0007
    )
    g2 = 0.64779 + 0.00004 * (MT_GEV - 173.34) + 0.00011 * (
        (MW_GEV - 80.384) / 0.014
    )
    g_y = 0.35830 + 0.00011 * (MT_GEV - 173.34) - 0.00020 * (
        (MW_GEV - 80.384) / 0.014
    )
    g1 = math.sqrt(5.0 / 3.0) * g_y
    g3 = 1.1666 + 0.00314 * ((ALPHA_S_MZ - 0.1184) / 0.0007) - 0.00046 * (
        MT_GEV - 173.34
    )
    return g1, g2, g3, yt, lam


def beta_sm_one_loop(_t: float, y: np.ndarray) -> list[float]:
    g1, g2, g3, yt, lam = map(float, y)
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


def integrate_rg():
    y0 = weak_scale_inputs()
    sol = solve_ivp(
        beta_sm_one_loop,
        (math.log(MT_GEV), math.log(1.0e20)),
        y0,
        rtol=1.0e-11,
        atol=1.0e-13,
        max_step=0.05,
        dense_output=True,
    )
    if not sol.success:
        raise RuntimeError(sol.message)
    return sol


def root_mu(sol, fn: Callable[[np.ndarray], float], lo: float, hi: float) -> float:
    return math.exp(
        brentq(lambda t: fn(sol.sol(t)), math.log(lo), math.log(hi), xtol=1.0e-13)
    )


def record_at(sol, mu: float) -> dict[str, float]:
    g1, g2, g3, yt, lam = map(float, sol.sol(math.log(mu)))
    return {"mu_GeV": mu, "g1": g1, "g2": g2, "g3": g3, "yt": yt, "lambda": lam}


def main() -> None:
    sol = integrate_rg()

    # Exact spectral ratios.
    top_yt_over_g = 2.0 / math.sqrt(3.0)
    top_mt_over_mw = math.sqrt(8.0 / 3.0)
    equal_nu_mt_over_mw = math.sqrt(2.0)

    # Existing two-loop pair crossing in the manuscript.
    mu_x = 1.08939e13
    g_pair = 0.545701
    g3_x = 0.580413
    f0_pair = PI**2 / (2.0 * g_pair**2)
    f0_strong = PI**2 / (2.0 * g3_x**2)
    gauge_threshold = {
        "mu_X_GeV": mu_x,
        "g1_equals_g2": g_pair,
        "g3": g3_x,
        "g3_above_pair_fraction": g3_x / g_pair - 1.0,
        "g3_reduction_fraction": g_pair / g3_x - 1.0,
        "delta_inverse_g3_squared": 1.0 / g_pair**2 - 1.0 / g3_x**2,
        "f0_from_pair": f0_pair,
        "f0_from_strong": f0_strong,
        "f0_relative_mismatch": f0_strong / f0_pair - 1.0,
    }

    # One-loop gauge crossings and least-squares spectral boundary.
    mu_g12 = root_mu(sol, lambda y: float(y[0] - y[1]), 1e3, 1e18)
    mu_g13 = root_mu(sol, lambda y: float(y[0] - y[2]), 1e3, 1e20)
    mu_g23 = root_mu(sol, lambda y: float(y[1] - y[2]), 1e3, 1e20)

    def gauge_variance(t: float) -> float:
        g = np.asarray(sol.sol(t)[:3], dtype=float)
        inv = 1.0 / g**2
        return float(np.var(inv))

    best = minimize_scalar(
        gauge_variance,
        bounds=(math.log(1e10), math.log(1e19)),
        method="bounded",
        options={"xatol": 1e-12},
    )
    mu_best = math.exp(best.x)
    g_best = np.asarray(sol.sol(best.x)[:3], dtype=float)
    inv_best = 1.0 / g_best**2
    common_inverse = float(inv_best.mean())
    g_best_common = 1.0 / math.sqrt(common_inverse)

    # Yukawa and quartic conditions.
    mu_y = root_mu(sol, lambda y: float(3.0 * y[3] ** 2 - 4.0 * y[1] ** 2), 1e3, 1e12)
    y_record = record_at(sol, mu_y)
    y_record["A_top"] = 3.0 * y_record["yt"] ** 2
    y_record["four_g2_squared"] = 4.0 * y_record["g2"] ** 2
    y_record["lambda_spectral_top"] = (4.0 / 3.0) * y_record["g2"] ** 2
    y_record["lambda_spectral_minimum"] = y_record["g2"] ** 2 / 6.0
    y_record["top_spectral_to_SM_lambda_factor"] = (
        y_record["lambda_spectral_top"] / y_record["lambda"]
    )

    mu_min_quartic = root_mu(
        sol, lambda y: float(y[4] - y[1] ** 2 / 6.0), 1e3, 1e8
    )
    min_quartic_record = record_at(sol, mu_min_quartic)
    min_quartic_record["g2_squared_over_6"] = min_quartic_record["g2"] ** 2 / 6.0

    mu_equal_nu = root_mu(sol, lambda y: float(y[3] - y[1]), 1e3, 1e15)
    equal_nu_record = record_at(sol, mu_equal_nu)
    equal_nu_record["lambda_spectral_equal_top_nu"] = equal_nu_record["g2"] ** 2
    equal_nu_record["lambda_spectral_minimum"] = equal_nu_record["g2"] ** 2 / 6.0

    # Dimensionless gravitational coefficients at the pair-normalized value.
    alpha_c = -3.0 / (20.0 * g_pair**2)
    tau_e = 11.0 / (120.0 * g_pair**2)

    results = {
        "weak_scale_inputs": {
            "mu_GeV": MT_GEV,
            "Mt_GeV": MT_GEV,
            "Mh_GeV": MH_GEV,
            "alpha_s_MZ": ALPHA_S_MZ,
            "g1_GUT_normalized": weak_scale_inputs()[0],
            "g2": weak_scale_inputs()[1],
            "g3": weak_scale_inputs()[2],
            "yt": weak_scale_inputs()[3],
            "lambda": weak_scale_inputs()[4],
        },
        "exact_spectral_relations": {
            "top_dominance_yt_over_g": top_yt_over_g,
            "top_dominance_mt_over_MW": top_mt_over_mw,
            "equal_top_neutrino_mt_over_MW": equal_nu_mt_over_mw,
            "quartic_lower_coefficient_lambda_over_g2": 1.0 / 6.0,
            "top_dominance_lambda_over_g2": 4.0 / 3.0,
            "equal_top_neutrino_lambda_over_g2": 1.0,
            "tau_E_over_alpha_C": -11.0 / 18.0,
            "xi0": 1.0 / 12.0,
        },
        "two_loop_pair_crossing_threshold": gauge_threshold,
        "one_loop_gauge_diagnostic": {
            "g1_equals_g2": record_at(sol, mu_g12),
            "g1_equals_g3": record_at(sol, mu_g13),
            "g2_equals_g3": record_at(sol, mu_g23),
            "least_squares_inverse_coupling_scale": {
                **record_at(sol, mu_best),
                "common_g_from_mean_inverse_squared": g_best_common,
                "inverse_squared_couplings": inv_best.tolist(),
                "deviations_from_mean_inverse_squared": (inv_best - common_inverse).tolist(),
            },
        },
        "one_loop_yukawa_quartic_diagnostic": {
            "minimum_spectral_quartic_crossing": min_quartic_record,
            "top_dominance_Yukawa_sum_crossing": y_record,
            "equal_top_neutrino_Yukawa_sum_crossing": equal_nu_record,
            "minimal_one_Higgs_Dirac_common_intersection_found": False,
        },
        "curvature_coefficients_at_g_pair": {
            "g_star": g_pair,
            "alpha_C": alpha_c,
            "tau_E": tau_e,
            "tau_E_over_alpha_C": tau_e / alpha_c,
            "xi0": 1.0 / 12.0,
        },
        "exact_CSC_CBA_compatibility": {
            "same_scale_threshold_free_nontrivial_Yukawa_sector": False,
            "reason": "CSC gives lambda=4 g^2 B/A^2 > 0, while CBA gives lambda=0.",
        },
    }

    json_path = Path("/mnt/data/canonical_spectral_closure_results.json")
    json_path.write_text(json.dumps(results, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    text_lines = [
        "Canonical Spectral Closure calculation record",
        "=============================================",
        "",
        f"Top-dominance y_t/g = {top_yt_over_g:.12f}",
        f"Top-dominance m_t/M_W = {top_mt_over_mw:.12f}",
        f"Equal top-neutrino m_t/M_W = {equal_nu_mt_over_mw:.12f}",
        "",
        f"Two-loop manuscript pair crossing: mu = {mu_x:.6e} GeV",
        f"  g1=g2={g_pair:.6f}; g3={g3_x:.6f}",
        f"  g3 excess = {100*gauge_threshold['g3_above_pair_fraction']:.6f}%",
        f"  required g3 reduction = {100*gauge_threshold['g3_reduction_fraction']:.6f}%",
        f"  Delta(1/g3^2) = {gauge_threshold['delta_inverse_g3_squared']:.9f}",
        f"  f0(pair)={f0_pair:.8f}; f0(strong)={f0_strong:.8f}",
        "",
        f"Minimum spectral quartic bound crossing: {mu_min_quartic:.6e} GeV",
        f"Top-dominance Yukawa-sum crossing: {mu_y:.6e} GeV",
        f"  lambda_SM={y_record['lambda']:.9f}",
        f"  lambda_spectral_top={y_record['lambda_spectral_top']:.9f}",
        f"  mismatch factor={y_record['top_spectral_to_SM_lambda_factor']:.6f}",
        f"Equal top-neutrino crossing: {mu_equal_nu:.6e} GeV",
        f"  lambda_SM={equal_nu_record['lambda']:.9f}",
        f"  lambda_spectral={equal_nu_record['lambda_spectral_equal_top_nu']:.9f}",
        "",
        f"Curvature coefficients at g*={g_pair:.6f}: alpha_C={alpha_c:.9f}, tau_E={tau_e:.9f}",
        f"  tau_E/alpha_C={tau_e/alpha_c:.12f}; xi0={1/12:.12f}",
        "",
        "Verdict: no threshold-free common one-loop spectral matching scale exists for the",
        "current minimal one-Higgs Dirac Standard Model diagnostic.",
    ]
    text_path = Path("/mnt/data/canonical_spectral_closure_calculations.txt")
    text_path.write_text("\n".join(text_lines) + "\n", encoding="utf-8")

    print(json.dumps(results, indent=2, sort_keys=True))
    print(f"\nWrote {json_path}")
    print(f"Wrote {text_path}")


if __name__ == "__main__":
    main()
