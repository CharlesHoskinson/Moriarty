VOTE fable-2 ACCEPT on DRAFT v1.1 (recursion lens; read design.md 3.x, specs/sherlock-recursion, requirements SRC rows, tasks M2-M5 at 21:19Z).
Accepted in v1.1: SRC-004 scenario now requires refusal at the node boundary from a valid baseline; design 3.2(2) genesis text no longer infers cancellation impossibility; 4.8 (S=8) moved after 5.3 and conditional.
Remaining synchronization for v2 (editorial, none blocks; several duplicate astra-1's delta list):
1. design.md 3.2(2) now labels (a) = neutral-carry constraint and (b) = scale-by-not_genesis, but requirements.md SRC-005 and specs/sherlock-recursion SRC-005 still label (a) = scale and (b) = keep-the-fold-with-argument. Pick one labelling, use it in design, requirements, spec, task 2.3 and task 5.2 ("malformed genesis carry" -> "per SRC-005 realization: non-neutral refusal, or irrelevance of a well-formed ignored carry; invalid leaf refused through the pairing").
2. design.md heading IDs: 3.2 covers SRC-004 and SRC-005; 3.4 is SRC-013 (not SRC-004); 3.5 is SRC-011 (not SRC-010).
3. tasks.md 4.4: "frozen admitted inventory R (SVM-008)" instead of "P1-P3 rule set"; 4.7 parity for P1/P2 in M4, P3 after 6.4 authorship and before OP-PV-CALL-P3.
4. tasks.md 2.4: "C6 digest leaf relation (M2 shape, sherlock_c6_child.rs)" instead of "leaf stand-in".
5. SRC-014: add "applies when S>1 is adopted; vacuous and satisfied at S=1" to the requirement row and spec.
Unresolved concern carried into tasks (not an objection): Q7 is partially answered only at 24a1eb5a; task 2.3 must re-read the accumulator fold at the re-pinned midnight-circuits 7.3.0 and record the chosen realization before node keygen. My preference stays the scale-by-not_genesis construction because its test needs no cryptographic argument.
