# PCD OpenSpec Roadmap Update Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Adopt the Midnight-native PCD decision into the Moriarty OpenSpec roadmap so mandatory Preview acceptance no longer waits on recursion, while certificates stay release scope.

**Architecture:** Keep all 21 stage ids and re-root `f3` on the Stage 0 seam; repurpose the native track for certificates. Add one non-`mc` change package with eighteen MC03–MC06-owned requirements, an integration amendment with a machine crosswalk, and a validator guard. Correct the report, PCD roadmap and wiki. Design: `docs/superpowers/specs/2026-09-11-pcd-openspec-roadmap-update-design.md`.

**Tech Stack:** Python 3 patch scripts with anchor assertions, JSON planning records, OpenSpec CLI, `openspec/sprints/verify.py`, pytest, claude-obsidian 2.1.1 transactions.

## Global Constraints

- Work in `/home/charl/Moriarty` on `main`. No commits, pushes or branches: the user commits only when asked, and Claude never authors commits.
- Never edit `openspec/REPORT-RECONCILIATION-2026-09-07.md`, `openspec/sprints/sp01-financial-contract-and-execution-admission.md`, `openspec/changes/afk-live-financial-execution/**`, `docs/FOOTGUNS.md`, `.moriarty-dev/**`, `AGENTS.md`, `site/**` or `repos/**`.
- Never change an original `- [ ] N.N` line in `openspec/changes/mc*/tasks.md`.
- Wiki files change only through a claude-obsidian transaction.
- Stage ids stay 21; only `f3.requires` changes. `dispatchEnabled` stays false; statuses, campaign ids, G01–G24 and LR01–LR18 stay.
- Every campaign keeps k ≤ 17, 8 GiB and two CPU jobs.
- Everything stays specified-only; nothing is described as implemented.
- Report and page corrections are silent: no change notes inside `REPORT.md` or the HTML page.
- JSON is written with indent 2, `ensure_ascii=False` and a trailing newline.
- Every patch asserts its anchor count through `pcdlib.rep`/`resub`; an assertion failure stops the task for inspection.
- Scratch directory: `S=/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad`.

## File map

| Group | Files | Responsibility |
|---|---|---|
| Records | `openspec/moriarty-completion-program.json`, `openspec/sprints/sprints.json` | Stage graph, package fields, sprint edges |
| Requirements | `openspec/changes/pcd-ledger-anchored-acceptance/**`, `openspec/sprints/coverage.json`, MC03–MC06 `spec.md`/`README.md`, MC04/MC05 `design.md`/`tasks.md` prose | New obligations, in-place amendment notes |
| Crosswalk and guard | `openspec/PCD-INTEGRATION-2026-09-11.md`, `openspec/sprints/pcd-integration.json`, `openspec/sprints/verify.py`, `openspec/sprints/test_verify.py` | Human amendment, machine twin, validator |
| Sprint contracts | SP04, SP06, SP09, SP10, `openspec/sprints/README.md`, `report-lessons.json`, `asset-study.json`, `package-task-map.json` titles, `openspec/ASSET-STUDY-INTEGRATION-2026-09-09.md` | Task wording and lessons |
| Narrative | `ROADMAP.md`, `openspec/MORIARTY-COMPLETION-PROGRAM.md`, `openspec/ROADMAP-REFINEMENT-2026-09-09.md` | Reader-facing roadmap |
| Corrections | `deliverables/pcd-midnight-native-2026-09-11/REPORT.md`, `moriarty-pcd-on-midnight.html`, `openspec/PCD-ROADMAP-2026-09-11.md` | Errors found by the audits |
| Vault | `wiki/decisions/pcd-midnight-native-architecture.md`, `wiki/benchmarks.md`, `wiki/moriarty-architecture.md`, `wiki/contradictions.md`, `wiki/log.md`, `wiki/meta/ledgers/claim-ledger.json` | Knowledge base |

---

### Task 0: Harness and baselines

**Files:** Create `$S/pcdlib.py`, `$S/validate-clean-copy.sh`, `$S/plugin-validate-before.json` (all scratch).

**Interfaces:** Produces `pcdlib.rep(path, old, new, count=1)`, `pcdlib.resub(path, pattern, repl, count=1)`, `pcdlib.load_json(path)`, `pcdlib.dump_json(path, value)`, `pcdlib.sha256(path)`; paths are repository-relative.

- [ ] **Step 1: Confirm helper and harness exist** (created during planning; contents recorded in the design session).

<!-- check: T0 -->
```bash
S=/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad
python3 -c "import sys; sys.path.insert(0,'$S'); import pcdlib; print(pcdlib.ROOT)"
$S/validate-clean-copy.sh
cat $S/plugin-validate-before.json
```
Expected: `/home/charl/Moriarty`; verify.py `"status": "pass"`; `8 passed`; `[]`.

---

### Task 1: Re-root the stage graph and Register fields

**Files:** Modify `openspec/moriarty-completion-program.json`, `openspec/sprints/sprints.json`.

**Interfaces:** Produces `f3.requires = [atomic-accept, i2, f0, native-path-freeze]`; SP09 `completionRequires = [SP03, SP05, SP07, SP08]`; SP12 `completionRequires = [SP06, SP11]`; MC04 deps `[MC01, MC02]`; MC05 deps `[MC04]`.

- [ ] **Step 1: Apply the record edits**

<!-- run: T1 -->
```bash
cd ~/Moriarty && python3 - <<'PY'
import sys; sys.path.insert(0, '/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
from pcdlib import load_json, dump_json
REG, SPR = 'openspec/moriarty-completion-program.json', 'openspec/sprints/sprints.json'
reg, spr = load_json(REG), load_json(SPR)
rr = reg['reportReconciliation']
stages = {s['id']: s for s in rr['stageAdmission']['stages']}
assert len(stages) == 21
stages['f0']['purpose'] = 'PCD Stage 0: pinned verification seam (operation key in ContractState.operations checked by ledger well_formed), toolchain manifest per network generation, head-discipline checker and deploy-audit designs, and a certificate-route go/no-go conditional on ledger-10'
stages['native-path-freeze']['purpose'] = 'Reviewed path and command-interface ownership for step-relation compiler output, head-discipline checker, deploy audit and certificate-relation roots, with source pins, before source authorship; actual commands freeze after F0a implementation'
stages['f0a']['purpose'] = 'Certificate-relation authorship against pinned pull request 738 / ledger-10 sources: guard-constant lint, Collapsed decider constraints on vk_repr and state, frozen hashes and current source reviews'
stages['f1-fixtures']['purpose'] = 'Separately admitted bounded generation of independent non-loan certificate fixtures (Poseidon zk-stdlib inner proofs and a verifier-test accumulator)'
stages['f1']['purpose'] = 'E3: native certificate accepted on a ledger-10 devnet with its accumulator pairing; free or mismatched guard, substituted vk_repr, unbound inner instance, foreign-key and tampered inner proofs all reject; fee against measured validation work recorded'
stages['f2']['purpose'] = 'E5: off-ledger segment certificate over the Moriarty step at 1, 10 and 100 steps, retained and verified in a fresh process, and imported through the certificate entry point'
stages['f3']['purpose'] = 'PCD Stages 1-3 for the atomic loan/swap profile: E2 fused step relation fit, E1 head read-then-write linearity on Preview, immutable-authority deploy audit and verification-enabled Preview acceptance of one step each'
stages['mandatory']['purpose'] = 'Versioned mandatory-claim extension under the fused step relation: claim discharge map, constrained genesis and termination, deploy-audited property certificate, block-time observation freshness and forward-declared migration (E4); requalify exactly changed atomic domains'
stages['composition']['purpose'] = 'Versioned composition extension: ledger-atomic split/join, cross-contract Release/JoinFrom/Reclaim on ledger 9, recipient-keyed handoff and operator rules under head discipline, with corresponding requalification'
assert stages['f3']['requires'] == ['atomic-accept', 'i2', 'f2', 'f1']
stages['f3']['requires'] = ['atomic-accept', 'i2', 'f0', 'native-path-freeze']
pk = {p['id']: p for p in reg['packages']}
assert pk['MC04']['dependencies'] == ['MC01', 'MC02', 'MC03'] and pk['MC05']['dependencies'] == ['MC03', 'MC04']
pk['MC04']['dependencies'] = ['MC01', 'MC02']
pk['MC05']['dependencies'] = ['MC04']
assert pk['MC03']['commands'][0].startswith('python3 experiments/moriarty-native-ivc-r3/successor/')
pk['MC03']['commands'] = []
pk['MC03']['commandStatus'] = 'No planned command: the experiments/moriarty-native-ivc-r3/successor root is retired by the PCD integration; certificate-relation commands are fixed at native-path-freeze and frozen after implemented-source review'
assert pk['MC04']['status'] == 'interface-blocked'
pk['MC04']['status'] = 'specified-only'
pk['MC04']['feasibilityStatus'] = 'Core verification seam declared from pinned ledger-8 source (operation key in ContractState.operations, ledger well_formed); RP02 review, E1 and E2 pending. The certificate route belongs to MC03 and waits on ledger-10.'
rr['feasibilityStages'].update({
    'F0': 'Stage 0 seam, toolchain manifest, head-discipline checker and deploy-audit design; certificate-route go/no-go',
    'F1': 'E3 native certificate on a ledger-10 devnet with negative controls; independent non-loan certificate fixtures',
    'F2': 'E5 off-ledger segment certificate and fresh-process verification',
    'F3': 'Ledger-anchored atomic core: E2 fit, E1 linearity on Preview, deploy audit and verification-enabled Preview acceptance',
    'F0a': 'MC03/MC04 certificate-relation authorship under recorded preparation grant; source audits and a reviewed resource amendment before F1',
})
rp02 = next(g for g in rr['gates'] if g['id'] == 'RP02')
assert rp02['status'] == 'blocked'
rp02['purpose'] = 'On-ledger history by ledger induction, artifact handoff and certificate route before native campaign'
rp02['reason'] = 'The on-ledger history route is settled in design by the PCD integration, pending RP02 review and experiments E1 and E4; the certificate route is blocked on ledger-10 release, a reviewed resource amendment for k >= 18 and accumulator fee accounting. No gate evidence or campaign admitted.'
dump_json(REG, reg)
sp = {s['id']: s for s in spr['sprints']}
gate = next(g for g in sp['SP09']['entryGates'] if g['stage'] == 'f3')
gate['requires'] = list(stages['f3']['requires'])
assert sp['SP09']['completionRequires'] == ['SP03', 'SP05', 'SP06', 'SP07', 'SP08']
sp['SP09']['completionRequires'] = ['SP03', 'SP05', 'SP07', 'SP08']
assert sp['SP12']['completionRequires'] == ['SP11']
sp['SP12']['completionRequires'] = ['SP06', 'SP11']
dump_json(SPR, spr)
print('T1 applied')
PY
```

- [ ] **Step 2: Verify graph, validator and plugin**

<!-- check: T1 -->
```bash
S=/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad
cd ~/Moriarty && python3 - <<'PY'
import json
reg = json.load(open('openspec/moriarty-completion-program.json'))
st = {s['id']: s for s in reg['reportReconciliation']['stageAdmission']['stages']}
def anc(i):
    out, todo = set(), list(st[i]['requires'])
    while todo:
        d = todo.pop()
        if d not in out: out.add(d); todo += st[d]['requires']
    return out
assert not {'f0a', 'f1-fixtures', 'f1', 'f2'} & anc('mandatory'), anc('mandatory')
assert 'f2' in anc('release')
print('graph ok')
import sys; sys.path.insert(0, 'plugins/moriarty-dev/scripts')
from moriarty_dev import records
m = []
records._validate_program(reg, m); records._validate_sprints(json.load(open('openspec/sprints/sprints.json')), m)
print('plugin', m)
PY
$S/validate-clean-copy.sh
```
Expected: `graph ok`; `plugin []`; verify.py pass; `8 passed`.

---

### Task 2: Correct the report, page and PCD roadmap

**Files:** Modify `deliverables/pcd-midnight-native-2026-09-11/REPORT.md`, `deliverables/pcd-midnight-native-2026-09-11/moriarty-pcd-on-midnight.html`, `openspec/PCD-ROADMAP-2026-09-11.md`.

**Interfaces:** Produces final bytes of REPORT.md and PCDR; Task 5 pins their digests.

- [ ] **Step 1: Apply corrections**

