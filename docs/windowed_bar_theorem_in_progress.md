# Windowed-bar theorem: completed resolution

**Date:** 2026-07-27  
**Status:** completed negative/conditional result  
**Scope:** the frozen Genesis lane and the adopted `nu-register-split-v1`

## Verdict

The proposed windowed bar has a rigorous but limited theorem behind it:
once an additive ledger is required to assign a seal-invariant efficiency
to the active two-interface aggregate, its scalar benchmark is uniquely

\[
\widehat{\operatorname{Bar}}_n
=
\frac{\nu_{n-1}+\nu_{n-2}}
     {\kappa_{n-1}+\kappa_{n-2}}.
\]

On the frozen **structural** ledger, every enacted entry from Step 3 through
Step 15 clears this benchmark. This proves WB-1a as a trace-relative
diagnostic statement.

The stronger claim does not survive the certified candidate surface:

\[
\boxed{\text{WB-1b is false.}}
\]

At Step 4, the complete raw strict-discharge cone contains four distinct
candidates. In the structural register all four clear the windowed bar with
the same overshoot. In the lawful semantic-family register all four clear,
and minimum overshoot leaves two inequivalent candidates tied. Thus the bar
does not uniquely reproduce the enacted choice in either register.

The Step-16 argument also cannot be made law-level. The value
\(165/17\) is structural testimony, whereas the EGP envelope
\(\rho\le 4\) has semantic-family authority. In the semantic register the
Step-16 windowed bar is instead \(9/17\), and the envelope does not imply
foreclosure. The adopted register split expressly forbids using the
structural value as law-level authority.

Accordingly the mathematical WB-1 disposition is:

\[
\boxed{\text{Z2: structural shadow only.}}
\]

In the original engine-task terminology, a final Z2 certificate would still
require the separately preregistered W-T1 create-new artifact and mutation
falsifiers. This document proves its arithmetic payload and supplies the
decisive W-T2 counterexample; it does not pretend that a missing certificate
has been emitted.

The actual halt remains the independently certified demand result
\(\mathrm O(16)=\varnothing\). It is not a consequence of windowed-bar
unattainability.

---

## 1. Fixed evidence and register discipline

The frozen structural ledger is

\[
\nu^{\mathrm{str}}
=(1,1,2,5,7,8,10,17,17,19,26,34,46,62,103)
\]

and

\[
\kappa
=(2,1,1,3,3,3,3,5,4,4,5,6,7,9,8).
\]

The completed source-first semantic-family audit gives

\[
\nu^{\mathrm{sem}}
=(1,0,1,3,6,3,1,1,0,4,1,0,2,3,6).
\]

These are different quantities, not two estimates of one quantity.
The adopted rule `nu-register-split-v1` assigns them the following
jurisdictions:

- structural \(\nu\) is sealed engine testimony and a lawful diagnostic;
- semantic-family \(\nu\) is the sole lawful value register for
  selection, parsimony, envelopes, and certification;
- scalar equality between the two registers is neither required nor
  permitted to be manufactured by reclassification.

The earlier draft used \(18\) at structural Step 8. The authoritative
mechanism-corrected value is \(17\), as recorded in
`PHASE5B_RESELECTION_V3_RESULT.md` and `nu_register_adjudication.md`.

The candidate-level facts used below are:

1. the current Step-4 enumerator produces a complete canonical cone of four
   distinct typed strict total dischargers;
2. all four have structural pair
   \((\kappa,\nu^{\mathrm{str}})=(3,5)\);
3. their semantic-family values are \(3,2,2,3\), with common
   \(\kappa=3\);
4. the two semantic minima are certified inequivalent on the adopted
   comparison surface.

These facts are recorded in the sealed issuance artifacts
`T_BF1_PREFIX_RESULT.md`, `STAGE4_SEMANTIC_PARSIMONY_V2_RESULT.md`, and
`STAGE4_SEMANTIC_PARSIMONY_V3_RESULT.md`. Their current-source replay
boundary is disclosed next.

### Verification boundary on 2026-07-27

The focused current-source test

```text
cargo test -q -p pen-search \
  t_bf1_honors_stage4_f_bf1_instead_of_importing_a_hidden_tie_break
```

passes and independently recomputes the decisive structural result: Step 4
has four equal minimizers. Replaying the older full T-BF1 certificate also
returns `stopped_stage = 4` and `minimizer_count_at_stop = 4`, but reports
`valid = false` because the complete reissued certificate has drifted.

