from pathlib import Path
import json,hashlib,sys,datetime,subprocess
from urllib.parse import quote
sys.path.insert(0,'/home/charl/.local/share/claude-obsidian');from claude_obsidian.ledgers import stable_source_id
R=Path(__file__).parent;M=Path('/home/charl/research/mina-recursion-2026-09-19');V=Path('/home/charl/Moriarty-aeon-study');W={};E={}
def put(p,s=None,file=None):
 q=V/p;E[p]=hashlib.sha256(q.read_bytes()).hexdigest() if q.exists() else None;w={'path':p,'mode':'replace' if q.exists() else 'create'}
 if file:w.update(content_file=str(file),sha256=hashlib.sha256(Path(file).read_bytes()).hexdigest())
 else:w['content']=s
 W[p]=w
def note(title,body,kind='explanation'):
 return f'---\ntitle: "{title}"\ntype: research\nstatus: research-draft\ncreated: 2026-09-19\nupdated: 2026-09-19\ndiataxis: {kind}\ntags: [moriarty, consolidation, recursion]\n---\n\n# {title}\n\n'+body+'\n'
b='wiki/research/consolidated-design/';d='../../../deliverables/consolidated-design-2026-09-19/'
put(b+'index.md',note('Consolidated Moriarty vision and roadmap','''One permissionless language and one U0–U7 implementation sequence now connect the research agenda. Moriarty defines authenticated intention and proved financial stage semantics; the optional federated kernel coordinates solvers, evidence, constrained signing and external execution. No Lean dependency or public project-approval requirement is introduced.

- [Explanation and governing design](explanation.md)
- [How to turn a requirement into a bounded task](how-to.md)
- [Reference and review evidence](reference.md)
- [Paper tutorial](tutorial.md)
- [Mina recursion study](../mina/index.md)
- [MPLR research log](../mplr/index.md)
''','reference'))
put(b+'explanation.md',note('One language and an optional execution federation','''[Canonical design](../../../docs/MORIARTY-CONSOLIDATED-DESIGN.md) and [roadmap](../../../ROADMAP.md) govern current scheduling. P/C/K plans are retained detail and provenance; MC/SP/G financial, proof, privacy and release obligations survive. Accepted SP07/SP08 semantics and requalification of later extensions remain prerequisites for full successor mandatory acceptance.

The language constrains complete effects, gross spending, fees, persistent liabilities, affine authority, conditional evidence and residual duties. The kernel supplies candidates and evidence under separately stated ZK/MPC/TEE/finality assumptions. The ledger enforces actual current state and resource consumption. A trusted template registry is not public deployment permission; a threshold signature is not owner intention unless its enforcing boundary checks that intention.

The user supplied a planning assumption on 2026-09-19: comprehensive Midnight recursion in about six months, approximately March 2027. Full native recursive compliance, private handoff and bounded multi-parent composition remain target requirements. Ledger induction can support an explicitly scoped early profile; it cannot close MC03/MC06. Growing finite history differs from unbounded stage work and cannot reset signed lifetime budgets.

[Next ZKIR/recursion contract](../../../docs/MORIARTY-BACKEND-REQUIREMENTS.md) lists ZR01–ZR16 and Mina-derived MNR01–MNR08. These are proposed requirements, not implemented backend guarantees. [Six-expert record]('''+d+'''CONSENSUS.md) preserves scope, disagreements and corrections. [Index](index.md).
'''))
put(b+'reference.md',note('Consolidation evidence and limits','''Sources are the pinned Aeon/APSS/NEAR/Daml/Simplicity/OWS-x402/Anoma research, the whole-language audit, current DeFiFormal review, and the Mina case study. The frozen evidence packet and source manifest preserve exactly what the six reviewers received. The final user steering adds the six-month recursion assumption and explicit backend requests.

- [Six-expert synthesis]('''+d+'''CONSENSUS.md)
- [Frozen source manifest]('''+d+'''source-manifest.json)
- [Final evidence and validation]('''+d+'''VERIFICATION.json)
- [DeFiFormal study](../../../deliverables/defiformal-study-2026-09-19/RESULT.md)
- [Mina case study](../../../deliverables/mina-recursion-study-2026-09-19/RESULT.md)
- [Requirement ownership](../../../openspec/changes/consolidated-language-kernel/traceability.md)

Design review is advisory and does not establish implementation, compiler correctness, native proof acceptance or financial settlement. No new build/proof/deployment ran in this consolidation. Static OpenSpec/Pel checks have their stated structural scope. [Index](index.md).
''','reference'))
put(b+'how-to.md',note('Turn a Moriarty requirement into one delivery task','''Select the requirement in the canonical traceability table and its U milestone. Read the full MPLR and EARS positive/hostile scenarios, then identify one missing observable predicate. Bind exact files, interfaces, supported semantics, target version, actual verification commands, resource limits and independent expected results in the existing Foreman workflow.

Label unavailable interfaces and missing commands honestly. Do not convert a symbolic Pel plan into a readiness claim. Keep project workflow metadata outside public program acceptance. Each result must retain complete effects, assumptions and unresolved duties; each changed semantic extension must requalify its dependent proof/ledger claims.

[OpenSpec workflow](../../../openspec/changes/consolidated-language-kernel/workflow.md) · [Index](index.md).
''','how-to'))
put(b+'tutorial.md',note('Paper tutorial — conditional exchange with recovery','''This is a design exercise, not a working program.

1. Sign an intention to trade 10 A for at least 20 B, with fees at most 1 A within an 11-A gross cap. Bind assets, domains, recipients, partial-fill rounding, evidence and recovery.
2. Fund escrow only under the applicable consent. Record the pending duty separately from delivery.
3. Require recipient acceptance and an authenticated document predicate. A digest alone establishes no document truth.
4. Commit an authorized partial fill and retain the remainder, cumulative fees, work and authority.
5. Race a late authenticated result with recovery. Timeout is not nonexecution; only the allowed terminal outcome consumes the entitlement.
6. Test a second permitted candidate and hostile changes to recipient, fees, predecessor, key, hidden reservation and residual debt.
7. Require native ZKIR correspondence and actual ledger effects before claiming completion. The optional federation is not necessary for direct authoring or submission.

[Design](explanation.md) · [Index](index.md).
''','tutorial'))
b='wiki/research/mina/';out='../../../deliverables/mina-recursion-study-2026-09-19/'
put(b+'index.md',note('Mina recursion case study','''Bounded pinned study of Mina, proof-systems, o1js and Snarky. Scrapling acquired selected official documentation; PixelRAG rendered the Pickles audit PDF. Source inspection and graphs inform native Midnight requirements; no Mina backend port, fresh proof or full-security audit is claimed.

- [Recursion lessons](explanation.md)
- [Sources and exact reading scope](reference.md)
- [How to qualify a recursive backend](how-to.md)
- [Paper tutorial](tutorial.md)
- [MPLR theory log](../mplr/index.md)
- [Consolidated design](../consolidated-design/index.md)
''','reference'))
put(b+'explanation.md',note('What Mina adds to native Midnight recursion','''Source observations: Pickles partitions verification between step/wrap and deferred/terminal checks. Its native verifier also checks accumulators. Kimchi binds verifier-index and recursive challenge context into transcripts. o1js separates proof parsing from verification, VK integrity from authorization, auxiliary data from proof statements, and proved computation from current-state settlement.

Recommendations: add MNR01–08 to the existing ZR contract for transcript order, deferred obligations/base masks, heterogeneous profiles, verified-value bindings, real proof modes, parameter/cache provenance, batch soundness and exact component assurance. Full details and source ranges are retained in the [study]('''+out+'''RESULT.md), [semantics review]('''+out+'''recursion-semantics.md) and [developer review]('''+out+'''developer-integration.md).

The acquired Mina proof-systems gitlink matches the acquired proof-systems pin, but other independently acquired dependency HEADs differ. No integrated build is established. Mina uses distinct proof artifacts and cryptographic assumptions; its design does not prove Midnight correctness. Historical audit findings remain tied to 2023 scope and do not certify current HEADs.

[Interactive graph]('''+out+'''graphify-out/graph.html) covers 400 selected files, with extracted/inferred relationships distinguished. It is navigation, not a proof graph. [Backend requirements](../../../docs/MORIARTY-BACKEND-REQUIREMENTS.md) · [Index](index.md).
'''))
put(b+'reference.md',note('Mina source and visual-reading reference','''Four latest-default-branch shallow sparse clones are pinned in [repos.json]('''+out+'''repos.json), with full tracked-file inventories retained in the study directory. This does not mean every Mina ecosystem repository was acquired. Graph scope selects 400 files across those repositories; 407 nodes and 1,516 directed edges are recorded with provenance. OCaml textual references are inferred, not compiler-resolved calls.

Scrapling captured ten selected official resources, including one PDF. PixelRAG pixelshot rendered all ten PDF pages. Extracted text was read for all pages; physical pages3/5/6 were visually inspected. No embedding or vector-index retrieval is claimed. The December2023 audit excludes a comprehensive custom-gate/lookup review and applies to its stated historical revision.

- [Source receipt inventory]('''+out+'''doc-receipts.json)
- [PixelRAG reading coverage]('''+out+'''pdf-processing.json)
- [Exact source verification]('''+out+'''SOURCE-VERIFICATION.json)
- [Graph scope]('''+out+'''graphify-out/GRAPH_REPORT.md)
- [Source inventory and immutable capture mapping](sources.md)

Root verified20 source-file hashes and59 cited ranges. This checks attribution, not runtime behavior. Source observations about an example's omitted verification, cache behavior and an SRS TODO are not reproduced exploits. [Index](index.md).
''','reference'))
put(b+'how-to.md',note('Qualify recursion without importing Mina assumptions','''Start with a legitimate funded base, two preserving financial steps and a bounded join. Bind every parent, key policy, schema/feature profile, transcript, deferred obligation and current consumption check. Mutate one property at a time while preserving an otherwise valid structure; use positive cases to avoid a vacuously rejecting system.

Export the complete certificate and independently run the real final verifier with the exact parameter/build tuple. Demonstrate failure of dummy-proof mode, wrong relation, stale state, missing parent, altered auxiliary effects and invalid deferred obligations. Measure all final checks and artifact sizes. Preserve the difference between proof validity, ledger acceptance and consensus finality.

[Required refinement clauses]('''+out+'''requirement-refinements.json) · [Index](index.md).
''','how-to'))
put(b+'tutorial.md',note('Paper tutorial — a valid proof under the wrong key','''This is a paper exercise, not an executed exploit.

1. An owner signs a policy requiring a financial relation R.
2. A solver supplies a valid proof for trivial relation T and a key whose data matches its hash.
3. Key integrity passes, but the program must reject T because the signed policy authorizes R.
4. Replace T with R, then alter an auxiliary recipient returned next to the proof. The consumer must use the bound statement/commitment, not unproved adjacent data.
5. Restore the recipient, then reuse a valid old-state proof after another transition has consumed that state. Current-state acceptance must reject the replay.
6. Finally verify the correct current successor with all deferred checks and real proof mode. Only this supported path can support the claimed effect.

[Explanation](explanation.md) · [Index](index.md).
''','tutorial'))
# Immutable captures + portable ledger + a source inventory in same transaction.
sl=json.loads((V/'wiki/meta/ledgers/source-ledger.json').read_text());sl['generated_at']=datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ');items=[]
for x in json.loads((M/'doc-receipts.json').read_text()):
 if x.get('status')==200:
  items.append((Path(x['path']),x['url']))
  if x.get('text'):items.append((Path(x['text']),x['url']))
