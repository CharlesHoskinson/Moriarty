from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys, tempfile

STATE = Path(__file__).resolve().parent
W = Path('/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok')
SCRIPTS = W/'plugins/moriarty-dev/scripts'
sys.path.insert(0, str(SCRIPTS))
from moriarty_dev.store import get_db_path, get_history, init_db

freeze = json.loads((STATE.parent/'agy-source-correction-03/candidate-freeze.json').read_text())
for rel, digest in freeze['files'].items():
    assert hashlib.sha256((W/rel).read_bytes()).hexdigest() == digest
observed = {}
with tempfile.TemporaryDirectory(prefix='moriarty-history-review-') as tmp:
    repo = Path(tmp)/'repo'
    repo.mkdir()
    subprocess.run(['git','init','-q',str(repo)],check=True)
    actions = json.loads((W/'.moriarty-dev/actions.json').read_text())
    (repo/'.moriarty-dev').mkdir()
    (repo/'.moriarty-dev/actions.json').write_text(json.dumps(actions))
    db = get_db_path(repo)
    receipt = repo/'wrong-review.json'
    receipt.write_text(json.dumps({
        'author':'agy-gemini-3.8-flash-high',
        'reviewer':'gpt-6-astra',
        'candidateHash':'f'*64,
        'scope':'plugin-source',
        'requirement':'SP01.1',
        'capability':'loan-review',
        'verdict':'APPROVED',
        'findings':[]
    }))
    env = dict(os.environ, PYTHONDONTWRITEBYTECODE='1', PYTHONPATH=str(SCRIPTS))
    result = subprocess.run([sys.executable,str(SCRIPTS/'moriarty_dev/cli.py'),
                             '--repo',str(repo),'review','--receipt',str(receipt),'--json'],
                            env=env,capture_output=True,text=True,timeout=10)
    observed['wrongCandidateReview'] = {
        'exitCode':result.returncode,'stdout':result.stdout,'stderr':result.stderr,
        'violation':result.returncode == 0,
        'expected':'Reject an unrelated candidate/scope before creating review history.'
    }
    corrupt = Path(tmp)/'corrupt.sqlite3'
    corrupt.write_text('not a SQLite database')
    try:
        history = get_history(corrupt,str(repo),'SP01.1','loan-review')
        observed['corruptHistory'] = {'returned':history,'violation':history is not None}
    except sqlite3.DatabaseError as exc:
        observed['corruptHistory'] = {'exception':type(exc).__name__,'message':str(exc),'violation':True}
    empty = Path(tmp)/'empty.sqlite3'
    init_db(empty).close()
    history = get_history(empty,str(repo),'SP01.1','loan-review')
    observed['unverifiedEmptyHistory'] = {
        'returned':history,'violation':isinstance(history,dict) and history.get('adminSeconds') == 0,
        'expected':'Absent verified history/timing cannot establish clean operational zeros.'
    }
for rel, digest in freeze['files'].items():
    assert hashlib.sha256((W/rel).read_bytes()).hexdigest() == digest
result = {'candidateHash':freeze['candidateHash'],'observations':observed,
          'candidateUnchanged':True,'scope':'Local public CLI/store only; no financial driver or network.'}
(STATE/'counterexamples.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
