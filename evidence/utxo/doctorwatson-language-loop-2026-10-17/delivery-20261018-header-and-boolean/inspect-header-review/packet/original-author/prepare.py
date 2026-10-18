from pathlib import Path
import subprocess,hashlib,json,difflib,re,datetime
O=Path(__file__).resolve().parent;L='/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018';H='615eceee9c6250d2f203cb5807d5c75482dbdb5d'
def git(*a):return subprocess.check_output(['git',*a],cwd=L)
def sha(b):return hashlib.sha256(b).hexdigest()
def body(p):return git('show',H+':'+p)
def write(p,b):p=O/p;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b.encode() if isinstance(b,str) else b)
def dump(p,x):write(p,json.dumps(x,indent=2,ensure_ascii=False)+'\n')
# Existing producer contract and prior ordinary simulate expectations, source-exposed;
# never evaluate producer or CLI to derive an expected value.
cases=[{'name':'unknown','source':'language "other";','error':{'code':'mori::core::profile','message':'Expected language "moriarty/1" or "moriarty/2"; found "other".','offset':16,'step':None},'stderr':'  × mori::command: mori::core::profile: Expected language "moriarty/1" or\n  │ "moriarty/2"; found "other".\n\n'}, {'name':'missing','source':'language ;','error':{'code':'mori::core::syntax','message':'Expected language "moriarty/1" or "moriarty/2". Expected a quoted string.','offset':9,'step':None},'stderr':'  × mori::command: mori::core::syntax: Expected language "moriarty/1" or\n  │ "moriarty/2". Expected a quoted string.\n\n'}, {'name':'commented','source':'// café\nlanguage "other";','error':{'code':'mori::core::profile','message':'Expected language "moriarty/1" or "moriarty/2"; found "other".','offset':25,'step':None},'stderr':'  × mori::command: mori::core::profile: Expected language "moriarty/1" or\n  │ "moriarty/2"; found "other".\n\n'}]
for c in cases:
 c['exitCode']=1;c['stdout']='';write('expectations/'+c['name']+'/header.mori',c['source']);write('expectations/'+c['name']+'/stdout.bin',b'');write('expectations/'+c['name']+'/stderr.txt',c['stderr']);dump('expectations/'+c['name']+'/producer-error.json',c['error'])
dump('EXPECTATIONS.json',{'status':'FROZEN_BEFORE_ANY_EXECUTION_UNRUN','cases':cases,'provenance':'Ordinary source/diagnostic expectations. Author inspected current parser::language, Core1 fallback, operation/main/Shell renderer and existing simulate_header_dispatch literals. These are reused source-exposed literals, not independent security/financial/work or actual-output goldens. Full stderr assumes --color never and captured pipe width80, matching existing normal-simulate test and render::WIDTH. Do not regenerate literals from candidate output.','positiveControls':[{'target':'capabilities','selector':'reported_core_profiles_select_real_source_only_cli_consumers','scope':'Existing valid Core1+Core2 public check/source-inspect reports and preservation, unchanged.'},{'target':'core_continuations','selector':'check_and_source_inspect_route_core_two_without_legacy_consumers','scope':'Existing real Core2 Reserve source check+inspect/stage-plan assertions, existing oracle unchanged.'}]})
expectation_files=[p for p in sorted(O.rglob('*')) if p.is_file() and ('expectations' in p.parts or p.name=='EXPECTATIONS.json')]
write('EXPECTATIONS.sha256',''.join(sha(p.read_bytes())+'  '+str(p.relative_to(O))+'\n' for p in expectation_files))
# One source-only header-probe expression; keep native/target branch identical.
p='crates/mori/src/commands/inspect.rs';before=body(p);needle=b'mori_core::language(&source).is_ok_and(|language| language == "moriarty/2")';replacement=b'mori_core::language(&source).map_err(super::core_io::operation)? == "moriarty/2"';assert before.count(needle)==2
after=before.replace(needle,replacement,1);assert after.count(needle)==1
assert before.split(b'    #[cfg(feature = "native")]\n    if mori_core::language',1)[1]==after.split(b'    #[cfg(feature = "native")]\n    if mori_core::language',1)[1]
write('before/'+p,before);write('candidate/'+p,after)
# Reuse established complete ordinary diagnostics, then adapt public consumer only.
sim=body('crates/mori/tests/simulate_header_dispatch.rs').decode()
const_start=sim.index('struct HeaderCase');const_end=sim.index('const EMPTY_CORE2')
constants=sim[const_start:const_end]
project_start=sim.index('struct Project');project_end=sim.index('#[test]\nfn normal_simulate')
project=sim[project_start:project_end].replace('MORI_SIMULATE_HEADER_OBSERVATIONS','MORI_INSPECT_HEADER_OBSERVATIONS').replace('mori-simulate-header-','mori-inspect-header-').replace('        fs::write(dir.join("scenario.json"), EMPTY_SCENARIO).unwrap();\n','').replace('            "simulate",','            "inspect",').replace('            "--action",\n            "run",\n            "--scenario",\n            "scenario.json",\n','').replace('run actual public simulate','run actual public source-only inspect')
test='''//! Source-only inspect preserves the Core header producer's diagnostics.
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

'''+constants+project+'''#[test]
fn source_only_inspect_keeps_complete_header_errors() {
    let project = Project::new();
    // Capture each public process stream before assertions, including on failure.
    let captured = CASES
        .iter()
        .map(|case| {
            let error = mori_core::language(case.source)
                .err()
                .map(|error| serde_json::to_value(error).unwrap())
                .unwrap_or(Value::Null);
            let output = project.run(case.name, case.source);
            fs::write(
                project.root.join(case.name).join("producer-error.json"),
                serde_json::to_vec_pretty(&error).unwrap(),
            )
            .unwrap();
            (case, error, output)
        })
        .collect::<Vec<_>>();
    for (case, error, output) in captured {
        let expected: Value = serde_json::from_str(case.expected_error).unwrap();
        assert_eq!(error, expected, "{}: complete producer Error", case.name);
        assert_eq!(output.status.code(), Some(1), "{}", case.name);
        assert!(output.stdout.is_empty(), "{}: complete stdout", case.name);
        assert_eq!(
            output.stderr,
            case.expected_stderr.as_bytes(),
            "{}: complete human stream",
            case.name
        );
    }
}
'''
testpath='crates/mori/tests/inspect_header_dispatch.rs';assert git('ls-tree',H,'--',testpath)==b'';write('candidate/'+testpath,test)
# Verify the copied test literal expectations agree with the already frozen files.
for c in cases:
 assert c['source'] in constants and c['stderr'] in constants
 assert c['error']['code'] in constants
