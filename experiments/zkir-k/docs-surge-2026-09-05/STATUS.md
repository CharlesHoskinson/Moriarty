# Surge ledger

Every chapter: drafted by one vendor, audited by two other vendors (developer and formal-methods roles), fixed by a Claude editor, re-audited where a blocking finding existed or the semantics changed underneath it, then edited by the orchestrator for cross-chapter consistency (receipts 2026-09-05c, 43 unit checks, corrected rules).

| ch | drafter | dev audit | fm audit | fix rounds | notes |
|---|---|---|---|---|---|
| 01 | claude | grok | codex | 2 | second codex round applied; orchestrator read in full |
| 02 | grok | claude | codex | 1 + codex round 2 (applied by orchestrator) | |
| 03 | codex | grok | claude (ACCEPT) | 1 | |
| 04 | claude | grok | codex | 1 | found K6 and the limb-layout page error |
| 05 | grok | claude | codex | 1 | totality settled by execution |
| 06 | codex | grok | claude | 1 | |
| 07 | claude | grok | codex | 1 | reordered to syntax order |
| 08 | grok | claude | codex | 1 | orchestrator read the trust section |
| 09 | codex | grok | claude | 1 + orchestrator patch (sha512 gate) | |
| 10 | claude | grok | codex | 1 | found check --ext gap |
| 11 | grok | claude | claude (codex stalled) | 2 | |
| 12 | codex | grok | claude | 1 | |
| 13 | claude | grok | codex | 1 | K6 section |
| 14 | grok | claude | claude (codex stalled) | 1 | found #selBit / #eqSupported overlaps |
| 15 | codex | grok | claude | 1 | wiki definition-page correction |

## Prose pass

Every chapter: Gottlieb edit (findings in prose/NN-gottlieb.md), Le Guin revision with the inkwell displacement check (no habit rising), humanizer file-mode loop, author self-diff, then an independent preservation check (prose/NN-verify.md). All fifteen verdicts PRESERVED; two minor precision losses in chapter 10 restored by the orchestrator. Whole-set displacement: semicolons -29%, expletive openers -77%, very short sentences -23%, nothing rose.
