"""Offline fidelity over public packet and Git objects; no product imports."""
import argparse, hashlib, json, pathlib, re, subprocess
from jsonschema import Draft202012Validator
ap=argparse.ArgumentParser();ap.add_argument("--repo",required=True);args=ap.parse_args()
out=pathlib.Path(__file__).resolve().parent
root=pathlib.Path("/home/charl/moriarty-c2-opus55-restored-20261017/simulate-header-terminal-review-20261018");p=root/"packet"
def load(path):return json.loads(path.read_text())
def sha(raw):return hashlib.sha256(raw).hexdigest()
def git(*parts):return subprocess.check_output(["git","-C",args.repo,*parts])
checks=[]
def ck(ok,name,detail=None):checks.append({"name":name,"match":bool(ok),"detail":detail})
reviewFile=root/"job/TERMINAL-REVIEW.json";reviewRaw=reviewFile.read_bytes();review=load(reviewFile);candidate=load(p/"CANDIDATE.json");head=candidate["head"];parent=candidate["parent"]
ck(head=="615eceee9c6250d2f203cb5807d5c75482dbdb5d" and candidate["tree"]=="4a78114c06d5fbb858a4bf97b922c0446fabe978","candidate identity")
ck(git("rev-parse",head+"^{tree}").decode().strip()==candidate["tree"] and git("rev-parse",head+"^").decode().strip()==parent,"Git tree/parent")
ck(set(git("diff","--name-only",parent,head).decode().splitlines())=={x["path"]for x in candidate["changedPaths"]},"exact two changed paths")
schema=load(p/"REVIEW-RESPONSE.schema.json");Draft202012Validator.check_schema(schema);schemaErrors=[{"path":list(e.path),"message":e.message}for e in Draft202012Validator(schema).iter_errors(review)];ck(not schemaErrors,"real runtime schema",schemaErrors)
mandatory=load(p/"MANDATORY-INPUTS.json");paths=[v["path"]for v in mandatory["files"]];enum=schema["properties"]["mandatoryInputCoverage"]["properties"]["filesReadCompletely"]["items"]["enum"]
ck(len(paths)==len(set(paths))==156 and set(paths)==set(enum)==set(review["mandatoryInputCoverage"]["filesReadCompletely"]),"exact 156 mandatory paths")
ck(review["mandatoryInputCoverage"]["unreadMandatoryFiles"]==[] and review["actualReadLimits"]==[],"declared no unread limits; not independently attested")
manifestCounts=[];physicalHashes=set()
for name in ["MANIFEST.json","PACKET-MANIFEST.json","MANDATORY-INPUTS.json"]:
    table=load(p/name)["files"];bad=[]
    for pin in table:
        raw=(p/pin["path"]).read_bytes();physicalHashes.add(sha(raw))
        if len(raw)!=pin["bytes"]or sha(raw)!=pin["sha256"]:bad.append(pin["path"])
    ck(not bad,"manifest "+name,bad);manifestCounts.append({"file":name,"pins":len(table)})
inline=(p/"FULL-INPUT.md").read_bytes();missing=[]
for pin in mandatory["files"]:
    raw=(p/pin["path"]).read_bytes()
    if pin["path"]=="REVIEW-REQUEST.md":
        if not inline.startswith(raw):missing.append(pin["path"])
        continue
    marker=("BEGIN FILE "+pin["path"]+" | SHA256 "+pin["sha256"]+" | BYTES "+str(pin["bytes"])+"\n").encode();at=inline.find(marker)
    if at<0 or inline[at+len(marker):at+len(marker)+len(raw)]!=raw:missing.append(pin["path"])
ck(not missing,"all 156 mandatory bodies literal in FULL-INPUT",missing)
ck(len(inline)==329878 and sha(inline)=="2df5f24bdbd3d90c360866ff6f6e326fa8cca8c7e6b5241796c9f4352a538551","whole inline bytes/hash")
sourceQuotes=[];sourceHashes=[]
for coverage in review["sourceAssessment"]["changedPathCoverage"]:
    path=coverage["path"];raw=(p/"source"/path).read_bytes();ck(raw==git("show",head+":"+path)and sha(raw)==coverage["sha256"],"candidate complete source "+path);sourceHashes.append({"path":path,"sha256":sha(raw),"gitBlob":git("rev-parse",head+":"+path).decode().strip()})
    lines=raw.decode().splitlines()
    for quote in coverage["quoteAnchors"]:
        exact=lines[quote["originalLine"]-1]==quote["exactQuote"];ck(exact,"original source quote "+path+":"+str(quote["originalLine"]));sourceQuotes.append({"path":path,"line":quote["originalLine"],"quoteSha256":sha(quote["exactQuote"].encode()),"exact":exact})
