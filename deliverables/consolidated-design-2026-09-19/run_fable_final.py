from pathlib import Path
import subprocess,concurrent.futures,json,hashlib,time
r=Path('/home/charl/research/moriarty-consolidation-2026-09-19')
packet=(r/'review-candidate-v1.txt').read_text()
roles={
'language':'PL semantics expert: small total core, refinements/effects/linear authority/assets/liabilities, certified primitives, intent-to-circuit correspondence, native Midnight PCD ledger induction versus recursive foreign/history proofs. Resolve contradictions, propose one design and phased roadmap.',
'kernel':'Distributed financial PL security expert: precise Moriarty / Federated DeFi Kernel / Midnight / external adapter responsibility boundaries. APSS, permissionless deployment versus consent and optional service policy, ZK/MPC/TEE assumptions, conditional escrow, partial transactions, recovery, solver delegation and payment-service semantics.',
'delivery':'Financial DSL and delivery expert: consolidate P/C/K/MC/SP plans into one dependency roadmap, composable libraries and ACTUS/DeFi breadth, native certified jets, exact arithmetic and price conventions, EARS/OpenSpec/Pel/MPLR traceability, next financial vertical slice with measurable evidence.'}
def run(role,lens):
 prompt='You are an independent non-author design reviewer. '+lens+'\nReview the whole candidate below. Output approved, changes_requested or insufficient_evidence FOR DESIGN CONSOLIDATION ONLY, followed by exact substantive findings with file/section, necessary correction and reasoning. Explicitly open implementation is not a defect in a proposal. No tools or code implementation. No other reviewer outputs available. Aim for concise rigorous review.\n\n'+packet
 (r/f'final-fable-{role}-prompt.txt').write_text(prompt)
 args=['claude','-p','--model','claude-fable-5-1','--effort','medium','--tools','','--permission-mode','plan','--no-session-persistence','--safe-mode','--no-chrome','--output-format','json']
 start=time.time()
 try:
  p=subprocess.run(args,input=prompt,text=True,capture_output=True,cwd='/tmp',timeout=1200)
  (r/f'final-fable-{role}-response.json').write_text(p.stdout);(r/f'final-fable-{role}-stderr.txt').write_text(p.stderr)
  receipt={'role':role,'requested_model':'claude-fable-5-1','effort':'medium','exit_code':p.returncode,'seconds':time.time()-start,'prompt_sha256':hashlib.sha256(prompt.encode()).hexdigest()}
  if p.returncode==0:
   j=json.loads(p.stdout); receipt.update(is_error=j.get('is_error'),subtype=j.get('subtype'),model_usage=j.get('modelUsage'))
   identity=j.get('modelUsage',{}).get('claude-fable-5-1',{}).get('canonicalModel')
   receipt['identity_verified']=identity=='claude-fable-5-1'
   if not j.get('is_error') and receipt['identity_verified']:(r/f'final-fable-{role}.md').write_text(j['result'])
 except Exception as e: receipt={'role':role,'failure':str(e),'seconds':time.time()-start}
 (r/f'final-fable-{role}-receipt.json').write_text(json.dumps(receipt,indent=2));print(json.dumps(receipt),flush=True)
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool: list(pool.map(lambda x:run(*x),roles.items()))
