# SP05 custody packet review

**APPROVED for separately recorded bounded implementation admission.** This is a preimplementation packet review, not approval of code, compilation, runtime results, ledger settlement, finality, proof, Preview or a sprint gate.

Packet SHA256 `acd2b9caac1eabdb50a64bb82f91dc49eb9a4ac0417eeb335cd77e1a17386265`. All16 protected source pins and byte counts match before/after; the parent proposal matches its decision hash. No packet-level blocking finding. The already-approved capability route is preserved.

## Protected source and actual kernel invocation

Both generated kernels are pure transition functions, with complete after/remaining/revision/effect results and no token ledger effects. Packet explicitly requires invoking these functions from generated wrappers, checking returned identities, materializing returned state/effects, retaining byte-identical kernel/arithmetic inputs and rejecting stale metadata. This is the required division of responsibility; absent wrapper source is expected at preimplementation review.

Evidence: `experiments/moriarty-language/compact/generated/loan/kernel.compact:13`, `experiments/moriarty-language/compact/generated/loan/kernel.compact:76`, `experiments/moriarty-language/compact/generated/swap/kernel.compact:13`, `experiments/moriarty-language/compact/generated/swap/kernel.compact:118`, `experiments/moriarty-language/compact/generated/loan/metadata.json#/actions`, `experiments/moriarty-language/compact/generated/swap/metadata.json#/actions`.

## Capability role and identity binding

Trusted enrollment seals role commitments and32-byte payout addresses; secret material is private, role-domain/network/program-separated and not stored in reports. Public text IDs from metadata are constrained mappings, not authentication. Borrower authorizes loan actions, trader authorizes swap, provider authorizes initialization/close. Lender is a fixed distinct payee, not claimed as a bilateral signer. Current revision, fixed executing contract/token color, network/program envelope and no recipient redirection are explicit requirements. This does not establish general wallet signing or independent institutional custody.

Evidence: `evidence/moriarty-completion-program-2026-09-07/SP05/custody-decision-01/proposal.md`, `raw/midnight-docs-2026-09-07/markdown/compact/reference/compact-reference.md:2319`, `raw/midnight-docs-2026-09-07/markdown/tokens/unshielded-token.md:245`, `repos/LFDT-Minokawa/compact/compiler/standard-library.compact:294`.

## Contract token custody versus payer attribution

receiveUnshielded only increments a color/amount input counter and has no payer parameter. sendUnshielded increments outputs and records recipient claims. Packet correctly requires actual signed offers and consumed UTXO owner checks later before claiming borrower/trader debit. Third-party funding remains a separate negative and may invalidate a financial comparison despite successful transaction execution.

Evidence: `repos/LFDT-Minokawa/compact/compiler/standard-library.compact:311`, `repos/LFDT-Minokawa/compact/compiler/standard-library.compact:320`, `repos/LFDT-Minokawa/compact/compiler/standard-library.compact:324`, `evidence/moriarty-completion-program-2026-09-07/SP05/custody-decision-01/proposal.md`.

## Initialization, supply and recipient generation

Token colors derive from domain plus kernel.self(). mintUnshieldedToken accepts Uint64, generates recipient spend claims and auto-receives mint-to-self. All proposed mint amounts fit Uint64. One-shot authorized initialization supplies loan borrower20,000,000,000 USD_micro, swap pool1,000,000A/2,000,000B and trader100,000A. Initializer is identified as synthetic issuer, not an external LP deposit. Packet forbids remint/reset and defers real identities/recovery to a separate admission.

Evidence: `repos/LFDT-Minokawa/compact/compiler/standard-library.compact:118`, `repos/LFDT-Minokawa/compact/compiler/standard-library.compact:296`, `raw/midnight-docs-2026-09-07/markdown/tokens/unshielded-token.md:222`, `raw/midnight-docs-2026-09-07/markdown/tokens/unshielded-token.md:247`, `experiments/moriarty-language/compact/generated/loan/bound-program.json#/manifest/initialState`, `experiments/moriarty-language/compact/generated/swap/bound-program.json#/manifest/initialState`.

## Complete financial sequence and residual duties

Interest floor33,972,602 remainder27,000, settlement533,972,602, borrower19,466,027,398 and residual principal4,500,000,000 match. Loan DueCreated IDs PR4/IP3, debtor2, creditor5, denomination1 and settlement asset0 match metadata. Swap output19,743 remainder162,290,000, fee30 retained, trader90,000A/19,743B and provider closure1,010,000A/1,980,257B match. Loan lifetime/revision2/0->1/1->0/2; swap8/0->7/1->6/2. Episode closure does not discharge residual loan notional or refill/copy allowances.

Evidence: `experiments/moriarty-language/compact/generated/loan/metadata.json#/stateFields`, `experiments/moriarty-language/compact/generated/loan/metadata.json#/textTable`, `experiments/moriarty-language/compact/generated/swap/metadata.json#/stateFields`, `experiments/moriarty-language/compact/generated/swap/metadata.json#/textTable`, `experiments/moriarty-language/spec/examples/loan.mori:90`, `experiments/moriarty-language/spec/examples/swap.mori:65`.

