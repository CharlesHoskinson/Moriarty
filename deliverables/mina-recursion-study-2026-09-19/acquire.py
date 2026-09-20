from pathlib import Path
import subprocess,json,concurrent.futures,datetime
r=Path(__file__).parent;(r/'repos').mkdir(exist_ok=True)
repos={'mina':('MinaProtocol/mina',['src/lib/pickles','src/lib/pickles_types','src/lib/transaction_snark','src/lib/blockchain_snark','src/lib/zkapp_command','src/lib/mina_base','docs','rfcs']),'proof-systems':('o1-labs/proof-systems',['kimchi','poly-commitment','curves','book','utils']),'o1js':('o1-labs/o1js',['src/lib/proof-system','src/lib/mina','src/examples/zkprogram','src/examples/recursion','src/bindings','docs']),'snarky':('o1-labs/snarky',['src','snarky_backendless','snarky_backendless.ml','snarky_backendless.mli'])}
def run(item):
 name,(repo,dirs)=item;p=r/'repos'/name;url='https://github.com/'+repo+'.git'
 try:
  a=subprocess.run(['git','clone','--depth','1','--filter=blob:none','--sparse',url,str(p)],capture_output=True,text=True,timeout=300)
  (r/(name+'-clone.log')).write_text(a.stdout+a.stderr)
  if a.returncode: return {'name':name,'error':a.stderr}
  b=subprocess.run(['git','sparse-checkout','set','--skip-checks',*dirs],cwd=p,capture_output=True,text=True,timeout=300);(r/(name+'-sparse.log')).write_text(b.stdout+b.stderr)
  def git(*args):return subprocess.check_output(['git',*args],cwd=p,text=True).strip()
  (r/(name+'-tracked-files.txt')).write_text(git('ls-tree','-r','--name-only','HEAD')+'\n')
  row={'name':name,'url':url,'path':str(p),'commit':git('rev-parse','HEAD'),'branch':git('branch','--show-current'),'sparse':True,'sparse_status':b.returncode,'submodules':git('submodule','status'),'dirty':git('status','--porcelain'),'retrieved_at':datetime.datetime.now(datetime.timezone.utc).isoformat()}
  (r/(name+'-pin.json')).write_text(json.dumps(row,indent=2));print(json.dumps(row),flush=True);return row
 except Exception as e: return {'name':name,'error':str(e)}
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool: out=list(pool.map(run,repos.items()))
(r/'repos.json').write_text(json.dumps(out,indent=2))
