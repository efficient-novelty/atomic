"""
A3 -- Complete four-dimensional EFT matching calculation.

Question: can the paper's boundary data be placed at ANY single matching scale
consistent with Standard-Model running, and how much of the factor-3.7 gap against
the Two-Law equal-trace crossing (1.1e13 GeV) is a placement artifact?
"""
import io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from mpmath import mp, mpf, pi, sqrt, exp, log, zeta, gamma, odefun, findroot

mp.dps = 40

# ---------------------------------------------------------------------------
# Paper's boundary data
# ---------------------------------------------------------------------------
def vol_sphere(k):
    return 2 * pi**(mpf(k + 1) / 2) / gamma(mpf(k + 1) / 2)

alpha_H = (2 * vol_sphere(2)) / (vol_sphere(4)**2 * pi) * (vol_sphere(9) / 160)**(mpf(1)/4)
s2_H = 3 / (4 * pi)                       # Eq. (75)
e_H = sqrt(4 * pi * alpha_H)
g2_H = e_H / sqrt(s2_H)
gY_H = e_H / sqrt(1 - s2_H)
g1_H = sqrt(mpf(5) / 3) * gY_H

print("=" * 78)
print("A3.1  THE PAPER'S OWN ARITHMETIC")
print("=" * 78)
print(f"  1/alpha_H          = {mp.nstr(1/alpha_H, 12)}")
print(f"  sin^2 theta_H = 3/4pi = {mp.nstr(s2_H, 12)}")
print(f"  e  = sqrt(4 pi alpha_H)  = {mp.nstr(e_H, 10)}")
print(f"  g  = e/sin theta_W       = {mp.nstr(g2_H, 10)}   paper Eq.(76) prints 0.6205"
      f"   -> error {mp.nstr((mpf('0.6205')-g2_H)/g2_H*100, 4)} %")
print(f"  g' = e/cos theta_W       = {mp.nstr(gY_H, 10)}   paper Eq.(77) prints 0.3469"
      f"   -> error {mp.nstr((mpf('0.3469')-gY_H)/gY_H*100, 4)} %")
print(f"  g1 = sqrt(5/3) g'        = {mp.nstr(g1_H, 10)}")

# ---------------------------------------------------------------------------
# Measured anchors at M_Z (MS-bar, full SM above M_Z)
# ---------------------------------------------------------------------------
MZ = mpf("91.1876")
alpha_MZ = 1 / mpf("127.951")
s2_MZ = mpf("0.23122")
alphas_MZ = mpf("0.1179")
e_MZ = sqrt(4 * pi * alpha_MZ)
g2_MZ = e_MZ / sqrt(s2_MZ)
gY_MZ = e_MZ / sqrt(1 - s2_MZ)
g1_MZ = sqrt(mpf(5) / 3) * gY_MZ
g3_MZ = sqrt(4 * pi * alphas_MZ)

print()
print("=" * 78)
print("A3.2  MEASURED MS-BAR ANCHORS AT M_Z")
print("=" * 78)
print(f"  g1(M_Z) = {mp.nstr(g1_MZ,10)}   g2(M_Z) = {mp.nstr(g2_MZ,10)}   "
      f"gY(M_Z) = {mp.nstr(gY_MZ,10)}   g3(M_Z) = {mp.nstr(g3_MZ,10)}")

# ---------------------------------------------------------------------------
# A3.3  THE DECISIVE TEST: is there ANY single scale for the paper's g and g'?
# ---------------------------------------------------------------------------
b1, b2, b3 = mpf(41) / 10, -mpf(19) / 6, mpf(-7)      # GUT-normalised g1
bY = mpf(41) / 6                                       # non-GUT-normalised gY

def scale_for(target, anchor, b, mu0=MZ):
    """One-loop: solve 1/g^2(mu) = 1/g^2(mu0) - b/(8 pi^2) ln(mu/mu0) for mu."""
    lhs = 1 / target**2 - 1 / anchor**2
    return mu0 * exp(-lhs * 8 * pi**2 / b)

mu_g2 = scale_for(g2_H, g2_MZ, b2)
mu_gY = scale_for(gY_H, gY_MZ, bY)

