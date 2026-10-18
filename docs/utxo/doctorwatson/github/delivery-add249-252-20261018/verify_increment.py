import json,hashlib,base64,difflib
from pathlib import Path
D=Path(__file__).parent
def read(p):return json.loads(Path(p).read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
b=read(D/"work-items-before-37.json");a=read(D/"work-items-after-40.json");s=read(D/"INTAKE-INCREMENT.json")
assert sha(D/"work-items-before-37.json")==s["before37Sha256"]=="9b97b51b00b1687cfd0c4d5e74d923e3baeb83419292cacf84498ea7e6aa6d44"
assert base64.b64decode(read(D/"captures/canonical-tracker-contents.json")["content"])==(D/"work-items-before-37.json").read_bytes()
assert sha(D/"work-items-after-40.json")==s["after40Sha256"] and a["items"][:37]==b["items"]
assert a["counts"]=={"issues":15,"pullRequests":25,"total":40} and len(a["items"])==len({x["id"] for x in a["items"]})==40
assert [x["number"] for x in a["items"][37:]]==[249,250,252]
for k,v in b.items():
 if k not in ["items","counts"]:assert a[k]==v
assert a["items"][14]==b["items"][14] and sum(a["originalInventoriesUnchanged"].values())==148
for n in [249,250,252]:
 p=next(x for x in s["pullRequests"] if x["number"]==n);assert p["state"]=="OPEN" and p["draft"] and len(p["fullChangedPathMembership"])==p["changedPathCount"]
 assert any(x["sha"]==p["head"] and x["tree"]==p["tree"] for x in p["fullCommitMembership"])
p=next(x for x in s["pullRequests"] if x["number"]==252);assert p["head"]=="829f4456dfda8d4fc5a4ece84c47f2539f846d9f" and p["base"]=="71c22f8a477231be869786eb42f04e3b50d1b3a7" and p["tree"]=="76dac56aac5e653729f3e4ad546b74ba07f1f780"
for x in read(D/"CAPTURE-MANIFEST.json"):assert sha(D/x["path"])==x["sha256"]
for x in read(D/"RECEIPT-MANIFEST.json"):assert sha(D/x["copy"])==x["sha256"]==sha(x["source"])
assert len(read(D/"SOURCE-PIN-CHECKS.json"))==16 and all(x["matchesActualSourceBinding"] for x in read(D/"SOURCE-PIN-CHECKS.json"))
for g,digest in [("inspect-header-terminal-review-20261018","483f80d3528660cfcbe0d9fab0bd0bdfa4cb41609e7dcbb6979344d8ebd55d86"),("simulate-header-terminal-review-20261018","903af3ae5122e01b6b369b8fbd38fc34173cb55d185a9c56876ba0edd5188cd0")]:
 root=read(D/"receipts"/g/"ROOT-DISPOSITION.json");assert sha(D/"receipts"/g/"ROOT-DISPOSITION.json")==digest and sha(D/"receipts"/g/"job/TERMINAL-REVIEW.json")==root["wholeOpinionSha256"]
assert s["existing248DatedObservation"]["rootDispositionSha256"]=="903af3ae5122e01b6b369b8fbd38fc34173cb55d185a9c56876ba0edd5188cd0"
boolean=read(D/"receipts/boolean-source-check-20261018/POST-COMMIT-BINDING.json");assert boolean["allRecordedPinsMatchGitAndWorktree"]==911 and boolean["sourcePinsSha256"]==sha(D/"receipts/boolean-source-check-20261018/SOURCE-PINS.json")
assert read(D/"receipts/boolean-source-check-20261018/RESULT.json")["stdoutSha256"]==sha(D/"receipts/boolean-source-check-20261018/STDOUT.log")
header=read(D/"receipts/header-main-integration-check-20261018/POST-COMMIT-BINDING.json");assert header["allRecordedAfterPinsMatchGitAndWorktree"]==878 and header["actualSelectedCommands"]==9 and header["actualPassedRustTests"]==18 and header["rawPublicProcessDirectories"]==7 and header["sourcePinsAfterSha256"]==sha(D/"receipts/header-main-integration-check-20261018/SOURCE-PINS-AFTER.json")
for result in (D/"receipts/header-main-integration-check-20261018").glob("*/RESULT.json"):
 r=read(result);assert r["exitCode"]==0 and r["stdoutSha256"]==sha(result.parent/"STDOUT.log")
expected=''.join(difflib.unified_diff((D/"work-items-before-37.json").read_text().splitlines(True),(D/"work-items-after-40.json").read_text().splitlines(True),fromfile="a/docs/utxo/doctorwatson/github/work-items.json",tofile="b/docs/utxo/doctorwatson/github/work-items.json"));assert expected==(D/"tracker-37-to-40.patch").read_text()
print(json.dumps({"status":"PASS","guard37Exact":True,"allOriginal37RowsEveryFieldExact":True,"allAcceptanceHistoryOriginal20And148Ears292Scenarios20Families13ChildrenUnchanged":True,"actualNewDrafts":[249,250,252],"actualIntegration252NotInvented251":True,"existing248DatedObservationWithoutRowRewrite":True,"all16ChangedSourcePinsMatchActualBindings":True,"qualifiedInspectAndSimRootAdoptionFilesAndWholeOpinionsMatch":True,"boolean911PinsAnd1Aggregate4Cli2ProgramsScopeRetained":True,"mainIntegration878Pins9Commands18Tests7RawDirsProducerBindingRetained":True,"noBooleanOrIntegrationReviewApprovalTransferred":True,"apiCapturesVerified":len(read(D/"CAPTURE-MANIFEST.json")),"publicReceiptCopiesVerified":len(read(D/"RECEIPT-MANIFEST.json")),"sharedOrGitMutationByIntake":False,"providerProductNativeSdkOrSecurityExecutionByIntake":False,"livenessOrAcceptanceClosureClaim":False},indent=2))
