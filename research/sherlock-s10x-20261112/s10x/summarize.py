"""Summarize the S10x JSON lines: python3 -I summarize.py <dir>."""
import json
import sys
from pathlib import Path

GATE = 1_782_573
CAP19 = 1 << 19


def load(p):
    return [json.loads(line) for line in open(p) if line.strip()]


def main(d):
    d = Path(d)
    for f in sorted(d.glob("*.jsonl")):
        rows = load(f)
        print(f"## {f.name}")
        for r in rows:
            s = r["shape"]
            if r["kind"] == "leaf":
                print(f"leaf range={r['range']} log_rows={r['w1_log_rows']} airs={s['num_airs']}"
                      f" max={s['max_air']} 2^{s['max_log_height']} cells={s['main_cells']}"
                      f" w_stack={s['w_stack_needed']} lsh={s['log_stacked_height']}"
                      f" fpqe/child={r['fpqe_rows_per_child_proof']}"
                      f" prove_s={[round(x, 1) for x in r['prove_s'][:3]]}...")
                for a in s["airs"]:
                    print(f"   {a['name']}: h={a['height']} nz={a['nonzero_rows']}"
                          f" w={a['width_common']} pre={a['width_preprocessed']} int={a['num_interactions']}")
                continue
            p = r.get("prove", {})
            nz = s["max_air_nonzero_rows"]
            print(f"node a={r['arity']} max={s['max_air']} 2^{s['max_log_height']}={s['max_height']}"
                  f" nz={nz} margin_gate={GATE - s['max_height']} margin_gate_nz={GATE - nz}"
                  f" cap19_ok={s['max_height'] <= CAP19} layout_ok={s['layout_ok']}"
                  f" w_stack={s['w_stack_needed']} cells={s['main_cells']} T={s['gkr_rounds_T']}"
                  f" tracegen_s={r['tracegen_s']:.1f} over_gate={r['over_gate']}"
                  f" prove={ {k: v for k, v in p.items() if k != 'proof_log_heights'} }")
            if r["arity"] in (1, 2, 4):
                top = sorted(s["airs"], key=lambda a: -a["height"])[:6]
                print("   top:", [(a["name"][:28], a["log_height"], a["nonzero_rows"]) for a in top])


if __name__ == "__main__":
    main(sys.argv[1])