print()
print("=" * 78)
print("A3.3  DECISIVE TEST -- IS THERE A SINGLE MATCHING SCALE?")
print("=" * 78)
print("  Treat the paper's g and g' as MS-bar couplings of the unbroken SM at some")
print("  common scale mu_0, and solve for mu_0 separately from each.")
print()
print(f"    g2 = {mp.nstr(g2_H,10)}  requires  mu = {mp.nstr(mu_g2,6)} GeV")
print(f"       (b2 = -19/6 < 0, so g2 falls with mu; g2_H < g2(M_Z) forces mu > M_Z)")
print(f"    gY = {mp.nstr(gY_H,10)}  requires  mu = {mp.nstr(mu_gY,6)} GeV")
print(f"       (bY = +41/6 > 0, so gY rises with mu; gY_H < gY(M_Z) forces mu < M_Z)")
print()
print(f"    ratio of the two required scales = {mp.nstr(mu_g2/mu_gY, 6)}"
      f"   ( {mp.nstr(log(mu_g2/mu_gY)/log(10),4)} decades )")
print("    ==> NO single matching scale exists in the unbroken SM.  The two couplings")
print("        must run in OPPOSITE directions to reach the paper's values.")

print()
print("  Alternative reading: the pair is the measured Q^2 -> 0 pair.")
low = {"alpha^-1(0)": mpf("137.035999177"),
       "sin^2 theta_W(0)_MSbar (PDG)": mpf("0.23863")}
print(f"    paper 1/alpha_H      = {mp.nstr(1/alpha_H,12)}   "
      f"measured 1/alpha(0) = {low['alpha^-1(0)']}   "
      f"dev = {mp.nstr((1/alpha_H-low['alpha^-1(0)'])/low['alpha^-1(0)']*10**6,4)} ppm")
print(f"    paper 3/(4 pi)       = {mp.nstr(s2_H,12)}   "
      f"measured sin^2(0)   = {low['sin^2 theta_W(0)_MSbar (PDG)']}          "
      f"dev = {mp.nstr((s2_H-low['sin^2 theta_W(0)_MSbar (PDG)'])/low['sin^2 theta_W(0)_MSbar (PDG)']*100,4)} %")
print(f"    paper 3/(4 pi) vs sin^2(M_Z)_MSbar = {s2_MZ}   "
      f"dev = {mp.nstr((s2_H-s2_MZ)/s2_MZ*100,4)} %")
print("    Both paper numbers sit on the measured Q^2 -> 0 pair, not on any high scale.")

# ---------------------------------------------------------------------------
# A3.4  mu_X under several placements, one loop and two loop
# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A3.4  THE g1 = g2 CROSSING UNDER DIFFERENT PLACEMENTS")
print("=" * 78)

def cross_1loop(g1_0, g2_0, mu0):
    lhs = 1 / g1_0**2 - 1 / g2_0**2
    t = lhs * 8 * pi**2 / (b1 - b2)
    mu = mu0 * exp(t)
    gX = 1 / sqrt(1 / g1_0**2 - b1 / (8 * pi**2) * t)
    return mu, gX

# two-loop SM (one Higgs doublet, 3 generations, top Yukawa only)
B = [[mpf(199)/50, mpf(27)/10, mpf(44)/5],
     [mpf(9)/10,   mpf(35)/6,  mpf(12)],
     [mpf(11)/10,  mpf(9)/2,   mpf(-26)]]
C = [mpf(17)/10, mpf(3)/2, mpf(2)]          # y_t^2 coefficients
bvec = [b1, b2, b3]

def rhs(t, y):
    g1, g2, g3, yt = y
    g = [g1, g2, g3]
    out = []
    for i in range(3):
        one = bvec[i] * g[i]**3
        two = g[i]**3 / (16 * pi**2) * (sum(B[i][j] * g[j]**2 for j in range(3))
                                        - C[i] * yt**2)
        out.append((one + two) / (16 * pi**2))
    dyt = yt * ((mpf(9)/2) * yt**2
                - (mpf(17)/20 * g1**2 + mpf(9)/4 * g2**2 + 8 * g3**2)) / (16 * pi**2)
    return [out[0], out[1], out[2], dyt]

yt_MZ = mpf("0.95")     # approx MS-bar top Yukawa at M_Z (m_t ~ 163 GeV running mass)

