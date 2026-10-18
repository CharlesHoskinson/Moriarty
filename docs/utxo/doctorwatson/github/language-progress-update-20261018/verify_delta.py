import json,hashlib,base64,difflib,re
from pathlib import Path
D=Path(__file__).parent
def read(p):return json.loads(Path(p).read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
b=read(D/"work-items-before.json");a=read(D/"work-items-after-32.json");s=read(D/"PROGRESS-UPDATE.json")
assert b["counts"]=={"issues":15,"pullRequests":14,"total":29} and a["counts"]=={"issues":15,"pullRequests":17,"total":32}
assert sha(D/"work-items-before.json")==s["beforeSha256"]=="304c7a1450caaffc96dc8ea91e68f81d74b2605a795645c0a09d283853de28be"
assert sha(D/"work-items-after-32.json")==s["afterSha256"]
assert base64.b64decode(read(D/"captures/canonical-tracker-contents.json")["content"])==(D/"work-items-before.json").read_bytes()
assert a["items"][:20]==b["items"][:20]
allowed={227:{"operationalObservationHistory","sourceResultObservationHistory","reviewObservationHistory","currentRemoteObservation","remoteStatus","currentSourceResultObservation","currentReviewObservation","currentRootDispositionObservation"},231:{"currentReviewObservation"}}
changed={}
for old,new in zip(b["items"],a["items"]):
 keys={k for k in set(old)|set(new) if old.get(k)!=new.get(k)}
 if keys:assert keys<=allowed[old["number"]];changed[old["number"]]=sorted(keys)
 assert old.get("acceptance")==new.get("acceptance")
assert set(changed)=={227,231}
for k,v in b.items():
 if k not in ["counts","items"]:assert a[k]==v
assert len(a["items"])==len({x["id"] for x in a["items"]})==32
assert [x["number"] for x in a["items"][29:]]==[236,237,238]
assert a["items"][14]==b["items"][14] and [x["number"] for x in a["items"] if x["kind"]=="issue"]==[23]+list(range(76,90))
assert sum(a["originalInventoriesUnchanged"].values())==148
for m in read(D/"CAPTURE-MANIFEST.json"):assert sha(D/m["path"])==m["sha256"]
for m in read(D/"RECEIPT-MANIFEST.json"):assert sha(D/m["copy"])==m["sha256"]==sha(m["source"])
for p in s["pullRequests"]:
 fs=p["fullChangedPathMembership"];assert len(fs)==p["changedPathCount"]
 assert any(c["sha"]==p["head"] and c["tree"]==p["tree"] for c in p["fullCommitMembership"])
 if p["number"] in [236,237,238]:assert p["state"]=="OPEN" and p["draft"] and p["base"]==s["main"]
p=next(x for x in s["pullRequests"] if x["number"]==227);merge=read(D/"captures/pr-227-merge-commit.json");root=read(D/"receipts/shared-check-main79-dual-role-terminal-review-20261018/ROOT-DISPOSITION.json")
assert p["state"]=="MERGED" and p["head"]==root["head"]=="09a442faa60192cde8f51f60986957ea5e4ff8da"
assert merge["sha"]==p["mergeCommit"]["oid"]=="8269b5b7ceeec5169c706aa31ea33f390d49c969"
assert merge["tree"]==p["tree"]==root["tree"]=="4b792e710bed78aa6a506f592dd283a52012b280"
for opinion in root["wholeOpinions"]:assert sha(D/"receipts/shared-check-main79-dual-role-terminal-review-20261018"/opinion["path"])==opinion["sha256"]
row=next(x for x in a["items"] if x["id"]=="GH-PR227");old=next(x for x in b["items"] if x["id"]=="GH-PR227");assert old["currentReviewObservation"] in row["reviewObservationHistory"] and old["currentRemoteObservation"] in row["operationalObservationHistory"] and old["currentSourceResultObservation"] in row["sourceResultObservationHistory"]
terminal=read(D/"receipts/source6-max-rebate-file-context-review-20261018/job/TERMINAL-EXPORT-RECEIPT.json");assert terminal["terminalExitCode"]==124 and terminal["decision"]=="NO_EXPORT" and not terminal["sourceReviewCorrectnessEstablished"]
assert len(read(D/"SOURCE-RESULT-PIN-CHECK.json"))==20 and all(x["matchesActualCheckSourceManifest"] for x in read(D/"SOURCE-RESULT-PIN-CHECK.json"))
for n,cycle,target,passed,filtered in [(236,"financial-source-check-cases-check-cycle-20261018","financial-source-check-tests",6,8),(237,"cli-profile-guidance-main8269-check-cycle-20261018","clap-help-test",1,7),(238,"language-header-diagnostic-main8269-check-cycle-20261018","language-header-test",1,18)]:
 d=D/"receipts"/cycle;p=next(x for x in s["pullRequests"] if x["number"]==n);r=read(d/"FINAL-RESULT.json");assert r["head"]==p["head"] and r["tree"]==p["tree"]
 result=read(d/target/"RESULT.json");assert result["exitCode"]==0 and result["stdoutSha256"]==sha(d/target/"STDOUT.log")
 out=(d/target/"STDOUT.log").read_text();assert f"{passed} passed; 0 failed; 0 ignored; 0 measured; {filtered} filtered out" in out
sherlock=read(D/"PR235-SEPARATE-CONTEXT.json");assert sherlock["state"]=="MERGED" and sherlock["baseRefName"]=="sherlock" and not sherlock["adoptedIntoUtxoTracker"]
assert not any(x["number"]==235 and x["kind"]=="pull-request" for x in a["items"])
expected=''.join(difflib.unified_diff((D/"work-items-before.json").read_text().splitlines(True),(D/"work-items-after-32.json").read_text().splitlines(True),fromfile="a/docs/utxo/doctorwatson/github/work-items.json",tofile="b/docs/utxo/doctorwatson/github/work-items.json"));assert expected==(D/"tracker-minimal-update.patch").read_text()
print(json.dumps({"status":"PASS","beforeCount":29,"afterCount":32,"original20AllFieldsExact":True,"all29OriginalAcceptanceFieldsExact":True,"originalSnapshotsAndIssue89ChildrenExact":True,"original148Ears292Scenarios20FamiliesUntouched":True,"changedExistingOperationalFields":changed,"newItems":[236,237,238],"merged227ExactReviewedTestedTree":True,"sourceAndResultWholeOpinionDigestsMatch":True,"prior227FailurePreserved":True,"review231124NoExportNoApproval":True,"newDraftSourceAndActualResultPinsMatch":20,"235SherlockContextSeparate":True,"apiCapturesVerified":len(read(D/"CAPTURE-MANIFEST.json")),"publicReceiptsVerified":len(read(D/"RECEIPT-MANIFEST.json")),"approvalByThisIntake":False,"runtimeOrProviderDispatchByThisLane":False},indent=2))
