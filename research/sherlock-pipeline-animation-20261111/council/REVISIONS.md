# Revisions applied from COUNCIL.md

Run: pipeline-anim council revisions 20261111. Work order: `council/COUNCIL.md`, section "Revisions in apply order". Nothing was built, tested, proved or committed.

Abbreviations: `ARCH` = `research/sherlock-final-design-20261111/ARCHITECTURE.md`; `DEC` = `research/sherlock-final-design-20261111/DECISIONS.md`; `ML/` = `openspec/changes/sherlock-modular-leaf-20261111/`.

Decision ids: the placeholders K-1 to K-6 are replaced by a new `P-` series (one letter per source section, as the file does; `P-` is unused in DEC and ARCH) in a new DEC section "From the pipeline-animation design check".

ARCH line numbers cited elsewhere in the design folder are shifted by these edits and are not updated here.

## C5 part 1: node `reg_root` equality

- ARCH, NODE INTEGRATION: Provides `reg_root` (equality rule and constraint); Invariants (new "Registry root" item); Row budget (new bullet, under 10 rows ESTIMATE); Verification (Quint invariant `RegRootEqualAcrossSlots`, mutant `node_reg_root_mixed`).
- ARCH, GP-3 hook: Interface `reg_root` line and Assumed line.
- ML/specs/sherlock-node-integration: new NODE-008 with scenario.
- Decision: P-1. New mutant: `node_reg_root_mixed`. Specified here (not in COUNCIL): Quint invariant name `RegRootEqualAcrossSlots`.
