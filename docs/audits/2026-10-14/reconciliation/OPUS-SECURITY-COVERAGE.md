# Moriarty security currentness and requirements audit, pass 2 (Read/Grep/Glob only, no execution)

Nothing was edited. Plan mode was on, and I didn't have a Write tool or ExitPlanMode in this session, so this report is the whole deliverable. Nothing was executed: no shell, tests, SDK, native work, keygen, prove/verify, ledger or target query. I read nothing private (keys, SRS, caches) and no frozen provider proposals. This is not approval of my own earlier security text. Native authority is still zero, no milestone closes, and every SEC observation is still a source reading at `cb8280ef`.

## 1. Coverage summary

- **Wiki handoff:** All 34 authored paths and all 8 exact-claim locations were inspected. 21 files were read in full in the current candidate. The other 13 were read in full in pass 1 at the baseline, and this pass re-checked only their cited lines.
- **OpenSpec handoff:** The JSON lists **64** file entries, not 57. My earlier "57" was a miscount. Every handed-off heading was read together with its surrounding clauses in the current candidate.
- **Changed files:** 13 OpenSpec files have `after_sha ≠ before_sha` because of nonsecurity insertions. In every location I read, the original predicates, scenarios, IDs, frozen budgets and reviewer identities look intact. That is a reading, not a byte comparison against `fc6353bf`. The "unchanged" status of the other 51 files rests on the root-supplied hashes; I did not verify it.
- **Were all handed-off public requirements inspected?** Yes, every handed-off heading or path. No, not all surrounding non-handed-off prose. The files and parts listed in §5 remain unknown.

## 2. Wiki per-file disposition

Method key: **F** = read in full in the current candidate. **P1** = read in full in pass 1; this pass re-checked only the cited lines.

| Path (wiki-llm/…) | Method | Disposition |
|---|---|---|
| midnight-crypto/README | F | Opus note OK (PR12, `7e7f3861`, unsafe k6 SRS) |
| midnight-crypto/{DELIVERY,DESIGN,PLAN,RESULT} | P1 + grep | Historical. 74 files, `db23fe02`, k=6 unsafe SRS confirmed. Astra/Grok roster frozen |
| native/README | F | Opus note OK. The routing "Preview ledger8.1.2" row moved to line 25 (§4 B5) |
| native/{BETA-DISPLAY-FOLLOWUP, LEDGER-BUILD-DIAGNOSTIC, LEDGER-PREPARATION-PROGRESS, NEXT-PROOF-SOURCE-PREPARATION, V4-HANDOFF-SOURCE, V4-ACTUAL-RESULT, V5, V6, V7, V8-03, R3-04, REVIEWED-KEYGEN-MILESTONE, R4-R5} | F (all) | Frozen and historical as of Oct 1–2. They agree with the currentness page: ECDSA owner ≠ Schnorr NIGHT payer (V5); freeze `32305c38…`; three R4/R5 approvals; Opus CPU dissent and the 2–1 vote kept; SRS ceremony unaudited (V7). No edits |
| signed-intent/README | F | Opus note OK. The original "Active", "pending" and "armed" lines moved to 6, 23–24 and 26 (§4 B5) |
| signed-intent/{DELIVERY, DEVEX-RESULT, REPAIR-PROCESS, REPAIR-TYPES, NATIVE-RESULT} | F | DELIVERY confirms both approvals of `fd9725e0`. DEVEX: keys have no account authority, and `preview` is a claimed network, not an observation. The actual status is `SignedPreparedUnqualified` (§4 B4). NATIVE-RESULT: k10 proof, ephemeral unsafe SRS, proofs0.8 not established against ledger proofs0.7.3 (§4 A4) |
| signed-intent/{LOCAL-SETTLEMENT, NATIVE-TRANSFER-PROPOSAL, PLAN, PREVIEW-ROUTE, PROTOCOL, REPAIR-AUDIT, REPAIR-CRYPTO, RESULT, SECURITY-CASES} | P1 + cited lines | Historical. RESULT:3, SECURITY-CASES:67–68 and PLAN:27 confirmed. A grep of PROTOCOL finds no "policy" or "claim root" |
| **Exact claims** | | |
| ALIGNMENT-REVIEW:54 | F | Superseded for the fixed fixture only, by R3 and then R4/R5. Still open for generic, signed-v2, Preview and Fenrir |
| ALIGNMENT-REVIEW:72 | F | Historical step, consumed by the R3 sorted attempt. Not a queue |
| LEDGER-PREPARATION-PROGRESS:7 | F | Later compiled and run (V4/V5). Its trusted-genesis caveat still holds |
| V4-HANDOFF-SOURCE:9 | F | "Goal active" is as of Oct 1. The authenticity, PCD and Preview items are still open |
| V8-03:19 | F | Accurate for that note. R3 and R4/R5 have their own records |
| beta CHECKLIST:70 | F (:1–78) | Keep unchecked. The authenticated snapshot/head is still not established |
| beta 01-surface:184 | F (:170–194) | Still open: /1 has no signed policy or claim roots (grep only) |
| mil4-s0:17 | F | Still valid. No positive authenticated consumer has been observed |