pins={x['name']:x for x in json.loads((M/'repos.json').read_text())};stage=M/'source-captures';stage.mkdir(exist_ok=True)
for x in json.loads((M/'verified-source-files.json').read_text()):
 data=subprocess.check_output(['git','show',x['pin']+':'+x['path']],cwd=M/'repos'/x['repo']);q=stage/(x['sha256']+Path(x['path']).suffix);q.write_bytes(data)
 url=pins[x['repo']]['url'].removesuffix('.git')+'/blob/'+x['pin']+'/'+x['path'];items.append((q,url))
sourceindex='# Mina source inventory\n\nPinned source and selected documentation captures; reading scope is in reference.md. These source records do not assert independent audit or implementation acceptance.\n\n| Portable source | Origin | Immutable capture |\n|---|---|---|\n'
for p,u in items:
 u=quote(u,safe=':/%?=&');h=hashlib.sha256(p.read_bytes()).hexdigest();sid=stable_source_id('url',u,h);cap='.raw/captured/mina-20260919/'+h+p.suffix
 if not(V/cap).exists():put(cap,file=p)
 sl['sources'][sid]={'origin':{'kind':'url','locator':u},'title':p.name,'content_kind':'document','authority':'primary','review_status':'active','content_sha256':h,'ingested_at':'2026-09-19','retrieved_at':'2026-09-19','refresh_due':'2026-10-19','pages':[b+'reference.md'],'independence_key':u.split('/')[2],'captured_payload':cap,'reading_scope':'Selected source ranges and documentation; exact review scope in Mina study. No build/proof/deployment or whole-repository audit.'}
 sourceindex+=f'| {sid} | [Source]({u}) | [Capture](../../../{cap}) |\n'
