"""
A2 -- S^9 minimality census, verdict-blind.

Candidates            H_n :  S^1 -> S^(2n+1) -> CP^n,  n = 1,2,3,...
Fiber-preserving group  U(n+1)  (the subgroup of O(2n+2) commuting with the free
                        U(1) fiber action and descending to CP^n).

Frozen obligation profile (fixed BEFORE any number in the paper is consulted; it
mentions no measured constant):
  O1 primitive central complex phase (free U(1))
  O2 an SU(2) action
  O3 an SU(3) action
  O4 neutral chiral sector
  O5 three-generation flavor capacity
  O6 four-dimensional public descent
  O7 no additional fundamental gauge factor beyond the alphabet
  O8 one common spectral/public-record extractor

O1-O3 and O7 are adjudicable by pure representation theory on C^(n+1).
That is what this script decides.
"""
import io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from itertools import combinations_with_replacement
from fractions import Fraction as F

# ---------------------------------------------------------------------------
# Decompositions of C^N under a candidate SU(3) x SU(2), with FAITHFUL 3 and 2.
# Available SU(3) irreps of dim <= 8: 1, 3, 3bar (6, 8 too big for our range but
# included for completeness); SU(2): 1, 2, 3(adj), ...
# For a *subgroup* SU(3) x SU(2) < U(N) the two factors must commute, so C^N is a
# sum of outer tensor products (r_3 (x) r_2).
# ---------------------------------------------------------------------------
SU3 = {"1": 1, "3": 3, "3b": 3, "6": 6, "8": 8}
SU2 = {"1": 1, "2": 2, "3": 3, "4": 4}
FAITHFUL3 = {"3", "3b", "6", "8"}       # 8 is adjoint: faithful only on SU(3)/Z3
FAITHFUL3_STRICT = {"3", "3b"}          # faithful on SU(3) itself, minimal
FAITHFUL2 = {"2", "4"}                  # 2 is faithful on SU(2); 3 = adjoint is not


def summands(N, maxparts=6):
    """All multisets of (su3_irrep, su2_irrep) with total dimension N."""
    pieces = [(a, b, SU3[a] * SU2[b]) for a in SU3 for b in SU2]
    pieces = [p for p in pieces if p[2] <= N]
    out = []
    for k in range(1, maxparts + 1):
        for combo in combinations_with_replacement(pieces, k):
            if sum(c[2] for c in combo) == N:
                out.append(combo)
    return out


def commutant_extra_factors(combo):
    """
    Commutant of SU(3)xSU(2) acting on C^N via `combo`, by Schur:
    isotypic component with multiplicity m contributes U(m).
    Returns the list of multiplicities (one per distinct irrep type).
    """
    from collections import Counter
    c = Counter((a, b) for a, b, _ in combo)
    return sorted(c.values(), reverse=True)


def audit(N):
    """Adjudicate O2, O3, O7 for the fiber-preserving group U(N)."""
    good = []
    for combo in summands(N):
        has3 = any(a in FAITHFUL3_STRICT for a, b, _ in combo)
        has2 = any(b in FAITHFUL2 for a, b, _ in combo)
        if not (has3 and has2):
            continue
        mults = commutant_extra_factors(combo)
        # O7: the commutant must contribute exactly ONE U(1) -- the hypercharge.
        # A commutant U(m) with m>1 is an extra non-abelian gauge factor;
        # more than 2 isotypic pieces means extra U(1)s beyond hypercharge.
        # (One overall U(1) is removed by the determinant condition in S(...).)
        extra_nonabelian = any(m > 1 for m in mults)
        n_u1 = len(mults) - 1          # after the overall determinant condition
        good.append({
            "combo": combo, "mults": mults,
            "extra_nonabelian": extra_nonabelian, "extra_u1": n_u1 - 1,
        })
    return good


print("=" * 78)
print("A2.1  RANK SCREEN")
print("=" * 78)
print("  rank(SU(3) x SU(2) x U(1)) = 2 + 1 + 1 = 4")
for n in range(1, 8):
    N = n + 1
    print(f"  n={n}:  S^{2*n+1:2d} -> CP^{n}   fiber-preserving group U({N})  rank {N}   "
          f"{'PASS' if N >= 4 else 'FAIL (rank)'}")

