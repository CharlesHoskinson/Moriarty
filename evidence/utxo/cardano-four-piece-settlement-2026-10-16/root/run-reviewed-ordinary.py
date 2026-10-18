from pathlib import Path
import json,hashlib,datetime,subprocess,time,os,sys
R=Path(__file__).resolve().parent;A=json.loads((R/'AUTHORIZATION.json').read_text());O=Path(A['operatorManifest']['path']).parent;T=Path(A['target']);H=lambda z:hashlib.sha256(z).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
assert len(sys.argv)==2 and H((R/'AUTHORIZATION.json').read_bytes())==sys.argv[1]
assert H(Path(__file__).read_bytes())==A['rootRunner']['sha256']
assert H((R/'PRE-LAUNCH.json').read_bytes())==A['prelaunchSha256'] and not os.path.lexists(T)
assert A['attempts']==1 and A['retries']==0 and len(A['steps'])==15
rec={'format':'root-fourway-ordinary-operator-receipt/1','startedAt':now(),'authorizationSha256':H((R/'AUTHORIZATION.json').read_bytes()),'pid':os.getpid(),'status':'running','steps':[],'integrityChecks':[],'retries':0};begin=time.monotonic()
def save():
 (R/'OPERATOR-RECEIPT.json').write_text(json.dumps(rec,indent=2)+'\n')
def integrity(label):
 mp=Path(A['operatorManifest']['path']);z=mp.read_bytes();assert len(z)==A['operatorManifest']['bytes'] and H(z)==A['operatorManifest']['sha256'];rows=json.loads(z)['files'];assert len(rows)==34
 for e in rows:
  p=O/e['path'];v=p.read_bytes();assert not p.is_symlink() and len(v)==e['bytes'] and H(v)==e['sha256'],str(p)
 rec['integrityChecks'].append({'at':now(),'stage':label,'files':34,'manifestMatch':True,'match':True});save()
save()
try:
 for step in A['steps']:
  sid=step['id'];integrity('before-'+sid);p=Path(step['scriptPath']);z=p.read_bytes();assert len(z)==step['bytes'] and H(z)==step['sha256']
  outpath=R/(sid+'-stdout.txt');row={'id':sid,'argv':step['argv'],'startedAt':now(),'stdoutPath':str(outpath),'status':'running'};rec['steps'].append(row);save();tick=time.monotonic();print(json.dumps({'step':sid,'status':'starting','at':row['startedAt']}),flush=True)
  with outpath.open('xb') as out:
   child=subprocess.Popen(step['argv'],stdin=subprocess.DEVNULL,stdout=out,stderr=subprocess.STDOUT,cwd=R);row['pid']=child.pid;save();ec=child.wait()
  ep=T/'logs'/(sid+'.exit') if sid not in ['OPX0','OPX7f'] else T/(sid+'.exit')
  row.update(status='terminal',wrapperExit=ec,endedAt=now(),elapsedSeconds=time.monotonic()-tick,recordedExit=ep.read_text().strip() if ep.is_file() and not ep.is_symlink() else None);save();print(json.dumps(row),flush=True)
  if sid=='OPX0' and ec!=0:
   rec.update(status='stopped',reason='OPX0 nonzero; no subsequent step authorized');break
  if sid=='OPX1' and row['recordedExit']!='0':
   rec.update(status='stopped',reason='OPX1 recorded exit missing/nonzero; no subsequent step authorized');break
 else:rec['status']='terminal-all-15-invoked'
 integrity('after-final-invoked-step')
except Exception as e:
 rec.update(status='stopped-root-control-error',errorType=type(e).__name__);save();raise
finally:
 rec.update(endedAt=now(),elapsedSeconds=time.monotonic()-begin);save();print(json.dumps({'status':rec['status'],'stepsInvoked':len(rec['steps']),'elapsedSeconds':rec['elapsedSeconds']}),flush=True)
