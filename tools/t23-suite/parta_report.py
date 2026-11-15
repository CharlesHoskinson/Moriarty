#!/usr/bin/env python3
"""T23 Part A: write PREVIEW-20261115.md from the zero-tx probe, the plan, the rerun's receipts and state, and the
per-call diff against 2026-11-14. Works on a partial rerun: programs not yet run are listed as NOT RUN YET.

Usage: parta_report.py OUT.md
Inputs: ~/.cache/t23/partA/{probe.json, suite/PREVIEW.md (report.py of the rerun), diff.json, home/state},
the 2026-11-14 suite (mycroft branch worktree) and the T23 logs.
"""
import json
import pathlib
import subprocess
import sys

H = pathlib.Path.home()
A = H / ".cache/t23/partA"
OLD = H / "Projects/Moriarty-wt/mycroft/mycroft/tests/compact-suite"
HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import parta  # noqa: E402


def sh(*a):
    return subprocess.run(a, capture_output=True, text=True).stdout.strip()


def status(have_new):
    log = H / ".cache/t23/logs/partA.log"
    text = log.read_text(errors="replace") if log.exists() else ""
    if "PARTA DONE" in text and have_new:
        return "COMPLETE"
    stops = [l for l in text.splitlines() if "; stop" in l]
    if stops:
        return "STOPPED: " + stops[-1].split(" ", 2)[-1]
    return ("IN PROGRESS: the rerun unit t23-partA runs one program at a time, each gated on the public Preview indexer "
            "(it returned 502, 503 or timeouts for long stretches) and on a heavy slot")


