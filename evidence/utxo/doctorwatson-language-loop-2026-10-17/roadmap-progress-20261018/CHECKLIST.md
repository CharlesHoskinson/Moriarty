# Language roadmap delta — 2026-10-18

PR202 and PR203 add merged finite local observations. PR204 adds only the financial report design contract. PR205 has a passing captured result; its first ordinary result audit was materially rejected, replacement review is pending, and it remains unmerged. All 69 full U/M/MC/SP/G exits remain open; their exact primary exit texts match the prior audit.

| PR | Exact scope | Current disposition |
|---|---|---|
| [202](https://github.com/CharlesHoskinson/Moriarty/pull/202) | Counter ordinary/cap16/max and empty-action ordinary/cap0: 5 actual CLI calls inferred from 2 passing tests; complete JSON, stdout/stderr bytes and exit; 53.784611432s | Merged `dac00001652b5c4a6297de1d3a8747f743432dfa`; finite NR-01–06 support only |
| [203](https://github.com/CharlesHoskinson/Moriarty/pull/203) | 2 full Program parser comparisons, including all 11 Counter nodes and the empty-frame uninvoked declaration; 25.556119662s | Merged `5fe213a922fca4e7a861cb4f106ef1cedb8e3b50`; `parse → to_value → Value equality`, no raw Program-byte claim |
| [204](https://github.com/CharlesHoskinson/Moriarty/pull/204) | Adopted Transfer/Fee/Liability/Discharge observations, deltas and evidence boundary | Merged `401c386bcd750f6ca3ac91f038d907dc33111f66`; source only |
| [205](https://github.com/CharlesHoskinson/Moriarty/pull/205) | Candidate `7e0d753c40a3e5f694aa226bd657c831da111f65`, tree `55fab2c3cbd818ffb04eeb7d5930a28758a858b3`: 2 tests, 2 complete Programs, 6 actual CLI reports; 52.624672563s | Actual result retained; original ordinary audit rejected, replacement pending; unmerged |

The direct child map is NR-01–03 and the financial observation paragraph under UNI-018; NR-04–05 and the financial delta paragraph under UNI-020; both evidence-boundary paragraphs under UNI-021; NR-06 under UNI-024. [DELTA.json](DELTA.json) retains exact clause text and all nine original parent scenario names. None of those original parent scenarios is marked wholly observed.

The financial capture covers three distinct executions: invoice47, its linked invoice80 successor, and reserve50/bond7. These contribute 12/12/15 step positions and 7/7/9 effects, repeated across ordinary/metered envelopes. Metered fields are `logical:{cap,used}`, with used42/42/45. Both invoice links compare complete actual post/pre snapshots. This is local supplied-state linkage.

The actual Program test calls `mori_core::parse` and `serde_json::to_value`; it does not call `parse_program`, `validate`, `from_value`, or assert a Program round-trip. The CLI test compares six whole report Values and exact `.stdout.json` bytes. It does not separately open the three standalone Execution oracle files. The rejected ordinary review's contrary claims are preserved in its root disposition. The root-adopted designated review is referred to separately, without a new security judgment.

Root integration checklist:

- [ ] Attach PR202/203 final integration receipts to the exact finite child observations; preserve original source-time UNRUN/design prose.
- [ ] Keep PR204's source adoption separate from PR205's actual result and review status.
- [ ] Finish PR205's replacement ordinary review over the existing capture; root decides integration against exact head/main/tree. No repeat test allocation is needed to repair an inaccurate review.
- [ ] Keep every U/M/MC/SP/G exit open and all ACTUS, DeFi, UTXO, formal, recursive/private/composition, target and release memberships unchanged.
- [ ] Refer security-owned assessment and hostile controls to designated reviewers. This lane supplies no security acceptance.

Next product predicates, in dependency order:

1. Resolve the existing PR205 review/integration boundary using accurate source/result facts.
2. Independently freeze and exercise already-admitted positive financial report branches absent from the selected executions: present-duty addition, absent versus present-zero creation, zero/self movements and zero discharge. Uninvoked register declarations in a complete Program are not executed branches.
3. Carry the same two positive financial specimens through one actually supported public compiler/artifact consumer with full independent outcomes, explicit profile/refusal boundaries and applicable existing authority. Two local simulations do not close U2's signed source/native/target walkthrough.
4. Compare the same transition slice through actual K/Quint/required Sherlock Lean-Clean/circuit consumers, retaining complete state/prefix/effect/work adapters and separate evidence.
5. Reach the actual proof/artifact, authenticated target and readback consumers, then qualify all composition, conformance and independent release exits. Cardano's source-artifact → C2 issuance/consumption → VI/four shards → transaction fit → readback → composition path stays separate from Midnight/Fenrir/Preview/stagenet evidence.

Mint, Burn and dedicated rate/time/day-count/compounding accrual remain broader missing constructs where required. Existing Liability amount addition does not implement them. All twenty original UTXO identities, ACTUS277/every field/18 types/32 dispositions/DS01–07, DeFi72/DA01–24/supplements/three held-outs/eight intents/eight regression classes, and all five composition operators are retained in the structured delta. Test totals supply no whole-language percentage.

Only this new handoff directory was written. No tests, Git, checkout/main/W updates, reviewer calls, network, formal-model, compiler, SDK/proof/VM/wallet/node action was run by this lane. Startup status was read only. [SOURCES.json](SOURCES.json) pins 63 explicit inspected files; archival absolute case/provenance pointers remained inert.
