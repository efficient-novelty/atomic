"""
A4 -- The S^9 neutrino sector: the paper's ONE parameter-free mass prediction.

Every coefficient in Eq. (127) is closed-form except zeta'_{Delta2}(0) = -0.41364,
which the paper states numerically only.  This script recomputes it from the paper's
own Eqs. (116)-(117) by exactly the prescription the paper gives, then evaluates the
whole spectrum.
"""
import io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", line_buffering=True)
import sympy as sp
from mpmath import mp, mpf, pi, sqrt, exp, log, zeta, mpmathify

mp.dps = 40

# ---------------------------------------------------------------------------
# 1. The multiplicity polynomial, Eq. (117), and the spectrum, Eq. (116)
# ---------------------------------------------------------------------------
k = sp.symbols("k")
d_expr = sp.together(k * (k + 1) * (k + 3) * (k + 4)**2 * (k + 5) * (k + 7) * (k + 8) / 720)

print("=" * 78)
print("A4.1  MULTIPLICITY AND SPECTRUM CHECKS")
print("=" * 78)
print(f"  d(k) = {sp.factor(d_expr)}")
for kk in range(1, 5):
    print(f"    d({kk}) = {sp.nsimplify(d_expr.subs(k, kk))}"
          f"     lambda_{kk} = (k+2)(k+6) = {(kk+2)*(kk+6)}")
print()
print("  SO(10) irrep [k-1,0,1,0,0] at k=1 is [0,0,1,0,0] = Lambda^3(10), dim 120.")
print(f"    d(1) computed = {sp.nsimplify(d_expr.subs(k,1))}   -> "
      f"{'MATCH' if sp.nsimplify(d_expr.subs(k,1)) == 120 else 'MISMATCH'}")
print()
print("  Standard Ikeda-Taniguchi spectrum for coexact p-forms on S^N:")
print("    eigenvalues (k+p)(k+N-1-p), k >= 1.  For N=9, p=2:  (k+2)(k+6).")
print("    -> Eq. (116) is CORRECT.")

# ---------------------------------------------------------------------------
# 2. zeta'_{Delta2}(0) by the paper's own prescription
# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A4.2  INDEPENDENT COMPUTATION OF zeta'_{Delta2}(0)")
print("=" * 78)
print("  Paper's prescription (p.50):")
print("    zeta'(0) = - sum_{k>=1} d(k) [ ln(k+2) + ln(k+6) ]")
print("  regularised by expanding d(k) in powers of (k+2) and of (k+6) and using")
print("  Hurwitz zeta derivatives.  Implementing exactly that:")
print()


def branch(shift, a_start):
    """
    Expand d(k) as a polynomial in u = k + shift; sum over k>=1 <=> u >= a_start.
    Return sum_m c_m * zeta_H'(-m, a_start), which regularises
       -sum_k d(k) ln(k+shift)   as   +F'(0).
    """
    u = sp.symbols("u")
    poly = sp.Poly(sp.expand(d_expr.subs(k, u - shift)), u)
    coeffs = {m: sp.Rational(c) for m, c in
              zip(range(poly.degree(), -1, -1), poly.all_coeffs())}
    total = mpf(0)
    rows = []
    for m, c in sorted(coeffs.items()):
        if c == 0:
            continue
        # d/ds of zeta_H(s-m, a) at s=0  =  zeta_H'(-m, a)
        zp = zeta(-mpf(m), mpf(a_start), 1)
        term = mpmathify(sp.Float(c, 40)) * zp
        rows.append((m, c, zp, term))
        total += term
    return total, rows, poly


FA, rowsA, polyA = branch(2, 3)   # u = k+2, so k = u-2; k>=1 -> u>=3
FB, rowsB, polyB = branch(6, 7)   # w = k+6, so k = w-6; k>=1 -> w>=7

print(f"  d as polynomial in u=k+2 : {sp.factor(polyA.as_expr())}")
print(f"  branch A  sum_m c_m zeta_H'(-m, 3) = {mp.nstr(FA, 12)}")
print(f"  branch B  sum_m e_m zeta_H'(-m, 7) = {mp.nstr(FB, 12)}")
zp_total = FA + FB
print()
print(f"  zeta'_{{Delta2}}(0) = A + B = {mp.nstr(zp_total, 12)}")
print(f"  paper states                 -0.41364")
print(f"  difference                   {mp.nstr(zp_total + mpf('0.41364'), 8)}")
print(f"  ratio                        {mp.nstr(zp_total / mpf('-0.41364'), 8)}")
print()
print("  METHOD CAVEAT (report this): the true spectral zeta is")
print("     zeta(s) = sum_k d(k) [(k+2)(k+6)]^{-s},")
print("  whose analytic continuation to s=0 is NOT the sum of the two separately")
print("  continued single-shift pieces.  The paper's prescription is a non-standard")
print("  split regularisation; the two agree only if the cross terms in the")
print("  Mellin continuation vanish, which is not established anywhere in the paper.")

# ---------------------------------------------------------------------------
# 3. Full neutrino spectrum, Eq. (127)
# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A4.3  THE NEUTRINO SPECTRUM, Eq. (127)")
print("=" * 78)

z3, z5 = zeta(3), zeta(5)
v_MeV = mpf("246220")
tau9 = {1: mpf(1), 2: mpf(4), 3: mpf(3)}
C9 = -(z3 / 8) * (1 + z3 / 28)
beta9 = z5 / (8 * pi**4)
sigma9 = z3 / (8 * pi**2)

