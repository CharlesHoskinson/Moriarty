# SP05 fixed kernel custody packet

Status: prepared for bounded implementation admission and review. Parent decision approves preparation only. This packet does not admit a wallet, compiler, Docker process, proof or transaction. The current SP05 comparison utility is independently blocked and must pass its correction review before integration. All SP01-SP12 obligations remain.

## Purpose and evidence boundary

Use the accepted Moriarty-generated loan and swap kernels to control real unshielded token primitives. Preserve exact source and generated kernel bytes. The first implementation result is a pair of locally compiled Compact contracts and actual compiled-runtime tests. Local runtime transcripts are not ledger settlement. A second, separately admitted task must execute the contracts against a local Midnight ledger, authenticate consumed inputs and recipients, decode finalized state and compare every financial field. Preview is a third separately admitted campaign after those results pass independent GPT-6 review.

The implementation author is Grok 4.6 high. A fresh independent GPT-6 reviews source, tests and frozen candidate. User publication and bounded continuation authority applies; no user confirmation is needed for this packet preparation.

## Protected inputs and owned outputs

The source manifest pins the accepted loan.mori and swap.mori, both generated kernel.compact/metadata.json/bound-program.json, arithmetic.compact, mapping implementation and tests, Compact standard library and local primary documentation, custody decision/proposal and this packet. Never change those inputs or substitute a handwritten financial result for transition0/transition1.

A new isolated worktree starts from the current recorded main. Own only these proposed paths, plus FOREMAN_REPORT.md and FOREMAN_REPORT.json:

- experiments/moriarty-midnight-financial/custody/README.md
- experiments/moriarty-midnight-financial/custody/build.mjs
- experiments/moriarty-midnight-financial/custody/generate.mjs
- experiments/moriarty-midnight-financial/custody/loan.compact
- experiments/moriarty-midnight-financial/custody/swap.compact
- experiments/moriarty-midnight-financial/custody/bindings.json
- experiments/moriarty-midnight-financial/custody/runtime.test.mjs
- experiments/moriarty-midnight-financial/custody/test-evidence.json

Do not modify the comparison utility, fixtures, shared package.json, current language implementation or any live worktree. Build outputs go to an explicitly supplied untracked directory. Own at most384KiB of source/report bytes; compiled artifacts have a separate future bound. No new package installs: use the exact existing runtime and compiler versions from the admission binding. A missing required API is a precise blocker, not permission to install a new stack.

Generate wrapper source reproducibly from the pinned metadata and fixed role/token binding table. Import or copy byte-identical protected generated kernels and arithmetic into the build staging directory; retain original hashes and module prefixes. Separate loan and swap contracts avoid collisions between exported kernel structs. Build must reject stale metadata/kernel/source mappings. The generated wrapper invokes the generated transition, constrains every returned effect identity and materializes its returned state and effects. It must not merely store a separately computed expected amount.

Proposed commands, to become executable through implementation:

- node custody/generate.mjs --output-dir DIR
- node custody/build.mjs --output-dir DIR --skip-zk
- MORIARTY_CUSTODY_ARTIFACTS=DIR node --test --test-reporter=tap custody/runtime.test.mjs

Commands are run with the custody parent directory as documented by the implementation; no implicit wallet or network imports. The test command needs an explicit artifact directory and fails when it is absent. Exact final argv and environment variable names belong in the implementation report.

## Constructor, role and network binding

A UserAddress is the standard-library struct containing exactly32 raw address bytes. Each constructor seals its program digest, network tag, role capability commitments and payout addresses. The enrollment of these values is trusted for this fixed test. No setter, migration or re-enrollment method exists in this packet.

Capability preimage is a fixed Vector<4,Bytes<32>>: role-specific padded ASCII domain, network tag, pinned program digest, private32byte role secret. Store only disclose(persistentHash(preimage)). Distinct role secrets and addresses are required in local controls. A secret must never be disclosed, logged, placed in bindings.json, fixtures or reports. Circuit arguments remain private unless their specific public values must be disclosed. Do not use ownPublicKey, a witness truth value or host-selected authorization as evidence of role possession.

Loan roles are borrower and lender. Borrower authorizes initialization, accrue and settle. Lender is a sealed distinct payout identity; this fixed inherited loan fixture does not claim a newly signed bilateral loan agreement. Swap roles are trader and provider. Provider authorizes initialization and close; trader authorizes swap. Wrong-role secrets must fail even when supplied with the correct public actor ID.

All action circuits require the exact expected program digest, network tag and current revision or bind those values through immutable contract state and an equality-checked call envelope. Deployment identity is the executing contract address; token colors derive from that address and fixed domain separators. The independent receipt adapter must also check the actual ledger network and deployment. A constructor network label is not proof of which network executed it.

Time uses the protected kernel horizon2000000000. If a caller supplies now, constrain it with the pinned block-time primitives to an explicit finite acceptance window, and pass that same constrained value as KernelObservations.o0. The proposed window is300seconds: blockTimeGte(now) and blockTimeLt(now+300), with checked bounds and horizon assertion. Local tests supply explicit simulated block-time context. They must not claim a simulated context authenticates public time. If the compiled runtime cannot execute these primitives, record that exact gap rather than replacing them with a witness boolean.

## Initialization and financial transitions

Initialization has a persistent one-shot guard. It creates the fixed kernel initial state, lifetime remaining and revision once. It must not refill a consumed lifetime, reset dues or mint on re-entry. Initialization is a distinct funding event before the financial trace; its outputs and issuer are explicitly recorded. Test tokens are not real USD or an existing asset deployment.