patch=''.join(difflib.unified_diff(before.decode().splitlines(keepends=True),after.decode().splitlines(keepends=True),fromfile='a/'+p,tofile='b/'+p))
patch+=''.join(difflib.unified_diff([],test.splitlines(keepends=True),fromfile='/dev/null',tofile='b/'+testpath))
write('CANDIDATE.patch',patch)
context_paths=['crates/mori-core/src/parser.rs','crates/mori-core/src/ir.rs','crates/mori/src/main.rs','crates/mori/src/shell.rs','crates/mori/src/error.rs','crates/mori/src/commands/core_io.rs','crates/mori/src/commands/check_source.rs','crates/mori/src/commands/simulate.rs','crates/mori-diagnostics/src/render.rs','crates/mori/tests/simulate_header_dispatch.rs','crates/mori/tests/capabilities.rs','crates/mori/tests/core_continuations.rs','crates/mori/Cargo.toml','crates/mori-core/Cargo.toml','examples/core/continuations/reserve.mori','RUST_GUIDELINES.md']
pins=[]
for f in [p]+context_paths:
 b=body(f);write('source-context/'+f,b);pins.append({'path':f,'commit':H,'gitBlob':git('rev-parse',H+':'+f).decode().strip(),'sha256':sha(b),'bytes':len(b)})
dump('SOURCE-PINS.json',{'base':H,'tree':git('rev-parse',H+'^{tree}').decode().strip(),'sources':pins,'candidateFiles':[{'path':f,'sha256':sha((O/'candidate'/f).read_bytes()),'bytes':(O/'candidate'/f).stat().st_size} for f in [p,testpath]]})
commands=[]
for target,selector in [('inspect_header_dispatch','source_only_inspect_keeps_complete_header_errors'),('capabilities','reported_core_profiles_select_real_source_only_cli_consumers'),('core_continuations','check_and_source_inspect_route_core_two_without_legacy_consumers')]:
 commands.append({'argv':['cargo','test','--frozen','-j1','-p','mori','--no-default-features','--test',target,selector,'--','--exact','--test-threads=1'],'status':'INSPECTED_UNRUN','scope':'Source-only public process; no --target, native, trace, SDK/proof/key/wallet/ledger feature/action.','expectedTestCount':1})
