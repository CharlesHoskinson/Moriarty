# Proof-carrying transactions for a bounded financial DSL

Research date: 2026-09-06. Status: design research, S2; no PCD implementation or
backend compatibility result. Authority: primary papers and official references,
with explicitly labeled design inferences. Source collection: SRC-0040.

The user has made proof-carrying transactions a core language requirement.
The proposed acceptance meaning is: a transaction supplies evidence that it
follows the specified financial rules from admissible predecessor states, under
declared assumptions and bounds. Defining those rules remains the first task.

## What the sources establish

| Primary source | Finding and limitation |
| --- | --- |
| Chiesa–Tromer, [Proof-Carrying Data and Hearsay Arguments from Signature Cards](https://cs-people.bu.edu/tromer/papers/pcd.pdf), ICS 2010, §§1.2, 1.5, 4.3 | PCD concerns data's computational history, unlike proof-carrying code's future executions. Recursive justification needs appropriate knowledge assumptions. Genesis still checks compliance. This historical construction uses signature-card assistance; do not present it as assumption-free software or inherently zero knowledge. |
| Chong–Tromer–Vaughan, [Enforcing Language Semantics Using Proof-Carrying Data](https://cs-people.bu.edu/tromer/papers/plpcd-20140113.pdf), extended 2014 version, §§6.1–6.2 | Computations form a directed acyclic multigraph. A local predicate relates output, local input and incoming messages; the PCD machinery receives their proofs. The paper separately connects its predicate to language semantics. Moriarty needs that correspondence argument for its own relation. |
| Bitansky–Canetti–Chiesa–Tromer, [Recursive Composition and Bootstrapping for SNARKs and Proof-Carrying Data](https://cs-people.bu.edu/tromer/papers/bootsnark-20121228.pdf), STOC 2013 extended version, Remark 5.4, §§6, 8 | Local outgoing-message checks do not automatically enforce consistency across recipients. The generic construction's depth and extraction qualifications matter; verifying a predecessor proof is not by itself a theorem about arbitrary-depth PCD. These historical limitations are not impossibility claims about later constructions. |
| Kothapalli–Setty, [HyperNova](https://www.andrew.cmu.edu/user/bparno/papers/hypernova.pdf), CRYPTO 2024 extended author mirror, §§3–4, 7–8 | Multi-folding and generalized constraints supply useful PCD building blocks. Zero knowledge and additional compression have their own construction steps. Multi-folding alone does not define the financial composition rule. The mirror's exact revision is unstated and differs from latest ePrint metadata; its content hash controls this review. |
| Microsoft, [Nova implementation README](https://github.com/microsoft/Nova/blob/main/README.md), live observation | Practical IVC, commitment choices and separate compression exist. Different configurations imply different assumptions and setup requirements. This is a feasibility lead, not evidence of Nova verification inside Compact. |
| Midnight, [Compact reference](https://docs.midnight.network/compact/reference/compact-reference), live observation, circuit/loop/witness sections | Circuit calls cannot recurse and loops have compile-time bounds. Witness results are untrusted. These restrictions constrain execution shape; they do not establish contract policy correctness or correspondence to the deployed ledger. |
| Lamela Seijas–Nemish–Smith–Thompson, [Marlowe: Implementing and Analysing Financial Contracts on Blockchain](https://fc20.ifca.ai/wtsc/WTSC2020/WTSC20_paper_18.pdf), 2020, static analysis/formal verification sections | Marlowe separates analysis of individual contracts from semantic properties such as money conservation and termination. This supports a layered assurance design, not the claim that Marlowe already requires recursive PCD for every transaction. |

Additional leads, with narrower inspection: [SuperNova](https://eprint.iacr.org/2022/1758)
was checked at abstract level for non-uniform sequential IVC. For the 2020
accumulation work, [Bünz's institutional explanation](https://cs.nyu.edu/~bb/)
distinguishes accumulation from its final decider; the full paper was inaccessible
in this research lane, so its theorem-level assumptions remain unverified.
The [official Midnight ZK repository](https://github.com/midnightntwrk/midnight-zk)
offers a separate native-backend research lead. No reviewed source establishes
Nova–Compact integration.

Acquisition and access limits are recorded in the
[manifest](../../evidence/moriarty-reset-2026-09-06/research-manifest.json).
Five full primary PDFs were preserved using Scrapling static fetching. Robots
rules prohibited direct IACR PDF acquisition; a Midnight robots request returned
429, so direct acquisition there stopped. Browser-readable official references
are supplemental live observations, not locally pinned implementations.

## Five distinct correctness claims

The following is a **design inference** from the sources and the user's
requirements. Each claim needs its own statement and evidence:

| Layer | Claim to specify | What it does not establish alone |
| --- | --- | --- |
| Language | Typed bounded evaluation terminates and follows deterministic rules. | Correct financial formulas or useful contract policy. |
| Contract | Named invariants hold for every admissible input/history within the declared contract domain and horizon. | Truth of an oracle or continued participant cooperation. |
| Transaction | This authorized transition computes the stated new state and effects from the stated inputs. | Safety of every other possible transition. |
| PCD history | Accepted predecessor claims and this transition form a compliant history rooted in admissible genesis. | A unique live branch, consensus or exclusive spending. |
| Implementation | Surface, Core, proof relation, generated Compact/ZKIR and ledger interpretation agree within their stated scopes. | Correctness merely because one compiler invocation succeeded. |

A contract-level certificate may establish an invariant initially and prove it
is preserved by every admitted step. Transaction PCD can then bind execution to
that certified program and policy. This is a proposed connection, not an
existing Moriarty theorem. The design must specify certificate checking and
trust roots; hashing an unchecked certificate is not sufficient.

## Required transaction relation

Proposed requirements, **not a frozen schema or executable syntax**:

- Bind program/package/semantics versions, property manifest, bounds, network
  domain and verifier identity to the statement being accepted.
- For genesis, validate the initial state, initial authority and policy. For a
  later step, authenticate each predecessor statement and its corresponding
  proof. Specify input order, identity, fan-in, splitting and joining.
- Check typed inputs, declared numeric behavior, authorized intent, observation
  provenance/freshness rules, event ordering and the deterministic transition.
- Bind resulting state commitments, asset movements, disclosures and consumption
  identifiers. No unproved side effect may be added after proof generation.
- Make the final verifier/decider explicit. Reject missing, malformed, wrong-
  domain or wrong-statement proofs. A folded accumulator is not automatically
  an accepted final proof.
- Couple proof acceptance to live ledger consumption and finality checks.
  Explain separately what makes observations trustworthy. A valid local history
  can otherwise coexist with a conflicting valid history.

The prover uses predecessor proofs; the financial compliance predicate can be
defined over their messages and local witnesses. Keep that semantic definition
separate from the selected cryptographic realization. A fixed finite verifier
circuit may verify a previous proof without recursive calls in the DSL source.

## Finite and bounded must be explicit

Turing incompleteness supports predictable evaluation but does not by itself
make every input domain finite, prove a desired invariant or make exhaustive
checking affordable. Moriarty should reject programs without a checked bound
for numeric representations and intermediates, collections, schedules, contract
nesting, per-step work and predecessor fan-in. It must define rounding,
overflow, failure and observation domains rather than inherit host behavior.

Specify contract lifetime/event horizons separately from per-transaction circuit
size and cryptographic history depth. Open-ended financial arrangements need an
explicit bounded analysis/execution scope. Creating a continuation must not
silently restore an exhausted lifecycle budget. A bounded prefix result is not
a whole-lifetime proof without a completeness argument or induction.

Finiteness is a language design requirement here; cryptographic soundness remains
conditional on the chosen construction's assumptions. “Proof of correctness”
must always name which of the five claims is actually proved.

## Decisions for the next design cycle

First derive the relation from ACTUS and DeFi examples. Then compare a single
serialized state machine using IVC with bounded multi-input PCD composition and
a target-native proof route. Assess topology, privacy, ledger anchoring and
developer behavior before selecting a backend. Test the smallest real target
verifier only after the required statement is clear.

The next [planning brief](../superpowers/plans/2026-09-06-actus-defi-pcd-replanning.md)
makes target study, a semantic proposal and a developer mock the reviewable
outputs. Neither a proof-system choice nor a general DSL has been implemented
by this research.
