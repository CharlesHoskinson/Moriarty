from pathlib import Path
import hashlib,json,os,runpy,stat
Q=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds')
H=Q/'root-manual-capture-data01'
helper=H/'capture-helper.FULL.py.txt'
assert hashlib.sha256(helper.read_bytes()).hexdigest()=='713119d9400f82d559967e4296313cec8cde7125dc91a74152500577ced00626'
h=runpy.run_path(str(helper),run_name='root_selected_api_only')
manifest=H/'CURRENT-4686-PIN-MANIFEST.json'
assert hashlib.sha256(manifest.read_bytes()).hexdigest()=='39d5de90f5f86e3c43fe55a6305d1ba4ca8102b194b50d6277119c2b71ef4bfd'
m=json.loads(manifest.read_text())
operation=H/'01-newNominalIdentityFrontend.COMMAND.selected.json'
assert hashlib.sha256(operation.read_bytes()).hexdigest()=='0081a5dc75c53e60825e37c91c29797c3da4589d842b8f0c02b1d0f9184ac14b'
op=json.loads(operation.read_text())
A=Q/'prospective-ordinary-QA'
assert not A.exists()
assert not (Q/'ordinary-TMP').exists()
assert not (Q/'proposed-itf').exists()
A.mkdir(mode=0o700)
phase=A/op['id'];phase.mkdir(mode=0o700)
for p in [Q/'ordinary-TMP',Q/'ordinary-TMP/HOME',Q/'ordinary-TMP/CACHE']:
 p.mkdir(mode=0o700)
h['save'](phase/'ROOT-SELECTION.json',{'status':'ROOT_SELECTED_R3_ORDERED_LIST_REPAIR_ONE_FRONTEND_ONLY','manualComposition':h['pin'](Path(__file__)),'operation':h['pin'](operation),'manifest':h['pin'](manifest),'sourceReview':h['pin'](Q/'/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/SOURCE-AND-ONE-FRONTEND-AUTHORITY.json'),'helper':h['pin'](helper),'followingOperationsSelected':False})
pre=h['static_check'](m['files'])
h['save'](phase/'PRE-FILES.json',pre)
assert pre['all_match']
assert sorted(p.name for p in (Q/'afterimages').iterdir())==m['sourceExactEntries']
assert all(stat.S_ISREG(p.lstat().st_mode) for p in (Q/'afterimages').iterdir())
symlinks=[]
for row in m['symlinks']:
 p=Path(row['path'])
 actual={'path':str(p),'target':os.readlink(p),'resolved':str(p.resolve(strict=True)),'isDirectory':p.resolve().is_dir()}
 assert actual==row
 symlinks.append(actual)
h['save'](phase/'PRE-SYMLINKS.json',symlinks)
result=h['capture'](op,phase)
h['save'](phase/'RESULT.json',result)
for p in phase.iterdir():
 if p.is_file():
  fd=os.open(p,os.O_RDONLY);os.fsync(fd);os.close(fd)
fd=os.open(phase,os.O_RDONLY);os.fsync(fd);os.close(fd)
post=h['static_check'](m['files'])
h['save'](phase/'POST-FILES.json',post)
assert post['all_match']
assert sorted(p.name for p in (Q/'afterimages').iterdir())==m['sourceExactEntries']
assert all(stat.S_ISREG(p.lstat().st_mode) for p in (Q/'afterimages').iterdir())
for row in m['symlinks']:
 p=Path(row['path'])
 assert os.readlink(p)==row['target'] and str(p.resolve(strict=True))==row['resolved'] and p.resolve().is_dir()==row['isDirectory']
h['save'](phase/'POST-SYMLINKS.json',symlinks)
accepted=result['actual_exit']==0 and result['controller_failure'] is None and len(result['streams'])==2 and all(s['full_stream_retained'] and s['bytes']==0 for s in result['streams'])
h['save'](phase/'ROOT-DISPOSITION.json',{'accepted':accepted,'scope':'new R3 list-API-repaired source11 frontend only; no tests selected','result':h['pin'](phase/'RESULT.json'),'staticRows':m['count'],'nextOperationSelected':False})
for p in phase.iterdir():
 if p.is_file():
  fd=os.open(p,os.O_RDONLY);os.fsync(fd);os.close(fd)
fd=os.open(phase,os.O_RDONLY);os.fsync(fd);os.close(fd)
print(json.dumps({'accepted':accepted,'result':result}))
assert accepted
