import json,hashlib,difflib
from pathlib import Path
D=Path(__file__).parent
def read(p):return json.loads(Path(p).read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
b=read(D/"work-items-before.json");a=read(D/"work-items-after-28.json");s=read(D/"FINAL-INTAKE.json")
assert sha(D/"work-items-before.json")==s["beforeSha256"]=="1b09d49890191145e25a17dc475b0c7fe97838dac0f1ce4be9f51211fd53b528"
assert sha(D/"work-items-after-28.json")==s["afterSha256"]
assert len(b["items"])==25 and len(a["items"])==28
assert a["items"][:20]==b["items"][:20]
allowed={227:{"operationalObservationHistory","currentRemoteObservation","currentSourceResultObservation"},229:{"operationalObservationHistory","currentRemoteObservation","remoteStatus","currentRootDispositionObservation"}}
changed={}
for old,new in zip(b["items"],a["items"]):
 keys={k for k in set(old)|set(new) if old.get(k)!=new.get(k)}
 if keys: assert keys<=allowed[old["number"]];changed[old["number"]]=sorted(keys)
 assert old.get("acceptance")==new.get("acceptance")
assert set(changed)=={227,229}
for k,v in b.items():
 if k not in ["items","counts"]:assert a[k]==v
assert a["counts"]=={"issues":15,"pullRequests":13,"total":28}
assert [r["number"] for r in a["items"][25:]]==[231,232,233]
assert sum(a["originalInventoriesUnchanged"].values())==148
assert len({r["id"] for r in a["items"]})==28
assert [r["number"] for r in a["items"] if r["kind"]=="issue"]==[23]+list(range(76,90))
assert a["items"][14]==b["items"][14] #89 retains complete original child predicate
assert s["currentOpenIssues"]==[23]+list(range(76,90))
assert s["currentOpenPRs"]==[220,222,223,225,227,228,231,232,233]
for p in s["pullRequests"]:
 assert len(p["fullChangedPathMembership"])==p["changedPathCount"]
 assert any(c["sha"]==p["head"] and c["tree"]==p["tree"] for c in p["fullCommitMembership"])
 if p["number"] in [231,232,233]: assert p["state"]=="OPEN" and p["draft"]
p229=next(p for p in s["pullRequests"] if p["number"]==229)
assert p229["state"]=="MERGED" and p229["mergeCommit"]["oid"]=="049395532c43b17c54bdbc5a529535b521dc56df"
assert p229["tree"]==read(D/"captures/pr-229-merge-commit.json")["tree"]=="6ef1a779f5c87299bf340d684cea33082cb1bcb2"
for m in read(D/"CAPTURE-MANIFEST.json"):assert sha(D/m["path"])==m["sha256"]
for m in read(D/"RECEIPT-MANIFEST.json"):assert sha(D/m["copy"])==m["sha256"]==sha(m["source"])
for n,g in [(227,"shared-check-core-integration-check-cycle-20261018"),(232,"escrow-invoice-check-cycle-20261018"),(233,"custody-oracle-cli-path-check-cycle-20261018")]:
 p=next(z for z in s["pullRequests"] if z["number"]==n);r=read(D/"receipts"/g/("FINAL-RESULT.json" if n==227 else "RESULT.json"));assert r["head"]==p["head"] and r["tree"]==p["tree"]
for g,sub in [("shared-check-core-integration-check-cycle-20261018","public-tests"),("shared-check-core-integration-check-cycle-20261018","core-tests"),("escrow-invoice-check-cycle-20261018",""),("custody-oracle-cli-path-check-cycle-20261018","")]:
 p=D/"receipts"/g/sub;r=read(p/"RESULT.json");assert sha(p/"STDOUT.log")==r["stdoutSha256"]
for old in read(D.parent/"OUTPUT-MANIFEST.json"):assert sha(D.parent/old["path"])==old["sha256"]
expected=''.join(difflib.unified_diff((D/"work-items-before.json").read_text().splitlines(True),(D/"work-items-after-28.json").read_text().splitlines(True),fromfile="a/docs/utxo/doctorwatson/github/work-items.json",tofile="b/docs/utxo/doctorwatson/github/work-items.json"));assert expected==(D/"tracker-minimal-update.patch").read_text()
print(json.dumps({"status":"PASS","beforeCount":25,"afterCount":28,"original20RecordsExact":True,"original25AcceptanceFieldsExact":True,"issue89ExactAnd13ChildrenRetained":True,"historicalWaveFilesExact":True,"ear148Scenario292Family20InventoriesUntouched":True,"changedExistingOperationalFields":changed,"newItems":[231,232,233],"newPRsAllObservedOpenDraft":True,"capturesVerified":len(read(D/"CAPTURE-MANIFEST.json")),"publicReceiptsVerified":len(read(D/"RECEIPT-MANIFEST.json")),"reviewApprovalByThisIntake":False},indent=2))
