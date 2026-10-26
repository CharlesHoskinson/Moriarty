"""Reproduce estimates only. This performs no prover or hardware benchmark."""
import json
from pathlib import Path


def amdahl(fraction, local_speedup):
    return 1 / (1 - fraction + fraction / local_speedup)


total, advice, lookups = 463.13, 306.44, 56.84
rest = total - advice - lookups
result = {
    "classification": "ESTIMATE: arithmetic from recorded external source, not a host measurement",
    "cusnark": {
        "total_s": total,
        "generate_advice_s": advice,
        "generate_lookups_s": lookups,
        "remaining_s": rest,
        "advice_fraction": advice / total,
        "advice_only_infinite_ceiling": total / (total - advice),
        "advice_and_lookup_infinite_ceiling": total / rest,
        "original_cpu_ceiling_if_advice_lookup_unchanged": 1762.75 / (advice + lookups),
        "advice_only": {
            str(s): {"new_total_s": total-advice+advice/s,
                     "total_speedup": total/(total-advice+advice/s)}
            for s in [1.2, 1.5, 2, 3, 4, 5, 8, 10, 20]
        },
    },
    "parent_unmeasured_fraction_sensitivity": {
        str(f): {str(s): amdahl(f, s) for s in [2, 4, 8, 10, 20]}
        for f in [0.1, 0.2, 0.5, 0.9]
    },
    "prior_astra_joint_scenario": {
        "required_witness_factor": .2 / (.1 - .78 / 20 - .02 - .01),
        "witness8_bulk20_gain": 1 / (.2 / 8 + .78 / 20 + .02 + .01),
        "witness_only8_gain": amdahl(.2, 8),
    },
    "scheduler": {
        "ideal_20core_gain_80percent_parallel": 1 / (.2 + .8 / 20),
        "ideal_20core_gain_95percent_parallel": 1 / (.05 + .95 / 20),
        "required_parallel_fraction_for_10x_at_20cores": .9 / .95,
    },
    "inverse_frontier": {
        "assumed_inverse_multiply_ratio": 80,
        "batches": {str(n): n*80/(3*n+80) for n in [1, 32, 256, 4096]},
        "witness_gain_if_50percent_inverses_frontier256": amdahl(.5, 256*80/(3*256+80)),
    },
    "g5": {
        "two_snapshots_per_event_vs_reuse_ratio": {str(n): 2*n/(n+1) for n in [10, 1000]},
        "serial_full_chain_gain_if_5percent_g5_and_10x_g5": amdahl(.05, 10),
    },
}
out = Path(__file__).with_name("bounds.json")
out.write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
