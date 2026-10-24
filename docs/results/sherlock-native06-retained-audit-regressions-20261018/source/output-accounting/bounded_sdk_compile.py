#!/usr/bin/python3
"""Unexecuted one-shot source proposal. This file grants no authority.

The example is deliberately incomplete. No phase may run until every exact
binding, host prerequisite, review and separate tier consent has been supplied.
No shell, downloads, alternate compiler, SDK API, test runner or retry path.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import resource
import selectors
import signal
import stat
import subprocess
import sys
import tarfile
import time

PHASES = ('A0', 'A', 'B', 'C', 'D', 'E', 'F')
CARGO = '/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo'
CARGO_PATH = str(Path(CARGO).parent) + ':/usr/bin:/bin'
BACKEND = ('test', '--no-run', '--frozen', '-p', 'mori-backend', '--lib', '-j4')
CLI_OBLIGATION = ('test', '--no-run', '--frozen', '-p', 'mori', '--test', 'stage_test_public', '-j4')
BASE = '5cf27d64fe91a3faef3ed1b4283284c50f660b24'
LOCK = 'dfffb853b47a7b2d0d1b80a64bb3e826d64d66f247881603b6fc31c8e3e34003'
REQUIRED_MASKS = {'/run', '/var/lib/docker', '/nix/var/nix/daemon-socket',
                  '/nix/var/determinate', '/home/charl',
                  '/home/charl/.cargo', '/sys/fs/cgroup',
                  '/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014'}
HEX = re.compile(r'[0-9a-f]{64}\Z')
CHUNK = 1024 * 1024
# Owner-bound Git mode-120000 rows, exactly as recorded in GIT-MODE-FACTS.json
# (sha256 18aacc53ad146565a7d54ee35b5387a1ea25d4d97f1727e241461f4e6b84c0c5).
# Only these (commit, path, blob) rows may be symlinks; every other path stays regular.
# Link text is the only content ever read for them; targets are never followed.
GIT_SYMLINK_MODE = '120000'
GIT_SYMLINK_TEXT_BYTES = (33, 55)
GIT_SYMLINKS = frozenset({
    ('04c9c5d9bcebb8d4427d8589fb54d58a55599c14', 'integration-tests/resources/sample_vk.verifier', 'bc7a71de4f851d7cf7d5057b7c37a34ea810d76d'),
    ('04c9c5d9bcebb8d4427d8589fb54d58a55599c14', 'wasm-proving-demos/webpage/public/midnight/dust/spend.bzkir', '90c050be858ccab1c5af1577f9d096c4a9100e7b'),
    ('04c9c5d9bcebb8d4427d8589fb54d58a55599c14', 'wasm-proving-demos/webpage/public/midnight/dust/spend.bzkir.sha256', '4c5a4df1af7a20d300d569f3b6f0c59a73ca9321'),
    ('04c9c5d9bcebb8d4427d8589fb54d58a55599c14', 'wasm-proving-demos/webpage/public/midnight/dust/spend.prover.sha256', '970ccd09630a4e9f4d163114448cb87da1fffe79'),
    ('04c9c5d9bcebb8d4427d8589fb54d58a55599c14', 'wasm-proving-demos/webpage/public/midnight/dust/spend.verifier', 'e0b21cadbbf1ba6e84ed129cb22ab7fd556cd6f9'),
    ('04c9c5d9bcebb8d4427d8589fb54d58a55599c14', 'wasm-proving-demos/webpage/public/midnight/dust/spend.verifier.sha256', '08a90b6bbf012c3edd41f7402643361b34031319'),
    ('04c9c5d9bcebb8d4427d8589fb54d58a55599c14', 'wasm-proving-demos/webpage/public/midnight/dust/spend.zkir', '66c922eba4290c4fc6a5b58bf2d29c40c6752f7b'),
    ('04c9c5d9bcebb8d4427d8589fb54d58a55599c14', 'wasm-proving-demos/webpage/public/midnight/dust/spend.zkir.sha256', 'c0034ed0eb572b69526f3d1d88c1d86cbb5d3fa4'),
    ('a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1', 'integration-tests/resources/sample_vk.verifier', 'bc7a71de4f851d7cf7d5057b7c37a34ea810d76d'),
    ('a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1', 'wasm-proving-demos/webpage/public/midnight/dust/spend.bzkir', '90c050be858ccab1c5af1577f9d096c4a9100e7b'),
    ('a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1', 'wasm-proving-demos/webpage/public/midnight/dust/spend.bzkir.sha256', '4c5a4df1af7a20d300d569f3b6f0c59a73ca9321'),
    ('a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1', 'wasm-proving-demos/webpage/public/midnight/dust/spend.prover.sha256', '970ccd09630a4e9f4d163114448cb87da1fffe79'),
    ('a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1', 'wasm-proving-demos/webpage/public/midnight/dust/spend.verifier', 'e0b21cadbbf1ba6e84ed129cb22ab7fd556cd6f9'),
    ('a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1', 'wasm-proving-demos/webpage/public/midnight/dust/spend.verifier.sha256', '08a90b6bbf012c3edd41f7402643361b34031319'),
    ('a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1', 'wasm-proving-demos/webpage/public/midnight/dust/spend.zkir', '66c922eba4290c4fc6a5b58bf2d29c40c6752f7b'),
    ('a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1', 'wasm-proving-demos/webpage/public/midnight/dust/spend.zkir.sha256', 'c0034ed0eb572b69526f3d1d88c1d86cbb5d3fa4'),
})

class Refusal(RuntimeError):
    pass

class Unsupported(Refusal):
    pass

def need(ok, why):
    if not ok:
        raise Refusal(why)

def exact(obj, keys, where):
    need(type(obj) is dict and set(obj) == set(keys.split()), 'schema keys: ' + where)

def no_null(value, where='config'):
    need(value is not None, 'unbound/null: ' + where)
    if isinstance(value, dict):
        for k, v in value.items():
            no_null(v, where + '.' + k)
    elif isinstance(value, list):
        for i, v in enumerate(value):
            no_null(v, where + '[' + str(i) + ']')
    elif isinstance(value, str):
        need('\0' not in value and value != '' and not value.startswith('UNBOUND'), 'unbound string: ' + where)

def positive(value, where):
    need(type(value) is int and value > 0, 'positive integer required: ' + where)

def absolute(value):
    need(type(value) is str and value.startswith('/') and str(Path(value)) == value,
         'normalized absolute path required')
    need('..' not in Path(value).parts and '\0' not in value, 'unsafe path')
    return Path(value)

def relative(value):
    p = PurePosixPath(value)
    need(type(value) is str and not p.is_absolute() and '..' not in p.parts and str(p) == value and value != '.', 'unsafe relative path')
    return p

def pin_schema(p):
    exact(p, 'path sha256', 'pin')
    absolute(p['path']); need(bool(HEX.fullmatch(p['sha256'])), 'full sha256 required')

def validate_cgroup(cg):
    exact(cg,'envelope_path supervisor_leaf_name phase_parent_name envelope_memory_max envelope_pids_max envelope_cpu_max','cgroup')
    p=absolute(cg['envelope_path'])
    need(re.fullmatch(r'(/[A-Za-z0-9_][A-Za-z0-9_.@-]{0,127})+',cg['envelope_path']) is not None and not any(part.startswith('cgroup.') for part in p.parts),'invalid envelope namespace path')
    for key in ('supervisor_leaf_name','phase_parent_name'):
        name=cg[key]
        need(type(name) is str and re.fullmatch(r'[a-z][a-z0-9-]{0,62}',name) is not None and not name.startswith('sdk-once-'),'invalid cgroup child name')
    need(cg['supervisor_leaf_name']!=cg['phase_parent_name'],'cgroup child names must differ')
    positive(cg['envelope_memory_max'],'envelope memory');positive(cg['envelope_pids_max'],'envelope tasks')
    need(type(cg['envelope_cpu_max']) is str and re.fullmatch(r'[1-9][0-9]* [1-9][0-9]*',cg['envelope_cpu_max']) is not None,'finite envelope cpu.max')

def validate(c):
    exact(c, 'version mode bindings paths masks readonly_tools source_copy_mounts limits cgroup quota_mounts review_receipts old_home_stat_manifest source_manifest git_checkouts registry_roots includes expected_generated_files executable_pins cpu_arch source_authoring_only external_tier_consent total_wall_seconds supervisor_cpu_seconds acquired_archives', 'config')
    need(c['version'] == 2 and c['mode'] in ('tier1', 'tier1-and-compile'), 'mode/version')
    no_null(c)
    need(type(c['acquired_archives']) is list and len(c['acquired_archives'])==5,'exact acquired archive list')
    for p in c['acquired_archives']:pin_schema(p)
    positive(c['total_wall_seconds'],'total wall'); positive(c['supervisor_cpu_seconds'],'supervisor CPU')
    exact(c['external_tier_consent'],'tier1 tier2','external tier consent')
    need(all(type(x) is bool for x in c['external_tier_consent'].values()),'tier consent booleans')
    need(c['source_authoring_only'] is False, 'proposal is source-only; execution config must be separately reviewed')
    need(c['cpu_arch'] == 'x86_64-unknown-linux-gnu', 'unsupported target')
    exact(c['bindings'], 'runner config_inputs subject staging host_units embedded amendments', 'bindings')
    for p in c['bindings'].values(): pin_schema(p)
    exact(c['paths'], 'worktree execution_root source_home target tmp packet empty_home mask_empty config', 'paths')
    for p in c['paths'].values(): absolute(p)
    need(c['paths']['worktree'] == '/home/charl/Moriarty/.worktrees/sherlock-a102-build-20261016', 'wrong backend candidate')
    root = Path(c['paths']['execution_root'])
    for key in ('source_home', 'target', 'tmp', 'packet', 'empty_home', 'mask_empty'):
        need(Path(c['paths'][key]).parent == root, 'owned direct child required: ' + key)
    need(len(set(c['paths'].values())) == len(c['paths']), 'overlapping exact paths')
    need(type(c['masks']) is list and REQUIRED_MASKS <= set(c['masks']), 'complete required masks missing')
    for p in c['masks']: absolute(p)
    for p in c['readonly_tools']: absolute(p)
    need(set(c['readonly_tools'])=={str(Path(CARGO).parent.parent)},'only the pinned Rust toolchain restoration is supported')
    need(type(c['source_copy_mounts']) is list and len(c['source_copy_mounts'])==6,'exact six copy-source mounts required')
    for p in c['source_copy_mounts']: absolute(p)
    copy_sources={'/home/charl/.cargo'} | {p['path'] for p in c['acquired_archives']}
    need(len(copy_sources)==6 and set(c['source_copy_mounts'])==copy_sources,'copy mounts must equal old Cargo home and five acquired archive files')
    exact(c['limits'], 'A0 A B C D E F', 'limits')
    for phase, v in c['limits'].items():
        exact(v, 'wall_seconds memory_bytes pids cpu_quota_us cpu_period_us cpu_time_us stdout_bytes stderr_bytes packet_bytes source_bytes target_tmp_bytes file_bytes namespace_tmpfs_bytes', 'limits.' + phase)
        for k, n in v.items(): positive(n, phase + '.' + k)
        need(v['stdout_bytes'] <= 16777216 and v['stderr_bytes'] <= 16777216, 'owner log ceilings')
    # Existing proposed ceilings are upper bounds, never a grant.
    for ph, wall, mem, pids in [('A',900,2147483648,None), ('B',600,2147483648,256), ('C',1200,2147483648,None), ('E',120,2147483648,256), ('F',2700,8589934592,1024)]:
        v=c['limits'][ph]; need(v['wall_seconds']<=wall and v['memory_bytes']<=mem, 'owner ceiling '+ph)
        if pids is not None: need(v['pids']<=pids, 'owner tasks ceiling '+ph)
    for ph in ('B','E'): need(c['limits'][ph]['source_bytes']<=1287099500, 'source-home ceiling')
    need(c['limits']['C']['packet_bytes']<=67108864, 'C packet ceiling')
    need(c['limits']['F']['target_tmp_bytes']<=21474836480, 'F target/tmp ceiling')
    validate_cgroup(c['cgroup'])
    exact(c['quota_mounts'], 'source_home target_tmp packet', 'quotas')
    for q in c['quota_mounts'].values():
        exact(q, 'mountpoint mount_id device max_bytes max_inodes', 'quota')
        absolute(q['mountpoint'])
        for k in ('mount_id','max_bytes','max_inodes'): positive(q[k],k)
    receipt_keys='security nonsecurity root tier1'+(' tier2' if c['mode']=='tier1-and-compile' else '')
    exact(c['review_receipts'],receipt_keys,'review receipts')
    for p in c['review_receipts'].values(): pin_schema(p)
    for key in ('old_home_stat_manifest','source_manifest'): pin_schema(c[key])
    for p in c['executable_pins']:
        exact(p, 'path realpath sha256', 'executable'); absolute(p['path']); absolute(p['realpath']); need(bool(HEX.fullmatch(p['sha256'])),'executable hash')
    need({CARGO,'/usr/bin/bwrap','/usr/bin/python3'} <= {p['path'] for p in c['executable_pins']}, 'runner tools not pinned')
    for r in c['registry_roots']:
        exact(r, 'archive source_relative package_checksum', 'registry root'); relative(r['archive']); relative(r['source_relative']); need(bool(HEX.fullmatch(r['package_checksum'])),'archive checksum')
    need(len(c['registry_roots'])==423,'all 423 archives must be bound')
    need(type(c['git_checkouts']) is list and len(c['git_checkouts'])==2,'exactly two Git checkouts')
    for g in c['git_checkouts']:
        exact(g, 'relative_root commit files', 'git checkout'); relative(g['relative_root']); need(re.fullmatch('[0-9a-f]{40}',g['commit']) is not None,'git commit')
        seen=set();declared=set()
        for p in g['files']:
            exact(p,'relative blob mode','git file'); relative(p['relative']); need(re.fullmatch('[0-9a-f]{40}',p['blob']) is not None,'blob identity'); need(p['mode'] in ('100644','100755',GIT_SYMLINK_MODE),'unsupported Git mode')
            need(p['relative'] not in seen,'duplicate Git file path');seen.add(p['relative'])
            # Mode 120000 only for the exact owner-bound rows; no general enum widening.
            if p['mode']==GIT_SYMLINK_MODE:
                need((g['commit'],p['relative'],p['blob']) in GIT_SYMLINKS,'undeclared Git symlink row');declared.add((g['commit'],p['relative'],p['blob']))
        need(declared=={x for x in GIT_SYMLINKS if x[0]==g['commit']},'exact declared Git symlink membership')
        names={x[1] for x in declared}
        need(not any(str(a) in names for r in seen for a in PurePosixPath(r).parents),'Git path beneath declared symlink')
    need({g['commit'] for g in c['git_checkouts']} == {'04c9c5d9bcebb8d4427d8589fb54d58a55599c14','a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1'},'full pinned Git commits required')
    # Source-home disjointness: no row of one checkout can lie at or beneath the other's links.
    first,second=(PurePosixPath(g['relative_root']) for g in c['git_checkouts'])
    need(first!=second and first not in second.parents and second not in first.parents,'Git checkout roots equal or nested')
    links=link_rows(c);need(len(links)==len(GIT_SYMLINKS),'declared Git symlink paths collide')
    for p in c['includes']:
        exact(p,'relative algorithm digest source_identifier','include'); relative(p['relative']); need(p['algorithm'] in ('archive-member','git-blob'),'include algorithm'); need(re.fullmatch('[0-9a-f]{'+('64' if p['algorithm']=='archive-member' else '40')+'}',p['digest']) is not None,'include pin')
        need(not p['relative'].endswith(('.zkir','.bzkir','.prover')),'excluded SDK payload include')
    need(len({p['relative'] for p in c['includes']})==len(c['includes']),'duplicate include path')
    for p in c['expected_generated_files']:
        exact(p,'relative kind','generated file');relative(p['relative']);need(p['kind'] in ('cargo-ok','cargo-checksum','cargo-lockfile','git-control'),'generated file kind')
        name=PurePosixPath(p['relative']).name
        if p['kind']=='cargo-ok':need(name=='.cargo-ok','cargo-ok name')
        elif p['kind']=='cargo-checksum':need(name=='.cargo-checksum.json','checksum name')
        elif p['kind']=='cargo-lockfile':need(p['relative'] in ('.package-cache','.package-cache-mutate','.global-cache','.global-cache-wal','.global-cache-shm','git/CACHEDIR.TAG','registry/CACHEDIR.TAG'),'bookkeeping name')
        elif p['kind']=='git-control':need(any(p['relative'].startswith(g['relative_root']+'/.git/') for g in c['git_checkouts']),'Git control outside checkout .git')
    for p in [*c['includes'],*c['expected_generated_files']]:
        need(not any(p['relative']==x or p['relative'].startswith(x+'/') for x in links),'include or generated path at or under declared Git symlink')
    return c

def jload(path):
    def pairs(items):
        result={}
        for k,v in items:
            need(k not in result,'duplicate JSON key: '+k);result[k]=v
        return result
    return json.loads(Path(path).read_text(), object_pairs_hook=pairs)

def stable_stat(path):
    s=os.lstat(path)
    return {'device':s.st_dev,'inode':s.st_ino,'size':s.st_size,'mtime_ns':s.st_mtime_ns,'mode':s.st_mode,'links':s.st_nlink}

def owned_paths(c):
    """Check actual output objects, not only textual directory relationships."""
    root=absolute(c['paths']['execution_root'])
    for key in ('execution_root','source_home','target','tmp','packet','empty_home','mask_empty'):
        p=absolute(c['paths'][key])
        need(p.resolve(strict=True)==p,'owned path resolves elsewhere: '+key)
        need(not any(x.is_symlink() for x in [p,*p.parents]),'owned path symlink: '+key)
        st=os.lstat(p);need(stat.S_ISDIR(st.st_mode) and st.st_uid==os.getuid(),'owned directory identity: '+key)
        if key!='execution_root':need(p.parent==root,'owned path escaped root')
    for key in ('target','tmp','empty_home','mask_empty'):
        need(os.stat(c['paths'][key]).st_dev==os.stat(root).st_dev,'owned path outside execution-root quota: '+key)

def executing_identity(c):
    actual=Path(__file__).resolve(strict=True);expected=absolute(c['bindings']['runner']['path'])
    need(actual==expected,'executing supervisor/worker differs from pinned runner path')
    need(digest(actual)==c['bindings']['runner']['sha256'],'executing runner bytes differ from pin')

def regular(path):
    p=absolute(str(path))
    need(not any(x.is_symlink() for x in [p,*p.parents]),'symlink path: '+str(p))
    s=os.stat(p,follow_symlinks=False);need(stat.S_ISREG(s.st_mode),'not regular: '+str(p));return s

def digest(path, git=False):
    s=regular(path); before=stable_stat(path)
    h=hashlib.sha1() if git else hashlib.sha256()
    if git:h.update(b'blob '+str(s.st_size).encode()+b'\0')
    fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW)
    try:
        with os.fdopen(fd,'rb') as f:
            while b:=f.read(CHUNK):h.update(b)
    finally:
        need(stable_stat(path)==before,'file changed while reading: '+str(path))
    return h.hexdigest()

def check_pin(p):
    need(digest(p['path'])==p['sha256'],'pin mismatch: '+p['path'])

def json_bytes(value):return (json.dumps(value,sort_keys=True,separators=(',',':'))+'\n').encode()

def write_new(path, data, cap):
    need(len(data)<=cap,'packet output cap')
    fd=os.open(path,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
    with os.fdopen(fd,'wb') as f:f.write(data);f.flush();os.fsync(f.fileno())

def receipt(c, name, value, phase):
    write_new(Path(c['paths']['packet'])/name,json_bytes(value),c['limits'][phase]['packet_bytes'])

def walk_tree(root,links=frozenset()):
    # No-follow walk: regular files, exactly the declared symlinks, and directories.
    # Entries are classified with lstat semantics through an O_NOFOLLOW directory fd.
    # A declared symlink is recorded but never descended, opened, stat'ed through or
    # chmod'ed; any other symlink refuses. Unreadable directories refuse, never skip.
    root=absolute(str(root));files=[];found=set();dirs=[];pending=[root]
    need(not any(x.is_symlink() for x in [root,*root.parents]),'symlink tree root: '+str(root))
    while pending:
        base=pending.pop();fd=os.open(base,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW|os.O_CLOEXEC)
        try:
            with os.scandir(fd) as it:entries=[(e.name,e.is_symlink(),e.is_dir(follow_symlinks=False)) for e in it]
        finally:os.close(fd)
        for name,link,directory in entries:
            p=base/name
            if link:need(p in links,'undeclared symlink: '+str(p));found.add(p)
            else:
                need(p not in links,'declared Git symlink is not a symlink: '+str(p))
                if directory:dirs.append(p);pending.append(p)
                else:regular(p);files.append(p)
    return sorted(files),found,sorted(dirs)

def walk_files(root):return walk_tree(root)[0]

def link_rows(c):
    # Validated mode-120000 rows keyed by source-home relative path.
    return {str(PurePosixPath(g['relative_root'])/p['relative']):(g['commit'],p['relative'],p['blob']) for g in c['git_checkouts'] for p in g['files'] if p['mode']==GIT_SYMLINK_MODE}

def declared_links(c):
    root=Path(c['paths']['source_home']);return frozenset(root/relative(rel) for rel in link_rows(c))

def link_identity(path):
    # Git blob identity over the link text only. The target is never opened, stat'ed or followed.
    p=absolute(str(path))
    need(not any(x.is_symlink() for x in p.parents),'symlink ancestor: '+str(p))
    before=stable_stat(p);need(stat.S_ISLNK(before['mode']),'declared Git symlink not materialized as a symlink: '+str(p))
    need(GIT_SYMLINK_TEXT_BYTES[0]<=before['size']<=GIT_SYMLINK_TEXT_BYTES[1],'Git symlink text outside admitted size: '+str(p))
    text=os.readlink(os.fsencode(p))
    need(stable_stat(p)==before and len(text)==before['size'],'symlink changed while reading: '+str(p))
    return text,hashlib.sha1(b'blob '+str(len(text)).encode()+b'\0'+text).hexdigest()

def link_target(relative_link,text,regular_rows):
    # Fail-closed lexical containment of authenticated link text; nothing is resolved.
    # Only leading '..' components are accepted, so the lexical result equals kernel
    # resolution through the walked non-symlink directories of the same checkout.
    need(all(0x21<=ch<=0x7e for ch in text) and not text.startswith(b'/'),'Git symlink text is not relative printable ASCII')
    parts=list(PurePosixPath(relative_link).parent.parts);descending=False
    for part in text.decode('ascii').split('/'):
        need(part not in ('','.'),'non-normalized Git symlink text')
        if part=='..':need(not descending and bool(parts),'Git symlink escapes checkout or ascends after descent');parts.pop()
        else:descending=True;parts.append(part)
    target='/'.join(parts)
    need(descending and '.git' not in parts and target in regular_rows,'Git symlink target is not a declared regular file of the same checkout')
    return target

def tree_hashes(root,links=frozenset()):
    files,found,_=walk_tree(root,links);root=absolute(str(root))
    need(found==set(links),'declared Git symlink missing from tree: '+str(root))
    hashes={str(p.relative_to(root)):digest(p) for p in files}
    hashes.update({str(p.relative_to(root)):'git-symlink:'+link_identity(p)[1] for p in sorted(found)})
    return hashes

def source_binding(c):
    check_pin(c['source_manifest']);m=jload(c['source_manifest']['path'])
    exact(m,'root head files excluded_directories head_files','checkout manifest')
    need(m['root']==c['paths']['worktree'] and m['head']==BASE,'candidate source base')
    # Exclusions are directory masks, not ways to omit compile inputs. Only .git
    # is supported here; existing target directories must be absent/masked.
    need(m['excluded_directories']==['.git'],'source omission unsupported')
    root=Path(m['root']); actual={}
    for base,dirs,files in os.walk(root,followlinks=False):
        if Path(base)==root:dirs[:]=[d for d in dirs if d!='.git']
        for d in dirs:need(not (Path(base)/d).is_symlink(),'source directory symlink')
        for f in files:
            if Path(base)==root and f=='.git':continue
            p=Path(base)/f;actual[str(p.relative_to(root))]=digest(p)
    need(actual==m['files'],'complete source membership/hash mismatch')
    for p in m['head_files']:pin_schema(p);check_pin(p)
    need(actual.get('Cargo.lock')==LOCK,'lock changed')
    subject=jload(c['bindings']['subject']['path'])
    need(len(subject['candidate_files'])==10,'backend selected-file membership')
    for entry in subject['candidate_files']:
        need(actual.get(entry['path'])==entry['sha256'],'selected backend source differs from reviewed subject')

def old_home_stats(c):
    check_pin(c['old_home_stat_manifest']);m=jload(c['old_home_stat_manifest']['path'])
    exact(m,'paths directory_memberships','old-home stat manifest')
    for p,e in m['paths'].items():need(stable_stat(p)==e,'original metadata changed: '+p)
    for p,names in m['directory_memberships'].items():need(sorted(os.listdir(p))==names,'original membership changed: '+p)

def bound_identities(c,host_originals=True):
    for p in c['bindings'].values():check_pin(p)
    for p in c['executable_pins']:
        need(str(Path(p['path']).resolve())==p['realpath'],'tool symlink changed')
        need(digest(p['realpath'])==p['sha256'],'tool changed')
    need(not any((Path(d)/'scheme').exists() for d in CARGO_PATH.split(':')),'scheme present on bounded PATH')
    subject=jload(c['bindings']['subject']['path'])
    need(subject['base_head']==BASE and subject['cargo_lock_sha256']==LOCK,'subject mismatch')
    need(subject['worktree']==c['paths']['worktree'],'subject path')
    approved_includes(c)
    source_binding(c)
    if host_originals:old_home_stats(c)
    frozen=Path(c['paths']['packet'])/'A0_FREEZE.json'
    if host_originals and frozen.exists():
        for row in jload(frozen)['inputs']:
            need(stable_stat(row['source'])==row['stat'],'original staged input stat changed')

def consent(c):
    import copy
    projection=copy.deepcopy(c);projection.pop('review_receipts');projection['bindings'].pop('config_inputs')
    check_pin(c['bindings']['config_inputs'])
    need(jload(c['bindings']['config_inputs']['path'])==projection,'configuration projection differs')
    need(c['external_tier_consent']['tier1'] is True,'Tier 1 not explicitly granted')
    need(c['external_tier_consent']['tier2']==(c['mode']=='tier1-and-compile'),'tier choice mismatch')
    records={}
    for key,pin in c['review_receipts'].items():
        check_pin(pin);record=jload(pin['path']);records[key]=record
        need(record['schema_version']==1 and record['authority_granted_by_this_file_alone'] is False,'receipt version or authority claim')
    allowed_routing={('claude-opus-5-5','claude-opus-5-5','claude-opus-5-5','high','primary'),('claude-opus-5-5','gemini-4-argon-high','gemini-4-argon-high','high','standing_opus_quota_outage_fallback')}
    for key,kind,role,decision in [('security','security_source_review','designated_security_owner','APPROVED_SOURCE_WITH_LIMITS'),('nonsecurity','nonsecurity_source_review','nonsecurity_source_reviewer','APPROVED_NORMAL_SOURCE')]:
        r=records[key]
        exact(r,'schema_version kind reviewer_role decision primary_model requested_model returned_identity effort fallback_status terminal_sha256 report_sha256 runner_sha256 source_manifest_sha256 subject_manifest_sha256 authority_granted_by_this_file_alone','source review receipt')
        need((r['kind'],r['reviewer_role'],r['decision'])==(kind,role,decision),'source review decision/role')
        need((r['primary_model'],r['requested_model'],r['returned_identity'],r['effort'],r['fallback_status']) in allowed_routing,'source review routing identity')
        need(HEX.fullmatch(r['terminal_sha256']) is not None and HEX.fullmatch(r['report_sha256']) is not None,'review terminal/report hash')
        need(r['runner_sha256']==c['bindings']['runner']['sha256'] and r['subject_manifest_sha256']==c['bindings']['subject']['sha256'],'review subject identity')
        need(HEX.fullmatch(r['source_manifest_sha256']) is not None,'review full-source manifest hash')
    need(records['security']['source_manifest_sha256']==records['nonsecurity']['source_manifest_sha256'],'review candidates differ')
    r=records['root'];exact(r,'schema_version kind decision admitted_mode runner_sha256 subject_manifest_sha256 config_inputs_sha256 security_receipt_sha256 nonsecurity_receipt_sha256 authority_granted_by_this_file_alone','Root admission')
    need((r['kind'],r['decision'],r['admitted_mode'])==('root_admission','ADMITTED_CONFIGURATION',c['mode']),'Root mode/decision')
    for field,key in [('runner_sha256','runner'),('subject_manifest_sha256','subject'),('config_inputs_sha256','config_inputs')]:need(r[field]==c['bindings'][key]['sha256'],'Root candidate binding')
    for key in ('security','nonsecurity'):need(r[key+'_receipt_sha256']==c['review_receipts'][key]['sha256'],'Root review binding')
    for tier in (1,2) if c['mode']=='tier1-and-compile' else (1,):
        r=records['tier'+str(tier)];keys='schema_version kind tier decision root_receipt_sha256 config_inputs_sha256 amendments_sha256 authority_granted_by_this_file_alone'
        if tier==2:keys+=' tier1_receipt_sha256'
        exact(r,keys,'user tier decision')
        need((r['kind'],r['tier'],r['decision'])==('user_tier'+str(tier)+'_grant',tier,'GRANTED_TIER'+str(tier)),'user tier decision')
        need(r['root_receipt_sha256']==c['review_receipts']['root']['sha256'],'grant Root identity')
        need(r['config_inputs_sha256']==c['bindings']['config_inputs']['sha256'] and r['amendments_sha256']==c['bindings']['amendments']['sha256'],'grant scope identity')
        if tier==2:need(r['tier1_receipt_sha256']==c['review_receipts']['tier1']['sha256'],'Tier 2 predecessor grant')
    # This parses the owner-proposed records; only Root's independent, externally
    # supplied full-config SHA anchors their bytes. A hash DAG grants nothing.


def cargo_argv(phase):
    if phase=='B':return [CARGO,'fetch','--frozen']
    if phase=='E':return [CARGO,'metadata','--frozen','--format-version','1','--filter-platform','x86_64-unknown-linux-gnu']
    if phase=='F':return [CARGO,*BACKEND]
    raise Refusal('not a Cargo phase')

def cargo_env(c,phase):
    p=c['paths'];env={'PATH':CARGO_PATH,'CARGO_HOME':p['source_home'],'HOME':p['empty_home'],'TMPDIR':p['tmp'],'LANG':'C'}
    if phase=='F':env['CARGO_TARGET_DIR']=p['target']
    return env

def check_tmpfs(q,allowed_nested):
    # Hard bounds, not a post-hoc du/polling claim. Provisioning is outside this
    # source and must be explicitly granted; no mount/setup command is hidden.
    rows=Path('/proc/self/mountinfo').read_text().splitlines();match=[]
    for line in rows:
        pre,post=line.split(' - ',1);a=pre.split();b=post.split()
        if a[4]==q['mountpoint']:match.append((a,b))
    need(len(match)==1,'required distinct tmpfs mount absent')
    a,b=match[0];need(int(a[0])==q['mount_id'] and a[2]==q['device'] and b[0]=='tmpfs','mount identity/type')
    need('rw' in a[5].split(','),'quota mount not writable')
    v=os.statvfs(q['mountpoint']);need(v.f_blocks*v.f_frsize<=q['max_bytes'] and v.f_files<=q['max_inodes'],'kernel filesystem bound exceeds declared limit')
    # Other namespace descendants must not bypass the same filesystem limit.
    for line in rows:
        a2=line.split(' - ',1)[0].split()
        if a2[4].startswith(q['mountpoint'].rstrip('/')+'/'):
            need(a2[4] in allowed_nested,'unbound nested mount under quota root')

def quotas(c,phase):
    owned_paths(c)
    for key in ('empty_home','mask_empty'):need(not any(Path(c['paths'][key]).iterdir()),'empty mount source changed')
    for key,q in c['quota_mounts'].items():
        permitted={c['paths']['source_home'],c['paths']['packet']} if key=='target_tmp' else set()
        check_tmpfs(q,permitted)
    p=c['paths'];q=c['quota_mounts'];v=c['limits'][phase]
    need(q['source_home']['mountpoint']==p['source_home'],'source quota must cover exact S')
    need(q['packet']['mountpoint']==p['packet'],'packet quota must cover exact packet')
    group=Path(q['target_tmp']['mountpoint'])
    need(str(group)==p['execution_root'],'target/tmp quota must cover exact execution root')
    need(all(Path(p[k]).parent==group for k in ('target','tmp')),'target and tmp need one common hard quota')
    need(q['source_home']['max_bytes']<=v['source_bytes'],'source hard bound too large for phase')
    need(q['target_tmp']['max_bytes']<=v['target_tmp_bytes'],'target/tmp hard bound too large')
    need(q['packet']['max_bytes']<=v['packet_bytes'],'packet hard bound too large')

def sandbox_args(c,phase,worker=True,precheck=False,retained_audit=False):
    need(not retained_audit or (phase=='F' and worker and not precheck),'retained audit sandbox mismatch')
    owned_paths(c)
    p=c['paths'];args=['--die-with-parent','--new-session','--unshare-user','--unshare-pid','--unshare-net','--unshare-ipc','--unshare-uts','--disable-userns','--cap-drop','ALL','--ro-bind','/','/']
    # Mask private home and old trees first; restore only the exact bound public
    # worktree/toolchain/proposal and current owned output mounts afterwards.
    args+=['--size',str(c['limits'][phase]['namespace_tmpfs_bytes']),'--tmpfs','/home/charl']
    for mask in sorted(set(c['masks'])-{'/home/charl'},key=lambda x:(len(Path(x).parts),x)):
        if phase in ('A0','A') and mask=='/home/charl/.cargo':continue
        args+=['--ro-bind',p['mask_empty'],mask]
    args+=['--proc','/proc','--remount-ro','/proc','--dev','/dev','--remount-ro','/dev']
    for path in c['readonly_tools']+[p['worktree']]:
        args+=['--ro-bind',path,path]
    if worker:
        runner=c['bindings']['runner']['path'];args+=['--ro-bind',runner,runner]
        for binding in [*c['bindings'].values(),*c['review_receipts'].values(),c['source_manifest'],c['old_home_stat_manifest']]:
            path=binding['path'];args+=['--ro-bind',path,path]
        args+=['--ro-bind',p['config'],p['config']]
        marker=str(Path(p['execution_root'])/'ONE_SHOT_STARTED');args+=['--ro-bind',marker,marker]
        for binding in jload(c['source_manifest']['path'])['head_files']:
            path=binding['path'];args+=['--ro-bind',path,path]
    if phase in ('A0','A'):
        for path in c['source_copy_mounts']:args+=['--ro-bind',path,path]
    args+=['--ro-bind',p['empty_home'],p['empty_home']]
    args+=['--bind' if phase in ('A','B','D') else '--ro-bind',p['source_home'],p['source_home']]
    args+=['--bind',p['tmp'],p['tmp']]
    if phase=='F':args+=['--ro-bind' if retained_audit else '--bind',p['target'],p['target']]
    # Only our data workers write phase receipts. Cargo never sees writable logs.
    if worker:args+=['--bind',p['packet'],p['packet']]
    for mask in sorted(set(c['masks'])-{'/home/charl'},key=lambda x:(len(Path(x).parts),x)):
        if phase in ('A0','A') and mask=='/home/charl/.cargo':continue
        args+=['--ro-bind',p['mask_empty'],mask]
    args+=['--remount-ro','/home/charl']
    args+=['--clearenv']
    for k,v in cargo_env(c,phase).items():args+=['--setenv',k,v]
    args+=['--chdir',p['worktree'],'--']
    if worker:args+=['/usr/bin/python3','-I','-B',c['bindings']['runner']['path'],'--worker',phase,'--config',p['config']]
    else:args+=cargo_argv(phase)
    if precheck:args+=['--precheck']
    if retained_audit:args+=['--retained-f-audit']
    return args

def _cg_text(path):return Path(path).read_text()
def _cg_children(path):return {p.name for p in Path(path).iterdir() if p.is_dir()}
def _cg_writable(path,search=False):return os.access(path,os.W_OK | (os.X_OK if search else 0))
def _cg_dirstat(path):
    p=Path(path);return p.lstat(),p.resolve(strict=True)
def _cg_mkdir(path):Path(path).mkdir(exist_ok=False)
def _cg_rmdir(path):Path(path).rmdir()
def _cg_write(path,value):Path(path).write_text(value)
def _mount_rows():
    rows=[]
    for line in _cg_text('/proc/self/mountinfo').splitlines():
        pre,post=line.split(' - ',1);rows.append((pre.split()[4],post.split()[0]))
    return rows

def cg_paths(c):
    cg=c['cgroup'];e=cg['envelope_path']
    return e,e+'/'+cg['supervisor_leaf_name'],e+'/'+cg['phase_parent_name']

def cg_fs(ns):return Path('/sys/fs/cgroup')/ns.lstrip('/')

def phase_cgroup(c,phase):return cg_paths(c)[2]+'/sdk-once-'+phase

def read_numbers(path):return dict((k,int(v)) for k,v in (x.split() for x in _cg_text(path).splitlines()))

class Group:
    def __init__(self,c,phase):
        self.path=cg_fs(phase_cgroup(c,phase));self.v=c['limits'][phase];self.created=False
    def create(self):
        _cg_mkdir(self.path);self.created=True
        need(_cg_text(self.path/'cgroup.type').strip()=='domain','phase cgroup type')
        need({'cpu','memory','pids'}<=set(_cg_text(self.path/'cgroup.controllers').split()),'phase controllers missing')
        need(not _cg_text(self.path/'cgroup.procs').splitlines(),'phase group populated at creation')
        for k,v in {'memory.max':self.v['memory_bytes'],'memory.swap.max':0,'pids.max':self.v['pids'],'cpu.max':str(self.v['cpu_quota_us'])+' '+str(self.v['cpu_period_us'])}.items():
            _cg_write(self.path/k,str(v));need(_cg_text(self.path/k).strip()==str(v),'cgroup setting not effective')
        _cg_dirstat(self.path/'cgroup.kill')  # Existence required; no fallback kill path.
    def join_child(self):
        (self.path/'cgroup.procs').write_text(str(os.getpid()))
        resource.setrlimit(resource.RLIMIT_FSIZE,(self.v['file_bytes'],self.v['file_bytes']))
        resource.setrlimit(resource.RLIMIT_CORE,(0,0))
    def kill(self):
        if not self.created:return
        try:_cg_dirstat(self.path)
        except FileNotFoundError:return
        _cg_write(self.path/'cgroup.kill','1')
    def check(self):
        need(getattr(self,'prior_cpu',0)+read_numbers(self.path/'cpu.stat').get('usage_usec',0)<=self.v['cpu_time_us'],'total cgroup CPU cap')
        need(read_numbers(self.path/'memory.events').get('oom_kill',0)==0,'cgroup OOM')
        need(read_numbers(self.path/'pids.events').get('max',0)==0,'tasks cap hit')
    def close(self):
        if not self.created:return
        self.kill()
        deadline=time.monotonic()+5
        while read_numbers(self.path/'cgroup.events').get('populated',0):
            need(time.monotonic()<deadline,'descendants did not terminate');time.sleep(.02)
        _cg_rmdir(self.path)

def supervisor_envelope(c):
    # Read-only E/L/P preflight, per adopted owner contract and mandatory K1-K3.
    cg=c['cgroup'];e_ns,l_ns,p_ns=cg_paths(c);uid=os.getuid()
    need(uid==os.geteuid() and uid!=0,'unprivileged matching supervisor uid required')
    status=dict(line.split(':',1) for line in _cg_text('/proc/self/status').splitlines() if ':' in line)
    for key in ('CapPrm','CapEff','CapAmb'):
        need(re.fullmatch(r'0+',status.get(key,'').strip()) is not None,'nonzero or missing '+key)
    need(_cg_text('/proc/self/cgroup').splitlines()==['0::'+l_ns],'unbound supervisor leaf')
    rows=_mount_rows();cgroups=[row for row in rows if row[1]=='cgroup2']
    need(cgroups==[('/sys/fs/cgroup','cgroup2')] and sum(row[0]=='/sys/fs/cgroup' for row in rows)==1,'single global cgroup2 mount required')
    need(not any(point.startswith('/sys/fs/cgroup/') for point,kind in rows),'nested cgroup mount')
    root_stat,_=_cg_dirstat('/sys/fs/cgroup');paths=[cg_fs(ns) for ns in (e_ns,l_ns,p_ns)];stats=[]
    for p in paths:
        st,resolved=_cg_dirstat(p)
        need(stat.S_ISDIR(st.st_mode) and not stat.S_ISLNK(st.st_mode) and resolved==p and st.st_dev==root_stat.st_dev,'cgroup path identity')
        stats.append(st)
    e,l,p=paths
    need(_cg_text(l/'cgroup.procs').splitlines()==[str(os.getpid())],'supervisor leaf membership')
    for path in paths:need(_cg_text(path/'cgroup.type').strip()=='domain','cgroup domain required')
    for k,v in [('memory.max',cg['envelope_memory_max']),('pids.max',cg['envelope_pids_max']),('cpu.max',cg['envelope_cpu_max']),('memory.swap.max',0)]:
        need(_cg_text(e/k).strip()==str(v),'aggregate resource envelope mismatch')
        st,_=_cg_dirstat(e/k)
        need(st.st_uid!=uid and not _cg_writable(e/k),'aggregate cap owned or writable by supervisor')
    need(not _cg_text(e/'cgroup.procs').splitlines(),'aggregate envelope populated')
    need({'cpu','memory','pids'}<=set(_cg_text(e/'cgroup.subtree_control').split()),'aggregate controllers missing')
    need(_cg_children(e)=={cg['supervisor_leaf_name'],cg['phase_parent_name']},'unexpected envelope children')
    need(_cg_writable(e/'cgroup.procs'),'migration ancestor not writable')
    need(not _cg_children(l),'supervisor leaf has children')
    need(not _cg_text(l/'cgroup.subtree_control').split(),'supervisor leaf enables controllers')
    need(not _cg_text(p/'cgroup.procs').splitlines(),'phase parent populated')
    for name in ('cgroup.controllers','cgroup.subtree_control'):
        need({'cpu','memory','pids'}<=set(_cg_text(p/name).split()),'phase parent controllers missing')
    need(stats[2].st_uid==uid and _cg_writable(p,search=True),'phase parent ownership or access')
    _cg_dirstat(p/'cgroup.kill')
    need(not _cg_children(p),'phase parent contains residual groups')

TRAMPOLINE = b"import array,ctypes as ct,fcntl,json,os,socket,sys\nstage_fd,control_fd,token_fd=map(int,sys.argv[1:4]);argv=sys.argv[4:]\nseals=fcntl.F_SEAL_WRITE|fcntl.F_SEAL_SHRINK|fcntl.F_SEAL_GROW|fcntl.F_SEAL_SEAL\nif fcntl.fcntl(stage_fd,fcntl.F_GET_SEALS)&seals!=seals:raise RuntimeError('unsealed stage')\nif token_fd>=0 and fcntl.fcntl(token_fd,fcntl.F_GET_SEALS)&seals!=seals:raise RuntimeError('unsealed worker token')\nclass Filter(ct.Structure):_fields_=[('code',ct.c_ushort),('jt',ct.c_ubyte),('jf',ct.c_ubyte),('k',ct.c_uint)]\nclass Program(ct.Structure):_fields_=[('len',ct.c_ushort),('filter',ct.POINTER(Filter))]\n# Default allow; individual network/uring syscalls notify. AF_UNIX sendmsg is\n# retained for SCM_RIGHTS handoff; no network FD is passed to the target.\nins=[(0x20,0,0,4),(0x15,1,0,0xc000003e),(0x06,0,0,0x80000000),\n     (0x20,0,0,0),(0x45,0,1,0x40000000),(0x06,0,0,0x80000000)]\nfor syscall in (42,425,426,427):ins.extend([(0x15,0,1,syscall),(0x06,0,0,0x7fc00000)])\nins.extend([(0x15,2,0,41),(0x15,1,0,53),(0x06,0,0,0x7fff0000),\n            (0x20,0,0,16),(0x15,0,1,1),(0x06,0,0,0x7fff0000),(0x06,0,0,0x7fc00000)])\nfilters=(Filter*len(ins))(*(Filter(*x) for x in ins));program=Program(len(ins),filters);libc=ct.CDLL(None,use_errno=True)\nif libc.prctl(38,1,0,0,0)!=0:raise RuntimeError('no_new_privs failed')\nlistener=libc.syscall(317,1,8,ct.byref(program))\nif listener<0:raise RuntimeError('seccomp NEW_LISTENER unavailable')\ncontrol=socket.socket(fileno=control_fd)\ncontrol.sendmsg([b'L'],[(socket.SOL_SOCKET,socket.SCM_RIGHTS,array.array('i',[listener]))])\nos.close(listener);control.close();os.close(stage_fd)\nkeep={0,1,2}\nif token_fd>=0:keep.add(token_fd);os.set_inheritable(token_fd,True)\nfor item in os.listdir('/proc/self/fd'):\n fd=int(item)\n if fd not in keep:\n  try:os.close(fd)\n  except OSError:pass\nif not argv or not argv[0].startswith('/'):raise RuntimeError('absolute target required')\nos.execve(argv[0],argv,dict(os.environ))\n"

def sealed_memfd(name,data):
    import fcntl
    fd=os.memfd_create(name,os.MFD_CLOEXEC|os.MFD_ALLOW_SEALING)
    offset=0
    while offset<len(data):offset+=os.write(fd,data[offset:])
    os.lseek(fd,0,os.SEEK_SET)
    seals=fcntl.F_SEAL_WRITE|fcntl.F_SEAL_SHRINK|fcntl.F_SEAL_GROW|fcntl.F_SEAL_SEAL
    fcntl.fcntl(fd,fcntl.F_ADD_SEALS,seals);return fd

def prior_receipts(c,phase,precheck,retained_audit=False):
    packet=Path(c['paths']['packet']);names=[p+'_RESULT.json' for p in PHASES[:PHASES.index(phase)]]
    extra={'A':['A0_FREEZE.json'],'C':['A0_FREEZE.json'],'D':['C_RAW_IDENTITIES.json'],'E':['D_FREEZE.json'],'F':['D_FREEZE.json','E_RESOLVED.json']}
    names+=extra.get(phase,[])
    if phase in ('E','F') and not precheck:names+=[phase+'.stdout',phase+'.stderr']
    if retained_audit:
        need(phase=='F' and not precheck,'retained audit phase mismatch')
        names+=['F_precheck.stdout','F_precheck.stderr','F_audit.stdout','F_audit.stderr','TERMINAL_FAILURE.json']
    return {name:digest(packet/name) for name in names}

def child_mounts(c,phase,retained_audit=False):
    p=c['paths'];wanted={'/':'ro','/proc':'ro','/dev':'ro','/home/charl':'ro',p['worktree']:'ro',p['source_home']:'rw' if phase in ('A','B','D') else 'ro',p['tmp']:'rw',p['empty_home']:'ro',p['packet']:'rw'}
    for tool in c['readonly_tools']:wanted[tool]='ro'
    if phase=='F':wanted[p['target']]='ro' if retained_audit else 'rw'
    return wanted

def verify_worker_context(c,phase,precheck,fd,config_sha,retained_audit=False):
    import fcntl
    need(fd is not None and fd>2,'internal inherited worker FD required')
    seals=fcntl.F_SEAL_WRITE|fcntl.F_SEAL_SHRINK|fcntl.F_SEAL_GROW|fcntl.F_SEAL_SEAL
    need(fcntl.fcntl(fd,fcntl.F_GET_SEALS)&seals==seals,'worker FD not sealed')
    os.lseek(fd,0,os.SEEK_SET)
    with os.fdopen(fd,'rb') as f:context=json.loads(f.read())
    exact(context,'nonce phase precheck config_sha256 runner_sha256 cgroup marker_sha256 predecessors host_originals_checked mounts','worker launch context')
    need(HEX.fullmatch(context['nonce']) is not None,'launch nonce')
    need((context['phase'],context['precheck'],context['config_sha256'],context['runner_sha256'])==(phase,precheck,config_sha,c['bindings']['runner']['sha256']),'worker phase/config/runner binding')
    expected='0::'+phase_cgroup(c,phase)
    need(context['cgroup']==expected and _cg_text('/proc/self/cgroup').strip()==expected,'worker cgroup differs')
    need(context['marker_sha256']==digest(Path(c['paths']['execution_root'])/'ONE_SHOT_STARTED'),'one-shot marker differs')
    need(context['predecessors']==prior_receipts(c,phase,precheck,retained_audit),'worker predecessor receipts differ')
    need(context['host_originals_checked'] is True,'supervisor original-home check absent')
    need(context['mounts']==child_mounts(c,phase,retained_audit),'worker mount contract differs')
    mounts={}
    for line in Path('/proc/self/mountinfo').read_text().splitlines():
        fields=line.split(' - ',1)[0].split();mounts[fields[4]]=set(fields[5].split(','))
    for path,access in context['mounts'].items():need(path in mounts and access in mounts[path],'worker mount mode differs: '+path)
    # Consume the nonce once inside the already verified bounded writable packet.
    write_new(Path(c['paths']['packet'])/('USED_LAUNCH_'+context['nonce']),b'consumed\n',64)
    bound_identities(c,host_originals=False)
    # Original-home objects remain masked in C/D/E/F. Their real checks happen
    # in the supervisor immediately before creating this sealed launch context.


def network_pending(fd):
    import select
    poll=select.poll();poll.register(fd,select.POLLIN)
    return any(events & select.POLLIN for _,events in poll.poll(0))

def launch(c,phase,config_sha,retained_audit=False):
    import array
    import socket
    import fcntl
    need(not retained_audit or phase=='F','retained audit only supports phase F')
    group=Group(c,phase);v=c['limits'][phase];packet=Path(c['paths']['packet'])
    need((Path(c['paths']['execution_root'])/'ONE_SHOT_STARTED').is_file(),'one-shot start missing')
    supervisor_envelope(c);bound_identities(c);quotas(c,phase)
    start=time.monotonic();counts={k:sum((packet/f'F{s}.{k}').stat().st_size for s in ('_precheck','','_audit')) for k in ('stdout','stderr')} if retained_audit else {'stdout':0,'stderr':0};records=[]
    for k,n in counts.items():need(n<=v[k+'_bytes'],'log cap hit '+phase+' '+k)
    if retained_audit:
        group.prior_cpu=read_numbers(group.path.parent/'cpu.stat').get('usage_usec',0)
        need(group.prior_cpu<v['cpu_time_us'],'total cgroup CPU cap')
    try:
        group.create()
        variants=[(True,False)] if retained_audit else [(False,False)] if phase in ('B','E','F') else [(True,False)]
        if phase in ('E','F') and not retained_audit:variants=[(True,True),(False,False),(True,False)]  # same phase/group/deadline, data audit only
        for audit,precheck in variants:
            args=sandbox_args(c,phase,audit,precheck,retained_audit);suffix='_retained_audit' if retained_audit else '_precheck' if precheck else '_audit' if audit and phase in ('E','F') else ''
            parent,child=socket.socketpair();proc=None;listener=None;sel=selectors.DefaultSelector();logs=[];token_fd=-1
            stage_fd=sealed_memfd('sdk-post-bwrap-stage',TRAMPOLINE)
            if audit:
                context={'nonce':os.urandom(32).hex(),'phase':phase,'precheck':precheck,'config_sha256':config_sha,'runner_sha256':c['bindings']['runner']['sha256'],
                         'cgroup':'0::'+phase_cgroup(c,phase),
                         'marker_sha256':digest(Path(c['paths']['execution_root'])/'ONE_SHOT_STARTED'),
                         'predecessors':prior_receipts(c,phase,precheck,retained_audit),'host_originals_checked':True,'mounts':child_mounts(c,phase,retained_audit)}
                token_fd=sealed_memfd('sdk-internal-worker-context',json_bytes(context))
            separator=args.index('--');target=args[separator+1:]
            if audit:target+=['--config-sha256',config_sha,'--worker-fd',str(token_fd)]
            preserved=[stage_fd,child.fileno()]+([token_fd] if audit else [])
            cmd=['/usr/bin/python3','-I','-B','-S','/proc/self/fd/'+str(stage_fd),str(stage_fd),str(child.fileno()),str(token_fd),*target]
            bwrap_opts=args[:separator]+([] if '--as-pid-1' in args[:separator] else ['--as-pid-1']);fd=sealed_memfd('sdk-bwrap-argv',b'\0'.join(x.encode() for x in bwrap_opts)+b'\0');args=bwrap_opts+['--',*cmd]
            try:
                for name in ('stdout','stderr'):logs.append(open(packet/(phase+suffix+'.'+name),'xb',buffering=0))
                def enter():
                    parent.close();group.join_child()
                proc=subprocess.Popen(['/usr/bin/bwrap','--args',str(fd),'--',*cmd],stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env={},pass_fds=tuple([fd,*preserved]),preexec_fn=enter,close_fds=True)
                child.close();parent.settimeout(max(.001,v['wall_seconds']-(time.monotonic()-start)))
                for name,stream,log in zip(('stdout','stderr'),(proc.stdout,proc.stderr),logs):
                    os.set_blocking(stream.fileno(),False);sel.register(stream,selectors.EVENT_READ,(name,log))
                received=[]
                try:
                    message,anc,flags,_=parent.recvmsg(1,socket.CMSG_SPACE(array.array('i').itemsize))
                    need(message==b'L' and not flags,'listener handoff failed')
                    for level,kind,data in anc:
                        need(level==socket.SOL_SOCKET and kind==socket.SCM_RIGHTS,'unexpected handoff')
                        values=array.array('i');values.frombytes(data);received.extend(values)
                    need(len(received)==1,'exact one seccomp listener required');listener=received[0]
                except BaseException:
                    for rfd in received:
                        try:os.close(rfd)
                        except OSError:pass
                    group.kill()
                    while sel.get_map():
                        if time.monotonic()-start>v['wall_seconds']:break
                        events=sel.select(.05)
                        if not events and proc.poll() is not None:break
                        for key,_ in events:
                            name,log=key.data;chunk=os.read(key.fileobj.fileno(),65536)
                            if not chunk:sel.unregister(key.fileobj);continue
                            take=min(len(chunk),max(0,v[name+'_bytes']-counts[name]))
                            if take:counts[name]+=take;log.write(chunk[:take])
                            if take<len(chunk):sel.unregister(key.fileobj)
                    raise
                sel.register(listener,selectors.EVENT_READ,('network',None))
                while len(sel.get_map())>1 or proc.poll() is None:
                    need(time.monotonic()-start<=v['wall_seconds'],'wall cap '+phase);group.check()
                    for key,_ in sel.select(.05):
                        name,log=key.data
                        if name=='network':
                            need(not network_pending(listener),'network syscall attempt: stop whole cgroup')
                            continue
                        chunk=os.read(key.fileobj.fileno(),65536)
                        if not chunk:sel.unregister(key.fileobj);continue
                        counts[name]+=len(chunk);need(counts[name]<=v[name+'_bytes'],'log cap hit '+phase+' '+name)
                        log.write(chunk)
                need(proc.wait()==0,'phase exit nonzero '+phase);group.check()
                need(not network_pending(listener),'late network notification')
                need(read_numbers(group.path/'cgroup.events').get('populated',0)==0,'background descendants remain after phase process')
                records.append({'argv':args,'audit_worker':audit,'child_exit':0})
            finally:
                if sys.exc_info()[0] is not None:signal.setitimer(signal.ITIMER_REAL,0)
                if proc is not None and proc.poll() is None:group.kill();proc.wait(timeout=5)
                if listener is not None:os.close(listener)
                parent.close();child.close();os.close(fd);os.close(stage_fd)
                if token_fd>=0:os.close(token_fd)
                sel.close()
                for log in logs:log.close()
        return {'phase':phase,'processes':records,'environment':cargo_env(c,phase),'wall_seconds':time.monotonic()-start,'log_bytes':counts}
    finally:
        if sys.exc_info()[0] is not None:signal.setitimer(signal.ITIMER_REAL,0)
        group.close()


def staging_destination(row,c):
    registry='index.crates.io-1949cf8c6b5b557f';db='git/db/midnight-ledger-b2f9c59d942dfdca/'
    dest=str(relative(row['destination_relative_to_future_home']));src=str(absolute(row['source']));kind=row['category']
    if kind=='registry-index':
        prefix='registry/index/'+registry+'/'
        need(dest==prefix+'config.json' or dest.startswith(prefix+'.cache/'),'index destination class')
        need(src=='/home/charl/.cargo/'+dest,'index source location')
    elif kind=='registry-archive-container':
        prefix='registry/cache/'+registry+'/'
        need(dest.startswith(prefix) and '/' not in dest[len(prefix):] and dest.endswith('.crate'),'archive destination class')
        acquired={p['path'] for p in c['acquired_archives']}
        need(src=='/home/charl/.cargo/'+dest or src in acquired,'archive source location')
        need(Path(src).name==Path(dest).name,'archive filename mapping')
    elif kind in ('git-db-metadata','git-loose-container-known-commit-tree-or-nonpayload-source','git-loose-unknown','git-pack-container-or-index'):
        need(dest.startswith(db) and src=='/home/charl/.cargo/'+dest,'Git DB source/destination')
        tail=dest[len(db):]
        if kind=='git-db-metadata':
            need(tail in ('HEAD','config','FETCH_HEAD','description','hooks/README.sample','info/exclude') or re.fullmatch(r'refs/commit/[0-9a-f]{40}',tail) is not None,'Git metadata destination')
        elif kind.startswith('git-loose'):
            need(re.fullmatch(r'objects/[0-9a-f]{2}/[0-9a-f]{38}',tail) is not None,'loose object destination')
        else:need(re.fullmatch(r'objects/pack/pack-[0-9a-f]{40}\.(pack|idx|rev)',tail) is not None,'Git pack destination')
    else:raise Refusal('unsupported staging category')
    return dest

def freeze_inputs(c):
    m=jload(c['bindings']['staging']['path']);rows=[];unknown=0
    need(len(m['inputs'])==2243,'staging membership changed')
    from collections import Counter
    counts=Counter(row['category'] for row in m['inputs'])
    need(counts=={'registry-index':1735,'registry-archive-container':423,'git-db-metadata':20,'git-loose-container-known-commit-tree-or-nonpayload-source':6,'git-loose-unknown':52,'git-pack-container-or-index':7},'staging category counts changed')
    destinations=set()
    for p in m['inputs']:
        destination=staging_destination(p,c);need(destination not in destinations,'duplicate staging destination');destinations.add(destination)
        src=p['source'];before=stable_stat(src);need(before['size']==p['bytes'],'staging size changed')
        got=digest(src)
        acquired={entry['path']:entry['sha256'] for entry in c['acquired_archives']}
        if src in acquired:need(got==acquired[src],'acquired archive pin changed')
        if p.get('sha256'):need(got==p['sha256'],'staging hash changed')
        else:
            need(p['category']=='git-loose-unknown','unclassified first hash');unknown+=1
        relative(p['destination_relative_to_future_home'])
        rows.append({'source':src,'destination':p['destination_relative_to_future_home'],'sha256':got,'stat':before})
    need(unknown==52,'unknown container count changed')
    receipt(c,'A0_FREEZE.json',{'observation':'FIRST CURRENT raw-container observation under separate AM-4 consent; P10 remains historical BLOCKED','inputs':rows},'A0')

def stage(c):
    frozen=jload(Path(c['paths']['packet'])/'A0_FREEZE.json');total=0;root=Path(c['paths']['source_home'])
    need(not any(root.iterdir()),'source-home not empty')
    source_rows=jload(c['bindings']['staging']['path'])['inputs']
    originals={staging_destination(p,c):p for p in source_rows}
    need(len(frozen['inputs'])==len(originals) and {p['destination'] for p in frozen['inputs']}==set(originals),'staging receipt membership differs')
    for p in frozen['inputs']:
        need(p['source']==originals[p['destination']]['source'],'staging receipt source differs')
        src=Path(p['source']);need(stable_stat(src)==p['stat'],'source stat changed')
        dest=root/relative(p['destination']);need(not dest.exists(),'duplicate staging destination');dest.parent.mkdir(parents=True,exist_ok=True)
        before=stable_stat(src);total+=before['size'];need(total<=268435456,'A total copy bytes')
        with open(src,'rb') as inf,open(dest,'xb') as outf:
            while b:=inf.read(CHUNK):outf.write(b)
        need(digest(dest)==p['sha256'] and stable_stat(src)==before,'copy identity changed')
    receipt(c,'A_COPY.json',{'files':len(frozen['inputs']),'bytes':total},'A')

def approved_includes(c):
    """Exact literal include inventory plus the owner's twelve digest entries."""
    need(c['bindings']['embedded']['sha256']=='ac397cfd5e0952b01ae1b892997808e95e13d3ac10ecff2e90418e484fab57d7','unreviewed embedded inventory')
    need(c['bindings']['amendments']['sha256']=='3676da94b6ad533a2276271c55c3c47b8c4473170b1200cf0dcae2b362fbda5d','unreviewed AM-1 through AM-4 bytes')
    amendments=jload(c['bindings']['amendments']['path']);inventory=jload(c['bindings']['embedded']['path'])['inputs']
    roots={g['commit']:g['relative_root'] for g in c['git_checkouts']};expected={};digests=set()
    for item in inventory:
        if 'git_tree_entry' in item:
            commit=item['git_commit'];rel=item['git_relative_target'];path=str(PurePosixPath(roots[commit])/relative(rel))
            expected[path]={'relative':path,'algorithm':'git-blob','digest':item['git_tree_entry']['blob'],'source_identifier':commit+':'+rel}
        elif 'archive_member' in item:
            member=item['archive_member'];parts=relative(member).parts
            matches=[r for r in c['registry_roots'] if Path(r['source_relative']).name==parts[0] and r['package_checksum']==item['locked_archive_sha256']]
            need(len(matches)==1,'include archive identity mapping');r=matches[0]
            path=str(PurePosixPath(r['source_relative']).joinpath(*parts[1:]))
            expected[path]={'relative':path,'algorithm':'archive-member','digest':r['package_checksum'],'source_identifier':Path(r['archive']).name+':'+member}
        else:
            need(item.get('enclosing_item',{}).get('name')=='exptfile' and 'conservative_static_tree_entries' in item,'unhandled include site')
    am=amendments['AM-3'];need(am['git_commit']=='04c9c5d9bcebb8d4427d8589fb54d58a55599c14','AM-3 commit')
    names={'zswap/static/'+op+'.'+kind+'.sha256' for op in ('spend','output','sign') for kind in ('prover','verifier','bzkir')}
    names|={'ledger/static/dust/spend.'+kind+'.sha256' for kind in ('prover','verifier','bzkir')}
    need(len(am['digest_files'])==12 and {x['path'] for x in am['digest_files']}==names,'exact AM-3 digest membership')
    for item in am['digest_files']:
        rel=item['path'];path=str(PurePosixPath(roots[am['git_commit']])/relative(rel));digests.add(path)
        expected[path]={'relative':path,'algorithm':'git-blob','digest':item['git_blob'],'source_identifier':am['git_commit']+':'+rel}
    supplied={x['relative']:x for x in c['includes']}
    need(supplied==expected,'include map differs from owner raw/digest identities')
    need(not any(p.endswith(('.zkir','.bzkir','.prover')) for p in expected),'excluded raw payload include')
    return digests

