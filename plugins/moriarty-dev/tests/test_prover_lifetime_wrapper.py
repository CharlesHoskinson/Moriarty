"""Process-driven tests for the static prover lifetime PID 1 wrapper."""

import os
from pathlib import Path
import shutil
import signal
import subprocess
import textwrap
import time

import pytest


PLUGIN_ROOT = Path(__file__).resolve().parents[1]
SOURCE = PLUGIN_ROOT / "scripts" / "moriarty_dev" / "prover_lifetime.c"
SCHEMA = "moriarty.prover-lifetime/1"
DIGEST = "ab" * 32

JS_CODEC = r"""/** Pure prover lifetime arithmetic and closed control-record bytes. No clock,
 * process, wallet, network, Docker or store is consulted by this module.
 */

export const PROVER_LATEST_START_SECONDS=120;
export const PROVER_KILL_DEADLINE_SECONDS=1620;
export const PROVER_MAX_RUNTIME_SECONDS=1500;
export const PROVER_CONTROL_SCHEMA='moriarty.prover-lifetime/1';
export const PROVER_CONTROL_MAX_BYTES=512;

const NS_PER_SECOND=1000000000n;
const check=(ok,code)=>{if(!ok)throw Error(code);};
const plain=o=>o&&Object.getPrototypeOf(o)===Object.prototype;
function exact(o,keys,code='PROVER_LIFETIME_CONTROL_FIELDS'){
 check(plain(o),code);const descriptors=Object.getOwnPropertyDescriptors(o);
 check(Reflect.ownKeys(descriptors).length===Object.keys(descriptors).length&&Object.values(descriptors).every(d=>Object.hasOwn(d,'value')&&d.enumerable)&&Object.keys(descriptors).sort().join('|')===keys.split(',').sort().join('|'),code);
}
const nonnegativeBigInt=value=>typeof value==='bigint'&&value>=0n;
const canonicalDecimal=value=>/^(0|[1-9][0-9]*)$/.test(value);
const uuid=value=>typeof value==='string'&&/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/.test(value);
const digest=value=>typeof value==='string'&&/^[0-9a-f]{64}$/.test(value);

export function computeProverLifetimeBounds(outerStartMonotonicNs){
 check(nonnegativeBigInt(outerStartMonotonicNs),'PROVER_LIFETIME_OUTER_START');
 return {
  latestStartMonotonicNs:outerStartMonotonicNs+BigInt(PROVER_LATEST_START_SECONDS)*NS_PER_SECOND,
  killDeadlineMonotonicNs:outerStartMonotonicNs+BigInt(PROVER_KILL_DEADLINE_SECONDS)*NS_PER_SECOND
 };
}

export function validateWrapperEntry(entryMonotonicNs,latestStartMonotonicNs){
 check(nonnegativeBigInt(entryMonotonicNs),'PROVER_LIFETIME_ENTRY');
 check(nonnegativeBigInt(latestStartMonotonicNs),'PROVER_LIFETIME_LATEST_START');
 check(entryMonotonicNs<latestStartMonotonicNs,'PROVER_LIFETIME_LATE_ENTRY');return true;
}

export function computeWrapperKillDeadline(entryMonotonicNs,controlKillDeadlineNs){
 check(nonnegativeBigInt(entryMonotonicNs),'PROVER_LIFETIME_ENTRY');
 check(nonnegativeBigInt(controlKillDeadlineNs),'PROVER_LIFETIME_CONTROL_DEADLINE');
 const own=entryMonotonicNs+BigInt(PROVER_MAX_RUNTIME_SECONDS)*NS_PER_SECOND;
 return own<controlKillDeadlineNs?own:controlKillDeadlineNs;
}

function inode(value){
 check(typeof value==='bigint'?value>=0n:Number.isSafeInteger(value)&&value>=0,'PROVER_LIFETIME_CONTROL_TIME_NAMESPACE');
 return BigInt(value);
}

export function encodeProverControlRecord(value){
 exact(value,'bootId,timeNamespaceInode,latestStartMonotonicNs,killDeadlineMonotonicNs,invocationDigest');
 check(uuid(value.bootId),'PROVER_LIFETIME_CONTROL_BOOT_ID');const timeNamespaceInode=inode(value.timeNamespaceInode);
 check(nonnegativeBigInt(value.latestStartMonotonicNs),'PROVER_LIFETIME_CONTROL_LATEST_START');
 check(nonnegativeBigInt(value.killDeadlineMonotonicNs),'PROVER_LIFETIME_CONTROL_KILL_DEADLINE');
 check(digest(value.invocationDigest),'PROVER_LIFETIME_CONTROL_INVOCATION_DIGEST');
 const encoded=Buffer.from(`${PROVER_CONTROL_SCHEMA}\n${value.bootId}\n${timeNamespaceInode}\n${value.latestStartMonotonicNs}\n${value.killDeadlineMonotonicNs}\n${value.invocationDigest}\n`,'ascii');
 check(encoded.length<=PROVER_CONTROL_MAX_BYTES,'PROVER_LIFETIME_CONTROL_SIZE');return encoded;
}

export function decodeProverControlRecord(buffer){
 check(Buffer.isBuffer(buffer),'PROVER_LIFETIME_CONTROL_BUFFER');
 check(buffer.length<=PROVER_CONTROL_MAX_BYTES,'PROVER_LIFETIME_CONTROL_SIZE');
 check(!buffer.includes(0x0d),'PROVER_LIFETIME_CONTROL_CR');
 check([...buffer].every(byte=>byte<=0x7f),'PROVER_LIFETIME_CONTROL_ASCII');
 const lines=buffer.toString('ascii').split('\n');check(lines.length===7&&lines[6]==='','PROVER_LIFETIME_CONTROL_LINES');
 const [schema,bootId,timeNamespace,latestStart,killDeadline,invocationDigest]=lines;
 check(schema===PROVER_CONTROL_SCHEMA,'PROVER_LIFETIME_CONTROL_SCHEMA');
 check(uuid(bootId),'PROVER_LIFETIME_CONTROL_BOOT_ID');
 check(canonicalDecimal(timeNamespace),'PROVER_LIFETIME_CONTROL_TIME_NAMESPACE');
 check(canonicalDecimal(latestStart),'PROVER_LIFETIME_CONTROL_LATEST_START');
 check(canonicalDecimal(killDeadline),'PROVER_LIFETIME_CONTROL_KILL_DEADLINE');
 check(digest(invocationDigest),'PROVER_LIFETIME_CONTROL_INVOCATION_DIGEST');
 return {bootId,timeNamespaceInode:BigInt(timeNamespace),latestStartMonotonicNs:BigInt(latestStart),killDeadlineMonotonicNs:BigInt(killDeadline),invocationDigest};
}

const identity=value=>typeof value==='bigint'||typeof value==='number'?String(value):value;
export function validateClockIdentity(value){
 exact(value,'parentBootId,parentTimeNamespaceInode,childBootId,childTimeNamespaceInode','PROVER_LIFETIME_CLOCK_IDENTITY');
 check(value.parentBootId===value.childBootId&&identity(value.parentTimeNamespaceInode)===identity(value.childTimeNamespaceInode),'PROVER_LIFETIME_CLOCK_IDENTITY');return true;
}
"""

