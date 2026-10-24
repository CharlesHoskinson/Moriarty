"""Planning simulation, NOT a prover benchmark. Standard library only.

Historical Fenrir stage times set the scale; all CPU/GPU shares are assumptions.
Jobs retain a host slot from CPU start through GPU/control completion. Four
leaves precede each parent, which precedes its decider. GPU phases serialize.
No proof data, keys, or network state is used.
"""
import heapq
import json
from pathlib import Path

TIMES = [34.779426369, 35.295744765, 44.185937479, 49.369468101,
         867.279750364, 586.381689547]


def simulate(cpu_workers, host_slots, active_batches, batches=200,
             cpu_fraction=.25, bulk_speed=10, overhead_fraction=.01):
    jobs = {(b, s): "blocked" for b in range(batches) for s in range(6)}
    events = []
    completed = {}
    cpu_busy = gpu_busy = control_busy = slots = 0
    now = 0.
    serial = 0

    def eligible(j, first):
        b, s = j
        if b >= first + active_batches:
            return False
        return s < 4 or all(jobs[b, k] == "done" for k in
                            (range(4) if s == 4 else [4]))

    def schedule(j, phase, duration):
        nonlocal serial
        jobs[j] = phase
        serial += 1
        heapq.heappush(events, (now + duration, serial, j, phase))

    while len(completed) < batches:
        # Oldest batch first; within it prioritize downstream work.
        first = next((i for i in range(batches) if i not in completed), batches)
        order = [(b, s) for b in range(first, min(batches, first+active_batches))
                 for s in range(5, -1, -1)]
        for j in order:
            if jobs[j] == "control_ready" and control_busy == 0:
                control_busy = 1
                schedule(j, "control", overhead_fraction * TIMES[j[1]])
            if jobs[j] == "gpu_ready" and gpu_busy == 0:
                gpu_busy = 1
                schedule(j, "gpu", (1-cpu_fraction)/bulk_speed * TIMES[j[1]])
            if (jobs[j] == "blocked" and eligible(j, first)
                    and cpu_busy < cpu_workers and slots < host_slots):
                cpu_busy += 1
                slots += 1
                schedule(j, "cpu", cpu_fraction * TIMES[j[1]])
        if not events:
            raise RuntimeError("scheduler deadlock")
        now = events[0][0]
        while events and events[0][0] == now:
            _, _, j, phase = heapq.heappop(events)
            if phase == "cpu":
                cpu_busy -= 1
                jobs[j] = "gpu_ready"
            elif phase == "gpu":
                gpu_busy = 0
                jobs[j] = "control_ready"
            else:
                control_busy = 0
                slots -= 1
                jobs[j] = "done"
                if j[1] == 5:
                    completed[j[0]] = now
        assert 0 <= slots <= host_slots
        assert 0 <= cpu_busy <= cpu_workers
        assert gpu_busy in (0, 1) and control_busy in (0, 1)
    # Exclude initial fill and terminal drain for a planning steady interval.
    interval = (completed[batches-21] - completed[19])/(batches-40)
    return dict(cpu_workers=cpu_workers, host_slots=host_slots,
                active_batches=active_batches, cpu_fraction=cpu_fraction,
                bulk_speed=bulk_speed, overhead_fraction=overhead_fraction,
                historical_serial_prove_sum=sum(TIMES),
                first_batch_seconds=completed[0],
                modeled_steady_interval_seconds=interval,
                modeled_batches_per_hour=3600/interval,
                modeled_speedup_vs_historical_serial=sum(TIMES)/interval)


if __name__ == "__main__":
    cases = [(1, 2, 6), (2, 3, 6), (3, 4, 6), (4, 5, 6),
             (4, 5, 1), (4, 5, 2), (4, 5, 4)]
    results = {"classification": "ESTIMATE: discrete-event planning model, not measured",
               "assumptions": "independent or valid speculative batches; identical phase fractions across all stages; no contention slowdown; CPU/GPU/control once per proof; 2 effective cores per CPU worker; no measured memory sizes",
               "cases": [simulate(*case) for case in cases],
               "sensitivity": [simulate(4, 5, 6, cpu_fraction=c, bulk_speed=g)
                               for c, g in [(.25, 8), (.35, 10), (.25, 20), (.15, 10), (.25, 3)]],
               "stronger_scenarios": [simulate(w, w+1, 8, cpu_fraction=c, bulk_speed=g)
                                      for w, c, g in [(6, .25, 20), (8, .25, 20),
                                                      (8, .25, 16), (8, .35, 20)]]}
    out = Path(__file__).with_name("queue_model_results.json")
    out.write_text(json.dumps(results, indent=2) + "\n")
    print(json.dumps(results, indent=2))
