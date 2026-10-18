"""R-only metadata adaptation of the existing automated-tier1 assembler.
No guard/operator imports, test runs, provisioning, SDK payload or native operation.
Outputs are PROPOSED; Root retains source/test judgment and adoption.
"""
from pathlib import Path
import argparse, ast, copy, hashlib, json

R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
D=Path(__file__).resolve().parent
SOURCE=R/'53-r2-integrated-source-candidate/source'
OLD=R/'387-fresh05-generated-membership-metadata/config-assembly/config.final.proposed.json'
GUARD=R/'392-native05-owner-literal-candidate/bounded_sdk_compile.py'
OPERATOR=R/'275-owner-literal-mock-fixes/afterimages/operator_provisioner.py'
GUARD_SHA='bb1492fd2b4b1411b873a9131073fa91d985da3c167dd557e99bccf9b6d73d34'

def pin(path):
    path=Path(path).resolve(); raw=path.read_bytes()
    return {'path':str(path),'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()}
def binding(path):
    p=pin(path); return {key:p[key] for key in ('path','sha256')}
def load(path): return json.loads(Path(path).read_text())

def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--qobs',type=Path,required=True)
    ap.add_argument('--staging',type=Path,required=True)
    ap.add_argument('--old-home-stats',type=Path,required=True)
    ap.add_argument('--security-record',type=Path,required=True)
    ap.add_argument('--nonsecurity-record',type=Path,required=True)
    ap.add_argument('--focused-test',type=Path,required=True)
    ap.add_argument('--test-result',type=Path,required=True)
    ap.add_argument('--runner',type=Path,default=GUARD)
    ap.add_argument('--output-dir',type=Path,required=True)
    args=ap.parse_args()
    request=D/'request.fresh06.candidate.json'; req=load(request); obs=load(args.qobs)
    assert pin(request)['sha256']=='59e46b4dd7b80598956c7dda6ec113b2d4b27257087d9d9ba652e9018391f0b9'
    assert pin(args.runner)['sha256']==GUARD_SHA
    assert pin(OPERATOR)['sha256']=='7a8e55b908a82f138037d6cc4cc87dca4105cad1ee5838b6110555a40e7f6476'
    assert obs['schema']=='sherlock-operator-provisioning-observation/1'
    assert obs['request_sha256']==pin(request)['sha256'] and obs['provisioner_sha256']==pin(OPERATOR)['sha256']
    c=load(OLD); before=copy.deepcopy(c)
    assert c['expected_generated_files']==load(D/'EXPECTED-GENERATED-575.preimage.json')
    c['expected_generated_files']=load(D/'EXPECTED-GENERATED-579.proposed.json')
    for key in ('execution_root','source_home','target','tmp','packet','empty_home','mask_empty'):
        c['paths'][key]=req['paths'][key]
    c['paths']['config']=str(Path(req['paths']['config_dir'])/'config.final.json')
    for key in c['cgroup']:
        expected=req['cgroup'][key]
        observed=obs['cgroup_verified'][key]
        if key!='envelope_path':
            assert expected==before['cgroup'][key]  # preserve every configured numeric cap
        if isinstance(expected,dict):
            assert all(observed[field]==value for field,value in expected.items())
            assert observed.get('memory_swap_max')==0
        else:
            assert observed==expected
        c['cgroup'][key]=copy.deepcopy(expected)  # observation carries extra diagnostic fields
    for key,q in obs['quota_mounts'].items():
        for field in ('mountpoint','max_bytes','max_inodes'):
            assert q[field]==req['quota_mounts'][key][field]
    assert set(obs['quota_mounts'])==set(c['quota_mounts'])
    c['quota_mounts']=copy.deepcopy(obs['quota_mounts'])
    c['bindings']['runner']=binding(args.runner)
    c['bindings']['staging']=binding(args.staging)
    c['old_home_stat_manifest']=binding(args.old_home_stats)
    load(args.old_home_stats)  # supplied metadata syntax only, no currentness inference
    assert c['limits']==before['limits'] and c['source_manifest']==before['source_manifest']
    assert c['bindings']['subject']==before['bindings']['subject']
    assert c['mode']=='tier1-and-compile' and c['source_authoring_only'] is False
    assert c['external_tier_consent']=={'tier1':True,'tier2':True}
    schema=load(SOURCE/'decision-records.schema.json')['$defs']
    provided={}
    for key,path in (('security',args.security_record),('nonsecurity',args.nonsecurity_record)):
        record=load(path)
        assert set(record)==set(schema[key]['required'])
        assert record['authority_granted_by_this_file_alone'] is False
        assert record['runner_sha256']==GUARD_SHA
        assert record['subject_manifest_sha256']==c['bindings']['subject']['sha256']
        assert record['source_manifest_sha256']==c['source_manifest']['sha256']
        provided[key]=path.read_bytes()  # Root-selected judgment bytes; never inferred here
    frozen=load(SOURCE/'SOURCE-FREEZE.json'); members=[]
    for row in frozen['files']:
        old=SOURCE/row['path']; actual=pin(old)
        assert actual['bytes']==row['bytes'] and actual['sha256']==row['sha256']
        selected=args.runner if row['path']=='bounded_sdk_compile.py' else (R/'395-cachedir-validation-test-owner-candidate/test_config_source.py' if row['path']=='test_config_source.py' else old)
        members.append({'basename':row['path'],'source':pin(selected)})
    focused_tree=ast.parse(args.focused_test.read_text())
    members.append({'basename':args.focused_test.name,'source':pin(args.focused_test)})
    ast.parse(args.runner.read_text())
    test_result_pin=pin(args.test_result)  # actual receipt provenance; Root judges PASS/scope
    output=args.output_dir.resolve(); output.mkdir(parents=True,exist_ok=False)
    def dump(name,value):
        path=output/name
        path.write_text(json.dumps(value,indent=2,ensure_ascii=False)+'\n')
        return path
    receipts={}
    for key in ('security','nonsecurity'):
        path=output/(key+'.source-receipt.proposed.json');path.write_bytes(provided[key]);receipts[key]=binding(path)
    projection=copy.deepcopy(c);projection.pop('review_receipts');projection['bindings'].pop('config_inputs')
    projection_path=dump('config-inputs.proposed.json',projection)
    c['bindings']['config_inputs']=binding(projection_path)
    root={'schema_version':1,'kind':'root_admission','decision':'ADMITTED_CONFIGURATION','admitted_mode':c['mode'],
          'runner_sha256':GUARD_SHA,'subject_manifest_sha256':c['bindings']['subject']['sha256'],
          'config_inputs_sha256':c['bindings']['config_inputs']['sha256'],
          'security_receipt_sha256':receipts['security']['sha256'],'nonsecurity_receipt_sha256':receipts['nonsecurity']['sha256'],
          'authority_granted_by_this_file_alone':False}
    receipts['root']=binding(dump('root.admission.proposed.json',root))
    for tier in (1,2):
        record={'schema_version':1,'kind':f'user_tier{tier}_grant','tier':tier,'decision':f'GRANTED_TIER{tier}',
                'root_receipt_sha256':receipts['root']['sha256'],'config_inputs_sha256':c['bindings']['config_inputs']['sha256'],
                'amendments_sha256':c['bindings']['amendments']['sha256'],'authority_granted_by_this_file_alone':False}
        if tier==2:record['tier1_receipt_sha256']=receipts['tier1']['sha256']
        receipts['tier'+str(tier)]=binding(dump(f'tier{tier}.automation-compatibility.proposed.json',record))
    c['review_receipts']=receipts
    final=dump('config.final.proposed.json',c)
    check=copy.deepcopy(c);check.pop('review_receipts');check['bindings'].pop('config_inputs');assert check==projection
    source_set=dump('SOURCE-SET-METADATA.proposed.json',{'status':'PROPOSED_NOT_ADOPTED','prior_freeze':pin(SOURCE/'SOURCE-FREEZE.json'),
        'members':members,'focused_AST_testdefs':[n.name for n in ast.walk(focused_tree) if isinstance(n,ast.FunctionDef) and n.name.startswith('test_')],
        'actual_test_result':test_result_pin,'prior_actual30_test_result':pin(R/'370-as-pid1-guard-minimal-tests-independent-review/actual-sdkfree30-01/RESULT.json'),'note':'Implementation-exposed tests; actual PASS/adoption determined by Root. SDK source manifest remains separate and unchanged.'})
    handoff=dump('ASSEMBLY-HANDOFF.proposed.json',{'status':'PROPOSED_NOT_ACTUAL_ADOPTION','request':pin(request),'operator':pin(OPERATOR),
        'actual_qobs':pin(args.qobs),'old_home_stats':pin(args.old_home_stats),'actual_test_result':test_result_pin,
        'config':pin(final),'projection':pin(projection_path),'five_records':receipts,'source_set_metadata':pin(source_set),
        'no_guard_import_test_provision_or_native_operation':True,'no_human_grant':'Existing tier kind strings serialize standing beta authorization.'})
    print(json.dumps({'handoff':pin(handoff),'config':pin(final)},indent=2))

if __name__=='__main__':main()
