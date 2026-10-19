import hashlib,importlib.util,json,signal,sys,tempfile,unittest,resource
from pathlib import Path
from unittest import mock
def L(n):
 p=Path(__file__).with_name(n+'.py');s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);sys.modules[n]=m;s.loader.exec_module(m);return m
Ms=[L('bounded_sdk_compile'),L('cached_current_build')]
H=next((m for m in Ms if hasattr(m,'enforce_post_p11_historical_debits')),Ms[0])
def F(n,d=None):
 for m in Ms:
  if hasattr(m,n):return getattr(m,n)
 return d
R=F('Refusal',RuntimeError)
def nd(o,w):
 if not o:raise R(w)
ND=F('need',nd)
def sh(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def T(p):return{'path':str(p),'bytes':Path(p).stat().st_size,'sha256':sh(p)}
class G:
 Refusal=R;need=staticmethod(ND)
 def __init__(s,c):s.c=c
 @staticmethod
 def exact(o,k,w):ND(type(o)is dict and set(o)==set(k.split()),'keys '+w)
 @staticmethod
 def regular(p):
  p=Path(p);ND(p.is_file() and not p.is_symlink(),'reg');return p.stat()
 @staticmethod
 def digest(p):return sh(p)
 @staticmethod
 def jload(p):return json.loads(Path(p).read_text())
 def cg_paths(s,c):
  g=c['cgroup'];e=g['envelope_path'];return e,e+'/'+g['supervisor_leaf_name'],e+'/'+g['phase_parent_name']
 def cg_fs(s,n):return s.c/n.lstrip('/')
 @staticmethod
 def read_numbers(p):return{k:int(v)for k,v in(x.split()for x in Path(p).read_text().splitlines()if len(x.split())==2)}
class P11RetiredCarryFocusedTests(unittest.TestCase):
 def setUp(s):
  s.sv={n:getattr(H,n,None)for n in('P11_ACTUAL_ROOT','P11_EVIDENCE_ROOT','P11_OUTER_DIR','P11_RECEIPT_PINS','P11_EVIDENCE_FILE_BYTES','POST_P11_HISTORICAL_DEBITS','enforce_post_p10_historical_debits','verify_post_compile_no_script_reexec')}
  s.td=tempfile.TemporaryDirectory();s.addCleanup(s.td.cleanup);s.addCleanup(s.rs);s.r=Path(s.td.name);s.bd();s.mk(10,20,30)
 def rs(s):
  for k,v in s.sv.items():setattr(H,k,v)
 def mk(s,e,v,p):
  s.bs={'live_envelope_usec_at_start':e,'live_supervisor_usec_at_start':v,'live_phases_usec_at_start':p,'historical_debits':{'x':1}};H.enforce_post_p10_historical_debits=lambda c,g,at_start=True:dict(s.bs)
 def cfg(s,**k):
  c={'total_wall_seconds':5921,'supervisor_cpu_seconds':10547,'limits':{'F':{'wall_seconds':2021,'cpu_time_us':10000000000,'stderr_bytes':100000,'stdout_bytes':100000}},'cgroup':{'envelope_path':'E','supervisor_leaf_name':'S','phase_parent_name':'P'}};c.update(k);return c
 def w(s,p,b):p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b if isinstance(b,bytes)else b.encode());return p
 def bd(s):
  a=s.r/'a';e=s.r/'e';o=a/'o';s.P={'outer':o/'RESULT.json','stderr':o/'stderr.bin','clean':a/'ROOT-P11-CLEANUP-RESULT.json','td':a/'td'/'RESULT.json','man':e/'EVIDENCE_SHA256_MANIFEST.json'}
  f=b'TERMINAL_REFUSAL: A0 source digest changed: registry/index/index.crates.io-1949cf8c6b5b557f/.cache/ca/mi/camino\n'
  ev={'ONE_SHOT_STARTED':b'no retry\n','packet/F_precheck.stderr':f,'packet/F_precheck.stdout':b'','packet/TERMINAL_FAILURE.json':b'{"message":"phase exit nonzero F","native_outputs_accepted":false,"route":"cached-current-f","type":"Refusal"}','packet/USED_LAUNCH_ac016eccddc7f851f05f244b81ac1a86e9e2a0fcfc8bee86ab0195f7826611e8':b'consumed\n'}
  for k,v in ev.items():s.w(e/k,v)
  s.w(s.P['stderr'],b'TERMINAL_REFUSAL: phase exit nonzero F\n');s.w(s.P['outer'],json.dumps({'id':'P11-one-current-WW-cached-current-F','actual_exit':2,'elapsed_seconds':H.P11_OUTER_SECONDS,'controller_failure':None,'streams':[]}));s.w(s.P['td'],json.dumps({'id':'P11-teardown','actual_exit':0}))
  D=H.POST_P11_HISTORICAL_DEBITS;P=H.POST_P10_HISTORICAL_DEBITS
  cl={'status':'DONE','cleanupOwed':False,'all3MountsAbsent':True,'EandSupervisorAndPhasesAbsent':True,'evidence':str(e),'noCargoStarted':True,'noELFProducedOrExecuted':True,'counts':{'phase':'F_precheck','failure':f.decode(),'result':T(s.P['outer']),'rawUsageUsec':{'E':D['retired_p11_envelope_usec'],'S':D['retired_p11_supervisor_usec'],'P':D['retired_p11_phases_usec']},'previousHistory':{'E':P['envelope_usec'],'S':P['supervisor_usec'],'P':P['phases_usec']},'cumulativeUsec':{'E':D['envelope_usec'],'S':D['supervisor_usec'],'P':D['phases_usec']},'currentResidualUsec':1,'CargoStarted':False,'SDKbuild':False,'newELF':False,'nativeOutputsAccepted':False,'oneShotConsumed':True,'cleanupOwed':True},'teardown':T(s.P['td'])}
  s.w(s.P['clean'],json.dumps(cl))
  for n,v in H.P11_RAW_CPU_STAT.items():s.w(a/'actual-terminal-preservation-01'/(n+'-cpu.stat'),'usage_usec %d\nuser_usec %d\nsystem_usec %d\nnice_usec 0\nnr_periods %d\nnr_throttled 0\nthrottled_usec 0\nnr_bursts 0\nburst_usec 0\n'%v)
  cg=s.r/'cg'
  for n,u in(('E',10),('E/S',20),('E/P',30)):s.w(cg/n/'cpu.stat','usage_usec %d\n'%u)
  s.g=G(cg);H.P11_ACTUAL_ROOT=str(a);H.P11_EVIDENCE_ROOT=str(e);H.P11_OUTER_DIR=str(o);s.sy()
 def sy(s):
  e=Path(H.P11_EVIDENCE_ROOT);ks=('ONE_SHOT_STARTED','packet/F_precheck.stderr','packet/F_precheck.stdout','packet/TERMINAL_FAILURE.json','packet/USED_LAUNCH_ac016eccddc7f851f05f244b81ac1a86e9e2a0fcfc8bee86ab0195f7826611e8');fs={k:sh(e/k)for k in ks};s.w(s.P['man'],json.dumps({'files':fs}));c=json.loads(s.P['clean'].read_text());c['counts']['result']=T(s.P['outer']);c['teardown']=T(s.P['td']);c['evidence']=str(e);s.w(s.P['clean'],json.dumps(c));H.P11_EVIDENCE_FILE_BYTES={k:(e/k).stat().st_size for k in fs};H.P11_RECEIPT_PINS={k:(str(p),p.stat().st_size,sh(p))for k,p in(('outer result',s.P['outer']),('outer stderr',s.P['stderr']),('Root cleanup',s.P['clean']),('teardown result',s.P['td']),('evidence manifest',s.P['man']))}
 def call(s,c=None,a=True):return H.enforce_post_p11_historical_debits(c or s.cfg(),s.g,at_start=a)
 def pin(s,k,i,v):
  p=dict(H.P11_RECEIPT_PINS);x=list(p[k]);x[i]=v;p[k]=tuple(x);H.P11_RECEIPT_PINS=p
 def test_positive_carry_and_exact_accounting(s):
  r=s.call();d=r['post_p11_debits'];s.assertEqual((d['envelope_usec'],d['supervisor_usec'],d['phases_usec']),(1092254321,252227979,840026337));s.assertEqual((d['independent_envelope_minus_supervisor_minus_phases_usec'],d['retired_p11_independent_residual_usec']),(5,1));s.assertEqual((d['remaining_total_wall_seconds_max'],d['remaining_f_wall_seconds_max'],d['remaining_supervisor_cpu_seconds_max']),(5921,2021,10547));s.assertEqual((d['f_stderr_bytes'],d['p11_f_stderr_bytes'],d['p11_outer_controller_stderr_bytes_separate']),(59519,112,39));s.assertEqual((r['effective_envelope_usec_at_start'],r['effective_supervisor_usec_at_start'],r['effective_phases_usec_at_start']),(1092254331,252227999,840026367));s.assertEqual(r['historical_debits'],{'x':1})
 def test_receipt_pins_and_manifest_drift_refused(s):
  s.call();s.pin('outer result',2,'0'*64)
  with s.assertRaises(R):s.call()
  s.sy();s.pin('outer stderr',1,40)
  with s.assertRaises(R):s.call()
  s.sy();m=json.loads(s.P['man'].read_text());m['files']['x']='0'*64;s.w(s.P['man'],json.dumps(m));s.pin('evidence manifest',1,s.P['man'].stat().st_size);s.pin('evidence manifest',2,sh(s.P['man']))
  with s.assertRaises(R):s.call()
  s.sy();m=json.loads(s.P['man'].read_text());m['files']['ONE_SHOT_STARTED']='0'*64;s.w(s.P['man'],json.dumps(m));s.pin('evidence manifest',1,s.P['man'].stat().st_size);s.pin('evidence manifest',2,sh(s.P['man']))
  with s.assertRaises(R):s.call()
 def test_raw_cpu_node_bytes_refused(s):
  for n in 'ESP':
   p=Path(H.P11_ACTUAL_ROOT)/'actual-terminal-preservation-01'/(n+'-cpu.stat');o=p.read_bytes();p.write_bytes(o.replace(str(H.P11_RAW_CPU_STAT[n][0]).encode(),str(H.P11_RAW_CPU_STAT[n][0]+1).encode(),1))
   with s.assertRaises(R):s.call()
   p.write_bytes(o);s.call()
 def test_terminal_and_stream_content_refused(s):
  def mut(f):
   f();s.sy()
   with s.assertRaises(R):s.call()
  def ow(**k):
   o=json.loads(s.P['outer'].read_text());o.update(k);s.w(s.P['outer'],json.dumps(o))
  mut(lambda:ow(actual_exit=1));ow(actual_exit=2);s.sy();mut(lambda:ow(controller_failure='x'));ow(controller_failure=None);s.sy();mut(lambda:ow(elapsed_seconds=1.0));ow(elapsed_seconds=H.P11_OUTER_SECONDS);s.sy()
  p=Path(H.P11_EVIDENCE_ROOT)/'packet/TERMINAL_FAILURE.json';p.write_bytes(p.read_bytes().replace(b'Refusal',b'Xefusal'));mut(lambda:None)
 def test_cleanup_snapshot_vs_current_state(s):
  s.call();c=json.loads(s.P['clean'].read_text());s.assertTrue(c['counts']['cleanupOwed']);s.assertFalse(c['cleanupOwed'])
  for k,v,ok in(('status','BAD','DONE'),('cleanupOwed',True,False),('all3MountsAbsent',False,True)):
   o=json.loads(s.P['clean'].read_text());o[k]=v;s.w(s.P['clean'],json.dumps(o));s.sy()
   with s.assertRaises(R):s.call()
   o[k]=ok;s.w(s.P['clean'],json.dumps(o));s.sy()
  t=json.loads(s.P['td'].read_text());t['actual_exit']=1;s.w(s.P['td'],json.dumps(t));s.sy()
  with s.assertRaises(R):s.call()
 def test_ceilings_arithmetic_and_live_counter_boundaries(s):
  D=dict(H.POST_P11_HISTORICAL_DEBITS)
  for k,v in(('total_wall_seconds',5922),('supervisor_cpu_seconds',10548)):
   with s.assertRaises(R):s.call(s.cfg(**{k:v}))
  c=s.cfg();c['limits']['F']['wall_seconds']=2022
  with s.assertRaises(R):s.call(c)
  c=s.cfg();c['limits']['F']['cpu_time_us']=D['phases_usec']+30
  with s.assertRaises(R):s.call(c,True)
  s.call(c,False);s.mk(11,20,30)
  with s.assertRaises(R):s.call()
  s.mk(10,20,30)
  for k in('envelope_usec','f_stderr_bytes'):
   b=dict(D);b[k]+=1;H.POST_P11_HISTORICAL_DEBITS=b
   try:
    with s.assertRaises(R):s.call()
   finally:H.POST_P11_HISTORICAL_DEBITS=D
 def test_cached_current_f_integration_no_native(s):
  ns=('source_home','target','tmp','packet','empty_home','mask_empty','execution_root');ps={k:str(s.r/('x'+k))for k in ns}
  for v in ps.values():Path(v).mkdir()
  c=s.cfg(mode='tier1-and-compile',paths=ps);c['limits']['F']['stderr_bytes']=59519;g=s.g;g.BASE='a'*40;g.LOCK='b'*64
  for n in('consent','supervisor_envelope','bound_identities','owned_paths','quotas','receipt','write_new'):setattr(g,n,mock.Mock())
  class Grp:
   def __init__(t,cfg,ph):t.prior_cpu=0
  g.Group=Grp;oi=Grp.__init__;seen={}
  def ln(cfg,ph,sha):
   q=Grp(cfg,ph);seen.update(prior=q.prior_cpu,phase=ph);return{'log_bytes':{'stderr':0,'stdout':0}}
  g.launch=mock.Mock(side_effect=ln)
  def jl(p):
   n=Path(p).name
   if n=='SOURCE_HOME_RECONSTITUTION.json':return{'ok':1}
   if n=='CACHED_TARGET_SEED.json':return{'partition_counts':{'a':1}}
   return json.loads(Path(p).read_text())
  g.jload=jl;H.verify_post_compile_no_script_reexec=lambda cfg,gr,sd:{'ok':1};oe=H.enforce_post_p11_historical_debits;fl=[]
  def spy(cfg,gr,at_start=True):fl.append(at_start);return oe(cfg,gr,at_start=at_start)
  H.enforce_post_p11_historical_debits=spy
  try:
   with mock.patch('signal.signal'),mock.patch('signal.setitimer')as it,mock.patch('resource.setrlimit')as rl:out=H.cached_current_f(c,'cfgsha',g)
   s.assertEqual(fl,[True,False]);s.assertEqual(seen,{'prior':840026367,'phase':'F'});s.assertEqual(out['route'],'cached-current-f');g.consent.assert_called_once();g.launch.assert_called_once();s.assertEqual(rl.call_args,mock.call(resource.RLIMIT_CPU,(10547,10547)));s.assertEqual(it.call_args_list[0],mock.call(signal.ITIMER_REAL,5921));s.assertEqual(it.call_args_list[-1],mock.call(signal.ITIMER_REAL,0));s.assertIs(Grp.__init__,oi)
   g.launch=mock.Mock(return_value={'log_bytes':{'stderr':1,'stdout':0}});fl.clear()
   with mock.patch('signal.signal'),mock.patch('signal.setitimer')as it,mock.patch('resource.setrlimit'):
    with s.assertRaises(R):H.cached_current_f(c,'cfgsha',g)
   s.assertEqual(fl,[True]);s.assertTrue(any(x.args[1]=='TERMINAL_FAILURE.json'for x in g.receipt.call_args_list));s.assertIs(Grp.__init__,oi);s.assertIn(mock.call(signal.ITIMER_REAL,0),it.call_args_list)
  finally:H.enforce_post_p11_historical_debits=oe