Loan token domain is pad32("moriarty:sp05:usd:v1"). Mint20000000000 units to the sealed borrower address; lender starts0. Its nominal unit is USD_micro at scale6, with one ledger integer unit per nominal integer unit. Initial principal5000000000, dues/paid0, cursor0, episode_closed0, remaining2, revision0. Other initial fields are taken exactly from pinned bound-program, not inferred from ordinal position alone.

Accrue invokes loan transition0 with the sealed borrower actor mapping and constrained time. Its output principal4500000000, principal due500000000, interest due33972602, cursor1, remaining1, revision1 must follow from the kernel. It creates two distinct nominal dues; no token transfer occurs. Preserve the exact two DueCreated records and identities.

Settle invokes loan transition1 on the stored kernel state with the exact USD_TEST_ASSET mapping and amount533972602. Constrain all returned transfer and DueSettled identifiers against metadata, then receiveUnshielded of the derived USD color and returned transfer amount and sendUnshielded of that amount to the sealed lender. Store returned state and all due-settlement fields. Loan residual principal4500000000 survives with its actual debtor/controller and is not marked discharged by episode closure or lifetime0. Borrower synthetic balance19466027398 and lender533972602 are expected only after receipt-level payer attribution and balance checks; the circuit alone has not authenticated who supplied the received token.

Swap domains are pad32("moriarty:sp05:asset-a:v1") and pad32("moriarty:sp05:asset-b:v1"). Initialize1000000 AssetA and2000000 AssetB to the contract and100000 AssetA to the sealed trader. Provider initial balances are0. Supply totals1100000A and2000000B; the initializer is the test issuer, not an asserted external liquidity deposit. Kernel remaining8/revision0 and all state fields must match pinned bound-program.

Swap invokes transition0 with exact trader/from/recipient and AssetA/AssetB mappings, input10000 and independently specified minimum output. Receive10000A and send returned19743B to sealed trader. Require sufficient entry B reserve and consistency with stored reserve; account for received A through explicit in-call delta. Fee30A remains inside gross input/pool reserve, not a second transfer. Preserve all returned state, remaining7/revision1, pool1010000A/1980257B, trader90000A/19743B. No recipient argument can redirect the output.

Close invokes transition1 under provider capability and current revision. Both returned transfers go only to sealed provider with exact colors and returned reserve amounts. Store closed flag1, reserves0, provider1010000A/1980257B, remaining6/revision2; do not refresh remaining to8. Reject replay/late swap and wrong provider. The six unused units are not copied to another continuation by this packet.

Unshielded balances are contract entry balances: neither preceding receive nor mint changes the queried value. Use explicit bounded intra-call deltas and standard-library reserve predicates. The actual ledger must enforce complete input/output funding. A compiled-runtime query transcript by itself does not prove consumed UTXO ownership or transaction balancing.

## Required local tests and independent oracle

Write meaningful tests first and retain the actual RED missing implementation or violated behavior, then implement and record GREEN. Tests import the compiled Contract and ledger decoder and use actual runtime constructor/circuit contexts. They must not substitute a JS mock for the Compact circuit. Independent expected constants derive from source-level arithmetic: floor(5000000000*8*31/(100*365))=33972602, residual4500000000; floor(10000*997*2000000/(1000000*1000+10000*997))=19743. Test all state fields and every returned transfer/due record, token color, recipient and lifetime counter.

Controls include wrong secret, another valid role secret, arbitrary actor/recipient, altered program/network binding, wrong/current/stale revision, repeat initialization, accrue replay, settlement before accrual, wrong settlement amount/color/unit mapping, wrong min output, wrong hints, closed swap, invalid time window, insufficient entry reserve and all financial rollback snapshots. Each negative must pass prerequisites up to its nominated failing predicate; otherwise label the earlier actual failure. A failure after an earlier successful call retains that successful call's complete state. An unsuccessful single circuit must not produce a new financial state.

Some ledger controls cannot be established by runtime tests: missing actual funding, authenticated payer attribution, transaction fees and finality. Preserve them as explicit required next-stage controls. Never satisfy them with asserted booleans, mocked balances, a receive instruction or a numeric equality.

The complete receipt adapter must later bind actual signed unshielded offers and consumed UTXO owners to the borrower/trader. It must distinguish third-party funding, exact source colors, recipient outputs/change, fees and complete canonical state. Future integration cannot accept the current independently blocked differential utility until its correction passes review. A transaction can succeed while its claimed payer comparison fails.

## Recovery, resource admission and exit

This packet creates no wallet identities. Local runtime tests use explicit synthetic public addresses and private in-memory test secrets, labeled synthetic; no real custody or recovery claim. Before Docker or Preview, prepare a separate exact identity task: existing Preview borrower/trader, distinct recoverable lender/provider, operator-control disclosure, private0600 secret storage and recovery readback without printing secrets. Do not import existing wallet modules merely to inspect them; some create identities at module load.

Before implementation dispatch, record base and all source/packet hashes, exact owned files, compiler/runtime versions, full worker/RED/compile/test/review/closure limits, retained/build limits and concurrency. Proposed implementation bound2400seconds, one author dispatch, no implicit retries. Proposed independent compile/runtime verification360seconds and fresh GPT-6 result review600seconds, plus preparation/closure separately. These are proposals, not charged execution allocations. No proof generation is included in --skip-zk.

Exit is a reviewed reproducible generated-wrapper source and actual compiled-runtime result with honest ledger gaps. It closes no Docker settlement, Preview, mandatory PCD, full BNF, K semantics or sprint gate by itself. Next steps are real local ledger execution/receipt verification, then the separately admitted Preview campaign with at most two attempts per case and actual transaction IDs posted to the user.
