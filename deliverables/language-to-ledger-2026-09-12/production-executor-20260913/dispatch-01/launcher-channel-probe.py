import hashlib,json,os,socket,struct,subprocess,tempfile,time
from pathlib import Path
launcher=Path('/home/charl/foreman/skills/foreman/runtime/dist/foreman-launch.js')
with tempfile.TemporaryDirectory(prefix='moriarty-channel-probe-') as d:
 p=Path(d);os.chmod(p,0o700);sock=socket.socket(socket.AF_UNIX);sock.bind(str(p/'channel'));os.chmod(p/'channel',0o600);sock.listen(1);sock.settimeout(15)
 child=p/'child.py';child.write_text('import os,socket,json\ns=socket.socket(socket.AF_UNIX)\ns.connect(os.environ["MORIARTY_INVOCATION_SOCKET"])\ns.sendall(json.dumps({"nonce":os.environ["MORIARTY_INVOCATION_NONCE"],"pid":os.getpid()}).encode()+b"\\n")\nassert s.recv(16)==b"ack"\ns.close()\n')
 env=dict(os.environ);env.update(MORIARTY_INVOCATION_SOCKET=str(p/'channel'),MORIARTY_INVOCATION_NONCE='isolated-probe-no-authority')
 node=subprocess.check_output(['which','node'],text=True).strip();argv=[node,str(launcher),'--timeout','15','--grace','2','--require-containment','strong','--','/usr/bin/python3',str(child)]
 proc=subprocess.Popen(argv,env=env,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
 connection,_=sock.accept();pid,uid,gid=struct.unpack('3i',connection.getsockopt(socket.SOL_SOCKET,socket.SO_PEERCRED,12));message=json.loads(connection.recv(1024));ancestry=[];cursor=pid
 for _ in range(32):
  stat=Path(f'/proc/{cursor}/stat').read_text();tail=stat[stat.rfind(')')+2:].split();ancestry.append({'pid':cursor,'ppid':int(tail[1]),'startTicks':int(tail[19])})
  if cursor==proc.pid:break
  cursor=int(tail[1])
 assert uid==os.getuid() and ancestry[-1]['pid']==proc.pid
 assert message['nonce']=='isolated-probe-no-authority'
 connection.sendall(b'ack');connection.close();sock.close();stdout,stderr=proc.communicate(timeout=20);assert proc.returncode==0
 receipt={'schema':'moriarty.isolated-launcher-channel-probe/1','pass':True,'launcher':str(launcher),'launcherSha256':hashlib.sha256(launcher.read_bytes()).hexdigest(),'argv':argv,'sameUid':uid==os.getuid(),'nonceInherited':True,'hostPeerPid':pid,'namespaceChildPid':message['pid'],'hostAncestryToLauncher':ancestry,'returnCode':proc.returncode,'stderr':stderr.decode(),'scope':'Real existing strong launcher AF_UNIX environment and host peer ancestry only. No accounting authentication, production runner or live reachability acceptance.'}
 Path('/tmp/moriarty-production-launcher-channel-probe.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt,indent=2))
