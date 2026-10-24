"""Root-only P10 data adaptation of the existing issuer; no native action.
Current actual review/test/setup facts remain UNISSUED until Root fills them.
No approval, result or admission is inferred from this materialization.
"""
from pathlib import Path
import json,hashlib,copy,datetime,ast
K=Path(__file__).resolve().parent;D=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-integration-metadata-plan-20261019');R=D.parent
T=R/'native06-p9-exact-camino-copy-owner-request-20261019/revision02-operation-review/metadata-recipe'
P=D/'future-actual-03'
A=R/'native-p10-owner-source-candidate-20261019/revision03-test-only'
def raw(x):return(json.dumps(x,sort_keys=True,indent=2)+'\n').encode()
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def load(p):return json.loads(Path(p).read_text())
def desc(p):p=Path(p);return{'path':str(p),'bytes':p.stat().st_size,'sha256':sha(p)}
def put(p,x):
 p=Path(p);p.parent.mkdir(parents=True,exist_ok=True,mode=0o700)
 with p.open('xb')as f:f.write(raw(x))
 return sha(p)
# Source/receipt literal equality only; no candidate import or accounting function call.
assert sha(A/'afterimages/bounded_sdk_compile.py')=='30077e57f829a111b08228e30f05fb3360e9b7534d7818dedf48d5ee367a0edc'
assert sha(A/'afterimages/cached_current_build.py')=='5a8d8f0ca1c1ab84de821ad403175c87beab651f717e90114f4b035fe55b3cf2'
values={}
for node in ast.parse((A/'afterimages/cached_current_build.py').read_text()).body:
 if isinstance(node,ast.Assign):
  for target in node.targets:
   if isinstance(target,ast.Name)and target.id in ['P7_MARKER_PATH','P7_MARKER_BYTES','P7_MARKER_SHA256','P7_CLEANUP_STEPS']:values[target.id]=ast.literal_eval(node.value)