def cross_2loop(g1_0, g2_0, g3_0, yt_0, mu0):
    f = odefun(rhs, 0, [g1_0, g2_0, g3_0, yt_0], tol=mpf(10)**-25)
    def diff(t):
        y = f(t)
        return y[0] - y[1]
    t = findroot(diff, mpf(25))
    y = f(t)
    return mu0 * exp(t), y[0], y[1], y[2]

placements = [
    ("Measured M_Z anchors (the correct EFT completion)", g1_MZ, g2_MZ, MZ),
    ("Paper's undressed data placed at mu_0 = v = 246.21965 GeV", g1_H, g2_H, mpf("246.21965")),
    ("Paper's undressed data placed at mu_0 = M_Z", g1_H, g2_H, MZ),
]
print("  ONE LOOP:")
res1 = {}
for name, a, b, mu0 in placements:
    mu, gX = cross_1loop(a, b, mu0)
    res1[name] = mu
    print(f"    {name}")
    print(f"        mu_X = {mp.nstr(mu, 6)} GeV      g_X = {mp.nstr(gX, 6)}")

print()
print("  TWO LOOP (SM, one Higgs doublet, top Yukawa included):")
mu2, g1x, g2x, g3x = cross_2loop(g1_MZ, g2_MZ, g3_MZ, yt_MZ, MZ)
print(f"    Measured M_Z anchors:  mu_X = {mp.nstr(mu2,6)} GeV   "
      f"g_X = {mp.nstr(g1x,6)}   g3(mu_X) = {mp.nstr(g3x,6)}")

print()
print("  DECOMPOSITION OF THE FACTOR ~3.7 THE USER OBSERVED:")
mu_meas = res1["Measured M_Z anchors (the correct EFT completion)"]
mu_hyb = res1["Paper's undressed data placed at mu_0 = v = 246.21965 GeV"]
mu_pMZ = res1["Paper's undressed data placed at mu_0 = M_Z"]
print(f"    hybrid / measured                    = {mp.nstr(mu_hyb/mu_meas, 6)}")
print(f"      of which reference-scale shift v/M_Z = {mp.nstr(mpf('246.21965')/MZ, 6)}")
print(f"      of which coupling-offset factor      = {mp.nstr(mu_pMZ/mu_meas, 6)}")
print(f"      product                              = "
      f"{mp.nstr((mpf('246.21965')/MZ)*(mu_pMZ/mu_meas), 6)}")
print("    ==> the factor is entirely a PLACEMENT artifact; nothing physical is")
print("        being measured by the discrepancy.")

# ---------------------------------------------------------------------------
# A3.5  sin^2 theta_W = 3/8 at any crossing -- identity, not prediction
# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A3.5  sin^2(theta_W) = 3/8 AT ANY g1 = g2 CROSSING -- AN IDENTITY")
print("=" * 78)
print("  g1^2 = (5/3) gY^2  =>  at g1 = g2:  gY^2 = (3/5) g2^2")
print("  sin^2 theta_W = gY^2/(g2^2 + gY^2) = (3/5)/(1 + 3/5) = 3/8")
for name, a, b, mu0 in placements:
    mu, gX = cross_1loop(a, b, mu0)
    gY_X = sqrt(mpf(3) / 5) * gX
    s2 = gY_X**2 / (gX**2 + gY_X**2)
    print(f"    {name[:44]:46s} sin^2 = {mp.nstr(s2, 20)}")
print("  Identical to 3/8 in every placement.  It carries NO information about mu_X.")

# ---------------------------------------------------------------------------
# A3.6  alpha_s from Eq. (78)
# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A3.6  THE STRONG COUPLING, Eq. (78)")
print("=" * 78)
N3 = 4 * pi**2
N5 = 8 * pi**3
pref = (mpf(8) / 3) * (N3 / N5)
print(f"  alpha_s^geom = (dimSU(3)/dimSU(2)) (N_3/N_5) alpha_eff")
print(f"               = (8/3)(4pi^2 / 8pi^3) alpha_eff = (4/(3 pi)) alpha_eff")
print(f"  prefactor 4/(3 pi) = {mp.nstr(pref, 12)}")
for label, ae in [("alpha_eff = alpha_H", alpha_H),
                  ("alpha_eff = alpha(M_Z) = 1/127.951", alpha_MZ),
                  ("alpha_eff = 1 (maximal defensible)", mpf(1))]:
    print(f"    {label:38s} -> alpha_s^geom = {mp.nstr(pref*ae, 8)}")