def verify_raw(c):
    digest_paths=approved_includes(c)
    root=Path(c['paths']['source_home']);expected=set();stamps=[]
    for r in c['registry_roots']:
        archive=root/relative(r['archive']);need(digest(archive)==r['package_checksum'],'archive identity')
        dest=root/relative(r['source_relative']);seen=set()
        with tarfile.open(archive,'r:gz') as tf:
            for m in tf:
                parts=relative(m.name).parts;need(parts[0]==dest.name,'archive package root')
                rel=Path(*parts[1:])
                if m.isdir():continue
                need(m.isfile() and not m.issparse(),'unsupported archive member type')
                need(str(rel) not in seen,'duplicate archive member');seen.add(str(rel))
                p=dest/rel;regular(p);need(p.stat().st_size==m.size,'archive member size')
                member=tf.extractfile(m);need(member is not None,'archive stream missing');h=hashlib.sha256()
                while b:=member.read(CHUNK):h.update(b)
                need(digest(p)==h.hexdigest(),'extracted file differs from archive');expected.add(str(p.relative_to(root)))
    for g in c['git_checkouts']:
        base=root/relative(g['relative_root']);regular_rows={row['relative'] for row in g['files'] if row['mode']!=GIT_SYMLINK_MODE}
        for row in g['files']:
            p=base/relative(row['relative'])
            if row['mode']==GIT_SYMLINK_MODE:
                # Proposed AM-4 extension: link text only; the target is never opened, stat'ed or followed.
                text,blob=link_identity(p);need(blob==row['blob'],'Git symlink text identity')
                link_target(row['relative'],text,regular_rows);expected.add(str(p.relative_to(root)));continue
            need(digest(p,True)==row['blob'],'Git tree blob identity')
            need(bool(p.stat().st_mode&0o111)==(row['mode']=='100755'),'Git executable mode mismatch');expected.add(str(p.relative_to(root)))
    for row in c['includes']:
        p=root/relative(row['relative']);need(str(p.relative_to(root)) in expected,'include outside authenticated materialization')
        if row['algorithm']=='git-blob':need(digest(p,True)==row['digest'],'include Git target mismatch')
        else:
            archive_name,member=row['source_identifier'].split(':',1)
            matches=[r for r in c['registry_roots'] if Path(r['archive']).name==archive_name and r['package_checksum']==row['digest']]
            need(len(matches)==1,'include archive origin differs')
            need(str(PurePosixPath(matches[0]['source_relative']).joinpath(*relative(member).parts[1:]))==row['relative'],'include member path differs')
        if row['relative'] in digest_paths:
            with open(p,'rb') as f:prefix=f.read(64)
            need(len(prefix)==64 and all(ch in b'0123456789abcdef' for ch in prefix),'AM-3 first 64 bytes are not lowercase hex')
    staged=jload(Path(c['paths']['packet'])/'A0_FREEZE.json')['inputs']
    for row in staged:need(digest(root/row['destination'])==row['sha256'],'staged input modified');expected.add(row['destination'])
    for row in c['expected_generated_files']:
        p=root/relative(row['relative']);regular(p);expected.add(row['relative'])
        if row['kind']=='cargo-ok':stamps.append({'path':row['relative'],'sha256':digest(p),'bytes':p.stat().st_size})
        # Cargo bookkeeping is recorded as generated, not authenticated source.
    actual=tree_hashes(root,declared_links(c));need(set(actual)==expected,'unexpected or missing source-home file membership')
    receipt(c,'C_RAW_IDENTITIES.json',{'files':actual,'cargo_ok':stamps,'raw_only':True},'C')