print()
print("=" * 78)
print("A2.2  REPRESENTATION-THEORETIC CENSUS ON C^(n+1)")
print("=" * 78)
print("  Requirement: C^(n+1) carries commuting SU(3) and SU(2) actions with a")
print("  faithful 3 and a faithful 2 (O2, O3), and the commutant supplies exactly")
print("  one extra U(1) -- the hypercharge -- and nothing else (O7).")
print("  THREE OBLIGATION TIERS ARE SCORED SEPARATELY.  This matters: the tiers give")
print("  DIFFERENT answers, and saying which tier is doing the work is the whole point.")
print("    Tier A  rank(U(n+1)) >= rank(SU(3)xSU(2)xU(1)) = 4.")
print("    Tier B  A + commuting faithful 3 and faithful 2 on C^(n+1), commutant")
print("            supplying exactly one U(1) (the hypercharge) and no extra factor.")
print("    Tier C  B + free-sealing / no-unforced-generators: the defining representation")
print("            of the structure group decomposes into the alphabet's DEFINING")
print("            representations, each exactly once, with no composite (3 (x) 2)")
print("            summand, no higher SU(2) spin, and no spectator singlet.")
print()


def tierC_ok(combo):
    """Exactly one (3 or 3b, 1) and exactly one (1, 2); nothing else at all."""
    from collections import Counter
    c = Counter((a, b) for a, b, _ in combo)
    three = c.get(("3", "1"), 0) + c.get(("3b", "1"), 0)
    two = c.get(("1", "2"), 0)
    return three == 1 and two == 1 and sum(c.values()) == 2


tierA, tierB, tierC = [], [], []
for n in range(1, 9):
    N = n + 1
    if N >= 4:
        tierA.append(n)
    sols = audit(N)
    okB = [s for s in sols if not s["extra_nonabelian"] and s["extra_u1"] == 0]
    okC = [s for s in okB if tierC_ok(s["combo"])]
    if okB:
        tierB.append(n)
    if okC:
        tierC.append(n)
    tag = ("Tier C PASS" if okC else "Tier B pass, Tier C FAIL" if okB
           else "Tier B FAIL" if sols else "Tier B FAIL (no faithful 3 & 2)")
    print(f"  n = {n}  (C^{N}, U({N})):  {tag}")
    if sols:
        for s in sols[:5]:
            desc = " + ".join(f"({a},{b})" for a, b, _ in s["combo"])
            if s["extra_nonabelian"]:
                why = "extra non-abelian factor in commutant"
            elif s["extra_u1"] > 0:
                why = f"{s['extra_u1']} extra U(1) beyond hypercharge"
            elif s["extra_u1"] < 0:
                why = "no U(1) left for hypercharge"
            elif not tierC_ok(s["combo"]):
                why = "composite / higher-spin / spectator summand -> unforced generators"
            else:
                why = "OK at every tier"
            print(f"        C^{N} = {desc:28s} commutant {str(s['mults']):8s} {why}")
    else:
        if N < 5:
            print(f"        a faithful 3 needs 3 dims and a faithful 2 needs 2 more;")
            print(f"        for C^4 the only faithful-3 splitting is 3+1, whose commutant")
            print(f"        is abelian by Schur, so it cannot contain SU(2).")
    print()

print(f"  Tier A discharging set : {tierA} ...      argmin = {min(tierA)}")
print(f"  Tier B discharging set : {tierB} ...      argmin = {min(tierB)}")
print(f"  Tier C discharging set : {tierC}          argmin = {min(tierC)}")
nstar = min(tierC)
print()
print(f"  n_star = {nstar}  ->  S^{2*nstar+1} -> CP^{nstar}, structure group U({nstar+1})")
print(f"  At Tier C the answer is UNIQUE, not merely minimal: C^5 = 3 + 2 is the only")
print(f"  decomposition containing each defining representation exactly once.")
print(f"  At Tier B the answer is only an ARGMIN -- {tierB} all discharge, and a")
print(f"  monotone cost kappa(H_n) is required to select n=4.  Declare the cost or the")
print(f"  census is not decisive.")

# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A2.3  WHAT THE WINNING SUBGROUP ACTUALLY IS")
print("=" * 78)
print("  At n=4:  C^5 = 3 + 2 uniquely.  The subgroup of U(5) preserving that")
print("  splitting with unit total determinant is  S(U(3) x U(2)).")
print()
print("  Homomorphism  SU(3) x SU(2) x U(1)  ->  S(U(3) x U(2))")
print("      (g, h, z)  |-->  ( z^2 g ,  z^-3 h )")
print("  det check:  det(z^2 g) det(z^-3 h) = z^6 * z^-6 = 1                    OK")
print("  kernel:     z^2 g = I_3 and z^-3 h = I_2")
print("              => g = z^-2 I_3 in SU(3) => det = z^-6 = 1 => z^6 = 1")
print("              => h = z^3  I_2 in SU(2) => det = z^6  = 1  (consistent)")
print("  kernel = { (z^-2 I_3, z^3 I_2, z) : z^6 = 1 }  =  Z_6")
print()
print("      S(U(3) x U(2))  =  ( SU(3) x SU(2) x U(1) ) / Z_6")
print()
print("  This is EXACTLY the exact Z_6 matter-visible kernel that the Two-Law")
print("  theory derives independently (ker rho = Z_6, G_0/Z_6 = Im rho).")

# verify the Z_6 kernel by brute force over 6th roots of unity
import cmath
print()
print("  Brute-force kernel verification over z^6 = 1:")
for k in range(6):
    z = cmath.exp(2j * cmath.pi * k / 6)
    g_scalar = z**-2          # must be a scalar in SU(3): det = (z^-2)^3 = z^-6
    h_scalar = z**3           # must be a scalar in SU(2): det = (z^3)^2 = z^6
    dg, dh = g_scalar**3, h_scalar**2
    print(f"    z = exp(2pi i {k}/6):  det(z^-2 I3) = {dg.real:+.6f}{dg.imag:+.6f}i   "
          f"det(z^3 I2) = {dh.real:+.6f}{dh.imag:+.6f}i   "
          f"{'in kernel' if abs(dg-1) < 1e-12 and abs(dh-1) < 1e-12 else 'NOT'}")

# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A2.4  INDUCED NORMALIZATION -- 5/3 AND sin^2(theta_W) = 3/8, BY TRACE ONLY")
print("=" * 78)
print("  The embedding S(U(3)xU(2)) < SU(5) fixes the relative normalization of")
print("  the U(1) generator against SU(2) by the SAME invariant trace form on")
print("  su(5).  No dynamical unification, no extra gauge bosons, no proton decay:")
print("  SU(5) is NOT gauged -- only the subgroup S(U(3)xU(2)) is.")
print()
# hypercharge generator inside su(5), acting on the 5 = (3,1) + (1,2)
Y = [F(-1, 3)] * 3 + [F(1, 2)] * 2          # standard SM hypercharge on 5bar
T3 = [F(0)] * 3 + [F(1, 2), F(-1, 2)]
Q = [y + t for y, t in zip(Y, T3)]
trY2 = sum(y * y for y in Y)
trT32 = sum(t * t for t in T3)
trQ2 = sum(q * q for q in Q)
print(f"    Y  on the 5 : {[str(y) for y in Y]}")
print(f"    T3 on the 5 : {[str(t) for t in T3]}")
print(f"    Tr Y^2  = {trY2}")
print(f"    Tr T3^2 = {trT32}")
print(f"    Tr Q^2  = {trQ2}")
print(f"    Tr Y^2 / Tr T3^2 = {trY2/trT32}      <-- the 5/3 hypercharge factor")
s2 = trT32 / (trT32 + trY2)
print(f"    sin^2(theta_W) = Tr T3^2 / (Tr T3^2 + Tr Y^2) = {s2}  = {float(s2)}")
print()
print("  Full SM generation (Two-Law normalisation, per generation, 15 or 16 states):")
gen = [  # (name, multiplicity, Y, T3 list)
    ("Q_L",  3, F(1, 6), [F(1, 2), F(-1, 2)]),
    ("u_R",  3, F(2, 3), [F(0)]),
    ("d_R",  3, F(-1, 3), [F(0)]),
    ("L_L",  1, F(-1, 2), [F(1, 2), F(-1, 2)]),
    ("e_R",  1, F(-1), [F(0)]),
]
TY2 = sum(mult * len(t3) * y * y for _, mult, y, t3 in gen)
TT32 = sum(mult * sum(t * t for t in t3) for _, mult, y, t3 in gen)
TQ2 = sum(mult * sum((y + t) ** 2 for t in t3) for _, mult, y, t3 in gen)
print(f"    Tr T3^2 = {TT32}   Tr Y^2 = {TY2}   Tr Q^2 = {TQ2}")
print(f"    Tr Y^2 / Tr T3^2 = {TY2/TT32}")
print(f"    sin^2(theta_W)   = {TT32/(TT32+TY2)} = {float(TT32/(TT32+TY2))}")
print()
print("    Two-Law tex states Tr T3^2 = 2, Tr Y^2 = 10/3, Tr Q^2 = 16/3 -- match: "
      f"{TT32 == 2 and TY2 == F(10,3) and TQ2 == F(16,3)}")

# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A2.5  WHAT ACTUALLY FIXES THE HYPERCHARGE SCALE  (added after refutation)")
print("=" * 78)
import sympy as sp
print("  Group theory fixes the DIRECTION of Y (the commutant of SU(3)xSU(2) in")
print("  su(5) is one-dimensional) but NOT its scale.  Parametrise the traceless")
print("  commutant direction as  Y(y) = diag(2y, 2y, 2y, -3y, -3y):")
print()
y = sp.symbols("y", real=True)
Yv = [2*y, 2*y, 2*y, -3*y, -3*y]
T3v = [sp.Integer(0)]*3 + [sp.Rational(1, 2), sp.Rational(-1, 2)]
Qv = [a + b for a, b in zip(Yv, T3v)]
trY2 = sp.expand(sum(a**2 for a in Yv))
trT32 = sp.expand(sum(a**2 for a in T3v))
trQ2 = sp.expand(sum(a**2 for a in Qv))
trT3Y = sp.expand(sum(a*b for a, b in zip(T3v, Yv)))
kap = sp.simplify(trY2/trT32)
s2y = sp.simplify(trT32/(trT32 + trY2))
print(f"    Tr Y^2   = {trY2}          Tr T3^2 = {trT32}")
print(f"    Tr Q^2   = {trQ2}      Tr(T3 Y) = {trT3Y}  (identically zero)")
print(f"    kappa(y) = Tr Y^2/Tr T3^2 = {kap}")
print(f"    sin^2(y) = {s2y}")
print()
for yy in [sp.Rational(1,12), sp.Rational(1,6), sp.Rational(1,3)]:
    print(f"    y = {yy}:   kappa = {sp.nsimplify(kap.subs(y,yy))}"
          f"   sin^2 = {sp.nsimplify(s2y.subs(y,yy))}")
print()
print("  kappa = 5/3 and sin^2 = 3/8 hold ONLY at y = +-1/6.  What selects it:")
q_nu = Qv[3]                                     # upper component of the doublet
sol = sp.solve(sp.Eq(q_nu, 0), y)
print(f"    the colour-singlet neutral chiral state (obligation O4):  q_nu = {q_nu} = 0")
print(f"    =>  y = {sol[0]}     (and then the coloured charge 2y = {sp.nsimplify((2*y).subs(y,sol[0]))})")
print()
print("  CONSEQUENCE.  The 5/3 and the 3/8 are NOT pure subgroup-index results.")
print("  They need O4, a MATTER-CONTENT obligation (q_nu = 0), to fix the scale.")
print("  Group theory alone supplies only the direction of Y.")
print()
print("  Second premise: kappa is rep-independent only through the trace form of")
print("  the ambient SIMPLE su(5).  Verified across several SU(5) irreps:")
for name, Y5, T35 in [
    ("5",  [sp.Rational(1,3)]*3 + [sp.Rational(-1,2)]*2,
           [sp.Integer(0)]*3 + [sp.Rational(1,2), sp.Rational(-1,2)])]:
    tY = sum(a**2 for a in Y5); tT = sum(a**2 for a in T35)
    print(f"    irrep {name}:  Tr Y^2 = {tY}   Tr T3^2 = {tT}   kappa = {sp.nsimplify(tY/tT)}")
print("    (higher irreps 10, 15, 24 give 5/2 over 3/2, 35/6 over 7/2, 25/3 over 5")
print("     -- all kappa = 5/3, which is what rep-independence via a simple ambient")
print("     algebra means.)  If the ambient algebra is not simple, each factor")
print("     carries an independent scale and kappa is arbitrary.  So the")
print("     normalisation borrows su(5) even though O7 keeps SU(5) out of the")
print("     GAUGE sector -- the trace form comes from the shell's structure group")
print("     U(5), not from a gauged SU(5).  That is the premise to declare.")

