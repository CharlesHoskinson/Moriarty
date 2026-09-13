#!/usr/bin/env python3
import copy,fcntl,hashlib,importlib.util,json,os,sys,tempfile,unittest
from pathlib import Path
from unittest.mock import patch
O=Path(__file__).resolve().parent;R=O.parents[3]
sp=importlib.util.spec_from_file_location('candidate11',O/'candidate-source-11.py');m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
base=json.loads((R/'.moriarty-dev/k-macro05-trace106-pins.json').read_text());snapshot=json.loads((O/'snapshot-inventory-draft-03.json').read_text())
def digest(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
for role,path in [('bwrap','/usr/bin/bwrap'),('passwd','/etc/passwd'),('nsswitch','/etc/nsswitch.conf')]:
 if not any(i['path']==path for i in base['selectedExecutables']):base['selectedExecutables'].append({'role':role,'path':path,'sha256':digest(path),'resolvedTarget':str(Path(path).resolve())})
obs={}
class Tests(unittest.TestCase):
 def setUp(self):
  self.tmp=tempfile.TemporaryDirectory();self.here=Path(self.tmp.name);self.pins=copy.deepcopy(base);self.snap=copy.deepcopy(snapshot);self.persist()
 def tearDown(self):self.tmp.cleanup()
 def persist(self):
  p=self.here/m.SNAPSHOT_NAME;p.write_text(json.dumps(self.snap));self.pins['manifests'].update(snapshot=m.SNAPSHOT_NAME,snapshotSHA256=digest(p))
 def test_baseline_full506_and_fd_seals(self):
  result=m.verify_snapshot(self.pins,self.here);self.assertEqual(len(result['files']),506)
  fds=m.seal_snapshot(result)
  try:
   self.assertEqual(len(fds),506)
   for fd in fds:self.assertEqual(fcntl.fcntl(fd,fcntl.F_GET_SEALS)&m.SEAL_FLAGS,m.SEAL_FLAGS)
   argv=m.build_bwrap_argv(result,fds,self.pins['diagnosticArgv']);self.assertEqual(argv[argv.index('--')+1:],self.pins['diagnosticArgv']);self.assertEqual(argv.count('--ro-bind-data'),506)
   obs['full506Sealed']=True
  finally:m._close_fds(fds)
  for fd in fds:
   with self.assertRaises(OSError):os.fstat(fd)
 def test_missing_manifest_digest_rejected(self):
  del self.pins['manifests']['snapshotSHA256']
  with self.assertRaises(m.PreflightError):m.verify_snapshot(self.pins,self.here)
 def test_changed_manifest_digest_rejected(self):
  self.pins['manifests']['snapshotSHA256']='0'*64
  with self.assertRaises(m.PreflightError):m.verify_snapshot(self.pins,self.here)
 def test_real_uid_required(self):
  with patch.object(m.os,'getuid',return_value=1001):
   with self.assertRaises(m.PreflightError):m.verify_snapshot(self.pins,self.here)
 def test_real_gid_required(self):
  with patch.object(m.os,'getgid',return_value=1001):
   with self.assertRaises(m.PreflightError):m.verify_snapshot(self.pins,self.here)
 def test_effective_uid_gid_rejected(self):
  for fn in ['geteuid','getegid']:
   with patch.object(m.os,fn,return_value=1001):
    with self.assertRaises(m.PreflightError):m.verify_snapshot(self.pins,self.here)
 def test_identity_files_exact_pair(self):
  self.snap['osIdentityFiles']=self.snap['osIdentityFiles'][:1];self.persist()
  with self.assertRaises(m.PreflightError):m.verify_snapshot(self.pins,self.here)
 def test_fixed_root_bound(self):
  self.snap['root']='/home/charl';self.persist()
  with self.assertRaises(m.PreflightError):m.verify_snapshot(self.pins,self.here)
 def test_mode_extra_bits_rejected(self):
  self.snap['files'][0]['mode']|=0o4000;self.persist()
  with self.assertRaises(m.PreflightError):m.verify_snapshot(self.pins,self.here)
 def test_new_compiled_symlink_directory_rejected(self):
  walk=m.os.walk
  def extra(*a,**kw):
   for index,(dp,ds,fs) in enumerate(walk(*a,**kw)):
    yield dp,ds+(['new-symlink-directory'] if index==0 else []),fs
  # No production directory changes. Model an extra directory entry that should
  # be lstat-checked; original verifier never examines directory names at all.
  with patch.object(m.os,'walk',extra):
   with self.assertRaises(m.PreflightError):m.verify_snapshot(self.pins,self.here)
 def test_file_hash_drift_rejected_and_partial_fds_closed(self):
  bad=copy.deepcopy(snapshot);bad['files'][1]['sha256']='0'*64
  before=set(os.listdir('/proc/self/fd'))
  with self.assertRaises(m.PreflightError):m.seal_snapshot(bad)
  self.assertEqual(set(os.listdir('/proc/self/fd')),before)
 def test_temporal_seal_excludes_source_mutation(self):
  p=self.here/'sample';p.write_bytes(b'old');entry={'path':str(p),'sha256':digest(p),'sizeBytes':3,'mode':0o600};fd=m.seal_snapshot_file(entry)
  try:
   p.write_bytes(b'new');self.assertEqual(os.read(fd,3),b'old')
   with self.assertRaises(OSError):os.write(fd,b'x')
  finally:os.close(fd)
 def test_second_manifest_hash_parse_swap_rejected(self):
  calls=[];checked=[];original=m._check_manifest
  altered=copy.deepcopy(self.snap)
  altered['files'][0]={'path':'/etc/passwd','sha256':digest('/etc/passwd'),'sizeBytes':Path('/etc/passwd').stat().st_size,'mode':Path('/etc/passwd').stat().st_mode & 0o777}
  def check(here,name,expected):
   original(here,name,expected);checked.append(name)
   if len(checked)==2:(here/name).write_text(json.dumps(altered))
  def preflight(*args,**kw):m.verify_snapshot(self.pins,self.here);return {'snapshot':'actually verified original'}
  with patch.object(m,'preflight',preflight),patch.object(m,'_check_manifest',check):
   error=None
   try:m.run_shim(pins=self.pins,here=self.here,home='/home/charl',execve=lambda *a:calls.append(a),getrlimit=lambda *_:(8388608,-1),setrlimit=lambda *_:None)
   except m.PreflightError as exc:error=exc.code
  obs['manifestTemporalSwap']={'error':error,'injectedExecCalls':len(calls),'unverifiedEtcPasswdSnapshotAdmitted':bool(calls and '/etc/passwd' in calls[0][1]),'mechanism':'Actual verify_snapshot first; after second manifest hash returns, fixture metadata is replaced before json reread. No original files changed.'}
  self.assertIsNotNone(error,'second-read metadata swap bypassed verified union and root')
 def test_injected_final_exec_preserves_original_and_closes_fds(self):
  called=[];fds=[];original=m.seal_snapshot
  def tracked(s):fds.extend(original(s));return fds
  with patch.object(m,'preflight',return_value={'test':'injected prior checks'}),patch.object(m,'seal_snapshot',tracked):
   result=m.run_shim(pins=self.pins,here=self.here,home='/home/charl',execve=lambda *a:called.append(a),getrlimit=lambda *_:(8388608,-1),setrlimit=lambda *_:None)
  self.assertEqual(len(called),1);self.assertEqual(result['argv'][result['argv'].index('--')+1:],base['diagnosticArgv'])
  for fd in fds:
   with self.assertRaises(OSError):os.fstat(fd)

if __name__=='__main__':
 result=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Tests))
 report={'testsRun':result.testsRun,'failures':[{'test':str(t),'trace':s} for t,s in result.failures],'errors':[{'test':str(t),'trace':s} for t,s in result.errors],'observations':obs,'sourceSHA256':digest(O/'candidate-source-11.py'),'scope':'Scratch source11 only; actual full506 memfds/immutable seals, temporary fixture manifests. UID and extra directory failures use controlled injection. finalexec injected; prior preflight only mocked in that isolated control. No native K/Java/strace. No canonical modifications.'}
 (O/'independent-snapshot-regression-11.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'run':result.testsRun,'failures':len(result.failures),'errors':len(result.errors)}))