put(b+'sources.md',note('Mina immutable source inventory',sourceindex,'reference'));put('wiki/meta/ledgers/source-ledger.json',json.dumps(sl,indent=2)+'\n')
# Scope reconciliation preserves original claims/receipts.
p='wiki/decisions/pcd-midnight-native-architecture.md';s=(V/p).read_text();s=s.replace('status: active','status: superseded',1);s=s.replace('# Midnight-native PCD architecture decision','# Midnight-native PCD architecture decision\n\n**Scope supersession, 2026-09-19:** The [consolidated design](../../docs/MORIARTY-CONSOLIDATED-DESIGN.md) now controls target scope. Comprehensive native recursion, private handoff and bounded multi-parent composition are planned for the user-assumed approximately March2027 horizon. The earlier categorical rejection of DAG/recursive history below is historical, not an active restriction. Ledger induction remains a distinct evidence mode and cannot close MC03/MC06. Source observations and measurements below retain their original dates/pins; none asserts current release support. Original claim IDs and raw receipts remain unchanged.');put(p,s)
p='wiki/index.md';s=(V/p).read_text();s=s.replace('# Moriarty language wiki index','# Moriarty language wiki index\n\n## Governing design — 2026-09-19 consolidation\n\n[One vision, language/kernel boundary and U0–U7 roadmap](research/consolidated-design/index.md) · [Mina recursion requirements study](research/mina/index.md) · [Next ZKIR/recursion contract](../docs/MORIARTY-BACKEND-REQUIREMENTS.md). Older sections below preserve dated research and evidence; they do not override current scope or select competing queues.');start=s.index('**Recommended direction.**');end=s.index('The [PCD integration amendment]',start);s=s[:start]+'''**Historical recommendation.** The September11 ledger-induction-first scope is preserved as dated evidence. Its blanket rejection of recursive/DAG history is superseded by the current full-recursion target. The old transcript/fee/guard observations are tied to their original pins and require reinspection before current support claims.

'''+s[end:];s=s.replace('## Completion sprint schedule','## Retained completion sprint acceptance');s=s.replace('assign the remaining language, K, native PCD, financial and developer work to twelve sprints','preserve twelve-sprint acceptance lineage under the single current U0–U7 roadmap');put(p,s)
p='wiki/hot.md';put(p,note('Moriarty current direction','''[One consolidated vision, design and roadmap](research/consolidated-design/index.md) controls the permissionless native Midnight language and optional federated kernel boundary. [Backend requirements](../docs/MORIARTY-BACKEND-REQUIREMENTS.md) specify ZR01–16 plus Mina-derived MNR01–08. The user assumes comprehensive Midnight recursion in about six months; native recursive/private composition remains required, without a Lean dependency.

[Six-expert synthesis](../deliverables/consolidated-design-2026-09-19/CONSENSUS.md) records scope and review; no implementation/proof completion follows from votes. [Mina](research/mina/index.md), [DeFiFormal](../deliverables/defiformal-study-2026-09-19/RESULT.md), [MPLRs](research/mplr/index.md), and the existing NEAR/Daml/Simplicity/Anoma/APSS/OWS banks supply research evidence. All U product exits remain open pending exact evidence.
''','reference'))
for id,body in [('020','Mina motivates specification-to-constraint traceability for each used custom gate, lookup and binding. Historical recursion-layer audits do not qualify every primitive or changed dependency tuple.'),('022','Research typed evidence objects that distinguish parsed proof, statement-bound value, private witness, auxiliary output and authenticated actual effect. Distinct representation identities require correspondence; VK integrity and authorization are separate.'),('023','Research a static/effect discipline that requires every mandatory predecessor and predicate to reach actual verification. Reading proof fields, false verification guards, dummy modes and ignored checks must not satisfy that obligation.'),('027','Research typed deferred-verification obligations with producer/consumer invariants, legitimate base masks, complete finalization and bounded-fan-in composition over growing finite history. Transcript/challenge schedules and batching soundness are part of the native contract; cumulative signed budgets and uniqueness remain separate state obligations.')]:
 p=f'wiki/research/mplr/MPLR-{id}.md';s=(V/p).read_text()+'\n## Mina recursion research refinement — 2026-09-19\n\n'+body+'\n\n[Source study](../mina/reference.md). This is a research direction and backend requirement, not an implemented theorem or feature.\n';put(p,s)
