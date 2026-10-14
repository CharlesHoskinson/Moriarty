# Security currentness — 2026-10-14

Owner: Opus (all security judgments). Baseline `fc6353bf` (same merged tree as `11a7d19f`). This page is a reading guide for the historical security families. It is not a security audit, an approval, native qualification or a milestone exit.

- Native authority is zero.
- Root-supplied status: the old campaign is still blocked and there are no pending transactions. This review did not query that status itself.

Findings are in the [Opus security review](../docs/audits/2026-10-14/SECURITY-REVIEW.md). The requirement addendum is [OSR-1014](../openspec/changes/consolidated-language-kernel/security-reconciliation-2026-10-14.md).

## Current security qualification by profile

| Profile / consumer | Security mechanism | Strongest current evidence | Not established |
|---|---|---|---|
| Rust Core `moriarty/1` / `moriarty-core/1` `verify-intent` | Owner-debit checks only; the signer label is not bound to a key | source-inspected | Third-party debits are reported `accepted` (SEC-07) |
| Ledger signed-v2 (`compile-call --authorization signed-v2`) | In-circuit JubJub Schnorr under a compiled key; private `at`; Impact nonce map | source-inspected; PR47/49 source-static capture at the same tree | Any current proof or verification; time binding (SEC-01); account authority (SEC-04); token identity (SEC-09) |
| Backend `mori-backend` signed relation | Public key, message, nonce and `at` | historical-local native, one case at `ee963f61` | Transfer to signed-v2 (SEC-02) |
| Unsigned / `unsigned-mapped-roles` | None | source-inspected non-promotion for two consumers | Authorization or custody of any kind (SEC-08, SEC-20) |
| Beta / Source6 / TS Core5 signed-intent /1 | secp256k1 BIP340 / ECDSA over a closed JSON frame; local stipulated registry | historical-local, reviewed scoped stage `fd9725e0` | Key authority, authenticated head, native or ledger acceptance; still `PreparedUnqualified` |
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
| `signed-intent-2026-10-01/README.md:3,20-21,23` | "Active", "audits pending", "native goal is armed" | DELIVERY.md records both approvals of `fd9725e0`. No live goal is asserted here |
| `signed-intent-2026-10-01/RESULT.md:3`, `SECURITY-CASES.md:67-68`, `NATIVE-RESULT.md:26` | Audits pending | Superseded by DELIVERY.md. The files themselves stay frozen |
| `midnight-crypto-2026-09-30/DESIGN.md:27`, `signed-intent-2026-10-01/PLAN.md:27-28` | Astra/Grok routing | Historical. Current routing: GPT-6.1 Sol high implementation, Muse Wonder nonsecurity review, Opus 5.5 security. The older Grok and Astra rosters are not restored |
| `native-financial-kernel-2026-10-01/README.md:22`, `V4-ACTUAL-RESULT.md:9` | Preview ledger 8.1.2 incompatibility | An observation as of Oct 1. Preview's current version was not re-observed |
| `AGENTS.md:100` | "Preview is the sole public execution target" | Stale (SECURITY-REVIEW §5 item 12). Owned by the nonsecurity authors |

## Next security work

- **Source-only:** WS-A A1–A4 (consumer inventory, two-stack field map, Schnorr argument, premise ledger).
- **Safe-static, future and root-authorized:** WS-B B1–B7.
- **Blocked-SDK-native:** WS-C C1–C5.
- **Blocked-external:** WS-D.
- **Requirements:** OSR-1014-01…15. Planned outputs go to the audit dossier's `security/` directory, as described in the [audit plan](../docs/audits/2026-10-14/AUDIT-PLAN.md).