The semantic v3 replay currently stops earlier on inherited chronological-v5
/ T-SM1a source drift, before it can reissue the Step-4 semantic certificate.
Therefore the semantic \(3,2,2,3\) row is used here as sealed-artifact
testimony, not as a claim of current full replay. This does not affect the
refutation: the passing current-source structural test alone is already a
counterexample to unique windowed-bar selection.

---

## 2. Conditional uniqueness of the aggregate benchmark

Let the active width-\(d\) window before Step \(n\) be

\[
W_n^{(d)}=(S_{n-1},\ldots,S_{n-d}),
\]

and let each sealed interface carry an additive ledger pair

\[
V(S_i)=(\nu_i,\kappa_i),\qquad \kappa_i>0.
\]

### Aggregate-benchmark lemma

Suppose a benchmark \(B_d\) obeys:

1. **window locality:** it consults only \(W_n^{(d)}\);
2. **additive sealing:** sealing the window into one aggregate replaces its
   ledger by
   \[
   \left(\sum_{j=1}^d\nu_{n-j},
         \sum_{j=1}^d\kappa_{n-j}\right);
   \]
3. **sealing invariance:** the benchmark is unchanged by that aggregation;
4. **singleton calibration:** \(B_1(\nu,\kappa)=\nu/\kappa\).

Then

\[
B_d(W_n^{(d)})
=
\frac{\sum_{j=1}^{d}\nu_{n-j}}
     {\sum_{j=1}^{d}\kappa_{n-j}}.
\]

#### Proof

By additive sealing and sealing invariance,

\[
B_d\bigl(V(S_{n-1}),\ldots,V(S_{n-d})\bigr)
=
B_1\left(
\sum_{j=1}^{d}\nu_{n-j},
\sum_{j=1}^{d}\kappa_{n-j}
\right).
\]

Singleton calibration evaluates the right-hand side as the ratio of its two
coordinates. This proves the formula. \(\square\)

For \(d=2\), this is exactly

\[
\widehat{\operatorname{Bar}}_n
=
\frac{\nu_{n-1}+\nu_{n-2}}
     {\kappa_{n-1}+\kappa_{n-2}}.
\]

### Scope of the lemma

This is a representation theorem, not a consequence of width two alone.
Additive valuation, sealing invariance, and singleton calibration do the
scalar work. Without them, width two permits many other functions of the two
interfaces. In particular, width two by itself supplies no inequality

\[
\rho(x)\ge\widehat{\operatorname{Bar}}_n
\]

and no selection rule.

---

## 3. W-T1: exact structural-shadow theorem

Using the corrected structural vector, exact rational arithmetic gives:

| Step \(n\) | \(\widehat{\operatorname{Bar}}^{\mathrm{str}}_n\) | enacted \(\rho^{\mathrm{str}}_n\) | margin |
|---:|---:|---:|---:|
| 3 | \(2/3\) | \(2\) | \(4/3\) |
| 4 | \(3/2\) | \(5/3\) | \(1/6\) |
| 5 | \(7/4\) | \(7/3\) | \(7/12\) |
| 6 | \(2\) | \(8/3\) | \(2/3\) |
| 7 | \(5/2\) | \(10/3\) | \(5/6\) |
| 8 | \(3\) | \(17/5\) | \(2/5\) |
| 9 | \(27/8\) | \(17/4\) | \(7/8\) |
| 10 | \(34/9\) | \(19/4\) | \(35/36\) |
| 11 | \(9/2\) | \(26/5\) | \(7/10\) |
| 12 | \(5\) | \(17/3\) | \(2/3\) |
| 13 | \(60/11\) | \(46/7\) | \(86/77\) |
| 14 | \(80/13\) | \(62/9\) | \(86/117\) |
| 15 | \(27/4\) | \(103/8\) | \(49/8\) |

Every margin is positive. Therefore:

> **Structural-shadow theorem.** Relative to the frozen structural ledger,
> every enacted entry for which the registered width-two bar is defined
> clears that bar.

This is exactly the surviving content of WB-1a. It is descriptive and
trace-relative. It says nothing about whether another candidate also clears,
whether the enacted candidate minimizes overshoot, or whether the structural
register may select.

At the next step,

