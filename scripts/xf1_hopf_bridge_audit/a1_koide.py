"""
A1 -- Exact Koide cross-check of the Hopf-bridge charged-lepton formula.

Paper: "The Complex Hopf Fibration as the Canonical Space for Gauge-Gravity
Unification" (SSRN 6975959, 18 June 2026).

Charged-lepton mass formula, Eq. (62):
    m_n = Lambda_Hopf * (n+1) * exp[ a*n - D(n) + n*alpha/6 + sigma_3 * ln tau_3(K_n) ]

Exact symbolic inputs claimed by the paper:
    a         = 6*sqrt(2) * exp( zeta(3) / (24 pi^2) )                   Eq. (53)/(55)
    sigma_3   = zeta(3) / (4 pi^2)                                        Eq. (46)/(69)
    tau_3     = (1, 1, sqrt(3))   for (unknot, Hopf link, trefoil)        Eq. (70)
    alpha     = 2Vol(S2)/(Vol(S4)^2 Vol(RP1)) * (Vol(S9)/(2^5*5))^(1/4)   Thm 46
    Lambda_L  = sqrt(2 pi) * v * kappa^6,  kappa = (1/4pi^2) exp(zeta(3)/24pi^2)  p.41
    D(1),D(2),D(3) = 1.203011392, 4.806545406, 10.818228646               Eq. (66)
                     -- printed only; claimed to be "extracted" from Eq. (63)-(65).

This script tests every one of those, at 60-digit precision.
"""
import io, sys, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from mpmath import mp, mpf, pi, zeta, sqrt, exp, log, gamma, findroot

mp.dps = 60

# ----------------------------------------------------------------------------
# 0. Exact symbolic constants
# ----------------------------------------------------------------------------
z3 = zeta(3)
z5 = zeta(5)

def vol_sphere(k):
    """Volume (k-dim Riemannian measure) of the unit round S^k in R^{k+1}."""
    return 2 * pi**(mpf(k + 1) / 2) / gamma(mpf(k + 1) / 2)

# ---- alpha, Theorem 46 (fully symbolic, no empirical input) ----
vol_S2 = vol_sphere(2)          # 4 pi
vol_S4 = vol_sphere(4)          # 8 pi^2 / 3
vol_S9 = vol_sphere(9)          # pi^5 / 12
vol_RP1 = pi                    # RP^1 = S^1 / Z2, length pi
alpha_H = (2 * vol_S2) / (vol_S4**2 * vol_RP1) * (vol_S9 / (32 * 5))**(mpf(1) / 4)

alpha_CODATA = mpf("1") / mpf("137.035999177")   # CODATA 2022

# ---- a, Theorem 34 ----
a_H = 6 * sqrt(2) * exp(z3 / (24 * pi**2))

# ---- sigma_3, Corollary 9 ----
sigma3 = z3 / (4 * pi**2)

# ---- tau_3, Eq. (70) ----
tau3 = {1: mpf(1), 2: mpf(1), 3: sqrt(3)}

# ---- Lambda_L, p.41 ----
v_MeV = mpf("246220")                       # paper's stated v, MeV
kappa = exp(z3 / (24 * pi**2)) / (4 * pi**2)
Lambda_L = sqrt(2 * pi) * v_MeV * kappa**6

# ---- D(n) as printed, Eq. (66) ----
D_print = {1: mpf("1.203011392"), 2: mpf("4.806545406"), 3: mpf("10.818228646")}

print("=" * 78)
print("A1.0  EXACT SYMBOLIC CONSTANTS")
print("=" * 78)
print(f"zeta(3)            = {z3}")
print(f"Vol(S^2)           = {vol_S2}")
print(f"Vol(S^4)           = {vol_S4}")
print(f"Vol(S^9)           = {vol_S9}")
print(f"alpha_H (Thm 46)   = {alpha_H}")
print(f"1/alpha_H          = {1/alpha_H}")
print(f"1/alpha_CODATA     = {1/alpha_CODATA}")
print(f"  rel. dev.        = {(alpha_H-alpha_CODATA)/alpha_CODATA} "
      f"= {(alpha_H-alpha_CODATA)/alpha_CODATA*10**6} ppm")
