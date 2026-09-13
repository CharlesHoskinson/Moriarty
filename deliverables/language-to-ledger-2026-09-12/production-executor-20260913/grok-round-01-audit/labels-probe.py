import sys,pathlib,json,sqlite3
ROOT=pathlib.Path('/home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor');sys.path.insert(0,str(ROOT/'plugins/moriarty-dev/tests/loan_executor'))
from harness import LoanFixture
from moriarty_dev import records,store,policy
out=pathlib.Path('/tmp/moriarty-production-grok-r1-independent-probes');p=out/'labels';p.mkdir();fx=LoanFixture(root=p).build();fx.live['acceptance']['status']='complete-design-subset-only';fx.write_live_records();fx.write_runner(fx.campaigns)
s=records.load_snapshot(str(fx.root),fx.action['id'],history_reader=store.make_history_reader(fx.db_path));rows=[{'name':'live_historical_acceptance','root':str(p),'external':str(fx.external),'decision':policy.assess(s,fx.action),'candidateCurrent':s['candidateCurrent']}]
store.record_event(fx.db_path,str(p),fx.action['requirement'],fx.action['capability'],fx.action['candidate'],fx.action['id'],'defect_failure',{'findingId':'SOURCE-CANNOT-RESOLVE'})
r={'candidateHash':fx.action['candidate'],'reviewer':'independent GPT-6','scope':records.LIVE_SCOPE,'verdict':'APPROVED','findings':[],'resolvedFindings':['SOURCE-CANNOT-RESOLVE']}
store.record_review(fx.db_path,str(p),r,fx.action)
with sqlite3.connect(fx.db_path) as c:kind=c.execute('SELECT event_kind FROM events ORDER BY id DESC LIMIT 1').fetchone()[0]
rows.append({'name':'source_approval_as_result','eventKind':kind,'history':store.get_history(fx.db_path,str(p),fx.action['requirement'],fx.action['capability'])});(out/'labels-results.json').write_text(json.dumps(rows,indent=2));print(json.dumps(rows))