unitResults=[]
for unit in load(p/"SOURCE-UNIT-MAP.json"):
    origin=unit["origin"];raw=(p/unit["packetPath"]).read_bytes();ck(sha(raw)==unit["unitSha256"],"unit literal pin "+unit["packetPath"])
    if "/utxo-positive-wave32-20261018/"in origin:full=git("show",head+":"+origin.split("/utxo-positive-wave32-20261018/",1)[1])
    else:full=(p/"original-author/renderer-source"/origin.split("/renderer-source/",1)[1]).read_bytes()
    selected=b"".join(full.splitlines(keepends=True)[unit["startLine"]-1:unit["endLine"]]);exact=sha(full)==unit["originSha256"]and selected==raw;ck(exact,"unit original coordinates "+unit["packetPath"]);unitResults.append({"path":unit["packetPath"],"startLine":unit["startLine"],"endLine":unit["endLine"],"exact":exact})
def batch(ref,paths):
    data=subprocess.run(["git","-C",args.repo,"cat-file","--batch"],input=("\n".join(ref+":"+path for path in paths)+"\n").encode(),stdout=subprocess.PIPE,check=True).stdout;cursor=0;found={}
    for path in paths:
        eol=data.index(b"\n",cursor);header=data[cursor:eol].split();cursor=eol+1
        if header[-1]==b"missing":found[path]=None;continue
        size=int(header[2]);found[path]=data[cursor:cursor+size];cursor+=size+1
    return found
before=load(p/"actual/SOURCE-PINS-BEFORE.json");after=load(p/"actual/SOURCE-PINS-AFTER.json");bmap={v["path"]:v["sha256"]for v in before["files"]};amap={v["path"]:v["sha256"]for v in after["files"]};testPath="crates/mori/tests/simulate_header_dispatch.rs";production="crates/mori/src/commands/simulate.rs"
ck(len(bmap)==len(amap)==len(before["files"])==len(after["files"])==902 and set(bmap)==set(amap),"902 unique before/after pins")
current=batch(head,list(amap));old=batch(parent,[path for path in bmap if path!=testPath]);ck(all(current[path]is not None and sha(current[path])==digest for path,digest in amap.items()),"902 after pins equal candidate Git");ck(all(old[path]is not None and sha(old[path])==digest for path,digest in bmap.items()if path!=testPath),"901 tracked before pins equal parent Git")
ck(git("ls-tree",parent,"--",testPath)==b""and bmap[testPath]==amap[testPath],"new frozen aggregate test present before RED and unchanged")
delta=[{"path":path,"beforeSha256":bmap[path],"afterSha256":amap[path]}for path in bmap if bmap[path]!=amap[path]];ck(len(delta)==1 and delta[0]["path"]==production,"sole production mutation",delta)
ck((p/"actual/SIMULATE-BEFORE.rs").read_bytes()==old[production],"complete simulate-before source")
binding=load(p/"actual/POST-COMMIT-BINDING.json");ck(binding["head"]==head and binding["tree"]==candidate["tree"]and binding["parent"]==parent and binding["allRecordedAfterPinsMatchGitAndWorktree"]==902 and binding["sourcePinsAfterSha256"]==sha((p/"actual/SOURCE-PINS-AFTER.json").read_bytes()),"postcommit binding and source manifest hash")
# Literal expectation and captured-process comparisons; no product evaluator.
freeze=load(p/"actual/EXPECTED-CASE-SPEC.json");frozenPins=[]
for line in(p/"actual/EXPECTED-FREEZE.sha256").read_text().splitlines():
    digest,path=line.split(None,1);path=path.strip();target=p/"actual/EXPECTED-CASE-SPEC.json"if path=="EXPECTED-CASE-SPEC.json"else p/"original-author"/path;ck(sha(target.read_bytes())==digest,"frozen expectation pin "+path);frozenPins.append({"path":path,"sha256":digest})