print()
print(f"  Remark 24 claims alpha_s ~ 1 at the geometric scale, running to 0.118 at M_Z.")
print(f"    alpha_eff needed for alpha_s^geom = 1     : {mp.nstr(1/pref, 8)}")
print(f"    alpha_eff needed for alpha_s^geom = 0.1179: {mp.nstr(alphas_MZ/pref, 8)}")
print(f"    discrepancy factor vs alpha_eff = alpha_H : "
      f"{mp.nstr(1/(pref*alpha_H), 6)}  (for the '~1' claim)")
print("  Eq. (78) with any electromagnetic-strength alpha_eff gives alpha_s ~ 3e-3,")
print("  which is BELOW the measured 0.1179, so the claimed 'decreasing at higher")
print("  spectral levels' behaviour has the wrong SIGN as well as the wrong size.")

# ---------------------------------------------------------------------------
# A3.7  Does Eq. (71) reproduce the paper's OWN boson table?
# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A3.7  THE BOSON SECTOR, Eq. (71) vs TABLE 1")
print("=" * 78)
from mpmath import sin as _sin
v_MeV = mpf("246220")
r = mpf(8)                                    # r = k + 2
Z_CS = sqrt(2 / r) * _sin(pi / r)             # SU(2)_k Chern-Simons partition function
Lam_B = v_MeV * Z_CS * exp(-2 * alpha_H)      # p.41
print(f"  v * sqrt(2/r) sin(pi/r) = {mp.nstr(v_MeV*Z_CS, 8)} MeV      paper: 47112")
print(f"  Lambda_B = that * e^-2a = {mp.nstr(Lam_B, 8)} MeV      paper: 46429")

T_W = (sqrt(3) / 2) * exp(-alpha_H * sqrt(2) / pi + sqrt(3) * alpha_H / (2 * pi))   # Eq.(72)
r_f = r + sqrt(3) * alpha_H / (2 * pi)
T_Z = _sin(4 * pi / r_f) / (4 * _sin(pi / r_f))                                     # Eq.(73)
T_H = (mpf(2) / 3) * exp(-3 * alpha_H * sqrt(2) / pi + 9 * sqrt(3) * alpha_H / (2 * pi))  # Eq.(74)
print(f"  T_W = {mp.nstr(T_W,10)}   r_f = {mp.nstr(r_f,10)} (paper 8.002012)")
print(f"  T_Z = {mp.nstr(T_Z,10)}   T_H = {mp.nstr(T_H,10)}")
print()
claimed = {"W": mpf("80369.5"), "Z": mpf("91187.8"), "H": mpf("125225")}
pdg = {"W": (mpf("80369"), mpf("13")), "Z": (mpf("91187.6"), mpf("2.1")),
       "H": (mpf("125200"), mpf("110"))}
print("  m_B(n) = Lambda_B (n+1) exp(n alpha/6) T_B(n):")
ratios = []
for name, n, T in [("W", 1, T_W), ("Z", 2, T_Z), ("H", 3, T_H)]:
    m = Lam_B * (n + 1) * exp(n * alpha_H / 6) * T
    c = claimed[name]; p, e = pdg[name]
    ratios.append(m / c)
    print(f"    m_{name} = {mp.nstr(m,9):12s} MeV   Table 1: {c}"
          f"   formula/table - 1 = {mp.nstr((m-c)/c*100,4)} %"
          f"   pull vs PDG: {mp.nstr((m-p)/e,4)} sigma  (paper claims "
          f"{'+0.04' if name=='W' else '+0.11' if name=='Z' else '+0.23'})")
print()
print(f"  The three overshoot ratios are {[mp.nstr(x,8) for x in ratios]}")
print(f"  -- a single CONSTANT factor, i.e. a pure Lambda_B normalisation gap.")
print(f"  Lambda_B required by Table 1 = {mp.nstr(Lam_B/ratios[0], 9)} MeV,")
print(f"  versus the {mp.nstr(Lam_B,9)} MeV the paper derives: an undisclosed")
print(f"  rescaling by {mp.nstr(1/ratios[0], 10)}.")
print(f"  The paper's p.41 text concedes a 0.054% residual and calls it O(alpha^2);")
print(f"  alpha^2 = {mp.nstr(alpha_H**2*100,4)} %, an order of magnitude too small.")
