# Security currentness — 2026-10-14

Owner: Opus (all security judgments). Document baseline `fc6353bf` (same merged tree as `11a7d19f`); security source observations are scoped to `cb8280ef`. This page is a reading guide for the historical security families. It is not a security audit, an approval, native qualification or a milestone exit.

- No command, test, SDK call, native step or target query was executed for this page. No U, M, MC, SP or G exit closes because of it.
- Coverage: the Opus currentness pass inspected 34 authored wiki paths plus 8 exact-claim locations, and all 64 OpenSpec handoff entries. It was not an exhaustive audit of all prose, fenced contracts, private material or raw captures.
- Native authority is zero.
- Root-supplied status: the old campaign is still blocked and there are no pending transactions. This review did not query that status itself.

Findings are in the [Opus security review](../docs/audits/2026-10-14/SECURITY-REVIEW.md). The requirement addendum is [OSR-1014](../openspec/changes/consolidated-language-kernel/security-reconciliation-2026-10-14.md).

## Current security qualification by profile

| Profile / consumer | Security mechanism | Strongest current evidence | Not established |
|---|---|---|---|
| Rust Core `moriarty/1` / `moriarty-core/1` `verify-intent` | Owner-debit checks only; the signer label is not bound to a key | source-inspected at `cb8280ef` | Authorization of third-party debits; a source reading indicates they are reported `accepted` (SEC-07, not executed) |
| Ledger signed-v2 (`compile-call --authorization signed-v2`) | In-circuit JubJub Schnorr under a compiled key; private `at`; Impact nonce map | source-inspected at `cb8280ef`; historical-captured PR47/PR49 source-static results (PR49 candidate `001bfac7` shares the `cb8280ef` tree) | Any proof or verification; time binding (SEC-01); account authority (SEC-04); token identity (SEC-09) |
| Backend `mori-backend` signed relation | Public key, message, nonce and `at` | historical-local native, one case at `ee963f61` | Transfer to signed-v2 (SEC-02) |
| Unsigned / `unsigned-mapped-roles` | None | source-inspected non-promotion for two consumers | Authorization or custody of any kind (SEC-08, SEC-20) |
| Beta / Source6 / TS Core5 signed-intent /1 | secp256k1 BIP340 / ECDSA over a closed JSON frame; local stipulated registry | historical-local, reviewed scoped stage `fd9725e0` | Key authority, authenticated head, native or ledger acceptance; the top-level status remains `SignedPreparedUnqualified` (DEVEX-RESULT) |
| Fixed native financial kernel | Manually specialized kernel with trusted genesis | historical-local native (R4/R5) | Generic correspondence, authenticated funding and history, Preview settlement |
| Targets | — | Preview: historical scope only. Fenrir: identity unknown. Local: `local-unqualified` | Any transfer between targets |

## Historic families: original tree, scope and present implication

| Family | Original tree and scope | Review identities (historical, preserved) | Implication today |
|---|---|---|---|
| [midnight-crypto-2026-09-30](midnight-crypto-2026-09-30/README.md) | Candidate `7e7f3861`, 74 files, merged PR12 `db23fe02`. Harness for secp256k1 primitives, Merkle state, a k6 proof under an unsafe test SRS, privileged local ledger | Astra medium; Grok 4.6 high | Primitive-level evidence only. Not M8, not signed-v2. PLAN.md's unchecked items are completed in DELIVERY.md |
| [signed-intent-2026-10-01](signed-intent-2026-10-01/README.md) | Base `2905cb6d`, reviewed candidate `fd9725e0`, 252 files. Closed /1 codec, Beta consumer, LocalSettlementStore, k10 numerical proof | Sol, Sonnet 5.5 seats; Astra and Grok 4.6 final audits | A different protocol from signed-v2 (OSR-1014-15). The registry is a stipulation. The k10 proof excludes identities, signatures and replay. "Pending" labels are superseded by DELIVERY.md |
| [native-financial-kernel-2026-10-01](native-financial-kernel-2026-10-01/README.md) | After PR13 `033e9a90`. V4–V8, R3 refusals, R4/R5 success, freeze `32305c38…` | Astra, Grok 4.6; R4/R5: GPT-6.1 Sol high, Claude Opus 5.5 high, Grok 4.7 xhigh; Opus CPU dissent and the 2–1 vote kept | A fixed trusted fixture. ECDSA owner ≠ Schnorr NIGHT payer. No generic compiler, PCD or Preview claim. "Native goal active" and "currently live" are as of Oct 1 |