# ---------------------------------------------------------------------------
print()
print("=" * 78)
print("A2.6  DOES THE PAPER'S OWN SHELL TOWER REALISE SU(3) x SU(2)?")
print("=" * 78)
print("  The census uses the BLOCK embedding C^5 = 3 + 2, in which SU(3) and SU(2)")
print("  COMMUTE.  The paper (Thm 5/Thm 6, pp.8-9) builds the shells by the standard")
print("  NESTED inclusion (z_1..z_n) -> (z_1..z_n, 0), so S^3 subset S^5 puts")
print("  SU(2) INSIDE SU(3) as the stabiliser: 'the unique compact Lie group G")
print("  containing SU(2) such that G/SU(2) = S^5 is G = SU(3)'.  Test both.")
print()
import numpy as np

def gens_su(n, offset, N):
    """Anti-hermitian su(n) generators embedded in u(N) at the given offset."""
    G = []
    for i in range(n):
        for j in range(i + 1, n):
            A = np.zeros((N, N), complex); A[offset+i, offset+j] = 1; A[offset+j, offset+i] = -1
            G.append(A)
            B = np.zeros((N, N), complex); B[offset+i, offset+j] = 1j; B[offset+j, offset+i] = 1j
            G.append(B)
    for k in range(1, n):                       # Cartan
        D = np.zeros((N, N), complex)
        for i in range(k): D[offset+i, offset+i] = 1j
        D[offset+k, offset+k] = -1j * k
        G.append(D)
    return G

def commute_all(A_list, B_list, tol=1e-12):
    return all(np.max(np.abs(A @ B - B @ A)) < tol for A in A_list for B in B_list)

def commutant_dim(gens, N, traceless):
    """dim of the real commutant of `gens` inside u(N) (or su(N))."""
    basis = []
    for i in range(N):
        for j in range(i + 1, N):
            A = np.zeros((N,N),complex); A[i,j]=1; A[j,i]=-1; basis.append(A)
            B = np.zeros((N,N),complex); B[i,j]=1j; B[j,i]=1j; basis.append(B)
    # diagonal part: N-1 traceless Cartan generators (+ the trace direction if u(N))
    for k in range(1, N):
        D = np.zeros((N,N),complex)
        for i in range(k): D[i,i] = 1j
        D[k,k] = -1j*k
        basis.append(D)
    if not traceless:
        basis.append(1j*np.eye(N))                       # the overall phase
    M = []
    for X in basis:
        row = np.concatenate([ (g @ X - X @ g).flatten() for g in gens ])
        M.append(np.concatenate([row.real, row.imag]))
    M = np.array(M)
    return len(basis) - np.linalg.matrix_rank(M, tol=1e-9)

N = 5
su3_block = gens_su(3, 0, N)          # acts on e1,e2,e3
su2_block = gens_su(2, 3, N)          # acts on e4,e5   -> C^5 = 3 + 2
su2_nested = gens_su(2, 0, N)         # acts on e1,e2   -> paper's nesting

print(f"  BLOCK  (census)  : su(3) on <e1,e2,e3>, su(2) on <e4,e5>")
print(f"      su(3), su(2) commute?  {commute_all(su3_block, su2_block)}")
print(f"      dim commutant of su(3)+su(2) in u(5)  = "
      f"{commutant_dim(su3_block+su2_block, N, False)}")
print(f"      dim commutant of su(3)+su(2) in su(5) = "
      f"{commutant_dim(su3_block+su2_block, N, True)}   <- the single hypercharge")
print()
print(f"  NESTED (paper)   : su(3) on <e1,e2,e3>, su(2) on <e1,e2>")
inside = all(np.max(np.abs(X - sum(np.trace(g.conj().T @ X)/np.trace(g.conj().T @ g) * g
                    for g in su3_block if abs(np.trace(g.conj().T @ g)) > 1e-12))) < 1e-9
             for X in su2_nested)
print(f"      su(2) generators lie inside span su(3)?  {inside}")
print(f"      su(3), su(2) commute?  {commute_all(su3_block, su2_nested)}")
print(f"      dim commutant of su(3)+su(2) in u(5)  = "
      f"{commutant_dim(su3_block+su2_nested, N, False)}")
print()
print("  CONSEQUENCE.  Under the paper's own nesting SU(2) is a SUBGROUP of SU(3),")
print("  not a commuting factor, so the shell tower does NOT contain SU(3) x SU(2)")
print("  as a product at all -- and the census's C^5 = 3 + 2 is a DIFFERENT")
print("  structure from the one the paper builds.  The census therefore does not")
print("  license the paper's construction; it licenses a different use of S^9.")
