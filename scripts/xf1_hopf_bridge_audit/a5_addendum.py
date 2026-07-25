"""
A5 -- Addendum checks (audit v1.1), added after the adversarial-verification
synthesis pass.  Each item here was raised by an independent lane and is
re-derived from scratch in this script; nothing is taken on report.
"""
import io, sys, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", line_buffering=True)
from mpmath import mp, mpf, pi, sqrt, exp, log, zeta, gamma

mp.dps = 40
z3 = zeta(3)
PDF = "docs/ssrn-6975959.pdf"          # relative to the repo root


def paper_text():
    """Plain text of all 78 pages, extracted fresh from the PDF."""
    import os
    here = os.path.dirname(os.path.abspath(__file__))
    pdf = os.path.join(here, "..", "..", PDF)
    import pypdf
    return "".join((pg.extract_text() or "") for pg in pypdf.PdfReader(pdf).pages)


def vol(k):
    return 2 * pi**(mpf(k + 1) / 2) / gamma(mpf(k + 1) / 2)


# ---------------------------------------------------------------------------
print("=" * 78)
print("A5.1  THE 'QUADRATIC PIECE' READING OF D(n) IS CLOSED OFF EXACTLY")
print("=" * 78)
zp1 = -z3 / (4 * pi**2) + log(2 * pi) / 2
Dz = {n: -(zp1 + sum(mpf(j) * (j + 2) * log(j + 1) for j in range(1, n))) for n in (1, 2, 3)}
sd = Dz[3] - 2 * Dz[2] + Dz[1]
print("  Any three-point quadratic fit c2*n^2 to -zeta'_n(0) has its curvature")
print("  fixed by the second difference, which is a CLOSED FORM:")
print(f"    2nd difference of -zeta'_n(0) = {mp.nstr(sd, 20)}")
print(f"    -(8 ln 3 - 3 ln 2)            = {mp.nstr(-(8*log(3)-3*log(2)), 20)}"
      f"   equal: {abs(sd + (8*log(3)-3*log(2))) < mpf(10)**-30}")
print(f"    => c2 = {mp.nstr(sd/2, 20)}")
print(f"    zeta(3) would be required     = {mp.nstr(z3, 20)}")
print(f"    ratio = {mp.nstr(sd/2/z3, 8)}  -- WRONG SIGN and 2.79x too large.")
print("  So no quadratic extracted from Eq. (65) can be D(n) ~ zeta(3)n^2.")

print()
print("=" * 78)
print("A5.2  THE PAPER'S ASYMPTOTIC CLAIM 'D(n) ~ zeta(3) n^2' IS FALSE")
print("=" * 78)
print("  Eq. (66) note, p.39: 'For large n, D(n) ~ zeta(3) n^2.'")
print("  The Eq. (65) object grows like -(1/3) n^3 ln n, not like +zeta(3) n^2:")
print("     n     -zeta'_n(0)/(zeta(3)n^2)     -zeta'_n(0)/(-(1/3)n^3 ln n)")
for n in (10, 100, 1000):
    val = -(zp1 + sum(mpf(j) * (j + 2) * log(j + 1) for j in range(1, n)))
    print(f"  {n:5d}     {mp.nstr(val/(z3*n**2), 8):18s}          "
          f"{mp.nstr(val/(-(mpf(1)/3)*n**3*log(n)), 8)}")

print()
print("=" * 78)
print("A5.3  Thm 46 STEP 2 NAMES A COSET WHOSE VOLUME IT DOES NOT USE")
print("=" * 78)
print("  p.61 verbatim: 'two copies of Vol(S^4) (the coset volume Vol(SU(3)/SU(2)))'.")
print("  But SU(3)/SU(2) is diffeomorphic to S^5, not S^4.")
print(f"    Vol(S^4) = 8pi^2/3 = {mp.nstr(vol(4), 12)}")
print(f"    Vol(S^5) = pi^3    = {mp.nstr(vol(5), 12)}")
a_S4 = (2*vol(2))/(vol(4)**2*pi)*(vol(9)/160)**(mpf(1)/4)
a_S5 = (2*vol(2))/(vol(5)**2*pi)*(vol(9)/160)**(mpf(1)/4)
print(f"    1/alpha using Vol(S^4)  (as PRINTED) = {mp.nstr(1/a_S4, 15)}")
print(f"    1/alpha using Vol(S^5)  (as NAMED)   = {mp.nstr(1/a_S5, 15)}")
print("  The recipe works only under the printed symbol, not under the named object.")
print()
pref = 9/(8*pi**4)
NB = (vol(9)/160)**(mpf(1)/4)
print("  How much of alpha actually comes from S^9?  Split ln alpha:")
print(f"    n-INdependent prefactor 9/(8 pi^4) = {mp.nstr(pref, 15)}"
      f"   -> {mp.nstr(log(pref)/log(pref*NB)*100, 6)} % of ln alpha")
