"""Harmless isolation experiment: snapshot retained bytes; never execute K/Java.

Not a production shim or native admission. Run under the existing launcher.
"""
import fcntl
import hashlib
import json
import os
from pathlib import Path
import resource
import stat
import subprocess
import time

ROOT = Path('/home/charl/Moriarty')
PINS = ROOT / '.moriarty-dev/k-macro05-trace106-pins.json'


def digest(data):
    return hashlib.sha256(data).hexdigest()


def main():
    pins = json.loads(PINS.read_text())
    binding = json.loads(Path(pins['binding']['path']).read_text())
    files = {str(Path(pins['binding']['kRoot']) / n): h for n, h in binding['sources'].items()}
    files.update({str(Path(pins['binding']['kompiled']) / n): h for n, h in binding['artifacts'].items()})
    files.update({i['path']: i['sha256'] for i in pins['retained']})
    home = '/home/charl'
    dirs = {home, str(ROOT)}
    entries, fds = [], []
    argv = ['/usr/bin/bwrap', '--unshare-user', '--uid', str(os.getuid()), '--gid', str(os.getgid()),
            '--unshare-pid', '--unshare-net', '--die-with-parent', '--disable-userns', '--cap-drop', 'ALL',
            '--ro-bind', '/', '/', '--proc', '/proc', '--dev', '/dev', '--tmpfs', home]
    start = time.monotonic()
    try:
        for name, expected in sorted(files.items()):
            p = Path(name)
            assert p.is_relative_to(home) and stat.S_ISREG(p.lstat().st_mode), name
            data = p.read_bytes()
            assert digest(data) == expected, name
            fd = os.memfd_create('ll01-snapshot-probe', os.MFD_ALLOW_SEALING)
            fds.append(fd)
            with os.fdopen(os.dup(fd), 'wb') as out:
                out.write(data)
            os.lseek(fd, 0, os.SEEK_SET)
            seals = fcntl.F_SEAL_WRITE | fcntl.F_SEAL_GROW | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_SEAL
            fcntl.fcntl(fd, fcntl.F_ADD_SEALS, seals)
            assert fcntl.fcntl(fd, fcntl.F_GET_SEALS) == seals
            mode = stat.S_IMODE(p.stat().st_mode)
            entries.append({'path': name, 'sha256': expected, 'bytes': len(data), 'mode': mode, 'fd': fd})
            dirs.update(str(parent) for parent in p.parents if parent.is_relative_to(home))
        for directory in sorted(dirs, key=lambda s: (len(Path(s).parts), s)):
            if directory != home:
                argv += ['--dir', directory]
        for item in entries:
            argv += ['--perms', oct(item['mode'] & ~0o222)[2:], '--ro-bind-data', str(item['fd']), item['path']]
        argv += ['--remount-ro', home, '--tmpfs', '/tmp', '--chdir', str(ROOT), '--', '/usr/bin/python3', '-I', '-B', '-c']
        child = r'''
import ctypes, errno, hashlib, json, os, pathlib, resource, socket, sys
entries=json.load(sys.stdin); home=pathlib.Path('/home/charl')
bad=[]
for item in entries:
 p=pathlib.Path(item['path'])
 if hashlib.sha256(p.read_bytes()).hexdigest()!=item['sha256']: bad.append(str(p))
assert not bad,bad
assert not (home/'.local/lib/kframework/java').exists()
assert not (home/'.kserver/socket').exists()
denied={}
for name in [home/'.kserver',home/'.local',pathlib.Path(entries[0]['path'])]:
 try:
  if name==pathlib.Path(entries[0]['path']): name.write_text('CHANGED')
  else: name.mkdir()
 except OSError as e: denied[str(name)]=e.errno
assert len(denied)==3 and set(denied.values())=={errno.EROFS},denied
pathlib.Path('/tmp/ll01-probe-write').write_text('private temporary data')
network_socket=socket.socket(); network_socket.settimeout(.2)
network=network_socket.connect_ex(('127.0.0.1',2113)); network_socket.close()
assert network!=0
libc=ctypes.CDLL(None,use_errno=True)
nested=libc.unshare(0x10000000); nested_errno=ctypes.get_errno()
assert nested==-1
mountinfo=pathlib.Path('/proc/self/mountinfo').read_text()
assert ' /home/charl ro,' in mountinfo
print(json.dumps({'verified_files':len(entries),'read_only_errors':denied,'home':os.environ['HOME'],
 'cwd':os.getcwd(),'uid':os.getuid(),'stack':resource.getrlimit(resource.RLIMIT_STACK),
 'network_connect_errno':network,'nested_userns_errno':nested_errno,
 'cap_eff':next(x for x in pathlib.Path('/proc/self/status').read_text().splitlines() if x.startswith('CapEff:')),
 'pid':os.getpid(),'fd_count':len(list(pathlib.Path('/proc/self/fd').iterdir()))}))
'''
        entries_json = json.dumps(entries, separators=(',', ':'))
        argv += [child]
        env = {'HOME': home, 'PATH': '/usr/bin:/bin', 'LANG': 'C.UTF-8', 'LC_ALL': 'C.UTF-8'}
        r = subprocess.run(argv, pass_fds=fds, env=env, input=entries_json, text=True, capture_output=True, timeout=15)
        print(json.dumps({'scope': 'Harmless Python snapshot reads only; no K, Java, strace or native diagnostic',
                          'files': len(entries), 'bytes': sum(i['bytes'] for i in entries),
                          'bwrap_sha256': digest(Path('/usr/bin/bwrap').read_bytes()),
                          'source_sha256': digest(Path(__file__).read_bytes()),
                          'pins_sha256': digest(PINS.read_bytes()),
                          'argv_bytes': sum(len(s.encode())+1 for s in argv), 'argv_prefix': argv[:-1],
                          'child_source_sha256': digest(child.encode()), 'entries_sha256': digest(entries_json.encode()),
                          'seconds': time.monotonic()-start, 'exit_code': r.returncode,
                          'stdout': r.stdout, 'stderr': r.stderr,
                          'outer_stack': resource.getrlimit(resource.RLIMIT_STACK)}, indent=2))
        return r.returncode
    finally:
        for fd in fds:
            os.close(fd)


if __name__ == '__main__':
    raise SystemExit(main())
