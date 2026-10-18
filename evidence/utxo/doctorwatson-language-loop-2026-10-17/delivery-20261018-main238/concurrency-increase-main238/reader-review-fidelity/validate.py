"""Mechanical fidelity only: public packet/Git reads, never subject execution."""
import argparse, collections, copy, hashlib, json, pathlib, subprocess
ap=argparse.ArgumentParser();ap.add_argument("--repo",required=True);args=ap.parse_args()
out=pathlib.Path(__file__).resolve().parent
job=pathlib.Path("/home/charl/moriarty-c2-opus55-restored-20261017/reader-source-location-keyed-terminal-review-20261018")
packet=job/"packet"
def read(path):return json.loads(path.read_text())
def digest(data):return hashlib.sha256(data).hexdigest()
def git(*parts):return subprocess.check_output(["git","-C",args.repo,*parts])
review=read(job/"job/TERMINAL-REVIEW.json"); inp=read(packet/"REVIEW-INPUT.json")
commit=review["candidateCommit"];base=inp["candidate"]["base"]
assert commit=="b3b4f9166832d3905492654edbc8f720cef8642b"
assert git("rev-parse",commit+"^{tree}").decode().strip()==inp["candidate"]["tree"]
changed=git("diff","--name-only",base,commit).decode().splitlines()
assert set(changed)==set(inp["scope"]["changedPaths"])==set(review["pathCoverage"])
assert review["reviewToken"]==inp["reviewToken"]
files=[]
for path,coverage in review["pathCoverage"].items():
    raw=git("show",commit+":"+path)
    assert raw==(packet/"source"/path).read_bytes()
    assert digest(raw)==coverage["sha256"] and coverage["path"]==path
    files.append({"path":path,"sha256":digest(raw),"gitBlob":git("rev-parse",commit+":"+path).decode().strip(),"bytes":len(raw),"splitlines":len(raw.splitlines()),"eofInclusiveSplit":len(raw.decode().split("\n")),"coverage":coverage["coverage"]})
quotes=[]
def walk(value,pointer=""):
    if isinstance(value,dict):
        if {"path","quote","lineStart","lineEnd"}<=value.keys():
            text=git("show",commit+":"+value["path"]).decode().splitlines()
            actual="\n".join(text[value["lineStart"]-1:value["lineEnd"]])
            assert actual==value["quote"],pointer
            quotes.append({"pointer":pointer,"path":value["path"],"lineStart":value["lineStart"],"lineEnd":value["lineEnd"],"quoteSha256":digest(actual.encode()),"exact":True})
        for key,child in value.items():walk(child,pointer+"/"+key)
    elif isinstance(value,list):
        for index,child in enumerate(value):walk(child,pointer+"/"+str(index))
walk(review)
manifest=read(packet/"MANIFEST.json")
for pin in manifest["files"]:
    raw=(packet/pin["path"]).read_bytes()
    assert digest(raw)==pin["sha256"] and len(raw)==pin["bytes"]
for pin in inp["sourceReview"]["fullCurrentSources"]:
    raw=git("show",commit+":"+pin["path"])
    assert raw==(packet/pin["physicalFile"]).read_bytes()
    assert digest(raw)==pin["sha256"] and len(raw)==pin["bytes"]
    assert git("rev-parse",commit+":"+pin["path"]).decode().strip()==pin["gitBlob"]
    if "text" in pin:assert pin["text"].encode()==raw
bounded=inp["sourceReview"]["boundedEvidenceFileReview"]
rowRepresentations=[]
for name in ["beforeRow","afterRow"]:
    row=bounded[name]
    text=git("show",row["candidate"]+":"+row["path"]).decode().splitlines()
    raw="\n".join(text[row["lineStart"]-1:row["lineEnd"]]).encode()
    assert raw==row["text"].encode()
    physical=(packet/("source-excerpts/EU-NAT-028-"+("before" if name=="beforeRow" else "after")+".txt")).read_bytes()
    assert physical==raw+b"\n" and digest(physical)==row["sha256"]
    rowRepresentations.append({"name":name,"inlineAndAnchorSha256":digest(raw),"physicalExcerptSha256":digest(physical),"physicalHasOneTrailingLF":True})
