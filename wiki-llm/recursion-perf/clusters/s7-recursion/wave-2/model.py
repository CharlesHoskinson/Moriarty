"""ESTIMATE: capacity/Amdahl arithmetic only; no proving benchmark."""

import json


def model():
    # Seven unusable rows is the historical inventory's configuration,
    # not a measured property of the not-yet-built C1 relation.
    child_allowances = {"a102": 2**13 - 7, "a80": 2**12 - 7}
    glue = {
        "link_context_envelopes_heights": 1024,
        "program_states_ordered_trace_hashes": 4096,
        "statement_hash": 1024,
        "communication_opening_public_template_and_reserve": 2048,
        "outer_blake2b_circuit_rows": 0,
        "inner_proof_verifier_and_pairing_rows": 0,
    }
    children = sum(child_allowances.values())
    total = children + sum(glue.values())
    points = []
    for k in (15, 16):
        q = 2 ** (k - 20)
        usable = 2**k - 7
        points.append({
            "k": k,
            "usable_if_u_is_7": usable,
            "provisional_total_rows": total,
            "headroom": usable - total,
            "max_glue_if_children_unchanged": usable - children,
            "child_expansion_limit_if_glue_8192": (usable - 8192) / children,
            "domain_ratio_q": q,
            "strict_10x_h_ceiling": (0.1 - q) / (1 - q),
            "scenarios": [
                {"h": h, "time_ratio": h + (1-h)*q,
                 "gain": 1/(h+(1-h)*q),
                 "a3_check_wall_proxy_seconds": [
                     t*(h+(1-h)*q) for t in (315.880, 382.278)
                 ]}
                for h in (0.02, 0.04, 0.06, 0.10)
            ],
        })
    n, d = 867.279750, 586.381690
    return {
        "classification": "ESTIMATE, arithmetic only; all source constants are historical",
        "child_capacity_allowances_not_row_measurements": child_allowances,
        "glue_reservations_not_row_measurements": glue,
        "points": points,
        "hash_input_counts_for_P86_S81": {
            "program": 87, "three_states": 3*82, "trace": 21,
            "statement": 19, "communication": 2, "total": 375,
        },
        "recursive_node_decider_fusion_control": {
            "baseline_N_plus_D": n+d,
            "optimistic_same_k_gain": (n+d)/n,
            "doubled_domain_fft_proxy_seconds": n*2.1,
            "doubled_domain_slowdown_fraction": n*2.1/(n+d)-1,
        },
    }


if __name__ == "__main__":
    print(json.dumps(model(), indent=2))
