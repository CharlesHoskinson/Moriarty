"""One frozen cold compile; run only through the reviewed stock Root scope."""
from pathlib import Path
import ast, hashlib, json, os, re, resource, stat, sys, time

Q = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-recursion-restart-20261022/rollup/next-product-code-source-investigation01')
BASELINE_D = Q / 'c6-source03-cold-ordinary-compile-card02'
D = Q / 'c6r2-parent-cold-compile-data01'
M = D
CARD = M / 'COLD-COMPILE-CARD-M1.json'
SCOPE = 'sherlock-c6r2-source05-cold-root01.scope'
DESC = 'Sherlock C6R2 Source05 cold root dfd5ca3e33335b7c exactly once'
CG = Path('/sys/fs/cgroup')
# Sole semantic guard delta: the exact normal Cargo marker from reviewed same-build recovery.
MARKER = D / 'work01/cargo-home/registry/CACHEDIR.TAG'
MARKER_TEXT = (b'Signature: 8a477f597d28d172789f06886806bc55\n'
               b'# This file is a cache directory tag created by cargo.\n'
               b'# For information about cache directory tags see https://bford.info/cachedir/\n')
MARKER_SHA256 = '6d9d1d216e0f83abc5e5662ca62c92b4f23009466b54fa27321a69acdb778bb2'

def require(ok, message):
    if not ok: raise RuntimeError(message)

def pin(path):
    p = Path(path)
    with p.open('rb') as f: digest = hashlib.file_digest(f, 'sha256').hexdigest()
    return dict(path=str(p), bytes=p.stat().st_size, sha256=digest,
                mode=stat.S_IMODE(p.stat().st_mode), realpath=str(p.resolve(strict=True)))

def check(row, path=None, mode=None):
    p = Path(path or row['path']); a = pin(p)
    require(all(a[k] == row[k] for k in ('bytes', 'sha256')), 'Pin drift: ' + str(p))
    if 'realpath' in row: require(a['realpath'] == row['realpath'], 'Alias drift: ' + str(p))
    if mode is not None: require(a['mode'] == mode, 'Mode drift: ' + str(p))
    return a

def plain(p, directory=False):
    p = Path(p)
    require(not any(x.is_symlink() for x in [p, *p.parents]), 'Symlink: ' + str(p))
    require((stat.S_ISDIR if directory else stat.S_ISREG)(p.lstat().st_mode), 'Kind: ' + str(p))

def save(path, value):
    p = Path(path)
    with p.open('x') as f:
        os.chmod(p, 0o600); json.dump(value, f, indent=2); f.write('\n'); f.flush(); os.fsync(f.fileno())
    fd = os.open(p.parent, os.O_RDONLY | os.O_DIRECTORY)
    try: os.fsync(fd)
    finally: os.close(fd)

def sample(p):
    names = ['memory.max', 'memory.swap.max', 'memory.swap.current', 'memory.peak', 'memory.events', 'cgroup.events']
    return dict(path=str(p), files={n: (p / n).read_text() for n in names})

def own_scope():
    lines = Path('/proc/self/cgroup').read_text().splitlines()
    require(len(lines) == 1 and lines[0].startswith('0::/'), 'Unified cgroup v2 required')
    p = CG / lines[0][3:].lstrip('/')
    require(p.name == SCOPE, 'Root must be inside exact scope')
    a = sample(p)
    require(a['files']['memory.max'].strip() == '1073741824' and a['files']['memory.swap.max'].strip() == '0', 'Scope caps')
    require({'memory','cpu','pids'} <= set((p.parent/'cgroup.controllers').read_text().split()), 'Delegated controllers')
    return a

def no_oom(before, after, native=False):
    require(before['path'] == after['path'], 'Cgroup changed')
    for s in (before, after): require(int(s['files']['memory.peak']) >= 0, 'Missing peak')
    a, b = [dict(x.split() for x in s['files']['memory.events'].splitlines()) for s in (before, after)]
    for k in ['oom', 'oom_kill'] + (['oom_group_kill'] if 'oom_group_kill' in a or 'oom_group_kill' in b else []):
        require(k in a and k in b and int(a[k]) == int(b[k]) and (not native or int(a[k]) == 0), 'OOM evidence: ' + k)
    require(after['files']['memory.swap.max'].strip() == '0', 'Swap cap drift')

