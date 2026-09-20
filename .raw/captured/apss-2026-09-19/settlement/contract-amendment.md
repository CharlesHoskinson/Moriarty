# Proposed phase-aware settlement amendment

**Status:** proposed documentation/OpenSpec amendment; no implementation or acceptance claim. Parent integration should retain existing requirement identities and add these as scoped obligations rather than weakening current financial acceptance. The primary source is [Midnight transaction semantics](https://docs.midnight.network/concepts/how-midnight-works/semantics), specifically `#transaction-fallibility` opening three paragraphs, `#well-formedness` and `#phase-execution`; captured in `sources/SET-07-midnight-semantics.html/.txt` with hash in `manifest.json`.

## Proposed product-contract paragraph

Moriarty must relate source behavior to the actual phase semantics of the pinned Midnight transaction. A local evaluator rejection establishes no general ledger rollback claim. When the backend uses guaranteed and fallible phases, fallible failure can preserve guaranteed effects and fees. The signed intention must bound every allowed outcome, including fees, partial effects, residual obligations and surviving authority. The proof/acceptance relation and receipt must distinguish non-inclusion, partial ledger success, full financial fulfillment and unresolved observation. A financial success claim requires the complete authorized outcome and its finalized native evidence. Proof correctness, authenticated observations, finality and conditional liveness remain separate claims.

This requirement does not impose administrative approval on public developers. It constrains publicly checkable supported semantics. Internal engineering reviews and funded campaign controls apply to maintainers' operations.

## Proposed EARS requirements

| ID | Requirement | Suggested existing scope |
|---|---|---|
| SET-MOR-001 | When a source action is lowered, the toolchain shall bind the generated transaction's phase/effect mapping to the source, profile, compiler and ledger versions. | MC01/MC04; source-to-ledger lifecycle |
| SET-MOR-002 | When an intention authorizes execution, it shall bound gross debits, fees, residual liabilities and authority across every permitted phase outcome; a successful outcome shall additionally satisfy the signed net goal. | MC04/MC05; intent refinement |
| SET-MOR-003 | When fallible execution fails after guaranteed execution succeeds, the successor observation and accounting shall retain all actual guaranteed effects and fees and shall not report financial fulfillment unless its separate predicate holds. | MC02/MC04/MC08 |
| SET-MOR-004 | When a proof is accepted, the acceptance relation shall bind the applicable phase semantics, complete effects and predecessor/authority consumption, retaining all mandatory claims. | MC03–MC05 |
| SET-MOR-005 | When execution evidence is absent or ambiguous, the client shall report an unresolved outcome and reconcile before issuing a conflicting retry; elapsed local time alone shall not establish non-execution. | MC02/MC06/MC08 |
| SET-MOR-006 | When a receipt relies on an attested external observation, it shall retain that dependency even if a proof verifies the attestation or a relation consuming it. | MC04/MC05/MC08 |
| SET-MOR-007 | When a completion or recovery bound is advertised, the interface shall state its liquidity, witness, inclusion, finality and other availability assumptions separately from execution-work bounds. | MC01/MC06/MC08 |

## Required positive and negative scenarios

1. **Full success:** independently compute the net credit, total fee, debt reduction and cumulative authorization consumption; verify complete finalized observations.
2. **Guaranteed rejection:** demonstrate non-inclusion for the pinned failure; do not attribute hypothetical fees or effects as observed.
3. **Fallible rejection with retained effects:** create a meaningful candidate that reaches fallible failure; compare surviving guaranteed effects/fees and unchanged or correctly residual financial duties. An all-zero output is not an acceptable substitute for complete readback.
4. **All-outcome cap:** reject a transaction shape whose guaranteed effect already exceeds authorized failure exposure, even if its success path would later refund it.
5. **Retry:** after a fee-only partial result, preserve cumulative debit and work; reject a new attempt that would exceed remaining authority or conflict with unresolved execution.
6. **Wrong evidence:** reject an adapter that promotes verified signatures to proved external payment or calls indexer inclusion canonical finality.
7. **Unsupported phase mapping:** if the current accepted profile cannot establish its phase/effect relation, keep its wider correspondence/acceptance claim open or reject that unsupported construction; do not silently broaden the meaning of atomicity.

Each scenario needs a named owner, pinned artifact, verification command and expected complete observation in the implementation package. Source inspection and local checks precede separately authorized Preview work. Existing acceptance identities, historical receipts and resource controls remain unchanged; new prose does not close them.
