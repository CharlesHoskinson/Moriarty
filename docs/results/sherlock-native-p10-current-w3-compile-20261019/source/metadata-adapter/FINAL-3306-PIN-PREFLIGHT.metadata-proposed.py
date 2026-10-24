"""Existing final data preflight, rebound only to current W3/source and actual receipts.
Never imports candidate code or starts native work. Root-selected use only.
"""
from pathlib import Path
import json,hashlib,os
K=Path(__file__).resolve().parent;D=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-integration-metadata-plan-20261019');R=D.parent;P=D/'future-actual-03'
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb')as f:
  for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
def load(p):return json.loads(Path(p).read_text())
def observed(p):
 s=os.lstat(p);return{'device':s.st_dev,'inode':s.st_ino,'size':s.st_size,'mtime_ns':s.st_mtime_ns,'mode':s.st_mode,'links':s.st_nlink}
c=load(P/'config.final.json');admit=load(P/'ROOT-ACTUAL-ADMISSION.json');assert sha(P/'config.final.json')==admit['config']['sha256']
for pin in [*c['bindings'].values(),*c['review_receipts'].values(),c['source_manifest'],c['old_home_stat_manifest']]:assert sha(pin['path'])==pin['sha256'],pin['path']
m=load(c['old_home_stat_manifest']['path']);assert len(m['paths'])==3306 and len(m['directory_memberships'])==1063
bad=[p for p,s in m['paths'].items()if observed(p)!=s];badmem=[p for p,n in m['directory_memberships'].items()if sorted(os.listdir(p))!=n]
assert not bad and not badmem,{'statDiffPaths':bad,'membershipDiffPaths':badmem}
operand_path=K/'EXACT-CURRENT-OPERAND-PINS.json'
assert sha(operand_path)==admit['exact_current_operand_list']['sha256']
rows=load(operand_path);assert type(rows)is list and rows
indexed={p['path']:p for p in rows};assert len(indexed)==len(rows)
assert set(admit['required_operand_paths']).issubset(indexed)
for p in rows:assert sha(p['path'])==p['sha256'] and Path(p['path']).stat().st_size==p['bytes'],p['path']
# Retain existing selected historical generated-byte input checks unchanged.
for v in load(R/'native06-current-cached-p7-integration-metadata-20261018/LIVE-HASH-QUALIFICATIONS-AND-PREFLIGHT-INPUTS.json')['selected8live_metadata']:assert sha(v['path'])==v['expected_sha256'],v['path']
# Root selects a current complete W3 source rehash before compile.
sm=load(c['source_manifest']['path']);root=Path(c['paths']['worktree']);actual={}
assert sm['root']==str(root)and sm['head']=='229770592adbf3fffdf5fe77f8fa82c8c3aee194'and sm['excluded_directories']==['.git']
for base,dirs,files in os.walk(root,followlinks=False):
 if Path(base)==root:dirs[:]=[x for x in dirs if x!='.git']
 for d in dirs:assert not(Path(base)/d).is_symlink()
 for f in files:
  if Path(base)==root and f=='.git':continue
  p=Path(base)/f;assert not p.is_symlink()and p.is_file();actual[p.relative_to(root).as_posix()]=sha(p)
assert actual==sm['files'],'current W3 complete membership/hash mismatch'
for p in sm['head_files']:assert sha(p['path'])==p['sha256'],p['path']
subject=load(c['bindings']['subject']['path']);assert subject['base_head']==sm['head']and subject['worktree']==str(root)
assert subject['cargo_lock_sha256']==actual['Cargo.lock']
assert len(subject['candidate_files'])==subject['expected_candidate_file_count']==458
assert all(actual[x['path']]==x['sha256']for x in subject['candidate_files'])
for name in ['security','nonsecurity']:
 rec=load(c['review_receipts'][name]['path']);assert rec['source_manifest_sha256']=='1632a29ac2f678640fe9e1821ddace5845094dc735fbec21f2f1fd3e4d186c27'
compat=admit['manifest_schema_compatibility']
for q in [compat['adoption'],compat['proof'],compat['original_review_manifest'],compat['consumed_manifest']]:assert sha(q['path'])==q['sha256']and Path(q['path']).stat().st_size==q['bytes']
assert load(compat['adoption']['path'])=={'decision':'ADOPTED_METADATA_SHAPE_EQUIVALENCE','compatibility_proof':compat['proof']}
assert c['source_manifest']=={k:compat['consumed_manifest'][k]for k in ['path','sha256']}
before=load(compat['original_review_manifest']['path']);after=load(compat['consumed_manifest']['path'])
assert len(before['head_files'])==len(after['head_files'])==5
assert all(set(q)=={'path','sha256','bytes'} for q in before['head_files'])and all(set(q)=={'path','sha256'}for q in after['head_files'])
assert after==dict(before,head_files=[{k:q[k]for k in ['path','sha256']}for q in before['head_files']])
assert(c['total_wall_seconds'],c['limits']['F']['wall_seconds'],c['supervisor_cpu_seconds'])==(6033,2133,10576)
argv=load(P/'ROOT-ACTUAL-ARGV.json');script=argv['argv'][-1]
assert argv['cwd']==str(root) and argv['config_final_sha256']==sha(P/'config.final.json')
assert script.count(c['bindings']['runner']['path'])==1 and script.count(str(P/'config.final.json'))==1
assert script.endswith('--config-sha256 '+sha(P/'config.final.json'))
cg='/sys/fs/cgroup'+c['cgroup']['envelope_path']+'/'+c['cgroup']['supervisor_leaf_name']+'/cgroup.procs'
assert script.splitlines()[0]=='echo $$ > '+cg and script.count(cg)==1
print(json.dumps({'status':'ALL3306_STATS_1063_MEMBERSHIPS_BINDINGS_CURRENT_ACCOUNTING_RECEIPTS_FINALSOURCE_CURRENT_W3_FULL_MANIFEST_MATCH','source_files':len(actual),'scope':'Instant observations only; guard strict rechecks remain. No lock/continuous immutability/native invocation.','config_sha256':sha(P/'config.final.json')}))
