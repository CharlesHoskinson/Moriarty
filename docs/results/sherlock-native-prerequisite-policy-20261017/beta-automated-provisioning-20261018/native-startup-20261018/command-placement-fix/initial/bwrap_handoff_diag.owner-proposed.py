#!/usr/bin/python3
"""Bounded host-only A0 listener-handoff discriminator (Step A, B1, B2).

Runs under UID/GID 1000, CapEff=0, NoNewPrivs=1 via setpriv/env -i.
Touches zero SDK/Cargo/worktree/archive/native-root/marker paths.
"""
import array
import errno
import fcntl
import hashlib
import json
import os
from pathlib import Path
import resource
import selectors
import signal
import socket
import stat
import subprocess
import sys
import tempfile
import time

GLOBAL_WALL_SECONDS = 9.0
VARIANT_WALL_SECONDS = 4.0
OUTPUT_CAP_BYTES = 4096
STREAM_CAP_BYTES = 768
PER_PROC_AS_BYTES = 268435456
PER_PROC_DATA_BYTES = 134217728
PER_PROC_FSIZE_BYTES = 65536

MASKS = tuple(sorted((
    '/run',
    '/var/lib/docker',
    '/nix/var/nix/daemon-socket',
    '/nix/var/determinate',
    '/sys/fs/cgroup',
    '/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014',
), key=lambda x: (len(Path(x).parts), x)))

# Exact verbatim TRAMPOLINE bytes from bounded_sdk_compile.py (94810a65..., L583-601).
TRAMPOLINE = (
    b"import array,ctypes as ct,fcntl,json,os,socket,sys\n"
    b"stage_fd,control_fd,token_fd=map(int,sys.argv[1:4]);argv=sys.argv[4:]\n"
    b"seals=fcntl.F_SEAL_WRITE|fcntl.F_SEAL_SHRINK|fcntl.F_SEAL_GROW|fcntl.F_SEAL_SEAL\n"
    b"if fcntl.fcntl(stage_fd,fcntl.F_GET_SEALS)&seals!=seals:raise RuntimeError('unsealed stage')\n"
    b"if token_fd>=0 and fcntl.fcntl(token_fd,fcntl.F_GET_SEALS)&seals!=seals:raise RuntimeError('unsealed worker token')\n"
    b"class Filter(ct.Structure):_fields_=[('code',ct.c_ushort),('jt',ct.c_ubyte),('jf',ct.c_ubyte),('k',ct.c_uint)]\n"
    b"class Program(ct.Structure):_fields_=[('len',ct.c_ushort),('filter',ct.POINTER(Filter))]\n"
    b"# Default allow; individual network/uring syscalls notify. AF_UNIX sendmsg is\n"
    b"# retained for SCM_RIGHTS handoff; no network FD is passed to the target.\n"
    b"ins=[(0x20,0,0,4),(0x15,1,0,0xc000003e),(0x06,0,0,0x80000000),\n"
    b"     (0x20,0,0,0),(0x45,0,1,0x40000000),(0x06,0,0,0x80000000)]\n"
    b"for syscall in (42,425,426,427):ins.extend([(0x15,0,1,syscall),(0x06,0,0,0x7fc00000)])\n"
    b"ins.extend([(0x15,2,0,41),(0x15,1,0,53),(0x06,0,0,0x7fff0000),\n"
    b"            (0x20,0,0,16),(0x15,0,1,1),(0x06,0,0,0x7fff0000),(0x06,0,0,0x7fc00000)])\n"
    b"filters=(Filter*len(ins))(*(Filter(*x) for x in ins));program=Program(len(ins),filters);libc=ct.CDLL(None,use_errno=True)\n"
    b"if libc.prctl(38,1,0,0,0)!=0:raise RuntimeError('no_new_privs failed')\n"
    b"listener=libc.syscall(317,1,8,ct.byref(program))\n"
    b"if listener<0:raise RuntimeError('seccomp NEW_LISTENER unavailable')\n"
    b"control=socket.socket(fileno=control_fd)\n"
    b"control.sendmsg([b'L'],[(socket.SOL_SOCKET,socket.SCM_RIGHTS,array.array('i',[listener]))])\n"
    b"os.close(listener);control.close();os.close(stage_fd)\n"
    b"keep={0,1,2}\n"
    b"if token_fd>=0:keep.add(token_fd);os.set_inheritable(token_fd,True)\n"
    b"for item in os.listdir('/proc/self/fd'):\n"
    b" fd=int(item)\n"
    b" if fd not in keep:\n"
    b"  try:os.close(fd)\n"
    b"  except OSError:pass\n"
    b"if not argv or not argv[0].startswith('/'):raise RuntimeError('absolute target required')\n"
    b"os.execve(argv[0],argv,dict(os.environ))\n"
)


def apply_limits():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    resource.setrlimit(resource.RLIMIT_FSIZE, (PER_PROC_FSIZE_BYTES, PER_PROC_FSIZE_BYTES))
    resource.setrlimit(resource.RLIMIT_DATA, (PER_PROC_DATA_BYTES, PER_PROC_DATA_BYTES))
    resource.setrlimit(resource.RLIMIT_AS, (PER_PROC_AS_BYTES, PER_PROC_AS_BYTES))
    resource.setrlimit(resource.RLIMIT_CPU, (5, 5))


