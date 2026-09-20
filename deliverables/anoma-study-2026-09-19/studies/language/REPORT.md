# Juvix and Anoma language boundary — 2026-09-19

Juvix is substantial prior art for a total-by-default functional language, Anoma resource programming, multiple execution targets and concrete compiler-transformation verification. It does **not**, in the inspected configuration, automatically provide Moriarty's required proof of a signed formal financial intention compiled to Midnight ZKIRv3. Most decisively, `--verify` emits Lean obligations containing `sorry`; that artifact needs completed, checked proofs. The RISC0 backend is a different execution-proof route, not a substitute for these obligations or for application-policy completeness.

This is a bounded source study, not a build, security audit, verified compiler assessment or deployment certification. Exact pinned URLs, SHA256s and line ranges for JPL-01 through JPL-12 are in claims.json. Reading scope and five repository pins are in reading-coverage.json. Acquired current source is separated from the older Anoma library API.

## Tutorial: follow one resource through the boundaries

The legacy Anoma library represents a resource with a logic function, quantity, label, data, nonce and key-related fields. The logic receives the resource and transaction and returns Bool. A transaction carries commitments, nullifiers, a delta and a list named proofs. In this particular library, `Proof` aliases `Resource` (JPL-10). This naming is not evidence that the list contains cryptographic proof objects. Its test helper checks that every listed resource's logic returns true; this test helper must not be presented as a full ledger verifier.

An application could express an escrow release predicate requiring authenticated signatures and evidence. That is a useful programming pattern, but the inspected library does not establish a complete pending-request/settle/refund lifecycle. Moriarty's conditional settlement must include unfunded submitted requests as well as funded escrow, preserve residual duties after partial effects, and define evidence consumption and races. A Boolean predicate permitting a transfer says only what its author encoded; it does not establish the intended recipient, value, document truth, uniqueness or external finality unless those are explicitly checked and authenticated.

## How to use this prior art safely

Adopt the distinction between program admission, runtime predicate acceptance, execution proof, and compilation equivalence. Pin the admitted source language, compiler, libraries, flags, external primitives, target semantics and proof checker. Reject unchecked escapes from proof-relevant programs unless separately certified; this is a program-validity rule, not a project or solver whitelist. Application-selected counterparties remain legitimate policy inputs.

For a portable design, define a finite or structurally terminating proof-relevant core with explicit input and resource bounds. Structural termination alone does not establish affordable circuit size, small execution cost or bounded evidence lists. Make dependency closure part of admission: a checked entry point cannot rescue an unchecked imported primitive. Make signed policy identity and stage-specific authority persist through compiled artifacts and continuation migration.

For code reuse, first separate conceptual adoption from copying code. Juvix supplies GPLv3 license text; juvix-lean is MIT with notice conditions. Neither inspected stdlib root supplies an explicit license file. That absence leaves reuse permission unresolved; it does not establish prohibition or permission. Transitive dependencies and generated runtime copying need their own license inventory (JPL-12). No legal compatibility conclusion is made here.

## Reference: exact source boundaries

| Area | Source-supported result | Limit |
|---|---|---|
| Totality | Termination states distinguish checked, marked failure and failure; marked failure passes admission but is unsafe to normalize (JPL-01). | `terminating` is an escape, not a termination proof. |
| Flags | CLI can disable termination, coverage and positivity; options propagate (JPL-02). | Default checking does not imply all builds use it. |
| Positivity | `positive` bypass is documented and used by legacy resource types (JPL-03). | Do not transfer total-language metatheorems unchanged to this profile. |
| Other escapes | Unsafe package-version conflicts and developer Core sanity bypass are separate options (JPL-04). | These are not one generic “unsafe language” switch. |
| Compilation | Dispatcher offers Anoma, Cairo, native and conditionally WASI/RISC0 Rust (JPL-05). | No Midnight backend is established in this bounded inspection. |
| Anoma artifact | Output is serialized Nockma, with optional representations (JPL-06). | Output bytes are not a proof of signed business policy. |
| RISC0 artifact | Command writes Rust project/guest; template host proves guest execution and reads a u32 journal (JPL-07). | Journal policy, source correspondence and external truth are separate obligations. |
| Lean artifact | `--verify` generates equivalence statements with `sorry`; generated dependency follows `main` (JPL-08). | Scaffold is not a completed certificate or reproducible proof closure. |
| Lean library | Concrete IR verification primitives and contextual-equivalence theorem (JPL-09). | No blanket compiler, backend or financial correctness theorem inferred. |
| Resource API | Boolean logic, alias named Proof, and test assertions (JPL-10). | Not evidence of native refinement/linear typing or full cryptographic verification. |
| Versioning | Anoma library 0.8.0 depends on stdlib v0.7.0; acquired stdlib is 0.12.1 (JPL-11). | Do not combine them as a tested current stack. |

## Explanation: refinements, proof carriage and target portability

Refinement-style acceptance can be expressed as predicates on resource transitions, but this study did not establish a language-wide refinement-type discipline proving arbitrary economic invariants, nor a linear-type soundness theorem enforcing resource uniqueness. Runtime nullifier/commitment handling, source-level use disciplines, and cryptographic constraint proofs have different responsibilities. The word resource alone does not identify which layer prevents duplication.

Juvix Lean is the closest reusable verification idea: prove equivalence of each concrete transformation, compose the checked steps, and bind the final artifact. The pinned default generator still leaves the work open. Even fully discharging those Core equivalences would preserve the source program's meaning; it would not show the source program matches a user's intention. Moriarty needs both a signed-policy satisfaction theorem and compilation correspondence through the exact ZKIRv3 semantics, plus trustworthy external observation and ledger uniqueness assumptions. A verified implementation of a mistaken policy remains mistaken.

Nockma and RISC0 targets demonstrate architectural separation of source and execution environment. They do not demonstrate that porting to Midnight is mechanical. Representation, arithmetic, hash/signature primitives, witness/public-input layouts, effects, cost limits and transaction semantics need explicit translations and correspondence obligations. This source inspection supplies a research route, not an implemented port or evidence of unrestricted all-DeFi expressiveness.

## Adversarial witnesses and open work

1. A nonterminating function marked `terminating` passes the escape path. Require rejection from the proved profile or a separate accepted termination certificate; retain a terminating but nonstructural example as a non-vacuity witness.
2. A source predicate always returning true can compile and execute correctly while allowing theft. Require policy completeness and signed constraints, not merely successful proof execution.
3. A proof artifact containing `sorry`, or a dependency silently advanced from `main`, looks like formal output but lacks the required closed evidence. Reject unproven obligations and unbound proof dependencies; accept a completed pinned certificate as the positive case.
4. An empty or incomplete list of resource logics can satisfy an all-list test. This is a limitation witness for the helper, not an asserted ledger exploit. Require authenticated completeness and independent ledger checks.
5. A resource transition passes a local predicate but spends an already consumed resource or refunds after successful settlement. Require global uniqueness plus conserved residual duties and explicit settle/refund competition.
6. A genuine execution proof on the wrong guest/program or with omitted policy inputs proves the wrong statement. Bind source policy, compiler profile, target program and public statement; independently authenticate external evidence.

No compiler, Lean checker or adversarial program was run. These are source-derived or explicitly hypothetical witnesses for future experimental work. Whether current ARM-RISC0 resource logic supplies additional mandatory proof constraints belongs to the separate resource-machine study, not this legacy library inference.
