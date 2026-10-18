import sys,json,sqlite3,tempfile,pathlib,hashlib,datetime
W=pathlib.Path('/home/charl/Moriarty/.worktrees/afk-live-financial-execution')
O=pathlib.Path('/home/charl/Moriarty/deliverables/afk-live-financial-execution-2026-09-11/source-review-01')
sys.path.insert(0,str(W/'plugins/moriarty-dev/scripts'))
from moriarty_dev import store,cli,records
live=pathlib.Path('/home/charl/Moriarty/.git/moriarty-dev/state.sqlite3')
def snap():
 d={p.name:{'exists':p.exists(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None} for p in (live,pathlib.Path(str(live)+'-wal'),pathlib.Path(str(live)+'-shm'))}
 c=sqlite3.connect('file:'+str(live)+'?mode=ro',uri=True);d['counts']={t:c.execute('select count(*) from '+t).fetchone()[0] for t in ['events','reservations','runner_claims','admin_intervals']};c.close();return d
before=snap(); results=[]
def action(profile,kind='verify',id='old'):
 return dict(id=id,requirement='SP05',capability='loan-executor',candidate='a'*64,kind=kind,evidenceProfile=profile,commandRef='commands.json#x',admissionRef='campaign:x')
with tempfile.TemporaryDirectory(prefix='astra-source01-',dir='/tmp') as tmp:
 root=pathlib.Path(tmp)
 for i,profile in enumerate([False,7,'','unknown-profile','local-runtime',None]):
  db=root/f'p{i}.sqlite3';old=store.reserve(db,str(root),action(profile),{})
  hist=store.get_history(db,str(root),'SP05','loan-executor');other=store.has_other_primary(db,'different')
  try: store.reserve(db,str(root),action('finalized-financial-settlement',id='new'),{}); admitted=True
  except store.ReservationConflictError: admitted=False
  results.append({'case':'legacy-profile','profile':profile,'primaryActive':hist['primaryActive'],'hasOtherPrimary':other,'secondLiveReservationAdmitted':admitted})
 # Simulate ordinary linked-worktree .git layout, entirely inside /tmp.
 common=root/'main'/'.git';gd=common/'worktrees'/'linked';gd.mkdir(parents=True);(gd/'commondir').write_text('../..')
 wt=root/'linked';wt.mkdir();(wt/'.git').write_text('gitdir: '+str(gd));(wt/'.moriarty-dev').mkdir()
 a=action('finalized-financial-settlement');(wt/'.moriarty-dev'/'actions.json').write_text(json.dumps({'schema':'moriarty-dev.actions/1','actions':[a]}))
 db=store.get_db_path(wt);c=store.init_db(db);c.execute('CREATE TABLE store_identity(token TEXT NOT NULL)');c.execute("INSERT INTO store_identity VALUES ('fixture-anchor')");c.close()
 for fid in ['F1','F2']:store.record_event(db,str(wt),'SP05','loan-executor','a'*64,'old','defect_failure',{'findingId':fid})
 reader=store.make_guarded_history_reader(db,live_lineage=cli.catalog_live_lineage(wt),identity_anchor='fixture-anchor')
 before_seed=records.load_snapshot(str(wt),'old',history_reader=reader)
 store.record_event(db,str(common),'SP05','loan-executor','a'*64,'seed','observation',{})
 after_seed=records.load_snapshot(str(wt),'old',history_reader=reader)
 results.append({'case':'linked-worktree-key-plus-common-key-seed','retainedDirectCount':store.get_history(db,str(wt),'SP05','loan-executor')['sameDefectFailures'],'snapshotBeforeSeed':{k:before_seed[k] for k in ['sameDefectFailures','missingEvidence']},'snapshotAfterSeed':{k:after_seed[k] for k in ['sameDefectFailures','missingEvidence']},'productionNoAnchor':store.get_live_history(db,str(common),'SP05','loan-executor')})
after=snap();r={'observedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'results':results,'liveBefore':before,'liveAfter':after,'liveUnchanged':before==after,'isolation':'All test DBs and Git metadata explicitly under /tmp; no budget lock or external runner used'}
(O/'astra-fault-results.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r,indent=2))