def chmod_nofollow(path,directory):
    # fd-based on an O_NOFOLLOW object, so a swapped-in symlink refuses instead of redirecting.
    fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW|os.O_CLOEXEC|(os.O_DIRECTORY if directory else 0))
    try:
        s=os.fstat(fd);need(stat.S_ISDIR(s.st_mode) if directory else stat.S_ISREG(s.st_mode),'freeze object type changed: '+str(path))
        os.fchmod(fd,0o555 if directory else stat.S_IMODE(s.st_mode)&~0o222)
    finally:os.close(fd)

def freeze_source(c):
    root=Path(c['paths']['source_home']);links=declared_links(c);expected=jload(Path(c['paths']['packet'])/'C_RAW_IDENTITIES.json')['files']
    need(tree_hashes(root,links)==expected,'source-home changed before freeze')
    # chmod is supplementary. E/F use read-only bind mounts independently.
    # Declared Git symlinks are never chmod'ed and no chmod here can follow a link.
    files,_,dirs=walk_tree(root,links)
    for p in files:chmod_nofollow(p,False)
    for d in sorted(dirs,key=lambda x:len(x.parts),reverse=True):chmod_nofollow(d,True)
    chmod_nofollow(root,True)
    receipt(c,'D_FREEZE.json',{'H_S':hashlib.sha256(json_bytes(expected)).hexdigest(),'files':expected},'D')