header=(p/"source"/testPath).read_text();literals=re.findall(r'r###"(.*?)"###',header,re.S);ck(len(literals)==12,"12 aggregate raw string literals")
for i,case in enumerate(freeze["header_cases"]):ck(literals[3*i]==case["source"]and json.loads(literals[3*i+1])==case["producer_error_json"]and literals[3*i+2]==case["public_simulate_stderr"],"frozen test literals "+case["name"])
empty=freeze["empty_core2_control"];ck(literals[9]==empty["source"]and json.loads(literals[10])==empty["scenario"]and json.loads(literals[11])==empty["execution"],"empty Core2 literal projection")
ck(header.index('collect::<Vec<_>>()')<header.index('let empty = project.run(')<header.index('for (case, error, output) in captured'),"all four calls captured before assertion loop")
ck(header.splitlines()[199].strip()=="assert_eq!("and 'case.expected_stderr.as_bytes()'in"\n".join(header.splitlines()[199:205]),"RED first human assertion at original line 200")
ck('assert_eq!(report["status"], "complete");'in header and 'report["segment"]["execution"]'in header,"empty status/segment.execution assertions only")
comparison=load(p/"actual/FROZEN-OBSERVATION-COMPARISON.json");captureResults=[];captureHashes=set();narrativeChecks=[]
for coverage in review["actualResultAssessment"]["capturedProcessCoverage"]:
    phase=coverage["phase"];name=coverage["case"];folder=p/"actual"/phase/"public-cli"/name;body=coverage["scopeAssessment"];process=load(folder/"process.json");ck(process["args"]==["--color","never","simulate","header.mori","--action","run","--scenario","scenario.json"]and(folder/"scenario.json").read_bytes()==literals[10].encode(),"complete process argv/scenario "+phase+"/"+name);row=next(v for v in comparison["rows"]if v["phase"]==phase and v["case"]==name);filePins=[]
    for f in folder.iterdir():
        if f.is_file():filePins.append({"file":f.name,"bytes":f.stat().st_size,"sha256":sha(f.read_bytes())});captureHashes.add(sha(f.read_bytes()))
    for filename in["header.mori","scenario.json","producer-error.json","stdout.bin","stderr.txt"]:
        file=folder/filename
        if not file.exists():continue
        # Narrative byte and digest claims, including filename followed by whitespace or a parenthesis.
        match=re.search(re.escape(filename)+r"(?: \(| )(\d+)B(?: SHA256 ([0-9a-f]{64}))?",body)
        if match:ck(int(match[1])==file.stat().st_size and(not match[2]or match[2]==sha(file.read_bytes())),"narrative raw bytes/hash "+phase+"/"+name+"/"+filename)
    for quote in re.findall(r'`([^`]+)`',body):
        if quote.startswith('language ') or quote.startswith('// café'):
            decoded=re.sub(r"\\{1,2}n","\n",quote);exact=decoded==(folder/"header.mori").read_text();ck(exact,"narrative source quote "+phase+"/"+name);narrativeChecks.append({"phase":phase,"case":name,"representation":"source literal with explicit newline escapes when present","exactAfterDeclaredEscapeDecoding":exact})
    if name!="empty-core2":
        expected=next(v for v in freeze["header_cases"]if v["name"]==name);producer=load(folder/"producer-error.json");stderr=(folder/"stderr.txt").read_bytes();stdout=(folder/"stdout.bin").read_bytes();repair=stderr==expected["public_simulate_stderr"].encode()
        ck((folder/"header.mori").read_text()==expected["source"]and producer==expected["producer_error_json"]and process["status"]==1 and stdout==b"","complete header raw inputs/DTO/exit "+phase+"/"+name)
        ck(repair==(phase=="green")and row["producerErrorExact"]and row["stdoutEmpty"]and row["exitCode"]==1 and row["stderrMatchesFrozenRepair"]==repair and row["stderrSha256"]==sha(stderr),"header repair comparison "+phase+"/"+name)
        ck(coverage["producerErrorDtoRead"]is True,"DTO coverage declaration "+phase+"/"+name)
        for quote in re.findall(r'`([^`]+)`',body):
            if quote.startswith("  ×"):
                decoded=re.sub(r"\\{1,2}n","\n",quote);exact=decoded.encode()==stderr;ck(exact,"explicit escaped narrative stderr "+phase+"/"+name);narrativeChecks.append({"phase":phase,"case":name,"representation":"explicit display newline escapes","rawSha256":sha(stderr),"exactAfterDeclaredEscapeDecoding":exact})
    else:
        report=json.loads((folder/"stdout.bin").read_bytes());ck(process["status"]==0 and(folder/"stderr.txt").read_bytes()==b""and report["status"]==empty["report_status"]and report["segment"]["execution"]==empty["execution"],"empty report selected projection "+phase)
        ck(row["emptyExecutionProjectionExact"]and row["otherFullRawReportFieldsCapturedButUnasserted"]and row["rustControlAssertionsReached"]==(phase=="green")and coverage["producerErrorDtoRead"]is None,"empty projection scope retained "+phase)
        ck(set(report)=={"attempted","authentication","committed","consumption","qualification","residual","status","segment"}and len((folder/"stdout.bin").read_bytes())==673,"captured empty wrapper field inventory "+phase)
    captureResults.append({"phase":phase,"case":name,"status":process["status"],"files":filePins})