<!-- run: T2 -->
```bash
cd ~/Moriarty && python3 - <<'PY'
import sys; sys.path.insert(0, '/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
import re
from pcdlib import rep, resub
R = 'deliverables/pcd-midnight-native-2026-09-11/REPORT.md'
H = 'deliverables/pcd-midnight-native-2026-09-11/moriarty-pcd-on-midnight.html'
P = 'openspec/PCD-ROADMAP-2026-09-11.md'
# Claim-model count and validity citation (R3 G10, G11)
rep(R, '- **Claim model.** It exists in four inconsistent forms:', '- **Claim model.** It exists in inconsistent forms, with at least five claim vocabularies across the repository. Three matter here:')
rep(H, '<li><strong>Claim model.</strong> It exists in four inconsistent forms:', '<li><strong>Claim model.</strong> It exists in inconsistent forms, with at least five claim vocabularies across the repository. Three matter here:')
for f in (R, H):
    rep(f, 'a registered atomic profile with a flat claim list and no verifier-key, validity, dependency or budget fields', 'a registered atomic profile whose claim requirements carry no verifier-key, validity, dependency or budget fields')
rep(R, 'Existing record bound is 128 per state [D: M `typed-schemas.md:448-460`]', 'Existing record bound is 128 per state [D: M `bounds.json:91`]')
resub(H, r'(128 per state.*?)typed-schemas\.md:448-460', r'\1bounds.json:91', flags=0)
# Memory figures
rep(R, '| Peak memory | 4.2 GiB | 8.0 GiB |', '| Peak memory | 4.1 GiB | 7.8 GiB |')
rep(H, '<td>4.2 GiB</td>\n<td>8.0 GiB</td>', '<td>4.1 GiB</td>\n<td>7.8 GiB</td>')
for f in (R, H):
    rep(f, 'one plain inner proof costs about 35 s and 4.2 GiB, and a carried accumulator about 82 s and 8 GiB.', 'one plain inner proof costs about 35 s and 4.1 GiB, and a carried accumulator about 82 s and 7.8 GiB.')
    rep(f, '82 s and 8 GiB for a trivial two-level test', '82 s and 7.8 GiB for a trivial two-level test')
# Entry points
rep(R, '`Release`, `JoinFrom` | M1 |', '`Release`, `JoinFrom`, `Reclaim` and `Terminate`, plus an optional principal-threshold `Pause` | M1 |')
resub(H, r'<code>Release</code>, <code>JoinFrom</code>(\s*</td>)', r'<code>Release</code>, <code>JoinFrom</code>, <code>Reclaim</code> and <code>Terminate</code>, plus an optional principal-threshold <code>Pause</code>\1', flags=0)
# Migration cycle
for f in (R, H):
    rep(f, 'prove  (Π_new, A_new) ∈ successorAllowList(Π_old)     -- forward-declared in Π_old', 'prove  Π_new ∈ successorAllowList(Π_old)              -- program digests only, forward-declared in Π_old\n                          ∧ migration threshold of Π_old signed H(MORIARTY-MIGRATE-v2 ‖ netTag ‖ A_old ‖ A_new ‖ Π_new ‖ r ‖ S)')
    rep(f, 'require A_old ∈ predecessorAllowList(Π_new) ∧ comm ∉ imported; record comm', 'require caller = A_old recorded in its deploy state Uninit(Π_new, netTag, A_old) ∧ comm ∉ imported; record comm')
rep(R, '- **Consumed state stays consumed.**', '- **No address cycle.** `successorAllowList` holds program digests only, and `A_new` records `A_old` in its deploy-time state. `A_old` commits to `Π_old`, which may contain `Π_new`; `A_new` commits to `Π_new` and `A_old`. Principals run the deploy audit on `A_new` before signing.\n- **Consumed state stays consumed.**')
rep(H, '<li><strong>Consumed state stays consumed.</strong>', '<li><strong>No address cycle.</strong> <code>successorAllowList</code> holds program digests only, and <code>A_new</code> records <code>A_old</code> in its deploy-time state. <code>A_old</code> commits to <code>Π_old</code>, which may contain <code>Π_new</code>; <code>A_new</code> commits to <code>Π_new</code> and <code>A_old</code>. Principals run the deploy audit on <code>A_new</code> before signing.</li>\n<li><strong>Consumed state stays consumed.</strong>')
# Commitment hash
rep(R, 'Split heavy hashing into a hand-written zk-stdlib relation with Poseidon commitments, or lower the effect bounds.', 'Split heavy hashing into a hand-written zk-stdlib relation, or lower the effect bounds. State commitments stay SHA-256 `persistentHash`; moving them to Poseidon needs a reviewed decision.')
rep(H, 'Split heavy hashing into a hand-written zk-stdlib relation with Poseidon commitments, or lower the effect bounds.', 'Split heavy hashing into a hand-written zk-stdlib relation, or lower the effect bounds. State commitments stay SHA-256 <code>persistentHash</code>; moving them to Poseidon needs a reviewed decision.')
# Certificate k bound
resub(R, r'\| Circuit size k \(certificate entry point\) \| ≤ 20 \| [^|\n]* \|', '| Circuit size k (certificate entry point) | Set by a reviewed resource amendment | Measured 18 for one level and 19 for two levels, above the k ≤ 17 campaign ceiling; a step circuit adds rows |')
resub(H, r'(<td>Circuit size k \(certificate entry point\)</td>\s*)<td>≤ 20</td>(\s*)<td>[^<]*</td>', r'\1<td>Set by a reviewed resource amendment</td>\2<td>Measured 18 for one level and 19 for two levels, above the k ≤ 17 campaign ceiling; a step circuit adds rows</td>', flags=0)
# Stage 3 and Stage 7 in section 18
rep(R, '- **Artifacts.** The same contract deployed with an empty maintenance committee and threshold 1 on Preview.', '- **Artifacts.** The same contract deployed on Preview through a custom deploy path with an empty maintenance committee and threshold 1, plus a passing deploy audit.')
rep(H, '<li><strong>Artifacts.</strong> The same contract deployed with an empty maintenance committee and threshold 1 on Preview.</li>', '<li><strong>Artifacts.</strong> The same contract deployed on Preview through a custom deploy path with an empty maintenance committee and threshold 1, plus a passing deploy audit.</li>')
for f in (R, H):
    rep(f, "Moriarty's mandatory acceptance for one step is ledger-enforced on a public network.", 'Mandatory acceptance for one step, without the deploy-time property certificate or certificate relations, is ledger-enforced on a public network.')
    rep(f, 'Certificates are sound and fit the k and time bounds, and fee accounting', 'Certificates are sound and fit the k and time bounds set by a reviewed resource amendment, and fee accounting')
# PCD roadmap
rep(P, """**Status.** Proposed planning revision. Specified-only; not adopted.

- Date: 2026-09-11 UTC.
- **Scope of effect.**
  - It changes no existing MC01–MC08 acceptance, SP01–SP12 exit gate, resource authority or model routing.
  - It does not alter the active eight-hour AFK work selection.
  - Adopting it into sprint acceptance requires the project's normal design review.
""", """**Status.** Adopted into the OpenSpec roadmap by the [PCD integration amendment](PCD-INTEGRATION-2026-09-11.md) and the [PCD change package](changes/pcd-ledger-anchored-acceptance/README.md). Specified-only.

- Date: 2026-09-11 UTC.
- **Scope of effect.**
  - The amendment re-roots atomic F3 on the Stage 0 seam, repurposes SP04 and SP06 for certificates and removes MC03 from the MC04 and MC05 dependencies.
  - It changes no resource authority or model routing. Certificate campaigns need a reviewed resource amendment.
  - It does not alter the active eight-hour AFK work selection.
  - Six design defaults await the user's confirmation, and the Charter's second reviewer has not reviewed the adoption.
""")
rep(P, 'took about 35 s and 4.3 GiB; a two-level one took about 82 s and 8.2 GiB.', 'took about 35 s and 4.1 GiB; a two-level one took about 82 s and 7.8 GiB.')
resub(P, r'^\| MC04 / SP04 native verifier feasibility and correspondence \|.*$', '| MC04 / SP09 ledger correspondence and consumption, with SP04 certificate feasibility | Complete native-to-Preview verifier with the actual final accumulator decision; compiler correspondence; durable one-time consumption | Declare the seam: the operation key in `ContractState.operations`, over the ledger-built statement. Map compiler correspondence to the fused step relation and head read-then-write discipline, and durable consumption to head discipline and E1. Certificates use pull request 738\'s ledger-side deferred pairing in SP04, with the P1/P2/P3 control discipline. | Strict encoding; deployed-version provenance; unique consumption under restart and concurrency. |', flags=re.M)
rep(P, 'Replace the transaction-level manifest with the compiled claim set and intent digest v2. |', 'Replace the transaction-level manifest with the compiled claim set and intent digest v2. Replace in-place verifier revocation with forward-declared migration and a principal-threshold `Pause`. |')
rep(P, '| Resolved in design by sections 12–13 of the report, pending experiments E1, E4 and E5. |', '| Settled in design for on-ledger history by sections 12–13 of the report, pending RP02 review and experiments E1 and E4. The certificate route stays conditional on the pull request 738 release, k ≥ 18 parameter serving, fee accounting, E3 and E5. |')
rep(P, '; wrong revision; stale observation. Each fails proving or verification.', '; wrong revision. Each fails proving or verification.')
rep(P, '- **Negative controls.** A second `Initialize`, a stale `Step` and a forged head all reject.', '- **Negative controls.** A second `Initialize`, a stale `Step`, a forged head and a stale observation all reject at application.')
rep(P, '- **Deliverables.** The Stage 2 contract deployed with committee `[]` and threshold 1.', '- **Deliverables.** The Stage 2 contract deployed through a custom deploy path with committee `[]` and threshold 1, plus a passing deploy audit.')
rep(P, '- **Exit.** Mandatory acceptance for one step is ledger-enforced on a public network.', '- **Exit.** Mandatory acceptance for one step, without the deploy-time property certificate or certificate relations, is ledger-enforced on a public network.')
rep(P, 'plus a terminal transition.', 'plus a `Terminate` transition.')
rep(P, 'fit k ≤ 20 and 600 s on the proof server', 'fit the k, time and memory bound set by a reviewed resource amendment')
rep(P, '| Absent; `netTag` mitigation |', '| Absent; `netTag` separates networks but not byte-identical replicated deployments |')
rep(P, '| Maintenance-authority threshold validation | Defense in depth | Absent; deploy audit covers it | M3 (optional) |', '| Maintenance-authority threshold validation | Defense in depth | Absent; deploy audit covers it | M3 (optional) |\n| Ledger 9 on Preview | SP09.3 migration and SP10.3 cross-contract Preview evidence | Hard fork in node 2.1.0-beta.1; Preview runs ledger 8 | M3 (Midnight) |\n| Reviewed resource amendment for certificate k | Stage 7, E3 and E5 | Not requested; campaigns capped at k ≤ 17 | Moriarty decision |')
rep(P, '| ≤ 17 / ≤ 20 |', '| ≤ 17 / set by a reviewed resource amendment (measured 18–19) |')
rep(P, '## 9. Open decisions requiring design review\n\n', '## 9. Open decisions requiring design review\n\nThe [PCD integration amendment](PCD-INTEGRATION-2026-09-11.md#decision-register) records these with owners, defaults and status.\n\n')
rep(P, 'Mandatory evidence is not weakened.', 'Mandatory evidence is not weakened.\n- **The certificate resource amendment is refused.** Stage 7 stays blocked. Stages 0–6 and the Preview gate stand.')
print('T2 applied')
PY
```

- [ ] **Step 2: Check no stale figure or rule remains**

<!-- check: T2 -->
```bash
cd ~/Moriarty && grep -n "4\.2 GiB\|8\.0 GiB\|4\.3 GiB\|8\.2 GiB\|predecessorAllowList\|≤ 20\|four inconsistent\|Poseidon commitments\|not adopted" deliverables/pcd-midnight-native-2026-09-11/REPORT.md deliverables/pcd-midnight-native-2026-09-11/moriarty-pcd-on-midnight.html openspec/PCD-ROADMAP-2026-09-11.md; echo "grep exit $? (1 means clean)"
```
Expected: no matches, `grep exit 1`.

---

### Task 3: PCD change package and coverage rows

**Files:** Create `openspec/changes/pcd-ledger-anchored-acceptance/{README.md,proposal.md,design.md,tasks.md}` and `specs/{pcd-ledger-correspondence,pcd-mandatory-acceptance,pcd-composition,pcd-certificates}/spec.md`. Modify `openspec/sprints/coverage.json`.

**Interfaces:** Produces eighteen requirement headings. Later tasks cite these exact headings:
`Declared ledger verification seam`, `Fused step relation per entry point`, `Head read-then-write discipline`, `Immutable authority deployment audit`, `Measured bounds frozen into the program digest`, `Claim discharge map`, `Constrained genesis and termination`, `Intent digest v2 and program digest`, `Observation freshness at application`, `Forward-declared migration replaces in-place revocation`, `Governed deploy-time property certificate`, `Ledger-atomic split and join`, `Cross-contract release with reclaim`, `Recipient-keyed successor handoff`, `Composition operators under head discipline`, `Bounded native certificates`, `Off-ledger segment certificate`, `Recursion dependency tracking and stop`.

- [ ] **Step 1: Write the failing check.** Before the files exist, the clean-copy harness still passes (no headings). The failing condition is created by Step 2 alone (headings without rows); Step 3 closes it.

- [ ] **Step 2: Write the package files**