\[
\widehat{\operatorname{Bar}}^{\mathrm{str}}_{16}
=
\frac{103+62}{8+9}
=
\frac{165}{17}.
\]

That identity is also a structural diagnostic only.

---

## 4. W-T2: Step-4 counterexample to unique selection

The registered selectivity probe says:

1. begin with the raw canonically quotiented candidate surface;
2. do not use the demand law to exclude candidates;
3. retain candidates that clear the windowed bar;
4. select minimum overshoot;
5. report a tie as non-unique.

Step 4 is a complete decisive instance.

### Structural register

The preceding structural ledger pairs are

\[
(\nu^{\mathrm{str}}_2,\kappa_2)=(1,1),
\qquad
(\nu^{\mathrm{str}}_3,\kappa_3)=(2,1),
\]

so

\[
\widehat{\operatorname{Bar}}^{\mathrm{str}}_4
=
\frac{1+2}{1+1}
=
\frac32.
\]

Every member of the four-candidate cone has

\[
\rho^{\mathrm{str}}=\frac53,
\qquad
\rho^{\mathrm{str}}
-\widehat{\operatorname{Bar}}^{\mathrm{str}}_4
=\frac16.
\]

Thus all four clear and all four have the same minimum overshoot. The
windowed bar returns four minimizers, not the enacted candidate uniquely.

### Semantic-family register

The common semantic prefix is \((1,0,1)\), hence

\[
\widehat{\operatorname{Bar}}^{\mathrm{sem}}_4
=
\frac{\nu^{\mathrm{sem}}_2+\nu^{\mathrm{sem}}_3}
       {\kappa_2+\kappa_3}
=
\frac{0+1}{1+1}
=
\frac12.
\]

The four candidate efficiencies are

\[
1,\quad \frac23,\quad \frac23,\quad 1.
\]

All four clear. Minimum overshoot is \(1/6\), attained by the two
semantic-\(\nu=2\) candidates. Those two candidates are certified
inequivalent on the adopted surface. The enacted candidate has efficiency
\(1\), overshoot \(1/2\), and is not a minimum-overshoot candidate.

Therefore W-T2 returns **non-unique** in the lawful register as well.
This is a direct counterexample to WB-1b and to the draft's proposed
unique-clearer induction.

---

## 5. The semantic window does not reproduce the trace

For completeness, applying the same aggregate formula to the lawful
semantic-family register gives:

| Step \(n\) | \(\widehat{\operatorname{Bar}}^{\mathrm{sem}}_n\) | enacted \(\rho^{\mathrm{sem}}_n\) | clears? |
|---:|---:|---:|:---:|
| 3 | \(1/3\) | \(1\) | yes |
| 4 | \(1/2\) | \(1\) | yes |
| 5 | \(1\) | \(2\) | yes |
| 6 | \(3/2\) | \(1\) | no |
| 7 | \(3/2\) | \(1/3\) | no |
| 8 | \(2/3\) | \(1/5\) | no |
| 9 | \(1/4\) | \(0\) | no |
| 10 | \(1/9\) | \(1\) | yes |
| 11 | \(1/2\) | \(1/5\) | no |
| 12 | \(5/9\) | \(0\) | no |
| 13 | \(1/11\) | \(2/7\) | yes |
| 14 | \(2/13\) | \(1/3\) | yes |
| 15 | \(5/16\) | \(3/4\) | yes |

Steps 6, 7, 8, 9, 11, and 12 fail. These are lawful guarded-stage
acceptances because total typed discharge, not scalar value, governs a
nonempty obligation window. Consequently Window Self-Benchmarking is not a
hidden theorem of the current laws; adopting it would change the accepted
history.

At Step 16 the lawful semantic benchmark is

\[
\widehat{\operatorname{Bar}}^{\mathrm{sem}}_{16}
=
\frac{3+6}{9+8}
=
\frac9{17}.
\]

The semantic EGP envelope

\[
\rho^{\mathrm{sem}}(x)\le4
\]

does not entail
\(\rho^{\mathrm{sem}}(x)<9/17\). It therefore supplies no windowed-bar
foreclosure.

---

## 6. No same-register repair under the adopted rule

The earlier conditional draft proposed a value functional \(\nu^W\) that
would simultaneously:

1. agree with the structural history, including \(62\) and \(103\);
2. carry law-level EGP authority;
3. act as a selector.

That is not a missing lemma inside the present theory. It conflicts with the
adopted register rule.

