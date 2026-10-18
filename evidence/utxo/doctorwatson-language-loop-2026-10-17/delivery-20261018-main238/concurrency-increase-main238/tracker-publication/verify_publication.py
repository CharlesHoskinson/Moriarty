import json,hashlib,re
from pathlib import Path
D=Path(__file__).parent;STAGE=D/"staged-publication";W=Path("/home/charl/Moriarty/.worktrees/utxo");P=Path("/home/charl/moriarty-c2-opus55-restored-20261017/github-language-progress-update-20261018")
def read(p):return json.loads(Path(p).read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
a=read(D/"PUBLIC-FILE-ALLOWLIST.json");destinations={x["destination"]:x for x in a}
assert len(destinations)==len(a)==472
assert sha(P/"final-main238-increment/work-items-before.json")=="304c7a1450caaffc96dc8ea91e68f81d74b2605a795645c0a09d283853de28be"
assert sha(W/"docs/utxo/doctorwatson/github/work-items.json")=="607983ac60fb09f83c2c95179b3a90c2777d268687e8f51d4853140348cd5efc"
candidate=P/"final-main238-increment/work-items-after-32.json";assert sha(candidate)=="607983ac60fb09f83c2c95179b3a90c2777d268687e8f51d4853140348cd5efc"
prohibited=re.compile(r"(^|/)(?:\.git|__pycache__|\.cache|node_modules|target|keys?|keystore|credentials?|auth)(?:/|$)|(?:^|[./_-])private(?:[./_-]|$)",re.I)
for row in a:
 src=Path(row["source"]);dst=Path(row["destination"]);assert src.is_file() and not src.is_symlink();assert not dst.is_absolute() and ".." not in dst.parts and not prohibited.search(str(dst))
 assert sha(src)==row["sha256"]==sha(STAGE/dst)==sha(W/dst) and (STAGE/dst).stat().st_size==(W/dst).stat().st_size==row["bytes"]
 assert not re.search(rb"-----BEGIN (?:RSA |EC |OPENSSH |ENCRYPTED )?PRIVATE KEY-----|github_pat_[A-Za-z0-9_]{20,}|ghp_[A-Za-z0-9]{30,}",(STAGE/dst).read_bytes())
for ref in read(D/"DIRECT-REFERENCE-RESOLUTION.json"):
 assert ref["resolvedAfterExplicitDependencyCopies"] and ref["publishedDestination"] in destinations and (STAGE/ref["publishedDestination"]).is_file() and (W/ref["publishedDestination"]).is_file()
for ref in read(D/"TRANSITIVE-EVIDENCE-REFERENCE-RESOLUTION.json"):
 assert ref["resolved"] and ref["publishedDestination"] in destinations and (STAGE/ref["publishedDestination"]).is_file() and (W/ref["publishedDestination"]).is_file()
assert not read(D/"DANGLING-REFERENCES.json") and not read(D/"HASH-MISMATCHES.json") and not read(D/"EXCLUSIONS.json")
for check in read(D/"ARTIFACT-HASH-CHECKS.json"):assert check["matches"] and sha(STAGE/check["target"])==check["declaredSha256"]
final=read(STAGE/"docs/utxo/doctorwatson/github/work-items.json");old=read(P/"final-main238-increment/work-items-before.json")
assert final==read(candidate) and final["items"][:20]==old["items"][:20] and final["counts"]=={"issues":15,"pullRequests":17,"total":32}
for before,after in zip(old["items"],final["items"]):assert before.get("acceptance")==after.get("acceptance")
assert final["originalInventoriesUnchanged"]==old["originalInventoriesUnchanged"] and sum(final["originalInventoriesUnchanged"].values())==148
assert final["items"][14]==old["items"][14] and [x["number"] for x in final["items"] if x["kind"]=="issue"]==[23]+list(range(76,90))
for packet in [P,P/"final-main238-increment"]:
 for row in read(packet/"OUTPUT-MANIFEST.json"):assert sha(packet/row["path"])==row["sha256"]
print(json.dumps({"status":"PASS","guard304cExact":True,"candidate607983Exact":True,"allowlistedPublicFiles":len(a),"physicalIsolatedStagingVerified":True,"actualSharedPublicationReadOnlyVerifiedAfterRootCopy":True,"actualSharedTracker607983Exact":True,"directOccurrencesResolved":len(read(D/"DIRECT-REFERENCE-RESOLUTION.json")),"uniqueDirectReferencesResolved":len({x["reference"] for x in read(D/"DIRECT-REFERENCE-RESOLUTION.json")}),"transitiveExplicitEvidenceReferencesResolved":len(read(D/"TRANSITIVE-EVIDENCE-REFERENCE-RESOLUTION.json")),"artifactHashChecksPassed":len(read(D/"ARTIFACT-HASH-CHECKS.json")),"danglingEvidenceReferences":0,"prohibitedPathOrAuthKeyMarkerPayloadsIncluded":0,"allFinal32StatusesAnd148Ears292Scenarios20Families13ChildrenUnchanged":True,"oldWorkerPacketsExact":True,"sharedWritesOrGitMutation":False,"providersProductNativeSdkExecution":False,"securityOrAcceptanceJudgment":False},indent=2))