<!-- run: T3a -->
```bash
C=~/Moriarty/openspec/changes/pcd-ledger-anchored-acceptance
mkdir -p $C/specs/pcd-ledger-correspondence $C/specs/pcd-mandatory-acceptance $C/specs/pcd-composition $C/specs/pcd-certificates
cat > $C/README.md <<'EOF'
# PCD ledger-anchored acceptance

Status: specified-only. This package grants no admission, dispatch, resource or acceptance.

It adopts the [Midnight-native PCD decision](../../../deliverables/pcd-midnight-native-2026-09-11/REPORT.md) into MC03–MC06 under the [PCD integration amendment](../../PCD-INTEGRATION-2026-09-11.md).

- [Proposal](proposal.md)
- [Design](design.md)
- [Tasks](tasks.md)
- Requirements:
  - [Ledger correspondence, MC04](specs/pcd-ledger-correspondence/spec.md)
  - [Mandatory acceptance, MC05](specs/pcd-mandatory-acceptance/spec.md)
  - [Composition, MC06](specs/pcd-composition/spec.md)
  - [Certificates, MC03](specs/pcd-certificates/spec.md)
EOF
cat > $C/proposal.md <<'EOF'
# PCD ledger-anchored acceptance

## Why

Midnight verifies each contract-call proof against the operation key stored in contract state. Contract-call proofs use a Blake2b transcript, and `verify_proof` accepts only Poseidon zk-stdlib proofs. A contract call therefore cannot re-verify its predecessor's proof. The current MC03 → MC04 → MC05 chain makes mandatory Preview acceptance wait for a recursive route that cannot carry contract proofs and that needs the unreleased `ledger-10`.

This package changes the terminal evidence-gated decision in one way. On-ledger history compliance follows by induction from constrained genesis, immutable operation keys and head read-then-write discipline. Recursion serves only bounded certificates. The hard gate is unchanged: verification-enabled mandatory acceptance on Midnight Preview.

The user directed that Midnight recursion be assumed to reach production within a few months. Certificates therefore stay release scope.

## What changes

- Add eighteen requirements in four capabilities owned by MC03, MC04, MC05 and MC06.
- Re-root stage `f3` on the Stage 0 seam instead of `f1` and `f2`. Keep `f2` in `release`.
- Remove MC03 from the MC04 and MC05 dependency lists.
- Add in-place amendment notes to affected MC03–MC06 scenarios. Requirement headings and locked task lines do not change.

## Capabilities

### New capabilities
- `pcd-ledger-correspondence`: seam, fused step relation, head discipline, deploy audit and bounds freeze (MC04).
- `pcd-mandatory-acceptance`: claim discharge, genesis and termination, digests, freshness, migration and property certificate (MC05).
- `pcd-composition`: split and join, release and reclaim, handoff and operators (MC06).
- `pcd-certificates`: bounded native certificates, segment certificates and dependency stops (MC03).

### Modified capabilities
- None. MC03–MC06 carry in-place amendment notes in their unarchived change directories.

## Impact

Affected artifacts:
- `openspec/moriarty-completion-program.json`: `f3` prerequisites, native-track stage purposes, MC03 and MC04 fields, MC04 and MC05 dependencies, feasibility stage text and the RP02 reason.
- `openspec/sprints/`: `sprints.json`, `coverage.json`, `report-lessons.json`, `asset-study.json`, `package-task-map.json` titles, the new `pcd-integration.json`, `verify.py`, `test_verify.py`, SP04, SP06, SP09, SP10 and the sprint README.
- `openspec/PCD-INTEGRATION-2026-09-11.md`, `openspec/PCD-ROADMAP-2026-09-11.md`, `ROADMAP.md`, the Charter and the September 9 refinement note.
- MC03–MC06 READMEs and scenario notes, and MC04/MC05 dependency prose.

Non-goals:
- No proof, campaign, deployment or public transaction runs.
- No resource ceiling changes. Certificate campaigns need a separate user-approved resource amendment.
- The SP01 sprint document, the September 7 reconciliation and the AFK change package are not edited.
- Site copy, `typed-schemas.md` and `bounds.json` are follow-ups.

The supersession and locked-task tables are in the [PCD integration amendment](../../PCD-INTEGRATION-2026-09-11.md#supersessions).
EOF
cat > $C/design.md <<'EOF'
# Design: PCD ledger-anchored acceptance

Dependencies: MC01 for the language profile, MC02 for integrated financial effects, and the Midnight dependency tracker in `openspec/sprints/pcd-integration.json`.

## Inputs

- Decision report `deliverables/pcd-midnight-native-2026-09-11/REPORT.md`, digest-pinned in `pcd-integration.json`.
- PCD roadmap `openspec/PCD-ROADMAP-2026-09-11.md`, digest-pinned the same way.
- Reproduced measurements `evidence/pcd-midnight-native-2026-09-11/MEASUREMENTS.md`.

## Outputs

Specified-only planning records. Implementation paths are fixed at `native-path-freeze` and in each sprint execution packet.

## Interfaces

| Interface | Definition | Owner |
|---|---|---|
| Verification seam | Ledger `well_formed` checks each contract-call proof against the operation key read from `ContractState.operations` | MC04 |
| Step relation | One circuit per entry point: authorization, transition validity, effect projection, intent refinement and per-step invariant | MC04 |
| Head | `(instanceId, headId, revision, stateCommit, lifecycle, Π_P, netTag)`; every write follows a read of the same head in one transcript section | MC04 |
| Deploy audit | Recomputes address, operation set, keys, `Uninit` state and authority: committee `[]`, threshold ≥ 1, counter 0 | MC04 |
| Program digest Π_P | Core, bounds, property-certificate hash, claim sets, toolchain versions and a successor allowlist of program digests | MC05 |
| Intent digest v2 | Exact-head and outcome modes | MC05 |
| Entry points | `Initialize`, `Step`, `Split`, `Join`, `Terminate`, optional `Pause`; cross-contract `Release`, `JoinFrom`, `Migrate`, `ImportFrom`, `Reclaim` | MC05, MC06 |
| Certificate entry point | `VerifyProof` and `InnerProof` with guard constant 1; ledger-side accumulator pairing on `ledger-10` | MC03 |

## Trust boundaries

- The ledger is trusted to verify proofs against stored keys, to check `Popeq` reads at application and to apply one intent atomically.
- The deploy audit is trusted to reject a deployment whose keys or authority differ from a reproducible build. Relying parties run it on counterparties.
- Oracle truth, custody and witness availability stay external assumptions.
- Claimed cross-contract calls are atomic in one direction only.
- `netTag` separates networks. It does not separate byte-identical replicated deployments.

## Semantic guarantees and feasibility

Semantic guarantees come from the step relation, head discipline and induction over ledger acceptance. Feasibility comes from experiments E1–E5 and the bounds freeze. A failed experiment stops its stage. It never relaxes an acceptance requirement.

## Failure handling

- E1 fails: stop Stages 2–6 until a reviewed consumption-set design replaces head discipline.
- E2 fails: revisit relation decomposition within k ≤ 17.
- `ledger-10` changes formats: pin the released formats and rerun E3.
- Recursion slips: certificate stages record `blocked`. The core and the Preview gate stand.
- The certificate resource amendment is refused: certificate stages stay blocked.
EOF
cat > $C/tasks.md <<'EOF'
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
EOF
cat > $C/specs/pcd-ledger-correspondence/spec.md <<'EOF'
## ADDED Requirements

### Requirement: Declared ledger verification seam
Moriarty acceptance SHALL count a contract-call proof as verified only when ledger `well_formed` checks it against the operation key read from `ContractState.operations`. A toolchain manifest SHALL pin Compact, ZKIR, ledger and proof-server versions for each network generation: ledger 8, ledger 9 and `ledger-10`.

#### Scenario: Ledger-verified call
- **WHEN** a Moriarty call is accepted on a pinned network generation
- **THEN** its evidence records the operation key read from contract state, the toolchain manifest entry and the ledger acceptance result.

#### Scenario: Substituted verification
- **WHEN** evidence offers a host verdict, a mock verifier, an in-circuit pairing claim or a proof checked against a key not read from contract state
- **THEN** the evidence is rejected and the requirement stays open.

### Requirement: Fused step relation per entry point
Each Moriarty entry point SHALL compile to one circuit that proves authorization, transition validity, effect correspondence, intent refinement and the per-step invariant together. The native effects SHALL equal the projection of the Moriarty effects. If experiment E2 shows the funded repayment step cannot fit k ≤ 17 in any of its three variants, work SHALL stop and revisit relation decomposition without relaxing acceptance requirements.

#### Scenario: Fused proof verifies
- **WHEN** the proof server proves a funded repayment step
- **THEN** one proof verifies against the compiled key and binds every effect, recipient, asset and cap.

#### Scenario: Unbound field or separate claim proofs
- **WHEN** an effect, recipient, asset or cap is mutated, or a design proves each claim of one action in a separate proof
- **THEN** proving or verification fails for the mutation, and review rejects the separate-proof design.

#### Scenario: Fit failure
- **WHEN** no E2 variant fits k ≤ 17
- **THEN** the step relation is not promoted and the acceptance requirements stay unchanged.

### Requirement: Head read-then-write discipline
Every head write SHALL follow a read of that head in the same transcript section, with no checkpoint between them. Head creation SHALL follow an absence read. A checker over generated ZKIR SHALL enforce both rules. Experiment E1 on Preview SHALL show that at most one of two individually valid conflicting calls from one head applies.

#### Scenario: Conflicting calls
- **WHEN** two individually valid calls consume the same head on Preview
- **THEN** at most one applies, and the other fails its read at application.

#### Scenario: Blind write
- **WHEN** generated ZKIR writes a head without the preceding read, or creates a head without an absence read
- **THEN** the checker rejects the build.

#### Scenario: E1 fails
- **WHEN** E1 shows both conflicting calls applied
- **THEN** Stages 2–6 stop until a reviewed explicit consumption-set design replaces head discipline.

### Requirement: Immutable authority deployment audit
Every deployment used for acceptance evidence SHALL pass a deploy audit. The audit SHALL recompute the contract address and check the exact operation set, every key against a reproducible build, the initial state `Uninit(Π_P, netTag)` and the maintenance authority: committee `[]`, threshold ≥ 1, counter 0. Deployments SHALL use a custom deploy path, because midnight-js installs a one-key committee by default. Tooling that requires a one-key committee SHALL NOT gate these deployments.

#### Scenario: Audited deployment
- **WHEN** a Moriarty contract is deployed to Preview through the custom path
- **THEN** the audit reproduces its address and keys and records an empty committee with threshold at least 1.

#### Scenario: Weak or altered deployment
- **WHEN** a deployment has an extra operation, a key mismatch, a non-`Uninit` initial state, threshold 0 or a non-empty committee
- **THEN** the audit fails and no acceptance is claimed for that deployment.

### Requirement: Measured bounds frozen into the program digest
Final bounds SHALL come from the report's section 14.4 microbenchmarks on one pinned machine plus browser WASM measurements. The frozen set SHALL cover join fan-in, branch fan-out, certificates per call, segment length, effects, assets and obligations per step, public inputs per call, core step k, proving time and memory, and verifier work. Frozen bounds SHALL enter `bounds.json`, owned by MC01, and Π_P as a new program-digest version.

#### Scenario: Frozen profile
- **WHEN** every bound has a retained benchmark
- **THEN** the profile's Π_P binds all bounds and campaigns may rely on them.

#### Scenario: Unmeasured bound
- **WHEN** a bound lacks a retained benchmark or Π_P omits it
- **THEN** the bound is not frozen and no campaign may rely on it.
EOF
cat > $C/specs/pcd-mandatory-acceptance/spec.md <<'EOF'
## ADDED Requirements

### Requirement: Claim discharge map
Acceptance SHALL discharge all four mandatory claim families at declared loci. ContractInvariant SHALL be discharged by the deploy-time property certificate and the per-step invariant. IntentRefinement and TransitionValidity SHALL be discharged by the fused step relation. HistoryCompliance SHALL be discharged by ledger applicability plus provenance: induction on ledger, or a certificate off ledger. One canonical claim vocabulary SHALL be used, with historical aliases recorded.

#### Scenario: Induction conditions hold
- **WHEN** a deployment has immutable keys, constrained genesis and a passing head-discipline check
- **THEN** HistoryCompliance for its on-ledger heads is discharged by induction over ledger acceptance.

#### Scenario: Induction condition missing
- **WHEN** immutable keys, constrained genesis or head discipline is not established for a deployment
- **THEN** HistoryCompliance is not discharged by induction and acceptance is not claimed for that deployment.

### Requirement: Constrained genesis and termination
Deployment SHALL write `Uninit(Π_P, netTag)`. `Initialize` SHALL prove the genesis predicate with all-principal signatures, succeed at most once and derive `instanceId = H(A ‖ H(G))`. `Terminate` SHALL move a `Live` head to `Terminated` only under the contract rules' terminal condition or the principal threshold declared in Π_P, and SHALL record a disposition for every residual obligation.

#### Scenario: Genesis and terminal step
- **WHEN** all principals sign a valid genesis body and later a permitted terminal condition holds
- **THEN** `Initialize` creates the instance once and `Terminate` records every residual obligation's disposition.

#### Scenario: Invalid lifecycle action
- **WHEN** a second `Initialize` runs, a principal signature is missing, the deployment is not in `Uninit`, any action targets a `Terminated` head, or `Terminate` leaves an obligation without a disposition
- **THEN** the call rejects.

### Requirement: Intent digest v2 and program digest
Canonical encodings and test vectors SHALL exist for Π_P, intent digest v2 and genesis body v2. Exact-head mode SHALL bind netTag, contract, entry point, instance, head, revision, state commitment, Π_P, action and arguments, caps, observation policy, certificate requirements and validity window; its nonce is informational. Outcome mode SHALL bind the constraint set, a program-digest allowlist, the validity window and a nonce recorded in a per-instance consumed-nonce set. Π_P bounds SHALL be checked before proving.

#### Scenario: Canonical vectors
- **WHEN** an implementation encodes the published test vectors
- **THEN** it reproduces every digest byte for byte.

#### Scenario: Altered or replayed intent
- **WHEN** a bound field is altered, an outcome nonce is reused, an exact-head intent is replayed after the revision increments, or an input exceeds a Π_P bound
- **THEN** acceptance rejects before applying effects, and a bound violation rejects before expensive proving.

### Requirement: Observation freshness at application
Observations SHALL be signed by keys in `σ.oracleKeys` and verified in the step relation. The feed allowlist and `maxObservationAge` SHALL be part of the observation policy bound in the intent digest. Freshness SHALL use block-time reads in the transcript that the step relation proves and the ledger re-executes at application.

#### Scenario: Fresh observation
- **WHEN** a signed observation from an allowed feed is within its maximum age at application
- **THEN** the call applies.

#### Scenario: Stale observation
- **WHEN** an observation is older than its maximum age at application on a local ledger node or on Preview
- **THEN** application rejects, and an offline proof verification is not evidence of freshness.

### Requirement: Forward-declared migration replaces in-place revocation
Operation keys SHALL be immutable. Replacement SHALL happen only through `Migrate` to a successor with a strictly greater version, under one of two branches. In the declared branch, Π_new SHALL be in the successor allowlist compiled into Π_old, which holds program digests only, and the migration threshold declared in Π_old SHALL sign netTag, A_old, A_new, Π_new, revision and state. In the unanimous branch, all principals SHALL sign `MORIARTY-MIGRATE-v2` over the same fields. The successor SHALL record its predecessor address in its deploy-time state `Uninit(Π_new, netTag, A_old)`. `ImportFrom` SHALL accept only a claimed `Migrate` call from that address whose commitment is absent from its imported set. An optional principal-threshold `Pause` entry point MAY halt new steps without changing keys. Cross-contract evidence requires ledger 9, and its Preview qualification is a tracked dependency.

#### Scenario: Declared migration
- **WHEN** principals audit A_new, sign a declared migration and submit `Migrate` with the claimed `ImportFrom`
- **THEN** the old head enters `Releasing`, the new head keeps obligations and budgets, and consumed state stays consumed.

#### Scenario: Unauthorized or unsafe migration
- **WHEN** a maintenance update, key substitution, downgrade, successor neither declared nor unanimously signed, import from a caller other than the recorded predecessor, second import of one commitment, or revival of consumed authority is attempted
- **THEN** the ledger or the step relation rejects it.

### Requirement: Governed deploy-time property certificate
The deploy audit SHALL check the universal ContractProperty certificate and act as the governed registration mechanism. The certificate hash SHALL be bound in Π_P. A ledger-checked `Initialize` certificate MAY replace the audit check once `ledger-10` certificates are accepted.

#### Scenario: Checked certificate
- **WHEN** the deploy audit verifies the property certificate against Π_P
- **THEN** the deployment may count ContractInvariant as discharged at deploy time.

#### Scenario: Hash without a check
- **WHEN** Π_P carries a certificate hash but the certificate is absent or fails its check
- **THEN** the audit fails and ContractInvariant is not discharged.
EOF
cat > $C/specs/pcd-composition/spec.md <<'EOF'
## ADDED Requirements

### Requirement: Ledger-atomic split and join
`Split` and `Join` within one contract SHALL be ledger-atomic. Child head identities SHALL be computed in the circuit. Budgets SHALL be conserved, obligations partitioned and authority attenuated. The lifetime invariant SHALL hold per head: `revision + remaining = allocatedLifetime(head)`, fixed when the head is created. `Join` SHALL require distinct heads and compatible policies. Fan-in and fan-out SHALL be at most 2 until the bounds freeze sets them.

#### Scenario: Split then join
- **WHEN** head A splits into B and C and they later join into D
- **THEN** budgets, residual obligations and authority are conserved and each head's lifetime invariant holds.

#### Scenario: Composition attack
- **WHEN** a join repeats a head, mixes policies, restores budget, drops an obligation, or exceeds fan-in or fan-out
- **THEN** the call rejects.

### Requirement: Cross-contract release with reclaim
Cross-contract `Release`, `JoinFrom` and `Reclaim` SHALL use claimed calls from the Compact 0.33 toolchain on ledger 9. An unclaimed `Release` applies alone and SHALL leave the head `Releasing`. `Reclaim` SHALL return a `Releasing` head to `Live` only when it claims, in the same intent, a target-side call that reads the release commitment as absent from the target's imported set and marks it dead there. Experiment E4 SHALL confirm this rule.

#### Scenario: Recovered unclaimed release
- **WHEN** a `Release` lands without its `JoinFrom`
- **THEN** the head stays `Releasing` until a `Reclaim` with the claimed target-side absence call restores it.

#### Scenario: Unsafe release or reclaim
- **WHEN** a commitment is imported twice, a `Reclaim` lacks the claimed absence call, a `Reclaim` follows an import, or a `JoinFrom` claims a missing `Release`
- **THEN** the call rejects.

### Requirement: Recipient-keyed successor handoff
A successor SHALL prove from the head commitment and a recipient-encrypted opening, with no predecessor proof or witness. Per-party sub-state commitments SHALL be used where parties differ. Isolation evidence SHALL use distinct OS users or containers.

#### Scenario: Independent successor
- **WHEN** Bob receives only the opening addressed to him
- **THEN** Bob proves the next step and the audit shows Bob never held Alice's secret fields.

#### Scenario: Tampered opening
- **WHEN** an opening is altered or addressed to another recipient
- **THEN** proving fails and no step applies.

### Requirement: Composition operators under head discipline
Sequential composition SHALL map to `Step`, disjoint parallel composition to `Split`, and atomic synchronization to same-contract `Join` or a claimed cross-contract call. Shared-state interleaving, asynchronous messaging and Pending SHALL receive a reviewed rule under head discipline before any SP10 campaign advertises them.

#### Scenario: Mapped operators
- **WHEN** a composition uses sequential, disjoint parallel or atomic synchronization
- **THEN** it runs through the mapped entry points under the same acceptance lineage.

#### Scenario: Operator without a rule
- **WHEN** an operator has no accepted rule
- **THEN** it is unavailable, is not advertised, and MC06 stays open.
EOF
cat > $C/specs/pcd-certificates/spec.md <<'EOF'
## ADDED Requirements

### Requirement: Bounded native certificates
Certificate relations SHALL be Poseidon zk-stdlib relations verified through `VerifyProof` and `InnerProof` with guard constant 1, with each accumulator pairing checked by ledger `well_formed` on `ledger-10`. Contract-call proofs SHALL NOT be used as inner proofs. The outer relation SHALL constrain `vk_repr` and the state decider. Certificate campaigns SHALL NOT be admitted before a reviewed resource amendment, because the measured outer circuits need k = 18 for one level and k = 19 for two levels, above the Charter's k ≤ 17 ceiling.

#### Scenario: Accepted certificate
- **WHEN** a certificate-bearing call runs on a pinned `ledger-10` devnet under an approved resource amendment
- **THEN** the ledger accepts the call and its accumulator pairing checks.

#### Scenario: Unsound certificate
- **WHEN** a guard is free or mismatched, `vk_repr` is substituted, an inner instance is unbound, or an inner proof is tampered or belongs to another key
- **THEN** the ledger rejects the call or the Moriarty lint rejects the build, and an outer proof over an invalid inner proof is not evidence.

### Requirement: Off-ledger segment certificate
Experiment E5 SHALL produce segment certificates over the Moriarty step at 1, 10 and 100 steps, with segment length at most 16 until benchmarked. Retained bytes SHALL be verified in a fresh process and imported through a certificate entry point.

#### Scenario: Verified segment
- **WHEN** a 10-step segment certificate is retained
- **THEN** a fresh process verifies it and the certificate entry point imports its final state.

#### Scenario: Substituted segment evidence
- **WHEN** evidence offers a host hash chain, a MockProver run, a nonrecursive re-proof or an altered segment
- **THEN** it is not segment evidence and the requirement stays open.

### Requirement: Recursion dependency tracking and stop
Certificate campaigns SHALL NOT dispatch until each tracker item they need holds and is pinned: pull request 738 merged and `ledger-10` released, accumulator fee accounting, k ≥ 18 parameters served, released formats pinned and a reviewed resource amendment. The tracker SHALL also carry ledger 9 on Preview for `mandatory` and `composition`. If recursion slips, certificate stages SHALL record `blocked`, the core requirements and the Preview gate SHALL stand, and no scope SHALL be dropped without new user direction.

#### Scenario: Tracker satisfied
- **WHEN** every tracker item for a certificate stage holds and is pinned
- **THEN** the stage may seek campaign admission.

#### Scenario: Unsatisfied tracker item
- **WHEN** a dispatch is attempted while a required tracker item is open
- **THEN** dispatch is refused and the stage records `blocked`.
EOF
echo T3a written
```

- [ ] **Step 3: Confirm the crosswalk now fails, then add coverage rows**

<!-- check: T3-red -->
```bash
/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad/validate-clean-copy.sh 2>&1 | head -3
```
Expected: `Requirement crosswalk differs` naming the eighteen pcd headings.

<!-- run: T3b -->
```bash
cd ~/Moriarty && python3 - <<'PY'
import sys; sys.path.insert(0, '/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
from pcdlib import load_json, dump_json
base = 'openspec/changes/pcd-ledger-anchored-acceptance/specs/'
rows = [
 ('pcd-ledger-correspondence', 'Declared ledger verification seam', 'MC04', ['SP01', 'SP04', 'SP09'], 'SP09'),
 ('pcd-ledger-correspondence', 'Fused step relation per entry point', 'MC04', ['SP09'], 'SP09'),
 ('pcd-ledger-correspondence', 'Head read-then-write discipline', 'MC04', ['SP09'], 'SP09'),
 ('pcd-ledger-correspondence', 'Immutable authority deployment audit', 'MC04', ['SP05', 'SP09'], 'SP09'),
 ('pcd-ledger-correspondence', 'Measured bounds frozen into the program digest', 'MC04', ['SP09', 'SP11'], 'SP11'),
 ('pcd-mandatory-acceptance', 'Claim discharge map', 'MC05', ['SP01', 'SP09'], 'SP09'),
 ('pcd-mandatory-acceptance', 'Constrained genesis and termination', 'MC05', ['SP09'], 'SP09'),
 ('pcd-mandatory-acceptance', 'Intent digest v2 and program digest', 'MC05', ['SP01', 'SP08', 'SP09'], 'SP09'),
 ('pcd-mandatory-acceptance', 'Observation freshness at application', 'MC05', ['SP09'], 'SP09'),
 ('pcd-mandatory-acceptance', 'Forward-declared migration replaces in-place revocation', 'MC05', ['SP09'], 'SP09'),
 ('pcd-mandatory-acceptance', 'Governed deploy-time property certificate', 'MC05', ['SP09'], 'SP09'),
 ('pcd-composition', 'Ledger-atomic split and join', 'MC06', ['SP10'], 'SP10'),
 ('pcd-composition', 'Cross-contract release with reclaim', 'MC06', ['SP10'], 'SP10'),
 ('pcd-composition', 'Recipient-keyed successor handoff', 'MC06', ['SP10'], 'SP10'),
 ('pcd-composition', 'Composition operators under head discipline', 'MC06', ['SP10', 'SP11'], 'SP10'),
 ('pcd-certificates', 'Bounded native certificates', 'MC03', ['SP04', 'SP06'], 'SP04'),
 ('pcd-certificates', 'Off-ledger segment certificate', 'MC03', ['SP06'], 'SP06'),
 ('pcd-certificates', 'Recursion dependency tracking and stop', 'MC03', ['SP01', 'SP04', 'SP06'], 'SP04'),
]
cov = load_json('openspec/sprints/coverage.json')
assert not any(r['spec'].startswith(base) for r in cov['requirements'])
for cap, name, pkg, sprints, primary in rows:
    cov['requirements'].append({'spec': f'{base}{cap}/spec.md', 'requirement': name, 'package': pkg, 'sprints': sprints,
                                'primaryClosingSprint': primary, 'contributingSprints': [s for s in sprints if s != primary]})
dump_json('openspec/sprints/coverage.json', cov)
print('rows', len(cov['requirements']))
PY
```

- [ ] **Step 4: Verify**

<!-- check: T3 -->
```bash
cd ~/Moriarty && openspec validate --all --strict 2>&1 | tail -3
/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad/validate-clean-copy.sh
```
Expected: `Totals: 11 passed, 0 failed`; `rows 79`; verify.py pass with `"requirements": 79`; `8 passed`.

---

### Task 4: In-place amendment notes and MC prose

**Files:** Modify the four MC03–MC06 `spec.md` and `README.md` files; `mc04…/design.md`, `mc04…/tasks.md`, `mc05…/design.md`, `mc05…/tasks.md` (prose lines only).

**Interfaces:** Consumes the Task 3 headings.

- [ ] **Step 1: Apply notes and prose**

