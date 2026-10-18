#!/usr/bin/env python3
"""Bounded host-only discriminator for bwrap P1/P2 eventfd exit ordering vs proc.wait()."""
import array
import ctypes
import fcntl
import hashlib
import json
import os
import resource
import selectors
import shutil
import signal
import socket
import struct
import subprocess
import sys
import tempfile
import time

EXPECTED_TRAMPOLINE_SHA256 = "fccbc0bf539c21f5afc1ead78cf821ba6aa1042cc2388438ad17b415732932b0"

TRAMPOLINE = b"import array,ctypes as ct,fcntl,json,os,socket,sys\nstage_fd,control_fd,token_fd=map(int,sys.argv[1:4]);argv=sys.argv[4:]\nseals=fcntl.F_SEAL_WRITE|fcntl.F_SEAL_SHRINK|fcntl.F_SEAL_GROW|fcntl.F_SEAL_SEAL\nif fcntl.fcntl(stage_fd,fcntl.F_GET_SEALS)&seals!=seals:raise RuntimeError('unsealed stage')\nif token_fd>=0 and fcntl.fcntl(token_fd,fcntl.F_GET_SEALS)&seals!=seals:raise RuntimeError('unsealed worker token')\nclass Filter(ct.Structure):_fields_=[('code',ct.c_ushort),('jt',ct.c_ubyte),('jf',ct.c_ubyte),('k',ct.c_uint)]\nclass Program(ct.Structure):_fields_=[('len',ct.c_ushort),('filter',ct.POINTER(Filter))]\n# Default allow; individual network/uring syscalls notify. AF_UNIX sendmsg is\n# retained for SCM_RIGHTS handoff; no network FD is passed to the target.\nins=[(0x20,0,0,4),(0x15,1,0,0xc000003e),(0x06,0,0,0x80000000),\n     (0x20,0,0,0),(0x45,0,1,0x40000000),(0x06,0,0,0x80000000)]\nfor syscall in (42,425,426,427):ins.extend([(0x15,0,1,syscall),(0x06,0,0,0x7fc00000)])\nins.extend([(0x15,2,0,41),(0x15,1,0,53),(0x06,0,0,0x7fff0000),\n            (0x20,0,0,16),(0x15,0,1,1),(0x06,0,0,0x7fff0000),(0x06,0,0,0x7fc00000)])\nfilters=(Filter*len(ins))(*(Filter(*x) for x in ins));program=Program(len(ins),filters);libc=ct.CDLL(None,use_errno=True)\nif libc.prctl(38,1,0,0,0)!=0:raise RuntimeError('no_new_privs failed')\nlistener=libc.syscall(317,1,8,ct.byref(program))\nif listener<0:raise RuntimeError('seccomp NEW_LISTENER unavailable')\ncontrol=socket.socket(fileno=control_fd)\ncontrol.sendmsg([b'L'],[(socket.SOL_SOCKET,socket.SCM_RIGHTS,array.array('i',[listener]))])\nos.close(listener);control.close();os.close(stage_fd)\nkeep={0,1,2}\nif token_fd>=0:keep.add(token_fd);os.set_inheritable(token_fd,True)\nfor item in os.listdir('/proc/self/fd'):\n fd=int(item)\n if fd not in keep:\n  try:os.close(fd)\n  except OSError:pass\nif not argv or not argv[0].startswith('/'):raise RuntimeError('absolute target required')\nos.execve(argv[0],argv,dict(os.environ))\n"

import select

PR_SET_CHILD_SUBREAPER = 36
SO_PASSCRED = getattr(socket, "SO_PASSCRED", 16)
SCM_CREDENTIALS = getattr(socket, "SCM_CREDENTIALS", 2)
def inspect_listener(fd):
    poll = select.poll()
    poll.register(fd, select.POLLIN | select.POLLPRI | select.POLLERR | select.POLLHUP)
    revents = 0
    for _, ev in poll.poll(0):
        revents |= ev
    bad = bool(revents & (select.POLLIN | select.POLLPRI | select.POLLERR | select.POLLNVAL))
    return {
        "revents": revents,
        "pollin": bad,
        "pollhup": bool(revents & select.POLLHUP),
        "pollerr": bool(revents & (select.POLLERR | select.POLLPRI | select.POLLNVAL)),
    }


def apply_bounds():
    for res, limit in (
        (resource.RLIMIT_CORE, 0),
        (resource.RLIMIT_FSIZE, 65536),
        (resource.RLIMIT_DATA, 67108864),
        (resource.RLIMIT_AS, 134217728),
        (resource.RLIMIT_NOFILE, 64),
        (resource.RLIMIT_CPU, 2),
    ):
        try:
            soft, hard = resource.getrlimit(res)
            new_hard = min(hard, limit) if hard != resource.RLIM_INFINITY else limit
            resource.setrlimit(res, (min(limit, new_hard), new_hard))
        except (ValueError, OSError):
            pass
    signal.signal(signal.SIGALRM, lambda s, f: (_ for _ in ()).throw(TimeoutError("4s internal wall cap")))
    signal.signal(signal.SIGTERM, lambda s, f: (_ for _ in ()).throw(TimeoutError("SIGTERM received")))
    signal.setitimer(signal.ITIMER_REAL, 4.0)