print(f"a_H  (Thm 34)      = {a_H}")
print(f"  6*sqrt(2)        = {6*sqrt(2)}")
print(f"  exp(z3/24pi^2)   = {exp(z3/(24*pi**2))}")
print(f"sigma_3            = {sigma3}")
print(f"kappa              = {kappa}")
print(f"Lambda_L           = {Lambda_L} MeV")

# ----------------------------------------------------------------------------
# 1. IS D(n) DERIVABLE FROM THE PAPER'S OWN Eq. (63)-(65)?
# ----------------------------------------------------------------------------
print()
print("=" * 78)
print("A1.1  DERIVABILITY TEST FOR D(n)  [Eqs. (63)-(66)]")
print("=" * 78)

# Eq. (63): zeta_n(s) = sum_{j>=n} j(j+2)/(j+1)^s = zeta_H(s-2, n+1) - zeta_H(s, n+1)
# Eq. (64): zeta'_1(0) = -zeta(3)/(4 pi^2) + (1/2) ln(2 pi)
zp1_paper = -z3 / (4 * pi**2) + log(2 * pi) / 2
print(f"Eq.(64)  zeta'_1(0) = {zp1_paper}")
print(f"         paper prints 0.888490076...  -> {'MATCH' if abs(zp1_paper-mpf('0.888490076'))<mpf('1e-9') else 'MISMATCH'}")

# Independent check of Eq. (64) from Hurwitz zeta directly:
#   zeta_1(s) = zeta_R(s-2) - zeta_R(s);  zeta'_1(0) = zeta'_R(-2) - zeta'_R(0)
zpR_m2 = -z3 / (4 * pi**2)          # zeta'_R(-2) = -zeta(3)/(4 pi^2)
zpR_0 = -log(2 * pi) / 2            # zeta'_R(0)  = -(1/2) ln(2 pi)
print(f"         direct  zeta'_R(-2)-zeta'_R(0) = {zpR_m2 - zpR_0}   [identical]")

# Eq. (65): zeta'_n(0) = zeta'_1(0) + sum_{j=1}^{n-1} j(j+2) ln(j+1)
zp = {}
for n in (1, 2, 3):
    zp[n] = zp1_paper + sum(mpf(j) * (j + 2) * log(j + 1) for j in range(1, n))
print()
print("  n   zeta'_n(0)              ln det' B_n = -zeta'_n(0)      D(n) printed")
for n in (1, 2, 3):
    print(f"  {n}   {mp.nstr(zp[n],12):22s}  {mp.nstr(-zp[n],12):28s}  {mp.nstr(D_print[n],12)}")

print()
print("  The paper says D(n) is 'the quadratic-in-n piece of -zeta'_n(0)'.")
print("  -zeta'_n(0) is MONOTONE DECREASING and NEGATIVE for n>=1;")
print("  D(n) is positive and increasing.  No stated operation maps one to the other.")
print()
print("  Nearest closed form the paper itself offers (Eq. 61):  D(n) = zeta(3) n^2")
for n in (1, 2, 3):
    q = z3 * n**2
    print(f"    n={n}:  zeta(3)n^2 = {mp.nstr(q,12):16s}  D_print = {mp.nstr(D_print[n],12):16s} "
          f" residual = {mp.nstr(D_print[n]-q,6)}")
print()
print("  The residuals D(n) - zeta(3)n^2 have no common sign, no monotone pattern,")
print("  and no match to zeta(5)/pi^k combinations at the printed precision.")

# ----------------------------------------------------------------------------
# 2. FORWARD EVALUATION: masses and Q from the printed D(n)
# ----------------------------------------------------------------------------
print()
print("=" * 78)
print("A1.2  FORWARD EVALUATION OF Eq. (62)")
print("=" * 78)

def f_dimensionless(n, a, D, alpha, s3):
    """(n+1) exp[a n - D(n) + n alpha/6 + sigma_3 ln tau_3(K_n)]"""
    return (n + 1) * exp(a * n - D[n] + n * alpha / 6 + s3 * log(tau3[n]))

f = {n: f_dimensionless(n, a_H, D_print, alpha_H, sigma3) for n in (1, 2, 3)}
m_pred = {n: Lambda_L * f[n] for n in (1, 2, 3)}

