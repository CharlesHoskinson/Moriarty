import hashlib, importlib, json, multiprocessing, os, sqlite3, subprocess, sys, tempfile, unittest
from pathlib import Path
STATE=Path(__file__).resolve().parent
BASE=STATE/'candidate'
PLUGIN=BASE/'plugins/moriarty-dev'
sys.dont_write_bytecode=True
sys.path[:0]=[str(PLUGIN/'scripts'),str(PLUGIN/'tests')]
import moriarty_dev.store as s
freeze=json.loads((STATE/'candidate-freeze.json').read_text())
def hashes_ok():
    return all(hashlib.sha256((BASE/p).read_bytes()).hexdigest()==h for p,h in freeze['files'].items())
assert hashes_ok()
checks=[]
def check(name, condition):
    assert condition, name
    checks.append(name)
def git(root,*args):
    return subprocess.run(['git','-C',str(root),*args],check=True,capture_output=True,text=True)
def contender(db, repo, barrier, queue, i):
    barrier.wait()
    try:
        rid=s.reserve(Path(db),repo,{'id':str(i),'kind':'implement' if i%2 else 'repair'}, {})
        queue.put(('reserved',rid,os.getpid()))
    except s.ReservationConflictError:
        queue.put(('conflict',None,os.getpid()))
    except Exception as e:
        queue.put(('error',repr(e),os.getpid()))
with tempfile.TemporaryDirectory(prefix='review-fixture-',dir=STATE) as temp:
    root=Path(temp)/'main';root.mkdir()
    git(root,'init');git(root,'config','user.email','review@example.invalid');git(root,'config','user.name','Review')
    (root/'README').write_text('fixture')
    git(root,'add','README');git(root,'commit','-m','fixture')
    wt=Path(temp)/'linked';git(root,'worktree','add','-b','review-linked',str(wt))
    db=s.get_db_path(root)
    check('git common directory database shared',db.resolve()==s.get_db_path(wt).resolve())
    s.init_db(db).close()
    for repo in (root,wt):s.record_event(db,str(repo),'req','cap','cand','seed','bootstrap',{})
    ctx=multiprocessing.get_context('fork');barrier=ctx.Barrier(12);queue=ctx.Queue()
    procs=[ctx.Process(target=contender,args=(str(db),str(root if i%2 else wt),barrier,queue,i)) for i in range(12)]
    for p in procs:p.start()
    outcomes=[queue.get(timeout=20) for _ in procs]
    for p in procs:p.join(20)
    check('12 concurrent processes exit normally',all(p.exitcode==0 for p in procs))
    winners=[x for x in outcomes if x[0]=='reserved']
    check('exactly one reservation across 12 linked worktree processes',len(winners)==1 and sum(x[0]=='conflict' for x in outcomes)==11)
    rid=winners[0][1]
    check('winning reservation parent actually exited',not s._is_pid_alive(winners[0][2]))
    def logical():
        with sqlite3.connect(db) as conn:return '\n'.join(conn.iterdump())
    before=logical();bytes_before=hashlib.sha256(db.read_bytes()).hexdigest()
    for repo in (root,wt):
        for _ in range(3):check('dead parent remains primaryActive in '+repo.name,s.get_history(db,str(repo),'req','cap')['primaryActive'] is True)
    check('history reads preserve SQL content',logical()==before)
    check('history reads preserve database bytes',hashlib.sha256(db.read_bytes()).hexdigest()==bytes_before)
    for repo in (root,wt):
        for kind in ('implement','repair'):
            try:s.reserve(db,str(repo),{'id':'retry','kind':kind},{'primaryActive':False})
            except s.ReservationConflictError:pass
            else:raise AssertionError('dead-parent retry admitted')
    check('denied stale-snapshot retries preserve SQL content',logical()==before)
    report=s.reserve(db,str(wt),{'kind':'report','id':'report'}, {})
    check('nonprimary report reservation does not release primary',s.get_history(db,str(root),'req','cap')['primaryActive'])
    s.finish(db,report,{'exitCode':0})
    s.finish(db,rid,{'exitCode':0})
    check('recorded normal finish clears primaryActive',s.get_history(db,str(wt),'req','cap')['primaryActive'] is False)
    successor=s.reserve(db,str(wt),{'kind':'repair','id':'successor'}, {})
    check('normal finish permits linked worktree successor',bool(successor))
    s.finish(db,successor,{'exitCode':0})

# Run only frozen regression tests in scope. Redirect source fixture constants and all temporary fixtures.
import test_records as tr
old=tr.SOURCE_ROOT;new=Path('/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok')
for key,value in list(vars(tr).items()):
    if isinstance(value,Path) and value.is_relative_to(old):setattr(tr,key,new/value.relative_to(old))
tempfile.tempdir=str(STATE)
import test_execution as te
names=['test_concurrent_reservation_denial','test_linked_worktrees_share_db','test_crashed_reservation_recovery','test_dead_parent_remains_active_in_read_only_history']
suite=unittest.TestSuite(te.ExecutionTestCase(name) for name in names)
with (STATE/'review-regressions.log').open('w') as stream:
    result=unittest.TextTestRunner(stream=stream,verbosity=2).run(suite)
check('four frozen focused regressions pass',result.wasSuccessful() and result.testsRun==4)
check('all 17 frozen source hashes preserved',hashes_ok())
output={'checks':checks,'concurrencyOutcomes':outcomes,'focusedTests':result.testsRun,'failures':len(result.failures),'errors':len(result.errors),'candidateHash':freeze['candidateHash']}
(STATE/'review-probe-results.json').write_text(json.dumps(output,indent=2)+'\n')
print(json.dumps(output,indent=2))
