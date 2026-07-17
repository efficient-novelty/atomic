# Runtime calculus from the formal axioms: executable no-go

**Date:** 2026-07-16. **Status:** resolution of Open Problem 1 at the
definition-derived level. **Code:** `crates/pen-eval/src/runtime_axioms.rs`.
The frozen July 5 runtime experiment remains untouched in
`RUNTIME_CALCULUS.md`, `runtime_calculus_run.json`, and
`runtime_calculus_results.md`.

## Result

The formal axioms determine a realization-order calculus, but they do not
determine a physical cadence, throughput, engagement fraction, or a map from
that calculus to redshift. The Step-15 temporal shell can supply internal
continuous time; the definitions of ν and κ do not calibrate structural audit
steps against it. Consequently no successor calculus derived only from those
definitions can compute the requested `w(z)` profile or a physical stalling
time. Supplying such a computation requires a new bridge principle and must be
graded as such.

This is a no-go result, not another failed numerical fit. It follows before a
target quantity is evaluated.

## 1. What the definitions do determine

Appendix A defines history as a sequence of **sealed extensions**
`H_n = (x_1, ..., x_n)`. Both audit quantities are marginal with respect to
that realized history:

- `κ_H(x)` is the size of the minimal normalized public kernel not derivable
  from `H`;
- `ν_H(x)` is the set difference `|L(I(H,x)) \ L(H)|` after normalization and
  equivalence quotienting.

It follows immediately that repetition needs no separate discount rule. If a
second application exports no public clause or schema outside the closure of
the enlarged history, then its kernel is empty (`κ = 0`) and it is a
theorem-readout, not a continuation. If it opens a genuinely new instance, it
is simply a new candidate audited against the new history. There is no third
category called “the same schema ticking again.”

The PEN transition schema also fixes what happens when no candidate clears.
The active cost horizon widens, but no history step is realized. Since Δ, Ω,
and the bar are defined from realized layers, an idle search tick cannot renew
Fibonacci debt or advance a separate ledger clock.

The Guard-Rail Theorem makes the same indexing discipline sharper. Its
directive debt is

`O(n+1) = C(S_n, S_{n-1}) \ D(B_n)`,

so Theorem 7's guarded recursion factors through the two most recently sealed
shells and the closure of realized history. Theorem 12 then proves the crucial
separation `O(16) = empty` while `Δ_16 = 987`: directive debt can vanish while
quantitative price continues. Neither quantity is defined on an independent
wall-clock tick.

Therefore, in the definition-derived semantics:

1. deduplication is the history-relative kernel and schema-set difference;
2. the ledger index is the sealing index;
3. idle search changes only the search horizon;
4. obligation export is candidate-local data in `O_•(x | H)`, available only
   once that normalized complex is represented by the engine.

The Guard-Rail note's phrase “runtime begins where debt ends” is compatible
with this result but does not evade it. In that note the phrase is conditional
on Theorem 17's extraction-guarded valuation, and it means that post-completion
motion is internally derivable rather than another library extension. It does
not furnish a physical unit for that motion.

## 2. The clock/throughput no-go

The ignition chapter already supplies the decisive type distinction. Its
algorithmic time `τ` is the discrete order of selection and sealing, while
physical time `t` is the internal continuous parameter of the evolution
operators supplied by the temporal shell. It states that no order relation
spans the two levels. The guarded-time chapter explains the orientation and
history-bearing character of internal update, but it gives no audit theorem
identifying a physical update with a new structural kernel or schema basis.
Using the Fibonacci structural ledger as a physical cadence therefore needs a
cross-level bridge that those chapters intentionally do not provide.

Take any realized trace with audited pairs `(ν_i, κ_i)`. Assign it strictly
increasing timestamps `t_i`, or instead any other strictly increasing
timestamps `f(t_i)`. The histories, marginal audits, `ρ`, `Ω`, Fibonacci layer
indices, bar comparisons, and selected order are identical. Only the external
durations differ.

Thus the theory is invariant under arbitrary monotone reparameterization of
an external time coordinate. Dimensionless schema counts cannot choose among
physical-time, scale-factor, or redshift intervals. In particular, the
previous engagement expression

`e_k = D_serv(k) / D_tot(k)`

requires three items not present in the axioms: a cadence index independent of
realized history, a rule renewing `D_tot` on that cadence, and a schedule
turning candidate applications into `D_serv`. Those were extra semantics in
the frozen July 5 experiment. Its unanticipated “two clocks” were a diagnostic
of that addition, not a theorem forced by ν and κ.

The Guard-Rail Theorem adds a fourth constraint on any future bridge: it must
keep directive debt `O` separate from quantitative debt `Δ`. Empty directive
debt opens the constitutive field; it is not zero quantitative demand, and it
does not by itself define a throughput denominator.

## 3. Executable gates

The new reference module encodes only the consequences above and passes these
tests:

- Genesis 1–15 is reproduced value for value, ending at
  `Σν = 359`, `Σκ = 64`, `Ω = 359/64`, with the exact next bar
  `354333/39040`;
- one hundred idle search ticks widen the horizon without changing the
  realized index, `Ω`, or the next bar;
- a zero-kernel repetition is rejected as a new integration;
- two radically different timestamp schedules erase to the identical audited
  realization trace.

The module accepts witnessed marginal counts rather than pretending the
current AST evaluator can produce the formal sets. The current evaluator
returns class-formula totals; it does not yet expose
`KernelClauses(x | H)`, `L(H)`, the equivalence quotient, or the normalized
obligation complex. Building those witness-producing layers remains valuable
mechanization work, but it cannot remove the clock no-go: even exact sets are
still dimensionless and realization-ordered.

## 4. Consequences for the four downstream items

- **The two Λ coefficient premises:** not derived by ν and κ. Adjacency
  priority can be represented inside a candidate's obligation complex; “full
  engagement” requires an extra physical scheduling bridge.
- **The `w(z)` amplitude and waveform:** cannot be exported from the formal
  runtime without a registered map from realization order and obligation
  records to cosmological stress-energy and redshift.
- **The C-FATE-2 stalling-state search:** a structural fixed point can be
  defined as absence of a bar-clearing prime candidate; its physical duration
  or cosmological interpretation is not fixed by the audit calculus.
- **The stratum-course replay:** the only canonical course is a sequence of
  genuine integrations. A fixed schema does not generate a cadence by
  repetition, so the formerly conjectured independent stratum clock has no
  definition-derived successor.

Open Problem 1 is therefore closed as an underdetermination theorem. A future
physical runtime model is possible, but it is a new bridge problem and must
declare its clock, demand, and observable map as additional premises before
evaluation.