def main():
    out = pathlib.Path(sys.argv[1])
    probe = json.loads((A / "probe.json").read_text())
    rows, plan_tot = parta.plan()
    st = A / "home/state"
    actual = json.loads(sh("python3", "-I", str(HERE / "parta.py"), "count") or "{}")
    new_md = A / "suite/PREVIEW.md"
    have_new = new_md.exists()
    if not have_new:  # nothing reported yet: diff against an empty rerun, every call NOT RUN YET
        new_md = A / "empty-PREVIEW.md"
        new_md.write_text("## Calls\n")
    sh("python3", "-I", str(HERE / "parta_diff.py"), str(OLD / "PREVIEW.md"), str(new_md), str(A / "diff.json"),
       str(H / ".cache/mycroft-preview/state"), str(st))
    diff = json.loads((A / "diff.json").read_text())
    ran = {f.stem for f in st.glob("*.json") if not f.name.endswith(".txcmp.json") and not f.name.startswith("start-")
           and json.loads(f.read_text()).get("finishedAt")}
    idn = probe["identity"]
    L = ["# Mycroft Compact suite on Midnight Preview, rerun 2026-11-15 (T23 Part A)", "",
         "The T18 (a) suite rerun natively on Preview with the same pinned stack (compactc 0.36.0, midnight-js 5.0.0, "
         "proof server 9.0.1, node 2.1.101, indexer 4.4.101), the same harness (tools/preview/preview.mjs, compare.mjs, "
         "report.py of mycroft c1c78e66f, run from a copy under ~/.cache/t23/partA so the 2026-11-14 receipts are not "
         "touched) and the same funded wallet OP-PV-WALLET. Preview only. All numbers are MEASURED.", "",
         f"Status: {status(have_new)}.", "",
         "## Chain after the hard fork", "",
         f"Zero-transaction probe at {idn['at']} (tools/t23-suite/preview_probe.py): genesis `{idn['genesis']}` (the pinned "
         f"Preview genesis), node `{idn['system_version']}`, spec {idn['specVersion']}, ledger {idn['ledgerVersion']}, tip "
         f"{idn['tip']}. On 2026-11-10 the same node reported `2.1.101-913ec011`, spec 2001100, ledger 1.0.1: the node build "
         "changed, the runtime and ledger did not. The ledger 9 hard fork relaunched Preview on 2026-11-07, before the "
         "T18 (a) run, so no fork lies between the two runs.", "",
         f"- {probe['summary']['contracts']} T18 (a) contract instances (33 programs plus the archived first x01 instance): "
         f"state served for {probe['summary']['statePresent']}; {probe['summary']['blocksChecked']} recorded deploy and call "
         f"blocks checked, {probe['summary']['blockHashMismatches']} block hash mismatches.",
         f"- {probe['summary']['unchangedSinceDeploy']} instances hold exactly their deploy-block state (midnight_contractState "
         "at the deploy block and at the tip are equal).",
         "- The public indexer answered 502, 503 or timed out for most of 12:50Z to 14:10Z; the rerun gates each "
         "program on three indexer answers within 10 blocks of the node tip.", "",
         "## Reuse or redeploy (LEAD PICK)", "",
         "An instance is reused when its state still equals its deploy-block state (so the scenario starts from the same "
         "state as native); every other program is deployed fresh, because its 2026-11-14 scenario changed its state. "
         "Nothing is redeployed because the chain broke it: the probe found every instance served and unchanged in "
         "identity.", "", "| program | mode | why | planned tx |", "|---|---|---|---:|"]
    for r in rows:
        L.append(f"| `{r['id']}` | {r['mode']} | {r['why']} | {r['tx']} |")
    L += ["", f"Planned Preview transactions (printed before the first send): {plan_tot['tx']} "
          f"({plan_tot['deploys']} deploys, {plan_tot['callees']} callee deploy, {plan_tot['keyInserts']} verifier-key inserts, "
          f"{plan_tot['calls']} calls expected included, {plan_tot['nodeRefusedSubmissions']} submissions expected refused by the node).",
          f"Actual Preview transactions: {actual.get('tx', 0)} ({', '.join(f'{k} {v}' for k, v in actual.items() if k != 'tx')}).", ""]
    notrun = [r["id"] for r in rows if r["id"] not in ran]
    L += [f"Programs run: {len(ran)} of {len(rows)}." + (f" NOT RUN YET: {', '.join(notrun)}." if notrun else ""), ""]
    if diff:
        o, n = diff["old"], diff["new"]
        L += ["## Totals and diff against 2026-11-14", "",
              "| result | 2026-11-14 | 2026-11-15 |", "|---|---:|---:|"]
        for b in ("pass", "refused-as-native", "mismatch", "blocked/not realizable", "n/a"):
            L.append(f"| {b} | {o.get(b, 0)} | {n.get(b, 0)} |")
        L += ["", f"Calls whose result changed: {diff['changed']}" + ("" if have_new else " (every call: none ran yet)") + ".", ""]
        ch = [r for r in diff["rows"] if r["changed"]] if have_new else []
        if ch:
            L += ["| program | call | 2026-11-14 | 2026-11-15 | raw harness verdict, error |", "|---|---|---|---|---|"]
            for r in ch:
                nr = r.get("newRaw") or {}
                L.append(f"| `{r['program']}` | `{r['call']}` | {r['old']} | {r['new'] or 'NOT RUN YET'} | "
                         f"{nr.get('verdict') or '-'} {('codes ' + ','.join(nr['nodeErrorCodes'])) if nr.get('nodeErrorCodes') else ''} "
                         f"{(nr.get('error') or '')[:160]} |")
            L.append("")
        L += ["## Calls", "", "| program | call | circuit | 2026-11-15 verdict | tx hash | block | words (chain/native) | 2026-11-14 verdict |",
              "|---|---|---|---|---|---:|---|---|"]
        for r in diff["rows"]:
            L.append(f"| `{r['program']}` | `{r['call']}` | `{r['circuit']}` | {r['new'] or 'NOT RUN YET'} | {r['new_tx'] or '-'} | "
                     f"{r['new_block'] or '-'} | {r['words'] or '-'} | {r['old']} |")
    L += ["", "## Raw receipts", "",
          "- `preview-20261115/<id>.json` and `preview-20261115/<id>/<label>.<j>.tx-vector.json`: the rerun's receipts (public "
          "fields only), the same schema as `preview/` of 2026-11-14.",
          "- `preview-20261115/probe.json`: the zero-transaction probe; `preview-20261115/diff.json`: the per-call diff.",
          "- Rerun: `tools/t23-suite/parta.sh` in a systemd user unit (logs `~/.cache/t23/logs/partA.log`)."]
    out.write_text("\n".join(L) + "\n")
    print(out)


if __name__ == "__main__":
    main()