proposal=read(packet/"rebinding/PROPOSAL.json")
beforeRaw=git("show",proposal["input"]["gitBlob"])
assert digest(beforeRaw)==proposal["input"]["sha256"]
before=json.loads(beforeRaw);current=json.loads(git("show",commit+":"+proposal["input"]["path"]))
assert bounded["jsonPatch"]==proposal["operations"] and len(proposal["operations"])==15
working=copy.deepcopy(before)
def parent(data,pointer):
    bits=[b.replace("~1","/").replace("~0","~") for b in pointer.split("/")[1:]]
    for bit in bits[:-1]:data=data[int(bit)] if isinstance(data,list) else data[bit]
    return data,bits[-1]
for operation in proposal["operations"]:
    target,key=parent(working,operation["path"]);key=int(key) if isinstance(target,list) and key!="-" else key
    if operation["op"]=="test":assert target[key]==operation["value"]
    elif operation["op"]=="replace":target[key]=copy.deepcopy(operation["value"])
    elif operation["op"]=="add":
        if isinstance(target,list) and key=="-":target.append(copy.deepcopy(operation["value"]))
        else:target[key]=copy.deepcopy(operation["value"])
    else:raise AssertionError("Unexpected operation")
assert working==current
assert sum(a==b for a,b in zip(before["rows"],current["rows"]))==147
assert before["families"]==current["families"] and len(current["families"])==20
assert {k:v for k,v in before.items() if k!="rows"}=={k:v for k,v in current.items() if k!="rows"}
row=current["rows"][100];history=row["historicalReferenceBindings"];assert len(history)==4
for binding in history:
    a,key=parent(before["rows"][100],binding["rowPointer"]);key=int(key) if isinstance(a,list) else key
    assert a[key]==binding["oldRef"]
    a,key=parent(row,binding["rowPointer"]);key=int(key) if isinstance(a,list) else key
    assert a[key]==binding["newRef"]
    hist=git("show",binding["historicalSource"]["commit"]+":"+binding["historicalSource"]["path"])
    assert digest(hist)==binding["oldRef"]["sha256"]
    assert git("rev-parse",binding["historicalSource"]["commit"]+":"+binding["historicalSource"]["path"]).decode().strip()==binding["historicalSource"]["gitBlob"]
for a,b in zip(before["rows"],current["rows"]):
    for field in ["id","key","source","heading","scenarioNames","applicability","implementation","acceptance"]:assert a[field]==b[field]
    assert a["validation"]["status"]==b["validation"]["status"]
    for aa,bb in zip(a["scenarioRecords"],b["scenarioRecords"]):
        assert {k:v for k,v in aa.items() if k!="evidenceRefs"}=={k:v for k,v in bb.items() if k!="evidenceRefs"}
assert collections.Counter(r["acceptance"]["status"] for r in current["rows"])=={"open":148}
statuses=collections.Counter(s["status"] for r in current["rows"] for s in r["scenarioRecords"])
assert statuses=={"unrun":174,"unestablished":118}
receipts=[]
for entry in inp["resultReview"]["actualReceipts"]:
    stage=entry["stage"];folder=packet/"receipts"/stage;start=read(folder/"START.json");result=read(folder/"RESULT.json")
    assert result["exitCode"]==entry["exitCode"]
    assert digest((folder/"STDERR.log").read_bytes())==result["stderrSha256"]
    assert result["stdoutSha256"]==entry["stdoutPin"]["sha256"]
    if "summary" in entry:assert entry["summary"]==read(folder/"SUMMARY.json")
    for file in entry["files"]:
        raw=(packet/file["path"]).read_bytes();assert digest(raw)==file["sha256"]
        if "bytes" in file:assert len(raw)==file["bytes"]
        if "text" in file:assert raw==file["text"].encode()
    receipts.append({"stage":stage,"exitCode":result["exitCode"],"head":start.get("head"),"argv":start["argv"],"stdoutSha256Recorded":result["stdoutSha256"],"stderrSha256Recomputed":result["stderrSha256"],"stdoutBodyPresentInPacket":False})