<!-- run: T4 -->
```bash
cd ~/Moriarty && python3 - <<'PY'
import sys; sys.path.insert(0, '/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
from pcdlib import read, write, rep
CH = 'openspec/changes/'
PK = {'MC03': 'mc03-native-recursive-proof', 'MC04': 'mc04-ledger-correspondence-and-consumption',
      'MC05': 'mc05-mandatory-claim-acceptance', 'MC06': 'mc06-private-handoff-and-composition'}
def note(pkg, requirement, scenario, governing, sentence):
    path = f'{CH}{PK[pkg]}/specs/{PK[pkg]}/spec.md'
    lines = read(path).split('\n')
    r = lines.index(f'### Requirement: {requirement}')
    end_req = next((i for i in range(r + 1, len(lines)) if lines[i].startswith('### ')), len(lines))
    s = lines.index(f'#### Scenario: {scenario}', r, end_req)
    end = next((i for i in range(s + 1, len(lines)) if lines[i].startswith('#### ') or lines[i].startswith('### ')), len(lines))
    last = max(i for i in range(s, end) if lines[i].strip())
    quoted = ' and '.join(f'"{g}"' for g in governing)
    word = 'requirement' if len(governing) == 1 else 'requirements'
    lines.insert(last + 1, f'- **Amended by** `pcd-ledger-anchored-acceptance` {word} {quoted}: {sentence}')
    write(path, '\n'.join(lines))
note('MC03', 'Reviewed restart', 'Checked smaller encoding', ['Off-ledger segment certificate'], 'the fixed 54-limb relation and its smaller-encoding retry are retired; SP06 reviews the segment relation instead.')
note('MC03', 'Real recursion and independent verification', 'Valid episode', ['Off-ledger segment certificate'], 'the fixed two-step episode becomes segment certificates at 1, 10 and 100 steps, still verified from retained bytes in a fresh process.')
note('MC03', 'Real recursion and independent verification', 'Invalid proof context', ['Bounded native certificates'], 'the rejection list also covers free or mismatched guards, substituted `vk_repr` and unbound inner instances.')
note('MC03', 'Complete backend decision before native dispatch', 'Report requirement omitted', ['Recursion dependency tracking and stop'], 'the backend decision is the certificate route, gated by the Midnight dependency tracker and a reviewed resource amendment.')
note('MC04', 'Actual verifier compatibility', 'Missing interface', ['Declared ledger verification seam'], 'the core interface is ledger `well_formed` against the operation key in contract state, so the core is not interface-blocked; host assertions and mocked verification still cannot substitute.')
note('MC04', 'Durable one-time consumption', 'Duplicate authority', ['Head read-then-write discipline'], 'unique consumption is enforced by reading each head before writing it, checked over generated ZKIR and by E1.')
note('MC04', 'Early complete verifier feasibility', 'Report requirement omitted', ['Declared ledger verification seam', 'Bounded native certificates'], 'early feasibility is the Stage 0 seam for the core and E3 for certificates; no in-circuit pairing is required.')
note('MC05', 'Mandatory acceptance', 'Downgrade attack', ['Immutable authority deployment audit', 'Claim discharge map', 'Forward-declared migration replaces in-place revocation'], 'claims are compiled into immutable operation keys, so a stripped claim or arbitrary verifier needs a different key and fails the deploy audit.')
note('MC05', 'Intent refinement and complete effects', 'Authorized route choice', ['Intent digest v2 and program digest'], 'route choice uses outcome mode, which binds a program-digest allowlist and a consumed nonce.')
note('MC05', 'No circular or simulated evidence', 'Bound outcome claims', ['Intent digest v2 and program digest'], 'the signature binds digest v2 fields, and the head read replaces predecessor lists.')
note('MC05', 'Authority covers liabilities and all protected effects', 'Revoked verifier or inactive specification', ['Forward-declared migration replaces in-place revocation'], 'keys cannot be revoked in place; a defective program is halted by `Pause` and replaced by migration, which still cannot restore consumed state or reset lifecycle authority.')
note('MC05', 'Authority covers liabilities and all protected effects', 'Oversized verification input', ['Intent digest v2 and program digest', 'Measured bounds frozen into the program digest'], 'the registered budget is the Π_P bound set, checked before proving; on-chain sidecar bytes are zero.')
note('MC06', 'Bounded history composition', 'Compatible composition', ['Ledger-atomic split and join', 'Cross-contract release with reclaim'], 'on-ledger branches need no branch proofs; off-ledger branches join only through certificates.')
note('MC06', 'Composition operators and witness ownership', 'Report requirement omitted', ['Recipient-keyed successor handoff', 'Composition operators under head discipline'], 'on-ledger successors need no predecessor proof verification, and handoff carries recipient-encrypted openings.')
for pkg in PK.values():
    rep(f'{CH}{pkg}/README.md', 'Status: specified-only.\n', 'Status: specified-only.\n\nAmended by [pcd-ledger-anchored-acceptance](../pcd-ledger-anchored-acceptance/README.md) under the [PCD integration amendment](../../PCD-INTEGRATION-2026-09-11.md).\n')
m4, m5 = f'{CH}{PK["MC04"]}', f'{CH}{PK["MC05"]}'
rep(f'{m4}/design.md', 'Dependencies: MC01, MC02, MC03.', 'Dependencies: MC01, MC02. The PCD integration removed MC03; the ledger-anchored core takes no MC03 proof.')
rep(f'{m4}/tasks.md', '**Dependencies:** MC01, MC02, MC03.', '**Dependencies:** MC01, MC02.')
rep(f'{m5}/design.md', 'Dependencies: MC03, MC04.', 'Dependencies: MC04. The PCD integration removed MC03; mandatory acceptance takes no MC03 proof.')
rep(f'{m5}/tasks.md', '**Dependencies:** MC03, MC04, plus', '**Dependencies:** MC04, plus')
for f in (f'{m4}/design.md', f'{m4}/tasks.md'):
    rep(f, 'Establish the retained MC03 verifier interface before extending the relation.', 'Use the declared ledger verification seam; no MC03 proof is an input.')
print('T4 applied')
PY
```

- [ ] **Step 2: Verify**

<!-- check: T4 -->
```bash
cd ~/Moriarty && grep -c "Amended by" openspec/changes/mc0[3-6]-*/specs/*/spec.md && openspec validate --all --strict 2>&1 | tail -1
/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad/validate-clean-copy.sh
```
Expected: MC03 4, MC04 3, MC05 5, MC06 2; `Totals: 11 passed, 0 failed`; verify.py pass (locked task lines untouched); `8 passed`.

---

### Task 5: Integration amendment, crosswalk and validator guard

**Files:** Create `$S/build_pcd_integration.py`, `openspec/PCD-INTEGRATION-2026-09-11.md`, `openspec/sprints/pcd-integration.json`. Modify `openspec/sprints/test_verify.py`, `openspec/sprints/verify.py`.

**Interfaces:**
- Consumes Task 1 Register values, Task 2 final REPORT.md/PCDR bytes, Task 3 headings.
- Produces `pcd-integration.json` keys `schemaVersion`, `status`, `amendment`, `stageMapping[{id,title,stage,tasks}]`, `experiments[{id,title,stage,tasks}]`, `supersessions[{spec,requirement,scenario,disposition,note,governing}]`, `lockedTasks[{package,taskId,disposition,note,governing}]`, `dependencyTracker[{item,neededBy,status,class,observed}]`, `decisions[{id,question,default,owner,blocks,status}]`, `sourceDigests{path: sha256}`.
- Diagnostics: `Stale PCD integration source`, `PCD core depends on certificate stage`, `PCD release lost certificate stage`, `Unknown PCD crosswalk reference`.
- The builder is rerunnable. When `$S/result-review.txt` exists, its text replaces the pending result-review line.

- [ ] **Step 1: Write and run the builder**

