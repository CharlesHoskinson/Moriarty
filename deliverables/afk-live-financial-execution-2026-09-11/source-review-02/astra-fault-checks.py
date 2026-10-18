import sys,json,sqlite3,tempfile,pathlib,hashlib,datetime,shutil,contextlib,io
W=pathlib.Path('/home/charl/Moriarty/.worktrees/afk-live-financial-execution')
O=pathlib.Path('/home/charl/Moriarty/deliverables/afk-live-financial-execution-2026-09-11/source-review-02')
sys.path.insert(0,str(W/'plugins/moriarty-dev/scripts'))
from moriarty_dev import store,cli,records
results=[]
def action(profile='finalized-financial-settlement',kind='verify',id='old'):
 return dict(id=id,requirement='SP05',capability='loan-executor',candidate='a'*64,kind=kind,evidenceProfile=profile,commandRef='commands.json#x',admissionRef='campaign:x')
with tempfile.TemporaryDirectory(prefix='astra-source02-',dir='/tmp') as tmp:
 root=pathlib.Path(tmp)
 for i,profile in enumerate([False,7,'','unknown-profile',' ',[],{},None,'local-runtime','syntax']):
  db=root/f'p{i}.sqlite3'; assert db.is_relative_to('/tmp')
  old=store.reserve(db,str(root),action(profile),{})
  hist=store.get_history(db,str(root),'SP05','loan-executor');other=store.has_other_primary(db,'different')
  try: store.reserve(db,str(root),action(id='new'),{}); admitted=True
  except store.ReservationConflictError: admitted=False
  expected=profile not in ['local-runtime','syntax']
  results.append({'case':'stored-profile','profile':profile,'primaryActive':hist['primaryActive'],'hasOtherPrimary':other,'secondLiveReservationAdmitted':admitted,'pass':hist['primaryActive']==expected and other==expected and admitted!=expected})
 # A historical start for another reservation is not evidence for an active row.
 db=root/'orphan-reservation.sqlite3'
 a=action('local-runtime');old=store.reserve(db,str(root),a,{})
 store.finish(db,old,{'exitCode':0})
 c=store.init_db(db)
 c.execute("INSERT INTO reservations (id,repository,requirement,capability,candidate,action_id,action_kind,pid,status,reserved_at) SELECT 'orphan-active',repository,requirement,capability,candidate,action_id,action_kind,pid,'active',reserved_at FROM reservations WHERE id=?",(old,));c.close()
 h=store.get_history(db,str(root),'SP05','loan-executor');o=store.has_other_primary(db,'different')
 try:store.reserve(db,str(root),action(id='new-live'),{});admitted=True
 except store.ReservationConflictError:admitted=False
 results.append({'case':'active-reservation-without-own-start-inherits-finished-nonlive-start','primaryActive':h['primaryActive'],'hasOtherPrimary':o,'secondLiveReservationAdmitted':admitted,'pass':h['primaryActive'] and o and not admitted})
 # Two simulated linked worktrees, with an authenticated common store fixture.
 common=root/'main'/'.git'; common.mkdir(parents=True)
 def worktree(name):
  gd=common/'worktrees'/name;gd.mkdir(parents=True);(gd/'commondir').write_text('../..')
  wt=root/name;wt.mkdir();(wt/'.git').write_text('gitdir: '+str(gd));(wt/'.moriarty-dev').mkdir()
  (wt/'.moriarty-dev'/'actions.json').write_text(json.dumps({'schema':'moriarty-dev.actions/1','actions':[action()]}));return wt
 oldwt=worktree('old-worktree');wt=worktree('current-worktree');db=store.get_db_path(wt);assert db.is_relative_to('/tmp')
 c=store.init_db(db);c.execute('CREATE TABLE store_identity(token TEXT NOT NULL)');c.execute("INSERT INTO store_identity VALUES ('fixture-anchor')");c.close()
 for fid in ['F1','F2']:store.record_event(db,str(oldwt),'SP05','loan-executor','a'*64,'old','defect_failure',{'findingId':fid})
 store.record_event(db,str(common),'SP05','loan-executor','a'*64,'seed','observation',{})
 reader=store.make_guarded_history_reader(db,live_lineage=cli.catalog_live_lineage(wt),identity_anchor='fixture-anchor')
 def observe(label):
  snap=records.load_snapshot(str(wt),'old',history_reader=reader)
  buf=io.StringIO()
  with contextlib.redirect_stdout(buf):cli.cmd_status(wt,db,reader,True)
  return {'case':label,'sameDefectFailures':snap['sameDefectFailures'],'missingEvidence':snap['missingEvidence'],'status':json.loads(buf.getvalue()),'retainedFailureRows':sqlite3.connect(db).execute("select count(*) from events where event_kind='defect_failure'").fetchone()[0],'pass':snap['sameDefectFailures'] is None or snap['sameDefectFailures']>=2}
 results.append(observe('existing-related-worktree'))
 (oldwt/'.git').unlink()
 results.append(observe('historical-worktree-git-metadata-removed'))
 results.append({'case':'production-no-anchor','unavailable':store.get_live_history(db,str(common),'SP05','loan-executor') is None,'pass':store.get_live_history(db,str(common),'SP05','loan-executor') is None})
 # Restoring metadata restores the previous count without changing any historical row.
 (oldwt/'.git').write_text('gitdir: '+str(common/'worktrees'/'old-worktree'))
 results.append(observe('restored-related-worktree'))
 # Approval preserves unnamed finding; explicit same-lineage names alone resolve.
 store.record_review(db,str(wt),{'author':'grok-4.6','reviewer':'gpt-6-astra','candidateHash':'a'*64,'scope':'source','verdict':'APPROVED','requirement':'SP05','capability':'loan-executor'})
 results.append(observe('approval-without-resolution-list'))
 c=sqlite3.connect(db);c.close()
live=pathlib.Path('/home/charl/Moriarty/.git/moriarty-dev/state.sqlite3')
c=sqlite3.connect('file:'+str(live)+'?mode=ro',uri=True);tables=[x[0] for x in c.execute("select name from sqlite_master where type='table'")];counts={t:c.execute('select count(*) from "'+t+'"').fetchone()[0] for t in tables};c.close()
hashes={str(p):{'exists':p.exists(),'size':p.stat().st_size if p.exists() else None,'sha256':hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None} for p in [live,pathlib.Path(str(live)+'-wal'),pathlib.Path(str(live)+'-shm')]}
before=json.loads((O/'astra-baseline.json').read_text());r={'observedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'results':results,'liveAfter':{'hashes':hashes,'counts':counts},'liveUnchanged':hashes==before['hashes'] and counts==before['counts'],'isolation':'All test DBs and Git metadata explicitly under /tmp; no budget lock or external runner used'}
(O/'astra-fault-results.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r,indent=2))