def sealed_memfd(name, data):
    fd = os.memfd_create(name, os.MFD_CLOEXEC | os.MFD_ALLOW_SEALING)
    try:
        offset = 0
        while offset < len(data):
            offset += os.write(fd, data[offset:])
        os.lseek(fd, 0, os.SEEK_SET)
        seals = fcntl.F_SEAL_WRITE | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_GROW | fcntl.F_SEAL_SEAL
        fcntl.fcntl(fd, fcntl.F_ADD_SEALS, seals)
        return fd
    except BaseException:
        os.close(fd)
        raise


def mode_kind(m):
    if stat.S_ISDIR(m):
        return 'dir'
    if stat.S_ISREG(m):
        return 'reg'
    if stat.S_ISLNK(m):
        return 'lnk'
    if stat.S_ISSOCK(m):
        return 'sock'
    if stat.S_ISFIFO(m):
        return 'fifo'
    if stat.S_ISCHR(m) or stat.S_ISBLK(m):
        return 'dev'
    return 'other'


def step_a_mask_lstats():
    out = {}
    for p in MASKS:
        try:
            st = os.lstat(p)
            out[p] = {
                'exists': True,
                'mode_octal': oct(st.st_mode),
                'kind': mode_kind(st.st_mode),
                'uid': st.st_uid,
                'gid': st.st_gid,
            }
        except OSError as e:
            out[p] = {
                'exists': False,
                'errno': e.errno,
                'error': errno.errorcode.get(e.errno, type(e).__name__),
            }
    return out


def close_quiet(fd):
    if fd is not None and fd >= 0:
        try:
            os.close(fd)
        except OSError:
            pass


def run_variant(name, mask_empty, use_masks, global_deadline):
    t0 = time.monotonic()
    deadline = min(global_deadline, t0 + VARIANT_WALL_SECONDS)
    parent = child = proc = sel = None
    fd = stage_fd = token_fd = -1
    bufs = {'stdout': bytearray(), 'stderr': bytearray()}
    trunc = {'stdout': False, 'stderr': False}
    handoff = {
        'received_msg_hex': None,
        'flags': None,
        'scm_rights_fds': 0,
        'error': None,
    }
    status = 'OK'
    rc = None
    try:
        parent, child = socket.socketpair()
        stage_fd = sealed_memfd('sdk-post-bwrap-stage', TRAMPOLINE)
        token_fd = sealed_memfd('sdk-internal-worker-context', b'{"diag":true}\n')
        args = [
            '--die-with-parent', '--new-session', '--unshare-user', '--unshare-pid',
            '--unshare-net', '--unshare-ipc', '--unshare-uts', '--disable-userns',
            '--cap-drop', 'ALL', '--ro-bind', '/', '/',
            '--size', '16777216', '--tmpfs', '/home/charl',
        ]
        if use_masks:
            for m in MASKS:
                args += ['--ro-bind', mask_empty, m]
        args += ['--proc', '/proc', '--remount-ro', '/proc', '--dev', '/dev', '--remount-ro', '/dev']
        if use_masks:
            for m in MASKS:
                args += ['--ro-bind', mask_empty, m]
        args += [
            '--remount-ro', '/home/charl', '--clearenv',
            '--setenv', 'PATH', '/usr/bin:/bin', '--setenv', 'LANG', 'C',
            '--chdir', '/', '--',
            '/usr/bin/python3', '-I', '-B', '-S', '/proc/self/fd/' + str(stage_fd),
            str(stage_fd), str(child.fileno()), str(token_fd), '/bin/true',
        ]
        fd = sealed_memfd('sdk-bwrap-argv', b'\0'.join(x.encode() for x in args) + b'\0')

        def enter():
            parent.close()
            os.setsid()

        proc = subprocess.Popen(
            ['/usr/bin/bwrap', '--args', str(fd)],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env={},
            pass_fds=(fd, stage_fd, child.fileno(), token_fd),
            preexec_fn=enter,
            close_fds=True,
        )
        child.close()
        child = None
        sel = selectors.DefaultSelector()
        os.set_blocking(parent.fileno(), False)
        sel.register(parent, selectors.EVENT_READ, 'handoff')
        for sname, stream in (('stdout', proc.stdout), ('stderr', proc.stderr)):
            os.set_blocking(stream.fileno(), False)
            sel.register(stream, selectors.EVENT_READ, sname)

        while sel.get_map():
            rem = deadline - time.monotonic()
            if rem <= 0:
                status = 'TIMEOUT'
                break
            events = sel.select(min(0.05, rem))
            for key, _ in events:
                tag = key.data
                if tag == 'handoff':
                    try:
                        msg, anc, flags, _ = parent.recvmsg(
                            1, socket.CMSG_SPACE(array.array('i').itemsize)
                        )
                        handoff['received_msg_hex'] = msg.hex()
                        handoff['flags'] = int(flags)
                        rec_fds = []
                        for level, kind, data in anc:
                            if level == socket.SOL_SOCKET and kind == socket.SCM_RIGHTS:
                                arr = array.array('i')
                                arr.frombytes(data)
                                rec_fds.extend(arr)
                        for rfd in rec_fds:
                            close_quiet(rfd)
                        handoff['scm_rights_fds'] = len(rec_fds)
                    except OSError as e:
                        handoff['error'] = errno.errorcode.get(e.errno, str(e))
                    try:
                        sel.unregister(parent)
                    except Exception:
                        pass
                else:
                    try:
                        chunk = os.read(key.fileobj.fileno(), 4096)
                    except OSError:
                        chunk = b''
                    if not chunk:
                        try:
                            sel.unregister(key.fileobj)
                        except Exception:
                            pass
                        continue
                    b = bufs[tag]
                    room = STREAM_CAP_BYTES - len(b)
                    if room > 0:
                        b.extend(chunk[:room])
                    if len(chunk) > room:
                        trunc[tag] = True

        if proc.poll() is None:
            rem = max(0.0, deadline - time.monotonic())
            try:
                proc.wait(timeout=min(0.5, rem))
            except subprocess.TimeoutExpired:
                status = 'TIMEOUT'
        rc = proc.poll()
    except BaseException as e:
        status = 'EXCEPTION:' + type(e).__name__
        handoff['error'] = str(e)[:256]
    finally:
        if sel is not None:
            try:
                sel.close()
            except Exception:
                pass
        if proc is not None:
            if proc.poll() is None:
                try:
                    os.killpg(proc.pid, signal.SIGKILL)
                except OSError:
                    try:
                        proc.kill()
                    except OSError:
                        pass
                try:
                    proc.wait(timeout=1.0)
                except Exception:
                    pass
            rc = proc.returncode
            for stream in (proc.stdout, proc.stderr):
                if stream is not None:
                    try:
                        stream.close()
                    except Exception:
                        pass
        if parent is not None:
            try:
                parent.close()
            except Exception:
                pass
        if child is not None:
            try:
                child.close()
            except Exception:
                pass
        for mfd in (fd, stage_fd, token_fd):
            close_quiet(mfd)
    return {
        'variant': name,
        'status': status,
        'returncode': rc,
        'elapsed_ms': round((time.monotonic() - t0) * 1000.0, 2),
        'handoff': handoff,
        'stdout': bufs['stdout'].decode('utf-8', errors='replace'),
        'stderr': bufs['stderr'].decode('utf-8', errors='replace'),
        'stdout_truncated': trunc['stdout'],
        'stderr_truncated': trunc['stderr'],
    }