CHILD_SOURCE = r"""
#define _GNU_SOURCE
#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>

static void write_pid(const char *path) {
    FILE *f = fopen(path, "w");
    if (f == NULL) _exit(90);
    fprintf(f, "%ld\n", (long)getpid());
    if (fclose(f) != 0) _exit(91);
}
static void append_heartbeat(const char *path) {
    FILE *f = fopen(path, "a");
    if (f == NULL) _exit(92);
    fputc('x', f);
    if (fclose(f) != 0) _exit(93);
}
static void hang(void) { for (;;) pause(); }
static void exit_zero(int number) { (void)number; _exit(0); }
int main(int argc, char **argv) {
    if (argc < 2) return 89;
    if (strcmp(argv[1], "exit") == 0) return atoi(argv[2]);
    if (strcmp(argv[1], "mark") == 0) { write_pid(argv[2]); return 0; }
    if (strcmp(argv[1], "hang") == 0) { write_pid(argv[2]); hang(); }
    if (strcmp(argv[1], "ignore") == 0) {
        signal(SIGTERM, SIG_IGN); signal(SIGINT, SIG_IGN);
        write_pid(argv[2]); hang();
    }
    if (strcmp(argv[1], "handle") == 0) {
        signal(SIGTERM, exit_zero);
        write_pid(argv[2]); hang();
    }
    if (strcmp(argv[1], "fork") == 0) {
        int ready[2];
        char byte;
        if (pipe(ready) != 0) return 84;
        pid_t pid = fork();
        if (pid < 0) return 88;
        if (pid == 0) {
            close(ready[0]); write_pid(argv[2]);
            if (write(ready[1], "x", 1) != 1) _exit(83);
            close(ready[1]); hang();
        }
        close(ready[1]);
        while (read(ready[0], &byte, 1) < 0 && errno == EINTR) {}
        close(ready[0]);
        return 0;
    }
    if (strcmp(argv[1], "detach-heartbeat") == 0) {
        int ready[2];
        char byte;
        if (pipe(ready) != 0) return 82;
        pid_t pid = fork();
        if (pid < 0) return 87;
        if (pid > 0) {
            close(ready[1]);
            while (read(ready[0], &byte, 1) < 0 && errno == EINTR) {}
            close(ready[0]);
            return 0;
        }
        close(ready[0]);
        if (setpgid(0, 0) != 0) _exit(86);
        write_pid(argv[2]);
        append_heartbeat(argv[3]);
        if (write(ready[1], "x", 1) != 1) _exit(81);
        close(ready[1]);
        for (;;) {
            struct timespec delay = {0, 75000000L};
            append_heartbeat(argv[3]);
            while (nanosleep(&delay, &delay) != 0 && errno == EINTR) {}
        }
    }
    return 85;
}
"""


