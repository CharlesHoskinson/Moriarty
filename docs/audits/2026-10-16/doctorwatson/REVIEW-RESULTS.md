# DoctorWatson candidate and result reviews

Both independent reviews approve candidate tree `74bc02967de655362555eed06d46bf376a6ebda6` **with scope**. The [candidate manifest](reviews/CANDIDATE.json) binds the reviewed active source, documentation and observed results. [Muse](reviews/muse-REPORT.md) assessed nonsecurity code/design/results; [Opus](reviews/opus-REPORT.md) assessed security and quality. Both reviewed supplied bytes and actual captured results without rerunning tools. Their scope excludes Sherlock execution, correspondence, native/target behavior and certification of the advisory archives.

[Opus identity](reviews/opus-IDENTITY.json) includes provider-returned canonical `claude-opus-5-5`, terminal success and requested high effort. [Muse identity](reviews/muse-IDENTITY.json) records configured `muse-wonder-1.0` through provider `meta`, requested high effort and terminal completion; its CLI exposed no separate provider-returned canonical modelUsage. Muse's original report links carry its `/tmp` reviewer working-directory prefix; resolve those source paths from the repository root. Original report text is preserved.

The predecessor reports remain under `reviews/predecessor-*-REPORT.md`. They describe tree `7c81ebc725dd580be55272119c825f3fe7ccfe3f`, not this candidate. Their findings motivated terminal-history preservation, mandatory directed completion and Opus-authored checker corrections. Current results are [design-04](quint/design-04.json), with 33 Python tests, 30 main Quint scenarios, one directed-completion test and separate sampled/directed observations. Seven default zero-hit warnings remain.

## Dispositions and remaining work

No blocking finding was returned. The complete original findings remain in the reports, including these limits:

- Opus N1: upstream/shared-input edits can clear **active** tasks' administrative stop counters in the model. Terminal history is preserved. Strengthening the active-task SF-14 projection remains follow-up work; the model is not an enforcement engine for real agents.
- Opus N2: the referenced 23-series and 26-series receipts and the isolated disabled-step mutant source are now included as evidence-only additions. They retain their original hashes and outcomes. The model README's current source-of-truth is design-04; `directed-result.json` remains a predecessor index. `QUINT-REVIEW.md`'s five-zero finding describes the predecessor; the current result has seven. The ordinary hardening GREEN receipt binds the QA README before its later results-only paragraph; its checker/test hashes match design-04.
- Opus N3–N6: caller environment/import trust, dedicated verified-byte compilation tests, malformed diagnostic overlays and existing manual-site supply-chain pins remain scoped security follow-ups owned by Opus. No new security conclusion or permission follows from source approval.
- Opus N7: the script's 256 MiB setting limits Node **old space**, not the full heap or process. The model README's shorter “Node heap” wording must be read with this qualification and the explicit CI contract.
- Opus N8 and Muse warnings: concrete ownership/receipt assumptions, cancellation-dominated sampling, trusted imported observations, weak eventual-progress coverage and tight per-command timing remain explicit limits. Passing tests and sampled traces do not prove implementation refinement.

This receipt, original reports, identities and additional captured artifacts were added after the reviewed candidate as nonexecuting evidence. They change no reviewed executable or input. The reviewed active source remains bound by the manifest; these receipt additions do not claim to have been included in the original audit tree.

DoctorWatson's foundation can support source implementation, but its initial checker checks frozen data and committed source literals. Connecting actual Sherlock execution and the required K/Quint/Lean-Clean/circuit observations is the next implementation slice. All full Moriarty roadmap exits, native authority 0, C4 and Sherlock's own Midnight stagenet requirement remain unchanged.
