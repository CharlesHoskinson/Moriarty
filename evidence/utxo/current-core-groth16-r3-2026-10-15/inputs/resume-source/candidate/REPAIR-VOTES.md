# Repair-author votes: review amendments C4, C9, C15 and the corrected proposal PC-1 to PC-4

**Voter:** `claude-opus-5-5`, acting as the SOURCEONLY review-repair author of this successor. I authored this repair's changed bytes, so these are design and resource votes, not an independent review of the successor. The r2 author, the compiler reviewer and I are the same model family, so independence is at the level of context only. No other model or agent was consulted. These votes give no execution, setup, key, adoption or target authority, and the root does the counting.

**Counting rule** (unchanged from claim-utxo-abi-v1 VOTES.md and the review's section 8): a vote counts only for the exact text it names. An amended agreement is not a vote for the literal text. Two votes are recorded only where the texts are actually identical. For the request blocks, identity is checked mechanically: `CORRECTED-EXECUTION-REQUEST.json` → `text_identity_with_reviewer_request` compares each block with the reviewer's JSON value.

## 1. Review amendments on the r2 author's C-items

| Item | Exact text voted on | My vote | Reason |
|---|---|---|---|
| **C4** (reviewer amendment) | "Add the balance-row-presence premise (F-04) and the unregistered-duty frame gap (F-05). The `REFUSALS` row for 'missing explicit balance' must say premise, not constant." | **AGREE (literal)** | I re-read Core `runtime.rs:609-627`: for every Transfer/Fee account Core requires an explicit `(asset, account)` row and otherwise refuses with `mori::core::balance`. The layout has `PreCell` UInt slots and no presence wire (`statement.rs`, `circuit.rs` entry), so a pre-balance of 0 for a missing row satisfies every constraint. `plan.rs:322-334` represents only source-registry duties, so an unregistered incoming duty has no slot at all. Implemented here as a relabel (`Support::TargetPremise`, TARGET_OBLIGATIONS, COVERAGE.md), **not** a fix. |
| **C9** (reviewer amendment) | "Add the Cargo.lock digest and toolchain identity (catalogue or build digest). The harness must recompute the caller-supplied digests (F-08)." | **AGREE (literal)**. Additive clause, my first vote: "In this successor both are proposed catalogue/admission metadata. Moving either into `CompilerBuild`, and so into the circuit header, changes the relation-identity format and needs its own vote and review." | `identity.rs` binds only `{compiler_source_manifest_sha256, core_closure_sha256}`, both caller-supplied and only checked as nonzero. The same BUILD-MANIFEST built with a different lock or toolchain can produce different matrices, and that difference is invisible. The literal text allows either location, so I can agree with it as worded. |
| **C15** (reviewer amendment) | "Agree in substance. It also needs an explicit toolchain selection (F-06), a hermetic CARGO_HOME or recorded side effects, and the checksum-level lock predicate (F-07)." | **AGREE (literal)** | `~/.rustup/settings.toml` names `stable` as the default (read as data), and no `rust-toolchain.toml` exists on the layout's ancestors, so only absolute 1.99.0 binaries select 1.99.0. Name-level lock checks cannot detect a newer cached version or a different checksum. |

**Tallies for the root (my reading):**

- C4 literal (r2 author's text): 1 vote (author).
- C4 as amended by the reviewer: **2 votes** (reviewer, repair author).
- C9 literal: 1 vote. C9 as amended by the reviewer: **2 votes**. My additive clause: 1 vote.
- C15 literal: 1 vote. C15 as amended by the reviewer: **2 votes**.

## 2. Corrected resource/execution proposal PC-1 to PC-4

| Item | My vote | Reason |
|---|---|---|
| **PC-1** (reviewer literal) | **AMEND.** I cannot agree with the literal text. My text **PC-1r** is in `CORRECTED-EXECUTION-REQUEST.json`. | The literal text requires "BUILD-MANIFEST must stay 2c943b51…". The mandatory F-04 correction is in `src/coverage.rs` and `src/adapter.rs`, and F-12 is in `src/circuit.rs` and `src/error.rs`. Both sit under BUILD-MANIFEST coverage, so it moved to `4e3ba2e2…3b47` (`BUILD-IDENTITY-DELTA.json`). The F-01 part of PC-1 is done as its first suggested option. PC-1 literal: 1 vote. PC-1r: 1 vote. |
| **PC-2** (reviewer literal) | **AGREE (literal)**. Additive **PC-2a** (my first vote): the authorization record also names the toolchain binary hashes, the lock-union registry-subset digest, the REPAIR-STATIC-CHECKS and DELTA digests and this request's digest. | The literal text names the right objects generically (candidate SOURCE-MANIFEST and BUILD-MANIFEST digests, the closure, the review manifest, the request), so it applies to the successor unchanged. PC-2a binds F-06, F-07 and F-08 inputs that PC-2 leaves implicit. **PC-2 literal: 2 votes.** |
| **PC-3** (reviewer literal) | **AGREE (literal)**. Additive **PC-3a** (my first vote): also record `REPAIR-EXPECTATIONS/FREEZE.json`, labelled repair-author and non-independent. | The frozen reviewer artefacts are unchanged (aggregate `7f6a4d1e…b53c` recomputed, 17/17 files match; addenda `bdc44401…` and `66b91c46…` unchanged). **PC-3 literal: 2 votes.** |
| **PC-4** (reviewer literal) | **AMEND (citation erratum only).** My text is **PC-4r**. | I agree with the rule itself. The literal text says the reviewer's vote "is in REPORT.md section 9", but section 9 of `a0d1725f…` holds the hostile and mutation proposals; the votes are in section 8 and the request review in section 10. Since the texts are not identical, no two-vote count arises. The underlying two-vote rule is existing governance (AGENTS.md) either way. PC-4 literal: 1 vote. PC-4r: 1 vote. |

### Request blocks (resource and execution text)

These reviewer blocks are copied into my request **by value**, so the reviewer's text and the one I vote on are identical. For each I vote **AGREE (literal)**:

| Block | Reason |
|---|---|
| `isolated_layout` | The text is generic over "the candidate SOURCE-MANIFEST". The successor keeps `CORE-CLOSURE-PINS.json` and the manifest format that `copy_verify.py` reads. |
| `toolchain.selection`, `toolchain.control` | These close F-06. I re-hashed the bytes read-only (cargo `e951141c…`, rustc `f3834d26…`, `librustc_driver` recorded), and they equal the reviewer's. |
| `environment` | Offline, no wrappers, no RUSTFLAGS. I found no `~/.cargo/config{,.toml}`. |
| `sandbox` | 8 GiB address space, per-step CPU caps (60 s for lock steps, 900 s for builds and tests), core dumps off, nice 10, 1800 s wall and a 3 GiB disk floor. This is a compile-and-test-only scope with no setup or proving, so rustc dominates memory. These limits match both earlier proposals in substance. Exhausting them is BLOCKED and recorded, never retried in the same authorization. |
| `never`, `interpretation` | They keep setup, keys, proofs, network, git, Aiken, SDK, ledger and any target claim out of scope. |
| Commands `E1`, `E1v`, `E1h`, `E2`, `E3a`, `E3c` | They are correct as written for the successor. |
| `scoring.independent`, `expected_documented_gaps`, `fail_handling` | E5 alone is independent. M-14 is still expected SATISFIABLE because F-04 is relabelled, not fixed. |

**Each of these blocks has 2 votes on identical text** (reviewer, repair author).

Amended blocks, with my first vote only:

- `E0`: the successor path, and record the recomputed digests.
- `E1hv`: an exact pair comparator replaces "a reviewed diff".
- `E3b`: the F-01 repair is done, and the build result is a prediction only.
- `E4`: 41 tests.
- `E5`: the harness's stale build constant is declared.

`PC-0` (a new full-candidate and delta Opus audit before any command) and the `inherited_counters`, `lock_identity` and `identity_binding` sections are new text with one vote (mine).

**Not authority.** Two votes on the resource text do not authorize a run. PC-0, root authorization (PC-2) and the frozen-expectation record (PC-3) still gate every command. Maintainer review and resource controls apply only to maintainer-run work and never become public developer compile, prove or deploy permissions.

## 3. Items not re-voted (preserved as recorded)

- **C1–C3, C5–C8, C10–C12, C14 and the C13 slot** already have two literal votes (author, reviewer). I cast no further vote; nothing in this repair touches them.
- **D-13 value**: I **ABSTAIN**, as the author and the reviewer did. It is a codec decision outside this repair, and the split (`Some(plan_total)` versus `None`) stays recorded.
- **P4-1 to P4-6** and the four audit questions: unchanged as tallied in review section 8. That includes the dissent from "refusal codes = first violated constraint region", which the author and reviewer hold.

## 4. P3 items: no vote cast, disagreements preserved, reconciliation deferred

I cast **no P3 vote**. The positions on record are:

| Item | V5 literal | r2 author | Compiler reviewer (`a0d1725f…`) | ABI review V6 (`f3ae8556…`) |
|---|---|---|---|---|
| P3-1 | proposed | AGREE | AGREE | AGREE (+ additive) |
| P3-2 account map | proposed | AGREE | AGREE | **CHANGE**: every account, custody or external, has a debit authority; full payout addresses |
| P3-3 | proposed | AMEND | AMEND (second vote for the author's text) | AGREE (+ additive) |
| P3-4 | proposed | AGREE | AGREE | AGREE (+ additive) |
| P3-5 | proposed | AGREE | AGREE | **CHANGE** (min-ADA, reserve) |
| P3-6 | proposed | AGREE | AGREE | **CHANGE** (lovelace allowance) |
| **P3-7 debit authority** | external accounts with gross debit > 0 | AGREE | AGREE | **DISSENT as worded; CHANGE**: authority for **every debited account, including custody-held balances**, thread-advance authority, and a **signed intent in the statement** before any "payment settled" wording, checked against the Core intent rules; "future" is not acceptable |
| P3-8 to P3-12 | proposed | AGREE | AGREE | AGREE (P3-8, P3-9 and P3-10 with additive clauses) |
| P3-13 | — | — | — | PROPOSE (1) |

- **The compiler review's P3-7 AGREE is not acceptance of P3-7.** It was cast on V5's external-account wording and does not cite `f3ae8556`. By PROCESS timestamps the two reviews overlapped: V6 ran 12:19:38–13:01:19Z and the compiler review 12:46:42–13:21:04Z. V6's mandatory all-account authority and signed-intent requirement stands and is **not** overruled by counting votes on the older wording.
- This successor removes the one place where its source implied the external-only reading. `adapter::TARGET_OBLIGATIONS` now records P3-7 as disputed and cites V6. COVERAGE.md says the same.
- The candidate implements **no** debit authority, signed intent, physical delta or conservation. No P3 item is implemented or claimed here.
- Integrated reconciliation of P3 (and of P4-1/P4-3, where V6 also adds clauses) is **deferred to the source owner**. I record no merged text.