## 3. OpenSpec per-file disposition (64 entries)

**Common to every row:**
- Original predicates, scenarios and IDs are preserved as read.
- Frozen rosters (Fable 5.1 / GPT-6, Astra / Grok) stay historical; current routing comes from `AGENTS.md` lines 3–13.
- "Verification commands", "Verification entry points" and "Tasks" sections are maintainer delivery. They add no product security clause, and their native or Preview items are blocked-SDK-native or blocked-external.
- **Δ** marks a file with nonsecurity insertions.

| File | Method | Security note |
|---|---|---|
| MORIARTY-COMPLETION-PROGRAM | :276–323 | Frozen budgets kept: k≤17, 8 GiB, 2 jobs, 10 min preflight, 256 MiB, 8 h; MC04 20 min / ≤6, MC05 20 / ≤8, MC06 20 / ≤8, MC07 120 / ≤349; 277 + 72 mandatory |
| REPORT-RECONCILIATION | :30–109 | RP01 signing display, revoked keys and verify budgets are already covered by MC05/MC08 scenarios |
| sprint-program spec | F | "Plan mistaken for execution authority" agrees with zero authority |
| bounded tasks | F | Plan checks only |
| consolidated design Δ | F | Routing only; line 7 says no security design is selected. The referenced design and backend documents were not inspected |
| consolidated spec Δ | F | UNI-001–017 read intact. UNI-018–024 are new nonsecurity clauses; UNI-022 agrees with OSR-06. The addendum's ID list needs §4 A2 |
| KSAS design, impl-plan, .pel, proposal, tasks, workflow | F (all) | OWS, threshold and TEE premises kept. Pel validates structure only |
| KSAS requirements + spec | F | MPLR-019–034 intact. OSR-10, -03 and -13 extend them. MPLR-035 exists only in traceability |
| L2L design Δ | F | Line 5 routes security to Opus. "Docker stopped" and "native goal is the loop" are design-baseline history |
| L2L loan contract | :170–250 | Local cap is untrusted. Task 5 consent supports OSR-10 / SEC-11 |
| lifecycle-k-agreement, lifecycle-ledger-integration, loan-origination specs | F | Local timestamps are not authority (supports OSR-01). Preview only; Fenrir is separate under UNI-022 |
| L2L tasks | F | §5 still open |
| MC01 spec | F | Intact |
| MC01 tasks | :35–91 | Commands are marked not passing |
| MC02 tasks | :30–90 | Preview submissions are blocked-external |
| MC03 README Δ, design, proposal, spec, tasks | F | Δ insertion is consistent. "Real recursion…" stays cross-linked (SEC-15) |
| MC04 design, proposal, spec, tasks | F | Durable consumption, permissionless correspondence and named correspondence are intact. Consumption identity is domain/principal/nonce (relevant to OSR-06/-07) |
| MC05 README Δ, design, proposal, spec, tasks | F | Bound exact-plan, authority-covers-liabilities, revoked-verifier and oversized-input scenarios intact. Optional §4 E2 |
| MC06 README Δ, design, proposal, spec, tasks | F | Intact |
| MC07 design / spec / tasks | :25–49 / F / :38–82 | 277 + 72 denominators and the 349 cap preserved |
| MC08 spec / tasks | F / :30–69 | "Misleading proof status" intact |
| partial-conditional spec / tasks | :55–201 / F | MPLR-006–018 intact |
| permissionless impl-plan / spec | :28–77 / :20–51 | MOR-003/004 intact. P2 names Preview only |
| sprints README Δ | F | Line 119 routing is current |
| SP01 execution / sp01 | :38–76 / :55–114 | "Goal observed active" is Sept 7 and self-qualified |
| sp02 Δ, sp05 Δ, sp06, sp08 Δ | F | sp05 now separates M3 Fenrir from MC02 Preview |
| sp04 / sp07 Δ / sp09 / sp10 | :1–75 / :58–95 / :36–110 / :1–80 | SP09.3 signature domains and expiry are extended by OSR-01/-06 |
| sp11 Δ / sp12 Δ | :30–88 / :30–96 | sp12 "explicit stubs" is ambiguous given SEC-19 (§4 E1) |

