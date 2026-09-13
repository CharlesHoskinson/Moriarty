#!/usr/bin/env python3
"""Scratch candidate09 regression; harmless Bash PATH only, no K/native tools."""
from pathlib import Path
import copy,hashlib,importlib.util,json,os,subprocess,tempfile,unittest
from unittest.mock import patch
O=Path(__file__).resolve().parent
R=O.parents[3]
S=O/'candidate-source-09.py'
spec=importlib.util.spec_from_file_location('candidate09',S);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
base=json.loads((R/'.moriarty-dev/k-macro05-trace106-pins.json').read_text())
old=json.loads((O/'path-stage-reproducer-08.json').read_text())
# Execute ONLY already recorded PATH assignments and printf, not sourced loaders.
run=subprocess.run(old['argv'],env={'PATH':base['childEnv']['PATH'],'HOME':os.environ['HOME'],'LANG':'C.UTF-8'},capture_output=True,text=True,check=True)
actual={line.split('=',1)[0]:line.split('=',1)[1].split(':') for line in run.stdout.splitlines()}
stages={'krun':actual['krun'],'kast':actual['kast'],'setenv':actual['kast'],'checkJava':actual['checkJava']}
pins=copy.deepcopy(base)
pins['setenvNativeDir']=actual['setenv'][0]
pins['setenvNativeDirMissing']=True
byrole={i['role']:i for i in pins['selectedExecutables']}
for role,command in [(n,n) for n in m.REQUIRED_HELPERS+('java',)]+[('llvm','llvm-krun'),('kore','kore-print')]:
 p=m.resolve_command(command,stages['krun']);assert p is not None
 item={'role':role,'command':command,'path':str(p),'sha256':sha(p),'resolvedTarget':str(p.resolve())}
 if p.is_symlink():item['symlinkTarget']=os.readlink(p)
 if role not in byrole:pins['selectedExecutables'].append(item)
 else:byrole[role].update(item)
for item in pins['selectedExecutables']:
 item['resolvedTarget']=str(Path(item['path']).resolve())
pins['commandSelections']={stage:{name:str(m.resolve_command(name,dirs)) for name in m.STAGE_COMMANDS[stage]} for stage,dirs in stages.items()}
obs={}
class Regression(unittest.TestCase):
 def test_actual_bash_stage_paths_match_candidate(self):
  seen=[];original=m.resolve_command
  def track(n,dirs):seen.append((n,list(dirs)));return original(n,dirs)
  with patch.object(m,'resolve_command',track):m.verify_wrapper_and_parser(copy.deepcopy(pins))
  cursor=0
  for stage,names in m.STAGE_COMMANDS.items():
   self.assertEqual([p for _,p in seen[cursor:cursor+len(names)]],[stages[stage]]*len(names),stage);cursor+=len(names)
  obs['actualStagePathsMatch']=True
 def test_first_duplicate_only_wrapper_prepend_matches_bash(self):
  directory=pins['wrapperPathDirs'][0]
  block=old['argv'][4].split('export PATH\n',1)[0]+'export PATH\nprintf "%s" "$PATH"'
  incoming=[directory,'/usr/bin',directory,'/bin']
  result=subprocess.run([old['argv'][0],'--noprofile','--norc','-c',block],env={'PATH':':'.join(incoming)},capture_output=True,text=True,check=True)
  self.assertEqual(m.prepend_path_dir(incoming,directory),result.stdout.split(':'))
  obs['firstDuplicateSemanticsMatch']=True
 def test_stage_selected_uncommitted_helper_rejected(self):
  with tempfile.TemporaryDirectory(prefix='e04-2-09-') as raw:
   candidate=Path(raw)/'dirname';candidate.write_text('#!/bin/sh\nexit 0\n');candidate.chmod(0o755)
   mutated=copy.deepcopy(pins);mutated['commandSelections']['checkJava']['dirname']=str(candidate)
   original=m.resolve_command
   def select(name,dirs):
    if name=='dirname' and dirs==stages['checkJava']:return candidate
    return original(name,dirs)
   hashed=[];original_hash=m.sha256_file
   def hashing(path):hashed.append(str(path));return original_hash(path)
   with patch.object(m,'resolve_command',select),patch.object(m,'sha256_file',hashing):
    error=None
    try:m.verify_selected(mutated);m.verify_wrapper_and_parser(mutated)
    except m.PreflightError as exc:error=exc.code
   obs['uncommittedStageSelection']={'rejected':error is not None,'error':error,'selectedPathHasEntry':any(i['path']==str(candidate) for i in mutated['selectedExecutables']),'selectedPathHashed':str(candidate) in hashed,'mechanism':'Mocked resolver for checkJava dirname only; actual temporary executable file never executed. Both real verify_selected and verify_wrapper_and_parser called.'}
   self.assertIsNotNone(error,'uncommitted stage helper accepted without bytes/symlink/ownership binding')
 def test_non_boolean_native_missing_rejected(self):
  accepted=[]
  for value in ['false','true',1,[],{}]:
   mutated=copy.deepcopy(pins);mutated['setenvNativeDirMissing']=value
   try:m.verify_wrapper_and_parser(mutated)
   except m.PreflightError:continue
   accepted.append(value)
  obs['malformedNativeMissingAccepted']=accepted
  self.assertEqual(accepted,[])
 def test_present_native_expectation_and_changed_native_path_rejected(self):
  for value in [False,None]:
   mutated=copy.deepcopy(pins);mutated['setenvNativeDirMissing']=value
   with self.assertRaises(m.PreflightError):m.verify_wrapper_and_parser(mutated)
  mutated=copy.deepcopy(pins);mutated['setenvNativeDir']='/tmp/uncommitted-native'
  with self.assertRaises(m.PreflightError):m.verify_wrapper_and_parser(mutated)
 def test_real_helpers_have_valid_host_link_and_target_binding(self):
  own=json.loads((R/'.moriarty-dev/k-macro05-trace106-ownership.json').read_text())
  entries=m.load_host_evidence(own)
  _,uid=m.read_namespace_mapping(own)
  checked=[]
  for item in pins['selectedExecutables']:
   if item['role'] not in m.REQUIRED_HELPERS+('java','llvm','kore'):continue
   for path in {item['path'],str(Path(item['path']).resolve())}:
    m.verify_ownership(path,entries,uid,own['hostRootUid']);checked.append(path)
  obs['actualHostOwnershipChecks']=len(checked)
 def test_bound_helper_hash_and_symlink_drift_rejected(self):
  for field,value in [('sha256','0'*64),('symlinkTarget','not-the-real-link'),('resolvedTarget','/not/the/target')]:
   mutated=copy.deepcopy(pins);item=next(i for i in mutated['selectedExecutables'] if i['role']=='dirname');item[field]=value
   with self.assertRaises(m.PreflightError):m.verify_selected(mutated)