### No-go theorem

Under `nu-register-split-v1`, no value functional can be both:

- extensionally equal to \(\nu^{\mathrm{str}}\) on the Genesis history; and
- the existing lawful semantic-family value functional.

#### Proof

The certified registers already disagree at Step 2:

\[
\nu^{\mathrm{str}}_2=1
\qquad\text{but}\qquad
\nu^{\mathrm{sem}}_2=0.
\]

They also disagree at every later stage. A single functional cannot take both
values on the same candidate and prefix. Moreover, the adopted rule gives
structural \(\nu\) zero law-level authority and forbids changing the semantic
extractor in order to reconcile archived magnitudes. Hence the proposed
identification is inconsistent with the current rule set. \(\square\)

A third value register could only be introduced by an independently grounded,
versioned adjudication. It would define a new theory; it would not complete
this theorem from current premises.

The numerical identity

\[
\frac{165}{17}-4=\frac{97}{17}
\qquad\text{and}\qquad
103-6=97
\]

mixes structural and semantic quantities. It is exact arithmetic but has no
law-level interpretation under the register discipline.

---

## 7. Width two does not imply bar authority or halt equivalence

### Scalar-authority no-go

Exact historical depth two does not imply that a total discharger clears the
aggregate benchmark.

#### Proof

Take a depth-two obligation system whose unique total discharger \(x\) has
\(\nu(x)=0\) and \(\kappa(x)>0\). Typed discharge is unchanged by this value
assignment. If the previous two interfaces have positive aggregate novelty,
then

\[
\rho(x)=0
<
\widehat{\operatorname{Bar}}_n.
\]

All depth-two jurisdictional statements still hold. The certified semantic
Steps 9 and 12 instantiate this pattern. \(\square\)

### Halt-equivalence no-go

Width two also supplies no implication

\[
\mathrm O(n+1)=\varnothing
\Longrightarrow
\sup_{x\text{ on the debt-free surface}}\rho(x)
<
\widehat{\operatorname{Bar}}_{n+1}.
\]

The left side is type-valued demand data; the right side is scalar valuation
data. Without an additional axiom connecting them, one may hold while the
other fails. For example, leave the depth-two demand profile empty and add a
debt-free candidate whose efficiency exceeds the previous aggregate
efficiency. This changes no obligation fact but falsifies trailing-density
unattainability.

Therefore WB-1d is not a theorem of width two. At Step 16,
\(\mathrm O(16)=\varnothing\) and the structural inequality
\(4<165/17\) are separately recorded facts in different jurisdictions.
Their co-occurrence does not prove an equivalence.

---

## 8. Final disposition of WB-1

| Clause | Disposition | Exact reason |
|---|---|---|
| WB-1a, structural shadow | **proved on the frozen trace** | every exact structural margin at Steps 3–15 is positive |
| WB-1b, independent selector | **refuted** | Step 4 has four structural minimizers and two semantic minimizers |
| WB-1c, common root | **partly conditional only** | the aggregate-ratio form follows from sealing axioms, but scalar authority and the Fibonacci/common-root claims do not follow from width two |
| WB-1d, halt equivalence | **not derivable; false in general** | demand emptiness and scalar unattainability are independent without a bridge axiom |
| Step-16 halt | **stands independently** | \(\mathrm O(16)=\varnothing\), not the windowed bar |

The completed theorem is therefore:

> **Windowed structural-shadow theorem with selectivity no-go.**  
> On the frozen Genesis structural ledger, the additive seal-invariant
> efficiency of the exact two-interface window is
> \[
> \widehat{\operatorname{Bar}}_n
> =
> \frac{\nu_{n-1}+\nu_{n-2}}
>      {\kappa_{n-1}+\kappa_{n-2}},
> \]
> and every enacted Step 3–15 clears it. Nevertheless this quantity is not an
> independent selector: the complete Step-4 cone is non-unique under both
> structural and semantic readings. Under the adopted register split,
> \(165/17\) cannot be compared to the semantic EGP ceiling to derive the
> halt. Thus the windowed bar is a structural diagnostic shadow of the trace,
> while the fifteen-step halt remains a demand theorem.

This result completes the W-T1 theorem arithmetically and resolves W-T2
negatively. The separately specified W-T1 engine certificate remains an
artifact task. No new bar, weighting, register, or selection law is
introduced.