def _compile(compiler, source, output):
    return subprocess.run(
        [compiler, "-std=c11", "-Wall", "-Wextra", "-Werror", "-O2", "-static", "-o", str(output), str(source)],
        capture_output=True,
        text=True,
    )


@pytest.fixture(scope="module")
def binaries(tmp_path_factory):
    compiler = shutil.which("gcc") or shutil.which("cc")
    if compiler is None:
        pytest.skip("static prover wrapper tests require gcc or cc")
    root = tmp_path_factory.mktemp("prover-lifetime-build")
    wrapper = root / "prover_lifetime"
    built = _compile(compiler, SOURCE, wrapper)
    assert built.returncode == 0, built.stdout + built.stderr
    child_source = root / "child.c"
    child_source.write_text(CHILD_SOURCE)
    child = root / "child"
    child_built = _compile(compiler, child_source, child)
    assert child_built.returncode == 0, child_built.stdout + child_built.stderr
    return wrapper, child


@pytest.fixture(scope="module")
def pid_namespace_available():
    unshare = shutil.which("unshare")
    if unshare is None:
        return False, "unshare is unavailable"
    probe = subprocess.run(
        [unshare, "--user", "--map-root-user", "--pid", "--fork", "--mount-proc", "--", "true"],
        capture_output=True,
        text=True,
    )
    detail = (probe.stderr or probe.stdout).strip()
    return probe.returncode == 0, detail or "combined user/PID namespace probe failed"


def identity():
    return (
        Path("/proc/sys/kernel/random/boot_id").read_text().strip(),
        os.stat("/proc/self/ns/time").st_ino,
    )


def encode_control(*, boot=None, inode=None, latest=None, deadline=None, digest=DIGEST):
    real_boot, real_inode = identity()
    if latest is None:
        latest = time.monotonic_ns() + 5_000_000_000
    if deadline is None:
        deadline = time.monotonic_ns() + 2_000_000_000
    return f"{SCHEMA}\n{boot or real_boot}\n{real_inode if inode is None else inode}\n{latest}\n{deadline}\n{digest}\n".encode("ascii")


def control_file(tmp_path, **kwargs):
    path = tmp_path / "control"
    path.write_bytes(encode_control(**kwargs))
    return path


def run_wrapper(wrapper, control, child, *args, timeout=6):
    return subprocess.run(
        [str(wrapper), str(control), "--", str(child), *map(str, args)],
        capture_output=True,
        text=True,
        timeout=timeout,
    )


def wait_for_file(path, proc, timeout=3):
    end = time.monotonic() + timeout
    while time.monotonic() < end and proc.poll() is None:
        if path.exists():
            return
        time.sleep(0.01)
    assert path.exists(), f"{path} was not created; stderr={proc.stderr.read()}"


def assert_pid_gone(pid, timeout=2):
    end = time.monotonic() + timeout
    while time.monotonic() < end and Path(f"/proc/{pid}").exists():
        time.sleep(0.01)
    assert not Path(f"/proc/{pid}").exists(), f"pid {pid} remains"


def process_children(pid):
    path = Path(f"/proc/{pid}/task/{pid}/children")
    return [int(value) for value in path.read_text().split()]


