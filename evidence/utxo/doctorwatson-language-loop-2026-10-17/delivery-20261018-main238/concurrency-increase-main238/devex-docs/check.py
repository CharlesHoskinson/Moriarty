#!/usr/bin/env python3
"""Offline documentation/patch/source-pin checks. Executes no documented command."""
import argparse,difflib,hashlib,json,os,re,subprocess
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--repo',required=True);args=p.parse_args()
O=Path(__file__).resolve().parent;K=Path(args.repo);REV='71c22f8a477231be869786eb42f04e3b50d1b3a7'
def git(*a):return subprocess.check_output(['git','-C',str(K),*a])
def sha(b):return hashlib.sha256(b).hexdigest()
pins=json.loads((O/'SOURCE-PINS.json').read_text());assert pins['base']==REV
assert git('rev-parse',REV+'^{tree}').decode().strip()==pins['tree']
for row in pins['files']:
 b=git('show',REV+':'+row['path']);assert sha(b)==row['sha256'] and len(b)==row['bytes']
 assert git('rev-parse',REV+':'+row['path']).decode().strip()==row['gitBlob']
rec=json.loads((O/'RECONCILIATION.json').read_text())
for row in rec['history_inputs_untouched']:
 b=Path(row['path']).read_bytes();assert sha(b)==row['sha256'] and len(b)==row['bytes']
patch='';links=[];existing_unresolved=[];blocks=[]
env=dict(os.environ);env.pop('BASH_ENV',None);env.pop('ENV',None)
for path in rec['changed_paths']:
 b=(O/'before'/path).read_bytes();assert b==git('show',REV+':'+path)
 old=b.decode();new=(O/'after'/path).read_text()
 patch+=''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+path,tofile='b/'+path))
 old_links=set(re.findall(r'\]\(([^)]+)\)',old))
 for dest in re.findall(r'\]\(([^)]+)\)',new):
  if re.match(r'^[a-zA-Z][a-zA-Z0-9+.-]*:',dest):continue
  name,sep,fragment=dest.partition('#');target=os.path.normpath(str(Path(path).parent/name)) if name else path
  if target in rec['changed_paths']:target_body=(O/'after'/target).read_bytes()
  else:
   r=subprocess.run(['git','-C',str(K),'show',REV+':'+target],capture_output=True)
   if r.returncode:
    assert dest in old_links,(path,dest)
    existing_unresolved.append({'from':path,'target':dest,'reason':'Unchanged baseline link target not in exact Git tree.'});continue
   target_body=r.stdout
  if sep and dest not in old_links:
   headers=re.findall(r'^#+\s+(.+)$',target_body.decode(),re.M)
   slugs=[re.sub(r'[^\w\- ]','',h.lower()).replace(' ','-') for h in headers]
   assert fragment in slugs,(path,dest)
  links.append({'from':path,'target':dest,'new':dest not in old_links,'target_sha256':sha(target_body)})
 for n,block in enumerate(re.findall(r'```(?:sh|bash)\n(.*?)\n```',new,re.S),1):
  r=subprocess.run(['bash','--noprofile','--norc','-n'],input=block.encode(),capture_output=True,env=env)
  assert r.returncode==0,(path,n,r.stderr.decode());blocks.append({'path':path,'ordinal':n,'bytes':len(block.encode()),'sha256':sha(block.encode()),'syntax_exit':0})
assert patch.encode()==(O/'DEVEX-DOCS.patch').read_bytes()
r=subprocess.run(['git','apply','--check',str(O/'DEVEX-DOCS.patch')],cwd=O/'before',capture_output=True,text=True);assert r.returncode==0,r.stderr
expected=json.loads((O/'EXPECTATIONS.json').read_text());assert expected['base']==REV
pkg=json.loads(git('show',REV+':packages/moriarty-beta/package.json'));assert pkg['name']==expected['installed_beta_entrypoint']['package'];assert pkg['version']==expected['installed_beta_entrypoint']['version'];assert pkg['bin']['mori']=='dist/cli.js'
counter=json.loads(git('show',REV+':crates/fixtures/core-report-contract/counter.unmetered.stdout.json'))
assert expected['core1_simulate']['pointers']=={'/status':counter['status'],'/qualification':counter['qualification'],'/execution/post/state/count':counter['execution']['post']['state']['count']}
assert expected['core1_simulate']['effects']==counter['execution']['effects']
escrow=json.loads(git('show',REV+':examples/core/continuations/normal-oracle.json'))['escrow']
for key,slot in [('prefix_waiting','core2_start'),('true_resume_complete','core2_true_resume')]:
 row=expected[slot];fixture=escrow[key]
 assert row['status']==fixture['status'] and row['committed']==str(fixture['committed']) and row['residual']==fixture['residual'] and row['complete_segment_post']==fixture['segment']['post']
repay=json.loads(git('show',REV+':packages/moriarty-beta/examples/local/repay/mori.tests.json'))['cases'][0]['expect']
assert expected['node_beta_repay']['complete_existing_expectation']==repay
assert expected['rust_beta_expansion']['stdout_sha256']==sha(git('show',REV+':crates/mori/tests/fixtures/source6-expansion/invoice.source6'))
assert expected['installed_beta_entrypoint']['archive']==pkg['name'].lstrip('@').replace('/','-')+'-'+pkg['version']+'.tgz'
report={'kind':'OFFLINE_DOC_SOURCE_LINK_SHELL_PATCH_CHECK_ONLY','base':REV,'source_pins':len(pins['files']),'changed_docs':rec['changed_paths'],'history_inputs_unchanged':len(rec['history_inputs_untouched']),'exact_patch_regeneration':True,'patch_check_exit':r.returncode,'links':links,'unchanged_baseline_unresolved_links':existing_unresolved,'shell_blocks_syntax_only':blocks,'source_derived_expectations_sha256':sha((O/'EXPECTATIONS.json').read_bytes()),'subject_install_build_provider_native_sdk_execution':False,'startup_status_bookkeeping_disclosed_in':'RECONCILIATION.json'}
(O/'VALIDATION.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'source_pins':len(pins['files']),'changed_docs':len(rec['changed_paths']),'links_resolved':len(links),'new_links':sum(r['new'] for r in links),'unchanged_baseline_unresolved_links':len(existing_unresolved),'shell_blocks_syntax_only':len(blocks),'patch_check_exit':r.returncode},indent=2))