if __name__=='__main__':
 result=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Regression))
 own=json.loads((R/'.moriarty-dev/k-macro05-trace106-ownership.json').read_text());host=json.loads(Path(own['hostEvidencePath']).read_text());entries={i['path']:i for i in host['entries']}
 helpers=[]
 for item in pins['selectedExecutables']:
  if item['role'] not in m.REQUIRED_HELPERS+('java','llvm','kore'):continue
  p=Path(item['path']);helpers.append({'role':item['role'],'path':str(p),'resolvedTarget':str(p.resolve()),'sha256':sha(p),'symlinkTarget':os.readlink(p) if p.is_symlink() else None,'directHostEvidence':str(p) in entries,'targetHostEvidence':str(p.resolve()) in entries,'hostWritable':os.access(p,os.W_OK)})
 report={'kind':'LL01-independent-E04-2-regression/09','testsRun':result.testsRun,'failures':[{'test':str(t),'trace':s} for t,s in result.failures],'errors':[{'test':str(t),'trace':s} for t,s in result.errors],'observations':obs,'actualBashPaths':actual,'helperObservations':helpers,'candidateSHA256':sha(S),'patchSHA256':sha(O/'author-patch-09.json'),'scope':'Scratch candidate functions and actual harmless pinned Bash PATH statements. No K, Java, strace, native diagnostic or guarded dispatch. Synthetic pins generated in memory from real helper observations; not adopted manifests or fabricated host evidence. Ownership coverage is inspected actual evidence membership, not refreshed admission. No canonical source edits.','requiredCorrection':['Every commandSelections path must map to a required selectedExecutables entry by path (role or stage-specific role may differ). Enforce bytes, exact symlink/non-symlink and resolved target binding; every such entry must traverse full host link/target ownership checks. Do not skip checks when by_role[name].path differs.','Require setenvNativeDirMissing as an explicit bool and enforce reviewed fixed missing=True policy here; reject a non-directory existing file or dangling symlink, not merely is_dir=False.','Turn09 setenv means pre-checkJava uname/dirname phase, whereas earlier reproducer setenv means post-native prepend. Name both phases unambiguously and cover later commands. REQUIRED_HELPERS still omits awk required by Opus C3; recheck actual reachable helper set.']}
 (O/'independent-e04-2-regression-09.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'testsRun':result.testsRun,'failures':len(result.failures),'errors':len(result.errors),'observations':obs}))
 raise SystemExit(0 if result.wasSuccessful() else 1)
