# `pen-sealed-history`

This isolated, kernel-only crate freezes the JG2b1b0 target-neutral sealed-log
protocol grammar. The grammar was frozen before a producer existed;
`pen-generative-audit` now separately implements the discharged JG2b1b1
process-local producer, discharged JG2b1b2 complete-through-head replay, and
protocol-only JG2b2a substitution/naturality entry gate. JG2b2b executable
structural occurrence/indexed-interface calculi, exact theorem subjects, and
generic substitution metatheory are now active.

Its sole opaque authority,
`VerifiedTargetNeutralSealedLogProtocolV1`, certifies:

- the exact schema and resource-policy versions;
- one closed genesis rule;
- one closed event-delimiter rule;
- one closed ordering rule;
- one closed sole-ingress rule;
- one closed branch/writer-epoch identity rule;
- one closed no-retrospective-promotion rule;
- one closed resource/failure rule;
- one closed finalization rule;
- one closed finalized-head origin rule;
- one closed terminal-verification rule; and
- the exact kernel protocol, normalizer protocol, and kernel resource
  configuration under which those rules are interpreted.

This crate itself deliberately contains no open writer, append operation, event
frame, branch head, log transcript, EOF marker, finalization receipt, or
verified history. A protocol token says what a conforming producer must
implement; it does not say that any producer, log, terminal event, or complete
history exists.

The frozen authority rules require a conforming private non-cloneable writer
as the sole append ingress, a distinct producer-minted identity for every independently
opened, continued, or forked writer epoch, and a finalized head minted only
from consuming that owned writer. A caller transcript, count, digest, terminal
boundary, or EOF flag can never designate an authoritative head.

The frozen terminal semantics are deliberately branch-relative: JG2b1b2 may
certify completeness only through its authoritative finalized head after
independent replay. Consuming finalization closes that writer/epoch; a
later continuation or fork must have a distinct head and authority identity.
The protocol never claims a globally latest history. Resource exhaustion is a
verification failure and can never be interpreted as EOF or finalization.

The only production dependency is `pen-kernel`. In particular, this crate
does not depend on `pen-law`, `pen-store`, `pen-engine`, semantic audit, or any
oracle or diagnostic lane.

Resource policy V1 binds a producer to the supplied kernel configuration. A
conforming producer must bound event count by `max_depth`, retained extension
and terminal declarations by `max_operations`, and aggregate replay material by
the checked `max_operations * (2 * max_depth + 1)` formula. Retained output is
reserved fallibly. Any reported resource failure must
leave the writer unclosed and may never yield a shorter prefix, finalization,
head, or EOF token; an unrecoverable allocator abort likewise yields no token.
Those obligations are frozen here and discharged for the in-memory producer in
JG2b1b1, but this protocol crate itself mints no producer or replay authority.
JG2b1b2 now consumes the closed producer state and replays it independently
before issuing completeness-through-head authority; that authority still says
nothing about global EOF, latestness, or actual-current external state.