def frozen_source(c):
    frozen=jload(Path(c['paths']['packet'])/'D_FREEZE.json');need(tree_hashes(Path(c['paths']['source_home']),declared_links(c))==frozen['files'],'H_S content/membership changed');return frozen

def unit_identity(package):return package['name']+' '+package['version']+' ('+str(package['source'])+')'

def metadata_check(c):
    frozen_source(c);m=jload(Path(c['paths']['packet'])/'E.stdout');units=jload(c['bindings']['host_units']['path'])['units'];allowed={(u['package_id'],u['kind'],u['path']) for u in units}
    need(len(units)==101 and sum(bool(u['sdk_origin_candidate']) for u in units)==8,'owner 8+93 host superset')
    nodes={n['id']:n for n in m['resolve']['nodes']};observed=[];roots={}
    for pkg in m['packages']:
        if pkg['id'] not in nodes:continue
        root=Path(pkg['manifest_path']).parent;roots[pkg['id']]=str(root)
        for target in pkg['targets']:
            kind='build-script' if 'custom-build' in target['kind'] else 'proc-macro' if 'proc-macro' in target['kind'] else None
            if kind:
                rel=str(Path(target['src_path']).relative_to(root));key=(unit_identity(pkg),kind,rel)
                need(key in allowed,'resolved host unit outside bound superset');observed.append({'package':pkg['id'],'identity':key,'root':str(root),'name':pkg['name'],'target_name':target['name']})
        fs=nodes[pkg['id']]['features']
        if pkg['name']=='midnight-onchain-runtime':need('vendored' in fs,'vendored missing')
        if pkg['name']=='lazy_static':need('spin_no_std' not in fs,'spin_no_std forbidden')
    need(any('midnight-onchain-runtime ' in u['identity'][0] for u in observed),'onchain build feature premise unobserved')
    receipt(c,'E_RESOLVED.json',{'host_units':observed,'roots':roots},'E')