def sealed_memfd(name, data):
    fd = os.memfd_create(name, os.MFD_CLOEXEC | os.MFD_ALLOW_SEALING)
    os.write(fd, data)
    os.lseek(fd, 0, os.SEEK_SET)
    fcntl.fcntl(
        fd,
        fcntl.F_ADD_SEALS,
        fcntl.F_SEAL_WRITE | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_GROW | fcntl.F_SEAL_SEAL,
    )
    return fd


def read_proc_ppid_and_state(pid):
    if not pid or pid <= 0:
        return {"exists": False, "ppid": None, "state": None, "comm": None}
    try:
        with open(f"/proc/{pid}/stat", "r", encoding="utf-8", errors="replace") as f:
            raw = f.read().strip()
        rparen = raw.rfind(")")
        lparen = raw.find("(")
        comm = raw[lparen + 1 : rparen] if lparen >= 0 and rparen > lparen else None
        rest = raw[rparen + 2 :].split() if rparen >= 0 else []
        state = rest[0] if len(rest) > 0 else None
        ppid = int(rest[1]) if len(rest) > 1 else None
        return {"exists": True, "ppid": ppid, "state": state, "comm": comm}
    except OSError:
        return {"exists": False, "ppid": None, "state": None, "comm": None}


def read_children(pid):
    try:
        with open(f"/proc/{pid}/task/{pid}/children", "r", encoding="utf-8") as f:
            return [int(x) for x in f.read().split() if x.isdigit()]
    except OSError:
        return []


