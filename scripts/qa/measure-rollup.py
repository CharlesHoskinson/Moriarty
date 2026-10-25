#!/usr/bin/env python3
"""Measure one stock proof stage without including Cargo/build resource usage."""
import argparse
import json
import os
from pathlib import Path
import resource
import subprocess
import time


def sample(pid):
    try:
        rows = Path(f'/proc/{pid}/status').read_text().splitlines()
    except (FileNotFoundError, ProcessLookupError):
        return 0, 0
    fields = {row.split(':', 1)[0]: row.split(':', 1)[1].strip() for row in rows if ':' in row}
    return tuple(int(fields.get(key, '0 kB').split()[0]) * 1024 for key in ('VmHWM', 'VmSwap'))


def run(command, output):
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    started = time.monotonic()
    child = subprocess.Popen(command)
    peak_rss = peak_swap = samples = 0
    while True:
        rss, swap = sample(child.pid)
        samples += 1
        peak_rss, peak_swap = max(peak_rss, rss), max(peak_swap, swap)
        code = child.poll()
        if code is not None:
            break
        time.sleep(0.02)
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    result = {'schema': 'sherlock-rollup-measurement/1', 'command': command, 'exit': code,
              'allocator_environment': {key: os.environ.get(key) for key in
                  ('MALLOC_MMAP_THRESHOLD_', 'MALLOC_TRIM_THRESHOLD_', 'MALLOC_ARENA_MAX', 'GLIBC_TUNABLES')},
              'wall_seconds': time.monotonic() - started,
              'cpu_user_seconds': usage.ru_utime - before.ru_utime,
              'cpu_system_seconds': usage.ru_stime - before.ru_stime,
              'max_rss_bytes': max(peak_rss, int(usage.ru_maxrss) * 1024),
              'swap_peak_bytes': peak_swap, 'swap_samples': samples, 'swap_sample_period_seconds': 0.02,
              'scope': 'one child process including all threads; build excluded',
              'swap_scope': 'maximum sampled VmSwap; includes current swap and excludes unrelated host processes'}
    Path(output).write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    if not command:
        parser.error('command is required')
    return run(command, args.output)['exit']


if __name__ == '__main__':
    raise SystemExit(main())
