from pathlib import Path
import contextlib,io,json,sqlite3,sys
sys.dont_write_bytecode=True
W=Path('/home/charl/Moriarty/.worktrees/utxo-guarded-full-prefix-main-20261018')
D=Path(__file__).resolve().parent
sys.path.insert(0,str(W/'plugins/moriarty-dev/scripts'))
original=sqlite3.connect
calls=[]
def read_only(database,*args,**kwargs):
 p=Path(database).resolve();calls.append(str(p))
 kwargs['uri']=True
 return original(p.as_uri()+'?mode=ro',*args,**kwargs)
sqlite3.connect=read_only
from moriarty_dev import cli
cli.record_admin_interval=lambda *a,**kw:None
sys.argv=['moriarty-dev','--repo',str(W),'status','--json']
out=io.StringIO()
with contextlib.redirect_stdout(out):cli.main()
value=json.loads(out.getvalue())
(D/'STARTUP-STATUS.json').write_text(json.dumps(value,indent=2)+'\n')
(D/'STARTUP-READONLY-METHOD.json').write_text(json.dumps({'checkout':str(W),'sqliteUriMode':'ro','databaseConnections':calls,'adminIntervalRecording':'disabled in memory','bytecodeWrites':False,'noCampaignDispatch':True},indent=2)+'\n')
print(out.getvalue())