def make_words(text):
    # Rust make dep-info: escaped whitespace/#/backslash and line continuations.
    # Unknown make syntax is refused, not silently omitted.
    text=text.replace('\\\n','');out=[];word='';esc=False
    for ch in text:
        if esc:word+=ch;esc=False
        elif ch=='\\':esc=True
        elif ch.isspace():
            if word:out.append(word);word=''
        elif ch in '$;|':raise Unsupported('unsupported dep-info syntax')
        else:word+=ch
    need(not esc,'trailing dep-info escape')
    if word:out.append(word)
    return out

def depinfo_path(word,worktree,roots=()):
    worktree=absolute(str(worktree))
    need(type(word) is str and '\0' not in word and word!='' and '//' not in word and not word.endswith('/'),'non-normalized dep-info path')
    raw_parts=word[1:].split('/') if word.startswith('/') else word.split('/')
    need(all(p not in ('','.') for p in raw_parts),'non-normalized dep-info path')
    if '..' not in raw_parts:
        p=Path(word) if word.startswith('/') else worktree/word
        return str(absolute(str(p)))
    if word.startswith('/'):
        candidates=[b for b in (worktree,*(absolute(str(r)) for r in roots)) if word.startswith(str(b)+'/')]
        need(bool(candidates),'non-normalized dep-info path')
        base=max(candidates,key=lambda b:len(str(b)))
        rel_parts=word[len(str(base))+1:].split('/')
    else:
        base=worktree;rel_parts=raw_parts
    stack=[];ascended=[]
    for part in rel_parts:
        if part=='..':
            need(bool(stack),'non-normalized dep-info path')
            ascended.append(base.joinpath(*stack));stack.pop()
        else:stack.append(part)
    need(bool(stack),'non-normalized dep-info path')
    norm=base.joinpath(*stack);raw=base/('/'.join(rel_parts))
    for d in ascended:
        need(not any(x.is_symlink() for x in [d,*d.parents]) and d.is_dir() and d.resolve(strict=True)==d,'dep-info parent directory identity: '+str(d))
    need(norm.exists(),'dep-info target missing: '+str(norm));regular(norm);need(raw.resolve(strict=True)==norm,'dep-info path resolution mismatch: '+word)
    return str(norm)