def host_diagnostics():
    return {k: v.strip() for line in Path('/proc/meminfo').read_text().splitlines()
            for k, v in [line.split(':',1)] if k in ['MemTotal','MemAvailable','SwapTotal','SwapFree']}

def children():
    while True:
        try:
            if os.waitpid(-1, os.WNOHANG)[0] == 0: break
        except ChildProcessError: break
    return Path(f'/proc/{os.getpid()}/task/{os.getpid()}/children').read_text().split()

def streams(result, phase, limit, zero=True):
    require(result['controller_failure'] is None and (not zero or result['actual_exit'] == 0), 'Capture exit/failure')
    rows = result['streams']; require(len(rows) == 2, 'Need exactly two streams')
    require({Path(r['path']).name for r in rows} == {'stdout.bin', 'stderr.bin'}, 'Stream names')
    for r in rows:
        p = Path(r['path']); require(p.parent == phase, 'Stream phase'); plain(p)
        require(r['full_stream_retained'] is True and r['bytes_seen'] == r['bytes_retained'] == r['bytes'] <= limit, 'Incomplete EOF stream')
        check(r)
    require(not children(), 'Unreaped Root children')

def disk():
    v = os.statvfs(D / 'work01'); candidates = []
    for line in Path('/proc/self/mountinfo').read_text().splitlines():
        left, right = line.split(' - ', 1); mount = left.split()[4]
        if str(D / 'work01') == mount or str(D / 'work01').startswith(mount.rstrip('/') + '/'):
            candidates.append((len(mount), right.split()[0], line))
    entry = max(candidates)
    require(entry[1] == 'ext4', 'Expected ext4')
    return dict(availableBytes=v.f_bavail * v.f_frsize, availableInodes=v.f_favail, mount=entry[2])

def membership(root, expected, source=False, post=False):
    root = Path(root); plain(root, True); seen = set(); dirs = {root}
    for p in expected:
        p = Path(p); require(p.is_relative_to(root), 'Manifest escaped root')
        dirs.update(x for x in p.parents if x.is_relative_to(root))
    growth = root / 'registry/src/index.crates.io-1949cf8c6b5b557f'
    locks = {'.package-cache', '.package-cache-mutate', '.global-cache', '.global-cache-journal', '.global-cache-wal', '.global-cache-shm'}
    for parent, dn, fn in os.walk(root, followlinks=False):
        for name in dn + fn:
            p = Path(parent) / name; isdir = name in dn; plain(p, isdir)
            allowed = post and not source and (p == growth or p.is_relative_to(growth) or p in growth.parents and p.is_relative_to(root) or p.parent == root and name in locks and not isdir)
            if isdir:
                require(p in dirs or allowed, 'Unexpected directory: ' + str(p))
                if source: require(stat.S_IMODE(p.stat().st_mode) == 0o555, 'Source directory mode')
            else:
                require(str(p) in expected or allowed, 'Unexpected file: ' + str(p)); seen.add(str(p))
    require(set(expected) <= seen and (post or seen == set(expected)), 'Membership drift')
    if source: require(stat.S_IMODE(root.stat().st_mode) == 0o555, 'Source root mode')