<!-- run: T5a -->
```bash
S=/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad
cat > $S/build_pcd_integration.py <<'PY'
import json, pathlib, sys
sys.path.insert(0, '/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
from pcdlib import dump_json, write, sha256, load_json
S = pathlib.Path('/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
SPEC = {k: f'openspec/changes/{v}/specs/{v}/spec.md' for k, v in {
    'MC03': 'mc03-native-recursive-proof', 'MC04': 'mc04-ledger-correspondence-and-consumption',
    'MC05': 'mc05-mandatory-claim-acceptance', 'MC06': 'mc06-private-handoff-and-composition'}.items()}
SEAM, FUSED, HEAD, AUDIT, BOUNDS = 'Declared ledger verification seam', 'Fused step relation per entry point', 'Head read-then-write discipline', 'Immutable authority deployment audit', 'Measured bounds frozen into the program digest'
CLAIMS, GENESIS, DIGEST, FRESH, MIGRATE, PROPERTY = 'Claim discharge map', 'Constrained genesis and termination', 'Intent digest v2 and program digest', 'Observation freshness at application', 'Forward-declared migration replaces in-place revocation', 'Governed deploy-time property certificate'
SPLIT, RELEASE, HANDOFF, OPERATORS = 'Ledger-atomic split and join', 'Cross-contract release with reclaim', 'Recipient-keyed successor handoff', 'Composition operators under head discipline'
CERT, SEGMENT, TRACKER = 'Bounded native certificates', 'Off-ledger segment certificate', 'Recursion dependency tracking and stop'
def row(i, title, stage, tasks): return {'id': i, 'title': title, 'stage': stage, 'tasks': tasks}
stage_mapping = [
    row('PCD-S0', 'Pin the verifier boundary', 'f0', ['SP01.4']),
    row('PCD-S0-paths', 'Path ownership before authorship', 'native-path-freeze', ['SP01.5']),
    row('PCD-S1', 'One native transition proof', 'f3', ['SP09.1']),
    row('PCD-S2', 'One authenticated predecessor extension', 'f3', ['SP09.1']),
    row('PCD-S3', 'Ledger-enforced acceptance on Preview', 'f3', ['SP09.1']),
    row('PCD-S4', 'Sequential history and termination', 'mandatory', ['SP09.2', 'SP09.5']),
    row('PCD-S5', 'Cross-party successor', 'composition', ['SP10.2']),
    row('PCD-S6', 'Branch, join, release and reclaim', 'composition', ['SP10.3']),
    row('PCD-S6-migration', 'Forward-declared migration', 'mandatory', ['SP09.3']),
    row('PCD-S7-authorship', 'Certificate-relation authorship', 'f0a', ['SP04.1']),
    row('PCD-S7-fixtures', 'Independent certificate fixtures', 'f1-fixtures', ['SP04.2']),
    row('PCD-S7-certificate', 'Native certificate on ledger-10', 'f1', ['SP04.3']),
    row('PCD-S7-segment', 'Off-ledger segment certificate', 'f2', ['SP06.1', 'SP06.2', 'SP06.3']),
    row('PCD-S8', 'Performance and profile freeze', 'finance', ['SP11.1']),
]
experiments = [
    row('E1', 'Head read-then-write linearity on Preview', 'f3', ['SP09.1']),
    row('E2', 'Fused step relation fit', 'f3', ['SP09.1']),
    row('E3', 'Native certificate with negative controls on ledger-10', 'f1', ['SP04.3']),
    row('E4-migration', 'Immutable keys and forward-declared migration', 'mandatory', ['SP09.3']),
    row('E4-reclaim', 'Cross-contract release and reclaim', 'composition', ['SP10.3']),
    row('E5', 'Off-ledger segment certificate at 1, 10 and 100 steps', 'f2', ['SP06.2', 'SP06.3']),
]
def sup(pkg, req, scen, disp, note, gov): return {'spec': SPEC[pkg], 'requirement': req, 'scenario': scen, 'disposition': disp, 'note': note, 'governing': gov}
supersessions = [
    sup('MC03', 'Reviewed restart', 'Checked smaller encoding', 'removed', 'The fixed 54-limb relation and its smaller-encoding retry are retired (decision PD7).', [SEGMENT]),
    sup('MC03', 'Real recursion and independent verification', 'Valid episode', 'mechanism-replaced', 'The fixed two-step episode becomes E5 segment certificates at 1, 10 and 100 steps.', [CERT, SEGMENT]),
    sup('MC03', 'Real recursion and independent verification', 'Invalid proof context', 'extended', 'Rejections add free or mismatched guards, substituted vk_repr and unbound inner instances.', [CERT]),
    sup('MC03', 'Complete backend decision before native dispatch', None, 'mechanism-replaced', 'The backend decision is the certificate route under the dependency tracker.', [TRACKER]),
    sup('MC04', 'Actual verifier compatibility', 'Missing interface', 'mechanism-replaced', 'The core is not interface-blocked; host assertions and mocked verification still cannot substitute.', [SEAM]),
    sup('MC04', 'Durable one-time consumption', 'Duplicate authority', 'mechanism-replaced', 'Unique consumption uses head read-then-write discipline and E1.', [HEAD]),
    sup('MC04', 'Early complete verifier feasibility', None, 'mechanism-replaced', 'Early feasibility is the Stage 0 seam for the core and E3 for certificates.', [SEAM, CERT]),
    sup('MC05', 'Mandatory acceptance', 'Downgrade attack', 'mechanism-replaced', 'Claims are compiled into immutable operation keys checked by the deploy audit.', [AUDIT, CLAIMS, MIGRATE]),
    sup('MC05', 'Intent refinement and complete effects', 'Authorized route choice', 'mechanism-replaced', 'Route choice uses outcome mode with a program-digest allowlist and consumed nonce.', [DIGEST]),
    sup('MC05', 'No circular or simulated evidence', 'Bound outcome claims', 'mechanism-replaced', 'The signature binds digest v2 fields; the head read replaces predecessor lists.', [DIGEST]),
    sup('MC05', 'Authority covers liabilities and all protected effects', 'Revoked verifier or inactive specification', 'mechanism-replaced', 'A defective or compromised program is stopped by Pause and migration, not key revocation (decision PD2).', [MIGRATE]),
    sup('MC05', 'Authority covers liabilities and all protected effects', 'Oversized verification input', 'mechanism-replaced', 'The registered budget is the Π_P bound set, checked before proving.', [DIGEST, BOUNDS]),
    sup('MC06', 'Bounded history composition', 'Compatible composition', 'mechanism-replaced', 'On-ledger branches need no branch proofs; off-ledger branches join through certificates.', [SPLIT, RELEASE]),
    sup('MC06', 'Composition operators and witness ownership', 'Report requirement omitted', 'mechanism-replaced', 'On-ledger successors need no predecessor proof verification; handoff carries recipient-encrypted openings.', [HANDOFF, OPERATORS]),
]
def lt(pkg, task, disp, note, gov): return {'package': pkg, 'taskId': task, 'disposition': disp, 'note': note, 'governing': gov}
locked = [
    lt('MC03', '1.1', 'removed', 'Retired with the fixed relation (decision PD7); SP06.1 reviews the segment relation encoding instead.', [SEGMENT]),
    lt('MC03', '2.1', 'mechanism-replaced', 'Preimage, arithmetic, context and malformed-genesis tests apply to the segment relation; limb-boundary tests are removed.', [GENESIS, SEGMENT]),
    lt('MC03', '2.2', 'retained', 'Applies to the Moriarty step folded by the segment.', [SEGMENT]),
    lt('MC03', '3.1', 'retained', 'The separate verifier process applies to certificates.', [CERT, SEGMENT]),
    lt('MC03', '3.2', 'retained', 'Fresh deserialization, key and SRS identity and accumulator obligations apply to certificates.', [CERT, SEGMENT]),
    lt('MC03', '4.2', 'mechanism-replaced', 'E5 runs 1, 10 and 100 steps under a reviewed resource amendment; stop on the first failed predicate is retained.', [SEGMENT, TRACKER]),
    lt('MC03', 'D.1', 'mechanism-replaced', 'The intake pins the released pull request 738 / ledger-10 formats.', [TRACKER]),
    lt('MC04', '1.1', 'retained', 'Becomes the Stage 0 toolchain manifest and seam.', [SEAM]),
    lt('MC04', '1.2', 'retained', 'Becomes the Stage 1 positive probe with mutated controls.', [FUSED]),
    lt('MC04', '4.1', 'mechanism-replaced', 'Conflict and restart tests run against head discipline and E1.', [HEAD]),
    lt('MC04', '4.2', 'mechanism-replaced', 'At most one finalized success follows from head discipline, shown by E1.', [HEAD]),
] + [lt('MC04', f'P.{i}', 'mechanism-replaced', 'Compact step circuits proved by the proof server and verified by a Rust harness; no MC03 proof input.', [FUSED]) for i in range(1, 8)] + [
    lt('MC04', 'D.1', 'mechanism-replaced', 'The intake outcome is the declared seam; the core needs no wrapper alternative.', [SEAM]),
    lt('MC04', 'D.2', 'retained', 'Adds immutable authority, the deploy audit and netTag.', [AUDIT, MIGRATE]),
    lt('MC05', '1.1', 'mechanism-replaced', 'Test vectors and mutations for Π_P, intent digest v2 and genesis body v2; certificate keys are circuit constants.', [DIGEST]),
    lt('MC05', '1.2', 'mechanism-replaced', 'Rejections cover every digest v2 field.', [DIGEST]),
    lt('MC05', '3.1', 'mechanism-replaced', 'Absent-proof and intent-invalid tests run against the fused step relation; no MC03 artifact is an input.', [FUSED, CLAIMS]),
    lt('MC05', '4.1', 'retained', 'Real evidence only for supported profiles.', [CLAIMS]),
    lt('MC05', 'D.1', 'mechanism-replaced', 'Signed intents are verified in the circuit with JubJub Schnorr; E2 measures fit.', [FUSED, DIGEST]),
    lt('MC06', '3.1', 'retained', 'Duplicate, policy, fan-in and amplification controls apply to heads.', [SPLIT]),
    lt('MC06', '3.2', 'mechanism-replaced', 'Ledger-atomic split and join; branch proofs only for off-ledger branches.', [SPLIT, RELEASE]),
    lt('MC06', '4.1', 'retained', 'Recovery scenarios add Reclaim.', [RELEASE]),
    lt('MC06', 'D.1', 'mechanism-replaced', 'Same-contract realization on ledger 8, cross-contract on ledger 9.', [SPLIT, RELEASE, HANDOFF]),
]
def dep(item, needed, status, cls): return {'item': item, 'neededBy': needed, 'status': status, 'class': cls, 'observed': '2026-09-11'}
tracker = [
    dep('Pull request 738 merged and ledger-10 released and deployed', 'f0a, f1, f2', 'Open; merge conditionally agreed', 'M3 (Midnight)'),
    dep('Fee accounting for accumulator public inputs and pairings', 'f1 exit, f2', 'Absent at the pull request head', 'M3 (Midnight)'),
    dep('Guard lint for VerifyProof and InnerProof', 'f0a', 'Absent; Moriarty lint covers it meanwhile', 'M2'),
    dep('Collapsed decider checking vk_repr and state', 'f0a, f2', 'Absent; the Moriarty outer relation covers it meanwhile', 'M2 or M3'),
    dep('k >= 18 parameters served through the proof server and data provider', 'f1, f2', 'Ceremony files published; serving path unverified', 'M1 or M2'),
    dep('Compact verifyProof frontend', 'f0a ergonomics', 'Draft MIP only; ZKIR hand emission meanwhile', 'M2'),
    dep('midnight-js and proof-server inner-proof plumbing; Poseidon proving for zk-stdlib relations', 'f1, f2', 'Absent; Moriarty Rust prover meanwhile', 'M2 (M0 workaround)'),
    dep('Published IVC module with instance and accumulator serialization', 'f2', 'Unpublished', 'M2'),
    dep('Recipient-keyed private-state handoff', 'composition', 'Absent; Moriarty encryption meanwhile', 'M2 (M0 workaround)'),
    dep('Network id in the contract statement', 'Residual replay risk', 'Absent; netTag separates networks but not byte-identical replicated deployments', 'M3 (optional)'),
    dep('Maintenance-authority threshold validation', 'Defense in depth', 'Absent; the deploy audit covers it', 'M3 (optional)'),
    dep('Ledger 9 on Preview', 'mandatory (SP09.3), composition (SP10.3)', 'Hard fork in node 2.1.0-beta.1; Preview runs ledger 8', 'M3 (Midnight)'),
    dep('Reviewed resource amendment for certificate k', 'f1, f2', 'Not requested; campaigns capped at k <= 17', 'Moriarty decision'),
]
def dec(i, q, default, owner, blocks, status): return {'id': i, 'question': q, 'default': default, 'owner': owner, 'blocks': blocks, 'status': status}
P = 'default-pending-confirmation'
decisions = [
    dec('PD1', 'Intent signing modes', 'Exact-head and outcome modes; cross-instance route choice owned by SP08 with MC05', 'MC05, SP08', 'Stage 1 digest', P),
    dec('PD2', 'Revocation and upgrade', 'Immutable keys, declared or unanimous migration and optional principal-threshold Pause; an all-principal committee with update delay only as a Π_P-declared option for long-lived profiles after review', 'MC05, SP09.3', 'Stage 3 deploy configuration; E4', P),
    dec('PD3', 'Where the ContractProperty certificate is checked', 'The deploy audit acts as governed registration', 'MC05, SP09.2', 'Stage 0 audit scope', P),
    dec('PD4', 'Cross-contract calls in the Core', 'Allowed for Release, JoinFrom, Migrate, ImportFrom and Reclaim from Compact 0.33 on ledger 9; supersedes wiki/moriarty-architecture.md:143 for these only; external calls stay excluded', 'MC05, MC06', 'Stage 6', P),
    dec('PD5', 'Shared-state interleaving, asynchronous messaging and Pending', 'Redesign under head discipline in SP10 before any campaign advertises them', 'MC06, SP10.1', 'Stage 6 scope', P),
    dec('PD6', 'Certificate entry point k bound', None, 'Program resource authority, MC03', 'f1, f2', 'user-decision-required'),
    dec('PD7', 'Fixed MC03 54-limb relation', 'Retire it; its k17 failure stays failed evidence; segment certificates replace it', 'MC03, SP06', 'Stage 7', P),
]
opens = [
    ('Contract layout: one contract per instance or several heads per contract', 'MC04', 'Stage 2'),
    ('Single step entry point to hide the action type', 'MC06, SP10 with SP02', 'Stage 1 circuit shape'),
    ('Join-summary content per profile', 'MC06, SP10 with RP01', 'Stage 6'),
    ('Migration of requiredClaimRoot to the compiled claim set', 'MC01 with MC05', 'Stage 1'),
    ('Per-head lifetime details across split, join and migration', 'MC01 with MC06', 'Stage 6'),
    ('now as a block-time-bounded value and accrual rounding', 'SP01, SP07 with MC05', 'Stage 1'),
    ('Unshielded versus contract-owned shielded settlement', 'MC02, SP05 with MC06', 'Stage 3'),
    ('State commitment hash: SHA-256 persistentHash unless a reviewed decision moves to Poseidon', 'SP04 with MC04', 'E2'),
    ('Network DUST fee caps: wallet policy versus proof', 'SP08 with MC08, SP12', 'Stage 1'),
    ('netTag residual for replicated deployments and a network-id request to Midnight', 'MC04', 'Stage 1'),
    ('Toolchain per network and the ledger-9 fork date', 'MC02, SP05 with RP03', 'Stage 6'),
    ('Compact versus hand-written zk-stdlib step relation', 'MC04, SP09 with SP04', 'E2'),
    ('Canonical claim vocabulary and site wording', 'MC01, SP01 with MC08, SP12', 'Stage 0'),
    ('Handoff encryption scheme, key directory and recovery owner', 'MC06, SP10.2', 'Stage 5'),
    ('Fallible-section placement and PartialSuccess handling', 'MC04', 'E1'),
    ('ledger-10 certificate soundness: parameter serving, format pin, subgroup and trailing-byte checks, unpriced accumulators', 'MC03, SP04', 'Stage 7'),
    ('Exported history proof for off-ledger auditors', 'MC08, SP12', 'Outside acceptance'),
    ('Signature griefing by intervening steps', 'SP08 with MC06', 'Stage 5'),
]
decisions += [dec(f'PO{i:02}', q, None, o, b, 'open') for i, (q, o, b) in enumerate(opens, 1)]
REPORT, PCDR = 'deliverables/pcd-midnight-native-2026-09-11/REPORT.md', 'openspec/PCD-ROADMAP-2026-09-11.md'
data = {'schemaVersion': 1, 'status': 'specified-only', 'amendment': 'openspec/PCD-INTEGRATION-2026-09-11.md',
        'stageMapping': stage_mapping, 'experiments': experiments, 'supersessions': supersessions, 'lockedTasks': locked,
        'dependencyTracker': tracker, 'decisions': decisions, 'sourceDigests': {REPORT: sha256(REPORT), PCDR: sha256(PCDR)}}
dump_json('openspec/sprints/pcd-integration.json', data)
def table(headers, rows):
    out = ['| ' + ' | '.join(headers) + ' |', '|' + '---|' * len(headers)]
    out += ['| ' + ' | '.join(str(c) for c in r) + ' |' for r in rows]
    return '\n'.join(out)
reg = load_json('openspec/moriarty-completion-program.json')
st = {s['id']: s for s in reg['reportReconciliation']['stageAdmission']['stages']}
stage_ids = ['f0', 'native-path-freeze', 'f0a', 'f1-fixtures', 'f1', 'f2', 'f3', 'mandatory', 'composition', 'release']
review = (S / 'result-review.txt').read_text().strip() if (S / 'result-review.txt').exists() else '- Result review: pending.'
short = lambda p: p.split('/specs/')[0].split('/')[-1][:4].upper()
md = f"""# PCD integration amendment

Status: S2, specified-only planning revision, dated 2026-09-11. It grants no admission, dispatch, resource or acceptance. The machine twin is [pcd-integration.json](sprints/pcd-integration.json), checked by `openspec/sprints/verify.py`.

## Decision adopted

Moriarty builds a ledger-anchored certified state machine with bounded native certificates. The [decision report](../deliverables/pcd-midnight-native-2026-09-11/REPORT.md) gives the analysis, the [PCD roadmap](PCD-ROADMAP-2026-09-11.md) gives the stages, and the [PCD change package](changes/pcd-ledger-anchored-acceptance/README.md) holds the requirements.

- **Contract proofs cannot be inner proofs.** They use a Blake2b transcript, and `verify_proof` accepts only Poseidon zk-stdlib proofs.
- **The ledger already enforces history.** It verifies each call against the operation key in contract state and rejects a changed head read at application. With immutable keys and constrained genesis, every live head descends from genesis through accepted steps.
- **Recursion serves bounded certificates only.** Off-ledger segments, attestations and imports use `ledger-10` `verify_proof`, with pairings checked by the ledger.

The hard gate is unchanged: verification-enabled mandatory acceptance on Midnight Preview.

## Planning assumption

The user directed that Midnight recursion (pull request 738 on `ledger-10`) be assumed to reach production within a few months. Certificates therefore stay release scope, and `release` keeps `f2`. If recursion slips, certificate stages record `blocked`; the core and the Preview gate stand.

## Stage graph

Stage identifiers, owners and statuses are unchanged. `f3` is the only stage whose prerequisites changed: it previously required `atomic-accept`, `i2`, `f2` and `f1`.

{table(['Stage', 'Owners', 'Requires', 'Purpose'], [(f'`{i}`', ', '.join(st[i]['owners']), ', '.join(st[i]['requires']) or '—', st[i]['purpose']) for i in stage_ids])}

- `mandatory` no longer depends, even transitively, on `f0a`, `f1-fixtures`, `f1` or `f2`. `release` still requires `f2`.
- `f2` keeps `f1`: E5's ledger import uses the certificate entry point whose soundness E3 establishes.
- `f2` keeps `rp01-mc03`, reinterpreted below.

## Record changes

- Package dependencies: MC04 now depends on MC01 and MC02; MC05 depends on MC04.
- MC03 has no planned commands; the `experiments/moriarty-native-ivc-r3/successor` root is retired.
- MC04 status is `specified-only`: the core seam is declared, pending RP02 review, E1 and E2.
- RP02 stays `blocked`, with the certificate-route conditions as its reason.
- Feasibility stage text now describes the Stage 0 seam, E3, E5 and the ledger-anchored core.
- SP09 completion no longer requires SP06. SP12 completion requires SP06 and SP11.

## PCD stages and experiments

{table(['Id', 'Title', 'Stage', 'Tasks'], [(r['id'], r['title'], f"`{r['stage']}`", ', '.join(r['tasks'])) for r in stage_mapping + experiments])}

## SP01 reinterpretation

The SP01 sprint document is hash-pinned by the loan design binding and consumed by in-flight work, so it is not edited. Its tasks are read as follows.

- **SP01.4 (`f0`).** PCD Stage 0: seam, toolchain manifest per network generation, head-discipline checker and deploy-audit designs, and a certificate-route go/no-go.
- **SP01.5 (`native-path-freeze`).** Path ownership for the step-relation compiler output, head-discipline checker, deploy audit and certificate-relation roots. It replaces the single MC03 successor namespace and the MC04 outer-port ownership.
- **SP01.7 (`rp01-mc03`).** The fixed native-statement subset becomes the reviewed statement subset of the Moriarty step that segment certificates fold.

## Reconciliation rows

The [September 7 reconciliation](REPORT-RECONCILIATION-2026-09-07.md) is hash-pinned and not edited. These rows are superseded or amended.

{table(['Reconciliation row', 'Disposition', 'Replacement'], [
 ('F0: finalizer feasibility and ledger-family decision', 'Amended', 'Stage 0 seam and toolchain manifest per generation: ledger 8 core, ledger 9 cross-contract, `ledger-10` certificates'),
 ('F0a: MC04 final pairing port and MC03 export', 'Superseded', 'Certificate-relation authorship; no outer finalizer port'),
 ('I2: parallel uncertified integration', 'Retained', 'Deployed Preview contracts still use a one-key committee'),
 ('F1: P1 transcript, P2 export, P3 constrained pairing', 'Superseded; no-waiver discipline retained', 'E3 controls on a `ledger-10` devnet'),
 ('F2: two-step native IVC proof', 'Superseded', 'E5 segment certificates'),
 ('F3: native verification inside the ledger proof, then Preview acceptance', 'Amended', 'Stages 1–3: one fused proof verified by the ledger, then verification-enabled Preview acceptance'),
 ('F4: private composition and broader finance', 'Amended', 'Stages 5–6 and finance'),
 ('RP02 pre-MC03 gate: P1/P2/P3 before MC03', 'Superseded; no relaxation of mandatory history', 'E1 and E2 decide the core; E3 and E5 decide certificates'),
 ('Retained candidate `moriarty_loan_r3.rs`', 'History only', '—'),
 ('F2/F3 native IVC checks: `vk_repr`, decider, accumulators, transcript EOF, pairing', 'Amended', 'Apply to certificates; the ledger discharges the pairing'),
 ('Terminal-limb and outer-binding controls', 'Amended', 'Binding input, communication commitment, transcript `Popeq` and effect controls'),
 ('P1–P3 fixture recipes', 'Partially reused', 'Inputs to E3 and E5 fixtures'),
 ('P1/P3 in the exact outer circuit stack with a constrained finalizer', 'Superseded', 'Pairings are never computed in-circuit'),
 ('Route: constrained complete native finalizer', 'Superseded', 'Ledger-side pairing from pull request 738'),
 ('Route: ledger-9 V3 source alignment', 'Amended', 'Cross-contract calls only'),
 ('Route: ledger-native accumulator interface at a newer pin', 'Selected', 'Pull request 738, conditional on `ledger-10` release'),
 ('Route: deterministic-only or host verification boolean', 'Retained as forbidden', '—'),
 ('Interface-blocked disposition when all routes fail', 'Amended', 'The core is not interface-blocked; removing mandatory PCD still needs user direction'),
 ('History compliance connects predecessor states and proofs', 'Amended', 'On-ledger history needs no predecessor proofs'),
 ('SP01.5 native-path-freeze with one MC03 successor root', 'Superseded', 'See the SP01 reinterpretation'),
])}

## Supersessions

MC requirement headings and locked task lines do not change. Each affected scenario carries an in-place "Amended by" note.

- **Mechanism replaced.** The rejection the MC text requires still holds; the PCD requirement supplies how it is enforced.
- **Extended.** The MC rejection list gains cases.
- **Removed.** The MC text described the retired recursive route; each removal is a decision below.

{table(['Package', 'Requirement', 'Scenario', 'Disposition', 'Note', 'Governing PCD requirements'], [(short(r['spec']), r['requirement'], r['scenario'] or '—', r['disposition'], r['note'], '; '.join(r['governing'])) for r in supersessions])}

## Locked task dispositions

Task lines stay verbatim. The closing sprint tasks in `package-task-map.json` now do the following.

{table(['Package', 'Task', 'Disposition', 'Note', 'Governing PCD requirements'], [(r['package'], r['taskId'], r['disposition'], r['note'], '; '.join(r['governing'])) for r in locked])}

## Decision register

Decisions PD1–PD5 and PD7 carry defaults awaiting the user's confirmation. None weakens a gate. PD6 has no default.

{table(['Id', 'Question', 'Default', 'Owner', 'Blocks', 'Status'], [(d['id'], d['question'], d['default'] or '—', d['owner'], d['blocks'], d['status']) for d in decisions])}

## Midnight dependency tracker

{table(['Item', 'Needed by', 'Status on 2026-09-11', 'Class'], [(d['item'], d['neededBy'], d['status'], d['class']) for d in tracker])}

## Resource ceiling

Every campaign keeps k ≤ 17, 8 GiB of process-group memory and two CPU jobs. The reproduced pull request 738 outer circuits needed k = 18 with a 4,168 MiB peak (about 4.1 GiB) and k = 19 with an 8,004 MiB peak (about 7.8 GiB). Certificate campaigns therefore need a reviewed `resourceAmendments` entry that the user approves (decision PD6). No such amendment exists.

## Stop conditions

- **E1 fails.** Stop Stages 2–6 until a reviewed consumption-set design replaces head discipline.
- **E2 fails.** Revisit relation decomposition within k ≤ 17; acceptance requirements stay unchanged.
- **`ledger-10` changes the pull request 738 formats.** Pin the released formats and rerun E3.
- **Recursion slips or the resource amendment is refused.** Certificate stages record `blocked`. The core and the Preview gate stand, and no scope is dropped without new user direction.

## Review record

- Design review: Fable 5.1 audit of the design, 2026-09-11. Verdict approve-with-changes; its two blocking, six major and nine minor findings were applied before this revision.
{review}
- The Charter's second independent reviewer has not reviewed this revision.
"""
write('openspec/PCD-INTEGRATION-2026-09-11.md', md)
print('built', len(stage_mapping), len(experiments), len(supersessions), len(locked), len(decisions))
PY
cd ~/Moriarty && python3 $S/build_pcd_integration.py
```
Expected: `built 14 6 14 29 25`.

- [ ] **Step 2: Write the failing tests**

<!-- run: T5b -->
```bash
cd ~/Moriarty && python3 - <<'PY'
import sys; sys.path.insert(0, '/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
from pcdlib import rep
T = 'openspec/sprints/test_verify.py'
rep(T, "        paths |= set(reports['sourceDigests']) | set(gates['sourceSha256'])\n",
       "        paths |= set(reports['sourceDigests']) | set(gates['sourceSha256'])\n        paths |= set(self.load('pcd-integration.json')['sourceDigests'])\n")
rep(T, "\n\nif __name__ == '__main__':", '''
    def set_stage_requires(self, stage, requires):
        path = self.root / 'openspec/moriarty-completion-program.json'
        program = json.loads(path.read_text())
        for record in program['reportReconciliation']['stageAdmission']['stages']:
            if record['id'] == stage:
                record['requires'] = requires
        path.write_text(json.dumps(program))
        sprints = self.load('sprints.json')
        for sprint in sprints['sprints']:
            for gate in sprint['entryGates']:
                if gate['stage'] == stage:
                    gate['requires'] = requires
        self.save('sprints.json', sprints)

    def test_core_depends_on_certificate_stage(self):
        self.set_stage_requires('f3', ['atomic-accept', 'i2', 'f2', 'f1'])
        self.rejects('PCD core depends on certificate stage')

    def test_release_lost_certificate_stage(self):
        self.set_stage_requires('release', ['atomic-accept', 'i2', 'f3', 'mandatory', 'composition', 'finance'])
        self.rejects('PCD release lost certificate stage')

    def test_unknown_pcd_crosswalk_reference(self):
        data = self.load('pcd-integration.json')
        data['stageMapping'][0]['tasks'] = ['SP99.9']
        self.save('pcd-integration.json', data)
        self.rejects('Unknown PCD crosswalk reference')


if __name__ == '__main__':''')
print('tests added')
PY
```