def parse_depinfo(text,worktree,roots=()):
    rows=[]
    for line in text.replace('\\\n','').splitlines():
        if not line or line.startswith('# env-dep:'):continue
        escaped=False;delimiter=None
        for i,ch in enumerate(line):
            if escaped:escaped=False
            elif ch=='\\':escaped=True
            elif ch==':':delimiter=i;break
        need(delimiter is not None,'unrecognized dep-info rule')
        def paths(part):
            return {depinfo_path(word,worktree,roots) for word in make_words(part)}
        rows.append((paths(line[:delimiter]),paths(line[delimiter+1:])))
    return rows

def artifact_dependencies(artifact,records):
    """Dep-info target identity, with verified same-directory Cargo alias copy."""
    result=[];artifact=str(artifact)
    for rows in records.values():
        for targets,dependencies in rows:
            if artifact in targets:result.append(dependencies);continue
            if Path(artifact).name!='build-script-build':continue
            for target in targets:
                p=Path(target)
                if p.parent!=Path(artifact).parent or re.fullmatch(r'build_script_build-[0-9a-f]+',p.name) is None:continue
                regular(p);regular(artifact)
                if p.stat().st_size==Path(artifact).stat().st_size and digest(p)==digest(artifact):result.append(dependencies)
    need(result,'host output lacks exact dep-info target or verified Cargo alias')
    return set().union(*result)