## 4. Exact replacement snippets

### A. `openspec/changes/consolidated-language-kernel/security-reconciliation-2026-10-14.md`

**A1. Line 3**, replace with:
```
**Status.** This is a specified-only addendum by the Opus security owner. It adds no implementation, evidence, approval or acceptance. It does not amend, renumber or replace any existing clause. No command, test, SDK call, native step or target action was executed for it, and no U, M, MC, SP or G exit closes because of it. The SEC observations it cites are source readings at `cb8280ef`, not executed results.
```

**A2. Line 10**, replace with:
```
- All original predicates, IDs and positive/hostile controls stay in force: U0–U7, M1–M17, MC01–MC08, SP01–SP12, G01–G24, L1–L8, ZR01–ZR16, MNR01–MNR08, MPLR-001–035, UNI-001–017, MOR-001–012 and AEO-001–004. The later nonsecurity UNI-018–024 clauses are outside this addendum, which neither modifies nor restates them; UNI-022 target separation is consistent with OSR-1014-06.
```

**A3. Lines 62, 113, 159, 171 and 225**, replace each line in turn:
```
- Evidence: `current-local` safe-static (WS-B B2). A source reading at `cb8280ef` indicates the request is accepted; this has not been executed.
```
```
- Evidence: `current-local` safe-static (WS-B B3). A source reading at `cb8280ef` indicates the output is `accepted`; this has not been executed, and changing that output is deferred.
```
```
- Current state: closed by source reading at `cb8280ef` for the two consumers inspected; completeness is a hypothesis.
```
```
- Evidence: `current-local` safe-static (WS-B B4). A source reading at `cb8280ef` indicates this is accepted; this has not been executed.
```
```
- Evidence: `current-local` (WS-B B5). A source reading at `cb8280ef` indicates the release succeeds (DL); this has not been executed.
```

**A4. Line 245**, replace with:
```
  - the k6 or k10 numerical proofs (ephemeral unsafe test SRS; the k10 proof's proofs 0.8/curves 0.3 stack is not established as compatible with the ledger proofs 0.7.3 verifier);
```

### B. `wiki-llm/SECURITY-CURRENTNESS-2026-10-14.md`

**B1. Lines 3–6**, replace with:
```
Owner: Opus (all security judgments). Document baseline `fc6353bf` (same merged tree as `11a7d19f`); security source observations are scoped to `cb8280ef`. This page is a reading guide for the historical security families. It is not a security audit, an approval, native qualification or a milestone exit.

- No command, test, SDK call, native step or target query was executed for this page. No U, M, MC, SP or G exit closes because of it.
- Native authority is zero.
- Root-supplied status: the old campaign is still blocked and there are no pending transactions. This review did not query that status itself.
```

