# Tasks: PCD ledger-anchored acceptance

Status: specified-only. Implementation belongs to the sprint tasks named in `openspec/sprints/coverage.json`. These tasks record adoption review and bind each requirement's evidence location.

Verification for every task: `python3 openspec/sprints/verify.py` and `openspec validate --all --strict`. Evidence tasks also require their sprint task's commands and reviews.

## 1. Planning adoption

- [ ] 1.1 Record each independent review verdict in `openspec/PCD-INTEGRATION-2026-09-11.md`.
- [ ] 1.2 Obtain the Charter's second reviewer verdict before any task below is admitted.
- [ ] 1.3 Record the user's confirmation or change for each default in the decision register.

## 2. Ledger correspondence (MC04)

- [ ] 2.1 Retain the seam and toolchain manifest under `evidence/moriarty-completion-program-2026-09-07/MC04/pcd-seam/` (SP01.4).
- [ ] 2.2 Retain E2 fit results under `evidence/moriarty-completion-program-2026-09-07/MC04/pcd-e2/` (SP09.1).
- [ ] 2.3 Retain E1 linearity results under `evidence/moriarty-completion-program-2026-09-07/MC04/pcd-e1/` (SP09.1).
- [ ] 2.4 Retain deploy-audit output for every Preview deployment under `evidence/moriarty-completion-program-2026-09-07/MC04/pcd-deploy-audit/` (SP09.1).
- [ ] 2.5 Retain bounds benchmarks under `evidence/moriarty-completion-program-2026-09-07/MC04/pcd-bounds/` (SP11.1).

## 3. Mandatory acceptance (MC05)

- [ ] 3.1 Retain Π_P, intent digest v2 and genesis body v2 test vectors under `evidence/moriarty-completion-program-2026-09-07/MC05/pcd-digests/` (SP09.3).
- [ ] 3.2 Retain genesis, termination and freshness controls under `evidence/moriarty-completion-program-2026-09-07/MC05/pcd-lifecycle/` (SP09.2, SP09.3).
- [ ] 3.3 Retain E4 migration results under `evidence/moriarty-completion-program-2026-09-07/MC05/pcd-e4/` (SP09.3).

## 4. Composition (MC06)

- [ ] 4.1 Retain split, join, release and reclaim controls under `evidence/moriarty-completion-program-2026-09-07/MC06/pcd-composition/` (SP10.3).
- [ ] 4.2 Retain handoff isolation evidence under `evidence/moriarty-completion-program-2026-09-07/MC06/pcd-handoff/` (SP10.2).

## 5. Certificates (MC03)

- [ ] 5.1 Retain E3 results under `evidence/moriarty-completion-program-2026-09-07/MC03/pcd-e3/` (SP04.3).
- [ ] 5.2 Retain E5 results under `evidence/moriarty-completion-program-2026-09-07/MC03/pcd-e5/` (SP06.2, SP06.3).
- [ ] 5.3 Update the dependency tracker in `openspec/sprints/pcd-integration.json` before each certificate campaign.