<!-- check: T5-red -->
```bash
/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad/validate-clean-copy.sh 2>&1 | tail -6
```
Expected: `3 failed, 8 passed` (the three new tests).

- [ ] **Step 3: Implement the guard**

<!-- run: T5c -->
```bash
cd ~/Moriarty && python3 - <<'PY'
import sys; sys.path.insert(0, '/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
from pcdlib import rep
rep('openspec/sprints/verify.py', "\nprint(json.dumps({'status': 'pass',", '''
# PCD integration: the ledger-anchored core must not wait on certificates, and release keeps them.
pcd = json.loads((SPRINTS / 'pcd-integration.json').read_text())
for path, digest in pcd['sourceDigests'].items():
    require(hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, f'Stale PCD integration source: {path}')
def stage_ancestors(id):
    found, pending = set(), list(stage_records[id]['requires'])
    while pending:
        dep = pending.pop()
        if dep not in found:
            found.add(dep)
            pending.extend(stage_records[dep]['requires'])
    return found
require(not {'f0a', 'f1-fixtures', 'f1', 'f2'} & stage_ancestors('mandatory'), 'PCD core depends on certificate stage')
require('f2' in stage_ancestors('release'), 'PCD release lost certificate stage')
pcd_requirements = {name for spec, name in actual if spec.startswith('openspec/changes/pcd-ledger-anchored-acceptance/specs/')}
for row in pcd['stageMapping'] + pcd['experiments']:
    require(row['stage'] in stage_records and row['tasks'] and set(row['tasks']) <= set(task_paths), f'Unknown PCD crosswalk reference: {row["id"]}')
for row in pcd['supersessions']:
    require((row['spec'], row['requirement']) in actual and set(row['governing']) <= pcd_requirements, f'Unknown PCD crosswalk reference: {row["requirement"]}')
for row in pcd['lockedTasks']:
    require((row['package'], row['taskId']) in mapped and set(row['governing']) <= pcd_requirements, f'Unknown PCD crosswalk reference: {row["package"]} {row["taskId"]}')
''' + "\nprint(json.dumps({'status': 'pass',")
print('guard added')
PY
```

- [ ] **Step 4: Verify green**

<!-- check: T5 -->
```bash
/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad/validate-clean-copy.sh
```
Expected: verify.py pass; `11 passed`.

---

### Task 6: Sprint contracts, lessons and asset study

**Files:** Replace `openspec/sprints/sp04-complete-native-verifier-component-feasibility.md` and `openspec/sprints/sp06-real-recursive-financial-history.md`. Modify SP09, SP10, `openspec/sprints/README.md`, `report-lessons.json`, `asset-study.json`, `package-task-map.json`, `openspec/ASSET-STUDY-INTEGRATION-2026-09-09.md`.

**Interfaces:** Task ids stay SP04.1–3, SP06.1–3, SP09.1–5, SP10.1–3. SP04.3 title becomes `Run E3 certificate controls`.

- [ ] **Step 1: Rewrite SP04 and SP06**

<!-- run: T6a -->
```bash
cd ~/Moriarty && B='- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SPXX.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.' && python3 - "$B" <<'PY'
import sys; sys.path.insert(0, '/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
from pcdlib import write
bind = sys.argv[1]
def tail(sp): return f'- [ ] Retain commands, outputs, resource use and exact source/profile digests under `{sp}` in the owning package evidence.\n- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.'
def head(sp, title, goal, owners, deps, stages, extra):
    return f"""# {sp}: {title} implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for admitted tasks. Use the existing task contract for routine authorized edits and checks; expand only an unresolved interface or a resource-controlled campaign into an execution note. Steps use checkbox (`- [ ]`) syntax.

**Goal:** {goal}

**Architecture:** This sprint contributes to {owners}; those OpenSpec packages retain acceptance ownership. It consumes the exact accepted profiles and artifacts named by its prerequisites. The [PCD integration](../PCD-INTEGRATION-2026-09-11.md) repurposes it for bounded native certificates; the ledger-anchored core does not wait for it.

**Tech Stack:** TypeScript/Node.js, `.mori`, ISO/IEC 14977 EBNF, K, Compact and the selected Midnight native stack where applicable. Versions are pinned at task admission.

## Global constraints

Status: S2, specified-only. [Program rules](README.md) apply to every task.

Full-completion dependencies (not individual task entry gates): {deps}. Stage scope: {stages}. Use `sprints.json` entryGates for task eligibility; those prerequisites mirror RP stage admission. Full-completion dependencies cannot delay an otherwise admitted task. A completed sprint never substitutes for a current campaign admission record. {extra}
"""
INTER = 'Interfaces use the common records in [the sprint contract](README.md#shared-artifact-contract). Exact code signatures belong to the reviewed execution packet. Do not invent a prover API before its pinned source is inspected.'
MAPINTRO = 'Paths marked create are proposed outputs. Their presence or commands are not claimed today. If a path already exists at dispatch, reconcile it first.'
EXPECT = 'Expected: exit 0 for valid supported inputs and all required controls passing. Invalid source/data must produce the documented rejection, not a crash or partial effect. Proof and public commands additionally require live RP03 admission.'
sp04 = head('SP04', 'Complete native verifier component feasibility', 'Decide whether bounded native certificates work on the ledger before certificate campaigns.', 'MC03, MC04', 'SP01', 'f0a, f1-fixtures, f1', 'Certificate campaigns also need a reviewed resource amendment, because the measured outer circuits need k = 18–19.') + f"""
## File and interface map

{MAPINTRO}

| Operation | Path | Responsibility |
| --- | --- | --- |
| inspect | `experiments/moriarty-native-ivc-r3/` | Existing native experiment and preserved failed encoding |
| inspect | `evidence/pcd-midnight-native-2026-09-11/` | Reproduced pull request 738 and IVC measurements |
| create | `experiments/moriarty-ledger-adapter/probes/README.md` | Pinned `ledger-10` interfaces and certificate test recipes |
| create | `experiments/moriarty-ledger-adapter/probes/run.py` | Admitted E3 dispatch and retained verification |
| create | `experiments/moriarty-ledger-adapter/probes/resource-contract.json` | Explicit limits, stop conditions and the approved resource amendment |
| create | `experiments/moriarty-ledger-adapter/probes/fixtures.json` | Independent non-loan certificate fixture provenance |

{INTER}

## SP04.1: Freeze component authorship after F0 go

{bind.replace('SPXX', 'SP04')}
- [ ] Author certificate relations only after F0a preparation admission, against pinned pull request 738 / `ledger-10` sources. Constrain `vk_repr` and the state decider in the outer relation. Add a guard-constant lint for `VerifyProof` and `InnerProof`. Freeze actual module paths and command arguments in execution/SP04.md. Review implemented source and the resource amendment before F1.
- [ ] Verify: A missing `ledger-10` interface, unbounded constraint estimate, unapproved resource amendment or unavailable current reviewer prevents F1. Pinning a source branch does not prove Preview deployment compatibility.
{tail('SP04')}

## SP04.2: Generate independent small fixtures

{bind.replace('SPXX', 'SP04')}
- [ ] Generate small non-loan Poseidon zk-stdlib inner proofs and a verifier-test accumulator. Retain generation commands, valid witnesses, public artifact hashes and negative controls. Keep private witness bytes outside Git.
- [ ] Verify: Fixture generation never depends on a loan or other financial proof. Fixture source, transcript, key and SRS identity are independently checked.
{tail('SP04')}

## SP04.3: Run E3 certificate controls

{bind.replace('SPXX', 'SP04')}
- [ ] Deploy a certificate entry point with constant guards on a pinned `ledger-10` devnet. Submit a certificate-bearing call and record the ledger's accumulator pairing result. Measure the charged fee against validation work.
- [ ] Verify: The valid call is accepted. A free or mismatched guard, substituted `vk_repr`, unbound inner instance, inner proof for another key and tampered inner proof each reject, or the lint rejects the build. An outer proof over an invalid inner proof is not evidence until the ledger pairing accepts. Native reference tests alone do not pass E3.
{tail('SP04')}

## Verification entry points

Existing package commands may run only in their declared scope. Other commands are proposed interfaces that task admission must create and verify. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

```sh
python3 experiments/moriarty-ledger-adapter/probes/run.py --contract experiments/moriarty-ledger-adapter/probes/resource-contract.json --preflight-only
python3 experiments/moriarty-ledger-adapter/probes/run.py --contract experiments/moriarty-ledger-adapter/probes/resource-contract.json --execute
python3 experiments/moriarty-ledger-adapter/probes/run.py --contract experiments/moriarty-ledger-adapter/probes/resource-contract.json --verify-retained
```

{EXPECT}

## Report-informed acceptance refinement

**Decide whether bounded native certificates work.** The [refinement contract](../ROADMAP-REFINEMENT-2026-09-09.md), [lesson/case crosswalk](report-lessons.json) and [PCD integration](../PCD-INTEGRATION-2026-09-11.md) add the following acceptance details to the existing task IDs. These remain specified-only.

- [ ] SP04.1/.2: author certificate relations from pinned sources; use independently generated non-loan fixtures so admission does not require the proof it is meant to permit.
- [ ] SP04.3: all E3 negative controls remain mandatory, with nontrivial positive fixtures. The ledger checks pairings; nothing computes them in-circuit. Native reference success alone cannot close E3.
- [ ] The first deliverable is a bounded, decisive certificate result. Failed fit or a refused resource amendment stops the certificate route under existing limits; the ledger-anchored core and eligible language work continue.

## Exit gate

E3 passes for the pinned `ledger-10` build, fixtures and candidate, or certificate status is recorded blocked. The ledger-anchored core is unaffected.
"""
sp06 = head('SP06', 'Real recursive financial history', 'Produce and independently verify an off-ledger segment certificate over the Moriarty step.', 'MC03', 'SP01, SP04', 'f2', 'Certificate campaigns also need a reviewed resource amendment.') + f"""
## File and interface map

{MAPINTRO}

| Operation | Path | Responsibility |
| --- | --- | --- |
| inspect | `experiments/moriarty-native-ivc-r3/` | Retired fixed relation and preserved k17 failure; history only |
| inspect | `evidence/pcd-midnight-native-2026-09-11/` | Reproduced IVC segment measurements |
| create | Segment certificate root recorded in `openspec/sprints/execution/native-paths.json` at `native-path-freeze` | Segment relation specification, resource contract, frozen campaign entry point and artifact schema |

{INTER}

## SP06.1: Review the replacement encoding

{bind.replace('SPXX', 'SP06')}
- [ ] Freeze the RP01-MC03 statement subset of the Moriarty step that segments fold. Bind program digest, head, revision, state commitment, authority, observations, effects and work. Retire the fixed 54-limb relation; its k17 failure stays failed evidence. Confirm the outer relation constrains `vk_repr` and the state decider.
- [ ] Verify: The segment relation review explains why it differs from the failed encoding. Source review, E3 evidence and an approved resource amendment precede proving. A fresh folder does not reset charged time.
{tail('SP06')}

## SP06.2: Produce and retain the native episode

{bind.replace('SPXX', 'SP06')}
- [ ] Run only admitted E5 segments at 1, 10 and 100 steps, with segment length at most 16 until benchmarked. Retain real certificates, canonical statements, key/SRS digests and resource receipts. Stop on the first failed control or resource ceiling.
- [ ] Verify: Each segment certificate checks its predecessor accumulator in the native relation. A host hash chain, MockProver run or nonrecursive re-proof fails acceptance.
{tail('SP06')}

## SP06.3: Verify in a fresh process

{bind.replace('SPXX', 'SP06')}
- [ ] Import only retained public bytes and explicitly inventoried artifacts. Verify natively, then import the final state through the certificate entry point on the pinned `ledger-10` devnet. Mutate proof bytes, public state, context, key and accumulator independently.
- [ ] Verify: Valid certificates pass and every invalid control rejects. This closes segment certificates only; the general DSL relation remains assigned to SP09.
{tail('SP06')}

## Verification entry points

Commands are frozen at `native-path-freeze` and the SP06.1 review. The retired `experiments/moriarty-native-ivc-r3/successor/` commands are not entry points. Behavioral task packets must include exact runnable inputs, failing tests and expected outputs before implementation.

{EXPECT}

## Report-informed acceptance refinement

**Produce and independently verify segment certificates.** The [refinement contract](../ROADMAP-REFINEMENT-2026-09-09.md), [lesson/case crosswalk](report-lessons.json) and [PCD integration](../PCD-INTEGRATION-2026-09-11.md) add the following acceptance details to the existing task IDs. These remain specified-only.

- [ ] SP06.1/.2: implement the segment relation only after atomic acceptance, RP01-MC03, path freeze, E3 and an approved resource amendment. Bind arithmetic constants, head identity, liabilities and work.
- [ ] SP06.3: verify retained serialized artifacts in a separate process; mutate proof bytes, state, context, keys and accumulators. The ledger discharges the accumulator pairing, not a host verdict.
- [ ] Keep the segment scope visible. This milestone does not establish general source execution, private branching or on-ledger consumption.

## Exit gate

Actual independently verified segment certificates for the admitted profile, imported through a `ledger-10` certificate entry point, with current result audits. No general language completion claim.
"""
write('openspec/sprints/sp04-complete-native-verifier-component-feasibility.md', sp04)
write('openspec/sprints/sp06-real-recursive-financial-history.md', sp06)
print('T6a written')
PY
```

- [ ] **Step 2: Patch SP09, SP10, sprint README, lessons, asset study and task titles**