print(f"  C_9     = -(z3/8)(1+z3/28) = {mp.nstr(C9, 10)}   paper: -0.15671")
print(f"  beta_9  = z5/(8 pi^4)      = {mp.nstr(beta9, 10)}   paper: 1.331e-3")
print(f"  sigma_9 = z3/(8 pi^2)      = {mp.nstr(sigma9, 10)}   paper: 1.522e-2")


def spectrum(zp, a9_mode):
    kappa9 = exp(zp / 16) / (32 * pi**5)
    Lam9 = sqrt(2 * pi) * v_MeV * kappa9**4                      # Eq. (120)
    a9 = sqrt(5) if a9_mode == "sqrt5" else exp(zp / 16) * sqrt(5)
    m = {}
    for n in (1, 2, 3):
        m[n] = Lam9 * (n + 1) * exp(a9 * n + C9 * n**2
                                    + beta9 * n * (n + 1) / 2
                                    + sigma9 * log(tau9[n]))
    return Lam9, a9, {n: m[n] * 10**6 for n in m}                # MeV -> eV


for zp_used, tag in [(mpf("-0.41364"), "paper's stated -0.41364"),
                     (zp_total, f"recomputed {mp.nstr(zp_total,8)}")]:
    print()
    print(f"  --- zeta' = {tag} ---")
    for mode, mlabel in [("sqrt5", "a_9 = sqrt(5) exactly (as the paper declares)"),
                         ("full", "a_9 = exp(zeta'/16) sqrt(5) (as the paper DERIVES)")]:
        Lam9, a9, m = spectrum(zp_used, mode)
        dm21 = (m[2]**2 - m[1]**2)
        dm31 = (m[3]**2 - m[1]**2)
        print(f"    {mlabel}")
        print(f"       Lambda_9 = {mp.nstr(Lam9,8)} MeV   a_9 = {mp.nstr(a9,10)}")
        print(f"       m_nu = {mp.nstr(m[1],6)}, {mp.nstr(m[2],6)}, {mp.nstr(m[3],6)} eV"
              f"    sum = {mp.nstr(m[1]+m[2]+m[3],6)} eV")
        print(f"       Dm2_21 = {mp.nstr(dm21,6)} eV^2    Dm2_31 = {mp.nstr(dm31,6)} eV^2")

print()
print("  Paper's claimed table:  m_nu = 0.000970, 0.008708, 0.049604 eV,")
print("                          Dm2_21 = 7.489e-5, Dm2_31 = 2.460e-3 eV^2,")
print("                          sum = 0.059282 eV")
print("  PDG:                    Dm2_21 = (7.53 +- 0.18)e-5, Dm2_31 = (2.453 +- 0.033)e-3")

# ---------------------------------------------------------------------------
# 4. The a_9 absorption problem
# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A4.4  THE a_9 ABSORPTION PROBLEM")
print("=" * 78)
corr = exp(mpf("-0.41364") / 16)
print(f"  The paper computes kappa_9 * N_9 = exp(zeta'/16) = {mp.nstr(corr,8)},")
print(f"  writes a_9 = exp(zeta'/16) sqrt(5) = {mp.nstr(corr*sqrt(5),10)},")
print(f"  then DISCARDS the factor and declares a_9 = sqrt(5) = {mp.nstr(sqrt(5),10)},")
print(f"  'absorbing the correction into the O(1) prefactor of the determinant'.")
print()
print(f"  But a_9 multiplies n.  Shifting a_9 by delta multiplies m_n by exp(n delta),")
print(f"  which is n-DEPENDENT and therefore CANNOT be absorbed into an n-independent")
print(f"  prefactor.  delta = {mp.nstr(sqrt(5)*(1-corr),8)}; the induced ratio change")
print(f"  between n=1 and n=3 is exp(2 delta) = {mp.nstr(exp(2*sqrt(5)*(1-corr)),8)}.")
_, _, mA = spectrum(mpf("-0.41364"), "sqrt5")
_, _, mB = spectrum(mpf("-0.41364"), "full")
print()
print(f"  Consequence for the observables:")
print(f"    Dm2_31  with a_9 = sqrt(5)          : {mp.nstr(mA[3]**2-mA[1]**2, 6)} eV^2")
print(f"    Dm2_31  with the derived a_9        : {mp.nstr(mB[3]**2-mB[1]**2, 6)} eV^2")
print(f"    ratio                              : "
      f"{mp.nstr((mA[3]**2-mA[1]**2)/(mB[3]**2-mB[1]**2), 6)}")

# ---------------------------------------------------------------------------
# 5. Dirac Yukawas at the Two-Law minimality tier
# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A4.5  IMPLIED DIRAC YUKAWA COUPLINGS  y = sqrt(2) m / v")
print("=" * 78)
v_eV = mpf("246.21965") * 10**9
for label, ms in [("paper's quoted spectrum",
                   [mpf("0.000970"), mpf("0.008708"), mpf("0.049604")])]:
    print(f"  {label}:")
    for i, m in enumerate(ms, 1):
        print(f"    y_nu{i} = {mp.nstr(sqrt(2)*m/v_eV, 6)}")
    print(f"    sum m_nu = {mp.nstr(sum(ms), 6)} eV")
