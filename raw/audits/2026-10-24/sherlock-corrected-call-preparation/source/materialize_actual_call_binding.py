"""After Root actual deployment adoption: freeze existing actual public operands.
Data only: no SDK, wallet, network, native, resource or orchestration action.
"""
# Every check is an assert; -O/PYTHONOPTIMIZE strips them here and in exec'd modules: refuse first.
if not __debug__:raise SystemExit('assertions required')
from pathlib import Path
import hashlib,json,os,sys
D=Path(__file__).resolve().parent;N=D.parents[1]
def pin(p):
 p=Path(p);assert not any(q.is_symlink() for q in [p,*p.parents]);b=p.read_bytes();return dict(path=str(p),bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
ADOPTION=dict(path=str(N/'native/source46-sdk-deploy06/revision06/ROOT-SCOPED-ACTUAL-STAGENET-DEPLOYMENT-ADOPTION.json'),bytes=6377,sha256='b1c5a10f7531e16beb64b4310ab7d05493874acd01815a05b732a76b108df14b')
def exact(row):
 # One read: the hashed bytes are the bytes parsed or executed.
 p=Path(row['path']);assert not any(q.is_symlink() for q in [p,*p.parents]);b=p.read_bytes();assert (str(p),len(b),hashlib.sha256(b).hexdigest())==(row['path'],row['bytes'],row['sha256']);return b
def checked(row):exact(row)
def load(row):return json.loads(exact(row))
def module(row):
 ns={'__name__':'actual_call_receiving','__file__':row['path']};exec(compile(exact(row),row['path'],'exec'),ns);return ns
def publish(p,data):
 # Exclusive atomic: fsynced partial, then link refuses any existing target; leftovers fail closed.
 # Dirty one attempt: a failure after the expected publish leaves it (or a partial) behind; the attempt is consumed, no cleanup or rerun.
 p=Path(p);assert p.parent==D;t=p.with_name('.'+p.name+'.partial')
 with t.open('xb') as f:f.write(data);f.flush();os.fsync(f.fileno())
 os.link(t,p);os.unlink(t)
 fd=os.open(D,os.O_RDONLY|os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)
 return pin(p)
def main():
 assert len(sys.argv)==3,'Usage: materialize_actual_call_binding.py ROOT_ACTUAL_DEPLOYMENT_ADOPTION_JSON REVIEWED_CONFIGURATION_SHA256'
 raw=(D/'EXACT-CONTROL-AND-OPERATIONS.HELD.json').read_bytes();assert hashlib.sha256(raw).hexdigest()==sys.argv[2];cfg=json.loads(raw);assert sys.argv[1]==ADOPTION['path'];adpin=dict(ADOPTION);ad=load(adpin);assert ad['status']=='ADOPTED_ACTUAL_SHERLOCK_STAGENET_DEPLOYMENT_AND_COMPLETE_OWNED_CLOSE' and ad['C13']=='STOPPED_UNCHANGED'
 rc=load(ad['deciderConfiguration']);record=load(ad['deploymentRecord']);db=load(ad['deploymentBinding']);prepared=load(ad['preparedRecord']);assert record['address']==db['address']==ad['address'] and db['deployment_record_sha256']==ad['deploymentRecord']['sha256'] and db['prepared_record_sha256']==ad['preparedRecord']['sha256']
 assert db['env']==db['network_id']=='stagenet' and db['receipt']['status']=='SUCCESS' and db['receipt']['finalized'] is True and db['receipt']['bytes_match'] is True
 key=Path(rc['key_dir']);tagger=N/'native/root-actual-handkey-tag-build01/ROOT-SCOPED-ACTUAL-TAGGER-BUILD-ADOPTION.json'
 b=dict(status='BOUND_ACTUAL_ADOPTED_STAGENET_DEPLOYMENT_AND_GENUINE_D_CALL_INPUTS',deploymentAdoption=adpin,deploymentActualReview=ad['actualReview'],address=ad['address'],deploymentRecord=ad['deploymentRecord'],deploymentBinding=ad['deploymentBinding'],preparedRecord=ad['preparedRecord'],originalDeciderConfiguration=ad['deciderConfiguration'],taggerAdoption=pin(tagger),rawDeciderVK=pin(key/'decider.vk'),deciderRelation=pin(key/'decider.relation'),taggedVerifier=pin(rc['verifier']),tagging=pin(rc['tagging']),provenance=pin(rc['provenance']),captureProjection=pin(rc['capture']),configuration=dict(path=str(D/'EXACT-CONTROL-AND-OPERATIONS.HELD.json'),bytes=len(raw),sha256=sys.argv[2]))
 assert b['rawDeciderVK']['sha256']=='bd9e9d89ad0a5570571160b29dc8b2d89e11e3c6065b46638a2e978f9218776e' and b['deciderRelation']['sha256']=='08b7f6db1db1d7143bfa7660dbb29c8070683cfca5757d690857f2e0a657785f'
 # Expected public DATA only after the adopted rc tagged-D pin; Root-reviewed cfg fields, never capture/projection.
 assert rc['verifier_sha256']==b['taggedVerifier']['sha256'] and rc['template_sha256']==cfg['publicTemplate']['sha256'] and rc['key_dir_sha256']=={'decider.vk':b['rawDeciderVK']['sha256'],'decider.relation':b['deciderRelation']['sha256']}
 # Settle row is bound to the historical adopted readback value rc.settle_verifier_sha256 (KeyError if absent), not to itself.
 rows={r['path']:r for r in cfg['buildMountFiles']};assert rows[str(Path(cfg['buildDirectory'])/'keys/settleValidity.verifier')]['sha256']==rc['settle_verifier_sha256']
 common=module(cfg['commonSource']);common['adopted_build'](cfg,ad,rc,load)
 e,data,identity=common['capture_public_expected'](cfg,b,checked)
 b.update(capturePublicExpected=publish(cfg['capturePublicExpectedPath'],data),capturePublicIdentity=identity)
 print(json.dumps(publish(cfg['actualCallBindingPath'],(json.dumps(b,indent=2)+'\n').encode())))
if __name__=='__main__':main()