suite=(packet/"receipts/full-reporting-suite/STDERR.log").read_text()
assert len([l for l in suite.splitlines() if l.startswith("test_") and l.endswith(" ... ok")])==63
assert "Ran 63 tests in 4.299s\n\nOK" in suite
assert "test_original_clause_location_resolves_from_another_directory" in suite
assert read(packet/"receipts/full-reporting-suite/START.json")["head"]==commit
assert read(packet/"receipts/metadata-after-rebinding/START.json")["head"]==commit
summary=read(packet/"receipts/metadata-after-rebinding/SUMMARY.json")
assert summary=={"integrity":{"status":"ok","issues":[]},"requirements":148,"scenarios":292,"families":20,"acceptanceStatuses":["open"],"closedGates":0}
failed=read(packet/"receipts/metadata/START.json");assert "--metadata-only" in failed["argv"]
assert read(packet/"receipts/metadata/RESULT.json")["exitCode"]==2
assert read(packet/"receipts/metadata-corrected-argv/RESULT.json")["exitCode"]==1
assert len(read(packet/"receipts/metadata-corrected-argv/SUMMARY.json")["integrity"]["issues"])==4
export=read(job/"job/TERMINAL-EXPORT-RECEIPT.json")
assert export["terminalExitCode"]==0 and export["terminalStatusLiteral"]=="SUCCESS" and export["decision"]=="EXPORT"
assert export["actualBackendIdentityAttested"] is False
assert review["sourceAxis"]["verdict"]=="approve" and review["resultAxis"]["verdict"]=="approve"
result={"format":"reader-review-mechanical-fidelity/1","result":"PASS_WITH_EXPLICIT_LIMITS","candidateCommit":commit,"candidateTree":inp["candidate"]["tree"],"reviewSha256":digest((job/"job/TERMINAL-REVIEW.json").read_bytes()),"reviewToken":review["reviewToken"],"keyedCoveragePaths":len(files),"sourceFiles":files,"quoteOccurrences":len(quotes),"quotes":quotes,"packetManifestPinsVerified":len(manifest["files"]),"rowExcerptRepresentations":rowRepresentations,"rebinding":{"operations":15,"originalRefsPreserved":4,"unchangedOtherRows":147,"bindings":history},"receiptChecks":receipts,"fullSuiteNamedPassingTests":63,"metadataSummary":summary,"allAcceptances":"148open","scenarioStatuses":dict(statuses),"familyRecords":20,"sourceAxisOpinion":"approve","resultAxisOpinion":"approve","terminalExportFields":{"status":export["terminalStatusLiteral"],"exit":export["terminalExitCode"],"decision":export["decision"],"requestedModel":export["requestedModel"],"actualBackendIdentityAttested":False},"minorDescriptiveIssue":"Coverage reasons use EOF-inclusive line counts128/463/98; splitlines gives127/462/97. Every actual quote and anchor matches exact source bytes. No scope omission follows from this counting convention.","limits":["Mechanical source/receipt fidelity, not independent model-read/backend identity or source provenance authentication.","Large metadata stdout bodies are not included in the public packet; their digests are checked for agreement across supplied pins/results, not recomputed from those absent bodies. Supplied summaries and complete public stderr/result files were inspected.","No private captures or external raw stdout paths were followed.","No command/subject/provider/native/product runtime was rerun; Git and stdlib JSON/hash comparisons only.","The canonical1.4MB file was parsed for equality/counts; no claim that reviewer model read all147unchanged rows.","Root alone adopts. No security/cryptographic/ledger/financial acceptance, no148parent closure, no source authenticity or backend attestation inferred."]}
(out/"RESULT.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps({"result":result["result"],"candidate":commit,"fourPathCoverage":len(files),"exactQuoteOccurrences":len(quotes),"packetPins":len(manifest["files"]),"jsonOperations":15,"historicalRefs":4,"passingTestsInReceipt":63,"metadataIntegrity":summary["integrity"]["status"],"sourceAuthenticityAttested":False,"backendIdentityAttested":False},indent=2))