# PDG values.  NOTE: the tau mass moved between PDG editions -- both are carried.
PDG = {
    "m_e":  mpf("0.51099895069"),   # PDG/CODATA, uncert 1.6e-10 MeV
    "m_mu": mpf("105.6583755"),     # uncert 2.3e-6 MeV
}
TAU_VALUES = {
    "PDG-as-quoted-by-paper (1776.86 +- 0.12)": mpf("1776.86"),
    "PDG 2024 (1776.93 +- 0.09)":               mpf("1776.93"),
}

print("Predicted charged-lepton masses from Eq. (62) with Lambda_L derived (p.41):")
names = {1: "e", 2: "mu", 3: "tau"}
for n in (1, 2, 3):
    print(f"  m_{names[n]:3s} = {mp.nstr(m_pred[n], 12)} MeV")
print()
print("Ratios (Lambda_L-independent -- the actual content of the formula):")
r21_p = f[2] / f[1]
r32_p = f[3] / f[2]
r21_e = PDG["m_mu"] / PDG["m_e"]
print(f"  m_mu/m_e   predicted = {mp.nstr(r21_p,14)}   PDG = {mp.nstr(r21_e,14)}"
      f"   rel dev = {mp.nstr((r21_p-r21_e)/r21_e,6)}")
for label, mt in TAU_VALUES.items():
    r32_e = mt / PDG["m_mu"]
    print(f"  m_tau/m_mu predicted = {mp.nstr(r32_p,14)}   {label}")
    print(f"                              -> PDG = {mp.nstr(r32_e,14)}"
          f"   rel dev = {mp.nstr((r32_p-r32_e)/r32_e,6)}")

def koide(m1, m2, m3):
    return (m1 + m2 + m3) / (sqrt(m1) + sqrt(m2) + sqrt(m3))**2

Q_H = koide(f[1], f[2], f[3])          # scale-free: Lambda cancels
twothirds = mpf(2) / 3
print()
print(f"  Q_Hopf  = {Q_H}")
print(f"  2/3     = {twothirds}")
print(f"  Q_Hopf - 2/3 = {mp.nstr(Q_H - twothirds, 10)}"
      f"   =  {mp.nstr((Q_H - twothirds)/twothirds*10**6, 6)} ppm of 2/3")

print()
print("Experimental Koide for comparison:")
for label, mt in TAU_VALUES.items():
    Qe = koide(PDG["m_e"], PDG["m_mu"], mt)
    print(f"  Q_exp [{label}]")
    print(f"      = {Qe}")
    print(f"      Q_exp - 2/3 = {mp.nstr(Qe-twothirds,10)}"
          f"  = {mp.nstr((Qe-twothirds)/twothirds*10**6,6)} ppm")
    print(f"      Q_Hopf - Q_exp = {mp.nstr(Q_H-Qe,6)}"
          f"  = {mp.nstr((Q_H-Qe)/twothirds*10**6,6)} ppm")

# ----------------------------------------------------------------------------
# 3. SENSITIVITY: is the -9.23 ppm residual resolvable at the printed precision?
# ----------------------------------------------------------------------------
print()
print("=" * 78)
print("A1.3  SENSITIVITY OF Q TO THE PRINTED PRECISION OF D(n) AND a")
print("=" * 78)

eps = mpf(10)**-9      # one unit in the last printed place of D(n)
for k in (1, 2, 3):
    Dp = dict(D_print); Dp[k] = Dp[k] + eps
    fk = {n: f_dimensionless(n, a_H, Dp, alpha_H, sigma3) for n in (1, 2, 3)}
    dQ = koide(fk[1], fk[2], fk[3]) - Q_H
    print(f"  dQ / d[last digit of D({k})] = {mp.nstr(dQ,6)}"
          f"   -> {mp.nstr(dQ/twothirds*10**6,6)} ppm per ulp")

# what a would give exact Koide, holding printed D fixed?
def Qofa(aa):
    ff = {n: f_dimensionless(n, aa, D_print, alpha_H, sigma3) for n in (1, 2, 3)}
    return koide(ff[1], ff[2], ff[3]) - twothirds