<!-- run: T6b -->
```bash
cd ~/Moriarty && python3 - <<'PY'
import sys; sys.path.insert(0, '/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
from pcdlib import rep, resub, load_json, dump_json
S9 = 'openspec/sprints/sp09-mandatory-pcd-and-ledger-correspondence.md'
rep(S9, '**Goal:** Accept financial transactions only when complete native proofs establish the language and signed authority predicates.', '**Goal:** Accept financial transactions only when ledger-verified step proofs and ledger induction establish the language and signed authority predicates.')
rep(S9, 'SP03, SP05, SP06, SP07, SP08. Stage scope: f3, mandatory.', 'SP03, SP05, SP07, SP08. Stage scope: f3, mandatory.')
rep(S9, 'Interfaces use the common records', 'The step-relation compiler output, head-discipline checker and deploy audit use roots recorded at `native-path-freeze` under the [PCD integration](../PCD-INTEGRATION-2026-09-11.md).\n\nInterfaces use the common records')
resub(S9, r'^- \[ \] Use the SP06 retained proof as the MC04 extension input\..*$', '- [ ] Use the Stage 0 seam; no SP06 or MC03 proof is an input. Complete MC04 P.1-P.7 for adapter-profile-01 as fused Compact step circuits for loan and swap, proved by the proof server and verified by a Rust harness against the compiled keys. Run E2 fit and stop if no variant fits k ≤ 17. Run E1 on Preview. Deploy through the custom immutable-authority path and pass the deploy audit. Connect the qualified relation to SP05 financial settlement. Pin actual Preview deployment, toolchain, key and SRS identity. Complete MC04 3.1-3.2 atomic source/Core/target correspondence, including head read-then-write discipline, before F3 promotion. Own the atomic K definition and claim manifest here, including all executable verification dependencies. Bootstrap the pinned K runner/toolchain if SP03 has not created it; serialize shared writers and preserve atomic sequential semantics separately. Discharge the mechanized loan/swap correspondence with explicit assumptions. Verify full output projection and unique head consumption before promoting atomic F3.', flags=8)
rep(S9, 'Tampered proof, output or predecessor rejects. A compiled wrapper without verified deployment alignment or discharged atomic correspondence cannot close F3.', 'Tampered proof, output or head rejects, and of two conflicting calls from one head at most one applies. A deployment that fails the audit, or lacks discharged atomic correspondence, cannot close F3.')
rep(S9, 'Define contract-property certificate judgments and admission into trusted deployment policy. Require all four named claims on every permitted path, including genesis and administration.', 'Define contract-property certificate judgments checked by the deploy audit and bound in Π_P. Discharge all four claim families at their declared loci on every permitted path, including `Initialize`, `Terminate` and migration. Run sequential history at 10 and 100 steps on one head.')
rep(S9, 'Bound claim counts, dependencies, sidecars and verification work before expensive allocation. Enforce verifier/spec activation, revocation and consumption-preserving migration.', 'Check Π_P bounds before expensive proving. Enforce block-time observation freshness, intent digest v2 in exact-head and outcome modes, a principal-threshold `Pause` and forward-declared migration (E4) instead of in-place key revocation.')
rep(S9, 'Restart, cancellation, key revocation and migration cannot revive consumed authority or work.', 'Restart, cancellation, pause and migration cannot revive consumed authority or work. A maintenance update and a downgrade migration reject. Migration Preview evidence waits for ledger 9 on Preview.')
rep(S9, 'Run mutation tests for omitted outputs, duplicated debt, changed policy and field-order/canonical-decoding attacks.', 'Run mutation tests for omitted outputs, duplicated debt, changed policy and field-order/canonical-decoding attacks. Include head read-then-write and section invariants, and blind-write mutants, in the correspondence domain.')
rep(S9, 'Run local and admitted Preview acceptance controls under the new lineage.', 'Run local and admitted Preview acceptance controls under the new lineage, with deploy audits for every deployment.')
rep(S9, '- [ ] SP09.1: close atomic F3 after I2/F2/F1 and atomic correspondence, without waiting for full successor ACTUS/DeFi. Verify actual loan/swap proofs on Preview and durable unique predecessor consumption.', '- [ ] SP09.1: close atomic F3 after I2, the Stage 0 seam, E2, E1 and atomic correspondence, without waiting for certificates or full successor ACTUS/DeFi. Verify actual loan/swap proofs on Preview and unique head consumption.')
rep(S9, 'HistoryCompliance on every permitted route, including genesis/admin.', 'HistoryCompliance on every permitted route, including genesis, termination and migration, through the claim discharge map.')
S10 = 'openspec/sprints/sp10-private-handoff-and-bounded-composition.md'
rep(S10, '| Private successor and actual branch/join relation |', '| Private successor, split/join step relations and release/reclaim rules |')
rep(S10, 'Implement sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging using the RP01 definitions.', 'Implement sequential composition as `Step`, disjoint parallel as `Split`, and atomic synchronization as same-contract `Join` or a claimed cross-contract call. Give shared-state interleaving, asynchronous messaging and Pending a reviewed rule under head discipline, using the RP01 definitions, before any campaign advertises them.')
rep(S10, 'Inventory proof, commitment openings, witness fragments, recipients and recovery responsibility.', 'Inventory recipient-encrypted openings, per-party sub-state commitments, witness fragments, recipients and recovery responsibility; on-ledger successors need no predecessor proof.')
rep(S10, 'Generate actual split, branch and join proofs with distinct predecessor identities and compatible policies.', 'Run ledger-atomic `Split` and `Join` within one contract, and cross-contract `Release`, `JoinFrom` and `Reclaim` on ledger 9 (E4), with distinct heads and compatible policies. Use certificates only for off-ledger branches.')
rep(S10, 'fill/cancel races and unavailable handoff recovery.', 'fill/cancel races and unavailable handoff recovery. Test an unclaimed `Release` recovered only by `Reclaim`.')
rep(S10, 'create real split, independent branches and join proofs.', 'realize ledger-atomic split and join, independent branches, and certificates for off-ledger branches.')
RD = 'openspec/sprints/README.md'
rep(RD, '| Complete native verifier feasibility result | SP01 | MC03, MC04 |', '| Bounded native certificate feasibility result (E3) | SP01 | MC03, MC04 |')
rep(RD, '| Real retained recursive financial proof | SP01, SP04 | MC03 |', '| Retained off-ledger segment certificates (E5) | SP01, SP04 | MC03 |')
rep(RD, '| General mandatory PCD, authorization and ledger correspondence | SP03, SP05, SP06, SP07, SP08 | MC01, MC04, MC05 |', '| General ledger-anchored mandatory PCD, authorization and ledger correspondence | SP03, SP05, SP07, SP08 | MC01, MC04, MC05 |')
rep(RD, '| Reproducible developer release | SP11 | MC08 |', '| Reproducible developer release | SP06, SP11 | MC08 |')
rep(RD, 'SP01 --> SP04[SP04 Native verifier feasibility]', 'SP01 --> SP04[SP04 Certificate feasibility]')
rep(RD, 'SP04 --> SP06[SP06 Native financial proof]', 'SP04 --> SP06[SP06 Segment certificates]')
rep(RD, '  SP06 --> SP09\n', '')
rep(RD, '  SP11 --> SP12[SP12 Developer release]\n', '  SP11 --> SP12[SP12 Developer release]\n  SP06 --> SP12\n')
rep(RD, 'SP06 needs atomic acceptance, RP01-MC03 and all F1 controls.', 'SP06 needs atomic acceptance, RP01-MC03, `native-path-freeze` and E3 (F1).')
rep(RD, 'SP09 may prepare and close atomic F3 after SP05/SP06 while extended language work continues;', 'SP09 may prepare and close atomic F3 after SP05 and the Stage 0 seam, without waiting for SP04 or SP06, while extended language work continues;')
rep(RD, 'This records one MC03 successor namespace and MC03-export/MC04-outer-port ownership.', 'This records path ownership for the step-relation compiler output, head-discipline checker, deploy audit and certificate-relation roots.')
rep(RD, 'P1/P2/P3 precede F2, and a complete finalizer is mandatory. No automatic k increase follows k17 exhaustion.', 'E3 controls precede F2. No automatic k increase follows k17 exhaustion; certificate campaigns need a reviewed resource amendment that the user approves.')
rep(RD, 'Product completion also requires actual native recursion, mandatory ledger PCD,', 'Product completion also requires bounded native certificates, mandatory ledger-anchored PCD,')
rep(RD, '## Latest refinement\n\n', '## Latest refinement\n\nThe [PCD integration](../PCD-INTEGRATION-2026-09-11.md) re-roots atomic F3 on the Stage 0 seam, repurposes SP04 and SP06 for bounded native certificates, and adds the [PCD change package](../changes/pcd-ledger-anchored-acceptance/README.md). Mandatory Preview acceptance no longer waits for recursion; certificates stay release scope.\n\n')
L = load_json('openspec/sprints/report-lessons.json')
lr = {x['id']: x for x in L['lessons']}
lr['LR13']['positive'] = 'Keep Stage 0 seam→path freeze→certificate-relation authorship→independent non-loan certificate fixtures→E3 on a ledger-10 devnet→E5 segment certificates, and close the ledger-anchored atomic core in F3 with E2 fit, E1 linearity and verification-enabled Preview acceptance. The ledger verifies each contract-call proof against the operation key in contract state and pairing-checks certificate accumulators.'
lr['LR13']['negative'] = 'Reject host booleans, transcript-only success, circular loan-derived admission fixtures, in-circuit pairing claims, free certificate guards, unconstrained vk_repr, blind head writes and retrying the failed k17 encoding without a reviewed changed hypothesis.'
assert lr['LR14']['positive'].startswith('Prove explicit elaboration/evaluator/compiler/native/ledger domains and all four mandatory claims')
lr['LR14']['positive'] = 'Prove explicit elaboration/evaluator/compiler/native/ledger domains, the claim discharge map for all four mandatory claim families and the head read-then-write invariant; include feasible positive transitions and a deliberately falsified claim.'
dump_json('openspec/sprints/report-lessons.json', L)
A = load_json('openspec/sprints/asset-study.json')
hits = 0
for section in A.values():
    if isinstance(section, list):
        for r in section:
            if isinstance(r, dict) and r.get('id') == 'AS09':
                for c in r['conditionalTasks']:
                    c['condition'] = c['condition'].replace('changes the native statement', 'changes a certificate statement').replace('preserve unaffected feasibility work', 'preserve unaffected certificate feasibility work')
                hits += 1
assert hits == 1
dump_json('openspec/sprints/asset-study.json', A)
rep('openspec/ASSET-STUDY-INTEGRATION-2026-09-09.md', 'SP04.2/.3 apply only if an admitted profile changes the native statement.', 'SP04.2/.3 apply only if an admitted profile changes a certificate statement.')
M = load_json('openspec/sprints/package-task-map.json')
n = 0
for r in M['rows']:
    if r['closingTaskTitle'] == 'Run all three component predicates':
        assert r['primaryClosingTask'] == 'SP04.3'
        r['closingTaskTitle'] = 'Run E3 certificate controls'; n += 1
assert n == 4
dump_json('openspec/sprints/package-task-map.json', M)
print('T6b applied')
PY
```

- [ ] **Step 3: Verify**

<!-- check: T6 -->
```bash
cd ~/Moriarty && grep -n "P1/P2/P3\|outer circuit stack\|final accumulator/pairing\|SP06 retained proof\|native-ivc-r3/successor/run" openspec/sprints/sp04-*.md openspec/sprints/sp06-*.md openspec/sprints/sp09-*.md; echo "grep exit $?"
/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad/validate-clean-copy.sh
```
Expected: only the retired-commands sentence in SP06 matches `native-ivc-r3/successor`; verify.py pass; `11 passed`.

---

### Task 7: Roadmap, Charter and refinement note

**Files:** Modify `ROADMAP.md`, `openspec/MORIARTY-COMPLETION-PROGRAM.md`, `openspec/ROADMAP-REFINEMENT-2026-09-09.md`.

- [ ] **Step 1: Apply narrative edits**

<!-- run: T7 -->
```bash
cd ~/Moriarty && python3 - <<'PY'
import re, sys; sys.path.insert(0, '/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad')
from pcdlib import rep, resub
R = 'ROADMAP.md'
resub(R, r'^The proposed \[Midnight-native PCD roadmap\].*$', 'The [Midnight-native PCD roadmap](openspec/PCD-ROADMAP-2026-09-11.md) turns the [September 11 PCD architecture report](deliverables/pcd-midnight-native-2026-09-11/REPORT.md) into nine staged gates and five ordered experiments. The [PCD integration amendment](openspec/PCD-INTEGRATION-2026-09-11.md) adopts it into this roadmap as specified-only planning: ledger-anchored certified state with bounded native certificates replaces per-transaction recursive history. Atomic F3 now rests on the Stage 0 seam, SP04 and SP06 deliver certificates, and the [PCD change package](openspec/changes/pcd-ledger-anchored-acceptance/README.md) adds eighteen requirements to MC03–MC06. Verification-enabled mandatory Preview acceptance stays the hard gate, and certificate campaigns need a reviewed resource amendment.', flags=re.M)
resub(R, r'^(\| \[SP04\]\(openspec/sprints/sp04-complete-native-verifier-component-feasibility\.md\) \|).*$', r'\1 Decide whether bounded native certificates work on the ledger | E3 certificate controls on a `ledger-10` devnet, including ledger-side accumulator pairing. |', flags=re.M)
resub(R, r'^(\| \[SP06\]\(openspec/sprints/sp06-real-recursive-financial-history\.md\) \|).*$', r'\1 Produce and independently verify an off-ledger segment certificate | E5 segment certificates at 1, 10 and 100 steps; retained bytes verified in a fresh process and mutations rejected. |', flags=re.M)
resub(R, r'^(\| \[SP09\]\(openspec/sprints/sp09-mandatory-pcd-and-ledger-correspondence\.md\) \|).*$', r'\1 Require complete proof and authority at ledger acceptance | All four mandatory claim families discharged by the fused step relation and ledger induction, proved correspondence and verification-enabled Preview acceptance. |', flags=re.M)
resub(R, r'^(\| \[SP10\]\(openspec/sprints/sp10-private-handoff-and-bounded-composition\.md\) \|).*$', r'\1 Prove private handoff and all five composition operators | Isolated recipient-keyed handoff, ledger-atomic split/join, cross-contract release and reclaim, and all five composition operators through acceptance. |', flags=re.M)
rep(R, 'Start three eligible tracks: **SP01→SP02→SP03** for language, **SP01 F0→SP04→SP06** for native feasibility/proofs, and **accepted atomic + loan/swap subset→SP05** for financial Preview integration. SP09.1 atomic F3 joins SP05/SP06 early; the full successor joins after SP07/SP08.', 'Start four eligible tracks: **SP01→SP02→SP03** for language, **SP01 F0→SP09.1** for the ledger-anchored core, **SP01 F0→SP04→SP06** for bounded native certificates, and **accepted atomic + loan/swap subset→SP05** for financial Preview integration. SP09.1 atomic F3 joins SP05 and the Stage 0 seam early without waiting for certificates; the full successor joins after SP07/SP08.')
rep(R, 'The original native recursion experiment exhausted rows at k17. Its fixed-instance replacement is source work that has not produced a recursive proof. The complete native-to-Preview verifier remains unresolved.', 'The original native recursion experiment exhausted rows at k17. The [PCD integration](openspec/PCD-INTEGRATION-2026-09-11.md) retires that fixed-instance route: the ledger verifies contract-call proofs against the operation key in contract state, and recursion serves only bounded certificates on `ledger-10`. The ledger-anchored core and its Preview acceptance remain unbuilt.')
rep(R, 'native verifier feasibility; Preview financial integration; real recursion;', 'certificate feasibility; Preview financial integration; segment certificates;')
resub(R, r'^- \[ \] \*\*RP02: Complete native history route\.\*\*.*$', '- [ ] **RP02: Complete native history route.** On-ledger history follows by induction from constrained genesis, immutable operation keys and head read-then-write discipline. The PCD integration settles this in design, pending RP02 review and experiments E1 and E4. Specify what a private successor receives, which secrets remain private, and how off-ledger segment certificates reach the ledger through `ledger-10` `verify_proof`. Pin source and deployment versions separately. Resolve source interfaces and run independently admitted component probes before a new native campaign.', flags=re.M)
rep(R, '- [ ] Correct and review the fixed financial relation, successor commands, canonical export and retained-proof verifier under a bounded campaign.', '- [ ] Review the certificate relations, guard-constant lint, `Collapsed` decider constraints, canonical export and retained-proof verifier under a bounded campaign.')
rep(R, '- [ ] Produce genuine recursive proofs for the admitted two-step loan episode and verify serialized artifacts in an independent process.', '- [ ] Produce off-ledger segment certificates over the Moriarty step at 1, 10 and 100 steps and verify serialized artifacts in an independent process.')
rep(R, '- [ ] Reject altered proof bytes, context, state, keys and accumulators; discharge the complete native final decision.', '- [ ] Reject altered proof bytes, context, state, keys, accumulators, free guards, substituted `vk_repr` and unbound inner instances; the ledger discharges the accumulator pairing.')
rep(R, 'Acceptance: actual retained native IVC evidence for the exact fixed episode.', 'Acceptance: actual retained segment-certificate evidence, accepted through a `ledger-10` certificate entry point.')
rep(R, '- [ ] Implement the complete native verification boundary under exact Midnight source and deployed-version provenance, including canonical decoding and final accumulator/pairing verification.', '- [ ] Declare and use the ledger verification seam, the operation key in contract state checked by ledger `well_formed`, under exact Midnight source and deployed-version provenance, with a deploy audit of immutable authority.')
rep(R, 'Bind program, semantic profile, policy, verifier, predecessors, observations, output state and complete effects across authorization, proof and ledger.', 'Bind the program digest, contract, instance, head, revision, observations, output state and complete effects across authorization, proof and ledger.')
rep(R, 'Enforce durable authorization, currentness, replay protection and unique predecessor consumption.', 'Enforce durable authorization, currentness, replay protection and unique consumption through head read-then-write discipline, checked over generated ZKIR and by experiment E1.')
rep(R, 'including constrained genesis and administrative transitions in scope.', 'including constrained genesis and administrative transitions in scope. The fused step relation discharges refinement and transition validity; on-ledger history compliance follows by ledger induction.')
rep(R, 'Use non-circular canonical commitments and trusted deployment policy.', 'Use non-circular canonical commitments, the compiled claim set and an audited immutable deployment.')
rep(R, '- [ ] Enforce verifier/spec activation and revocation, cheap bounded admission before expensive verification, and safe consumption-preserving migration.', '- [ ] Replace verifier revocation with forward-declared migration and a principal-threshold pause. Check program-digest bounds before expensive proving, and keep migration consumption-preserving.')
rep(R, '- [ ] Produce genuine split, independent branch and join proofs with compatible policies, distinct identities and no duplicate predecessor use.', '- [ ] Realize ledger-atomic split and join, cross-contract release with reclaim, and certificates for off-ledger branches, with compatible policies, distinct heads and no duplicate consumption.')
resub(R, r'```mermaid\nflowchart TD\n  RP1\[RP01 semantic design map\].*?```', """```mermaid
flowchart TD
  RP1[RP01 semantic design map] -->|RP01-MC02 subset| MC02
  RP1 -->|RP01-MC03 subset| F2
  MC01[MC01-ATOMIC acceptance] --> MC02
  MC01 --> F3
  SRC[MC04 pinned source findings] --> F0[RP02 F0 Stage 0 seam and toolchain manifest]
  F0 --> NPF[native-path-freeze]
  NPF --> F3[MC04 / F3 ledger-anchored core: E2 fit, E1 linearity, Preview acceptance]
  NPF --> F0A[F0a certificate-relation authorship]
  F0A --> F1[F1 / E3 certificate on a ledger-10 devnet]
  F1 --> F2[MC03 / F2 / E5 segment certificate]
  ADM[RP03 separate candidate-bound campaign admission] --> F0A
  ADM --> F1
  ADM --> MC02[MC02 / I2 uncertified financial integration]
  ADM --> F2
  ADM --> F3
  MC02 --> F3
  F3 --> MC05[MC05 mandatory acceptance]
  MC05 --> MC06[MC06 private composition]
  MC06 --> MC07[MC07 complete finance]
  MC07 --> MC08[MC08 developer release]
  F2 --> MC08
