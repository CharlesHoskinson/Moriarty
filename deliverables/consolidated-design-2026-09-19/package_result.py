from pathlib import Path
import shutil,json,hashlib
r=Path('/home/charl/research/moriarty-consolidation-2026-09-19');m=Path('/home/charl/research/mina-recursion-2026-09-19');v=Path('/home/charl/Moriarty-aeon-study')
(r/'CONSENSUS.md').write_text('''# Consolidated design review\n\nSix independent reviewers (three GPT-6 Astra medium through native host routing, three Claude Fable 5.1 medium through the Claude CLI) approve the expanded v2 design. Approval concerns the design, not implemented behavior, proofs or release. CLI prompts, returned model identities and receipts are retained.\n\nThe initial v1 round had two approvals and four changes-requested verdicts. Original findings and verdicts are retained. Corrections resolved lifecycle authorization, SP07/SP08 dependencies, U6 requalification, actual SP10 operators, full native recursion, optional federation, numeric conventions and evidence scope. The user-requested Mina expansion was independently reviewed by all six reviewers as v2, whose packet digest is fa2dfa966981bcfa3a3be0c3175be8305fa070b6133cfe27e4978434741c8c5e.\n\nAfter the v2 freeze, the Pel input alias changed to the existing artifact:approved-spec slot with a corresponding workflow explanation; Astra kernel and delivery explicitly reviewed these two replacement files. Further nonblocking review corrections qualify backend-specific arithmetic and proof shape, distinguish extracted audit text from visual checking, and reconcile MNR responsibility/U3/U5 mappings. These are disclosed post-freeze edits, not a claim that all six reviewed identical final bytes. See the language closure addendum for their independent review. Final hashes are in final-candidate-manifest.json.\n\nConsensus preserves permissionless supported programs, native Midnight ZKIRv3, no Lean dependency, bounded stages and growing finite histories, complete effects and surviving duties, optional federated coordination and full native private/multi-parent recursion. The approximately March 2027 recursion horizon is the user's planning assumption. Mina supplies eight mandatory refinements of sixteen backend requirements; it does not supply Midnight-compatible proofs or transferred assurance.\n\nThe next implementation milestone is U0 semantic and target-matrix qualification followed by the staged U1–U7 roadmap. Required executable and ledger evidence remains open. README and GitHub Pages revisions were requested after this review and are separately validated documentation.\n''')
manifest={str(p.relative_to(r/'candidate')):hashlib.sha256(p.read_bytes()).hexdigest() for p in (r/'candidate').rglob('*') if p.is_file()};(r/'final-candidate-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
for p in (r/'candidate').rglob('*'):
 if p.is_file():
  rel=p.relative_to(r/'candidate'); dest=v/rel;before=r/'before'/rel
  if dest.exists() and not before.exists(): before.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(dest,before)
  dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,dest)
for src,name in [(m,'mina-recursion-study-2026-09-19'),(r,'consolidated-design-2026-09-19')]:
 dest=v/'deliverables'/name;dest.mkdir(parents=True,exist_ok=True)
 for p in src.iterdir():
  if p.is_file() and p.suffix in ('.md','.json','.txt','.py') and p.name not in ('ingest-consolidation.json',):shutil.copy2(p,dest/p.name)
 if src==m:
  for d in ['graphify-out','docs','pdf-tiles']:
   shutil.copytree(src/d,dest/d,dirs_exist_ok=True)
(v/'deliverables/consolidated-design-2026-09-19/VERIFICATION.json').write_text(json.dumps({'status':'verification-in-progress'},indent=2)+'\n')
print('Copied candidate and evidence; no git commit or push.')