def output_check(c,receipt_name='F_QUARANTINED_OUTPUTS.json'):
    frozen=frozen_source(c)['files'];worktree=Path(c['paths']['worktree']);source_home=Path(c['paths']['source_home']);root=Path(c['paths']['target'])
    worktree_files=set(jload(c['source_manifest']['path'])['files']);resolved=jload(Path(c['paths']['packet'])/'E_RESOLVED.json')['host_units']
    dep_roots=[source_home/g['relative_root'] for g in c['git_checkouts']]+[source_home/r['source_relative'] for r in c['registry_roots']]+[root,*(Path(t) for t in c['readonly_tools']),Path('/usr')]
    included={str(source_home/p['relative']) for p in c['includes']};deps={};host=[];fingerprints=[];out_dirs=[]
    for p in root.rglob('*.d'):
        regular(p);deps[str(p)]=parse_depinfo(p.read_text(),worktree,dep_roots)
        all_paths=set().union(*(t|v for t,v in deps[str(p)]))
        for x in all_paths:
            if x==str(worktree) or x.startswith(str(worktree)+'/'):
                need(str(Path(x).relative_to(worktree)) in worktree_files,'unbound worktree dep-info input: '+x)
            elif x==str(source_home) or x.startswith(str(source_home)+'/'):
                rel=str(Path(x).relative_to(source_home))
                need(rel in frozen and not frozen[rel].startswith('git-symlink:'),'unbound source-home dep-info input: '+x)
        inputs=set().union(*(values for _,values in deps[str(p)]))
        sdk=any('/midnight-' in x or '/midnight-ledger-' in x for x in inputs)
        if sdk:
            for x in inputs:
                if x.startswith(c['paths']['source_home']+'/') and not x.endswith('.rs'):need(x in included,'unbound SDK non-Rust dep-info input')
    for p in walk_files(root):
        is_build=('/build/' in str(p) and (p.name=='build-script-build' or p.name.startswith('build_script_build-')) and p.stat().st_mode&0o111)
        if p.suffix!='.so' and not is_build:continue
        kind='build-script' if is_build else 'proc-macro';inputs=artifact_dependencies(p,deps)
        units=[u for u in resolved if u['identity'][1]==kind and any(x.startswith(u['root']+'/') for x in inputs)]
        need(len(units)==1,'host artifact exact package/kind/source identity ambiguous');u=units[0]
        host.append({'path':str(p.relative_to(root)),'source_root':u['root'],'identity':u['identity'],'name':u['name'],'target_name':u['target_name'],'kind':kind,'sha256':digest(p)})
    for p in walk_files(root):
        if '/.fingerprint/' not in str(p):continue
        label=p.name.removesuffix('.json');directory=p.parent.name
        match=re.fullmatch(r'(.+)-([0-9a-f]{16})',directory)
        need(match is not None,'unknown fingerprint directory identity');name,unit_hash=match.groups()
        if label.startswith('run-build-script-'):
            units=[u for u in resolved if u['name']==name and u['identity'][1]=='build-script']
            need(len(units)==1,'run-build-script fingerprint has ambiguous exact source root');u=units[0]
            need(any(h['identity']==u['identity'] and h['source_root']==u['root'] and h['kind']=='build-script' for h in host),'run-unit lacks dep-info-associated compiled build script')
            # Cargo's run-unit hash names both its fingerprint and OUT_DIR.
            profile=p.parents[2];out=profile/'build'/directory/'out'
            need(out.is_dir() and out.resolve()==out,'run-unit OUT_DIR identity')
            need(out.stat().st_dev==root.stat().st_dev,'OUT_DIR device differs')
            out_dirs.append({'path':str(out.relative_to(root)),'identity':u['identity'],'source_root':u['root'],'files':tree_hashes(out)})
        elif label.startswith('build-script-'):
            candidates=[h for h in host if h['kind']=='build-script' and Path(h['path']).parent.name==directory]
            identities={(h['name'],h['kind'],h['source_root']) for h in candidates}
            need(len(identities)==1 and next(iter(identities))[0]==name,'build-script fingerprint exact identity missing')
            u=candidates[0]
        elif label.startswith('lib-'):
            crate_name=label[4:]
            candidates=[h for h in host if h['kind']=='proc-macro' and h['name']==name and h['target_name'].replace('-','_')==crate_name and Path(h['path']).name.endswith('-'+unit_hash+'.so')]
            if not candidates:
                need(not any(u['name']==name and u['identity'][1]=='proc-macro' and u['target_name'].replace('-','_')==crate_name for u in resolved),'resolved proc-macro fingerprint lacks artifact provenance')
                continue
            identities={(h['name'],h['kind'],h['source_root']) for h in candidates};need(len(identities)==1,'proc-macro fingerprint identity ambiguous');u=candidates[0]
        else:continue
        fingerprints.append({'path':str(p.relative_to(root)),'name':name,'kind':u.get('kind',u['identity'][1]),'source_root':u.get('source_root',u.get('root')),'identity':u['identity']})
    receipt(c,receipt_name,{'all_outputs':tree_hashes(root),'host_outputs':host,'host_fingerprints':fingerprints,'build_script_outputs':out_dirs,'acceptance':'UNACCEPTED: independent result review required; no artifact executed'},'F')