## Statements to read as historical

| Location | Statement | Current reading |
|---|---|---|
| `signed-intent-2026-10-01/README.md:6,23-24,26` (after the currentness note) | "Active", "whole-candidate audits are pending", "native goal is armed" | DELIVERY.md records both approvals of `fd9725e0`. No live goal is asserted here |
| `signed-intent-2026-10-01/RESULT.md:3`, `SECURITY-CASES.md:67-68`, `NATIVE-RESULT.md:26` | Audits pending | Superseded by DELIVERY.md. The files themselves stay frozen |
| `midnight-crypto-2026-09-30/DESIGN.md:27`, `signed-intent-2026-10-01/PLAN.md:27-28` | Astra/Grok routing | Historical. Current routing is stated in `AGENTS.md` (October 14 section) and the develop skill; this page does not restate it. The older Grok and Astra rosters are not restored |
| `AGENTS.md:100` at `cb8280ef` | "Preview is the sole public execution target" | Resolved in the current candidate: `AGENTS.md:107` gives Preview and required Fenrir devnet qualification separate target evidence. SECURITY-REVIEW §5 item 12 describes the `cb8280ef` source |
| `native-financial-kernel-2026-10-01/README.md:25` (after the currentness note), `V4-ACTUAL-RESULT.md:9` | Preview ledger 8.1.2 incompatibility | An observation as of Oct 1. Preview's current version was not re-observed |
| `ALIGNMENT-REVIEW-2026-10-01.md:53` (historical original-text section; `:17` at `fc6353bf`) | Strict WF refused `OutputsNotSorted`; independent verification and full Success unproved | Superseded for the fixed trusted fixture only: R3 (sorted envelope, WF passed, `NightBalance` failure), then R4/R5 (Success, independent verification, three approvals). Open for generic compiler, signed-v2, authenticated funding, Preview and Fenrir |
| `ALIGNMENT-REVIEW-2026-10-01.md:71` (historical original-text section; `:35` at `fc6353bf`) | Repair offer ordering before binding/proving | Historical step consumed by the R3 sorted attempt; not a current queue. Native authority is zero |
| `native-financial-kernel-2026-10-01/LEDGER-PREPARATION-PROGRESS.md:7` | Caller source "frozen and uncompiled" | Later compiled and run in V4/V5 (keyless preparation; all eight CallContext checks). Its statement that a trusted local genesis cannot close authenticity obligations still holds |
| `native-financial-kernel-2026-10-01/V4-HANDOFF-SOURCE.md:9` | "Goal active"; financial proof open | As of Oct 1. The fixed-fixture proof later succeeded (R4/R5); authenticated genesis/custody/time, generic correspondence, PCD and Preview remain open |
| `native-financial-kernel-2026-10-01/V8-03-ACTUAL-REFUSAL.md:19` | No retry authorized by this note | Accurate for that note. Later R3 and R4/R5 attempts are separate records with their own reviews; nothing here grants authority today |
| `beta-language-2026-09-30/CHECKLIST.md:70` | Unchecked: canonical intent signatures and authenticated snapshot/head | Keep unchecked. Signed-intent /1 later supplied a closed secp256k1 codec for the Node beta path; an authenticated snapshot/head is still not established. Ledger signed-v2 is a separate path |
| `beta-language-2026-09-30/domains/01-surface.md:184` | Beta signing needs a codec over terms, signed policy, completed values and claim roots | Still open as written; the /1 protocol does not cover signed policy or claim roots |
| `mil4-s0-executable-boundary-2026-09-30.md:17` | A consumer must rederive effects from authenticated state; a caller-supplied hash cannot discharge the premises | Still valid; no positive authenticated consumer has been observed. Consistent with OSR-1014-03 and -05 |

## Next security work

- **Source-only:** WS-A A1–A4 (consumer inventory, two-stack field map, Schnorr argument, premise ledger).
- **Safe-static, future and root-authorized:** WS-B B1–B7.
- **Blocked-SDK-native:** WS-C C1–C5.
- **Blocked-external:** WS-D.
- **Requirements:** OSR-1014-01…15. Planned outputs go to the audit dossier's `security/` directory, as described in the [audit plan](../docs/audits/2026-10-14/AUDIT-PLAN.md).
