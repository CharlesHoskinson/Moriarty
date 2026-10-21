# Synthetic rollup data EARS

Status: specified, not implemented or accepted. SSRD clauses supplement SRPOC-001 to SRPOC-009 and SBENCH-001 to SBENCH-010. Their count never enters the original UIREQ, SCON or SRPOC completion totals. The [spec delta](specs/synthetic-rollup-data/spec.md) gives the scenarios. The [design](design.md) gives the mechanisms.

## Source of truth and generation

- **SSRD-001 Design cut.** When FSYN generates any artefact, it shall verify every entry of the pinned design cut, and it shall refuse to generate with exit 2 if one byte count or SHA-256 differs.
- **SSRD-002 Extracted constants.** The generator shall read every rollup constant, field order and domain tag from the pinned Quint models, `liv_params.planning.json`, FRTF `params-v3.json` or the extracted layout table, and from no hand-typed literal.
- **SSRD-003 Determinism.** When FSYN receives the same cut digest, generator version, seed, profile and scale, it shall emit byte-identical artefacts on every supported host.
- **SSRD-004 Independent streams.** The generator shall draw each concern (header schedule, transaction mix, committee behaviour, Celestia chain, adversarial mutation) from a separate named random stream, so a change to one concern leaves the others byte-identical.
- **SSRD-005 FRTF v3 reproduction.** When FSYN runs scale S0 with seed `0x5354524157`, it shall reproduce every FRTF v3 envelope, view digest and `workloads.json` digest byte for byte.

## Shape and profiles

- **SSRD-006 Scale ladder.** FSYN shall provide scales S0 to S5 with the counts in design section 3, and each artefact shall name its scale; a smaller scale shall never carry a larger scale's label.
- **SSRD-007 Transaction mix.** The generator shall emit every leaf step kind (MAIN, LATE, FORCED, DEPOSIT, SYS, IMPORT, CLOSE) and every forced outcome code at the declared mix, and it shall record the realized counts per batch.
- **SSRD-008 Header and blob sizes.** When FSYN emits a header, it shall size its `Records` payload from the declared mix, count Celestia shares by the sparse-share rule, and refuse a header above `MAX_HEADER_SHARES`.
- **SSRD-009 Committee events.** The generator shall model 16 sealed seats, with at most f = 4 Byzantine seats a run, and it shall start a migration scenario when permanent seat losses reach 3.
- **SSRD-010 Celestia light client.** When FSYN emits a light-client chain, every append shall satisfy `0 < B - pred(B) <= GAP_MAX`, and every pointer shall have exactly one covering stored header.
- **SSRD-011 Two profiles, one schema.** FSYN shall emit Profile A and Profile B views of one batch from the same core record set, and the two views shall agree on `pre`, `post`, `in_root`, `h_first`, `h_last` and every sum field.

## Fidelity and oracles

- **SSRD-012 Fidelity map.** Each artefact type shall name its interface operation, its statement (`stmt`, `stmt_da`, `stmt_lc` or `stmt_dm`) and its Quint oracle module in the fidelity table, and an artefact without a row shall not be emitted.
- **SSRD-013 Forward replay.** When FSYN emits a scenario, it shall emit a Quint replay test of the same action sequence, and the scenario shall be published only if `quint test` passes with every named invariant.
- **SSRD-014 Reverse replay.** FSYN shall consume ITF traces sampled from each oracle model and recompute their observable outcomes, and any disagreement shall fail the oracle gate.
- **SSRD-015 Independent oracle.** The generator's own replay shall be labelled internal consistency, and acceptance shall use only the pinned Quint models, Lean bounds, Fenrir `rtf verify` and independently frozen expected states.

## Tests

- **SSRD-016 Golden vectors.** FSYN shall keep golden vectors for the relation digest, envelope digests, share counts, L_inc leaf counts, the I16 sum and the rotation batch cap, each bound to its source line or theorem.
- **SSRD-017 Property tests.** For every module property listed in design section 6, the suite shall run a property test over generated scenarios and assert the Lean-proved bound where one exists.
- **SSRD-018 Mutation.** Every generator rule and every oracle check shall have a registered mutant, and each mutant shall be killed by at least one test before a fixture release.
- **SSRD-019 Adversarial corpora.** For each of the 14 adversary classes of the security model, FSYN shall emit at least one case with its expected failing layer, reason code, Quint invariant and killing mutant.
- **SSRD-020 Replayable fixtures.** Each fixture release shall carry a manifest of path, bytes and SHA-256, its cut digest and its exact regeneration command, and `fsyn verify` shall reproduce it byte for byte.

## Benchmarks and phasing

- **SSRD-021 Benchmark trigger.** A benchmark phase shall start only after its named trigger artefact exists (design section 9), and a missing trigger shall leave the phase open without a substitute workload.
- **SSRD-022 Benchmark scope.** When a benchmark runs, it shall report per leaf type and layer the SRPOC-005 metrics on the SBENCH /2 sample schema, plus the FSYN workload digest and scale.
- **SSRD-023 Modeled and measured series.** When a report gives a fleet size or a settled-TPS point, it shall label each value measured or projected, and it shall derive projections only from measured unit costs.
- **SSRD-024 Design change.** If the live design differs from the pinned cut, FSYN shall mark every dependent fixture stale, and stale fixtures shall not enter a benchmark sample.
