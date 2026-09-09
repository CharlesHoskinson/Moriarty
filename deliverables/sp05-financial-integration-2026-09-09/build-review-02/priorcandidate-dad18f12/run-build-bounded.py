#!/usr/bin/env python3
"""Fixed SP05 build containment. Admission and terminal cgroup review remain external.

One reviewed request per invocation, no retries or unit resets. A wrapper result
never accepts a build. Abrupt cgroup termination can lose volatile partial files.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import stat
import subprocess
import sys
import time

UNIT = 'moriarty-sp05-financial-build.service'
QUOTA = 1073741824
MEMORY = 4294967296
BUILDER = Path(__file__).resolve().with_name('build-proven.mjs')
NODE = '/usr/local/bin/node'


def require(condition, message):
    if not condition:
        raise ValueError(message)


def safe_path(value, missing=False):
    p = Path(value)
    require(p.is_absolute() and str(p) == os.path.normpath(value), 'normalized absolute path required')
    for part in [*reversed(p.parents), p]:
        if not part.exists():
            require(missing and part == p, 'missing ancestor/path')
        else:
            require(not part.is_symlink(), 'symlink path rejected')
    require(not p.is_symlink(), 'dangling symlink rejected')
    return p


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def exclusive(path, data):
    with open(path, 'x', encoding='utf8') as f:
        os.chmod(path, 0o600)
        json.dump(data, f, sort_keys=True)
        f.write('\n')
        f.flush()
        os.fsync(f.fileno())
    fd = os.open(path.parent, os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def validate(request_path, expected_hash, result_path):
    request_path = safe_path(request_path)
    require(request_path.is_file() and digest(request_path) == expected_hash, 'request hash mismatch')
    request = json.loads(request_path.read_text())
    require('commandAdapter' not in request, 'adapter prohibited')
    kind = request['case']
    require(kind in ('loan', 'swap'), 'unsupported case')
    output = safe_path(request['outputDir'], missing=True)
    require(not output.exists(), 'output exists')
    parent = output.parent
    require(parent.stat().st_uid == os.getuid() and not list(parent.iterdir()), 'output parent must be owned and empty')
    require(stat.S_IMODE(parent.stat().st_mode) == 0o700, 'output parent must be private mode 0700')
    admission = request['admission']
    require(admission['allowFullBuild'] is True and admission['case'] == kind and admission['outputDir'] == str(output), 'admission mismatch')
    require(admission['maxAttempts'] == 1 and 0 < admission['maxCompileMs'] <= 300000, 'compile allowance exceeded')
    require(0 < admission['deadlineMs'] - time.time() * 1000 <= 300000, 'deadline outside fixed compile window')
    refs = [admission['resourceRecord'], *admission['reviews']]
    require(len(admission['reviews']) >= 2, 'two bound reviews required; external reviewer authentication still required')
    outside = [request_path, safe_path(result_path, missing=True)]
    records = []
    for ref in refs:
        path = safe_path(ref['path'])
        require(path.is_file() and digest(path) == ref['sha256'], 'bound record mismatch')
        records.append(json.loads(path.read_text()))
        outside.append(path)
    resource = records[0]
    for key in ('case', 'outputDir', 'maxCompileMs', 'deadlineMs', 'maxAttempts'):
        require(resource[key] == admission[key], 'resource mismatch: ' + key)
    require(resource['id'] == admission['resourceId'] and resource['sourceCandidateSha'] == admission['candidateHash'], 'resource identity mismatch')
    for review in records[1:]:
        require(review['sourceCandidateSha'] == admission['candidateHash'] and review['verdict'] == 'APPROVED', 'review mismatch')
    attempt = safe_path(resource['attemptFile'], missing=True)
    require(not attempt.exists(), 'attempt already charged')
    outside.append(attempt)
    for path in outside:
        require(not path.is_relative_to(parent), 'persistent evidence inside volatile output parent')
    require(not Path(result_path).exists(), 'result exists')
    return request, parent


def retain(source, mirror, quota=QUOTA):
    """Bound logical retained bytes independently of sparse tmpfs allocation.

    Preflight the complete tree before creating output. Copy from no-follow,
    identity-checked descriptors with one shared budget, including on growth.
    """
    flags = os.O_RDONLY | os.O_NOFOLLOW
    total = 0
    def inspect(directory):
        nonlocal total
        plan = []
        for name in sorted(os.listdir(directory)):
            info = os.stat(name, dir_fd=directory, follow_symlinks=False)
            if stat.S_ISDIR(info.st_mode):
                child = os.open(name, flags | os.O_DIRECTORY, dir_fd=directory)
                try:
                    require(identity(os.fstat(child)) == identity(info), 'directory changed')
                    children = inspect(child)
                finally:
                    os.close(child)
                plan.append((name, info, children))
            else:
                require(stat.S_ISREG(info.st_mode) and info.st_nlink == 1, 'nonregular/hardlinked asset rejected')
                total += info.st_size
                require(total <= quota, 'logical retention quota exceeded')
                plan.append((name, info, None))
        return plan
    def identity(info):
        return (info.st_dev, info.st_ino)
    remaining = quota
    def copy(directory, destination, plan):
        nonlocal remaining
        for name, before, children in plan:
            fd = os.open(name, flags | (os.O_DIRECTORY if children is not None else 0), dir_fd=directory)
            try:
                current = os.fstat(fd)
                require(identity(current) == identity(before), 'source identity changed')
                target = destination / name
                if children is not None:
                    target.mkdir(mode=0o700)
                    copy(fd, target, children)
                else:
                    require(stat.S_ISREG(current.st_mode) and current.st_nlink == 1 and current.st_size == before.st_size,
                            'source size/type/link changed')
                    file_remaining = before.st_size
                    with target.open('xb') as dst:
                        while True:
                            chunk = os.read(fd, min(65536, remaining + 1, file_remaining + 1))
                            if not chunk:
                                break
                            require(len(chunk) <= remaining and len(chunk) <= file_remaining, 'source grew beyond retention budget')
                            dst.write(chunk)
                            remaining -= len(chunk)
                            file_remaining -= len(chunk)
                        after = os.fstat(fd)
                        require(file_remaining == 0 and after.st_size == before.st_size and after.st_nlink == 1,
                                'source changed during retention')
                        dst.flush()
                        os.fsync(dst.fileno())
            finally:
                os.close(fd)
        fd = os.open(destination, os.O_DIRECTORY | os.O_NOFOLLOW)
        try:
            os.fsync(fd)
        finally:
            os.close(fd)
    root = os.open(source, flags | os.O_DIRECTORY)
    try:
        plan = inspect(root)
        copy(root, mirror, plan)
    finally:
        os.close(root)


def quota_run(parent, mirror, command, quota=QUOTA):
    """Bound the designated output tree, not arbitrary malicious host writes.

    The host output tree remains reachable only through a noninherited descriptor
    until the child terminates; expose its retention mirror only afterwards.
    """
    def mount(*args):
        subprocess.run(['mount', *args], check=True)
    mount('--make-rprivate', '/')
    require(not mirror.exists() and not mirror.is_symlink(), 'mirror must be absent during compiler execution')
    host = os.open(parent, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    os.set_inheritable(host, False)
    mounted = mirrored = False
    try:
        mount('-t', 'tmpfs', '-o', f'size={quota},nosuid,nodev,mode=0700', 'sp05-build', str(parent))
        mounted = True
        try:
            return subprocess.run(command, check=False, close_fds=True).returncode
        finally:
            mirror.mkdir(mode=0o700)
            # mount must see this one descriptor, but the compiler never does.
            subprocess.run(['mount', '--no-canonicalize', '--bind', f'/proc/self/fd/{host}', str(mirror)],
                           check=True, pass_fds=(host,))
            mirrored = True
            retain(parent, mirror, quota)
    finally:
        try:
            if mirrored:
                subprocess.run(['umount', str(mirror)], check=True)
        finally:
            try:
                if mounted:
                    subprocess.run(['umount', str(parent)], check=True)
            finally:
                os.close(host)


def verify_cgroup():
    relative = next(line[3:] for line in Path('/proc/self/cgroup').read_text().splitlines() if line.startswith('0::'))
    require(relative.endswith('/' + UNIT), 'wrong service cgroup')
    group = Path('/sys/fs/cgroup') / relative.lstrip('/')
    require((group / 'memory.max').read_text().strip() == str(MEMORY), 'MemoryMax not active')
    require((group / 'memory.swap.max').read_text().strip() == '0', 'MemorySwapMax not active')
    settings = subprocess.check_output(['systemctl', '--user', 'show', UNIT, '--property=RuntimeMaxUSec,KillMode'], text=True)
    require('RuntimeMaxUSec=5min 30s' in settings and 'KillMode=control-group' in settings, 'service runtime/kill containment mismatch')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--request', required=True)
    parser.add_argument('--request-sha256', required=True)
    parser.add_argument('--result', required=True)
    parser.add_argument('--inner', action='store_true', help=argparse.SUPPRESS)
    args = parser.parse_args()
    request, parent = validate(args.request, args.request_sha256, args.result)
    mirror = parent.with_name(parent.name + '.sp05-mirror')
    safe_path(str(mirror), missing=True)
    require(not mirror.exists(), 'mirror already exists')
    if args.inner:
        verify_cgroup()
        try:
            return quota_run(parent, mirror, [NODE, str(BUILDER), '--request', args.request])
        finally:
            if mirror.exists():
                mirror.rmdir()
    marker = safe_path(args.request + '.bounded-launch.json', missing=True)
    require(not marker.is_relative_to(parent), 'launch marker inside quota')
    exclusive(marker, {'schema': 'moriarty.sp05-build-launch/1', 'requestSha256': args.request_sha256,
                       'case': request['case'], 'unit': UNIT, 'startedAtMs': int(time.time()*1000),
                       'accepted': False, 'attemptMayBeCharged': True})
    command = ['systemd-run', '--user', '--wait', '--unit=' + UNIT,
               '--property=MemoryMax=' + str(MEMORY), '--property=MemorySwapMax=0',
               '--property=RuntimeMaxSec=330', '--property=KillMode=control-group',
               '--property=TimeoutStopSec=0', '--property=SendSIGKILL=yes',
               'unshare', '--user', '--map-root-user', '--mount', '--fork',
               sys.executable, str(Path(__file__).resolve()), '--inner', '--request', args.request,
               '--request-sha256', args.request_sha256, '--result', args.result]
    code = None
    try:
        code = subprocess.run(command, check=False).returncode
        return code
    finally:
        exclusive(Path(args.result), {'schema': 'moriarty.sp05-build-containment/1',
            'requestSha256': args.request_sha256, 'unit': UNIT, 'exitCode': code,
            'status': 'REQUIRES_TERMINAL_AND_ARTIFACT_REVIEW' if code == 0 else 'FAILED_OR_INCOMPLETE',
            'accepted': False, 'terminalCgroupVerified': False,
            'volatilePartialLossPossible': code != 0, 'finishedAtMs': int(time.time()*1000)})


if __name__ == '__main__':
    try:
        sys.exit(main())
    except Exception as error:
        print('SP05 bounded build failed: ' + str(error), file=sys.stderr)
        sys.exit(1)
