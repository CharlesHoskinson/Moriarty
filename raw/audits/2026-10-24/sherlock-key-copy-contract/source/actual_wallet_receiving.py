"""Actual adopted deployment -> one final-balanced wallet call data/readback.
Selected by the existing owned receiver, never a standalone dispatcher. No main.
"""
# Every check is an assert; -O/PYTHONOPTIMIZE strips them here and in exec'd modules: refuse first.
if not __debug__:raise SystemExit('assertions required')
from pathlib import Path
import hashlib,json,os,stat,time

def pin(p):
 p=Path(p);b=p.read_bytes();return dict(path=str(p),bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
ADOPTION=(6377,'b1c5a10f7531e16beb64b4310ab7d05493874acd01815a05b732a76b108df14b');SDK06_CONTROL='9e467975878fec787c8da507407ab75d86c779b913bf7fffcee82e16387c85c2'
TESTED_CAPTURE_PUBLIC=(11457,'14f0d1e6b9b50daeae69c0dd9d796b1c851f74393bbd83f8b692bfc583cd4d05')
# Approved C2 integration/proof_protocol_v2.py byte copy under the legacy basename; old D protocol bytes never match.
PROOF_PROTOCOL=('proof_protocol.py',32778,'32f355f6dbc05daf97310f042a0ac7d1a75c127900e44ac36a99b0e6f2f5a89f')
# C4 oracle accepting exactly the two C3 writer keys; the old oracle bytes never match.
WALLET_RECEIPT_ORACLE=('wallet_receipt_oracle.py',12349,'4a44fae9ae248247f7d3792972ccf23221fb032209305fc589976f5a651ba96e')
def module(row,checked):
 # Caller pin plus a local hash of the exact bytes executed; no reread between check and exec.
 checked(row);b=Path(row['path']).read_bytes();assert (len(b),hashlib.sha256(b).hexdigest())==(row['bytes'],row['sha256']);ns={'__name__':'actual_call_receiving','__file__':row['path']};exec(compile(b,row['path'],'exec'),ns);return ns
def public_module(cfg,checked):
 # ROOT typed-public22 tested capture_public.py identity, asserted before any use.
 assert (cfg['capturePublicModule']['bytes'],cfg['capturePublicModule']['sha256'])==TESTED_CAPTURE_PUBLIC;return module(cfg['capturePublicModule'],checked)
def adopted_build(cfg,ad,rc,pinned_json):
 # No ad.configuration descriptor: ad.sourceFreeze -> full freeze.files[] unique control row with sha ad.configuration_sha256 -> full SDK06 control. Missing fields raise.
 assert ad['configuration_sha256']==SDK06_CONTROL;files=pinned_json(ad['sourceFreeze'])['files'];assert len({r['path'] for r in files})==len(files)
 rows=[r for r in files if Path(r['path']).name=='EXACT-CONTROL-AND-OPERATIONS.HELD.json'];assert len(rows)==1 and rows[0]['path']==str(Path(ad['sourceFreeze']['path']).parent/'EXACT-CONTROL-AND-OPERATIONS.HELD.json') and rows[0]['sha256']==SDK06_CONTROL
 control=pinned_json({k:rows[0][k] for k in ['path','bytes','sha256']});P=lambda rs:{r['path']:(r['bytes'],r['sha256']) for r in rs};build=P(cfg['buildMountFiles']);root=Path(cfg['buildDirectory'])
 assert control['buildDirectory']==cfg['buildDirectory'] and P(control['buildMountFiles'])==build and len(build)==len(cfg['buildMountFiles'])==13
 assert build[str(root/'keys/acceptValidity.verifier')]==(1353,'f2d10277b0e853e37e448e1a57c97ab6d3edcf6f5d15cab8673c8708136796f4') and build[str(root/'keys/settleValidity.verifier')][1]==rc['settle_verifier_sha256']
 return control
def adopted_key_copies(b,kad,control,mcfg,ad,rc,cfg):
 # b key rows are the SDK06 staged copies (materializer pins adopted rc.key_dir), never the keygen02 rows themselves.
 # Origin: the unique Root-adopted keygen02 products, frozen as the SDK06-control-pinned material02 keyProducts of the same keygen adoption.
 # Copy: exact SDK06 control stagedKeyDir/name (= adopted rc.key_dir, under the sourceFreeze root) carrying exactly the origin bytes/sha.
 names={'decider.vk','decider.relation'};key={Path(r['path']).name:r for r in kad['products']};mat={Path(r['path']).name:r for r in mcfg['keyProducts']}
 assert len(key)==len(kad['products'])==2 and set(key)==names and len(mat)==len(mcfg['keyProducts'])==2 and set(mat)==names
 O=lambda r:(r['path'],r['bytes'],r['sha256']);ka,ma=cfg['actualKeygenAdoption'],mcfg['actualKeygenAdoption']
 assert all(O(mat[n])==O(key[n]) for n in names) and (ma['path'],ma['sha256'])==(ka['path'],ka['sha256'])
 root=Path(ad['sourceFreeze']['path']).parent;keys=Path(control['stagedKeyDir']);run=Path(control['privateRuntimeRoot'])
 assert control['preparationRoot']==str(root) and keys.parent==run and run.parent==root and ad['deciderConfiguration']['path']==control['deciderConfigurationPath'] and rc['key_dir']==control['stagedKeyDir']
 for k,n in [('rawDeciderVK','decider.vk'),('deciderRelation','decider.relation')]:
  r=b[k];assert set(r)=={'path','bytes','sha256'} and r['path']==str(keys/n) and type(r['bytes']) is int and (r['bytes'],r['sha256'])==(key[n]['bytes'],key[n]['sha256'])
 return key
def durable(p,v):
 p=Path(p)
 with p.open('x') as f:json.dump(v,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
 fd=os.open(p.parent,os.O_RDONLY|os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)
def copy(row,path,mode,checked):
 path=Path(path);path.parent.mkdir(parents=True,exist_ok=True,mode=0o700)
 with Path(row['path']).open('rb') as inp,path.open('xb') as out:
  for b in iter(lambda:inp.read(1024**2),b''):out.write(b)
  out.flush();os.fsync(out.fileno())
 os.chmod(path,mode);checked(row,path,mode)

def capture_public_expected(cfg,b,checked):
 # Closed sherlock-capture-public-expected/1 DATA from Root-adopted cfg fields plus adopted tagged D only; no capture/projection/provenance/constructor read, no capture-time equality assumed.
 m=public_module(cfg,checked);assert m['EXPECTED_SCHEMA']=='sherlock-capture-public-expected/1' and len(m['BUILD_FILES'])==9
 root=Path(cfg['buildDirectory']);rows={r['path']:r for r in cfg['buildMountFiles']};assert len(rows)==len(cfg['buildMountFiles'])==13
 i=lambda r:dict(bytes=r['bytes'],sha256=r['sha256']);build={f:i(rows[str(root/f)]) for f in m['BUILD_FILES']}
 # Settle is the cfg build row; callers bind cfg rows to the adopted SDK06 control and to rc.settle_verifier_sha256 (historical adopted readback). Four roles pairwise distinct.
 roles=[b['rawDeciderVK']['sha256'],b['taggedVerifier']['sha256'],build['keys/acceptValidity.verifier']['sha256'],build['keys/settleValidity.verifier']['sha256']];assert len(set(roles))==4
 e=dict(schema=m['EXPECTED_SCHEMA'],templateSha256=cfg['publicTemplate']['sha256'],ctor=cfg['expectedConstructor'],source=i(cfg['expectedCaptureSource']),guards=i(cfg['expectedCaptureGuards']),build=build,settleVerifierSha256=roles[3],taggedVerifierSha256=roles[1])
 m['build_public'](e);return e,m['canonical_bytes'](e),m['public_identity'](e)

def binding(cfg,checked,pinned_json,actual_elf_binding):
 decision=json.loads(Path(cfg['resourceDecisionPath']).read_text())
 assert decision['status']=='ADOPTED_SOURCE46_ACTUAL_WALLET_BOUND_D_CALL01' and decision['configuration_sha256']==pin(Path(cfg['preparationRoot'])/'EXACT-CONTROL-AND-OPERATIONS.HELD.json')['sha256']
 assert decision['attempts']==1 and decision['P_hold']==3740 and decision['S_hold']==3740 and decision['whole_hold']==1900
 assert decision['C13']=='STOPPED_UNCHANGED' and decision['old_holds_retained'] is True and len(set(decision['agreeing_review_ids']))>=2
 elf=actual_elf_binding(decision)
 row=decision['actual_call_binding'];assert row['path']==cfg['actualCallBindingPath'];b=pinned_json(row)
 assert b['status']=='BOUND_ACTUAL_ADOPTED_STAGENET_DEPLOYMENT_AND_GENUINE_D_CALL_INPUTS'
 assert (b['deploymentAdoption']['bytes'],b['deploymentAdoption']['sha256'])==ADOPTION;ad=pinned_json(b['deploymentAdoption']);assert ad['status']=='ADOPTED_ACTUAL_SHERLOCK_STAGENET_DEPLOYMENT_AND_COMPLETE_OWNED_CLOSE' and ad['C13']=='STOPPED_UNCHANGED'
 assert ad['address']==b['address'] and type(b['address']) is str and b['address'] and ad['deploymentRecord']==b['deploymentRecord'] and ad['deploymentBinding']==b['deploymentBinding'] and ad['preparedRecord']==b['preparedRecord']
 assert len(b['address'])%2==0 and all(c in '0123456789abcdef' for c in b['address']),'normalized deployment address'
 checked(b['deploymentActualReview'])
 kad=pinned_json(cfg['actualKeygenAdoption']);assert kad['status']=='ADOPTED_ACTUAL_SOURCE46_BOUND_D_KEYGEN_AND_COMPLETE_OWNED_CLEANUP' and kad['configuration_sha256']==cfg['keygenConfiguration']['sha256'] and kad['full1330HoldRetained'] is True and kad['C13']=='STOPPED_UNCHANGED' and kad['Source46ELF_SHA256']==elf['retainedELF']['sha256']
 for r in kad['products']:checked(r)
 # Typed origin-versus-copy against the authenticated SDK06 control, never full-row equality; originals (above) and copies (below) stay live-checked.
 assert b['originalDeciderConfiguration']==ad['deciderConfiguration'];rc=pinned_json(b['originalDeciderConfiguration']);control=adopted_build(cfg,ad,rc,pinned_json)
 adopted_key_copies(b,kad,control,pinned_json({k:control['materialConfiguration'][k] for k in ['path','bytes','sha256']}),ad,rc,cfg)
 tad=pinned_json(b['taggerAdoption']);assert tad['status']=='ADOPTED_ACTUAL_HANDKEY_TAG_BUILD01_AND_COMPLETE_OWNED_CLEANUP' and tad['configuration_sha256']==cfg['taggerConfiguration']['sha256'] and tad['full1340HoldRetained'] is True and tad['C13']=='STOPPED_UNCHANGED';checked(tad['retainedELF'])
 for k in ['rawDeciderVK','deciderRelation','taggedVerifier','tagging','provenance','captureProjection','originalDeciderConfiguration','deploymentRecord','deploymentBinding','preparedRecord']:checked(b[k])
 provenance=pinned_json(b['provenance']);projection=pinned_json(b['captureProjection']);record=pinned_json(b['deploymentRecord']);db=pinned_json(b['deploymentBinding']);prepared=pinned_json(b['preparedRecord'])
 assert record['schema']=='sherlock-anchor-deployment/1' and record['address']==b['address'] and db['schema']=='sherlock-deployment-binding/1' and db['address']==b['address'] and db['deployment_record_sha256']==b['deploymentRecord']['sha256'] and db['prepared_record_sha256']==b['preparedRecord']['sha256']
 assert db['env']==db['network_id']=='stagenet' and db['receipt']['status']=='SUCCESS' and db['receipt']['finalized'] is True and db['receipt']['bytes_match'] is True
 assert rc['kind']=='sherlock' and rc['bin_sha256']==elf['retainedELF']['sha256'] and rc['handkey_sha256']==cfg['helper']['expectedSHA256'] and rc['parent_vk_sha256']==cfg['parentVK_SHA256'] and rc['template_sha256']==cfg['publicTemplate']['sha256']
 assert rc['key_dir_sha256']=={'decider.vk':b['rawDeciderVK']['sha256'],'decider.relation':b['deciderRelation']['sha256']}
 for k,bk in [('verifier_sha256','taggedVerifier'),('tagging_sha256','tagging'),('provenance_sha256','provenance'),('capture_sha256','captureProjection')]:assert rc[k]==b[bk]['sha256']
 assert provenance['decider_vk_raw_sha256']==b['rawDeciderVK']['sha256'] and provenance['decider_relation_sha256']==b['deciderRelation']['sha256'] and provenance['template_sha256']==cfg['publicTemplate']['sha256']
 assert projection['template']['sha256']==cfg['publicTemplate']['sha256'] and projection['template']['provisional'] is False
 assert b['configuration']['path']==str(Path(cfg['preparationRoot'])/'EXACT-CONTROL-AND-OPERATIONS.HELD.json') and b['configuration']['sha256']==decision['configuration_sha256']
 row=b['capturePublicExpected'];assert row['path']==cfg['capturePublicExpectedPath'];checked(row)
 e,data,identity=capture_public_expected(cfg,b,checked);assert Path(row['path']).read_bytes()==data and row['sha256']==hashlib.sha256(data).hexdigest() and b['capturePublicIdentity']==identity
 assert e['templateSha256']==rc['template_sha256'] and e['taggedVerifierSha256']==rc['verifier_sha256'] and e['settleVerifierSha256']==rc['settle_verifier_sha256']
 return decision,b,elf,rc

def check_auxiliary_closure(cfg,checked,pinned_json):
 qualification=pinned_json(cfg['walletSDKQualification']);assert qualification['reviewedSourceRows']==cfg['walletQualifiedPublicSourceRows']
 for r in cfg['walletQualifiedPublicSourceRows']:checked(r)
 build=pinned_json(cfg['nativeBuildConfiguration']);assert build['cargoCwd']==cfg['bakedCargoManifestDirectory']
 for alias in cfg['toolAliases']:assert str(Path(alias['path']).resolve(strict=True))==alias['resolved']
 fee=pinned_json(cfg['feeDataObservation']);qualification=cfg['feeProvingDataQualification'];cache=qualification['localCache']
 assert qualification['schema']=='sherlock-sdk-dust-dynamic-public-inputs/1' and qualification['remoteBytes']=='UNPINNED_DYNAMIC_HTTPS' and qualification['remoteBodyHashes'] is None
 assert cache['rows']==fee['rows'] and [r['k'] for r in cache['rows']]==list(range(5,19)) and cache['missingDegrees']==list(range(5))
 for r in cache['rows']:checked(r)
 for p in cache['missingPaths']:assert not Path(p).exists() and not Path(p).is_symlink()
 for p in cfg['absentNativeSRSAlternatives']+[cfg['absentHelperSRSDirectory']]:assert not Path(p).exists() and not Path(p).is_symlink()

def check_code_inputs(cfg,checked,pinned_json):
 for r in cfg['frozenPublicFiles']+cfg['sourceFiles']+cfg['toolPins']+[cfg['moduleClosure']]:checked(r)
 for r in [cfg['srs14'],cfg['srs20']]:checked(r,mode=r['mode'])
 check_auxiliary_closure(cfg,checked,pinned_json)
 manifest=pinned_json(cfg['moduleClosure'])
 for r in manifest['files']:checked(r)
 for r in manifest['symlinks']:
  p=Path(r['path']);assert p.is_symlink() and os.readlink(p)==r['target'] and str(p.resolve())==r['resolved']
 assert len(manifest['files'])==16436 and len(manifest['symlinks'])==4
 declaredFiles={};declaredLinks={}
 for graph in cfg['walletSDKGraphs']:
  rows=pinned_json(graph['manifest']);assert type(rows) is list and len(rows)==graph['leaves']
  for row in rows:
   name=str(Path(graph['root'])/row['path'])
   if row['kind']=='file':declaredFiles[name]=(row['bytes'],row['sha256'])
   else:assert row['kind']=='symlink';declaredLinks[name]=row['target']
 assert declaredFiles=={r['path']:(r['bytes'],r['sha256']) for r in manifest['files']} and declaredLinks=={r['path']:r['target'] for r in manifest['symlinks']}
 for alias in cfg['walletSDKLinks']:
  p=Path(alias['path']);assert p.is_symlink() and os.readlink(p)==alias['target'] and str(p.resolve())==alias['expected_physical_root']
 for group in manifest['membership']:
  root=Path(group['root']);seen=set()
  for base,dirs,files in os.walk(root,followlinks=False):
   for name in list(dirs):
    if (Path(base)/name).is_symlink():files.append(name);dirs.remove(name)
   seen.update(str((Path(base)/name).relative_to(root)) for name in files)
  assert seen==set(group['relative_files'])
 lock=Path(cfg['sharedLock']['path']);s=lock.lstat();assert not any(p.is_symlink() for p in [lock,*lock.parents]);assert stat.S_ISREG(s.st_mode) and s.st_size==0 and s.st_dev==cfg['sharedLock']['device'] and s.st_ino==cfg['sharedLock']['inode']
 # The native fixed-k lookup must reach the pinned HOME cache. Earlier baked
 # manifest alternatives have exact explicit absence predicates, alongside the262-file source closure.
 root=Path(cfg['sourceRoot']);seen=set()
 for base,dirs,files in os.walk(root,followlinks=False):
  assert not any((Path(base)/x).is_symlink() for x in dirs+files);seen.update(str(Path(base)/x) for x in files)
 assert seen=={r['path'] for r in cfg['sourceFiles']} and len(seen)==262
 disk=Path(cfg['preparationRoot']);mounts=[]
 for line in Path('/proc/self/mountinfo').read_text().splitlines():
  head,tail=line.split(' - ',1);mounts.append((Path(head.split()[4]),tail.split()[0]))
 rootMount=max((m for m in mounts if disk.is_relative_to(m[0])),key=lambda m:len(str(m[0])));assert rootMount[1]=='ext4' and not any(m[0].is_relative_to(disk) for m in mounts if m[0]!=rootMount[0])
 space=os.statvfs(cfg['preparationRoot']);assert space.f_bavail*space.f_frsize>=4*1024**3 and space.f_favail>=32768
 return dict(sourceCount=262,SDKRegularFiles=len(manifest['files']),SDKLinks=len(manifest['symlinks']),SDKLeaves=len(manifest['files'])+len(manifest['symlinks']))

def frozen_inputs(cfg,checked,pinned_json,actual_elf_binding):
 decision,b,elf,rc=binding(cfg,checked,pinned_json,actual_elf_binding)
 source=check_code_inputs(cfg,checked,pinned_json)
 return dict(**source,actualELF=elf['retainedELF'],actualKeygen=cfg['actualKeygenAdoption'],actualDeployment=b['deploymentAdoption'],actualAddress=b['address'],actualCallBinding=decision['actual_call_binding'],allPublicSourceAndSDKPins=True,privatePayloadManuallyInspected=False)

def stage_inputs(cfg,checked,pinned_json,actual_elf_binding):
 decision,b,elf,rc=binding(cfg,checked,pinned_json,actual_elf_binding)
 # New C2 wrapper pins from the already-authenticated public control fields, checked before any staging side effect;
 # never assumed present in the adopted rc and never allowed to differ from it.
 dom,srs20=cfg['expectedDomain'],cfg['srs20']['boundPath']
 assert type(dom) is str and len(dom)==64 and all(c in '0123456789abcdef' for c in dom) and int.from_bytes(bytes.fromhex(dom),'little')<int('73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001',16),'expected domain'
 assert type(srs20) is str and srs20.startswith('/') and all(' '<=c<='~' for c in srs20),'srs20 bound path'
 assert rc.get('expected_dom_stmt',dom)==dom and rc.get('srs20_bound_path',srs20)==srs20,'adopted rc pin conflict'
 proto=cfg['proofProtocol'];assert (Path(proto['path']).name,proto['bytes'],proto['sha256'])==PROOF_PROTOCOL,'C2 proof protocol copy';checked(proto)
 orc=cfg['walletReceiptOracle'];assert (Path(orc['path']).name,orc['bytes'],orc['sha256'])==WALLET_RECEIPT_ORACLE,'C4 wallet receipt oracle';checked(orc)
 # Opaque C3 identity: the already-verified binding row (binding() asserted it equals public_identity(e)); staged, never recomputed here.
 ident=b['capturePublicIdentity'];assert type(ident) is dict and list(ident)==['schema','bytes','sha256'] and rc.get('capture_public_identity',ident)==ident,'capture public identity pin'
 for k in ['nativeHOME','maskEmpty','capturePhase','TMPDIR','privateRuntimeRoot']:Path(cfg[k]).mkdir(mode=0o700)
 Path(cfg['outputRoot']).mkdir(mode=0o700)
 # bound-d-call-phase.mjs requires both executables as same-UID regular hash-pinned 0555 copies.
 copy(elf['retainedELF'],cfg['executionCopyPath'],0o555,checked);copy(cfg['helper']['actualRetained'],cfg['helper']['privateExecutableCopy'],0o555,checked)
 inputs=[('rawDeciderVK','decider.vk'),('deciderRelation','decider.relation'),('taggedVerifier','tagged-decider.vk'),('tagging','tagging.json'),('provenance','provenance.json'),('captureProjection','capture-projection.private.json')]
 for k,name in inputs:copy(b[k],Path(cfg['stagedMaterialDir'])/name,0o444,checked)
 copy(cfg['publicTemplate'],Path(cfg['stagedMaterialDir'])/'template.json',0o444,checked)
 for r in cfg['genuineW']:copy(r,Path(cfg['stagedWDir'])/Path(r['path']).name,0o444,checked)
 for k,name in [('deploymentRecord','deployment-record.json'),('deploymentBinding','deployment-record.json.binding.json'),('preparedRecord','deployment-record.json.prepared.json')]:copy(b[k],Path(cfg['privateRuntimeRoot'])/name,0o444,checked)
 # Raw projection/provenance stay in RO stagedMaterialDir; publicOutputRoot stays empty for the wrapper.
 e,_,_=capture_public_expected(cfg,b,checked)
 rc.update(bin=cfg['executionCopyPath'],key_dir=cfg['stagedKeyDir'],verifier=str(Path(cfg['stagedMaterialDir'])/'tagged-decider.vk'),tagging=str(Path(cfg['stagedMaterialDir'])/'tagging.json'),provenance=str(Path(cfg['stagedMaterialDir'])/'provenance.json'),capture=str(Path(cfg['stagedMaterialDir'])/'capture-projection.private.json'),capture_public_expected=e,capture_public_expected_sha256=b['capturePublicExpected']['sha256'],capture_public_identity=ident,w_dir=str(Path(cfg['privateRuntimeRoot'])/'w-{batch}'),route_log=str(Path(cfg['privateRuntimeRoot'])/'route.log'),template=str(Path(cfg['stagedMaterialDir'])/'template.json'),expected_dom_stmt=dom,srs20_bound_path=srs20)
 # Separate raw key directory retains exact two-file production key-dir guard.
 for k,name in [('rawDeciderVK','decider.vk'),('deciderRelation','decider.relation')]:copy(b[k],Path(cfg['stagedKeyDir'])/name,0o444,checked)
 durable(cfg['deciderConfigurationPath'],rc);os.chmod(cfg['deciderConfigurationPath'],0o444)
 durable(Path(cfg['preparationRoot'])/'execution01/control01/PRE-PROOF-OPERANDS.json',dict(status='STAGED_ACTUAL_ADOPTED_DEPLOYMENT_CALL_OPERANDS',actualCallBinding=decision['actual_call_binding'],rawDeciderVK=b['rawDeciderVK'],deciderRelation=b['deciderRelation'],provenance=pin(Path(cfg['stagedMaterialDir'])/'provenance.json'),captureProjection=pin(Path(cfg['stagedMaterialDir'])/'capture-projection.private.json'),capturePublicExpected=b['capturePublicExpected'],capturePublicIdentity=b['capturePublicIdentity'],deciderConfiguration=pin(cfg['deciderConfigurationPath'])))
 assert not any(Path(cfg['outputRoot']).iterdir())

def operation(cfg,checked,pinned_json,actual_elf_binding,deadline_record=None):
 decision,b,elf,rc=binding(cfg,checked,pinned_json,actual_elf_binding);op=json.loads(json.dumps(cfg['operation']))
 op['argv'] += ['--deployment-record-sha256',b['deploymentRecord']['sha256'],'--deployment-binding-sha256',b['deploymentBinding']['sha256']]
 if deadline_record is None:return op
 assert deadline_record['nativeSubjectBudgetSeconds']==1800 and deadline_record['deadlineEpochMS']==deadline_record['dispatchEpochMS']+1800000
 op['environment']['MORIARTY_SHERLOCK_OPERATION_DEADLINE_MS']=str(deadline_record['deadlineEpochMS']);return op

def inspect_output(cfg,checked,pinned_json,actual_elf_binding):
 decision,b,elf,rc=binding(cfg,checked,pinned_json,actual_elf_binding)
 operands=json.loads((Path(cfg['preparationRoot'])/'execution01/control01/PRE-PROOF-OPERANDS.json').read_text());assert operands['actualCallBinding']==decision['actual_call_binding']
 for k in ['rawDeciderVK','deciderRelation']:assert operands[k]==b[k]
 for k in ['provenance','captureProjection','deciderConfiguration']:checked(operands[k])
 staged=pinned_json(operands['deciderConfiguration']);assert operands['deciderConfiguration']['path']==cfg['deciderConfigurationPath'] and staged['capture_public_expected_sha256']==b['capturePublicExpected']['sha256'] and public_module(cfg,checked)['canonical_bytes'](staged['capture_public_expected'])==Path(b['capturePublicExpected']['path']).read_bytes()
 assert staged['expected_dom_stmt']==cfg['expectedDomain'] and staged['srs20_bound_path']==cfg['srs20']['boundPath'] and staged['settle_verifier_sha256']==rc['settle_verifier_sha256'] and staged['capture_public_identity']==b['capturePublicIdentity'],'staged rc pins'
 assert [operands[k]['path'] for k in ['provenance','captureProjection']]==[str(Path(cfg['stagedMaterialDir'])/n) for n in ['provenance.json','capture-projection.private.json']] and all(operands[k]['sha256']==b[k]['sha256'] for k in ['provenance','captureProjection'])
 assert operands['capturePublicExpected']==b['capturePublicExpected'] and operands['capturePublicIdentity']==b['capturePublicIdentity']
 command=json.loads((Path(cfg['capturePhase'])/'COMMAND.json').read_text());deadline=json.loads((Path(cfg['preparationRoot'])/'execution01/control01/DISPATCH-DEADLINE.json').read_text());expected=operation(cfg,checked,pinned_json,actual_elf_binding,deadline)
 assert set(command)==set(expected) and all(command[k]==v for k,v in expected.items() if k!='wallSeconds') and type(command['wallSeconds']) in (int,float) and 1810<=command['wallSeconds']<=1830
 # Actual final pre-state is read by the pinned production driver at this same
 # chain address. The fee-balanced callback freeze is its public oracle receipt.
 before=json.loads((Path(cfg['outputRoot'])/'EXPECTED-PUBLIC-INPUTS.BEFORE-PROOF.json').read_text());intent=before['submission_intent'];assert intent['address']==b['address'] and intent['deployment_record_sha256']==b['deploymentRecord']['sha256']
 # The entire wallet stream remains PRIVATE. Only the selected driver's fixed
 # terminal summary can be projected after exact owned cleanup by its oracle.
 # The oracle runs first: the driver's C3 call-proof profile (A evidence checked in process, rechecked after
 # readback) is the callContext/provingModel authority. C2 safe/4 is only compared to it, never its source.
 # Pinned C4 oracle (module() re-hashes the executed bytes): exact two C3 keys, call_proof checked against b.
 orc=cfg['walletReceiptOracle'];assert (Path(orc['path']).name,orc['bytes'],orc['sha256'])==WALLET_RECEIPT_ORACLE,'C4 wallet receipt oracle'
 wallet=module(orc,checked)['inspect_actual_call'](cfg,b)
 profile=wallet['call_proof'];assert type(profile) is dict,'C3 profile';ctx,model=profile['call_context'],profile['proving_model'];a=b['address'];a=(a[2:] if a.startswith('0x') else a).lower()
 assert type(ctx) is dict and type(model) is dict and profile['capture_public']==b['capturePublicIdentity'] and ctx['network']=='stagenet' and ctx['entry_point']=='acceptValidity' and ctx['address']==a and ctx['deployment_record_sha256']==b['deploymentRecord']['sha256'],'C3 profile context'
 dynamic=dict(cfg);dynamic.update({k:operands[k] for k in ['rawDeciderVK','deciderRelation','provenance','captureProjection','capturePublicExpected','capturePublicIdentity']});dynamic.update(publicOutputRoot=cfg['outputRoot'],submissionIntent=intent,settleVerifierSha256=rc['settle_verifier_sha256'],callContext=dict(ctx),provingModel=dict(model))
 proto=cfg['proofProtocol'];assert (Path(proto['path']).name,proto['bytes'],proto['sha256'])==PROOF_PROTOCOL,'C2 proof protocol copy'
 result=module(proto,checked)['inspect_products'](dynamic)
 # Metadata-only binding of exactly these ten products to the oracle-authenticated finalized outcome;
 # any refusal leaves the root DIRTY: retained, not adopted, no resend/revert.
 bound=public_module(cfg,checked)['bind_receipt'](wallet,result['products'],b['capturePublicIdentity']['sha256'],cfg['outputRoot'],ctx['address'])
 return dict(proof=result,wallet=wallet,receiptBinding=bound,scope='ACTUAL_REAL_ADDRESS_FINAL_FEE_BALANCED_PROOF_AND_OWNED_SUBMISSION_PENDING_ROOT_ACTUAL_REVIEW')
