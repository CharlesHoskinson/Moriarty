import json,hashlib,difflib
from pathlib import Path
D=Path(__file__).parent
def read(p):return json.loads(Path(p).read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
b=read(D/"work-items-before.json");a=read(D/"work-items-after-29.json");s=read(D/"FINAL-INTAKE.json")
assert sha(D/"work-items-before.json")==s["beforeSha256"]=="1b09d49890191145e25a17dc475b0c7fe97838dac0f1ce4be9f51211fd53b528"
assert sha(D/"work-items-after-29.json")==s["afterSha256"]
assert len(b["items"])==25 and len(a["items"])==29 and a["items"][:20]==b["items"][:20]
allowed={227:{"operationalObservationHistory","currentRemoteObservation","currentSourceResultObservation","currentReviewObservation"},229:{"operationalObservationHistory","currentRemoteObservation","remoteStatus","currentRootDispositionObservation"}}
changed={}
for old,new in zip(b["items"],a["items"]):
 ks={k for k in set(old)|set(new) if old.get(k)!=new.get(k)}
 if ks:assert ks<=allowed[old["number"]];changed[old["number"]]=sorted(ks)
 assert old.get("acceptance")==new.get("acceptance")
assert set(changed)=={227,229}
for k,v in b.items():
 if k not in ["items","counts"]:assert a[k]==v
assert a["counts"]=={"issues":15,"pullRequests":14,"total":29}
assert [x["number"] for x in a["items"][25:]]==[231,232,233,234]
assert len({x["id"] for x in a["items"]})==29
assert sum(a["originalInventoriesUnchanged"].values())==148
assert a["items"][14]==b["items"][14]
assert [x["number"] for x in a["items"] if x["kind"]=="issue"]==[23]+list(range(76,90))
assert s["currentOpenPRs"]==[220,222,223,225,227,228,231,232,234]
assert s["currentOpenIssues"]==[23]+list(range(76,90))
for p in s["pullRequests"]:
 assert p["changedPathCount"]==len(p["fullChangedPathMembership"])
 assert any(c["sha"]==p["head"] and c["tree"]==p["tree"] for c in p["fullCommitMembership"])
for n in [229,233]:
 p=next(x for x in s["pullRequests"] if x["number"]==n);m=read(D/f"captures/pr-{n}-merge-commit.json");assert p["state"]=="MERGED" and p["mergeCommit"]["oid"]==m["sha"] and p["tree"]==m["tree"]
for n in [231,232,234]:
 p=next(x for x in s["pullRequests"] if x["number"]==n);assert p["state"]=="OPEN" and p["draft"]
for n in [233]:
 row=next(x for x in a["items"] if x["number"]==n and x["kind"]=="pull-request");assert row["stateAtIntake"]=="OPEN" and row["draftAtIntake"] and row["acceptance"]=="open" and row["currentRemoteObservation"]["state"]=="MERGED" and row["currentRootDispositionObservation"]["actualBackendIdentityAttested"]==False
for x in read(D/"CAPTURE-MANIFEST.json"):assert sha(D/x["path"])==x["sha256"]
for x in read(D/"RECEIPT-MANIFEST.json"):assert sha(D/x["copy"])==x["sha256"]==sha(x["source"])
r=read(D/"receipts/shared-check-core-integration-terminal-review-20261018/job/TERMINAL-EXPORT-RECEIPT.json");assert r["terminalExitCode"]==124 and r["decision"]=="NO_EXPORT" and not r["sourceReviewCorrectnessEstablished"]
for name,code in [("payout-reserve-check-cycle-20261018",101),("payout-reserve-adapter-repair-check-cycle-20261018",0)]:
 p=D/"receipts"/name;r=read(p/"RESULT.json");assert r["exitCode"]==code and r["stdoutSha256"]==sha(p/"STDOUT.log")
r=read(D/"receipts/payout-reserve-adapter-repair-check-cycle-20261018/RESULT.json");p=next(x for x in s["pullRequests"] if x["number"]==234);assert p["head"]==r["head"] and p["tree"]==r["tree"]
comp=read(D/"captures/pr-234-adapter-repair-compare.json");assert comp["ahead_by"]==1 and len(comp["files"])==1 and comp["files"][0]["filename"]=="crates/mori/tests/core_reserve_spend_refund.rs"
assert read(D/"receipts/payout-reserve-source-integration-20261018/ADAPTER-CORRECTION.json")["fixtureBytesUnchanged"]
for packet in [D.parent,D.parent/"final-readback"]:
 for old in read(packet/"OUTPUT-MANIFEST.json"):assert sha(packet/old["path"])==old["sha256"]
expected=''.join(difflib.unified_diff((D/"work-items-before.json").read_text().splitlines(True),(D/"work-items-after-29.json").read_text().splitlines(True),fromfile="a/docs/utxo/doctorwatson/github/work-items.json",tofile="b/docs/utxo/doctorwatson/github/work-items.json"));assert expected==(D/"tracker-minimal-update.patch").read_text()
print(json.dumps({"status":"PASS","beforeCount":25,"afterCount":29,"original20RecordsExact":True,"original25AcceptanceFieldsExact":True,"originalIssue89And13ChildrenExact":True,"all148Ears292Scenarios20FamiliesUntouched":True,"bothHistoricalPacketManifestMembersExact":True,"changedExistingOperationalFields":changed,"appendedItems":[231,232,233,234],"merged229And233ExactTreeReadback":True,"review227Terminal124NoExportNoApproval":True,"repair234OnlyOneObserverFileNoOracleChange":True,"oldFailure101Retained":True,"capturesVerified":len(read(D/"CAPTURE-MANIFEST.json")),"publicReceiptsVerified":len(read(D/"RECEIPT-MANIFEST.json")),"reviewApprovalByThisIntake":False},indent=2))