def main():
    t_start = time.monotonic()
    global_deadline = t_start + GLOBAL_WALL_SECONDS
    apply_limits()

    def on_alarm(signum, frame):
        raise TimeoutError('global diagnostic wall cap exceeded')

    signal.signal(signal.SIGALRM, on_alarm)
    signal.setitimer(signal.ITIMER_REAL, GLOBAL_WALL_SECONDS)
    report = {
        'schema': 'sherlock-a0-handoff-host-diag/1',
        'uid': os.getuid(),
        'gid': os.getgid(),
        'trampoline_sha256': hashlib.sha256(TRAMPOLINE).hexdigest(),
        'step_a_mask_lstat': None,
        'B1_no_masks': {'status': 'NOT_RUN'},
        'B2_with_masks': {'status': 'NOT_RUN'},
        'top_error': None,
        'output_truncated': False,
    }
    try:
        report['step_a_mask_lstat'] = step_a_mask_lstats()
        with tempfile.TemporaryDirectory(prefix='bwrap-diag-', dir='/tmp') as scratch:
            mask_empty = os.path.join(scratch, 'mask-empty')
            os.mkdir(mask_empty, 0o700)
            report['B1_no_masks'] = run_variant('B1', mask_empty, False, global_deadline)
            if time.monotonic() < global_deadline:
                report['B2_with_masks'] = run_variant('B2', mask_empty, True, global_deadline)
    except BaseException as e:
        report['top_error'] = type(e).__name__ + ': ' + str(e)[:256]
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)

    raw = (json.dumps(report, sort_keys=True, separators=(',', ':')) + '\n').encode('utf-8')
    if len(raw) > OUTPUT_CAP_BYTES:
        report['output_truncated'] = True
        for k in ('B1_no_masks', 'B2_with_masks'):
            if isinstance(report.get(k), dict):
                for s in ('stdout', 'stderr'):
                    if s in report[k] and len(report[k][s]) > 256:
                        report[k][s] = report[k][s][:256]
                        report[k][s + '_truncated'] = True
        raw = (json.dumps(report, sort_keys=True, separators=(',', ':')) + '\n').encode('utf-8')[:OUTPUT_CAP_BYTES]
    sys.stdout.buffer.write(raw)
    sys.stdout.buffer.flush()


if __name__ == '__main__':
    main()