def main():
    require(len(sys.argv) == 1 and not sys.flags.optimize, 'No CLI modes or optimized assertions')
    scope0 = own_scope()  # First observation: refuse every unscoped start.
    check(dict(path=str(CARD), bytes=17349, sha256='0597b478b6feaab250d8b74682753e04436d6ab0453e4e6cb3544c5e701cf038'), mode=0o444)
    card = json.loads(CARD.read_text()); run = Path(card['runRoot']); env = card['outerCapture']['environment']
    rows = [(dict(path=str(CARD), bytes=17349, sha256='0597b478b6feaab250d8b74682753e04436d6ab0453e4e6cb3544c5e701cf038'), 0o444)]
    for k in ['source03Seal', 'stagingPins', 'qualifiedCurrentNormalCacheFacts', 'toolPins', 'composition', 'subjectData', 'selector', 'selectorCfg', 'selectorBasis', 'jobScopeClarification', 'source05Materialization', 'source05DependencyClosure']:
        rows.append((card[k], 0o644 if k in ['composition', 'jobScopeClarification', 'source05Materialization', 'source05DependencyClosure'] else 0o444))
    for name, size, digest in [
        ('COLD-COMPILE-CARD.json',14707,'6eff67cc0a28675d45d1bf3b0b9b63dc3abc325851c27b1eddd52e9bc023c076'),
        ('COMPILE-SUBJECT.json',7333,'a41e19264893a10836e26b949a96d107b4f717e1a1fcc602c34a369ae0975cf1'),
        ('SOURCE-SEAL.json',3495,'ca8b42b2987fd634cf2223a1822d126e2d8df71e890d0c040dbe1195311deeee'),
        ('STAGING-RECEIPT.json',335337,'027e011b11f5062d4586931242d8b7606ae6d80eb5e19a2aa647c072ae4f5699')]:
        rows.append((dict(path=str(BASELINE_D / name), bytes=size, sha256=digest), 0o444))
    # Current-revision metadata bindings; historical Source03 data above remain pinned as before.
    for relative, size, digest, mode in [
        ('c6r2-parent-cold-compile-data01/STAGING-RECEIPT.json',1839,'ca4d6b790da320ab231a77cd916bf86c904f3ff45019a54d581db54053914316',0o444),
        ('c6r2-parent-source05/SOURCE-SEAL.json',8446,'efd071e11566342716b598612507060cdd8411dd18946d771666bad205d9efc2',0o644),
        ('c6r2-parent-source05/HANDOFF.json',15530,'fef454fac67c74e83175f0e556f67ba6932dbe0e9d60bb26581185b4c10acb19',0o644),
        ('c6r2-parent-source05/actual-opus-independent-source-review01/REPORT.txt',12923,'3226cbb20bfe1d632b62dae62b422b6750eb0d91e12ea089b5b7608072856673',0o644)]:
        rows.append((dict(path=str(Q / relative), bytes=size, sha256=digest), mode))
    for r, mode in rows: plain(r['path']); check(r, mode=mode)
    rows.append((json.loads((BASELINE_D/'COLD-COMPILE-CARD.json').read_text())['selectorCfg'],0o444))
    tools = json.loads(Path(card['toolPins']['path']).read_text())
    require(len(tools['toolPins']) == 28, '28 tools required')
    rows += [(r, 0o755) for r in tools['toolPins']]
    rows += [(tools[k], 0o644) for k in ['existingCapture', 'existingGuard', 'existingComposition']]
    rows += [(dict(path='/usr/bin/env', realpath='/usr/lib/cargo/bin/coreutils/env', bytes=11807104, sha256='f84e76ea4da1e0c4e92254bd0d7dccdabe2c8b3098e734f27997dae3ee5d8e02'), 0o755), (dict(path='/usr/bin/systemctl', realpath='/usr/bin/systemctl', bytes=302112, sha256='6394f5e8df92878184de9d4dfb7ac242471cb09daf89d23be4e81ae17e9c03b2'), 0o755)]
    selfpin = pin(Path(__file__).resolve()); require(selfpin['mode'] == 0o444, 'Frozen receiver mode')
    def readbacks(post=False):
        evidence = [check(r, mode=mode) for r, mode in rows]
        require(pin(__file__) == selfpin, 'Receiver drift')
        source = {}; cache = {}
        for r in staging['sourceFiles']:
            p = r['snapshotDestination']; plain(p); check(r, p, 0o444); source[p] = r
        for r in staging['cacheFiles']:
            p = r['destination']; plain(p); check(r, p, 0o444); cache[p] = r
        require(str(MARKER) not in cache, 'Marker must not be a staged cache row')
        if post:
            plain(MARKER)
            marker = check(dict(path=str(MARKER), bytes=177, sha256=MARKER_SHA256), mode=0o600)
            require(marker['realpath'] == str(MARKER) and MARKER.lstat().st_nlink == 1 and
                    MARKER.read_bytes() == MARKER_TEXT, 'Marker text/alias')
            cache[str(MARKER)] = marker
        membership(staging['sourceRoot'], source, True, post)
        membership(staging['cargoHome'], cache, False, post)
        require(stat.S_IMODE(Path(staging['cargoHome']).stat().st_mode) == 0o700, 'Cargo home mode')
        for name in ['empty-home','mask-empty']:
            p = D/'work01'/name; plain(p,True)
            require(stat.S_IMODE(p.stat().st_mode) == 0o555 and not list(p.iterdir()), 'Empty directory drift')
        return evidence
    for r, mode in rows: check(r, mode=mode)
    staging = json.loads(Path(card['stagingPins']['path']).read_text())
    require(len(staging['sourceFiles']) == 267 and len(staging['cacheFiles']) == 298, 'Input cardinality')
    subject = json.loads(Path(card['subjectData']['path']).read_text())['subject']
    cfg = json.loads(Path(card['selectorCfg']['path']).read_text())
    require(cfg['operation'] == subject and card['outerCapture']['argv'] == card['serviceArgv'], 'Effective operation binding')
    pre = readbacks(); paths = card['dataStaging']['paths']
    for k in ['target', 'TMPDIR', 'emptyHome', 'maskEmpty']:
        p = Path(paths[k]); plain(p, True); require(not list(p.iterdir()), 'Nonempty fresh path: ' + str(p))
        require(stat.S_IMODE(p.stat().st_mode) == (0o700 if k in ['target', 'TMPDIR'] else 0o555), 'Fresh mode')
    require(os.access(paths['TMPDIR'], os.W_OK) and os.access(paths['cargoHome'], os.W_OK), 'Writable roots')
    args = subject['argv']; mask = paths['maskEmpty']; masks = []
    for i, item in enumerate(args[:-2]):
        if item == '--ro-bind' and args[i+1] == mask:
            p = Path(args[i+2]); require(stat.S_ISDIR(p.lstat().st_mode), 'Mask destination'); masks.append(str(p))
    require(set(['/run','/sys/fs/cgroup','/nix/var/determinate','/nix/var/nix/daemon-socket','/var/lib/docker','/mnt','/tmp','/var/tmp','/opt','/srv','/media']) <= set(masks), 'M1 masks')
    require(stat.S_ISDIR(Path('/home').lstat().st_mode), '/home mask destination')
    cwd = Path(cfg['cargoCwd'])
    for parent in [cwd, *[p for p in cwd.parents if p.is_relative_to(staging['sourceRoot'])], Path(paths['cargoHome'])]:
        for name in ['.cargo/config', '.cargo/config.toml'] if parent != Path(paths['cargoHome']) else ['config', 'config.toml', 'credentials', 'credentials.toml']:
            require(not os.path.lexists(parent / name), 'Unexpected Cargo discovery input')
    for p in ['/.cargo', '/Cargo.toml']: require(not os.path.lexists(p), 'Unexpected visible Cargo ancestor')
    for p in [cwd/'src/main.rs', cwd/'vendor/history', cwd.parent/'verifier-patch/midnight-circuits', cwd.parent/'private-sdk/midnight-proofs-0.8.2']:
        require(p.exists(), 'Missing source topology')
    limits = {n: resource.getrlimit(getattr(resource,n)) for n in ['RLIMIT_CPU','RLIMIT_AS','RLIMIT_FSIZE','RLIMIT_CORE']}
    require(limits['RLIMIT_CPU'][1] == resource.RLIM_INFINITY or limits['RLIMIT_CPU'][1] >= 700, 'Root inherited CPU limit')
    disk0 = disk(); require(disk0['availableBytes'] >= 2147483648 and disk0['availableInodes'] >= 4096, 'Disk floor')
    require(not os.path.lexists(run), 'One-shot run already consumed')
    tree = ast.parse(Path(tools['existingCapture']['path']).read_text())
    ns = {}; nodes = [n for n in tree.body if isinstance(n,(ast.Import,ast.ImportFrom)) or isinstance(n,ast.FunctionDef) and n.name in {'pin','save','stop_group','capture'}]
    exec(compile(ast.Module(body=nodes,type_ignores=[]),tools['existingCapture']['path'],'exec'),ns)
    capture = ns['capture']
    run.mkdir(mode=0o700); (run/'controls').mkdir(mode=0o700)
    save(run/'ISSUED.json', dict(receiver=selfpin, pid=os.getpid(), attempts=1, retry=False))
    def take(op, phase):
        result = capture(op, phase)
        for r in result['streams']:
            with open(r['path'],'rb') as f: os.fsync(f.fileno())
        save(phase/'RESULT.json', result); return result
    def control(name, argv, zero=False):
        phase = run/'controls'/name; phase.mkdir(mode=0o700)
        op = dict(id=name, argv=argv, cwd=str(D), environment=env, wallSeconds=15, keptBytesPerStream=1048576)
        r = take(op,phase); streams(r,phase,1048576,zero)
        if 'show' in argv: require(r['actual_exit'] in [0,1], 'Show exit')
        text = (phase/'stdout.bin').read_text()
        return r, dict(line.split('=',1) for line in text.splitlines() if '=' in line)
    scope_show = ['/usr/bin/systemctl','--user','show',SCOPE,'--no-pager','--property=LoadState','--property=ControlGroup','--property=Description']
    errors = []; outer = None; selected = None; elapsed = None; final = None
    try:
        _, s = control('00-scope',scope_show,True)
        require(s.get('LoadState') == 'loaded' and s.get('Description') == DESC and s.get('ControlGroup') == scope0['path'][len(str(CG)):], 'Scope identity')
        save(run/'SCOPE-BEFORE.json',scope0)
        started = time.clock_gettime_ns(time.CLOCK_BOOTTIME)
        _, absent = control('00-absence',card['controlShowArgv'])
        require(absent.get('LoadState') == 'not-found','Native unit already present')
        (run/'compile').mkdir(mode=0o700); (run/'outer-compile').mkdir(mode=0o700)
        save(run/'ROOT-SELECTION.json',dict(card=pin(CARD),receiver=selfpin,preissuePins=pre,scope=scope0,serviceArgv=card['serviceArgv'],outerCapture=card['outerCapture'],rootRlimits=limits,disk=disk0,masks=masks,hostMemoryDiagnosticOnly=host_diagnostics(),toolchainInternals='UNPINNED_NO_WHOLE_TOOLCHAIN_ABI_CLAIM',jobCapBytes=9663676416,privacy=card['privacyQualification'],whole400=card['whole400Qualification'],scopeOutside=card['rootScopeContract']['outsideQualified'],unitStateRule='Description and InvocationID locked; terminal empty ControlGroup allowed; not-found GC allowed; unknown owner untouched'))
        try:
            outer = take(card['outerCapture'],run/'outer-compile')
        except BaseException as e: errors.append('capture: '+repr(e))
        finally:
            try:
                _, state = control('01-after',card['controlShowArgv'])
                if state.get('LoadState') != 'not-found':
                    require(state.get('LoadState') == 'loaded','Unknown LoadState; untouched')
                    ident = state.get('InvocationID',''); group = state.get('ControlGroup','')
                    require(state.get('Description') == card['description'] and re.fullmatch('[0-9a-f]{32}',ident) and int(ident,16), 'Foreign or unknown unit; untouched')
                    terminal = state.get('ActiveState') in ['failed','inactive']
                    require(terminal or state.get('ActiveState') in ['active','activating','deactivating'], 'Unknown unit state')
                    require(group.endswith('/'+card['unit']) if group else terminal, 'Unit cgroup identity')
                    if group: require(not (CG/group.lstrip('/')).is_relative_to(Path(scope0['path'])), 'Native nested in Root scope')
                    before_path = run/'compile/CGROUP-BEFORE.json'
                    if group and before_path.exists(): require(str(CG/group.lstrip('/')) == json.loads(before_path.read_text())['path'], 'Native cgroup mismatch')
                    if not terminal: errors.append('Native was not terminal before cleanup')
                    stop, _ = control('02-stop',card['controlStopArgv'])
                    _, stopped = control('02-after-stop',card['controlShowArgv'])
                    require(stop['actual_exit'] == 0 or stopped.get('LoadState') == 'not-found' and 'not loaded' in (run/'controls/02-stop/stderr.bin').read_text(), 'Stop failed without GC')
                    if stopped.get('LoadState') != 'not-found':
                        require(stopped.get('Description') == card['description'] and stopped.get('InvocationID') == ident and stopped.get('ActiveState') in ['failed','inactive'] and stopped.get('ControlGroup','') in ['',group], 'Post-stop ownership drift; reset refused')
                        reset, _ = control('03-reset',card['controlResetArgv'])
                        if reset['actual_exit'] != 0:
                            require('not loaded' in (run/'controls/03-reset/stderr.bin').read_text(), 'Reset error')
                            _, gone = control('03-reset-gc',card['controlShowArgv'])
                            require(gone.get('LoadState') == 'not-found','Reset failed without GC')
            except BaseException as e: errors.append('cleanup: '+repr(e))
            finally:
                try:
                    _, final = control('04-final',card['controlShowArgv'])
                    require(final.get('LoadState') == 'not-found','Final unit remains')
                except BaseException as e: errors.append('final: '+repr(e))
                elapsed = (time.clock_gettime_ns(time.CLOCK_BOOTTIME)-started)/1e9
        try:
            post = readbacks(True); save(run/'POST-READBACK.json',dict(pins=post,disk=disk()))
            scope1 = own_scope(); no_oom(scope0,scope1); save(run/'SCOPE-AFTER-CLEANUP.json',scope1)
            bp = run/'compile/CGROUP-BEFORE.json'
            if bp.exists():
                cg = Path(json.loads(bp.read_text())['path'])
                require(cg.is_relative_to(CG) and cg.name == card['unit'] and cg.parent == Path(scope0['path']).parent, 'Native sibling')
                if cg.exists(): require(dict(x.split() for x in (cg/'cgroup.events').read_text().splitlines()).get('populated') == '0' and not (cg/'cgroup.procs').read_text().strip(), 'Native populated')
        except BaseException as e: errors.append('readback: '+repr(e))
        require(not errors, repr(errors)); require(elapsed <= 400,'whole400 exceeded')
        raw = json.loads((run/'compile/RAW-RESULT.json').read_text())
        before = json.loads((run/'compile/CGROUP-BEFORE.json').read_text()); after = raw['cgroupBeforeServiceExit']
        native = Path(before['path']); require(native.is_relative_to(CG) and native.name == card['unit'] and not native.is_relative_to(Path(scope0['path'])), 'Native sibling cgroup')
        require(before['files']['memory.max'].strip() == after['files']['memory.max'].strip() == '8589934592', 'Native memory cap')
        for sample0 in [before,after]:
            require(sample0['files']['cpu.max'].split() == ['200000','100000'] and sample0['files']['pids.max'].strip() == '1024', 'Native CPU/pids caps')
        no_oom(before,after,True)
        streams(outer,run/'outer-compile',4194304); streams(raw,run/'compile',16777216)
        require(raw['directChildrenAfterCapture'] == [],'Service children remain')
        text = (run/'outer-compile/stdout.bin').read_bytes()
        require(text.count(b'\n') == 1 and text.endswith(b'\n') and json.loads(text) == dict(subjectExit=0,complete=True),'Outer completion JSON')
        err = (run/'compile/stderr.bin').read_text()
        require(not re.search(r'(?im)\bE(?:0514|0460)\b|^error(?:\[|:|\s)',err),'Compiler error')
        for line in (run/'compile/stdout.bin').read_text().splitlines():
            m = json.loads(line); require(m.get('reason') != 'compiler-message' or m.get('message',{}).get('level') != 'error','Cargo compiler error')
        selector_ns = dict(Path=Path,json=json,stat=stat,hashlib=hashlib,re=re,cfg=cfg)
        check(card['selector'],mode=0o444)
        exec(compile(Path(card['selector']['path']).read_text(),card['selector']['path'],'exec'),selector_ns)
        selected = selector_ns['cargo_libtest_record'](raw)
    except BaseException as e: errors.append(repr(e))
    try:
        scope_end = own_scope(); no_oom(scope0,scope_end); save(run/'SCOPE-END.json',scope_end)
        require(not children(),'Root children remain')
    except BaseException as e: errors.append('Root completion: '+repr(e))
    metadata = None
    if selected is not None and not errors:
        save(run/'CURRENT-LIBTEST-METADATA.json',selected)
        metadata = pin(run/'CURRENT-LIBTEST-METADATA.json')
    result = dict(status='CURRENT_LIBTEST_METADATA_SELECTED' if metadata is not None else 'FAILED',currentLibtestMetadata=metadata,errors=errors,wholeSeconds=elapsed,finalUnit=final,receiver=selfpin,testsExecuted=0,SDKBody=False,nativeProof=False,financialAcceptance=False,fullGoalComplete=False,fullParentAnd35RoleAbi='UNADOPTED_UNSUPPORTED',W_D_and_verifiedChild='UNSUPPORTED',C13='STOPPED_UNCHANGED',noFitClaim=True,retentionSealingListing18Cases='LATER_SEPARATE_PREREQUISITES')
    save(run/'ROOT-RESULT.json',result); print(json.dumps(result),flush=True)
    return 0 if result['status'] == 'CURRENT_LIBTEST_METADATA_SELECTED' else 1

if __name__ == '__main__':
    sys.exit(main())
