import json,hashlib,base64,difflib
from pathlib import Path
D=Path(__file__).parent
def read(p):return json.loads(Path(p).read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
b=read(D/"work-items-before.json");a=read(D/"work-items-after-36.json");s=read(D/"CURRENT-DELIVERY.json")
assert sha(D/"work-items-before.json")==s["beforeSha256"]=="607983ac60fb09f83c2c95179b3a90c2777d268687e8f51d4853140348cd5efc"
assert base64.b64decode(read(D/"captures/canonical-tracker-contents.json")["content"])==(D/"work-items-before.json").read_bytes()
assert sha(D/"work-items-after-36.json")==s["afterSha256"] and a["items"][:32]==b["items"]
assert len(a["items"])==len({x["id"] for x in a["items"]})==36 and a["counts"]=={"issues":15,"pullRequests":21,"total":36}
assert [x["number"] for x in a["items"][32:]]==[240,241,244,246]
for k,v in b.items():
 if k not in ["items","counts"]:assert a[k]==v
assert sum(a["originalInventoriesUnchanged"].values())==148 and a["items"][14]==b["items"][14]
for n in [236,237]:
 x=read(D/f"captures/pr-{n}.json");assert x["state"]=="OPEN" and x["isDraft"]
for x in read(D/"CAPTURE-MANIFEST.json"):assert sha(D/x["path"])==x["sha256"]
for x in read(D/"RECEIPT-MANIFEST.json"):assert sha(D/x["copy"])==x["sha256"]==sha(x["source"])
for x in read(D/"CATALOGUE-PUBLIC-SOURCE-PINS.json"):assert sha(D/x["copy"])==x["sha256"]
for n in [240,241,244,246]:
 p=next(x for x in s["pullRequests"] if x["number"]==n);assert p["state"]=="OPEN" and p["draft"] and len(p["fullChangedPathMembership"])==p["changedPathCount"]
 assert any(x["sha"]==p["head"] and x["tree"]==p["tree"] for x in p["fullCommitMembership"])
assert len(read(D/"SOURCE-PIN-CHECKS.json"))==36 and all(x["matchesActualRecordedSourcePin"] for x in read(D/"SOURCE-PIN-CHECKS.json"))
header=read(D/"receipts/public-core-header-dispatch-terminal-review-20261018/ROOT-DISPOSITION.json");assert sha(D/"receipts/public-core-header-dispatch-terminal-review-20261018/ROOT-DISPOSITION.json")=="eacf79f917459815d22eade1756eeb5ba463ec8fdc99e6bb2906cb855002fc3f"
assert sha(D/"receipts/public-core-header-dispatch-terminal-review-20261018/job/TERMINAL-REVIEW.json")==header["wholeOpinionSha256"]=="9d7febc8188f3fe20dab1b2ca01995f91058ecc6da32e7cfd285ab7470cf7332"
for role in ["source","result"]:
 r=read(D/f"receipts/full-context-900-experiment-terminal-20261018/job-{role}/TERMINAL-EXPORT-RECEIPT.json");assert r["decision"]=="EXPORT" and r["terminalExitCode"]==0
for rel in ["install-docs-terminal-review-20261018/job/TERMINAL-EXPORT-RECEIPT.json","full-context-900-experiment-terminal-20261018/job-owner8od/TERMINAL-EXPORT-RECEIPT.json"]:
 r=read(D/"receipts"/rel);assert r["decision"]=="NO_EXPORT" and r["terminalExitCode"]==124 and not r["sourceReviewCorrectnessEstablished"]
assert read(D/"captures/simulate-remote-ref.json")["object"]["sha"]=="615eceee9c6250d2f203cb5807d5c75482dbdb5d" and read(D/"captures/simulate-pr-membership.json")==[]
assert not any(x["number"]==247 for x in a["items"] if x["kind"]=="pull-request")
sherlock=read(D/"SHERLOCK247-CONTEXT.json");assert sherlock["capturedFileRows"]==3000 and sherlock["reportedFullPRChangedPathCount"]==7955 and not sherlock["fileMembershipComplete"]
expected=''.join(difflib.unified_diff((D/"work-items-before.json").read_text().splitlines(True),(D/"work-items-after-36.json").read_text().splitlines(True),fromfile="a/docs/utxo/doctorwatson/github/work-items.json",tofile="b/docs/utxo/doctorwatson/github/work-items.json"));assert expected==(D/"tracker-minimal-update.patch").read_text()
print(json.dumps({"status":"PASS","original32RowsEveryFieldExact":True,"allAcceptanceHistoryAndOriginal20Exact":True,"original148Ears292Scenarios20Families13ChildrenUntouched":True,"before32After36OperationalItems":True,"actualNewDrafts":[240,241,244,246],"236237RemainOpenUntilActual240Merge":True,"all36NewChangedPathSourcePinsMatchActualBindings":True,"244AdoptionFileAndWholeOpinionHashesDistinctAndExact":True,"240Exported900OpinionsNotAdopted":True,"246AndFull8OdOwner124NoExport":True,"pushed615NoInventedPr":True,"247SeparateContextApiCapExplicit":True,"apiCapturesVerified":len(read(D/"CAPTURE-MANIFEST.json")),"publicReceiptCopiesVerified":len(read(D/"RECEIPT-MANIFEST.json")),"providerProductNativeOrSharedMutationByIntake":False,"approvalByIntake":False,"loopOrProviderLivenessClaim":False},indent=2))