**B2. Line 14**, replace with:
```
| Rust Core `moriarty/1` / `moriarty-core/1` `verify-intent` | Owner-debit checks only; the signer label is not bound to a key | source-inspected at `cb8280ef` | Authorization of third-party debits; a source reading indicates they are reported `accepted` (SEC-07, not executed) |
```

**B3. Line 15**, replace with:
```
| Ledger signed-v2 (`compile-call --authorization signed-v2`) | In-circuit JubJub Schnorr under a compiled key; private `at`; Impact nonce map | source-inspected at `cb8280ef`; historical-captured PR47/PR49 source-static results (PR49 candidate `001bfac7` shares the `cb8280ef` tree) | Any proof or verification; time binding (SEC-01); account authority (SEC-04); token identity (SEC-09) |
```

**B4. Line 18**, last cell becomes:
```
| Key authority, authenticated head, native or ledger acceptance; the top-level status remains `SignedPreparedUnqualified` (DEVEX-RESULT) |
```

**B5. Lines 34, 36 and 38**, replace each row in turn; the AGENTS row and the native-kernel row are separate rows:
```
| `signed-intent-2026-10-01/README.md:6,23-24,26` (after the currentness note) | "Active", "whole-candidate audits are pending", "native goal is armed" | DELIVERY.md records both approvals of `fd9725e0`. No live goal is asserted here |
```
```
| `midnight-crypto-2026-09-30/DESIGN.md:27`, `signed-intent-2026-10-01/PLAN.md:27-28` | Astra/Grok routing | Historical. Current routing is stated in `AGENTS.md` (October 14 section) and the develop skill; this page does not restate it. The older Grok and Astra rosters are not restored |
```
```
| `AGENTS.md:100` at `cb8280ef` | "Preview is the sole public execution target" | Resolved in the current candidate: `AGENTS.md:106` gives Preview and required Fenrir devnet qualification separate target evidence. SECURITY-REVIEW §5 item 12 describes the `cb8280ef` source |
| `native-financial-kernel-2026-10-01/README.md:25` (after the currentness note), `V4-ACTUAL-RESULT.md:9` | Preview ledger 8.1.2 incompatibility | An observation as of Oct 1. Preview's current version was not re-observed |
```

**B6. Append to the "Statements to read as historical" table**, after the B5 rows:
```
| `ALIGNMENT-REVIEW-2026-10-01.md:54` | Strict WF refused `OutputsNotSorted`; independent verification and full Success unproved | Superseded for the fixed trusted fixture only: R3 (sorted envelope, WF passed, `NightBalance` failure), then R4/R5 (Success, independent verification, three approvals). Open for generic compiler, signed-v2, authenticated funding, Preview and Fenrir |
| `ALIGNMENT-REVIEW-2026-10-01.md:72` | Repair offer ordering before binding/proving | Historical step consumed by the R3 sorted attempt; not a current queue. Native authority is zero |
| `native-financial-kernel-2026-10-01/LEDGER-PREPARATION-PROGRESS.md:7` | Caller source "frozen and uncompiled" | Later compiled and run in V4/V5 (keyless preparation; all eight CallContext checks). Its statement that a trusted local genesis cannot close authenticity obligations still holds |
| `native-financial-kernel-2026-10-01/V4-HANDOFF-SOURCE.md:9` | "Goal active"; financial proof open | As of Oct 1. The fixed-fixture proof later succeeded (R4/R5); authenticated genesis/custody/time, generic correspondence, PCD and Preview remain open |
| `native-financial-kernel-2026-10-01/V8-03-ACTUAL-REFUSAL.md:19` | No retry authorized by this note | Accurate for that note. Later R3 and R4/R5 attempts are separate records with their own reviews; nothing here grants authority today |
| `beta-language-2026-09-30/CHECKLIST.md:70` | Unchecked: canonical intent signatures and authenticated snapshot/head | Keep unchecked. Signed-intent /1 later supplied a closed secp256k1 codec for the Node beta path; an authenticated snapshot/head is still not established. Ledger signed-v2 is a separate path |
| `beta-language-2026-09-30/domains/01-surface.md:184` | Beta signing needs a codec over terms, signed policy, completed values and claim roots | Still open as written; the /1 protocol does not cover signed policy or claim roots |
| `mil4-s0-executable-boundary-2026-09-30.md:17` | A consumer must rederive effects from authenticated state; a caller-supplied hash cannot discharge the premises | Still valid; no positive authenticated consumer has been observed. Consistent with OSR-1014-03 and -05 |
```