p='wiki/research/mplr/index.md';put(p,(V/p).read_text()+'\n2026-09-19 — [Mina recursion case study](../mina/index.md) refines MPLR-020/022/023/027. [Consolidated traceability](../../../openspec/changes/consolidated-language-kernel/traceability.md) maps all35 existing IDs to the single roadmap; no ID is recycled or closed by design review.\n')
p='wiki/contradictions.md';put(p,(V/p).read_text()+'\n## Recursion scope reconciliation — 2026-09-19\n\nThe older categorical rejection of DAG/recursive history is superseded as target scope. The user assumes comprehensive Midnight recursion around March2027; full native financial recursion/private composition remains required. Ledger induction is a separate early evidence mode, not closure of MC03/MC06. Old source facts retain original pins. [Consolidated design](../docs/MORIARTY-CONSOLIDATED-DESIGN.md) and [backend requirements](../docs/MORIARTY-BACKEND-REQUIREMENTS.md) govern. Mina/DeFiFormal mechanisms are comparative evidence, not portable Midnight proofs.\n')
p='wiki/log.md';put(p,(V/p).read_text()+'\n2026-09-19 — Consolidated design/U0–U7 roadmap, six-expert deliberation, explicit next ZKIR/recursion requirements and Mina case study filed. Four pinned shallow sparse repositories; Scrapling10selectedresources; PixelRAG10auditpages rendered/textread and3visualchecks; bounded400-file graph. MPLR020/022/023/027 refined, oldPCDscope superseded explicitly. All implementation exits remain open.\n')
bundle={'schema':'claude-obsidian.transaction.v1','operation_id':'consolidated-design-mina-20260919','operation_type':'autoresearch','writes':list(W.values()),'expected_hashes':E,'address_requests':[],'source_manifest_updates':{}}
(R/'ingest-consolidation.json').write_text(json.dumps(bundle,indent=2));print({'writes':len(W),'source_records':len(items)})
