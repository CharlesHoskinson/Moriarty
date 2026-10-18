#!/usr/bin/python3
"""Prospective human/operator provisioner and Q-OBS observer.

Source proposal only; grants no execution, provisioning, configuration or tier
authority. Never imports or invokes bounded_sdk_compile.py.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import stat
import subprocess
import time

OP0_MIN_MEM_AVAILABLE_BYTES = 23622320128
HOST_RESERVE_BYTES = 2147483648
OPERATOR_UID = 1000
OPERATOR_GID = 1000
GUARD_OUTER_MAXIMA = {
    'source_bytes': 1287099500,
    'packet_bytes': 67108864,
    'phase_F_memory_bytes': 8589934592,
    'phase_F_pids': 1024,
}
CONFIGURED_PHASE_LIMITS = {
    'source_bytes': 1287098368,
    'target_tmp_bytes': 8589934592,
    'packet_bytes': 67108864,
    'phase_F_memory_bytes': 8589934592,
    'phase_F_pids': 1024,
    'phase_F_cpu_quota_us': 400000,
    'phase_F_cpu_period_us': 100000,
}
OWNER_CEILINGS = {
    'source_bytes': CONFIGURED_PHASE_LIMITS['source_bytes'],
    'target_tmp_bytes': CONFIGURED_PHASE_LIMITS['target_tmp_bytes'],
    'packet_bytes': CONFIGURED_PHASE_LIMITS['packet_bytes'],
}
FORBIDDEN_HISTORICAL_PATHS = frozenset({
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/opus-sprint-loop-20261015/sdk-compile-host-grant-r1',
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/a102-compile-exec-r1',
})
FORBIDDEN_HISTORICAL_CGROUPS = frozenset({'/moriarty-a102-compile-r1'})
CHILD_KEYS = ('source_home', 'target', 'tmp', 'packet', 'empty_home', 'mask_empty')
CHILD_BASENAMES = {
    'source_home': 'source-home',
    'target': 'target',
    'tmp': 'tmp',
    'packet': 'packet',
    'empty_home': 'empty-home',
    'mask_empty': 'mask-empty',
}
CAP_FILES = ('memory.max', 'memory.swap.max', 'pids.max', 'cpu.max')
OPERATOR_TOOLS = ('/usr/bin/mount', '/usr/bin/umount', '/usr/bin/setpriv', '/usr/bin/env', '/usr/bin/bwrap', '/usr/bin/python3')
TEARDOWN_POLL_MAX_ATTEMPTS = 250
TEARDOWN_POLL_SLEEP_SECONDS = 0.02

class Refusal(RuntimeError):
    pass

def need(ok: bool, why: str) -> None:
    if not ok:
        raise Refusal(why)

def exact(obj: object, keys: str, where: str) -> None:
    need(type(obj) is dict and set(obj) == set(keys.split()), 'schema keys: ' + where)

def no_null(value: object, where: str = 'request') -> None:
    need(value is not None, 'unbound/null: ' + where)
    if isinstance(value, dict):
        for k, v in value.items():
            no_null(v, f'{where}.{k}')
    elif isinstance(value, list):
        for i, v in enumerate(value):
            no_null(v, f'{where}[{i}]')
    elif isinstance(value, str):
        need('\0' not in value and value != '' and not value.startswith(('UNBOUND', 'OBSERVATION_REQUIRED', 'PROPOSAL')), 'unbound string: ' + where)

def positive(value: object, where: str) -> int:
    need(type(value) is int and value > 0, 'positive int required: ' + where)
    return value

def page_aligned(value: object, where: str) -> int:
    n = positive(value, where)
    need(n % 4096 == 0, '4096-byte page alignment required: ' + where)
    return n

def absolute(value: object) -> Path:
    need(type(value) is str and value.startswith('/') and str(Path(value)) == value and re.fullmatch(r'(/[A-Za-z0-9._@-]+)+', value) is not None, 'normalized absolute path required')
    p = Path(value)
    need('..' not in p.parts and '\0' not in value, 'unsafe path')
    return p

def require_absent_nonsymlink(path: Path, where: str) -> None:
    need(not any(x.is_symlink() for x in path.parents), f'symlink in parent of {where}: {path}')
    need(not path.exists() and not path.is_symlink(), f'fresh {where} already exists: {path}')

def parse_cpu_max(value: object, where: str) -> tuple[int, int]:
    need(type(value) is str and re.fullmatch(r'[1-9][0-9]* [1-9][0-9]*', value) is not None, 'finite cpu.max required: ' + where)
    q, p = (int(x) for x in value.split())
    return q, p

def sha256_regular(path: str | Path) -> str:
    p = absolute(str(path))
    need(not any(x.is_symlink() for x in [p, *p.parents]), 'symlink in regular path: ' + str(p))
    st = os.lstat(p)
    need(stat.S_ISREG(st.st_mode) and not stat.S_ISLNK(st.st_mode), 'not regular file: ' + str(p))
    h = hashlib.sha256()
    fd = os.open(p, os.O_RDONLY | os.O_NOFOLLOW | getattr(os, 'O_CLOEXEC', 0))
    with os.fdopen(fd, 'rb') as f:
        fst = os.fstat(f.fileno())
        need(stat.S_ISREG(fst.st_mode) and fst.st_dev == st.st_dev and fst.st_ino == st.st_ino and fst.st_size == st.st_size, 'fd/lstat mismatch: ' + str(p))
        while chunk := f.read(1048576):
            h.update(chunk)
    return h.hexdigest()

def pin_operator_tool(path: str) -> dict:
    p = absolute(path)
    cur = p
    for _ in range(8):
        for anc in cur.parents:
            need(not anc.is_symlink(), 'symlink in tool parent path: ' + str(cur))
            ast = os.lstat(anc)
            need(stat.S_ISDIR(ast.st_mode) and ast.st_uid == 0 and ast.st_gid == 0 and (stat.S_IMODE(ast.st_mode) & 0o022) == 0 and (os.geteuid() == 0 or not os.access(anc, os.W_OK)), 'tool ancestor must be root:root non-writable dir: ' + str(anc))
        cst = os.lstat(cur)
        need(cst.st_uid == 0 and cst.st_gid == 0, 'tool entry must be root:root: ' + str(cur))
        if not stat.S_ISLNK(cst.st_mode):
            break
        target = os.readlink(cur)
        need('\0' not in target and target != '', 'invalid tool symlink target: ' + str(cur))
        cur = absolute(target) if target.startswith('/') else absolute(os.path.normpath(str(cur.parent / target)))
    else:
        raise Refusal('too many symlink hops in tool path: ' + str(p))
    rp = absolute(str(p.resolve(strict=True)))
    need(cur == rp, 'tool realpath resolution mismatch: ' + str(p))
    rst = os.lstat(rp)
    need(stat.S_ISREG(rst.st_mode) and not stat.S_ISLNK(rst.st_mode) and rst.st_uid == 0 and rst.st_gid == 0 and (stat.S_IMODE(rst.st_mode) & 0o022) == 0 and (os.geteuid() == 0 or not os.access(rp, os.W_OK)), 'resolved tool must be root:root non-writable regular file: ' + str(rp))
    if str(p) == '/usr/bin/bwrap':
        need((rst.st_mode & (stat.S_ISUID | stat.S_ISGID)) == 0, '/usr/bin/bwrap must be non-setuid/non-setgid')
    return {'path': str(p), 'realpath': str(rp), 'sha256': sha256_regular(rp)}

def jload_with_sha256(path: str | Path) -> tuple[dict, str]:
    p = absolute(str(path))
    need(not any(x.is_symlink() for x in [p, *p.parents]), 'symlink in regular path: ' + str(p))
    st = os.lstat(p)
    need(stat.S_ISREG(st.st_mode) and not stat.S_ISLNK(st.st_mode), 'not regular file: ' + str(p))
    def pairs(items):
        out = {}
        for k, v in items:
            need(k not in out, 'duplicate JSON key: ' + k)
            out[k] = v
        return out
    fd = os.open(p, os.O_RDONLY | os.O_NOFOLLOW | getattr(os, 'O_CLOEXEC', 0))
    with os.fdopen(fd, 'rb') as f:
        fst = os.fstat(f.fileno())
        need(stat.S_ISREG(fst.st_mode) and fst.st_dev == st.st_dev and fst.st_ino == st.st_ino and fst.st_size == st.st_size, 'fd/lstat mismatch: ' + str(p))
        raw = f.read()
    need(len(raw) == st.st_size, 'short/changed read on JSON file: ' + str(p))
    digest = hashlib.sha256(raw).hexdigest()
    doc = json.loads(raw.decode('utf-8'), object_pairs_hook=pairs)
    need(type(doc) is dict, 'JSON object required: ' + str(p))
    return doc, digest

def jload(path: str | Path) -> dict:
    doc, _ = jload_with_sha256(path)
    return doc

def fsync_dir(path: Path) -> None:
    fd = os.open(path, os.O_RDONLY | getattr(os, 'O_DIRECTORY', 0) | os.O_NOFOLLOW | getattr(os, 'O_CLOEXEC', 0))
    try:
        os.fsync(fd)
    finally:
        os.close(fd)

def write_new_json(path: Path, payload: dict) -> None:
    p = absolute(str(path))
    require_absent_nonsymlink(p, 'json_output')
    need(p.parent.exists() and not p.parent.is_symlink() and stat.S_ISDIR(p.parent.lstat().st_mode), 'json parent dir missing: ' + str(p))
    data = (json.dumps(payload, sort_keys=True, separators=(',', ':')) + '\n').encode('utf-8')
    fd = os.open(p, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | getattr(os, 'O_CLOEXEC', 0), 0o600)
    with os.fdopen(fd, 'wb') as f:
        f.write(data)
        f.flush()
        os.fsync(f.fileno())
    fsync_dir(p.parent)

def validate_request(req: dict) -> dict:
    exact(req, 'schema_version operator_uid operator_gid paths quota_mounts cgroup', 'request')
    no_null(req)
    need(req['schema_version'] == 1, 'request schema_version')
    need(req['operator_uid'] == OPERATOR_UID and req['operator_gid'] == OPERATOR_GID, 'operator uid/gid must equal 1000:1000')
    p = req['paths']
    exact(p, 'execution_root source_home target tmp packet empty_home mask_empty config_dir observation_output evidence_dir', 'paths')
    for k, v in p.items():
        absolute(v)
        need(not any(v == old or v.startswith(old + '/') or old.startswith(v + '/') for old in FORBIDDEN_HISTORICAL_PATHS), 'historical path reuse or ancestor overlap forbidden: ' + k)
    root = absolute(p['execution_root'])
    cfg_dir = absolute(p['config_dir'])
    obs_out = absolute(p['observation_output'])
    ev_dir = absolute(p['evidence_dir'])
    for k in CHILD_KEYS:
        ch = absolute(p[k])
        need(ch.parent == root and ch.name == CHILD_BASENAMES[k], 'canonical direct child of execution_root required: ' + k)
    for k in ('config_dir', 'observation_output', 'evidence_dir'):
        ak = absolute(p[k])
        need(ak != root and root not in ak.parents and ak not in root.parents, 'path must be disjoint from execution_root: ' + k)
    for other_k, other_p in (('config_dir', cfg_dir), ('observation_output', obs_out)):
        need(ev_dir != other_p and ev_dir not in other_p.parents and other_p not in ev_dir.parents, 'evidence_dir must be disjoint from ' + other_k)
    need(obs_out.parent == cfg_dir and obs_out.name != 'ONE_SHOT_STARTED', 'observation_output must be a direct file child of config_dir')
    need(len(set(p.values())) == len(p), 'overlapping request paths')
    qm = req['quota_mounts']
    exact(qm, 'source_home target_tmp packet', 'quota_mounts')
    for k, expected_mp, ceil_key in (
        ('source_home', p['source_home'], 'source_bytes'),
        ('target_tmp', p['execution_root'], 'target_tmp_bytes'),
        ('packet', p['packet'], 'packet_bytes'),
    ):
        q = qm[k]
        exact(q, 'mountpoint max_bytes max_inodes', f'quota_mounts.{k}')
        need(q['mountpoint'] == expected_mp, f'quota mountpoint mismatch: {k}')
        mb = page_aligned(q['max_bytes'], f'{k}.max_bytes')
        need(mb <= OWNER_CEILINGS[ceil_key] and mb <= GUARD_OUTER_MAXIMA.get(ceil_key, OWNER_CEILINGS[ceil_key]), f'quota exceeds owner ceiling: {k}')
        positive(q['max_inodes'], f'{k}.max_inodes')
    cg = req['cgroup']
    exact(cg, 'envelope_path supervisor_leaf_name phase_parent_name envelope_memory_max envelope_pids_max envelope_cpu_max supervisor_leaf_limits phase_parent_limits', 'cgroup')
    ep = absolute(cg['envelope_path'])
    need(cg['envelope_path'] not in FORBIDDEN_HISTORICAL_CGROUPS, 'historical cgroup path forbidden')
    need(re.fullmatch(r'/[A-Za-z0-9_][A-Za-z0-9_.@-]{0,127}', cg['envelope_path']) is not None and not ep.name.startswith('cgroup.'), 'top-level non-user cgroup envelope_path required')
    for k in ('supervisor_leaf_name', 'phase_parent_name'):
        name = cg[k]
        need(type(name) is str and re.fullmatch(r'[a-z][a-z0-9-]{0,62}', name) is not None and not name.startswith('sdk-once-'), 'invalid cgroup child name: ' + k)
    need(cg['supervisor_leaf_name'] != cg['phase_parent_name'], 'cgroup child names must differ')
    e_mem = page_aligned(cg['envelope_memory_max'], 'envelope_memory_max')
    e_pids = positive(cg['envelope_pids_max'], 'envelope_pids_max')
    e_q, e_p = parse_cpu_max(cg['envelope_cpu_max'], 'envelope_cpu_max')
    need(e_mem + HOST_RESERVE_BYTES <= OP0_MIN_MEM_AVAILABLE_BYTES, 'envelope memory plus 2 GiB reserve exceeds OP-0 gate')
    for sub in ('supervisor_leaf_limits', 'phase_parent_limits'):
        exact(cg[sub], 'memory_max pids_max cpu_max', f'cgroup.{sub}')
    l_mem = page_aligned(cg['supervisor_leaf_limits']['memory_max'], 'L.memory_max')
    l_pids = positive(cg['supervisor_leaf_limits']['pids_max'], 'L.pids_max')
    l_q, l_p = parse_cpu_max(cg['supervisor_leaf_limits']['cpu_max'], 'L.cpu_max')
    p_mem = page_aligned(cg['phase_parent_limits']['memory_max'], 'P.memory_max')
    p_pids = positive(cg['phase_parent_limits']['pids_max'], 'P.pids_max')
    p_q, p_p = parse_cpu_max(cg['phase_parent_limits']['cpu_max'], 'P.cpu_max')
    need(e_p == l_p == p_p, 'E/L/P cpu.max periods must match')
    need(l_mem + p_mem <= e_mem, 'L + P memory.max exceeds E memory.max')
    need(l_pids + p_pids == e_pids, 'L + P pids.max must equal E pids.max')
    need(l_q + p_q == e_q, 'L + P cpu quota must equal E cpu quota')
    tmpfs_sum = sum(qm[k]['max_bytes'] for k in ('source_home', 'target_tmp', 'packet'))
    need(tmpfs_sum + CONFIGURED_PHASE_LIMITS['phase_F_memory_bytes'] <= p_mem, 'three tmpfs quotas plus phase F memory_bytes exceed P memory.max')
    need(p_pids >= CONFIGURED_PHASE_LIMITS['phase_F_pids'], 'P pids.max below phase F pids ceiling')
    need(p_q * CONFIGURED_PHASE_LIMITS['phase_F_cpu_period_us'] >= CONFIGURED_PHASE_LIMITS['phase_F_cpu_quota_us'] * p_p, 'P cpu.max below phase F cpu quota')
    return req

def require_unprivileged_operator() -> None:
    uid = os.getuid()
    gid = os.getgid()
    need(uid == os.geteuid() == OPERATOR_UID and gid == os.getegid() == OPERATOR_GID, 'must run as unprivileged uid=1000 gid=1000')
    status = dict(line.split(':', 1) for line in Path('/proc/self/status').read_text(encoding='utf-8').splitlines() if ':' in line)
    for k in ('CapPrm', 'CapEff', 'CapAmb'):
        need(re.fullmatch(r'0+', status.get(k, '').strip()) is not None, 'nonzero capability: ' + k)

def require_root_operator() -> None:
    need(os.getuid() == 0 and os.geteuid() == 0, 'must run as root operator (uid=0 euid=0)')

def cg_paths(req: dict) -> tuple[Path, Path, Path]:
    cg = req['cgroup']
    base = Path('/sys/fs/cgroup') / cg['envelope_path'].lstrip('/')
    return base, base / cg['supervisor_leaf_name'], base / cg['phase_parent_name']

def parse_mountinfo() -> list[tuple[list[str], list[str]]]:
    rows = []
    for line in Path('/proc/self/mountinfo').read_text(encoding='utf-8').splitlines():
        pre, post = line.split(' - ', 1)
        rows.append((pre.split(), post.split()))
    return rows

def check_host_predicates() -> dict:
    meminfo = {}
    for line in Path('/proc/self/meminfo' if Path('/proc/self/meminfo').exists() else '/proc/meminfo').read_text(encoding='utf-8').splitlines():
        if ':' in line:
            k, v = line.split(':', 1)
            parts = v.strip().split()
            if parts and parts[0].isdigit():
                meminfo[k] = int(parts[0]) * (1024 if parts[-1:] == ['kB'] else 1)
    mem_avail = meminfo.get('MemAvailable', 0)
    need(mem_avail >= OP0_MIN_MEM_AVAILABLE_BYTES, f'OP-0 MemAvailable {mem_avail} < {OP0_MIN_MEM_AVAILABLE_BYTES}')
    rows = parse_mountinfo()
    cgroups = [(a[4], b[0]) for a, b in rows if b[0] == 'cgroup2']
    need(cgroups == [('/sys/fs/cgroup', 'cgroup2')] and sum(a[4] == '/sys/fs/cgroup' for a, _ in rows) == 1, 'single global cgroup2 mount required')
    need(not any(a[4].startswith('/sys/fs/cgroup/') for a, _ in rows), 'nested mount under /sys/fs/cgroup')
    cg_root = Path('/sys/fs/cgroup')
    st = cg_root.lstat()
    need(stat.S_ISDIR(st.st_mode) and not stat.S_ISLNK(st.st_mode) and cg_root.resolve(strict=True) == cg_root and st.st_uid == 0 and st.st_gid == 0 and not os.access(cg_root, os.W_OK), 'invalid /sys/fs/cgroup root ownership/identity')
    root_procs = cg_root / 'cgroup.procs'
    rp_st = root_procs.lstat()
    need(stat.S_ISREG(rp_st.st_mode) and not stat.S_ISLNK(rp_st.st_mode) and rp_st.st_uid == 0 and rp_st.st_gid == 0 and not os.access(root_procs, os.W_OK), 'root cgroup.procs must be root-owned non-writable')
    for fname in ('cgroup.controllers', 'cgroup.subtree_control'):
        ctrls = set((cg_root / fname).read_text(encoding='utf-8').split())
        need({'cpu', 'memory', 'pids'} <= ctrls, f'/sys/fs/cgroup/{fname} missing cpu/memory/pids')
    self_cg = Path('/proc/self/cgroup').read_text(encoding='utf-8').splitlines()
    need(len(self_cg) == 1 and self_cg[0].startswith('0::/'), 'single-line cgroup v2 membership required')
    rel_cg = self_cg[0][4:].lstrip('/')
    if rel_cg:
        need(re.fullmatch(r'[A-Za-z0-9_.@:-]+(/[A-Za-z0-9_.@:-]+)*', rel_cg) is not None and '..' not in Path(rel_cg).parts, 'invalid self cgroup path')
        probe_cg = cg_root / rel_cg
        need(str(probe_cg) == '/sys/fs/cgroup/' + rel_cg and cg_root in probe_cg.parents, 'self cgroup outside /sys/fs/cgroup')
    else:
        probe_cg = next((x for x in sorted(cg_root.iterdir(), key=lambda p: p.name) if not x.is_symlink() and x.is_dir()), None)
        need(probe_cg is not None, 'non-root cgroup required to observe cgroup.kill')
    need(not any(x.is_symlink() for x in [probe_cg, *probe_cg.parents]), 'symlink in non-root cgroup path')
    need((probe_cg / 'cgroup.kill').exists() and (probe_cg / 'cgroup.events').exists(), 'non-root cgroup.kill/cgroup.events missing')
    bwrap = Path('/usr/bin/bwrap')
    bst = bwrap.lstat()
    if stat.S_ISLNK(bst.st_mode):
        need(bst.st_uid == 0 and bst.st_gid == 0, '/usr/bin/bwrap symlink must be root:root')
        bwrap = absolute(str(bwrap.resolve(strict=True)))
        bst = bwrap.lstat()
    need(stat.S_ISREG(bst.st_mode) and bst.st_uid == 0 and bst.st_gid == 0 and (stat.S_IMODE(bst.st_mode) & 0o022) == 0 and (bst.st_mode & (stat.S_ISUID | stat.S_ISGID)) == 0, '/usr/bin/bwrap must be regular non-setuid/non-setgid')
    max_userns_path = Path('/proc/sys/user/max_user_namespaces')
    if max_userns_path.exists():
        need(int(max_userns_path.read_text(encoding='utf-8').strip()) > 0, 'user.max_user_namespaces <= 0')
    return {'mem_available_bytes': mem_avail, 'self_cgroup': self_cg[0]}

def cmd_op0(req: dict) -> dict:
    require_unprivileged_operator()
    facts = check_host_predicates()
    root = absolute(req['paths']['execution_root'])
    cfg_dir = absolute(req['paths']['config_dir'])
    obs_out = absolute(req['paths']['observation_output'])
    ev_dir = absolute(req['paths']['evidence_dir'])
    e_dir, _, _ = cg_paths(req)
    require_absent_nonsymlink(root, 'execution_root')
    require_absent_nonsymlink(cfg_dir, 'config_dir')
    require_absent_nonsymlink(obs_out, 'observation_output')
    require_absent_nonsymlink(ev_dir, 'evidence_dir')
    require_absent_nonsymlink(e_dir, 'cgroup envelope_path')
    return {'step': 'OP-0', 'status': 'OK', **facts}

def cmd_op1(req: dict) -> dict:
    require_unprivileged_operator()
    root = absolute(req['paths']['execution_root'])
    cfg_dir = absolute(req['paths']['config_dir'])
    require_absent_nonsymlink(root, 'execution_root')
    require_absent_nonsymlink(cfg_dir, 'config_dir')
    os.mkdir(root, 0o700)
    os.chmod(root, 0o700)
    os.mkdir(cfg_dir, 0o755)
    os.chmod(cfg_dir, 0o755)
    return {'step': 'OP-1', 'status': 'OK', 'execution_root': str(root), 'config_dir': str(cfg_dir)}

def mount_flags(q: dict, noexec: bool) -> str:
    opts = ['rw', 'nosuid', 'nodev']
    if noexec:
        opts.append('noexec')
    opts.extend([
        'huge=never',
        f"size={q['max_bytes']}",
        f"nr_inodes={q['max_inodes']}",
        'mode=0700',
        f'uid={OPERATOR_UID}',
        f'gid={OPERATOR_GID}',
    ])
    return ','.join(opts)

def run_mount_tmpfs(mountpoint: Path, flags: str) -> None:
    need(not any(x.is_symlink() for x in [mountpoint, *mountpoint.parents]), 'symlink mountpoint path')
    need(not any(a[4] == str(mountpoint) for a, _ in parse_mountinfo()), f'mountpoint already mounted: {mountpoint}')
    st = mountpoint.lstat()
    need(stat.S_ISDIR(st.st_mode) and st.st_uid == OPERATOR_UID and st.st_gid == OPERATOR_GID and stat.S_IMODE(st.st_mode) == 0o700, 'mountpoint must be 1000:1000 0700 empty dir')
    need(not any(mountpoint.iterdir()), 'mountpoint not empty')
    mount_bin = pin_operator_tool('/usr/bin/mount')['realpath']
    subprocess.run([mount_bin, '-t', 'tmpfs', '-o', flags, 'tmpfs', str(mountpoint)], check=True, shell=False)
    subprocess.run([mount_bin, '--make-private', str(mountpoint)], check=True, shell=False)

def cmd_op2(req: dict) -> dict:
    require_root_operator()
    root = absolute(req['paths']['execution_root'])
    run_mount_tmpfs(root, mount_flags(req['quota_mounts']['target_tmp'], noexec=False))
    return {'step': 'OP-2', 'status': 'OK', 'mounted': str(root)}

def cmd_op3(req: dict) -> dict:
    require_unprivileged_operator()
    root = absolute(req['paths']['execution_root'])
    need(not any(x.is_symlink() for x in [root, *root.parents]), 'symlink in execution_root path')
    root_matches = [(a, b) for a, b in parse_mountinfo() if a[4] == str(root)]
    need(len(root_matches) == 1 and root_matches[0][1][0] == 'tmpfs', 'execution_root must be an active single tmpfs mount before OP-3')
    for k in CHILD_KEYS:
        ch = absolute(req['paths'][k])
        need(ch.parent == root and ch.name == CHILD_BASENAMES[k], 'child path invalid: ' + k)
        require_absent_nonsymlink(ch, k)
        os.mkdir(ch, 0o700)
        os.chmod(ch, 0o700)
    return {'step': 'OP-3', 'status': 'OK', 'children': [req['paths'][k] for k in CHILD_KEYS]}

def cmd_op4(req: dict) -> dict:
    require_root_operator()
    root = absolute(req['paths']['execution_root'])
    need(not any(x.is_symlink() for x in [root, *root.parents]), 'symlink in execution_root path')
    root_matches = [(a, b) for a, b in parse_mountinfo() if a[4] == str(root)]
    need(len(root_matches) == 1 and root_matches[0][1][0] == 'tmpfs', 'execution_root must be an active single tmpfs mount before OP-4')
    root_dev = root.lstat().st_dev
    for k in ('source_home', 'packet'):
        mp = absolute(req['paths'][k])
        need(mp.parent == root and mp.lstat().st_dev == root_dev, f'{k} must reside on execution_root tmpfs before OP-4 mount')
        run_mount_tmpfs(mp, mount_flags(req['quota_mounts'][k], noexec=True))
    return {'step': 'OP-4', 'status': 'OK', 'mounted': [req['paths']['source_home'], req['paths']['packet']]}

def write_cg_checked(path: Path, val: str, expected_readback: str | None = None) -> None:
    path.write_text(val, encoding='utf-8')
    if expected_readback is not None:
        need(path.read_text(encoding='utf-8').strip() == expected_readback, f'cgroup readback mismatch on {path}')

def lock_root_cap_files(cg_dir: Path) -> None:
    for name in (*CAP_FILES, 'cgroup.procs', 'cgroup.threads', 'cgroup.subtree_control'):
        f = cg_dir / name
        os.chown(f, 0, 0, follow_symlinks=False)
        os.chmod(f, 0o644)

def cmd_op5(req: dict) -> dict:
    require_root_operator()
    rows = parse_mountinfo()
    cgroups = [(a[4], b[0]) for a, b in rows if b[0] == 'cgroup2']
    need(cgroups == [('/sys/fs/cgroup', 'cgroup2')] and sum(a[4] == '/sys/fs/cgroup' for a, _ in rows) == 1 and not any(a[4].startswith('/sys/fs/cgroup/') for a, _ in rows), 'single global cgroup2 mount required for OP-5')
    cg = req['cgroup']
    e_dir, l_dir, p_dir = cg_paths(req)
    need(e_dir.parent == Path('/sys/fs/cgroup'), 'E must be directly under /sys/fs/cgroup')
    require_absent_nonsymlink(e_dir, 'cgroup envelope_path')
    os.mkdir(e_dir, 0o755)
    os.chown(e_dir, 0, 0, follow_symlinks=False)
    os.chmod(e_dir, 0o755)
    write_cg_checked(e_dir / 'memory.max', str(cg['envelope_memory_max']), str(cg['envelope_memory_max']))
    write_cg_checked(e_dir / 'memory.swap.max', '0', '0')
    write_cg_checked(e_dir / 'pids.max', str(cg['envelope_pids_max']), str(cg['envelope_pids_max']))
    write_cg_checked(e_dir / 'cpu.max', cg['envelope_cpu_max'], cg['envelope_cpu_max'])
    write_cg_checked(e_dir / 'cgroup.subtree_control', '+cpu +memory +pids')
    lock_root_cap_files(e_dir)
    os.chown(e_dir / 'cgroup.procs', OPERATOR_UID, OPERATOR_GID, follow_symlinks=False)
    os.chmod(e_dir / 'cgroup.procs', 0o644)
    os.mkdir(l_dir, 0o755)
    os.chown(l_dir, 0, 0, follow_symlinks=False)
    os.chmod(l_dir, 0o755)
    ll = cg['supervisor_leaf_limits']
    write_cg_checked(l_dir / 'memory.max', str(ll['memory_max']), str(ll['memory_max']))
    write_cg_checked(l_dir / 'memory.swap.max', '0', '0')
    write_cg_checked(l_dir / 'pids.max', str(ll['pids_max']), str(ll['pids_max']))
    write_cg_checked(l_dir / 'cpu.max', ll['cpu_max'], ll['cpu_max'])
    lock_root_cap_files(l_dir)
    os.mkdir(p_dir, 0o755)
    os.chown(p_dir, 0, 0, follow_symlinks=False)
    os.chmod(p_dir, 0o755)
    pl = cg['phase_parent_limits']
    write_cg_checked(p_dir / 'memory.max', str(pl['memory_max']), str(pl['memory_max']))
    write_cg_checked(p_dir / 'memory.swap.max', '0', '0')
    write_cg_checked(p_dir / 'pids.max', str(pl['pids_max']), str(pl['pids_max']))
    write_cg_checked(p_dir / 'cpu.max', pl['cpu_max'], pl['cpu_max'])
    write_cg_checked(p_dir / 'cgroup.subtree_control', '+cpu +memory +pids')
    lock_root_cap_files(p_dir)
    for target in (p_dir, p_dir / 'cgroup.procs', p_dir / 'cgroup.threads', p_dir / 'cgroup.subtree_control'):
        os.chown(target, OPERATOR_UID, OPERATOR_GID, follow_symlinks=False)
    os.chmod(p_dir, 0o755)
    for target in (p_dir / 'cgroup.procs', p_dir / 'cgroup.threads', p_dir / 'cgroup.subtree_control'):
        os.chmod(target, 0o644)
    return {'step': 'OP-5', 'status': 'OK', 'E': str(e_dir), 'L': str(l_dir), 'P': str(p_dir)}

def default_request_sha256(req: dict) -> str:
    return hashlib.sha256((json.dumps(req, sort_keys=True, separators=(',', ':')) + '\n').encode('utf-8')).hexdigest()

def default_provisioner_sha256() -> str:
    fp = Path(__file__)
    if not str(fp).startswith('/'):
        fp = fp.resolve(strict=True)
    return sha256_regular(fp)

def verify_and_observe(req: dict, request_sha256: str | None = None, provisioner_sha256: str | None = None) -> dict:
    require_unprivileged_operator()
    req_sha = request_sha256 if request_sha256 is not None else default_request_sha256(req)
    prov_sha = provisioner_sha256 if provisioner_sha256 is not None else default_provisioner_sha256()
    host_facts = check_host_predicates()
    p = req['paths']
    root = absolute(p['execution_root'])
    for k in ('execution_root', *CHILD_KEYS):
        pk = absolute(p[k])
        need(pk.resolve(strict=True) == pk and not any(x.is_symlink() for x in [pk, *pk.parents]), 'symlink or resolution mismatch: ' + k)
        st = pk.lstat()
        need(stat.S_ISDIR(st.st_mode) and st.st_uid == OPERATOR_UID and st.st_gid == OPERATOR_GID and stat.S_IMODE(st.st_mode) == 0o700, 'owned 1000:1000 0700 dir required: ' + k)
        if k != 'execution_root':
            need(not any(pk.iterdir()), 'child dir not empty: ' + k)
    root_dev = root.lstat().st_dev
    for k in ('target', 'tmp', 'empty_home', 'mask_empty'):
        need(absolute(p[k]).lstat().st_dev == root_dev, 'child escaped target_tmp mount device: ' + k)
    rows = parse_mountinfo()
    q_obs = {}
    devices = set()
    mount_ids = set()
    for k, noexec in (('target_tmp', False), ('source_home', True), ('packet', True)):
        q = req['quota_mounts'][k]
        mp = q['mountpoint']
        matches = [(a, b) for a, b in rows if a[4] == mp]
        need(len(matches) == 1, f'exact single mountinfo row required for {k}')
        a, b = matches[0]
        mid = int(a[0])
        dev = a[2]
        opts = set(a[5].split(','))
        need(a[3] == '/' and not any(x.startswith(('shared:', 'master:', 'propagate_from:')) for x in a[6:]), f'private mount root required for {k}')
        need(b[0] == 'tmpfs' and {'rw', 'nosuid', 'nodev'} <= opts, f'tmpfs mount options invalid for {k}')
        need(('noexec' in opts) == noexec, f'noexec mismatch for {k}')
        need(len(b) > 2, f'mountinfo superblock options missing for {k}')
        sb_opts = set(b[2].split(','))
        need({f'uid={OPERATOR_UID}', f'gid={OPERATOR_GID}'} <= sb_opts and ('mode=700' in sb_opts or 'mode=0700' in sb_opts), f'tmpfs superblock uid/gid/mode mismatch for {k}')
        vfs = os.statvfs(mp)
        need(vfs.f_blocks * vfs.f_frsize == q['max_bytes'] and vfs.f_files == q['max_inodes'], f'statvfs bound mismatch for {k}')
        devices.add(dev)
        mount_ids.add(mid)
        q_obs[k] = {
            'mountpoint': mp,
            'mount_id': mid,
            'device': dev,
            'max_bytes': q['max_bytes'],
            'max_inodes': q['max_inodes'],
        }
    need(len(devices) == 3 and len(mount_ids) == 3, 'three quota mounts must have distinct mount_ids and devices')
    allowed_nested = {p['source_home'], p['packet']}
    for a, _ in rows:
        if a[4].startswith(p['execution_root'].rstrip('/') + '/'):
            need(a[4] in allowed_nested, 'unbound nested mount under execution_root: ' + a[4])
    cg = req['cgroup']
    e_dir, l_dir, p_dir = cg_paths(req)
    cg_root = Path('/sys/fs/cgroup')
    cg_root_dev = cg_root.lstat().st_dev
    need(not os.access(cg_root, os.W_OK) and not os.access(cg_root / 'cgroup.procs', os.W_OK), '/sys/fs/cgroup and root cgroup.procs must not be writable')
    for node in (e_dir, l_dir, p_dir):
        st = node.lstat()
        need(stat.S_ISDIR(st.st_mode) and not stat.S_ISLNK(st.st_mode) and node.resolve(strict=True) == node and st.st_dev == cg_root_dev, f'cgroup dir identity: {node}')
        need((node / 'cgroup.type').read_text(encoding='utf-8').strip() == 'domain', f'cgroup domain required: {node}')
        need(not (node / 'cgroup.procs').read_text(encoding='utf-8').splitlines(), f'cgroup populated prior to launch: {node}')
        need((node / 'cgroup.kill').exists() and (node / 'cgroup.events').exists(), f'cgroup.kill/cgroup.events missing: {node}')
        ev = dict(line.split() for line in (node / 'cgroup.events').read_text(encoding='utf-8').splitlines() if len(line.split()) == 2)
        need(ev.get('populated') == '0', f'cgroup.events populated != 0: {node}')
    for anc in (e_dir, l_dir):
        st = anc.lstat()
        need(st.st_uid == 0 and st.st_gid == 0 and stat.S_IMODE(st.st_mode) == 0o755 and not os.access(anc, os.W_OK), f'root-owned 0755 non-writable dir required: {anc}')
    pst = p_dir.lstat()
    need(pst.st_uid == OPERATOR_UID and pst.st_gid == OPERATOR_GID and stat.S_IMODE(pst.st_mode) == 0o755 and os.access(p_dir, os.W_OK | os.X_OK), 'P must be 1000:1000 0755 writable/searchable')
    need({x.name for x in e_dir.iterdir() if x.is_dir()} == {cg['supervisor_leaf_name'], cg['phase_parent_name']}, 'E children must equal {L, P}')
    need(not any(x.is_dir() for x in l_dir.iterdir()), 'L must have no child cgroups')
    need(not any(x.is_dir() for x in p_dir.iterdir()), 'P must have no residual phase cgroups')
    need({'cpu', 'memory', 'pids'} <= set((e_dir / 'cgroup.subtree_control').read_text(encoding='utf-8').split()), 'E subtree_control missing cpu/memory/pids')
    need(not (l_dir / 'cgroup.subtree_control').read_text(encoding='utf-8').split(), 'L subtree_control must be empty')
    for fname in ('cgroup.controllers', 'cgroup.subtree_control'):
        need({'cpu', 'memory', 'pids'} <= set((p_dir / fname).read_text(encoding='utf-8').split()), f'P {fname} missing cpu/memory/pids')
    expected_caps = [
        (e_dir, {'memory.max': str(cg['envelope_memory_max']), 'memory.swap.max': '0', 'pids.max': str(cg['envelope_pids_max']), 'cpu.max': cg['envelope_cpu_max']}),
        (l_dir, {'memory.max': str(cg['supervisor_leaf_limits']['memory_max']), 'memory.swap.max': '0', 'pids.max': str(cg['supervisor_leaf_limits']['pids_max']), 'cpu.max': cg['supervisor_leaf_limits']['cpu_max']}),
        (p_dir, {'memory.max': str(cg['phase_parent_limits']['memory_max']), 'memory.swap.max': '0', 'pids.max': str(cg['phase_parent_limits']['pids_max']), 'cpu.max': cg['phase_parent_limits']['cpu_max']}),
    ]
    for node, kv in expected_caps:
        for k, val in kv.items():
            fp = node / k
            need(fp.read_text(encoding='utf-8').strip() == val, f'cap value mismatch on {fp}')
            fst = fp.lstat()
            need(fst.st_uid == 0 and fst.st_gid == 0 and stat.S_IMODE(fst.st_mode) == 0o644 and not os.access(fp, os.W_OK), f'cap file must be root:root 0644 non-writable: {fp}')
    delegated_files = {e_dir / 'cgroup.procs', p_dir / 'cgroup.procs', p_dir / 'cgroup.threads', p_dir / 'cgroup.subtree_control'}
    for rfile in (e_dir / 'cgroup.subtree_control', e_dir / 'cgroup.threads', l_dir / 'cgroup.procs', l_dir / 'cgroup.threads', l_dir / 'cgroup.subtree_control'):
        rst = rfile.lstat()
        need(rst.st_uid == 0 and rst.st_gid == 0 and not os.access(rfile, os.W_OK), f'control file must stay root-owned non-writable: {rfile}')
    for ufile in delegated_files:
        ust = ufile.lstat()
        need(ust.st_uid == OPERATOR_UID and ust.st_gid == OPERATOR_GID and stat.S_IMODE(ust.st_mode) == 0o644 and os.access(ufile, os.W_OK), f'delegated file must be 1000:1000 0644 writable: {ufile}')
    node_readbacks = {}
    for label, node in (('E', e_dir), ('L', l_dir), ('P', p_dir)):
        for entry in sorted(node.iterdir(), key=lambda x: x.name):
            est = entry.lstat()
            need(not stat.S_ISLNK(est.st_mode) and est.st_dev == cg_root_dev, f'invalid cgroup entry identity: {entry}')
            if entry.is_dir():
                continue
            need(stat.S_ISREG(est.st_mode), f'non-regular cgroup entry: {entry}')
            if entry not in delegated_files:
                need(est.st_uid == 0 and est.st_gid == 0 and (stat.S_IMODE(est.st_mode) & 0o022) == 0 and not os.access(entry, os.W_OK), f'non-delegated cgroup file must be root:root non-writable: {entry}')
        nst = node.lstat()
        node_readbacks[label] = {
            'path': str(node),
            'uid': nst.st_uid,
            'gid': nst.st_gid,
            'mode': stat.S_IMODE(nst.st_mode),
            'cgroup_type': (node / 'cgroup.type').read_text(encoding='utf-8').strip(),
            'controllers': sorted((node / 'cgroup.controllers').read_text(encoding='utf-8').split()),
            'subtree_control': sorted((node / 'cgroup.subtree_control').read_text(encoding='utf-8').split()),
            'procs': (node / 'cgroup.procs').read_text(encoding='utf-8').splitlines(),
            'populated': int(dict(line.split() for line in (node / 'cgroup.events').read_text(encoding='utf-8').splitlines() if len(line.split()) == 2)['populated']),
            'caps': {k: (node / k).read_text(encoding='utf-8').strip() for k in CAP_FILES},
        }
    tool_pins = {t: pin_operator_tool(t) for t in OPERATOR_TOOLS}
    return {
        'schema': 'sherlock-operator-provisioning-observation/1',
        'request_sha256': req_sha,
        'provisioner_sha256': prov_sha,
        'op0_facts': host_facts,
        'quota_mounts': q_obs,
        'cgroup_verified': {
            'envelope_path': cg['envelope_path'],
            'supervisor_leaf_name': cg['supervisor_leaf_name'],
            'phase_parent_name': cg['phase_parent_name'],
            'envelope_memory_max': int(node_readbacks['E']['caps']['memory.max']),
            'envelope_memory_swap_max': int(node_readbacks['E']['caps']['memory.swap.max']),
            'envelope_pids_max': int(node_readbacks['E']['caps']['pids.max']),
            'envelope_cpu_max': node_readbacks['E']['caps']['cpu.max'],
            'supervisor_leaf_limits': {
                'memory_max': int(node_readbacks['L']['caps']['memory.max']),
                'memory_swap_max': int(node_readbacks['L']['caps']['memory.swap.max']),
                'pids_max': int(node_readbacks['L']['caps']['pids.max']),
                'cpu_max': node_readbacks['L']['caps']['cpu.max'],
            },
            'phase_parent_limits': {
                'memory_max': int(node_readbacks['P']['caps']['memory.max']),
                'memory_swap_max': int(node_readbacks['P']['caps']['memory.swap.max']),
                'pids_max': int(node_readbacks['P']['caps']['pids.max']),
                'cpu_max': node_readbacks['P']['caps']['cpu.max'],
            },
            'nodes': node_readbacks,
            'k_invariants': {'K1': True, 'K2': True, 'K3': True},
        },
        'operator_tool_sha256': tool_pins,
    }

def cmd_op6(req: dict, request_sha256: str | None = None, provisioner_sha256: str | None = None) -> dict:
    obs = verify_and_observe(req, request_sha256=request_sha256, provisioner_sha256=provisioner_sha256)
    out_path = absolute(req['paths']['observation_output'])
    write_new_json(out_path, obs)
    return {'step': 'OP-6', 'status': 'OK', 'observation_output': str(out_path), 'observation_sha256': sha256_regular(out_path)}

def cmd_prelaunch_verify(req: dict, request_sha256: str | None = None, provisioner_sha256: str | None = None) -> dict:
    current = verify_and_observe(req, request_sha256=request_sha256, provisioner_sha256=provisioner_sha256)
    obs_path = absolute(req['paths']['observation_output'])
    recorded, obs_sha = jload_with_sha256(obs_path)
    need(recorded.get('schema') == current['schema'], 'observation schema mismatch in prelaunch-verify')
    for k in ('request_sha256', 'provisioner_sha256', 'quota_mounts', 'cgroup_verified', 'operator_tool_sha256'):
        need(recorded.get(k) == current[k], f'prelaunch-verify drift vs OP-6 observation on {k}')
    rec_op0 = recorded.get('op0_facts')
    need(type(rec_op0) is dict and type(rec_op0.get('mem_available_bytes')) is int and rec_op0['mem_available_bytes'] >= OP0_MIN_MEM_AVAILABLE_BYTES and type(rec_op0.get('self_cgroup')) is str and rec_op0['self_cgroup'].startswith('0::/'), 'invalid op0_facts in recorded OP-6 observation')
    return {
        'step': 'PRELAUNCH-VERIFY',
        'status': 'OK',
        'observation_output': str(obs_path),
        'observation_sha256': obs_sha,
        'request_sha256': current['request_sha256'],
        'provisioner_sha256': current['provisioner_sha256'],
        'mem_available_bytes': current['op0_facts']['mem_available_bytes'],
    }

def copy_regular_verified(src: Path, dst: Path, expected_sha256: str) -> str:
    shutil.copy2(src, dst, follow_symlinks=False)
    fd = os.open(dst, os.O_RDONLY | os.O_NOFOLLOW | getattr(os, 'O_CLOEXEC', 0))
    try:
        os.fsync(fd)
    finally:
        os.close(fd)
    dst_sha = sha256_regular(dst)
    need(dst_sha == expected_sha256, f'copied evidence digest mismatch on {dst}')
    return dst_sha

def cmd_collect_evidence(req: dict, request_sha256: str | None = None, provisioner_sha256: str | None = None) -> dict:
    require_unprivileged_operator()
    req_sha = request_sha256 if request_sha256 is not None else default_request_sha256(req)
    prov_sha = provisioner_sha256 if provisioner_sha256 is not None else default_provisioner_sha256()
    root = absolute(req['paths']['execution_root'])
    pkt = absolute(req['paths']['packet'])
    obs_path = absolute(req['paths']['observation_output'])
    ev_dir = absolute(req['paths']['evidence_dir'])
    need(not any(x.is_symlink() for x in [root, *root.parents, pkt, *pkt.parents]), 'symlink in execution_root/packet path')
    obs_doc, obs_sha = jload_with_sha256(obs_path)
    need(obs_doc.get('schema') == 'sherlock-operator-provisioning-observation/1' and obs_doc.get('request_sha256') == req_sha and obs_doc.get('provisioner_sha256') == prov_sha, 'observation provenance mismatch in collect-evidence')
    obs_qm = obs_doc.get('quota_mounts')
    need(type(obs_qm) is dict, 'observation quota_mounts missing in collect-evidence')
    rows = parse_mountinfo()
    for qk, expected_mp in (('target_tmp', str(root)), ('packet', str(pkt))):
        qrec = obs_qm.get(qk)
        need(type(qrec) is dict and qrec.get('mountpoint') == expected_mp, f'observation quota_mounts.{qk} mismatch')
        matches = [(a, b) for a, b in rows if a[4] == expected_mp]
        need(len(matches) == 1 and int(matches[0][0][0]) == qrec.get('mount_id') and matches[0][0][2] == qrec.get('device') and matches[0][1][0] == 'tmpfs', f'active tmpfs mount drift vs OP-6 on {qk}')
    require_absent_nonsymlink(ev_dir, 'evidence_dir')
    marker = root / 'ONE_SHOT_STARTED'
    need(not marker.is_symlink() and marker.is_file(), 'ONE_SHOT_STARTED regular marker missing')
    marker_sha = sha256_regular(marker)
    pkt_items = []
    for item in sorted(pkt.iterdir(), key=lambda x: x.name):
        need(not item.is_symlink() and item.is_file(), 'unexpected non-regular packet entry: ' + item.name)
        pkt_items.append((item, sha256_regular(item)))
    os.mkdir(ev_dir, 0o700)
    os.chmod(ev_dir, 0o700)
    manifest = {}
    manifest['ONE_SHOT_STARTED'] = copy_regular_verified(marker, ev_dir / 'ONE_SHOT_STARTED', marker_sha)
    pkt_out = ev_dir / 'packet'
    os.mkdir(pkt_out, 0o700)
    os.chmod(pkt_out, 0o700)
    for item, item_sha in pkt_items:
        dst = pkt_out / item.name
        manifest[f'packet/{item.name}'] = copy_regular_verified(item, dst, item_sha)
    fsync_dir(pkt_out)
    write_new_json(ev_dir / 'EVIDENCE_SHA256_MANIFEST.json', {
        'request_sha256': req_sha,
        'provisioner_sha256': prov_sha,
        'observation_sha256': obs_sha,
        'files': manifest,
    })
    fsync_dir(ev_dir.parent)
    return {'step': 'COLLECT-EVIDENCE', 'status': 'OK', 'evidence_dir': str(ev_dir), 'count': len(manifest)}

def kill_and_rmdir_cgroup(cg_dir: Path, recurse_children: bool = False, depth: int = 0) -> None:
    need(not any(x.is_symlink() for x in [cg_dir, *cg_dir.parents]), f'symlink in cgroup teardown path: {cg_dir}')
    if not cg_dir.exists():
        return
    st = cg_dir.lstat()
    need(stat.S_ISDIR(st.st_mode) and not stat.S_ISLNK(st.st_mode), f'not cgroup directory: {cg_dir}')
    kill_file = cg_dir / 'cgroup.kill'
    if kill_file.exists():
        kill_file.write_text('1', encoding='utf-8')
    if recurse_children:
        need(depth <= 4, f'cgroup nesting depth exceeded under P: {cg_dir}')
        for sub in sorted(cg_dir.iterdir(), key=lambda x: x.name):
            need(not sub.is_symlink(), f'symlink inside cgroup subtree: {sub}')
            if sub.is_dir():
                kill_and_rmdir_cgroup(sub, recurse_children=True, depth=depth + 1)
    events_file = cg_dir / 'cgroup.events'
    if events_file.exists():
        for _ in range(TEARDOWN_POLL_MAX_ATTEMPTS):
            ev = dict(line.split() for line in events_file.read_text(encoding='utf-8').splitlines() if len(line.split()) == 2)
            procs_file = cg_dir / 'cgroup.procs'
            procs_empty = not procs_file.read_text(encoding='utf-8').splitlines() if procs_file.exists() else True
            if ev.get('populated', '0') == '0' and procs_empty:
                break
            time.sleep(TEARDOWN_POLL_SLEEP_SECONDS)
        else:
            raise Refusal(f'cgroup remained populated after cgroup.kill: {cg_dir}')
    os.rmdir(cg_dir)

def umount_quota_tmpfs(mp: Path, require_mounted: bool) -> bool:
    need(not any(x.is_symlink() for x in [mp, *mp.parents]), f'symlink in unmount path: {mp}')
    matches = [(a, b) for a, b in parse_mountinfo() if a[4] == str(mp)]
    if not matches:
        need(not require_mounted, f'expected active tmpfs mount missing: {mp}')
        return False
    need(len(matches) == 1 and matches[0][1][0] == 'tmpfs', f'mountpoint is not a single active tmpfs mount: {mp}')
    st = mp.lstat()
    need(stat.S_ISDIR(st.st_mode) and not stat.S_ISLNK(st.st_mode), f'mountpoint is not a directory: {mp}')
    umount_bin = pin_operator_tool('/usr/bin/umount')['realpath']
    subprocess.run([umount_bin, '--', str(mp)], check=True, shell=False)
    return True

def teardown_cgroups(req: dict) -> None:
    e_dir, l_dir, p_dir = cg_paths(req)
    need(not any(x.is_symlink() for x in [*e_dir.parents]), 'symlink in /sys/fs/cgroup parents')
    if e_dir.exists():
        need(not e_dir.is_symlink(), f'symlink cgroup E: {e_dir}')
        if (e_dir / 'cgroup.kill').exists():
            (e_dir / 'cgroup.kill').write_text('1', encoding='utf-8')
    if p_dir.exists():
        need(not p_dir.is_symlink(), f'symlink cgroup P: {p_dir}')
        for ch in sorted(p_dir.iterdir(), key=lambda x: x.name):
            need(not ch.is_symlink(), f'symlink inside P cgroup: {ch}')
            if ch.is_dir():
                kill_and_rmdir_cgroup(ch, recurse_children=True, depth=1)
    for node in (p_dir, l_dir, e_dir):
        kill_and_rmdir_cgroup(node)

def cmd_teardown(req: dict, request_sha256: str | None = None, provisioner_sha256: str | None = None) -> dict:
    require_root_operator()
    req_sha = request_sha256 if request_sha256 is not None else default_request_sha256(req)
    prov_sha = provisioner_sha256 if provisioner_sha256 is not None else default_provisioner_sha256()
    ev_dir = absolute(req['paths']['evidence_dir'])
    obs_path = absolute(req['paths']['observation_output'])
    ev_manifest = ev_dir / 'EVIDENCE_SHA256_MANIFEST.json'
    need(ev_manifest.exists() and not ev_manifest.is_symlink(), 'refusing teardown before durable evidence manifest exists')
    manifest_doc, manifest_sha = jload_with_sha256(ev_manifest)
    exact(manifest_doc, 'request_sha256 provisioner_sha256 observation_sha256 files', 'evidence_manifest')
    obs_sha = sha256_regular(obs_path)
    need(manifest_doc['request_sha256'] == req_sha and manifest_doc['provisioner_sha256'] == prov_sha and manifest_doc['observation_sha256'] == obs_sha, 'evidence manifest provenance mismatch in teardown')
    files = manifest_doc['files']
    need(type(files) is dict and 'ONE_SHOT_STARTED' in files, 'evidence manifest missing ONE_SHOT_STARTED')
    for rel_name, f_sha in files.items():
        need(type(rel_name) is str and (rel_name == 'ONE_SHOT_STARTED' or re.fullmatch(r'packet/[A-Za-z0-9._-]+', rel_name) is not None), f'invalid manifest file key: {rel_name}')
        need(type(f_sha) is str and re.fullmatch(r'[0-9a-f]{64}', f_sha) is not None and sha256_regular(ev_dir / rel_name) == f_sha, f'evidence file digest mismatch: {rel_name}')
    teardown_cgroups(req)
    for k in ('packet', 'source_home', 'execution_root'):
        umount_quota_tmpfs(absolute(req['paths'][k]), require_mounted=False)
    return {'step': 'TEARDOWN', 'status': 'OK', 'evidence_manifest_sha256': manifest_sha}

def cmd_abort_prelaunch_cleanup(req: dict) -> dict:
    require_root_operator()
    root = absolute(req['paths']['execution_root'])
    need(not any(x.is_symlink() for x in root.parents), 'symlink in execution_root parents')
    if root.exists() or root.is_symlink():
        need(not root.is_symlink(), 'execution_root is a symlink')
        marker = root / 'ONE_SHOT_STARTED'
        need(not marker.exists() and not marker.is_symlink(), 'refusing prelaunch abort cleanup after ONE_SHOT_STARTED')
    teardown_cgroups(req)
    unmounted = []
    for k in ('packet', 'source_home', 'execution_root'):
        mp = absolute(req['paths'][k])
        if umount_quota_tmpfs(mp, require_mounted=False):
            unmounted.append(str(mp))
    return {'step': 'ABORT-PRELAUNCH-CLEANUP', 'status': 'OK', 'unmounted': unmounted}

def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--request', required=True, help='Path to operator provisioning request JSON')
    ap.add_argument('--request-sha256', default=None, help='Optional expected 64-hex SHA-256 of request JSON')
    ap.add_argument('step', choices=('op0', 'op1', 'op2', 'op3', 'op4', 'op5', 'op6', 'prelaunch-verify', 'collect-evidence', 'teardown', 'abort-prelaunch-cleanup'))
    args = ap.parse_args(argv)
    raw_req, req_sha = jload_with_sha256(args.request)
    if args.request_sha256 is not None:
        need(re.fullmatch(r'[0-9a-f]{64}', args.request_sha256) is not None and req_sha == args.request_sha256, 'request SHA-256 mismatch against --request-sha256')
    prov_sha = default_provisioner_sha256()
    req = validate_request(raw_req)
    dispatch = {
        'op0': cmd_op0,
        'op1': cmd_op1,
        'op2': cmd_op2,
        'op3': cmd_op3,
        'op4': cmd_op4,
        'op5': cmd_op5,
        'op6': lambda r: cmd_op6(r, request_sha256=req_sha, provisioner_sha256=prov_sha),
        'prelaunch-verify': lambda r: cmd_prelaunch_verify(r, request_sha256=req_sha, provisioner_sha256=prov_sha),
        'collect-evidence': lambda r: cmd_collect_evidence(r, request_sha256=req_sha, provisioner_sha256=prov_sha),
        'teardown': lambda r: cmd_teardown(r, request_sha256=req_sha, provisioner_sha256=prov_sha),
        'abort-prelaunch-cleanup': cmd_abort_prelaunch_cleanup,
    }
    res = dispatch[args.step](req)
    print(json.dumps(res, sort_keys=True, separators=(',', ':')))
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
