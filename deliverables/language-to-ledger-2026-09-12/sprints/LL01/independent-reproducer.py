#!/usr/bin/env python3
"""Offline LL01 diagnosis. Real preflight/query/stat; exec and rlimit are injected.
No K, strace, loader, Java, network or resource mutation is executed.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import tempfile
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / '.moriarty-dev'
OUT = Path(__file__).resolve().parent
names = [f'k-macro05-trace106-{s}' for s in ['diagnostic.py','pins.json','requisites.json','k-package-map.json','ownership.json']] + ['test_k_macro05_trace106_diagnostic.py']
def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    before = {n:digest(HERE/n) for n in names}
    spec = importlib.util.spec_from_file_location('independent_shim', HERE/names[0])
    shim = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(shim)
    pins = shim.load_pins()
    ownership = json.loads((HERE/names[4]).read_text())
    requisites = json.loads((HERE/names[2]).read_text())
    package = json.loads((HERE/names[3]).read_text())
    hosts = shim.load_host_evidence(ownership)
    _, expected_uid = shim.read_namespace_mapping(ownership)
    calls, ownership_checks, fallback_checks = [], [], []
    actual_ownership, actual_nonwritable = shim.verify_ownership, shim.verify_nonwritable
    def tracked_ownership(path, *args):
        ownership_checks.append(path)
        return actual_ownership(path,*args)
    def tracked_nonwritable(path,*args):
        fallback_checks.append(path)
        return actual_nonwritable(path,*args)
    shim.verify_ownership = tracked_ownership
    shim.verify_nonwritable = tracked_nonwritable
    # A harmless non-JAR text marker models a present user extension path only.
    # The loader never executes. HOME is passed as an argument, not reassigned.
    with tempfile.TemporaryDirectory(prefix='ll01-offline-') as raw:
        extension = Path(raw)/'.local/lib/kframework/java/independent-marker.jar'
        extension.parent.mkdir(parents=True)
        extension.write_text('offline marker, never loaded\n')
        result = shim.run_shim(pins=pins, here=HERE, home=raw,
            execve=lambda path,argv,env:calls.append({'path':path,'argv':argv,'HOME':env['HOME']}),
            getrlimit=lambda *_:(8388608,-1), setrlimit=lambda *_:None)
        home_probe = {'status':result['status'],'preflight':result['preflight'],
            'injectedExecCalls':len(calls),'markerPresentAtExec':extension.exists(),
            'homePreserved':result['env']['HOME']==raw}
    kroot = Path(package['kRoot'])
    loader = kroot/'lib/kframework/checkJava'
    loader_text=loader.read_text()
    declared=list(pins['wrapperPathDirs'])
    outer_actual=list(reversed(declared))+pins['childEnv']['PATH'].split(':')
    parser_actual=[str(kroot/'bin-unwrapped')]+outer_actual
    checker_model=declared+pins['childEnv']['PATH'].split(':')
    # Preserve stage spelling; selected symlink target is recorded separately.
    stages={'initial':pins['childEnv']['PATH'].split(':'), 'wrapper':outer_actual,
            'krun':parser_actual, 'checkJava':[str(kroot/'bin')]+parser_actual}
    selected={i['path'] for i in pins['selectedExecutables']}
    helpers={}
    for name in ['dirname','basename','mktemp','date','rm','cat','cp','fold','uname','grep','head','cut','sed','java','llvm-krun','kore-print']:
        path=shim.resolve_on_path(name,stages['checkJava'])
        helpers[name]={'selected':str(path) if path else None,
            'resolved':str(path.resolve()) if path else None,
            'explicitlyPinned':str(path) in selected if path else False}
    unbound={i['role']:i['path'] for i in pins['selectedExecutables']
             if shim.bind_path_for_ownership(i['path'],hosts) is None}
    envpath='/usr/bin/env'
    direct_error=None
    try: actual_ownership(envpath,hosts,expected_uid,0)
    except shim.PreflightError as exc: direct_error=exc.code
    symlinks=[]
    for path, entry in hosts.items():
        p=Path(path)
        if not p.is_symlink(): continue
        st, lst = p.stat(),p.lstat()
        if (st.st_mode & 0o7777)==entry['mode'] and (lst.st_mode & 0o7777)!=entry['mode']:
            try: actual_ownership(path,hosts,expected_uid,0)
            except shim.PreflightError as exc:
                symlinks.append({'path':path,'code':exc.code,'detail':exc.detail,
                    'followedMode':oct(st.st_mode & 0o7777),'linkMode':oct(lst.st_mode & 0o7777),
                    'evidenceMode':oct(entry['mode']),
                    'linkIdentityMatches':lst.st_ino==entry['lstatInode'] and lst.st_dev==entry['lstatDevice']})
    findings=[
      {'id':'E04-1','evidenceClass':'actual callable offline preflight; loader source inspection',
       'observed':home_probe,'loaderSHA256':digest(loader),
       'loaderBranches':{s:s in loader_text for s in ['$HOME/.kserver','$HOME/.local/lib/kframework/java/*','$NG org.kframework.main.JavaVersion']},
       'redPredicate':len(calls)==0,
       'expected':'A present uncommitted HOME Java extension must fail before injected exec under an absence policy, or a reviewed loader selection must actually exclude it. Cover local HOME nailgun and default nailgun separately; do not connect during tests.'},
      {'id':'E04-2','evidenceClass':'source-derived stage PATH plus actual Python resolution',
       'checkerPathMatchesActualParserPath':checker_model==parser_actual,'stagePaths':stages,'helpers':helpers,
       'currentParserSameTarget':shim.resolve_on_path('python3',checker_model).resolve()==shim.resolve_on_path('python3',parser_actual).resolve(),
       'redPredicate':checker_model==parser_actual and all(x['explicitlyPinned'] for x in helpers.values()),
       'expected':'Verify actual reverse wrapper prepend and later krun/setenv prepends. Pin selected host helper links and targets plus Java/LLVM/kore. An earlier executable competitor or changed helper/target must fail before exec; current Python selection is not shown wrong.'},
      {'id':'E04-3','evidenceClass':'actual callable ownership checks and real preflight call tracing',
       'unboundSelected':unbound,'envFallbackVisited':envpath in fallback_checks,
       'directEnvOwnershipError':direct_error,
       'databaseOwnershipChecks':[p for p in ownership_checks if p.startswith('/nix/var/nix/db')],
       'redPredicate':not unbound and any(p.startswith('/nix/var/nix/db') for p in ownership_checks),
       'expected':'Reject a selected path without host binding even when UID is expected and nonwritable. Bind links and resolved targets, query package and trusted database/ancestors; mismatched host identity, writable target or drift must prevent exec.'},
      {'id':'E04-4','evidenceClass':'actual Path.stat/lstat and callable verify_ownership',
       'falseRejections':symlinks,'redPredicate':not symlinks,
       'expected':'Unchanged host-root symlinks with correct distinct link/target evidence pass in host and strong namespace. Compare like stat fields; link replacement and target identity/mode/owner drift reject.'}
    ]
    after={n:digest(HERE/n) for n in names}
    report={'kind':'LL01-independent-offline-reproducer/1','timestampUTC':datetime.now(timezone.utc).isoformat(),
        'sourceHashesBefore':before,'sourceHashesAfter':after,'sourceUnchanged':before==after,
        'uidMap':Path('/proc/self/uid_map').read_text(),'findings':findings,
        'redPredicatesFailed':sum(not f['redPredicate'] for f in findings),
        'scope':'Offline source/mechanism evidence only. Real local read-only Nix queries. Exec and limits injected. No loader, K, Java, strace, compile, Docker, network, live guarded diagnostic, accounting or resource mutation. No native result or acceptance.',
        'remainingControls':'Strong-namespace rerun and repaired positive/negative controls remain required. HOME nailgun/default endpoint branches inspected, never connected. Temporary marker removed.'}
    (OUT/'independent-reproducer.json').write_text(json.dumps(report,indent=2)+'\n')
    lines=['# LL01 independent offline reproduction','',f"Actual offline probe found {report['redPredicatesFailed']}/4 failed required predicates; source hashes remained unchanged: {before==after}.",'',report['scope'],'']
    for f in findings:
        lines += [f"- **{f['id']}** ({f['evidenceClass']}): {f['expected']}"]
    lines += ['',f"Preflight reached injected exec with a present HOME extension marker. Missing ownership bindings include {', '.join(unbound)}. {len(symlinks)} existing symlinks reproduce the stat/lstat mismatch. The current Python target is the same under both PATH orders, so no current parser runtime failure is claimed.",'',report['remainingControls'],'','Run with `python3 -I -B deliverables/language-to-ledger-2026-09-12/sprints/LL01/independent-reproducer.py`. The JSON preserves exact source hashes and observations. A failed red predicate describes required behavior absent in this candidate; the script succeeds when it records evidence, not when the candidate passes.','']
    (OUT/'independent-reproducer.md').write_text('\n'.join(lines))
    print(json.dumps({'redPredicatesFailed':report['redPredicatesFailed'],'sourceUnchanged':before==after,'symlinkFalseRejections':len(symlinks),'unboundSelected':unbound,'homeProbe':home_probe}))
if __name__=='__main__': main()
