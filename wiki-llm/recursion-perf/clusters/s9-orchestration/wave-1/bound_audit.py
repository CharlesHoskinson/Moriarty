"""ESTIMATE only: necessary capacity bounds, not a scheduler/prover benchmark.

Run with python3 -B; prints Markdown and does not modify source artifacts.
CPU, GPU and control shares below are inherited hypothetical inputs, not data.
"""

import math

LEAVES = (34.779426369, 35.295744765, 44.185937479, 49.369468101)
NODE = 867.279750364
DECIDER = 586.381689547
T = sum(LEAVES) + NODE + DECIDER


def interval_bound(cpu_core_seconds, cores, gpu_seconds, control_seconds,
                   critical_path, window, finality=0.0, transfer_seconds=0.0):
    """Serial GPU/control servers; transfers get a separate idealized server.

    A batch holds its window slot through finality. No measured contention,
    queue overhead, per-phase byte peaks, or memory feasibility is inferred.
    """
    return max(cpu_core_seconds / cores, gpu_seconds, control_seconds,
               transfer_seconds, (critical_path + finality) / window)


def main():
    cpu = 2 * .25 * T
    gpu = .75 * T / 10
    control = .01 * T
    critical = (.25 + .75 / 10 + .01) * (max(LEAVES) + NODE + DECIDER)
    print('# Resource-bound arithmetic (ESTIMATE, not new measurements)\n')
    print('SOURCE inputs: raw/s9-orchestration/prior-astra3.md. '
          'All phase shares, worker efficiency, finality and windows are assumptions.\n')
    print(f'ESTIMATE: T={T:.6f} s; C={cpu:.6f} core-s; G={gpu:.6f} s; '
          f'Q={control:.6f} s; best-path L={critical:.6f} s. '
          '16 productive CPU cores, four reserved; one serialized GPU server.\n')
    print('| Legal batch window W | Finality F (s) | Interval lower bound (s) '
          '| Speedup upper bound vs historical T | Upper bound vs CPU baseline T/2 |')
    print('|---:|---:|---:|---:|---:|')
    for finality in (0, 300):
        for window in (1, 2, 3, 4, 5, 8):
            bound = interval_bound(cpu, 16, gpu, control, critical, window, finality)
            print(f'| {window} | {finality} | {bound:.3f} | {T/bound:.3f}x '
                  f'| {T/2/bound:.3f}x |')
    target = T / 10
    print(f'\nESTIMATE: historical 10x target interval={target:.6f} s; '
          f'CPU budget at 16 cores={16*target:.3f} core-s/batch; '
          f'window minimum at F=0: ceil(L/target)={math.ceil(critical/target)}; '
          f'at F=300: {math.ceil((critical+300)/target)}. These are necessary, not sufficient.\n')
    print(f'ESTIMATE: four infinitely provisioned leaf workers with unchanged proofs: '
          f'{T:.6f}/({max(LEAVES):.6f}+{NODE:.6f}+{DECIDER:.6f})='
          f'{T/(max(LEAVES)+NODE+DECIDER):.6f}x latency ceiling. '
          f'Zero-time leaves ceiling={T/(NODE+DECIDER):.6f}x.\n')
    print('ESTIMATE: B3 recorded target endpoints: '
          f'{920.389/10:.4f}--{1287.534/10:.4f} s; '
          f'B5 suite/12={5558.878/12:.6f} s per batch instance; '
          f'B5 10x suite target={5558.878/10:.4f} s. '
          'B5 division is suite amortization, never steady throughput or latency.')


if __name__ == '__main__':
    main()