def run_case(use_as_pid_1):
    bwrap_opts = [
        "--unshare-all",
        "--die-with-parent",
        "--new-session",
        "--clearenv",
        "--ro-bind",
        "/usr",
        "/usr",
        "--ro-bind",
        "/lib",
        "/lib",
        "--ro-bind",
        "/lib64",
        "/lib64",
        "--ro-bind",
        "/bin",
        "/bin",
        "--proc",
        "/proc",
        "--dev",
        "/dev",
        "--tmpfs",
        "/tmp",
        "--chdir",
        "/tmp",
    ]
    if use_as_pid_1:
        bwrap_opts.append("--as-pid-1")

    parent, child = socket.socketpair(socket.AF_UNIX, socket.SOCK_STREAM)
    parent.setsockopt(socket.SOL_SOCKET, SO_PASSCRED, 1)
    stage_fd = sealed_memfd("diag-stage", TRAMPOLINE)
    fd = sealed_memfd("diag-bwrap-argv", b"\0".join(x.encode() for x in bwrap_opts) + b"\0")
    # Hold inner target on stdin (FD 0, preserved by TRAMPOLINE keep={0,1,2}) until handoff /proc snapshot.
    cmd = [
        "/usr/bin/python3",
        "-I",
        "-B",
        "-S",
        f"/proc/self/fd/{stage_fd}",
        str(stage_fd),
        str(child.fileno()),
        "-1",
        "/usr/bin/python3",
        "-I",
        "-B",
        "-S",
        "-c",
        "import os; os.read(0, 1)",
    ]
    sel = selectors.DefaultSelector()
    received_fds = []
    listener = None
    proc = None
    try:
        proc = subprocess.Popen(
            ["/usr/bin/bwrap", "--args", str(fd), "--", *cmd],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env={},
            pass_fds=(fd, stage_fd, child.fileno()),
            preexec_fn=parent.close,
            close_fds=True,
        )
        child.close()
        parent.settimeout(1.5)
        for name, stream in (("stdout", proc.stdout), ("stderr", proc.stderr)):
            os.set_blocking(stream.fileno(), False)
            sel.register(stream, selectors.EVENT_READ, name)

        cmsg_space = socket.CMSG_SPACE(array.array("i").itemsize) + socket.CMSG_SPACE(struct.calcsize("iii"))
        msg, anc, flags, _ = parent.recvmsg(1, cmsg_space)
        ucred_pid = None
        for level, kind, data in anc:
            if level == socket.SOL_SOCKET and kind == socket.SCM_RIGHTS:
                vals = array.array("i")
                vals.frombytes(data[: len(data) - (len(data) % vals.itemsize)])
                received_fds.extend(vals)
            elif level == socket.SOL_SOCKET and kind == SCM_CREDENTIALS:
                ucred_pid, _, _ = struct.unpack("iii", data[: struct.calcsize("iii")])
        if msg != b"L" or (flags & socket.MSG_CTRUNC) or len(received_fds) != 1 or not ucred_pid:
            raise RuntimeError(f"invalid handoff: msg={msg!r} flags={flags} fds={len(received_fds)} ucred={ucred_pid}")
        listener = received_fds[0]
        p1_children_at_handoff = read_children(proc.pid)
        ucred_info_at_handoff = read_proc_ppid_and_state(ucred_pid)
        p2_pid = (
            p1_children_at_handoff[0]
            if p1_children_at_handoff
            else (ucred_info_at_handoff["ppid"] if not use_as_pid_1 else ucred_pid)
        )
        proc.stdin.close()
        proc.stdin = None

        listener_events = []
        listener_active = True
        sel.register(listener, selectors.EVENT_READ, "network")
        while len(sel.get_map()) > (1 if listener_active else 0) or proc.poll() is None:
            for key, _ in sel.select(0.02):
                if key.data == "network":
                    ev_info = inspect_listener(listener)
                    if len(listener_events) < 4:
                        listener_events.append(ev_info)
                    if ev_info["pollin"] or "notif" in ev_info:
                        raise RuntimeError(f"seccomp notification: {json.dumps(ev_info, sort_keys=True)}")
                    sel.unregister(listener)
                    listener_active = False
                    continue
                chunk = os.read(key.fileobj.fileno(), 65536)
                if not chunk:
                    sel.unregister(key.fileobj)
        p1_rc = proc.wait(timeout=1.5)
        t_after_p1_wait = time.monotonic()

        # Immediately inspect P2 and P3 state at t=0 after proc.wait() ( exactly where L711 runs )
        p2_at_l711 = read_proc_ppid_and_state(p2_pid)
        ucred_at_l711 = read_proc_ppid_and_state(ucred_pid)

        # Because PR_SET_CHILD_SUBREAPER is enabled on this diagnostic process, any child of P1 (like P2)
        # that P1 failed to waitpid() on before exiting is reparented to os.getpid()!
        reparented_reaped = []
        deadline = time.monotonic() + 1.0
        while time.monotonic() < deadline:
            try:
                wpid, wstatus = os.waitpid(-1, os.WNOHANG)
                if wpid == 0:
                    time.sleep(0.0005)
                    continue
                reparented_reaped.append(
                    {
                        "reaped_pid": wpid,
                        "wstatus": wstatus,
                        "elapsed_us_after_p1_wait": round((time.monotonic() - t_after_p1_wait) * 1e6, 1),
                    }
                )
            except ChildProcessError:
                break

        p2_after_subreap = read_proc_ppid_and_state(p2_pid)
        post_wait_listener = inspect_listener(listener)
        if post_wait_listener["pollin"] or "notif" in post_wait_listener:
            raise RuntimeError(f"late seccomp notification: {json.dumps(post_wait_listener, sort_keys=True)}")
        return {
            "as_pid_1": use_as_pid_1,
            "p1_outer_bwrap_pid": proc.pid,
            "p1_rc": p1_rc,
            "p1_children_at_handoff": p1_children_at_handoff,
            "ucred_info_at_handoff": ucred_info_at_handoff,
            "p2_inner_pid1_host_pid": p2_pid,
            "ucred_trampoline_host_pid": ucred_pid,
            "distinct_p2_and_p3": bool(p2_pid and ucred_pid and p2_pid != ucred_pid),
            "listener_events": listener_events,
            "post_wait_listener": post_wait_listener,
            "p2_state_immediately_after_proc_wait_at_L711": p2_at_l711,
            "ucred_state_immediately_after_proc_wait_at_L711": ucred_at_l711,
            "reparented_orphans_reaped_by_subreaper": reparented_reaped,
            "p2_state_after_subreaper_wait": p2_after_subreap,
        }
    finally:
        if proc is not None and proc.stdin is not None:
            try:
                proc.stdin.close()
            except OSError:
                pass
        if proc is not None and proc.poll() is None:
            try:
                proc.kill()
            except OSError:
                pass
            try:
                proc.wait(timeout=0.5)
            except Exception:
                pass
        while True:
            try:
                wpid, _ = os.waitpid(-1, os.WNOHANG)
                if wpid == 0:
                    break
            except ChildProcessError:
                break
        for rfd in received_fds:
            try:
                os.close(rfd)
            except OSError:
                pass
        for s in (parent, child):
            try:
                s.close()
            except OSError:
                pass
        for f in (fd, stage_fd):
            try:
                os.close(f)
            except OSError:
                pass
        sel.close()
        if proc is not None and proc.stdout:
            proc.stdout.close()
        if proc is not None and proc.stderr:
            proc.stderr.close()


def main():
    apply_bounds()
    tramp_sha = hashlib.sha256(TRAMPOLINE).hexdigest()
    if tramp_sha != EXPECTED_TRAMPOLINE_SHA256:
        raise RuntimeError(f"trampoline sha mismatch: {tramp_sha}")
    libc = ctypes.CDLL(None, use_errno=True)
    if libc.prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0) != 0:
        raise RuntimeError("PR_SET_CHILD_SUBREAPER failed")
    results = {}
    failed = False
    for key_name, flag in (("step_A_default_bwrap_no_as_pid_1", False), ("step_B_bwrap_with_as_pid_1", True)):
        try:
            results[key_name] = run_case(use_as_pid_1=flag)
        except Exception as exc:
            failed = True
            results[key_name] = {"status": "ERROR", "error": str(exc)}
    out = {
        "status": "ERROR" if failed else "OK",
        "diagnostic_pid": os.getpid(),
        "trampoline_sha256": tramp_sha,
        **results,
    }
    sys.stdout.write(json.dumps(out, sort_keys=True) + "\n")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