dump('REGRESSION-PLAN.json',{'status':'SOURCE_ONLY_UNRUN','commands':commands,'redGreen':'Root may apply only new regression on base615 for RED and then one-expression source fix for GREEN under its own existing execution plan; preserve all captures. No such execution by this author.','captureEnv':'MORI_INSPECT_HEADER_OBSERVATIONS: optional root-owned absolute directory that must be absent before test; when set, process captures retained even on success. Without it, successful owned temp directory removed; panic leaves diagnostics. No global cleanup.','assertionScope':'3complete producer errors+status1+empty entire stdout+literal entire stderr; existing separate valid Core1/Core2 controls unchanged. No new financial/work oracle.','sideEffectsIfAuthorized':'Cargo ordinary build outputs; test fresh-owned temp or root capture directory with3source files, process/stdout/stderr/producer JSON; subprocesses are actual mori inspect --color never source-only. No external tools via product.'})
dump('FINDING.json',{'status':'REAL_ORDINARY_SOURCE_DEFECT_SOURCE_CONFIRMED_RUNTIME_UNRUN','at':'inspect.rs28-30 source-only branch; Core1 fallback at59','before':'language Err is converted tofalse by is_ok_and; then parse(/1) emits narrower Expected language moriarty/1 or underlying syntax message, losing existing language() dual-profile prefix.','after':'Within args.target.is_none() short-circuit, preserve original language Error via existing core_io::operation and main/Shell failure renderer. /2 success still uses same stage loader/report; /1 success still same parse/report.','argsAndJSON':'inspect Args has file and optional --target, no --json flag. Source-only accepted reports are JSON; a recognized /2 staged-load error keeps existing rejected JSON. Invalid header failure already used operation stderr; preserve that channel and empty stdout rather than invent a report DTO.','unchangedTarget':'The entire native /2 --target branch and downstream target/report helpers remain byteidentical; no target/native profile expansion or decision. Non-native --target still returns native_unavailable before source I/O.','knownLimits':'Native-target header is_ok_and remains present and outside this source-only repair; no judgment or execution on that path. No shared router/general profile semantics changes.','acceptance':'UNAPPLIED/UNRUN; source proof only, frozen exposed ordinary expectations, independent source/result review still required.','preserve':'All27grammar/148requirements/292scenarios/20families/fullU0-U7/M1-M17/MC/SP/G/finance/composition/native/target/Artifact/security ownership unchanged.'})
write('README.md','''Source-only inspect header error propagation, base615

CANDIDATE.patch changes exactly one source-only expression in inspect.rs and adds one real public-process regression. Source-only inspect currently drops language() Err, falls into the /1 parser and replaces the established dual-profile diagnostic. Preserve the producer error through existing operation/main/Shell rendering. No new JSON shape or helper: inspect has no --json flag. Valid Core1/Core2 report paths are unchanged; full target/native branch and helpers stay byte-identical.

EXPECTATIONS.json and12physical source/error/full stdout/full stderr inputs are frozen before execution (EXPECTATIONS.sha256). Literals are explicitly source-exposed from parser and existing ordinary simulate regression, not independent financial/work expectations or actual inspect captures. New test captures all3public inspect processes before assertions, including on RED. Existing capabilities/Core2 continuation selectors give valid-profile controls; expectations are unchanged.

Full production before/after, full new test, exact source pins and direct-context files are physical. REGRESSION-PLAN lists3ordinary exact SDK-free test argv, all UNRUN by author. Root alone integrates, executes and reviews. No provider/Git/shared-worktree/product/native/SDK/proof/wallet operation occurred; no security judgment. All original grammar/family/financial/work and U/M/MC/SP/G exits retained. Do not regenerate frozen expectations from candidate output.
''')
# Only static transport/one-expression preservation assertions; no Rust tool invoked.
assert after.replace(replacement,needle,1)==before
assert '--target' not in test and '"inspect"' in test and 'EMPTY_SCENARIO' not in test
assert len(re.findall(r'HeaderCase \{\n        name:',constants))==3
dump('STATIC-VALIDATION.json',{'status':'PASS_SOURCE_TRANSPORT_ONLY','oneProductionExpressionChanged':True,'targetAndAllDownstreamHelpersVerbatim':True,'newRealPublicProcessTestCases':3,'frozenStreams':{'stdout':'empty bytes all3','stderr':'complete existing human diagnostic literals'},'existingBothProfileAndCore2Controls':True,'runtimeExecutions':0,'providerCalls':0,'GitSharedWrites':0})
manifest={'format':'inspect-source-header-current615-source-proposal/1','files':[{'path':str(f.relative_to(O)),'bytes':f.stat().st_size,'sha256':sha(f.read_bytes())} for f in sorted(O.rglob('*')) if f.is_file() and f.name not in ['MANIFEST.json','FREEZE.sha256']]};dump('MANIFEST.json',manifest)
write('FREEZE.sha256',''.join(sha(f.read_bytes())+'  '+str(f.relative_to(O))+'\n' for f in sorted(O.rglob('*')) if f.is_file() and f.name!='FREEZE.sha256'))
print(json.dumps({'patchSha256':sha((O/'CANDIDATE.patch').read_bytes()),'expectationFreezeSha256':sha((O/'EXPECTATIONS.sha256').read_bytes()),'manifestSha256':sha((O/'MANIFEST.json').read_bytes()),'freezeSha256':sha((O/'FREEZE.sha256').read_bytes()),'candidateTestBytes':len(test.encode())},indent=2))