ck(len(captureResults)==8,"exact eight raw captured process directories")
logQuotes=[];phaseResults=[];final=load(p/"actual/FINAL-RESULT.json")
for coverage in review["actualResultAssessment"]["phaseCoverage"]:
    name=coverage["phase"];folder=p/"actual"/name;result=load(folder/"RESULT.json");start=load(folder/"START.json");stdout=(folder/"STDOUT.log").read_bytes();stderr=(folder/"STDERR.log").read_bytes();expectedExit=101 if name=="red"else 0
    ck(result["exitCode"]==expectedExit and result["ownedSessionRemaining"]==[],"phase exit/session "+name);ck(sha(stdout)==result["stdoutSha256"]and sha(stderr)==result["stderrSha256"],"complete phase stdout/stderr digest "+name);ck(start["head"]==parent,"historical base-head retained "+name)
    original=next(v for v in final["results"]if v["name"]==name);ck(original==result,"FINAL exact result "+name)
    for quote in coverage["quoteAnchors"]:
        raw=(p/quote["packetPath"]).read_bytes();value=quote["exactQuote"].encode();exact=value in raw;ck(exact,"real complete log quote "+quote["packetPath"]);logQuotes.append({"phase":name,"path":quote["packetPath"],"line":raw[:raw.index(value)].count(b"\n")+1 if exact else None,"exactQuote":quote["exactQuote"],"quoteSha256":sha(value),"exact":exact})
    match=re.search(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;',stdout.decode());ck(bool(match)and int(match[1])==(0 if name=="red"else 1)and int(match[2])==(1 if name=="red"else 0),"phase aggregate test count "+name)
    phaseResults.append({"phase":name,"exit":result["exitCode"],"stdoutSha256":sha(stdout),"stderrSha256":sha(stderr),"baseHead":start["head"],"counts":list(map(int,match.groups()))if match else None})
ck(final["allFourCommandsCompleted"]and final["existsAfter"]is False,"four phase completion/cleanup receipts")
redLog=(p/"actual/red/STDOUT.log").read_text();ck('simulate_header_dispatch.rs:200:9:'in redLog and 'unknown: complete human stream'in redLog,"actual RED panic/reachability")
oracle=load(p/"positive-custody/examples/core/continuations/normal-oracle.json");projection=load(p/"positive-oracle-used-fields.json");ck(projection=={k:v for k,v in oracle.items()if k not in["reserve","compatibility_v1"]},"complete selected oracle projection preserved")
core1=(p/"units/positive-core1-complete-helpers-and-test.rs").read_text();core2=(p/"units/positive-core2-complete-helpers-and-test.rs").read_text();ck('mori_core::simulate(&program'in core1 and'mori_core::json::export(&legacy)'in core1 and'cost(&metered, cap, "16")'in core1,"Core1 shared-implementation compatibility limits")
ck('for key in ["pre", "post", "effects"]'in core2 and'for key in ["index", "value", "snapshot", "effects"]'in core2 and'assert_eq!(observed["continuation"], *c);'in core2,"Core2 selected fields plus whole actual continuation preservation")
constructors=load(p/"original-author/PRESERVED-27-CONSTRUCTOR-SCOPE.json");ck(constructors["count"]==27 and sum(map(len,constructors["constructors"].values()))==27 and constructors["allGapsRemainOpen"]and constructors["notWholeGrammarClosure"],"27 constructor obligations retained open")
custody=load(p/"SOURCE-RESULT-CUSTODY.json");mutation=load(p/"actual/SOURCE-MUTATION.json")
ck(custody["reportedCommit"]==head and custody["reportedTree"]==candidate["tree"]and custody["reportedParent"]==parent and custody["beforeAndAfterSourcePinsCount"]==902 and custody["sourceManifestDigest"]==sha((p/"actual/SOURCE-PINS-AFTER.json").read_bytes())and custody["soleBetweenPhaseProductionMutation"]==delta,"exact custody binding/source delta")
ck(mutation["beforeSha256"]==bmap[production]and mutation["afterSha256"]==amap[production],"source mutation receipt exact hashes")
oldLine='    if mori_core::language(&source) == Ok(mori_core::stage::LANGUAGE_2) {'
newLines='    let language = mori_core::language(&source).map_err(super::core_io::operation)?;\n    if language == mori_core::stage::LANGUAGE_2 {'
ck(old[production].decode().count(oldLine)==1 and old[production].decode().replace(oldLine,newLines,1).encode()==current[production],"production mutation is only stated diagnostic propagation hunk")
for pin in load(p/"MANIFEST.json")["files"]:
    if pin["path"].startswith("source/"):
        ck((p/pin["path"]).read_bytes()==git("show",head+":"+pin["path"][7:]),"full current packet source equals Git "+pin["path"])
for row in custody["capturedObservationComparisons"]:
    folder=p/"actual"/row["phase"]/"public-cli"/row["case"]
    if row["case"]!="empty-core2":
        expected=next(x for x in freeze["header_cases"]if x["name"]==row["case"])
        ck(row["producerErrorExact"]==(load(folder/"producer-error.json")==expected["producer_error_json"])and row["exit"]==load(folder/"process.json")["status"]and row["stdoutEmpty"]==((folder/"stdout.bin").read_bytes()==b"")and row["stderrMatchesFrozenRepair"]==((folder/"stderr.txt").read_bytes()==expected["public_simulate_stderr"].encode())and row["stderrSha256"]==sha((folder/"stderr.txt").read_bytes()),"custody capture direct corroboration "+row["phase"]+"/"+row["case"])
    else:
        raw=load(folder/"stdout.bin")
        ck(row["frozenCompleteExecutionProjectionExact"]==(raw["segment"]["execution"]==empty["execution"])and row["statusComplete"]==(raw["status"]=="complete")and row["emptyStderrAndExit0"]and row["otherFullReportFieldsCapturedButNotAsserted"]and row["aggregateRustEmptyAssertionsReached"]==(row["phase"]=="green"),"custody empty projection direct corroboration "+row["phase"])
export=load(root/"job/TERMINAL-EXPORT-RECEIPT.json")
ck(export["decision"]=="EXPORT"and export["terminalExitCode"]==0 and export["terminalStatusLiteral"]=="SUCCESS"and export["actualBackendIdentityAttested"]is False,"recorded terminal export only; backend unattested")
ck(len(reviewRaw)==37454 and review["reviewToken"]==candidate["reviewToken"],"whole exported opinion byte count/token")
inputRoots=[{"path":str(p/name),"sha256":sha((p/name).read_bytes()),"bytes":(p/name).stat().st_size}for name in["MANIFEST.json","PACKET-MANIFEST.json","MANDATORY-INPUTS.json","REVIEW-RESPONSE.schema.json","FULL-INPUT.md"]]
allOpinionDigests=set(re.findall(r"\b[0-9a-f]{64}\b",reviewRaw.decode()));ck(allOpinionDigests<=physicalHashes,"every full narrative SHA256 occurs in verified public packet",sorted(allOpinionDigests-physicalHashes))
ck(review["sourceAssessment"]["verdict"]==review["actualResultAssessment"]["verdict"]==review["verdict"]=="approve","original three verdict fields; not new vote")
ck(reviewFile.read_bytes()==reviewRaw,"original opinion unchanged")
qualifications=[{"jsonPointer":"/actualResultAssessment/positiveOracleIndependenceDisposition","exactPassage":review["actualResultAssessment"]["positiveOracleIndependenceDisposition"],"evidencePath":"crates/mori/tests/core_continuations.rs","evidenceLine":173,"evidence":core2.splitlines()[172],"classification":"The phrase program/plan hashes unasserted is too broad if read literally. Whole continuation equality indirectly compares carried hash fields for preservation; no independent expected numeric hash is asserted. Read in its stated oracle-independence context; no corrected opinion supplied."},{"jsonPointer":"/actualResultAssessment/reason","exactPassage":review["actualResultAssessment"]["reason"],"evidencePath":"actual/FROZEN-OBSERVATION-COMPARISON.json","classification":"Universal freeze-equality reading would be inaccurate: all three RED human stderr files intentionally fail frozen-repair equality. The detailed opinion explicitly states this and distinguishes captured-root comparison from Rust reachability; detailed phase dispositions control."}]
result={"format":"simulate-header-opinion-mechanical-fidelity/1","result":"PASS_WITH_SCOPE_QUALIFICATIONS"if all(v["match"]for v in checks)else"MISMATCHES_FOUND","reviewSha256":sha(reviewRaw),"reviewBytes":len(reviewRaw),"reviewToken":review["reviewToken"],"candidate":candidate,"checks":checks,"literalMismatches":[v for v in checks if not v["match"]],"scopeWordingQualifications":qualifications,"sourceHashes":sourceHashes,"sourceQuotes":sourceQuotes,"logQuotes":logQuotes,"narrativeEscapedQuotes":narrativeChecks,"sourceUnits":unitResults,"manifestChecks":manifestCounts,"inputManifests":inputRoots,"terminalExportReceiptSha256":sha((root/"job/TERMINAL-EXPORT-RECEIPT.json").read_bytes()),"mandatoryCount":156,"schemaSha256":sha((p/"REVIEW-RESPONSE.schema.json").read_bytes()),"inline":{"bytes":len(inline),"sha256":sha(inline),"splitlines":len(inline.splitlines()),"eofInclusive":len(inline.decode().split("\n")),"schemaSplitlines":len((p/"REVIEW-RESPONSE.schema.json").read_bytes().splitlines()),"schemaEofInclusive":len((p/"REVIEW-RESPONSE.schema.json").read_text().split("\n")),"interpretation":"Review8866/1888line counts include the empty terminal EOF entry; physical splitlines8865/1887. No omitted bytes."},"sourcePinCustody":{"before":902,"after":902,"beforeSha256":sha((p/"actual/SOURCE-PINS-BEFORE.json").read_bytes()),"afterSha256":sha((p/"actual/SOURCE-PINS-AFTER.json").read_bytes()),"soleDelta":delta,"baseHead":parent,"postCommitBinding":head},"frozenExpectationPins":frozenPins,"phaseResults":phaseResults,"eightRawCaptures":captureResults,"reachability":"RED captures all4 processes before assertions; only unknown producer/exit/stdout checks execute before human stderr fails. Other RED assertions are unreachable; root capture comparison is separate. GREEN reaches all assertions.","emptyControl":"Exact captured report is retained; only status plus complete segment.execution projection is oracle-asserted. Other wrapper fields remain unasserted, even though RED/GREEN whole raw files happen to be equal.","limits":["This is a mechanical fidelity check, not a fresh review vote, corrected/spliced opinion or root adoption.","Full declared model reads, actual backend identity and source provenance cannot be authenticated by packet hashes/schema/quotes.","All902source pins and complete packet bytes were machine-processed, not a full repository model audit.","Phase logs name parent605and a changing tested workingtree; postcommit615binding equates bytes, not a new execution at615.","Expectations are source-exposed; existing controls provide shared-library compatibility/selected-field assertions, not independent financial/compiler/security oracles.","No private captures, providers, product/native/security executions or shared/Git writes; no148/292/20,U/M/MC/SP/G,constructor or target closure."],"originalOpinionUnchanged":reviewFile.read_bytes()==reviewRaw,"rootAdoptsSeparately":True}
(out/"RESULT.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps({"result":result["result"],"checks":len(checks),"literalMismatches":result["literalMismatches"],"scopeQualifications":len(qualifications),"sourceQuotes":len(sourceQuotes),"logQuotes":len(logQuotes),"captures":len(captureResults),"pins":len(amap),"reviewSha256":sha(reviewRaw)},indent=2))
raise SystemExit(0 if not result["literalMismatches"]else 1)