def test_build_is_static_and_import_free(binaries):
    wrapper, _ = binaries
    file_result = subprocess.run(["file", str(wrapper)], capture_output=True, text=True, check=True)
    assert "statically linked" in file_result.stdout
    ldd_result = subprocess.run(["ldd", str(wrapper)], capture_output=True, text=True)
    assert "not a dynamic executable" in (ldd_result.stdout + ldd_result.stderr)


def test_normal_exit_and_exec_failure_preserve_child_status(binaries, tmp_path):
    wrapper, child = binaries
    control = control_file(tmp_path)
    started = time.monotonic()
    assert run_wrapper(wrapper, control, child, "exit", "23").returncode == 23
    assert time.monotonic() - started < 1.0
    missing = tmp_path / "missing-executable"
    result = run_wrapper(wrapper, control, missing)
    assert result.returncode == 126


def test_control_deadline_kills_hanging_child_and_is_independent(binaries, tmp_path):
    wrapper, child = binaries
    pidfile = tmp_path / "hang.pid"
    control = control_file(tmp_path, deadline=time.monotonic_ns() + 450_000_000)
    started = time.monotonic()
    proc = subprocess.Popen(
        [str(wrapper), str(control), "--", str(child), "hang", str(pidfile)],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    wait_for_file(pidfile, proc)
    time.sleep(0.25)  # no signals and no polling: the external supervisor is absent
    stdout, stderr = proc.communicate(timeout=3)
    assert proc.returncode == 124, stdout + stderr
    assert 0.25 <= time.monotonic() - started < 2.0
    assert_pid_gone(int(pidfile.read_text()))


def test_dead_external_launcher_cannot_disable_wrapper_deadline(binaries, tmp_path):
    wrapper, child = binaries
    pidfile = tmp_path / "orphan-child.pid"
    wrapper_pid_file = tmp_path / "orphan-wrapper.pid"
    control = control_file(tmp_path, deadline=time.monotonic_ns() + 650_000_000)
    launcher = os.fork()
    if launcher == 0:
        proc = subprocess.Popen(
            [str(wrapper), str(control), "--", str(child), "hang", str(pidfile)],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        wrapper_pid_file.write_text(str(proc.pid))
        os._exit(0)
    _, launcher_status = os.waitpid(launcher, 0)
    assert os.waitstatus_to_exitcode(launcher_status) == 0
    end = time.monotonic() + 2
    while not pidfile.exists() and time.monotonic() < end:
        time.sleep(0.01)
    assert pidfile.exists()
    wrapper_pid = int(wrapper_pid_file.read_text())
    child_pid = int(pidfile.read_text())
    assert Path(f"/proc/{wrapper_pid}").exists()
    assert_pid_gone(child_pid, timeout=3)
    assert_pid_gone(wrapper_pid, timeout=3)


def test_forked_grandchild_is_killed_and_reaped(binaries, tmp_path):
    wrapper, child = binaries
    pidfile = tmp_path / "grandchild.pid"
    result = run_wrapper(wrapper, control_file(tmp_path), child, "fork", pidfile)
    assert result.returncode == 0, result.stderr
    assert pidfile.exists()
    assert_pid_gone(int(pidfile.read_text()))


def test_forwarded_signal_escalates_for_ignoring_child(binaries, tmp_path):
    wrapper, child = binaries
    pidfile = tmp_path / "ignore.pid"
    control = control_file(tmp_path, deadline=time.monotonic_ns() + 5_000_000_000)
    proc = subprocess.Popen(
        [str(wrapper), str(control), "--", str(child), "ignore", str(pidfile)],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    wait_for_file(pidfile, proc)
    started = time.monotonic()
    proc.send_signal(signal.SIGTERM)
    stdout, stderr = proc.communicate(timeout=5)
    assert proc.returncode == 128 + signal.SIGKILL, stdout + stderr
    assert 1.5 <= time.monotonic() - started < 4.0
    assert_pid_gone(int(pidfile.read_text()))


def test_forwarded_signal_is_non_success_when_child_handler_exits_zero(binaries, tmp_path):
    wrapper, child = binaries
    pidfile = tmp_path / "handle.pid"
    control = control_file(tmp_path, deadline=time.monotonic_ns() + 5_000_000_000)
    proc = subprocess.Popen(
        [str(wrapper), str(control), "--", str(child), "handle", str(pidfile)],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    wait_for_file(pidfile, proc)
    proc.send_signal(signal.SIGTERM)
    stdout, stderr = proc.communicate(timeout=3)
    assert proc.returncode == 128 + signal.SIGTERM, stdout + stderr


def test_pid_namespace_tears_down_detached_process_group(
    binaries, tmp_path, pid_namespace_available
):
    wrapper, child = binaries
    pidfile = tmp_path / "detached.pid"
    heartbeat = tmp_path / "heartbeat"
    control = control_file(tmp_path)
    available, reason = pid_namespace_available
    if not available:
        # Still drive the real process-group path before recording the narrower boundary.
        result = run_wrapper(wrapper, control, child, "fork", pidfile)
        assert result.returncode == 0, result.stderr
        assert_pid_gone(int(pidfile.read_text()))
        pytest.skip(f"PID-namespace teardown assertion unavailable: {reason}")
    unshare = shutil.which("unshare")
    result = subprocess.run(
        [
            unshare,
            "--user",
            "--map-root-user",
            "--pid",
            "--fork",
            "--mount-proc",
            "--",
            str(wrapper),
            str(control),
            "--",
            str(child),
            "detach-heartbeat",
            str(pidfile),
            str(heartbeat),
        ],
        capture_output=True,
        text=True,
        timeout=5,
    )
    assert result.returncode == 0, result.stdout + result.stderr
    before = heartbeat.read_bytes()
    time.sleep(0.25)
    assert heartbeat.read_bytes() == before


def test_abrupt_pid_one_death_tears_down_namespace_descendants(
    binaries, tmp_path, pid_namespace_available
):
    available, reason = pid_namespace_available
    if not available:
        pytest.skip(f"abrupt PID 1 namespace teardown unavailable: {reason}")
    wrapper, child = binaries
    pidfile = tmp_path / "killed-pid-one-child.pid"
    heartbeat = tmp_path / "killed-pid-one-heartbeat"
    control = control_file(tmp_path, deadline=time.monotonic_ns() + 5_000_000_000)
    proc = subprocess.Popen(
        [
            shutil.which("unshare"),
            "--user",
            "--map-root-user",
            "--pid",
            "--fork",
            "--mount-proc",
            "--",
            str(wrapper),
            str(control),
            "--",
            str(child),
            "detach-heartbeat",
            str(pidfile),
            str(heartbeat),
        ],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    try:
        wait_for_file(heartbeat, proc)
        end = time.monotonic() + 2
        children = []
        while not children and time.monotonic() < end:
            children = process_children(proc.pid)
            if not children:
                time.sleep(0.01)
        assert len(children) == 1
        os.kill(children[0], signal.SIGKILL)
        stdout, stderr = proc.communicate(timeout=3)
        assert proc.returncode != 0, stdout + stderr
        before = heartbeat.read_bytes()
        time.sleep(0.25)
        assert heartbeat.read_bytes() == before
    finally:
        if proc.poll() is None:
            proc.kill()
            proc.communicate(timeout=3)


@pytest.mark.parametrize(
    "record",
    [
        b"not-the-schema\n",
        encode_control().replace(b"\n", b"\r\n", 1),
        encode_control()[:-1],
        encode_control() + b"\n",
        encode_control().replace(DIGEST.encode(), b"AB" * 32),
        encode_control().replace(b"\n" + str(identity()[1]).encode() + b"\n", b"\n01\n"),
        encode_control().replace(b"\n" + str(identity()[1]).encode() + b"\n", b"\n18446744073709551616\n"),
        encode_control()[:-1] + b"\x80\n",
    ],
)
def test_malformed_controls_refuse_without_child(binaries, tmp_path, record):
    wrapper, child = binaries
    control = tmp_path / "control"
    control.write_bytes(record)
    marker = tmp_path / "marker"
    result = run_wrapper(wrapper, control, child, "mark", marker)
    assert result.returncode == 64
    assert "refuse:" in result.stderr
    assert not marker.exists()


def test_oversize_nonregular_and_symlink_controls_refuse_without_child(binaries, tmp_path):
    wrapper, child = binaries
    marker = tmp_path / "marker"
    cases = []
    oversize = tmp_path / "oversize"
    oversize.write_bytes(b"x" * 513)
    cases.append((oversize, "oversize"))
    directory = tmp_path / "directory"
    directory.mkdir()
    cases.append((directory, "nonregular"))
    target = control_file(tmp_path)
    symlink = tmp_path / "symlink"
    symlink.symlink_to(target)
    cases.append((symlink, "open"))
    for path, reason in cases:
        result = run_wrapper(wrapper, path, child, "mark", marker)
        assert result.returncode == 64
        assert reason in result.stderr
        assert not marker.exists()


def test_fifo_control_refuses_without_blocking_or_child(binaries, tmp_path):
    wrapper, child = binaries
    fifo = tmp_path / "control-fifo"
    marker = tmp_path / "marker"
    os.mkfifo(fifo)
    result = run_wrapper(wrapper, fifo, child, "mark", marker, timeout=2)
    assert result.returncode == 64
    assert "nonregular" in result.stderr
    assert not marker.exists()


def test_clock_identity_mismatches_refuse_without_child(binaries, tmp_path):
    wrapper, child = binaries
    marker = tmp_path / "marker"
    wrong_boot = "00000000-0000-0000-0000-000000000000"
    if wrong_boot == identity()[0]:
        wrong_boot = "11111111-1111-1111-1111-111111111111"
    for name, kwargs in (
        ("boot", {"boot": wrong_boot}),
        ("time namespace", {"inode": identity()[1] + 1}),
    ):
        control = tmp_path / name.replace(" ", "-")
        control.write_bytes(encode_control(**kwargs))
        result = run_wrapper(wrapper, control, child, "mark", marker)
        assert result.returncode == 64
        assert name in result.stderr
        assert not marker.exists()


def test_late_entry_refuses_before_child_execution(binaries, tmp_path):
    wrapper, child = binaries
    marker = tmp_path / "marker"
    control = control_file(
        tmp_path,
        latest=time.monotonic_ns() - 1,
        deadline=time.monotonic_ns() + 2_000_000_000,
    )
    result = run_wrapper(wrapper, control, child, "mark", marker)
    assert result.returncode == 64
    assert "late entry" in result.stderr
    assert not marker.exists()


def test_expired_deadline_refuses_before_child_execution(binaries, tmp_path):
    wrapper, child = binaries
    marker = tmp_path / "marker"
    control = control_file(
        tmp_path,
        latest=time.monotonic_ns() + 2_000_000_000,
        deadline=time.monotonic_ns() - 1,
    )
    result = run_wrapper(wrapper, control, child, "mark", marker)
    assert result.returncode == 64
    assert "expired deadline" in result.stderr
    assert not marker.exists()


def test_cli_usage_errors_exit_two(binaries):
    wrapper, _ = binaries
    for args in ([], ["control"], ["control", "not-separator", "/bin/true"]):
        result = subprocess.run([str(wrapper), *args], capture_output=True, text=True)
        assert result.returncode == 2
        assert result.stderr.startswith("usage:")


def test_c_codec_bytes_and_decode_agree_with_verbatim_js(binaries, tmp_path):
    wrapper, _ = binaries
    node = shutil.which("node")
    if node is None:
        pytest.skip("byte-agreement test requires Node.js")
    codec = tmp_path / "prover-lifetime.mjs"
    codec.write_text(JS_CODEC)
    driver = tmp_path / "encode.mjs"
    driver.write_text(
        textwrap.dedent(
            f"""
            import {{encodeProverControlRecord}} from {codec.as_uri()!r};
            process.stdout.write(encodeProverControlRecord({{
              bootId:'12345678-9abc-def0-1234-56789abcdef0',
              timeNamespaceInode:42n,
              latestStartMonotonicNs:1234567890123456789n,
              killDeadlineMonotonicNs:1234569390123456789n,
              invocationDigest:'{DIGEST}'
            }}));
            """
        )
    )
    encoded_by_js = subprocess.run([node, str(driver)], capture_output=True, check=True).stdout
    encoded_by_c = subprocess.run(
        [
            str(wrapper),
            "--selftest-encode",
            "12345678-9abc-def0-1234-56789abcdef0",
            "42",
            "1234567890123456789",
            "1234569390123456789",
            DIGEST,
        ],
        capture_output=True,
        check=True,
    ).stdout
    assert encoded_by_c == encoded_by_js
    decoded = subprocess.run(
        [str(wrapper), "--selftest-decode"],
        input=encoded_by_js,
        capture_output=True,
        check=True,
    ).stdout.decode("ascii").splitlines()
    assert decoded == [
        "12345678-9abc-def0-1234-56789abcdef0",
        "42",
        "1234567890123456789",
        "1234569390123456789",
        DIGEST,
    ]