```""")
resub(R, r'^F0 establishes a reviewed proposed route without native execution.*$', 'F0 records the Stage 0 seam without native execution: the operation key in contract state that ledger `well_formed` checks, a toolchain manifest per network generation, and the head-discipline checker and deploy-audit designs. It also records a certificate-route go/no-go, conditional on `ledger-10`. `native-path-freeze` then fixes path ownership for the step-relation compiler output, checker, deploy audit and certificate-relation roots. F3 builds the ledger-anchored core: E2 measures the fused step relation against k ≤ 17, E1 shows head read-then-write linearity on Preview, and verification-enabled Preview acceptance follows a deploy audit of immutable authority. F3 needs no MC03 proof. On the certificate track, F0a authors certificate relations against pinned pull request 738 sources, F1 runs E3 with its negative controls on a `ledger-10` devnet, and F2 runs E5. Nothing computes pairings in-circuit; the ledger checks each accumulator. Certificate campaigns need k 18–19 on the measured evidence, so they also need a reviewed resource amendment. No incomplete control is waived, and no certificate fixture may depend on a loan proof. The [PCD integration amendment](openspec/PCD-INTEGRATION-2026-09-11.md) maps the earlier P1–P3 recipes and route table.', flags=re.M)
rep(R, 'The native precondition is the reviewed RP01-MC03 fixed-statement subset and F0/F0a, successful F1, MC01-ATOMIC acceptance and campaign-specific RP03 admission. The full RP01 design map gates general successor profiles and later semantic extensions; it is not a prerequisite for the unchanged fixed-instance experiment.', 'The certificate precondition is the reviewed RP01-MC03 statement subset of the Moriarty step, F0/F0a, successful F1, MC01-ATOMIC acceptance, a reviewed resource amendment and campaign-specific RP03 admission. The full RP01 design map gates general successor profiles and later semantic extensions.')
C = 'openspec/MORIARTY-COMPLETION-PROGRAM.md'
rep(C, '| Reviewed encoding; two recursive financial steps; independent retained-proof verification | MC01 |', '| Certificate relations and off-ledger segment certificates, independently verified | MC01 |')
rep(C, '| Proof/ledger compatibility, compiler correspondence, durable authority and consumption | MC01–MC03 |', '| Proof/ledger compatibility, compiler correspondence, durable authority and consumption | MC01, MC02 |')
rep(C, '| All four mandatory claims enforced in actual acceptance | MC03, MC04 |', '| All four mandatory claims enforced in actual acceptance | MC04 |')
rep(C, 'MC03 proves the retained financial episode under its fixed authority assumptions.\nMC04 and MC05 must extend that evidence to actual authorization and ledger acceptance.\n', 'MC03 proves bounded native certificates and off-ledger segment history.\nMC04 and MC05 build the ledger-anchored step relation, authorization and ledger acceptance without an MC03 proof input.\nThe [PCD integration](PCD-INTEGRATION-2026-09-11.md) records this change.\n')
rep(C, 'MC04 first probes the retained MC03 proof interface without claiming swap or dynamic-authority correspondence.', 'MC04 first pins the declared ledger verification seam without claiming swap or dynamic-authority correspondence.')
rep(C, 'All campaigns retain k at most 17, 8 GiB process-group memory, and two CPU jobs.\n', 'All campaigns retain k at most 17, 8 GiB process-group memory, and two CPU jobs.\nCertificate campaigns need k 18–19 on the measured evidence; they cannot be admitted until the user approves a reviewed resource amendment.\n')
rep(C, 'If the interface is absent, propose a checked Compact/ZKIR decider wrapper or pinned-version alignment for review.', 'The PCD integration records the core interface: ledger `well_formed` checks each contract-call proof against the operation key in contract state.\nCertificates use `ledger-10` `verify_proof`; no in-circuit decider wrapper is planned.')
rep(C, 'Before MC06 proving, probe successor continuation from retained artifacts and a two-predecessor join at the pinned interface.', 'Before MC06 proving, probe successor continuation from a head commitment and recipient-encrypted opening, and a two-head join, at the pinned interface.')
rep(C, ', real recursion,', ', bounded native certificates,')
rep('openspec/ROADMAP-REFINEMENT-2026-09-09.md', 'and the [program register](moriarty-completion-program.json) is unchanged:', 'is unchanged by this refinement; the later [PCD integration](PCD-INTEGRATION-2026-09-11.md) re-roots `f3` and repurposes the native track:'.replace('is unchanged by', 'and the [program register](moriarty-completion-program.json) is unchanged by'))
print('T7 applied')
PY
```

- [ ] **Step 2: Verify**

<!-- check: T7 -->
```bash
cd ~/Moriarty && grep -n "MC03 supplies the terminal proof\|constrained final pairing acceptance\|retained MC03 proof interface\|real recursion,\|SP06 early" ROADMAP.md openspec/MORIARTY-COMPLETION-PROGRAM.md; echo "grep exit $?"
/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad/validate-clean-copy.sh
```
Expected: `grep exit 1`; verify.py pass; `11 passed`.

---

### Task 8: Vault transaction and project memory

**Files:** Create `$S/vault/pcd-adopt-{bundle,inspect,apply,lint}.json`. The transaction modifies `wiki/decisions/pcd-midnight-native-architecture.md`, `wiki/benchmarks.md`, `wiki/moriarty-architecture.md`, `wiki/contradictions.md`, `wiki/log.md` and `wiki/meta/ledgers/claim-ledger.json`. Separately modify `~/.claude/projects/-home-charl/memory/moriarty-pcd-midnight-native-report.md` and `MEMORY.md`.

**Interfaces:**
- Consumes the claude-obsidian 2.1.1 CLI at `/home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py`, the subcommands `transaction inspect|apply` and `lint`.
- Bundle schema `claude-obsidian.transaction.v1`, keys `schema, operation_id, operation_type, expected_hashes, writes[{path,mode,content,sha256}], address_requests, source_manifest_updates`.

- [ ] **Step 1: Build and inspect the bundle**

<!-- run: T8a -->
```bash
S=/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad; V=$S/vault; CORE=/home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py
cd ~/Moriarty && python3 - <<'PY'
import datetime, hashlib, json, pathlib, re
ROOT = pathlib.Path.home() / 'Moriarty'
V = pathlib.Path('/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad/vault')
earlier = json.loads((V / 'ingest-bundle.json').read_text())
w0 = earlier['writes'][0]
assert earlier['schema'] == 'claude-obsidian.transaction.v1' and hashlib.sha256(w0['content'].encode()).hexdigest() == w0['sha256']
texts = {}
def cur(p): return texts.get(p, (ROOT / p).read_text())
def edit(p, old, new):
    t = cur(p); assert t.count(old) == 1, (p, old[:70]); texts[p] = t.replace(old, new)
def append(p, block): texts[p] = cur(p).rstrip('\n') + '\n\n' + block.rstrip('\n') + '\n'
now = datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
D = 'wiki/decisions/pcd-midnight-native-architecture.md'
edit(D, 'The roadmap is specified-only and not adopted. Every existing MC/SP acceptance gate is unchanged.', "The [PCD integration amendment](../../openspec/PCD-INTEGRATION-2026-09-11.md) adopted the roadmap into OpenSpec planning on 2026-09-11 as specified-only work. Atomic F3 rests on the Stage 0 seam, SP04 and SP06 deliver certificates, and verification-enabled mandatory Preview acceptance remains the hard gate. Six design defaults await the user's confirmation, and the certificate k bound needs a user-approved resource amendment.")
texts[D] = re.sub(r'^updated_at: .*$', f'updated_at: {now}', cur(D), count=1, flags=re.M)
edit(D, '| 34.8 s | 4.3 GiB | 6,550 B |', '| 34.8 s | 4.1 GiB | 6,550 B |')
edit(D, '| 81.9 s | 8.2 GiB | 6,550 B |', '| 81.9 s | 7.8 GiB | 6,550 B |')
edit('wiki/benchmarks.md', '| 5.9 ms | 6,550 B | 4.3 GiB |', '| 5.9 ms | 6,550 B | 4.1 GiB |')
edit('wiki/benchmarks.md', '| 6.2 ms | 6,550 B | 8.2 GiB |', '| 6.2 ms | 6,550 B | 7.8 GiB |')
edit('wiki/moriarty-architecture.md', 'manifests and hash allowlists without claiming that external code inherits\nMoriarty proofs.\n', "manifests and hash allowlists without claiming that external code inherits\nMoriarty proofs.\n\n**Superseded in part, 2026-09-11.** For `Release`, `JoinFrom`, `Migrate`, `ImportFrom` and `Reclaim` only, the [PCD integration amendment](../openspec/PCD-INTEGRATION-2026-09-11.md) allows claimed cross-contract calls between Moriarty contracts from Compact 0.33 on ledger 9, pending the user's confirmation (decision PD4). External non-Moriarty calls stay excluded.\n")
append('wiki/contradictions.md', """## Cross-contract calls and memory figures — 2026-09-11

- **Cross-contract calls.** The [architecture page](moriarty-architecture.md) keeps cross-contract calls outside the initial Core. The [PCD integration amendment](../openspec/PCD-INTEGRATION-2026-09-11.md) needs claimed calls for `Release`, `JoinFrom`, `Migrate`, `ImportFrom` and `Reclaim`. Decision PD4 resolves this for those entry points only, pending the user's confirmation. External non-Moriarty calls stay excluded in both positions.
- **Memory figures.** Earlier vault pages quoted 4.3 GiB and 8.2 GiB for the pull request 738 tests. The retained logs give peaks of 4,168 MiB and 8,004 MiB, about 4.1 GiB and 7.8 GiB. The decision page, the benchmark page and CLM-0957 now use the retained values.""")
append('wiki/log.md', """## [2026-09-11] planning | PCD roadmap adopted into OpenSpec

**Deliverables.** The [PCD integration amendment](../openspec/PCD-INTEGRATION-2026-09-11.md), the [PCD change package](../openspec/changes/pcd-ledger-anchored-acceptance/README.md) and the updated [PCD roadmap](../openspec/PCD-ROADMAP-2026-09-11.md).

**Changes.** Atomic F3 rests on the Stage 0 seam instead of recursive stages. SP04 and SP06 deliver certificates, and `release` still requires them. MC04 and MC05 no longer depend on MC03. CLM-0957 memory figures now match the retained logs.

**Not performed.** No proof, campaign, deployment, transaction or commit. Six design defaults and the certificate k bound await the user.""")
L = json.loads(cur('wiki/meta/ledgers/claim-ledger.json'))
c = L['claims']['clm-moriarty-0957']
assert '4.3 GiB' in c['text'] and '8.2 GiB' in c['text']
c['text'] = c['text'].replace('4.3 GiB', '4.1 GiB').replace('8.2 GiB', '7.8 GiB')
c['notes'] += ' Memory figures corrected on 2026-09-11 to the retained peaks of 4,168 MiB and 8,004 MiB.'
texts['wiki/meta/ledgers/claim-ledger.json'] = json.dumps(L, indent=2, ensure_ascii=False) + '\n'
bundle = {'schema': 'claude-obsidian.transaction.v1', 'operation_id': 'update-pcd-openspec-adoption-20260911', 'operation_type': earlier['operation_type'],
          'expected_hashes': {p: hashlib.sha256((ROOT / p).read_bytes()).hexdigest() for p in texts},
          'writes': [{'path': p, 'mode': 'replace', 'content': t, 'sha256': hashlib.sha256(t.encode()).hexdigest()} for p, t in texts.items()],
          'address_requests': [], 'source_manifest_updates': {}}
(V / 'pcd-adopt-bundle.json').write_text(json.dumps(bundle, ensure_ascii=False, indent=1))
print('bundle writes', len(bundle['writes']))
PY
python3 $CORE transaction inspect $V/pcd-adopt-bundle.json --vault . > $V/pcd-adopt-inspect.json; echo "inspect rc=$?"; head -c 600 $V/pcd-adopt-inspect.json
```
Expected: `bundle writes 6`; `inspect rc=0` with an `approval_sha256`. If inspect rejects the operation type, read the allowed values from its message and rebuild with the matching one.

- [ ] **Step 2: Apply and lint**

<!-- run: T8b -->
```bash
S=/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad; V=$S/vault; CORE=/home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py
cd ~/Moriarty && A=$(python3 -c "import json;print(json.load(open('$V/pcd-adopt-inspect.json'))['approval_sha256'])")
python3 $CORE transaction apply $V/pcd-adopt-bundle.json --vault . --approved-plan-sha256 $A > $V/pcd-adopt-apply.json; echo "apply rc=$?"
timeout 600 python3 $CORE lint --vault . --format json > $V/pcd-adopt-lint.json; echo "lint rc=$?"
python3 -c "
import json; d=json.load(open('$V/pcd-adopt-lint.json'))
print({k: (len(v) if isinstance(v, (list, dict)) else v) for k, v in d.items()})"
```
Expected: `apply rc=0`; `lint rc=0` with zero findings.

- [ ] **Step 3: Update project memory**

<!-- run: T8c -->
```bash
python3 - <<'PY'
import pathlib
M = pathlib.Path.home() / '.claude/projects/-home-charl/memory'
f = M / 'moriarty-pcd-midnight-native-report.md'
t = f.read_text()
old = 'Also stored in openspec/PCD-ROADMAP-2026-09-11.md (specified-only, not adopted; linked from ROADMAP.md)'
assert t.count(old) == 1
t = t.replace(old, 'Also stored in openspec/PCD-ROADMAP-2026-09-11.md, adopted into OpenSpec planning on 2026-09-11 by openspec/PCD-INTEGRATION-2026-09-11.md, change package openspec/changes/pcd-ledger-anchored-acceptance (18 requirements) and crosswalk openspec/sprints/pcd-integration.json guarded by verify.py')
t = t.rstrip('\n') + '\n\nAdoption facts (2026-09-11): f3 requires [atomic-accept, i2, f0, native-path-freeze]; release still requires f2; MC04/MC05 no longer depend on MC03; decisions PD1-PD5 and PD7 are defaults pending user confirmation; PD6 (certificate k 18-19 vs the k<=17 ceiling) needs a user-approved resource amendment. Peak memory for PR 738 tests is 4.1/7.8 GiB (4,168/8,004 MiB). Uncommitted.\n'
f.write_text(t)
idx = M / 'MEMORY.md'
it = idx.read_text()
old_i = '— 2026-09-11 decision: ledger-anchored state + bounded native certificates; vault page wiki/decisions/pcd-midnight-native-architecture.md, roadmap openspec/PCD-ROADMAP-2026-09-11.md'
assert it.count(old_i) == 1
idx.write_text(it.replace(old_i, '— 2026-09-11 decision: ledger-anchored state + bounded native certificates; adopted into OpenSpec via openspec/PCD-INTEGRATION-2026-09-11.md (uncommitted)'))
print('memory updated')
PY
```

---

### Task 9: Full validation and independent result audit

**Files:** Create `$S/notes/F2-results-audit.md`, `$S/result-review.txt`. Rerun `$S/build_pcd_integration.py`.

- [ ] **Step 1: Run every check**

<!-- check: T9 -->
```bash
S=/tmp/claude-1000/-home-charl/6cf9f7d5-736c-478c-9709-7d8984dd63fa/scratchpad
cd ~/Moriarty && openspec validate --all --strict 2>&1 | tail -1
$S/validate-clean-copy.sh
echo "== real tree"; python3 openspec/sprints/verify.py 2>&1 | head -c 120; echo
T=$(mktemp -d $S/clean2.XXXXXX); rsync -aR --exclude 'openspec/changes/afk-live-financial-execution' openspec docs deliverables/defi-language-design-2026-09-07 deliverables/defi-taxonomy-papers-2026-09-08 deliverables/erc4626-vault-report-2026-09-08 deliverables/modern-defi-taxonomy-2026-09-08 deliverables/pcd-midnight-native-2026-09-11 deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml evidence/moriarty-design-sprint-2026-09-06 evidence/execution raw/assignments "$T/"; (cd $T && python3 openspec/sprints/verify.py 2>&1 | head -c 120); echo; rm -rf "$T"
python3 -m pytest plugins/moriarty-dev/tests -q 2>&1 | tail -1
python3 - <<'PY'
import json, sys
sys.path.insert(0, 'plugins/moriarty-dev/scripts')
from moriarty_dev import records
m = []
records._validate_program(json.load(open('openspec/moriarty-completion-program.json')), m)
records._validate_sprints(json.load(open('openspec/sprints/sprints.json')), m)
print('plugin validators', m)
PY
(cd site && npm test 2>&1 | grep -E "^ℹ (pass|fail)")
sha256sum -c $S/forbidden-before.sha
```
Expected: `Totals: 11 passed, 0 failed`; clean copy pass and `11 passed`; real tree prints `Requirement crosswalk differs`; the AFK-free copy prints `Stale report source: docs/FOOTGUNS.md`; `174 passed`; `plugin validators []`; site `pass 11`, `fail 0`; every protected file `OK`.

- [ ] **Step 2: Independent result audit.** Dispatch a Fable 5.1 agent, read-only, to audit the implemented revision. Inputs: the design spec, this plan, `git diff` for tracked files, the untracked new files, the audit notes R1–R4 and F1. The agent writes `$S/notes/F2-results-audit.md` with a verdict and numbered findings.

- [ ] **Step 3: Apply findings.** Fix every blocking and major finding with anchored patches. Write `$S/result-review.txt` with one line, `- Result review: Fable 5.1 audit of the implemented revision, 2026-09-11. Verdict <verdict>; <what was applied>.` Then rerun `python3 $S/build_pcd_integration.py` and repeat Step 1.

---

## Self-review

**Spec coverage.**

| Design section | Task |
|---|---|
| §4.1 stage graph, §4.1a Register fields, §4.2 sprint records, §4.3 package dependencies | T1; dependency prose T4, Charter table T7 |
| §4.4 change package and eighteen requirements | T3 |
| §4.5 supersessions and in-place notes, §4.5a locked tasks | T4 notes; T5 tables and crosswalk |
| §4.6 amendment, crosswalk, validator and tests | T5 |
| §4.7 sprint contracts, lessons, asset study | T6 |
| §4.8 narrative documents, PCDR status | T7; PCDR in T2 |
| §4.9 decision register | T5 |
| §4.10 corrections 1–12 and wiki | T2; wiki in T8 |
| §4.11 out of scope | No task, by design |
| §5 validation and review | T9 |

**Placeholder scan.** The only pending-state text is the result-review line, which T9 Step 3 replaces through the builder.

**Consistency.** Requirement headings in T3 match the crosswalk constants in T5, the notes in T4 and the coverage rows. The diagnostic strings in T5 tests match `verify.py`. The SP04.3 title in T6 matches the `package-task-map.json` update.

## Execution handoff

The user is not available for questions, so execution is inline with superpowers:executing-plans. Each task runs its `run` block, then its `check` block, and stops on the first failed expectation.