### C. Family entrypoints

- The three README insertions are accurate as applied, and I have no security correction to them.
- Minor nonsecurity formatting: there is an extra blank line in `native-financial-kernel-2026-10-01/README.md:10-11` and in `signed-intent-2026-10-01/README.md:4-5`.

### D. Corrections to my earlier runtime report (`OPUS-SECURITY-REQUIREMENTS.md`)

**D1.** Replace the bullet "All 57 files and all handoff locations are preserved byte-for-byte." with:
```
- The OpenSpec handoff lists 64 file entries (the earlier "57" was a miscount). Thirteen received nonsecurity context insertions in the current candidate. In every handed-off location read there, the original predicates, scenarios, IDs, budgets and reviewer identities appear unchanged; this is a reading, not a byte comparison against `fc6353bf`.
```

**D2.** Replace the three "Handoffs to nonsecurity authors" bullets with:
```
- Resolved in the current candidate: `AGENTS.md:3-13` and `SKILL.md:27` route all security work to Opus 5.5; `AGENTS.md:106` separates Preview and Fenrir evidence; `wiki-llm/README.md:4` links the currentness page.
```

### E. Optional suggestions for nonsecurity owners

**E1.** In `openspec/sprints/sp12-…md:89`, replace "Rust LSP and MCP remain explicit stubs" with:
```
Rust LSP and MCP remain stubs that panic through `todo!` (source reading at `cb8280ef`, SEC-19; OSR-1014-14 requires an explicit unsupported status)
```

**E2.** At the end of `mc05-…/README.md:13`, append:
```
 Opus source observations SEC-07 and SEC-18 apply (OSR-1014-05, -13).
```

**E3.** In `wiki-llm/ALIGNMENT-REVIEW-2026-10-01.md:31-33`, replace "the historical security passages below are preserved verbatim, without a new security verdict" with:
```
Opus dispositions of the historical security passages below are in the [security currentness guide](SECURITY-CURRENTNESS-2026-10-14.md#statements-to-read-as-historical); the passages themselves are preserved verbatim
```

## 5. Remaining gaps (unknown, not cleared)

- **Not inspected:**
  - frozen provider proposals `signed-intent/plans/*`
  - all evidence and audit subdirectories
  - `docs/MORIARTY-CONSOLIDATED-DESIGN.md` and `MORIARTY-BACKEND-REQUIREMENTS.md`, which the consolidated design routes its security content to
  - `wiki/research/mplr/*`
  - `raw/assignments/*`
  - native-owner-intent `RESULT.md`
  - the horizon's fenced contracts
- **Prose outside the handed-off sections, not read:**
  - partial-conditional MPLR-001–005
  - MOR-001/002 and MOR-005–012
  - completion program, all sections except :276–323
  - report reconciliation :1–29 and :110–end
  - the leading lines of sp01, sp04, sp07, sp09, sp11 and sp12
  - MC01/MC02/MC07/MC08 task headers
  - L2L contract :1–169
- **Not re-read in this pass:** the 13 wiki files marked P1. Their current-candidate bytes rest on wiki-audit.md's preservation claim.
- **Byte identity:** not verified for any file.
- **Hypotheses still unresolved:** SEC-03, SEC-08 completeness, SEC-17, SEC-18 double release.
- **Fenrir:** identity is still unknown.
