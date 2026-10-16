#!/usr/bin/env python3
"""Finite ordinary harness for the repaired Collector/drain/snapshot only.

Usage (proposed, not run by the source author):
  /usr/bin/python3 -I ordinary_selftest.py /tmp/k6-drain-fixture/launcher.py
Imports the candidate launcher under a unique module name. Never calls
main, verify_record_and_auth, any guard, native binary, or protected path.
Uses only ordinary OS pipes, thread stubs, and a fake clock. Stdlib only.
"""
import hashlib
import importlib.util
import os
import sys
import threading
import time

sys.dont_write_bytecode = True


def load_launcher(path):
    name = "launcher_candidate_drain_selftest_unique"
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def check(label, condition):
    if not condition:
        raise AssertionError(label)
    print("ok - " + label)


class FakeClock:
    def __init__(self):
        self.now = 1000.0

    def __call__(self):
        return self.now


class StubCollector:
    def __init__(self, clock, advance, finishes):
        self.clock = clock
        self.advance = advance
        self.finishes = finishes
        self.alive = True
        self.timeouts = []

    def join(self, timeout=None):
        self.timeouts.append(timeout)
        self.clock.now += min(self.advance, timeout)
        if self.finishes and self.clock.now >= 1000.0 + self.advance:
            self.alive = False

    def is_alive(self):
        return self.alive


def wait_observed(collector, want, seconds=1.0):
    end = time.monotonic() + seconds
    snap = collector.snapshot()
    while snap[2] < want and time.monotonic() < end:
        time.sleep(0.005)
        snap = collector.snapshot()
    return snap


def main(argv):
    if len(argv) != 2:
        print("usage: ordinary_selftest.py <candidate launcher.py>", file=sys.stderr)
        return 2
    path = argv[1]
    with open(path, "r", encoding="utf-8") as handle:
        source = handle.read()
    check("old late join(timeout=5) absent", "join(timeout=5)" not in source)
    check("no buffered parent-pipe close in drain path", "proc.stdout.close()" not in source and "proc.stderr.close()" not in source)
    check("exactly one snapshot call per stream in main", source.count(".snapshot()") == 2)
    check("single shared drain helper used", "drain_collectors(collectors, drain_deadline)" in source)
    check("drain bound and stream cap unchanged", "POST_EXIT_DRAIN_SECONDS = 60" in source and "STREAM_CAP = 4194304" in source)
    check("exactly one Popen site preserved", source.count("subprocess.Popen") == 1)
    mod = load_launcher(path)
    check("launcher import exposes Collector/drain/snapshot", hasattr(mod, "Collector") and hasattr(mod, "drain_collectors"))
    # Fake-clock shared-deadline test: first stub consumes the whole budget
    # and stays alive, so the second stub must receive no positive join.
    clock = FakeClock()
    first = StubCollector(clock, 0.30, False)
    second = StubCollector(clock, 0.30, False)
    joined = mod.drain_collectors([first, second], 1000.0 + 0.30, clock=clock)
    total_wait = sum(t for t in first.timeouts + second.timeouts if t)
    check("shared deadline not exceeded", total_wait <= 0.30 + 1e-9)
    check("no post-deadline join for second collector", second.timeouts == [])
    check("unjoined drain reports incomplete", joined is False)
    # Finishing stubs join within the same shared budget.
    clock2 = FakeClock()
    a = StubCollector(clock2, 0.05, True)
    b = StubCollector(clock2, 0.05, True)
    check("joined drain reports complete", mod.drain_collectors([a, b], 1000.0 + 0.30, clock=clock2) is True)
    # Ordinary pipe capture: bounded prefix, full hash, count, overflow.
    old_cap = mod.STREAM_CAP
    mod.STREAM_CAP = 8
    try:
        read_fd, write_fd = os.pipe()
        collector = mod.Collector(read_fd)
        collector.start()
        os.write(write_fd, b"0123456789ABCDEFGHIJ")
        snap = wait_observed(collector, 20)
        check("snapshot coherent while collector active", hashlib.sha256(snap[0]).hexdigest() != snap[1] or snap[2] == len(snap[0]))
        check("retained prefix bounded at patched cap", snap[0] == b"01234567" and len(snap[0]) == 8)
        check("full-stream hash over all observed bytes", snap[1] == hashlib.sha256(b"0123456789ABCDEFGHIJ").hexdigest())
        check("observed count and overflow", snap[2] == 20 and snap[3] is True)
        check("snapshot prefix is immutable bytes", isinstance(snap[0], bytes))
        os.close(write_fd)
        collector.join(timeout=2.0)
        final = collector.snapshot()
        check("EOF marks complete without error", final[4] is True and final[5] is False and not collector.is_alive())
    finally:
        mod.STREAM_CAP = old_cap
    # Exact small-stream capture at the real cap logic.
    read_fd, write_fd = os.pipe()
    collector = mod.Collector(read_fd)
    collector.start()
    os.write(write_fd, b"abc")
    os.close(write_fd)
    collector.join(timeout=2.0)
    snap = collector.snapshot()
    check("small stream exact", snap[0] == b"abc" and snap[1] == hashlib.sha256(b"abc").hexdigest() and snap[2] == 3 and snap[3] is False and snap[4] is True)
    os.close(read_fd)
    # Read error cannot become complete capture or PASS.
    read_fd, write_fd = os.pipe()
    os.close(write_fd)
    os.close(read_fd)
    broken = mod.Collector(read_fd)
    broken.start()
    broken.join(timeout=2.0)
    snap = broken.snapshot()
    drain_complete = (not broken.is_alive()) and snap[4] and not snap[5]
    check("read error recorded, not EOF", snap[5] is True and snap[4] is False)
    check("read error cannot satisfy drain-complete/PASS predicate", drain_complete is False)
    print("PASS ordinary drain/snapshot harness")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