## Time, entry reserves and local runtime context

blockTimeGte/Lt take Uint64 while generated observation is Uint128; the proposed horizon2,000,000,000 and checked now+300 fit Uint64. Runtime0.16.0 createCircuitContext has an explicit time argument and uses ContractState.balance for entry balances; CallContext also records timestamp uncertainty. Packet correctly forbids treating preceding receive/mint as updating queried entry balance. Synthetic context supports tests but is not network time, funding or finality evidence.

Evidence: `repos/LFDT-Minokawa/compact/compiler/standard-library.compact:275`, `repos/LFDT-Minokawa/compact/compiler/standard-library.compact:279`, `repos/LFDT-Minokawa/compact/compiler/standard-library.compact:324`, `/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/compact-runtime/dist/circuit-context.d.ts:30`, `/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/compact-runtime/dist/circuit-context.js:46`, `/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/onchain-runtime-v3/onchain-runtime-v3.d.ts:275`.

## Negative controls, rollback and evidence hierarchy

Required tests invoke compiled Contract/ledger decoder with real runtime contexts, inspect every state/effect field, isolate prerequisite failures and retain earlier successful calls while rolling back a failed single circuit. Runtime Effects exposes unshieldedMints, inputs, outputs and recipient claims to inspect. Packet retains missing funding, transaction fees, payer attribution and finality as ledger-stage obligations and blocks integration with the currently unaccepted comparison utility.

Evidence: `implementation-packet.md#Required local tests and independent oracle`, `implementation-packet.md#Purpose and evidence boundary`, `/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/onchain-runtime-v3/onchain-runtime-v3.d.ts:335`, `raw/midnight-docs-2026-09-07/markdown/compact/reference/compact-reference.md:2465`.

## Command/resource scope

Owned source/report set,384KiB limit, explicit output-directory commands, skip-zk, no installs, fresh worktree, one proposed2400-second author dispatch,360-second verification and600-second result review are explicit proposals. Packet requires exact base/hash/version/argv/environment/build/retention/concurrency/closure limits before dispatch. This review authorizes no command execution itself and does not convert proposed allocations into charged ones.

Evidence: `implementation-packet.md#Protected inputs and owned outputs`, `implementation-packet.md#Recovery, resource admission and exit`.

## Concrete checks retained for the next phase

- **before author dispatch (I01):** Materialize the already-required admission binding: current base, this packet/source hashes, exact compiler/runtime paths and versions, all owned files, actual argv/cwd/env, author/RED/compile/test/review/closure limits, retained/build bytes and concurrency. Pin supplemental runtime files if used. No missing-path fallback install or network command.
- **implementation result (I02):** Publish fixed nonsecret role-domain/network/program/address encodings in bindings.json; generate the mappings by metadata names and textTable, check all returned effect identities and all kernel state fields. Preserve both nominal due histories and the live residual notional/controller. Never log capability preimages.
- **implementation result (I03):** Inspect actual runtime output claims and mint/input/output maps, not only kernel return amounts. Loan settle must have input/output533972602 of derived USD color and lender recipient. Swap must have input10000A, output19743B to trader. Close must have both reserve outputs to provider. Initialization supply and recipients must appear in actual effects.
- **implementation result (I04):** Choose an explicit positive min_out fixture independently, e.g.19700, and use19744 for the isolated minimum-output failure with valid secret/revision/time/hints and entry reserves. A lower min_out is valid under the kernel and must not be rejected merely for differing from the positive fixture. Document whether close-before-swap is accepted per protected source; do not invent a kernel phase rule.
- **implementation result (I05):** Set explicit synthetic block time and relevant uncertainty/context fields; test both sides of the300-second window and horizon with safe Uint64 conversion. Apply entry-reserve checks to all spending paths, including both close colors, and use explicit in-call deltas after receive/mint. Record exact earlier failure for each negative.
- **later ledger/Preview admission (I06):** Keep actual funding, signed payer/UTXO ownership, exact recipient/change/fees, canonical decoded finalized state, network/deployment identity and recovery evidence separate from runtime transcript success. Do not integrate the blocked comparison utility until its correction passes independent review.

## Verification scope

Independently checked all loan and swap initial/intermediate/final numeric kernel fields, effect identity mappings, remaining/revision values, supplies, mint widths and arithmetic remainders. Full named states, source hashes and command results are in `gpt6-packet-review.json`.

One exploratory read returned exit1 because the proposed directory was absent and a default runtime search yielded no matches; focused existing-file reads resolved source/API inspection. Absence of proposed implementation is expected and was not treated as a defect.

Only these review files were written. No compiler, runtime execution, build, network, wallet, Docker, proof, git mutation or transaction was attempted. Actual output-generation behavior must be shown in compiled-runtime evidence; runtime effect maps alone do not establish signed funding or ledger settlement.
