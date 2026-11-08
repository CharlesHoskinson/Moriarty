# Synthetic rollup data EARS

Status: specified, not implemented or accepted. SSRD clauses supplement SRPOC-001 to SRPOC-009 and SBENCH-001 to SBENCH-010. Their count never enters the original UIREQ, SCON or SRPOC completion totals. The [spec delta](specs/synthetic-rollup-data/spec.md) gives the scenarios. The [design](design.md) gives the mechanisms.

## Owner amendment 2026-11-08

The owner decided (2026-11-08, verbatim): "Sherlock should not depend at all on Fenrir, We have a synthetic rollup design, we use that", and "keep them out of C4". [proposal.md](proposal.md) "Owner amendment 2026-11-08" gives the full amendment and overrides any conflicting text here.

SSRD-001, 002, 005, 011, 013 to 017 and 024 are amended 2026-11-08 below; SSRD-029 and SSRD-030 are added. The original wording of each amended clause is in git history before this amendment.

## Source of truth and generation

- **SSRD-001 Synthetic parameter set (amended 2026-11-08).** When FSYN generates any artefact, it shall verify every file of the in-repository synthetic parameter set against its manifest, and it shall refuse to generate with exit 2 if one byte count or SHA-256 differs. It shall not read `design-cut.json` or any path outside the repository.
- **SSRD-002 Parameter constants (amended 2026-11-08).** The generator shall read every rollup constant, field order, domain tag, layout and rule from the Moriarty-owned parameter set, and from no hand-typed literal in code.
- **SSRD-003 Determinism.** When FSYN receives the same cut digest, generator version, seed, profile and scale, it shall emit byte-identical artefacts on every supported host.
- **SSRD-004 Independent streams.** The generator shall draw each concern (header schedule, transaction mix, committee behaviour, Celestia chain, adversarial mutation) from a separate named random stream, so a change to one concern leaves the others byte-identical.
- **SSRD-005 S0 golden reproduction (amended 2026-11-08).** When FSYN runs scale S0 with seed `0x5354524157`, it shall reproduce every envelope, view digest and `workloads.json` digest of the frozen in-repository S0 golden byte for byte, without `rtf generate` or an FRTF tree.

## Shape and profiles

- **SSRD-006 Scale ladder.** FSYN shall provide scales S0 to S5 with the counts in design section 3, and each artefact shall name its scale; a smaller scale shall never carry a larger scale's label.
- **SSRD-007 Transaction mix.** The generator shall emit every leaf step kind (MAIN, LATE, FORCED, DEPOSIT, SYS, IMPORT, CLOSE) and every forced outcome code at the declared mix, and it shall record the realized counts per batch.
- **SSRD-008 Header and blob sizes.** When FSYN emits a header, it shall size its `Records` payload from the declared mix, count Celestia shares by the sparse-share rule, and refuse a header above `MAX_HEADER_SHARES`.
- **SSRD-009 Committee events.** The generator shall model 16 sealed seats, with at most f = 4 Byzantine seats a run, and it shall start a migration scenario when permanent seat losses reach 3.
- **SSRD-010 Celestia light client.** When FSYN emits a light-client chain, every append shall satisfy `0 < B - pred(B) <= GAP_MAX`, and every pointer shall have exactly one covering stored header.
- **SSRD-011 Two profiles, one schema (amended 2026-11-08: Profile B is the synthetic production shape, defined by the parameter set).** FSYN shall emit Profile A and Profile B views of one batch from the same core record set, and the two views shall agree on `pre`, `post`, `in_root`, `h_first`, `h_last` and every sum field.

## Fidelity and oracles

- **SSRD-012 Fidelity map.** Each artefact type shall name its interface operation, its statement (`stmt`, `stmt_da`, `stmt_lc` or `stmt_dm`) and its in-repository Quint oracle module in the fidelity table, and an artefact without a row shall not be emitted.
- **SSRD-013 Forward replay (amended 2026-11-08: against the Moriarty-owned oracle models in the repository).** When FSYN emits a scenario, it shall emit a Quint replay test of the same action sequence, and the scenario shall be published only if `quint test` passes with every named invariant.
- **SSRD-014 Reverse replay (amended 2026-11-08: the oracle models are the in-repository ones).** FSYN shall consume ITF traces sampled from each oracle model and recompute their observable outcomes, and any disagreement shall fail the oracle gate.
- **SSRD-015 Independent oracle.** The generator's own replay shall be labelled internal consistency, and acceptance shall use only the in-repository oracle models, the bounds restated in the parameter set and independently frozen expected states (amended 2026-11-08: no Fenrir model, Lean file or `rtf verify`).