def worker(c,phase,retained_audit=False):
    # A0/A/C/D workers and E/F post-command audits are dispatched. They run inside the same bounded
    # cgroup/sandbox envelope; Cargo receives no Python callback or wrapper flag.
    if phase=='A0':freeze_inputs(c)
    elif phase=='A':stage(c)
    elif phase=='C':verify_raw(c)
    elif phase=='D':freeze_source(c)
    elif phase=='E':metadata_check(c)
    elif phase=='F':output_check(c,'F_RETAINED_QUARANTINED_OUTPUTS.json' if retained_audit else 'F_QUARANTINED_OUTPUTS.json')
    else:raise Unsupported('invalid data worker phase')

def retained_f_audit(c,config_sha):
    start=time.monotonic()
    need(c['mode']=='tier1-and-compile','retained F audit requires tier1-and-compile mode')
    consent(c);supervisor_envelope(c)
    def expired(signum,frame):raise Refusal('whole-job wall cap')
    init_wall=min(c['total_wall_seconds'],c['limits']['F']['wall_seconds'])
    need(init_wall>0,'whole-job wall cap')
    signal.signal(signal.SIGALRM,expired);signal.setitimer(signal.ITIMER_REAL,init_wall)
    try:
        sup_cpu=c['supervisor_cpu_seconds']-(read_numbers(cg_fs(cg_paths(c)[1])/'cpu.stat').get('usage_usec',0)+999999)//1000000
        need(sup_cpu>0,'supervisor CPU cap');resource.setrlimit(resource.RLIMIT_CPU,(sup_cpu,sup_cpu))
        bound_identities(c);owned_paths(c);quotas(c,'F');frozen_source(c)
        p=c['paths'];root=Path(p['execution_root']);packet=Path(p['packet'])
        need((root/'ONE_SHOT_STARTED').is_file(),'one-shot start missing')
        predecessors=prior_receipts(c,'F',False,retained_audit=True)
        need(jload(packet/'TERMINAL_FAILURE.json')=={'message':'phase exit nonzero F','native_outputs_accepted':False,'type':'Refusal'},'unexpected prior terminal failure')
        prior_wall=sum(jload(packet/(ph+'_RESULT.json'))['wall_seconds'] for ph in PHASES[:-1])
        wall_budget=min(c['total_wall_seconds']-prior_wall,c['limits']['F']['wall_seconds'])
        rem_wall=wall_budget-(time.monotonic()-start)
        need(rem_wall>0,'whole-job wall cap');signal.setitimer(signal.ITIMER_REAL,rem_wall)
        for name in ('F_RESULT.json','F_QUARANTINED_OUTPUTS.json','F_RETAINED_AUDIT_RESULT.json','F_RETAINED_QUARANTINED_OUTPUTS.json','TERMINAL_FAILURE_RETAINED_AUDIT.json','F_retained_audit.stdout','F_retained_audit.stderr'):
            need(not (packet/name).exists(),'retained audit output already exists: '+name)
        try:
            result=launch(c,'F',config_sha,retained_audit=True)
            bound_identities(c)
            need(time.monotonic()-start<=wall_budget,'whole-job wall cap')
            result.update({'retained_audit':True,'audit_config_sha256':config_sha,'audit_runner_sha256':c['bindings']['runner']['sha256'],'marker_sha256':digest(root/'ONE_SHOT_STARTED'),'predecessors':predecessors,'quarantined_outputs_sha256':digest(packet/'F_RETAINED_QUARANTINED_OUTPUTS.json'),'native_outputs_accepted':False})
            receipt(c,'F_RETAINED_AUDIT_RESULT.json',result,'F')
        except BaseException as e:
            signal.setitimer(signal.ITIMER_REAL,0)
            try:receipt(c,'TERMINAL_FAILURE_RETAINED_AUDIT.json',{'type':type(e).__name__,'message':str(e),'native_outputs_accepted':False},'F')
            except Exception:pass
            raise
    finally:
        signal.setitimer(signal.ITIMER_REAL,0)

def run(c,config_sha):
    consent(c);supervisor_envelope(c)
    def expired(signum,frame):raise Refusal('whole-job wall cap')
    signal.signal(signal.SIGALRM,expired);signal.setitimer(signal.ITIMER_REAL,c['total_wall_seconds'])
    resource.setrlimit(resource.RLIMIT_CPU,(c['supervisor_cpu_seconds'],c['supervisor_cpu_seconds']))
    bound_identities(c)
    p=c['paths'];root=Path(p['execution_root'])
    # No hidden privilege/setup attempt. Exact empty quota mountpoints and a
    # reviewed supervisor envelope must already exist under a separate grant.
    owned_paths(c)
    for k in ('source_home','target','tmp','packet','empty_home','mask_empty'):
        need(Path(p[k]).is_dir() and not any(Path(p[k]).iterdir()),'required fresh empty owned path: '+k)
        need(Path(p[k]).stat().st_uid==os.getuid(),'output ownership')
    write_new(root/'ONE_SHOT_STARTED',b'no retry\n',64)
    # This marker is never removed, even on failure. No old record/counter edit.
    phases=PHASES if c['mode']=='tier1-and-compile' else PHASES[:-1]
    try:
        for phase in phases:
            bound_identities(c);quotas(c,phase)
            result=launch(c,phase,config_sha)
            bound_identities(c)
            receipt(c,phase+'_RESULT.json',result,phase)
    except BaseException as e:
        signal.setitimer(signal.ITIMER_REAL,0)
        # Failures are terminal; cleanup kills descendants, never deletes evidence
        # or resets the one-shot marker. No auto-repair, retries or phase skipping.
        try:receipt(c,'TERMINAL_FAILURE.json',{'type':type(e).__name__,'message':str(e),'native_outputs_accepted':False},phase)
        except Exception:pass
        raise

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--config',required=True)
    parser.add_argument('--config-sha256',required=True)
    parser.add_argument('--worker-fd',type=int)
    parser.add_argument('--worker',choices=('A0','A','C','D','E','F'))
    parser.add_argument('--precheck',action='store_true')
    parser.add_argument('--retained-f-audit',action='store_true')
    a=parser.parse_args();need(HEX.fullmatch(a.config_sha256) is not None,'external full-config SHA required')
    need(digest(a.config)==a.config_sha256,'final config differs from Root-supplied external anchor')
    c=validate(jload(a.config));need(str(Path(a.config).resolve())==c['paths']['config'],'config path binding')
    executing_identity(c)
    if a.worker:
        need(not a.retained_f_audit or (a.worker=='F' and not a.precheck),'invalid retained audit worker flags')
        verify_worker_context(c,a.worker,a.precheck,a.worker_fd,a.config_sha256,a.retained_f_audit)
        consent(c)
        if a.precheck:
            need(a.worker in ('E','F'),'invalid precheck');frozen_source(c)
        else:worker(c,a.worker,a.retained_f_audit)
    else:
        need(a.worker_fd is None and not a.precheck,'internal flags on supervisor')
        if a.retained_f_audit:retained_f_audit(c,a.config_sha256)
        else:run(c,a.config_sha256)

if __name__=='__main__':
    try:main()
    except (Refusal,OSError,ValueError,KeyError) as e:
        print('TERMINAL_REFUSAL: '+str(e),file=sys.stderr);sys.exit(2)