print(f"    n-dependent  N_B = (Vol(S^9)/160)^(1/4) = {mp.nstr(NB, 15)}"
      f"   -> {mp.nstr(log(NB)/log(pref*NB)*100, 6)} %")
print("  So ~91% of ln alpha is carried by a factor independent of the shell,")
print("  which weakens 'alpha is derived FROM S^9' considerably.")

print()
print("=" * 78)
print("A5.4  TABLE 1 IS INCOMPATIBLE WITH THE PAPER'S OWN g (Thm 35)")
print("=" * 78)
alpha_H = a_S4
s2H = 3/(4*pi)
g = sqrt(4*pi*alpha_H)/sqrt(s2H)
v = mpf("246.21965")
mW = g*v/2
print(f"  Thm 35 gives g = {mp.nstr(g, 12)};  Axiom 1 gives v = {v} GeV.")
print(f"  Tree-level  m_W = g v / 2 = {mp.nstr(mW, 12)} GeV")
print(f"  Table 1     m_W           = 80.3695 GeV")
print(f"  discrepancy = {mp.nstr((mW-mpf('80.3695'))/mpf('80.3695')*100, 6)} %"
      "     <-- 5%, not the 0.055% of the Lambda_B gap")
s2_tab = 1 - (mpf('80.3695')/mpf('91.1878'))**2
print()
print(f"  Table-1 on-shell sin^2 = 1 - m_W^2/m_Z^2 = {mp.nstr(s2_tab, 12)}")
print(f"  Thm 35 / Eq.(75) sin^2 = 3/(4 pi)        = {mp.nstr(s2H, 12)}")
print(f"    relative to 3/(4pi):      {mp.nstr((s2_tab-s2H)/s2H*100, 6)} %")
print(f"    relative to the on-shell: {mp.nstr((s2_tab-s2H)/s2_tab*100, 6)} %")
print("  The boson table and the gauge-coupling theorem are two disconnected")
print("  chains that disagree at the 5-7% level about the same two observables.")

print()
print("=" * 78)
print("A5.5  OBLIGATION O8 (ONE COMMON EXTRACTOR) FAILS IN THE PAPER'S OWN TABLE")
print("=" * 78)
print("  Summary table, p.52 -- the three massive shells use three different")
print("  extractors, which is exactly what O8 forbids:")
print("     quantity          S^3            S^5            S^9")
print("     action type       CS             CS             L^2 torsion")
print("     operator          B=*d on Om^1   B=*d on Om^2   Delta_2 on Om^2   (order 1 -> 2)")
print("     framing  l        6 (knot)       6 (knot)       16 (contact chirality)")
print("     det exponent      -1/2           -1/2           +1/2")
print("  A single public-record extractor cannot change operator order, framing")
print("  rule and determinant sign between sectors of the same theory.")
print()
print("  Also: Eq. (112) S_9[T] = gamma_9 Int T ^ *T is a POSITIVE-DEFINITE")
print("  quadratic form in a COMMUTING 2-form field.  Gaussian integration gives")
print("  (det)^{-1/2}.  The paper's Z = (det Delta_2)^{+1/2} 'fermionic sign'")
print("  (p.49) has no spinor field to justify it.")

print()
print("=" * 78)
print("A5.6  WHAT THE PAPER DOES AND DOES NOT SAY (full-text search)")
print("=" * 78)
t = paper_text()
low = t.lower()
for term in ["koide", "ppm", "proton decay", "georgi", "grand unif"]:
    n = len(re.findall(re.escape(term), low))
    print(f"    {term!r:15s} occurrences: {n}"
          + ("   (bibliography entry [16] only)" if term == "grand unif" and n else ""))
print("  Consequences: (i) the paper makes no Koide claim, so it can neither")
print("  support nor burn XF-C; (ii) 'consistent with no intermediate GUT group'")
print("  is vacuous as a statement ABOUT the paper -- it never raises the issue;")
print("  (iii) its single Z_6 mention is imported from Baez-Huerta [16].")

print()
print("=" * 78)
print("A5.7  CORRECTION TO THE v1 QUOTATION OF l_9")
print("=" * 78)
i = low.find("dimc s")
print("  PDF text as extracted (superscripts flattened):")
print("    " + " ".join(t[i-40:i+150].split()))
print("  Correct reading: dim_C S+- = 2^((10-2)/2) = 2^4 = 16, and")
print("  l_9 = 2^(8/2) = 16.  The v1 dossier's '2^8/2 = 16' was a misquote")
print("  (2^8/2 = 128).  THE NUMBER 16 IS FINE; only the justification fails:")
print("  S^9 = Spin(10)/Spin(9) is odd-dimensional and Spin(9) has a UNIQUE")
print("  16-dimensional spinor irrep, so there is no chirality split to count.")
print("  Note 16 is available U(5)-covariantly as dim Lambda^even(C^5)")
print("  = 1 + 10 + 5 = 16 -- the right number by a route the paper does not use.")