## Tests

- **SSRD-016 Golden vectors.** FSYN shall keep golden vectors for the relation digest, envelope digests, share counts, L_inc leaf counts, the I16 sum and the rotation batch cap, each bound to its parameter-set entry (amended 2026-11-08: not to a Fenrir source line or theorem).
- **SSRD-017 Property tests.** For every module property listed in design section 6, the suite shall run a property test over generated scenarios and assert the bound restated in the parameter set where one exists (amended 2026-11-08: the bound is a Moriarty-owned parameter; no Lean file is read).
- **SSRD-018 Mutation.** Every generator rule and every oracle check shall have a registered mutant, and each mutant shall be killed by at least one test before a fixture release.
- **SSRD-019 Adversarial corpora.** For each of the 14 adversary classes of the security model and the two exit attackers of the exit design note, FSYN shall emit at least one case with its expected failing layer, reason code, Quint invariant and killing mutant.
- **SSRD-020 Replayable fixtures.** Each fixture release shall carry a manifest of path, bytes and SHA-256, its cut digest and its exact regeneration command, and `fsyn verify` shall reproduce it byte for byte.

## Benchmarks, phasing, exits and lifecycle

- **SSRD-021 Benchmark trigger.** A benchmark phase shall start only after its named trigger artefact exists (design section 9), and a missing trigger shall leave the phase open without a substitute workload.
- **SSRD-022 Benchmark scope.** When a benchmark runs, it shall report per leaf type and layer the SRPOC-005 metrics on the SBENCH /2 sample schema, plus the FSYN workload digest and scale.
- **SSRD-023 Modeled and measured series.** When a report gives a fleet size or a settled-TPS point, it shall label each value measured or projected, and it shall derive projections only from measured unit costs.
- **SSRD-024 Parameter change (amended 2026-11-08).** If the in-repository parameter set changes, FSYN shall mark every dependent fixture stale, and stale fixtures shall not enter a benchmark sample. A change to a Fenrir design or cut shall not mark any fixture stale and shall not stop generation; an optional offline sync may propose a parameter change as an ordinary commit.
- **SSRD-025 Exit paths and related actions.** FSYN shall emit every route and related action of design section 4.4 and preserve the shared routed key `xk` across spend, escape and import according to each pinned transition. An applied `FENC_RELEASE` shall lift the hold and consume the capability while leaving the note unspent for a later withdrawal; Claim shall mark the withdrawal paid without spending the note again, and import shall enter the already escaped `xk` into routed nullifier state without another payout.
- **SSRD-026 Holds.** The generator shall key each hold by `H(DOM_HT, nf)`, give each capability at most one hold, and decide each hold once at execution.
- **SSRD-027 Boundary disclosure.** Each fixture shall expose only its pinned route's public fields and preserve the modeled unlinkability to L2 history. Forced TRANSFER shall expose no note asset, amount or destination. WITHDRAW, escape and payout shall expose asset and amount (Q17); shielded payout shall hide its destination, while unshielded payout shall reveal its own destination (Q16).
- **SSRD-028 Sealed lifecycle.** The generator shall emit only key-preserving in-place rotations, and shall emit every key or roster change as a migration to a new deployment with wind-down exits.

## Qualification and independence (added 2026-11-08)

- **SSRD-029 Staged synthetic-rollup qualification.** The factory B rows shall qualify Sherlock on FSYN-generated data only, local first (B1, B2, B3, B6, B7), then Preview (B4p, B5p, B8p, B9p), then Mainnet (B4m, B5m, B8m, B9m); each Preview row shall depend on the local rows it uses and each Mainnet row on its Preview counterpart. No B row shall target stagenet. The B rows shall stay out of the Sherlock C4 closure (owner decision 2026-11-08, "keep them out of C4").
- **SSRD-030 Self-contained in Moriarty.** FSYN generation and tests and every B-row check shall read only tracked files of this repository, and no path outside it and no path containing `projectfenrir` or `rollup-subarch`; an optional offline comparison with a Fenrir design shall never be a gate.