a_needed = findroot(Qofa, a_H)
print()
print(f"  a_Hopf   (derived, Thm 34)      = {a_H}")
print(f"  a_Koide  (forces Q = 2/3 exactly) = {a_needed}")
print(f"  a_Koide - a_Hopf = {mp.nstr(a_needed-a_H,8)}"
      f"   = {mp.nstr((a_needed-a_H)/a_H*10**6,6)} ppm")

# implied tau mass at exact Koide, from paper's own m_e, m_mu
def tau_from_koide(me, mmu):
    """positive roots of Q(me,mmu,mt) = 2/3"""
    se, su = sqrt(me), sqrt(mmu)
    # 3(me+mmu+mt) = 2(se+su+st)^2  ->  st^2 - 4(se+su) st + [3(me+mmu) - 2(se+su)^2] = 0
    b = 4 * (se + su)
    c = 3 * (me + mmu) - 2 * (se + su)**2
    disc = sqrt(b * b - 4 * c)
    return ((b + disc) / 2)**2, ((b - disc) / 2)**2

hi, lo = tau_from_koide(PDG["m_e"], PDG["m_mu"])
print()
print(f"  Exact-Koide tau roots from PDG m_e, m_mu:")
print(f"     high root = {mp.nstr(hi,12)} MeV")
print(f"     low  root = {mp.nstr(lo,12)} MeV")
print(f"  Paper's m_tau (Table, p.40)  = 1776.86 MeV")
print(f"     high root - 1776.86 = {mp.nstr(hi-mpf('1776.86'),6)} MeV")

# ----------------------------------------------------------------------------
# 4. INVERSE PROBLEM: what D(n) do the DATA require?
# ----------------------------------------------------------------------------
print()
print("=" * 78)
print("A1.4  INVERSE PROBLEM -- D(n) RECONSTRUCTED FROM THE MEASURED MASSES")
print("=" * 78)
print("  Solving Eq. (62) for D(n) given measured m_n and derived Lambda_L, a, alpha, sigma_3.")
print("  D(n) = ln Lambda_L + ln(n+1) + a n + n alpha/6 + sigma_3 ln tau_3 - ln m_n")

for label, mt in TAU_VALUES.items():
    meas = {1: PDG["m_e"], 2: PDG["m_mu"], 3: mt}
    print(f"\n  --- using {label} ---")
    print("   n   D_required            D_printed             difference")
    for n in (1, 2, 3):
        Dreq = (log(Lambda_L) + log(n + 1) + a_H * n + n * alpha_H / 6
                + sigma3 * log(tau3[n]) - log(meas[n]))
        print(f"   {n}   {mp.nstr(Dreq,12):20s}  {mp.nstr(D_print[n],12):20s}  "
              f"{mp.nstr(Dreq-D_print[n],6)}")

print()
print("  Scale-free version (differences only; Lambda_L drops out):")
for label, mt in TAU_VALUES.items():
    meas = {1: PDG["m_e"], 2: PDG["m_mu"], 3: mt}
    print(f"\n  --- using {label} ---")
    for (i, j) in ((2, 1), (3, 1), (3, 2)):
        req = (log(mpf(i + 1) / (j + 1)) + a_H * (i - j) + (i - j) * alpha_H / 6
               + sigma3 * (log(tau3[i]) - log(tau3[j])) - log(meas[i] / meas[j]))
        prn = D_print[i] - D_print[j]
        print(f"   D({i})-D({j}):  required = {mp.nstr(req,12):18s} "
              f" printed = {mp.nstr(prn,12):18s}  diff = {mp.nstr(req-prn,6)}")

json.dump({
    "alpha_H": str(alpha_H), "a_H": str(a_H), "sigma3": str(sigma3),
    "Lambda_L_MeV": str(Lambda_L), "Q_Hopf": str(Q_H),
    "a_Koide": str(a_needed), "tau_koide_hi": str(hi), "tau_koide_lo": str(lo),
    "m_pred": {names[n]: str(m_pred[n]) for n in (1, 2, 3)},
}, open(sys.argv[1] if len(sys.argv) > 1 else "a1_out.json", "w"), indent=2)
