"""Mechanical execution of Opus's reviewed upgrade procedure; invoke only after reviews."""
from pathlib import Path
import datetime,hashlib,io,json,shutil,subprocess,tarfile,os
r=Path('/home/charl/Moriarty/.moriarty-dev/runtime/spec-wiki-plugin-reconcile-20261014')
w=Path('/home/charl/Moriarty/.worktrees/spec-wiki-plugin-reconcile-20261014')
task_source=Path('/home/charl/plugins/moriarty-dev')
task_original_cache=Path('/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.0+codex.20260912173932')
task_new_cache=Path('/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.1')
task_candidate='9bd7ccc1b5522b22cf633b46cdad080d24448f39'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=w,text=True).strip()==task_candidate
assert not task_source.is_symlink()
def hashes(root):
 return {p.relative_to(root).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in root.rglob('*') if p.is_file() and '__pycache__' not in p.parts}
expected={line.split('  ',1)[1].removeprefix('./'):line.split('  ',1)[0] for line in (r/'OPUS-PLUGIN-RECONCILIATION.sha256').read_text().splitlines()}
assert len(expected)==37
initial=json.loads((r/'plugin-pre-reconciliation-inventory.json').read_text())
expected_old={row['path']:row['mutable'] for row in initial['files'] if row['mutable'] is not None}
assert hashes(task_source)==expected_old,'Mutable source changed since inspection; preserve and reconcile before overwrite'
assert hashes(task_original_cache)==expected_old,'Original installed public source changed since inspection'
task_stamp=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
task_backup=Path('/home/charl/.local/share/moriarty-dev/backups')/('mutable-source-0.1.0+codex.20260912173932-'+task_stamp)
task_backup.parent.mkdir(parents=True,exist_ok=True);shutil.copytree(task_source,task_backup,symlinks=True)
task_stage=r/('reviewed-plugin-stage-'+task_stamp);task_stage.mkdir()
blob=subprocess.check_output(['git','archive',task_candidate,'plugins/moriarty-dev'],cwd=w)
with tarfile.open(fileobj=io.BytesIO(blob)) as archive:
 for member in archive.getmembers():
  if not member.isfile():continue
  rel=Path(member.name).relative_to('plugins/moriarty-dev');p=task_stage/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(archive.extractfile(member).read());p.chmod(member.mode)
assert hashes(task_stage)==expected
# The old35-file package is a subset of the reviewed37-file package. No source file deletion needed.
assert set(expected_old)<=set(expected)
shutil.copytree(task_stage,task_source,dirs_exist_ok=True)
assert hashes(task_source)==expected
task_env=os.environ.copy()
task_prior_roots={key:task_env.pop(key,None) for key in ['PLUGIN_ROOT','CLAUDE_PLUGIN_ROOT']}
task_env.pop('PYTHONPATH',None)
task_env['PYTHONDONTWRITEBYTECODE']='1'
receipt={'schema':'moriarty.plugin-refresh-receipt/1','source_candidate_commit':task_candidate,'plugin_version':'0.1.1','backup':str(task_backup),'source_manifest_files':len(expected),'commands':[],'native_actions':0,'host_interception':'not-established','original_cache_before':expected_old,'inherited_plugin_roots':task_prior_roots,'installer_environment':'PLUGIN_ROOT, CLAUDE_PLUGIN_ROOT and PYTHONPATH removed per Opus F3/A2; HOME and CODEX_HOME unchanged; isolated Python and bytecode disabled'}
def save():
 (r/'plugin-installation.json').write_text(json.dumps(receipt,indent=2)+'\n')
def run(args):
 p=subprocess.run(args,cwd=task_source,text=True,capture_output=True,timeout=180,env=task_env)
 receipt['commands'].append({'argv':args,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr});save()
 if p.returncode:raise RuntimeError('Reviewed upgrade command failed; inspect retained receipt')
 return p
receipt['bytecode_before']={str(root):[str(p.relative_to(root)) for p in root.rglob('*') if p.name=='__pycache__' or p.suffix=='.pyc'] for root in [task_source,task_new_cache]}
save()
prepare=run(['python3','-I','-B',str(task_source/'scripts/moriarty_dev/deployment.py'),'prepare','--plugin',str(task_source)])
assert hashes(task_source)==expected,'Prepare changed reviewed package bytes'
install=run(['python3','-I','-B',str(task_source/'scripts/moriarty_dev/deployment.py'),'install','--plugin',str(task_source),'--marketplace','personal'])
try:receipt['installer_result']=json.loads(install.stdout)
except ValueError:receipt['installer_result_parse']='not-json; see actual stdout'
receipt['installed_cache_exists']=task_new_cache.is_dir()
receipt['installed_hashes_match']=task_new_cache.is_dir() and hashes(task_new_cache)==expected
receipt['original_cache_preserved']=task_original_cache.is_dir() and hashes(task_original_cache)==expected_old
receipt['source_hashes_match']=hashes(task_source)==expected
receipt['installed_manifest_version']=json.loads((task_new_cache/'.codex-plugin/plugin.json').read_text())['version'] if task_new_cache.is_dir() else None
listing=run(['codex','plugin','list','--marketplace','personal','--json'])
try:receipt['plugin_listing']=json.loads(listing.stdout)
except ValueError:receipt['plugin_listing_parse']='not-json'
receipt['bytecode_after_install']={str(root):[str(p.relative_to(root)) for p in root.rglob('*') if p.name=='__pycache__' or p.suffix=='.pyc'] for root in [task_source,task_new_cache]}
receipt['old_cache_file_count']=len(expected_old);receipt['new_cache_file_count']=len(expected);save()
assert receipt['installed_hashes_match'] and receipt['original_cache_preserved'] and receipt['source_hashes_match']
assert receipt['installed_manifest_version']=='0.1.1'
print(json.dumps({k:receipt[k] for k in ['plugin_version','backup','installed_hashes_match','original_cache_preserved','source_hashes_match','host_interception','native_actions']},indent=2))

assert receipt.get('installer_result',{}).get('installedVerified') is True,'Installer verification flag is not true; inspect actual receipt before post-install tests'
assert receipt.get('installer_result',{}).get('restorationComplete') is True,'Installer cache restoration flag is not true'