P8=R/'native06-current-cached-p8-integration-metadata-20261018'
rawP8=load(P8/'post-terminal-actual-01/RAW-CGROUP-AND-MARKER.json');cleanupP8=load(P8/'actual-p8-preserve-and-cleanup-01/RESULT.json')
assert rawP8['raw']['marker']=={'path':values['P7_MARKER_PATH'],'bytes':values['P7_MARKER_BYTES'],'sha256':values['P7_MARKER_SHA256']}
assert [x['step']for x in cleanupP8['steps']]==list(values['P7_CLEANUP_STEPS'])
# Specific actual transport/QA data checks. No candidate import or subject action.
CURRENT_SOURCE_PINS = [{'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision03-test-only/afterimages/bounded_sdk_compile.py', 'bytes': 96351, 'sha256': '30077e57f829a111b08228e30f05fb3360e9b7534d7818dedf48d5ee367a0edc'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision03-test-only/afterimages/cached_current_build.py', 'bytes': 120677, 'sha256': '5a8d8f0ca1c1ab84de821ad403175c87beab651f717e90114f4b035fe55b3cf2'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision03-test-only/afterimages/test_cached_current_build.py', 'bytes': 99140, 'sha256': '46502283e1f0db1e529cfd6f238bd8af7f751086730f56028cd8f9eabb0d4713'}]
PRIOR_SOURCE_PINS = [{'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision02/afterimages/bounded_sdk_compile.py', 'bytes': 96351, 'sha256': '30077e57f829a111b08228e30f05fb3360e9b7534d7818dedf48d5ee367a0edc'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision02/afterimages/cached_current_build.py', 'bytes': 120677, 'sha256': '5a8d8f0ca1c1ab84de821ad403175c87beab651f717e90114f4b035fe55b3cf2'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision02/afterimages/test_cached_current_build.py', 'bytes': 99058, 'sha256': '2d1dd7719b78fd6be11b2075087fd2bce1132605909000428c4b33c8f36ac449'}]
STATIC_INPUT_PINS = [{'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-exact-camino-copy-owner-request-20261019/revision02-operation-review/metadata-recipe/config.PROSPECTIVE.json', 'bytes': 680322, 'sha256': '93b608bee1e08ef5f22a30091b2f73bef2b01642ee4dea37449d4ee3780b19ec'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-exact-camino-copy-owner-request-20261019/revision02-operation-review/metadata-recipe/config.CAPTURE-BASIS.json', 'bytes': 680225, 'sha256': 'd59c8f609a635799440bc47652c6785d923025fb8ddaf4f08b69c29f52a29244'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-exact-camino-copy-owner-request-20261019/revision02-operation-review/metadata-recipe/ALLOWED-EXACT-MEMBERSHIP-ADDITIONS.json', 'bytes': 3036, 'sha256': 'ac82bc74b6437b562e0bfa9f0a784ac8bf1a5985526af9e3f0e659f2af4dc73a'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-exact-camino-copy-owner-request-20261019/revision02-operation-review/metadata-recipe/CACHED-COMPILE-ARGV.proposed.json', 'bytes': 2038, 'sha256': '16added564c8cfa845a653bf4ad5748e0deacf46f97fbbdf403f3aaed0c95808'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-integration-metadata-plan-20261019/request.cached-current-p10-01.proposed.json', 'bytes': 2277, 'sha256': '3510ec35144079c8aba72584039b8c7de923705ff75e8c27c8f8776b95732909'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-integration-metadata-plan-20261019/COMPILE-SUBJECT.proposed.json', 'bytes': 84423, 'sha256': '590f58c4f6b058a74f59803934b282baaa0b50819d0c411c60232f7dfa388df0'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-pr276-sourcecut-preparation-20261019/security-owner-source-request/revision02-W3-bound/CHECKOUT-SOURCE-MANIFEST.json', 'bytes': 7444184, 'sha256': '1632a29ac2f678640fe9e1821ddace5845094dc735fbec21f2f1fd3e4d186c27'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-current-cached-p7-integration-metadata-20261018/LIVE-HASH-QUALIFICATIONS-AND-PREFLIGHT-INPUTS.json', 'bytes': 2912, 'sha256': 'c73914204b00b7b878877aae17ca39ab55b99e40f8cfbc103496819b104d4510'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-current-cached-p8-integration-metadata-20261018/post-terminal-actual-01/RAW-CGROUP-AND-MARKER.json', 'bytes': 5899, 'sha256': 'c1195a3551e9ba4119a37cb2a9549462f7509d3e3bff01e0cfce0db382350f44'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-current-cached-p8-integration-metadata-20261018/actual-p8-preserve-and-cleanup-01/RESULT.json', 'bytes': 9014, 'sha256': 'cb7febe2e98c2a600a273cc2817b5a02e99b2ff497594c184db9354742ca952b'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision03-test-only/ROOT-MATERIALIZATION.json', 'bytes': 1907, 'sha256': '815a084ab905d7e49b0d1adbcf5589ac04091737751b5653808fdb13ea22a31a'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/p10-r03-single-method-inline-review-20261019/DELTA-AND-TEN-PASS-PROOF.json', 'bytes': 7519, 'sha256': '6ce4cde1d8b15f2a23c41e694aac38c22789b45d741aba9f5e6f90b6344a6cc4'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-exact-camino-copy-owner-request-20261019/revision02-operation-review/metadata-recipe/security.EXISTING-SCHEMA-TEMPLATE.json', 'bytes': 832, 'sha256': '672c74da5fe598ccf30b985c41e658e297a54a3508b30d073dda423b99091194'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-exact-camino-copy-owner-request-20261019/revision02-operation-review/metadata-recipe/nonsecurity.EXISTING-SCHEMA-TEMPLATE.json', 'bytes': 832, 'sha256': 'abbe6d45146c65f47d8ad0391ef470d66e3bab55f98e7aef4cfb7eab92f4bea8'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-exact-camino-copy-owner-request-20261019/revision02-operation-review/metadata-recipe/root.EXISTING-SCHEMA-TEMPLATE.json', 'bytes': 658, 'sha256': '1e0d54a60f8c7aff6e2f25053091c76a35c055e47a9685b9be45a6bb66244dbe'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-exact-camino-copy-owner-request-20261019/revision02-operation-review/metadata-recipe/tier1.EXISTING-SCHEMA-TEMPLATE.json', 'bytes': 776, 'sha256': '23d305bb47557d326634f6e0bb41e446bbe37ae6af71f08a2b4b2166d8f64362'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-exact-camino-copy-owner-request-20261019/revision02-operation-review/metadata-recipe/tier2.EXISTING-SCHEMA-TEMPLATE.json', 'bytes': 870, 'sha256': '93115a0a5b470a012ba44d703ff3c0a9eaa15a2c77859ea9b6065d52624e695a'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-existing-issuer-materialization-20261019/revision02-opus-B1-B4/FINAL-3306-PIN-PREFLIGHT.metadata-proposed.py', 'bytes': 4176, 'sha256': '1435d2ac56b7ad34ef45643349393a9568133fdfdabd522956a099a3f7461f3b'}]

def verify_pin(p):
 assert set(p)=={'path','bytes','sha256'} and type(p['bytes'])is int and p['bytes']>=0
 assert type(p['path'])is str and type(p['sha256'])is str and len(p['sha256'])==64
 assert sha(p['path'])==p['sha256'] and Path(p['path']).stat().st_size==p['bytes'],p['path']

def observed_review(r,role,decision):
 for key in ['terminal','transport','command','report','identity_record']:verify_pin(r[key])
 control=load(r['terminal']['path']);command=load(r['command']['path'])
 assert control['exit_code']==0 and control['subject_tests_native_run']==0
 initial=[];finished=[];events=[]
 with Path(r['transport']['path']).open() as stream:
  for line in stream:
   e=json.loads(line);events.append(e.get('event'))
   if e.get('event')=='init':initial.append(e)
   elif e.get('event')=='result':finished.append(e['result'])
 assert len(initial)==len(finished)==1 and events[0]=='init' and events[-1]=='result' and 'error'not in events
 init=initial[0];result=finished[0]
 assert result['status']=='SUCCESS' and result['conversation_id']==init['conversation_id']
 assert result['response'].encode('utf-8')==Path(r['report']['path']).read_bytes()
 av=command['argv'];assert av.count('--model')==av.count('--effort')==1
 requested=av[av.index('--model')+1];effort=av[av.index('--effort')+1]
 assert command['model']==requested and command['effort']==effort
 assert command['fresh_context']is True and command['conversation_argument']is None
 assert command['cwd']==init['init']['cwd'] and command['role']==role and command['positive_source_label']==decision
 assert requested==init['init']['model']=='gemini-4-argon-high' and effort=='high'
 # Primary/fallback classification is the unchanged Root/guard route; model/context are observed above.
 observed_tuple=('claude-opus-5-5',requested,init['init']['model'],effort,'standing_opus_quota_outage_fallback')
 assert tuple(r[k]for k in ['primary_model','requested_model','returned_identity','effort','fallback_status'])==observed_tuple
 assert r['actual_context_id']==init['conversation_id']
 identity=load(r['identity_record']['path'])
 assert identity['raw_transport']==r['transport'] and identity['command']==r['command'] and identity['controller']==r['terminal'] and identity['report']==r['report']
 assert identity['init']=={'conversation_id':init['conversation_id'],'model':init['init']['model'],'cwd':init['init']['cwd']}
 assert identity['result']=={'conversation_id':result['conversation_id'],'status':'SUCCESS','response_bytes':len(result['response'].encode('utf-8')),'response_sha256':sha(r['report']['path'])}
 assert r['Root_actual_source_adoption_decision']==decision
 assert r['source_manifest_sha256']=='1632a29ac2f678640fe9e1821ddace5845094dc735fbec21f2f1fd3e4d186c27'
 assert r['runner_sha256']=='30077e57f829a111b08228e30f05fb3360e9b7534d7818dedf48d5ee367a0edc'
 assert r['subject_manifest_sha256']=='590f58c4f6b058a74f59803934b282baaa0b50819d0c411c60232f7dfa388df0'
 assert r['reviewed_candidate_source_pins']==CURRENT_SOURCE_PINS
 return init['conversation_id']

def bound_qa(bundle,expected_sources,count):
 for key in ['result','stdout','stderr','postcheck','command']:verify_pin(bundle[key])
 result=load(bundle['result']['path']);post=load(bundle['postcheck']['path']);command=load(bundle['command']['path'])
 assert post['result']==result and result['controller_failure']is None
 assert command['source_pins']==expected_sources and command['expected_test_count']==count and len(command['selectors'])==count
 assert Path(command['cwd']).is_dir()
 assert sorted(p.name for p in Path(command['cwd']).iterdir())==sorted(Path(p['path']).name for p in expected_sources)
 assert command['argv'][-1].count(repr(command['cwd']))==1
 for source in expected_sources:
  verify_pin(source);stage=Path(command['cwd'])/Path(source['path']).name
  assert sha(stage)==source['sha256'] and stage.stat().st_size==source['bytes']
 for pin in result['streams']:
  assert pin['full_stream_retained']is True and pin['bytes_seen']==pin['bytes_retained']==pin['bytes']
  selected=bundle['stdout'if pin['path']==bundle['stdout']['path']else'stderr']
  assert {k:pin[k]for k in ['path','bytes','sha256']}==selected
 return result,post,command

# Data-only consumed pin-schema projection. Historical review identity stays original.
ORIGINAL_MANIFEST_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-pr276-sourcecut-preparation-20261019/security-owner-source-request/revision02-W3-bound/CHECKOUT-SOURCE-MANIFEST.json', 'bytes': 7444184, 'sha256': '1632a29ac2f678640fe9e1821ddace5845094dc735fbec21f2f1fd3e4d186c27'}
PROJECTED_MANIFEST_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-existing-issuer-materialization-20261019/revision04-pin-schema-projection/CHECKOUT-SOURCE-MANIFEST.pin-schema.json', 'bytes': 7444090, 'sha256': '1e042df3b0115993900bdc81dff1891863533e023d056d4d83e05e6092aeae59'}
MANIFEST_EQUIVALENCE_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-existing-issuer-materialization-20261019/revision04-pin-schema-projection/MANIFEST-PROJECTION-EQUIVALENCE.json', 'bytes': 3032, 'sha256': 'd638addab2b7541834504c7521eb81c99a682647dae7167dfa05485c08dde852'}
PREPHASE_RAW_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-integration-metadata-plan-20261019/future-reversible-maintenance-02/RAW-POST-TERMINAL-CGROUPS.json', 'bytes': 2783, 'sha256': '11c35eb75fcebddb2db65d6c4c810d14b3ff68a1720adbc6e154dcc852e67e7a'}
for metadata_pin in [ORIGINAL_MANIFEST_PIN,PROJECTED_MANIFEST_PIN,MANIFEST_EQUIVALENCE_PIN,PREPHASE_RAW_PIN]:verify_pin(metadata_pin)
manifest_before=load(ORIGINAL_MANIFEST_PIN['path']);manifest_after=load(PROJECTED_MANIFEST_PIN['path'])
assert len(manifest_before['head_files'])==len(manifest_after['head_files'])==5
assert all(set(p)=={'path','sha256','bytes'} for p in manifest_before['head_files'])
assert all(set(p)=={'path','sha256'} for p in manifest_after['head_files'])
assert manifest_after==dict(manifest_before,head_files=[{k:p[k]for k in ['path','sha256']}for p in manifest_before['head_files']])
manifest_adoption_path=K/'ROOT-MANIFEST-SCHEMA-ADOPTION.json'
manifest_adoption=load(manifest_adoption_path)
assert manifest_adoption=={'decision':'ADOPTED_METADATA_SHAPE_EQUIVALENCE','compatibility_proof':MANIFEST_EQUIVALENCE_PIN}
prephase_raw=load(PREPHASE_RAW_PIN['path'])['raw'];cgbase='/sys/fs/cgroup/moriarty-sherlock-ledger9-20261018-06'
usage=lambda entry:dict((line.split()[0],int(line.split()[1]))for line in entry['cpu.stat'].splitlines())['usage_usec']
assert [usage(prephase_raw[cgbase+s])for s in ['', '/supervisor','/phases']]==[4358085,4358085,0]
assert all(not x['cgroup.procs'].strip()for x in prephase_raw.values())
# Live E/S4.358085s is already retained in the same kernel groups; never add it to frozen P9 history.

actuals=load(K/'ROOT-ACTUAL-INPUTS.json')
assert actuals['candidate_source_pins']==CURRENT_SOURCE_PINS
for source in CURRENT_SOURCE_PINS:verify_pin(source)
prior=actuals['qa']['prior_eleven'];changed=actuals['qa']['corrected_one']
pr,pp,pc=bound_qa(prior,PRIOR_SOURCE_PINS,11);cr,cp,cc=bound_qa(changed,CURRENT_SOURCE_PINS,1)
assert pp['all_source_and_stage_pins_unchanged']is True and pp['no_extra_stage_entries']is True and pp['native_execution']is False
assert cp['exact3files_source_and_stage_unchanged']is True and cp['extra_stage_entries']is False
assert pr['actual_exit']==1 and cr['actual_exit']==0
prior_text=Path(prior['stderr']['path']).read_text();changed_text=Path(changed['stderr']['path']).read_text()
assert sum(line.endswith(' ... ok')for line in prior_text.splitlines())==10 and prior_text.count('PermissionError:')==7 and 'Ran 11 tests'in prior_text
assert cc['selectors']==['test_cached_current_build.CachedCurrentBuildSyntheticTest.test_p9_receipts_once_only_pinned_and_tamper_refused']
assert 'Ran 1 test'in changed_text and '\nOK\n'in changed_text
proof=load('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/p10-r03-single-method-inline-review-20261019/DELTA-AND-TEN-PASS-PROOF.json');assert proof['guard_helper_byteequal']is True and proof['outside_failed_method_bytes_equal']is True
assert proof['current_source_pins']==CURRENT_SOURCE_PINS and proof['test_before']==PRIOR_SOURCE_PINS[2]
insert=proof['only_literal_insertion'].encode('utf-8');old_test=Path(PRIOR_SOURCE_PINS[2]['path']).read_bytes();new_test=Path(CURRENT_SOURCE_PINS[2]['path']).read_bytes()
assert len(insert)==82 and new_test.count(insert)==1 and new_test.replace(insert,b'',1)==old_test
assert len(proof['ten_prior_ok_method_proofs'])==10 and all(x['byteequal_prior_actual_ok']is True for x in proof['ten_prior_ok_method_proofs'])
reviewinputs=actuals['reviews']
security_cid=observed_review(reviewinputs['security'],'designated_security_owner','APPROVED_SOURCE_WITH_LIMITS')
normal_cid=observed_review(reviewinputs['nonsecurity'],'nonsecurity_source_reviewer','APPROVED_NORMAL_SOURCE')
assert security_cid!=normal_cid
# Coverage is mandatory, nonempty, conflict-free, and checked BEFORE every output.
rows=load(K/'EXACT-CURRENT-OPERAND-PINS.json');assert type(rows)is list and rows
indexed={p['path']:p for p in rows};assert len(indexed)==len(rows)
required=list(STATIC_INPUT_PINS)+CURRENT_SOURCE_PINS+PRIOR_SOURCE_PINS
required += [desc(K/'ROOT-ACTUAL-INPUTS.json'),desc(Path(__file__).resolve()),ORIGINAL_MANIFEST_PIN,PROJECTED_MANIFEST_PIN,MANIFEST_EQUIVALENCE_PIN,PREPHASE_RAW_PIN,desc(manifest_adoption_path)]
for r in reviewinputs.values():required += [r[k]for k in ['terminal','transport','command','report','identity_record']]
for bundle in [prior,changed]:required += [bundle[k]for k in ['result','stdout','stderr','postcheck','command']]
reqpath=D/'request.cached-current-p10-01.proposed.json';req=load(reqpath)
required += [actuals['fresh_inputs']['QOBS'],actuals['fresh_inputs']['old_home_stats']]
live=load(R/'native06-current-cached-p7-integration-metadata-20261018/LIVE-HASH-QUALIFICATIONS-AND-PREFLIGHT-INPUTS.json')
for entry in live['selected8live_metadata']:
 assert sha(entry['path'])==entry['expected_sha256'];required.append(desc(entry['path']))
source_manifest=load(R/'native-p10-pr276-sourcecut-preparation-20261019/security-owner-source-request/revision02-W3-bound/CHECKOUT-SOURCE-MANIFEST.json')
for entry in source_manifest['head_files']:
 assert sha(entry['path'])==entry['sha256'];required.append(desc(entry['path']))
assert actuals['fresh_inputs']['QOBS']['path']==req['paths']['observation_output']
assert actuals['fresh_inputs']['old_home_stats']['path']==str(P/'old_home_stat_manifest.current.json')
basis=load(T/'config.CAPTURE-BASIS.json');oldstat_pin=basis['old_home_stat_manifest']
assert sha(oldstat_pin['path'])==oldstat_pin['sha256'];required.append(desc(oldstat_pin['path']))
for p in required:
 assert p['path']in indexed and indexed[p['path']]==p,p['path'];verify_pin(p)
for p in rows:verify_pin(p)
required_operand_paths=sorted({p['path']for p in required})
reqpath=D/'request.cached-current-p10-01.proposed.json';req=load(reqpath);obs=load(req['paths']['observation_output'])
assert obs['request_sha256']==sha(reqpath) and obs['provisioner_sha256']=='7a8e55b908a82f138037d6cc4cc87dca4105cad1ee5838b6110555a40e7f6476'
c=load(T/'config.PROSPECTIVE.json')
assert (c['total_wall_seconds'],c['limits']['F']['wall_seconds'],c['supervisor_cpu_seconds'])==(6159,2254,10592)
# Exact current literal data rebindings, preserving every other source/resource field.
c['paths']['worktree']='/home/charl/Moriarty/.worktrees/sherlock-native-handwritten-20261019'
c['paths']['config']=str(P/'config.final.json')
c['bindings']['runner']={'path':str(A/'afterimages/bounded_sdk_compile.py'),'sha256':'30077e57f829a111b08228e30f05fb3360e9b7534d7818dedf48d5ee367a0edc'}
c['source_manifest']={k:PROJECTED_MANIFEST_PIN[k]for k in ['path','sha256']}
c['bindings']['subject']={'path':str(D/'COMPILE-SUBJECT.proposed.json'),'sha256':'590f58c4f6b058a74f59803934b282baaa0b50819d0c411c60232f7dfa388df0'}
c['total_wall_seconds']=6033;c['limits']['F']['wall_seconds']=2133;c['supervisor_cpu_seconds']=10576
assert set(obs['quota_mounts'])==set(c['quota_mounts'])
for name,q in obs['quota_mounts'].items():
 assert set(q)==set(c['quota_mounts'][name])
 for f in ['mountpoint','max_bytes','max_inodes']:assert q[f]==c['quota_mounts'][name][f]
assert all(obs['cgroup_verified'][k]==v for k,v in c['cgroup'].items())
assert not any(token in raw(obs)for token in [b'<ROOT_',b'UNISSUED',b'PENDING_',b'UNOBSERVED',b'UNBOUND'])
for mount in obs['quota_mounts'].values():
 assert type(mount['mount_id'])is int and mount['mount_id']>0
 assert type(mount['device'])is str and len(mount['device'].split(':'))==2 and all(x.isdigit()for x in mount['device'].split(':'))
oldstat=load(oldstat_pin['path']);statpath=P/'old_home_stat_manifest.current.json';fresh=load(statpath)
assert set(fresh)=={'paths','directory_memberships'} and len(fresh['paths'])==3306 and len(fresh['directory_memberships'])==1063
assert set(fresh['paths'])==set(oldstat['paths'])
assert set(fresh['directory_memberships'])==set(oldstat['directory_memberships'])
# Conditional Root metadata draft: ONLY the exact already-diagnosed eight additions;
# reject every removal, extra parent, extra name, or other membership difference.
diagnosis=load(T/'ALLOWED-EXACT-MEMBERSHIP-ADDITIONS.json')
assert diagnosis['pathsAdded']==[] and diagnosis['pathsMissing']==[]
assert len(diagnosis['memberDifferences'])==8
membershipDiffs=[]
for p,before in oldstat['directory_memberships'].items():
 after=fresh['directory_memberships'][p]
 assert isinstance(after,list) and after==sorted(set(after)),p
 added=sorted(set(after)-set(before));missing=sorted(set(before)-set(after))
 assert not missing,{'path':p,'missing':missing}
 if added:membershipDiffs.append({'path':p,'added':added,'missing':missing})
assert sorted(membershipDiffs,key=lambda x:x['path'])==sorted(diagnosis['memberDifferences'],key=lambda x:x['path'])
assert all(not x['missing'] for x in diagnosis['memberDifferences'])
changes=[{'path':p,'before':v,'after':fresh['paths'][p]}for p,v in oldstat['paths'].items()if fresh['paths'][p]!=v]
wrapper_basis=load(T/'CACHED-COMPILE-ARGV.proposed.json');wrapper_script=wrapper_basis['argv'][-1]
assert wrapper_basis['argv'][:6]==['/usr/bin/sudo','-n','--','/bin/sh','-eu','-c'] and len(wrapper_basis['argv'])==7
assert wrapper_script.count('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p8-sol-accounting-candidate-20261018/afterimages/bounded_sdk_compile.py')==1
assert wrapper_script.count('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-integration-prerequisites-20261019/actual-p9-recovered-index-config-and-cached-compile-02/config.final.json')==1
cg='/sys/fs/cgroup'+c['cgroup']['envelope_path']+'/'+c['cgroup']['supervisor_leaf_name']+'/cgroup.procs'
assert wrapper_script.splitlines()[0]=='echo $$ > '+cg and wrapper_script.count(cg)==1
wrapper_prefix,wrapper_oldhash=wrapper_script.rsplit(' ',1)
assert wrapper_prefix.endswith('--config-sha256') and wrapper_oldhash=='<ROOT_ACTUAL_FINAL_SUCCESSOR_CONFIG_SHA256>'
stat_cut={'old':load(T/'config.CAPTURE-BASIS.json')['old_home_stat_manifest'],'new':desc(statpath),'statDiffs':changes,'all3306pathkeys_same':True,'all1063memberships_same':False,'membershipDiffs':membershipDiffs,'exact_diagnosed_additions_only':True,'membership_diagnosis':desc(T/'ALLOWED-EXACT-MEMBERSHIP-ADDITIONS.json'),'source_content_read_by_capture':0}
c['old_home_stat_manifest']={'path':str(statpath),'sha256':sha(statpath)};c['quota_mounts']=obs['quota_mounts'];c['source_authoring_only']=False
assert(c['total_wall_seconds'],c['limits']['F']['wall_seconds'],c['supervisor_cpu_seconds'])==(6033,2133,10576)
assert len(c['paths'])==9 and 'evidence_dir'not in c['paths']
c['bindings']['config_inputs']={'path':str(P/'config-inputs.json'),'sha256':'PENDING_PROJECTION'}
for name in ['security','nonsecurity','root','tier1','tier2']:c['review_receipts'][name]={'path':str(P/'receipts'/f'{name}.json'),'sha256':'PENDING_RECEIPT'}
projection=copy.deepcopy(c);projection.pop('review_receipts');projection['bindings'].pop('config_inputs')
assert not any(token in raw(projection)for token in [b'<ROOT_',b'UNISSUED',b'PENDING_',b'UNOBSERVED',b'UNBOUND'])
put(P/'ROOT-CURRENT-STAT-CUT-DIFF.json',stat_cut)
projsha=put(P/'config-inputs.json',projection);c['bindings']['config_inputs']['sha256']=projsha
runner=c['bindings']['runner']['sha256'];subject=c['bindings']['subject']['sha256'];source=c['source_manifest']['sha256'];reviewfacts={}
for name,required_decision in [('security','APPROVED_SOURCE_WITH_LIMITS'),('nonsecurity','APPROVED_NORMAL_SOURCE')]:
 r=reviewinputs[name];decision=r['Root_actual_source_adoption_decision']
 assert decision==required_decision # Actual Root disposition, never inferred from process exit.
 assert r['source_manifest_sha256']==ORIGINAL_MANIFEST_PIN['sha256'] and r['runner_sha256']==runner and r['subject_manifest_sha256']==subject
 terminalpath=r['terminal']['path'];reportpath=r['report']['path']
 rec=load(T/(name+'.EXISTING-SCHEMA-TEMPLATE.json'))
 rec.update(decision=decision,runner_sha256=runner,subject_manifest_sha256=subject,source_manifest_sha256=ORIGINAL_MANIFEST_PIN['sha256'],report_sha256=sha(reportpath),terminal_sha256=sha(terminalpath),primary_model=r['primary_model'],requested_model=r['requested_model'],returned_identity=r['returned_identity'],effort=r['effort'],fallback_status=r['fallback_status'])
 c['review_receipts'][name]['sha256']=put(P/'receipts'/f'{name}.json',rec)
 reviewfacts[name]={'report':desc(reportpath),'actual_terminal':desc(terminalpath),'actual_identity_record':r['identity_record'],'actual_context_id':r['actual_context_id'],'Root_actual_source_adoption_decision':decision,'not_inferred_from_terminal_exit':True,'effort_scope':'Requested HIGH, not independent weight/effort attestation.'}
root=load(T/'root.EXISTING-SCHEMA-TEMPLATE.json');root.update(decision='ADMITTED_CONFIGURATION',runner_sha256=runner,subject_manifest_sha256=subject,config_inputs_sha256=projsha,security_receipt_sha256=c['review_receipts']['security']['sha256'],nonsecurity_receipt_sha256=c['review_receipts']['nonsecurity']['sha256'])
c['review_receipts']['root']['sha256']=put(P/'receipts/root.json',root)
for name in ['tier1','tier2']:
 rec=load(T/(name+'.EXISTING-SCHEMA-TEMPLATE.json'))
 rec.update(config_inputs_sha256=projsha,root_receipt_sha256=c['review_receipts']['root']['sha256'],amendments_sha256=c['bindings']['amendments']['sha256'])
 assert rec['kind']=='root_standing_beta_'+name+'_grant'and rec['authorization_mode']=='root_standing_beta_override'
 if name=='tier2':rec['tier1_receipt_sha256']=c['review_receipts']['tier1']['sha256']
 c['review_receipts'][name]['sha256']=put(P/'receipts'/f'{name}.json',rec)
assert sha(c['bindings']['runner']['path'])==runner
serialized=raw(c)
assert not any(token in serialized for token in [b'<ROOT_',b'UNISSUED',b'PENDING_',b'UNOBSERVED',b'UNBOUND'])
for mount in c['quota_mounts'].values():
 assert type(mount['mount_id'])is int and mount['mount_id']>0
 assert type(mount['device'])is str and len(mount['device'].split(':'))==2 and all(x.isdigit()for x in mount['device'].split(':'))
finalsha=put(P/'config.final.json',c);check=copy.deepcopy(c);check.pop('review_receipts');check['bindings'].pop('config_inputs');assert raw(check)==(P/'config-inputs.json').read_bytes()
put(P/'ROOT-ACTUAL-ADMISSION.json',{'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'Root_selected_actual_security_and_normal_source_verdicts':True,'manifest_schema_compatibility':{'adoption':desc(manifest_adoption_path),'proof':MANIFEST_EQUIVALENCE_PIN,'original_review_manifest':ORIGINAL_MANIFEST_PIN,'consumed_manifest':PROJECTED_MANIFEST_PIN},'prephase_guard_attempt':{'actual_raw_cgroups':PREPHASE_RAW_PIN,'live_E_usec':4358085,'live_S_usec':4358085,'live_P_usec':0,'whole_wall_debit_ceil':5,'current_total_wall_seconds':6033,'F_wall_seconds':2133,'config_supervisor_ceiling_seconds':10576,'observed_live_subtracted_floor_seconds':10571,'historical_P9_debits_unchanged':True,'live_CPU_not_added_again':True},'actualQOBS':desc(req['paths']['observation_output']),'request':desc(reqpath),'current_stat_capture':desc(statpath),'config':desc(P/'config.final.json'),'config_inputs':desc(P/'config-inputs.json'),'receipts':c['review_receipts'],'reviewfacts':reviewfacts,'QA_retained_ten_plus_corrected_one':actuals['qa'],'exact_current_operand_list':desc(K/'EXACT-CURRENT-OPERAND-PINS.json'),'required_operand_paths':required_operand_paths,'current_candidate_source_pins':[{'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision03-test-only/afterimages/bounded_sdk_compile.py', 'bytes': 96351, 'sha256': '30077e57f829a111b08228e30f05fb3360e9b7534d7818dedf48d5ee367a0edc'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision03-test-only/afterimages/cached_current_build.py', 'bytes': 120677, 'sha256': '5a8d8f0ca1c1ab84de821ad403175c87beab651f717e90114f4b035fe55b3cf2'}, {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision03-test-only/afterimages/test_cached_current_build.py', 'bytes': 99140, 'sha256': '46502283e1f0db1e529cfd6f238bd8af7f751086730f56028cd8f9eabb0d4713'}],'history':{'E_usec':962803819,'S_usec':223293846,'P_usec':739509969,'p9_outer_wall_seconds_ceil':121,'whole_wall_seconds':6038,'F_wall_seconds':2133,'supervisor_cpu_seconds':10576,'cumulative_F_stdout_bytes':0,'cumulative_F_stderr_bytes':43081},'cleanupOwed':True,'scope':'ONE changed W3 cached-current-f compile from actual P9 quarantine; no CargoBCE replay/ELF/test/proof/key/stagenet; outputs UNACCEPTED pending result review. Existing restoration/preservation/cleanup duties retained.','P8':'permanently FAILED/no-script-reexecVIOLATED/outputsUNACCEPTED','no_guard_source_change_by_metadata_issuance':True})
argv=copy.deepcopy(wrapper_basis)
assert argv['argv'][:6]==['/usr/bin/sudo','-n','--','/bin/sh','-eu','-c'] and len(argv['argv'])==7
script=argv['argv'][-1]
old_guard='/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p8-sol-accounting-candidate-20261018/afterimages/bounded_sdk_compile.py'
old_config='/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p9-integration-prerequisites-20261019/actual-p9-recovered-index-config-and-cached-compile-02/config.final.json'
assert script.count(old_guard)==script.count(old_config)==1
script=script.replace(old_guard,c['bindings']['runner']['path']).replace(old_config,str(P/'config.final.json'))
assert script.count(c['bindings']['runner']['path'])==script.count(str(P/'config.final.json'))==1
prefix,oldhash=script.rsplit(' ',1);assert prefix.endswith('--config-sha256') and oldhash=='<ROOT_ACTUAL_FINAL_SUCCESSOR_CONFIG_SHA256>'
script=prefix+' '+finalsha
assert script.endswith('--config-sha256 '+finalsha) and '<'not in script
cg='/sys/fs/cgroup'+c['cgroup']['envelope_path']+'/'+c['cgroup']['supervisor_leaf_name']+'/cgroup.procs'
assert script.splitlines()[0]=='echo $$ > '+cg and script.count(cg)==1
argv['argv'][-1]=script;argv['cwd']=c['paths']['worktree']
argv.update(status='ROOT_ISSUED_P10_ARGV_NOT_YET_EXECUTED',config_final_sha256=finalsha,config_inputs={'path':str(P/'config-inputs.json'),'sha256':projsha});put(P/'ROOT-ACTUAL-ARGV.json',argv)
print(json.dumps({'status':'ROOT_ISSUED_P10_METADATA_DAG_ONLY_NO_NATIVE_EXECUTION','config_sha256':finalsha,'projection_sha256':projsha,'stat_changed_rows':len(changes),'fresh_QOBS_sha256':sha(req['paths']['observation_output']),'final_preflight_required':True}))
