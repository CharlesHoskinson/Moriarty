#!/usr/bin/env python3
"""T23 Part B benchmark: per-stage wall time of the Compact -> Sherlock -> Preview path (bench.jsonl, BENCH.md).

Owner 2026-11-15: "validate the end to end design by compiling the compact contracts to sherlock and then deploying
the proofs to Midnight Preview. Include a benchmark table showing the time it takes throughout the entire process".

This file adds no timer of its own. Every number comes from a timer that the tools already print, or from the wall
clock of partb-bench.sh around a tool that prints none:

  stage           source
  compactc        partb-bench.sh compactc: process wall of `compactc --skip-zk` (muninn)
  mycroft         partb-bench.sh: process wall of mycroft-emit on a one-program suite (Mycroft compile to R_SL)
  rsl_exec        partb-bench.sh: process wall of `rsl_suite DIR OUT exec` on one call (R_SL execution + witness)
  rsl_app         RSLPROBE app_prove_s   (t7_chain rsl: R_SL app proof)
  rsl_leaf        RSLPROBE leaf_prove_s  (leaf<1> over the R_SL app proof)
  n1, n2, n3      T7CHAIN times n1_prove, n2_prove, n3_prove (t7_chain chain)
  l, lfr          T10CALL l_prove_s, lfr_prove_s (lfr_call / t7_chain t10; t14.cold, t14.proof_bytes_bitcode)
  wrap            E5JSON phase "prove" prove_s (keygen_s kept apart; proof_bytes)
  ledger_check    E5JSON phase "ledger_9_1_0_1_verify" verify_seconds (ledger-check, stock 9.1.0.1 crates)
  preview_submit  t10a-state.json submissions[tag].seconds (submit to inclusion of acceptRun)
  preview_deploy  t10a-state.json submissions["deploy-ledger"] (settle rows: tx hash, block, fee; not in call totals)

T23 final run (T23-FINAL-PLAN.md): `ingest --final-dir DIR` reads a box return of final/final-box.sh: times.tsv rows
final_chain / final_wrap, and per call calls/<label>/prove-result.partial.json (seconds{} per stage, device, keys),
rsl-chain.json (R_SL bucket), keys.out (T25SUITE check) and wrap.log (E5JSON prove). `--settle STATE.json=PROG/CALL`
reads one SherlockLedger settle (deploy and acceptRun, tx hash and block; refused submissions counted). Rows carry
their box, so a calibration call proved on two boxes keeps both rows. BENCH.md then adds the per program by box
table (prove, wrap, ledger check, Preview tx, blocks), the settle table and the Preview tx count (MEASURED included,
DERIVED full suite at one ledger per call).

A stage without a measurement is null with a reason, never 0. A call has an end-to-end total only when every stage
is measured. Rows of subject "standin:A" are the universal upper chain (N1..wrap) measured on the slice program A
on the same box; they are labelled as such and never summed into an R_SL call's total.

Usage:
  bench.py ingest --run NAME --box-dir DIR [--box-dir DIR ...] [--muninn-dir DIR] [--preview STATE.json --map TAG=PROG/CALL]
                  [--final-dir DIR ...] [--settle STATE.json=PROG/CALL ...] [--derived-tx N]
                 --out BENCH_DIR        writes BENCH_DIR/bench.jsonl and BENCH_DIR/BENCH.md
  bench.py render BENCH.jsonl [--title T] writes BENCH.md beside it
  bench.py selftest                      parsers on lines cut from real logs, render, aggregation
"""
import argparse
import json
import pathlib
import re
import statistics
import sys

STAGES = ["compactc", "mycroft", "rsl_exec", "rsl_app", "rsl_leaf", "n1", "n2", "n3", "l", "lfr", "wrap",
          "ledger_check", "preview_submit"]
LABEL = {"compactc": "compactc", "mycroft": "Mycroft -> R_SL", "rsl_exec": "R_SL exec + witness", "rsl_app": "R_SL app proof",
         "rsl_leaf": "leaf proof", "n1": "N1", "n2": "N2", "n3": "N3", "l": "L", "lfr": "L_Fr", "wrap": "k=21 wrap",
         "ledger_check": "ledger-check 9.1.0.1", "preview_submit": "Preview submit -> inclusion"}
PROGRAM_STAGES = ("compactc", "mycroft")
CALL_STAGES = [s for s in STAGES if s not in PROGRAM_STAGES]
UPPER = ["n1", "n2", "n3", "l", "lfr", "wrap", "ledger_check"]
PROVE = ["rsl_exec", "rsl_app", "rsl_leaf", "n1", "n2", "n3", "l", "lfr"]   # the chain process (rslchain key)
MUNINN = ("ledger_check", "preview_submit")   # stages measured on muninn for a call proved on a box
# the t23-prove-result/1 seconds{} keys a prover box returns in the final run (t23.py BoxProver.message)
RESULT_SECONDS = ["rsl_exec", "rsl_app", "rsl_leaf", "n1", "n2", "n3", "l", "lfr", "wrap"]
BLOCK_UPPER = ("rehearsal only: R_SL leaf -> N1 not wired at openvm rekey-int 95cf796: `t7_chain rsl` keys N1 over the R_SL leaf "
               "but does not prove it (settle_ready false, SLC-012 planner key_root); see the standin:A rows")
TSV = ["stage", "program", "call", "box", "device", "keys", "seconds", "rc", "start", "end", "load_start", "load_end",
       "step", "log"]


def marker(lines, tag):
    """JSON payloads of the lines `TAG {json}` (TAG followed by a space and a JSON object)."""
    out = []
    for ln in lines:
        if ln.startswith(tag + " "):
            body = ln[len(tag) + 1:].strip()
            if body.startswith("times "):
                body = body[len("times "):]
            try:
                out.append(json.loads(body))
            except json.JSONDecodeError:
                pass
    return out


def read_tsv(p):
    rows = []
    if not pathlib.Path(p).exists():
        return rows
    for ln in pathlib.Path(p).read_text().splitlines():
        if not ln.strip() or ln.startswith("#"):
            continue
        f = ln.split("\t")
        r = dict(zip(TSV, f))
        r["seconds"], r["rc"] = float(r["seconds"]), int(r["rc"])
        rows.append(r)
    return rows


def row(run, subject, program, call, stage, seconds, box, device, keys, source, reason="", **extra):
    r = {"schema": "t23-bench/1", "run": run, "subject": subject, "program": program, "call": call, "stage": stage,
         "seconds": None if seconds is None else round(float(seconds), 3), "box": box, "device": device, "keys": keys,
         "source": source, "reason": reason}
    r.update({k: v for k, v in extra.items() if v is not None})
    return r


def lines_of(d, rel):
    p = pathlib.Path(d) / rel
    return p.read_text(errors="replace").splitlines() if p.exists() else []


def ingest_box(run, d):
    """One box result dir of partb-bench.sh box."""
    d = pathlib.Path(d)
    out, calls = [], []
    for t in read_tsv(d / "times.tsv"):
        st, prog, call, box, dev, keys = t["stage"], t["program"], t["call"], t["box"], t["device"], t["keys"]
        src = f"{box}:{t['log']} (wall)"
        ok = t["rc"] == 0
        why = "" if ok else f"exit {t['rc']} ({t['log']})"
        common = {"load_start": t["load_start"], "load_end": t["load_end"]}
        if st == "build":
            out.append(row(run, "build", "-", t["step"], "build", t["seconds"] if ok else None, box, dev, "-", src, why, **common))
        elif st in ("compactc", "mycroft"):
            n = None
            if st == "mycroft":
                n = len(list((d / "out" / f"calls-{prog}").glob("*.rsl.json"))) or None
            out.append(row(run, "program", prog, "-", st, t["seconds"] if ok else None, box, dev, "-", src, why,
                           calls_emitted=n, **common))
        elif st == "rsl_exec":
            ex = d / "out" / f"exec-{prog}.{call}.json"
            status = mstat = None
            if ex.exists():
                cs = json.loads(ex.read_text()).get("calls", [])
                status = cs[0].get("status") if cs else None
                mstat = cs[0].get("mycroft_status") if cs else None
            good = ok and status in ("executed", "proved")
            calls.append((prog, call, box))
            out.append(row(run, "rsl-call", prog, call, "rsl_exec", t["seconds"] if good else None, box, "CPU", "-", src,
                           "" if good else (why or f"rsl_suite status {status}"), rsl_status=status, mycroft_status=mstat, **common))
        elif st == "rsl_app_leaf":
            pr = marker(lines_of(d, t["log"]), "RSLPROBE")
            p = pr[0] if pr else None
            msrc = f"{box}:{t['log']} RSLPROBE"
            for stage, k in (("rsl_app", "app_prove_s"), ("rsl_leaf", "leaf_prove_s")):
                out.append(row(run, "rsl-call", prog, call, stage, p.get(k) if (p and ok) else None, box, "CPU", "cold", msrc,
                               "" if (p and ok) else (why or "no RSLPROBE line"),
                               keygen_s_derived=(round(p["total_s"] - p["app_prove_s"] - p["leaf_prove_s"], 3) if (p and stage == "rsl_app") else None),
                               n1_pre_hash_equals_universal_n1=(p or {}).get("n1_pre_hash_equals_universal_n1") if stage == "rsl_leaf" else None,
                               **common))
        elif st == "standin_chain":
            tm = marker(lines_of(d, t["log"]), "T7CHAIN")
            tm = next((x for x in tm if "n1_prove" in x), None)
            for stage in ("n1", "n2", "n3"):
                out.append(row(run, "standin:A", "A", "chain", stage, tm.get(f"{stage}_prove") if (tm and ok) else None, box, dev,
                               keys, f"{box}:{t['log']} T7CHAIN times", "" if (tm and ok) else (why or "no T7CHAIN times"), **common))
        elif st == "standin_lfr":
            tc = marker(lines_of(d, t["log"]), "T10CALL")
            c = tc[-1] if tc else None
            t14 = (c or {}).get("t14", {})
            k = "cold" if (c and (c.get("mode") == "key" or t14.get("cold"))) and call == "key" else keys
            for stage, key in (("l", "l_prove_s"), ("lfr", "lfr_prove_s")):
                out.append(row(run, "standin:A", "A", call, stage, c.get(key) if (c and ok) else None, box, dev if stage == "lfr" else "CPU",
                               k, f"{box}:{t['log']} T10CALL", "" if (c and ok) else (why or "no T10CALL line"),
                               tree_s=(c or {}).get("tree_s") if stage == "l" else None,
                               lfr_keygen_or_load_s=(c or {}).get("lfr_keygen_or_load_s") if stage == "lfr" else None,
                               proof_bytes=t14.get("proof_bytes_bitcode") if stage == "lfr" else None, **common))
        elif st == "standin_wrap":
            e = [x for x in marker(lines_of(d, t["log"]), "E5JSON") if x.get("phase") == "prove"]
            e = e[-1] if e else None
            out.append(row(run, "standin:A", "A", call, "wrap", e.get("prove_s") if (e and ok) else None, box, dev, keys,
                           f"{box}:{t['log']} E5JSON prove", "" if (e and ok) else (why or "no E5JSON prove line"),
                           keygen_s=(e or {}).get("keygen_s"), proof_bytes=(e or {}).get("proof_bytes"),
                           vk_bytes=(e or {}).get("key_bytes_midnightvk"), **common))
        elif st == "ledger_check":
            e = [x for x in marker(lines_of(d, t["log"]), "E5JSON") if x.get("phase") == "ledger_9_1_0_1_verify"]
            e = e[-1] if e else None
            good = ok and e and e.get("result") == "Ok(())"
            out.append(row(run, "standin:A" if prog == "A" else "rsl-call", prog, call, "ledger_check",
                           e.get("verify_seconds") if good else None, box, "CPU", "-", f"{box}:{t['log']} E5JSON ledger",
                           "" if good else (why or "ledger-check did not print Ok(())"), wall_s=t["seconds"],
                           negatives_refused=(all(e.get("changed_public_input_refused", [False])) and all(e.get("flipped_proof_byte_refused", [False]))) if e else None,
                           **common))
    return out, calls


def ingest_preview(run, state, mapping, box="muninn"):
    """mapping: {tag: "prog/call"}; seconds = submit to inclusion of acceptRun (t10a-ledger-run.mjs)."""
    s = json.loads(pathlib.Path(state).read_text())
    out = []
    for tag, pc in mapping.items():
        prog, call = pc.split("/", 1)
        sub = s.get("submissions", {}).get(tag)
        inc = sub and sub.get("verdict") == "included"
        out.append(row(run, "rsl-call", prog, call, "preview_submit", sub.get("seconds") if inc else None, box, "-", "-",
                       f"{state} submissions.{tag}", "" if inc else f"not included ({(sub or {}).get('verdict')})",
                       tx=(sub or {}).get("txHash"), block=(sub or {}).get("blockHeight")))
    return out


def from_result(run, prog, call, box, result):
    """Final run: the t23-prove-result/1 of one call (seconds{} per stage, device, keys, proof bytes)."""
    sec = result.get("seconds") or {}
    dev = result.get("device") or {}
    return [row(run, "rsl-call", prog, call, s, sec.get(s), box, dev.get(s, "-") if isinstance(dev, dict) else dev,
                result.get("keys", "-"), f"{box}: t23-prove-result/1", "" if sec.get(s) is not None else "not in the prove result",
                proof_bytes=(result.get("wrap_proof_bytes") if s == "wrap" else None)) for s in RESULT_SECONDS]


def ingest_final(run, d):
    """One box return of final/final-box.sh (T23 final run on the T25 keys)."""
    d = pathlib.Path(d)
    out, calls = [], []
    for t in read_tsv(d / "times.tsv"):
        st, prog, call, box = t["stage"], t["program"], t["call"], t["box"]
        cd = d / "calls" / f"{prog}.{call}"
        ok = t["rc"] == 0
        why = "" if ok else f"exit {t['rc']} ({t['log']})"
        common = {"load_start": t["load_start"], "load_end": t["load_end"]}
        if st == "final_chain":
            calls.append((prog, call, box))
            pr = cd / "prove-result.partial.json"
            res = json.loads(pr.read_text()) if (ok and pr.exists()) else None
            rc = json.loads((cd / "rsl-chain.json").read_text()) if (cd / "rsl-chain.json").exists() else {}
            kl = (cd / "keys.out").read_text().splitlines() if (cd / "keys.out").exists() else []
            extra = {"bucket": rc.get("rsl_bucket"), "keys_check": kl[0].split(" ")[0] if kl else None,
                     "chain_wall_s": t["seconds"], **common}
            if res:
                for r in from_result(run, prog, call, box, res):
                    if r["stage"] in PROVE:
                        r["source"] = f"{box}:{t['log']} prove-result seconds"
                        r.update(extra)
                        out.append(r)
            else:
                for s in PROVE:
                    out.append(row(run, "rsl-call", prog, call, s, None, box, "-", "cold", f"{box}:{t['log']}",
                                   why or "no prove-result.partial.json", **extra))
        elif st == "final_wrap":
            e = [x for x in marker(lines_of(d, t["log"]), "E5JSON") if x.get("phase") == "prove"]
            e = e[-1] if e else None
            out.append(row(run, "rsl-call", prog, call, "wrap", e.get("prove_s") if (e and ok) else None, box, t["device"],
                           t["keys"], f"{box}:{t['log']} E5JSON prove", "" if (e and ok) else (why or "no E5JSON prove line"),
                           keygen_s=(e or {}).get("keygen_s"), proof_bytes=(e or {}).get("proof_bytes"),
                           wrap_wall_s=t["seconds"], **common))
    return out, calls


def ingest_settle(run, state, prog, call, box="muninn"):
    """One SherlockLedger settle on Preview (t10a-state.json of one ledger): deploy + acceptRun, refusals counted."""
    s = json.loads(pathlib.Path(state).read_text())
    subs = s.get("submissions", {})
    out = []
    refused = sum(1 for k, v in subs.items() if k not in ("deploy-ledger", "A") and v.get("verdict") != "included")
    for tag, stage in (("deploy-ledger", "preview_deploy"), ("A", "preview_submit")):
        sub = subs.get(tag)
        inc = bool(sub) and sub.get("verdict") == "included"
        out.append(row(run, "settle" if stage == "preview_deploy" else "rsl-call", prog, call, stage,
                       sub.get("seconds") if inc else None, box, "-", "-", f"{state} submissions.{tag}",
                       "" if inc else f"not included ({(sub or {}).get('verdict')})", tx=(sub or {}).get("txHash"),
                       block=(sub or {}).get("blockHeight"), included=inc,
                       refused_submissions=refused if stage == "preview_submit" else None))
    return out


def fill_blocked(rows, calls, run, preview_note, upper_note=None):
    """Every R_SL call gets a row per stage; a stage without a measurement is null with its reason."""
    have = {(r["program"], r["call"], r["box"], r["stage"]) for r in rows if r["subject"] == "rsl-call"}
    have_any = {(r["program"], r["call"], r["stage"]) for r in rows if r["subject"] == "rsl-call"}
    for prog, call, box in calls:
        for s in CALL_STAGES:
            if (prog, call, box, s) in have or (s in MUNINN and (prog, call, s) in have_any):
                continue
            why = preview_note if s == "preview_submit" else ((upper_note or BLOCK_UPPER) if s in UPPER else "not run")
            rows.append(row(run, "rsl-call", prog, call, s, None, box, "-", "-", "-", why))
    return rows


# ---------------------------------------------------------------- statistics and rendering
def p95(xs):
    """Nearest-rank 95th percentile."""
    xs = sorted(xs)
    return xs[max(0, -(-95 * len(xs) // 100) - 1)] if xs else None


def fmt(x):
    if x is None:
        return "—"
    return f"{x:,.1f}" if x >= 10 else f"{x:.2f}"


def hms(x):
    if x is None:
        return "—"
    m, s = divmod(int(round(x)), 60)
    h, m = divmod(m, 60)
    return f"{h}h{m:02d}m{s:02d}s" if h else (f"{m}m{s:02d}s" if m else f"{x:.1f} s")


def stats(rows, subject):
    out = {}
    for s in STAGES:
        xs = [r["seconds"] for r in rows if r["stage"] == s and r["subject"] in subject and r["seconds"] is not None]
        out[s] = {"n": len(xs), "median": statistics.median(xs) if xs else None, "p95": p95(xs), "min": min(xs) if xs else None,
                  "max": max(xs) if xs else None}
    return out


def render(rows, title="T23 Part B benchmark", meta=None):
    meta = meta or {}
    # a call is (program, call, proving box); ledger check and Preview rows (muninn) attach to every box of the call
    cr = [r for r in rows if r["subject"] == "rsl-call"]
    pbox = {}
    for r in cr:
        if r["stage"] not in MUNINN:
            pbox.setdefault((r["program"], r["call"]), set()).add(r["box"])
    calls = sorted({(r["program"], r["call"], b) for r in cr for b in (pbox.get((r["program"], r["call"])) or {r["box"]})})
    progs = sorted({r["program"] for r in rows if r["subject"] in ("program", "rsl-call")})
    exact = {(r["program"], r["call"], r["box"], r["stage"]): r for r in cr}
    anyb = {}
    for r in cr:
        if r["stage"] in MUNINN:
            anyb.setdefault((r["program"], r["call"], r["stage"]), r)

    class By(dict):
        def get(self, k, d=None):
            if k[0] == "rsl-call":
                _, p, c, b, st = k
                return exact.get((p, c, b, st)) or (anyb.get((p, c, st)) if st in MUNINN else None) or d
            return dict.get(self, k, d)

        def __getitem__(self, k):
            v = self.get(k)
            if v is None:
                raise KeyError(k)
            return v
    by = By({(r["subject"], r["program"], r["call"], r["stage"]): r for r in rows if r["subject"] != "rsl-call"})
    prog_rows = {(r["program"], r["stage"]): r for r in rows if r["subject"] == "program"}
    totals, complete = {}, 0
    for pc in calls:
        xs = [by.get(("rsl-call", *pc, s)) for s in CALL_STAGES]
        pg = [prog_rows.get((pc[0], s)) for s in PROGRAM_STAGES]
        allx = [r for r in xs + pg if r]
        ok = all(r and r["seconds"] is not None for r in xs + pg)
        totals[pc] = sum(r["seconds"] for r in allx) if ok else None
        complete += ok
    sa = [r for r in rows if r["subject"] == "standin:A"]
    boxes = sorted({r["box"] for r in rows if r["box"] not in ("-",)})
    L = [f"# {title}", ""]
    for k, v in meta.items():
        L.append(f"- {k}: {v}")
    if meta:
        L.append("")
    # one-line summary
    if complete:
        tt = [v for v in totals.values() if v is not None]
        L.append(f"**End to end (compactc to Preview inclusion), {complete} of {len(calls)} calls complete:** median "
                 f"{hms(statistics.median(tt))}, p95 {hms(p95(tt))}, max {hms(max(tt))}.")
    else:
        front = [sum(by[("rsl-call", *pc, s)]["seconds"] for s in ("rsl_exec", "rsl_app", "rsl_leaf"))
                 for pc in calls if all(by.get(("rsl-call", *pc, s)) and by[("rsl-call", *pc, s)]["seconds"] is not None
                                        for s in ("rsl_exec", "rsl_app", "rsl_leaf"))]
        up = {}
        for b in boxes:
            u = [next((r["seconds"] for r in sa if r["stage"] == s and r["box"] == b and r["seconds"] is not None
                       and r.get("keys") != "cold"), None) or next((r["seconds"] for r in sa if r["stage"] == s and r["box"] == b
                                                                    and r["seconds"] is not None), None) for s in UPPER]
            if all(x is not None for x in u):
                up[b] = sum(u)
        L.append(f"**End to end: no call has every stage measured ({len(calls)} calls).** "
                 + (f"R_SL front (exec, app, leaf) median {hms(statistics.median(front))} over {len(front)} calls; " if front else "")
                 + ("universal upper chain N1 to ledger-check on the stand-in program A: "
                    + ", ".join(f"{b} {hms(v)}" for b, v in sorted(up.items())) + "; " if up else "")
                 + "Preview submit not measured in this run.")
    L += ["", "Times are wall seconds. A dash is a stage without a measurement (reason in the notes); a call has a total "
          "only when every stage is measured. Keys: cold = the process generated or loaded its keys in that run, warm = "
          "keys resident from a previous call.", ""]
    # per-call table
    L += ["## Per call", "", "| program | call | box | " + " | ".join(LABEL[s] for s in CALL_STAGES) + " | total |",
          "|---|---|---|" + "---:|" * (len(CALL_STAGES) + 1)]
    notes = {}
    for pc in calls:
        cells = []
        for s in CALL_STAGES:
            r = by.get(("rsl-call", *pc, s))
            if r and r["seconds"] is not None:
                cells.append(fmt(r["seconds"]))
            else:
                why = (r or {}).get("reason") or "not run"
                n = notes.setdefault(why, len(notes) + 1)
                cells.append(f"— [{n}]")
        L.append(f"| `{pc[0]}` | `{pc[1]}` | {pc[2]} | " + " | ".join(cells) + f" | {hms(totals[pc]) if totals[pc] is not None else '—'} |")
    if notes:
        L += [""] + [f"[{n}] {w}" for w, n in notes.items()]
    # per-program totals
    L += ["", "## Per program", "", "| program | calls | compactc | Mycroft -> R_SL | measured call stages (sum) | stages measured | total |",
          "|---|---:|---:|---:|---:|---:|---:|"]
    for p in progs:
        pcs = [pc for pc in calls if pc[0] == p]
        tot_ok = pcs and all(totals[pc] is not None for pc in pcs) and all((p, s) in prog_rows for s in PROGRAM_STAGES)
        mr = [by[("rsl-call", *pc, s)] for pc in pcs for s in CALL_STAGES if by.get(("rsl-call", *pc, s))]
        meas = [r for r in mr if r["seconds"] is not None]
        tot = (sum(totals[pc] for pc in pcs) - (len(pcs) - 1) * sum(prog_rows[(p, s)]["seconds"] for s in PROGRAM_STAGES)
               if tot_ok else None)
        c, m = prog_rows.get((p, "compactc")), prog_rows.get((p, "mycroft"))
        L.append(f"| `{p}` | {len(pcs)} | {fmt((c or {}).get('seconds'))} | {fmt((m or {}).get('seconds'))} | "
                 f"{fmt(sum(r['seconds'] for r in meas)) if meas else '—'} | {len(meas)} of {len(mr)} | {hms(tot)} |")
    L += per_program_by_box(rows, calls, by)
    L += settle_section(rows, calls, meta)
    # stand-in upper chain
    if sa:
        L += ["", "## Universal upper chain on the stand-in program A (same boxes)", "",
              "The node, `L`, `L_Fr` and wrap keys are universal and fixed in shape, so these stages do not depend on "
              "the Compact program; they are measured here on the slice program A because no R_SL leaf drives N1 yet.", "",
              "| box | stage | run | device | keys | seconds | key or load s | proof bytes | load at start |", "|---|---|---|---|---|---:|---:|---:|---|"]
        for r in sorted(sa, key=lambda r: (r["box"], UPPER.index(r["stage"]) if r["stage"] in UPPER else 99, r["call"])):
            kk = r.get("keygen_s") or r.get("lfr_keygen_or_load_s")
            L.append(f"| {r['box']} | {LABEL[r['stage']]} | {r['call']} | {r['device']} | {r['keys']} | {fmt(r['seconds'])} | "
                     f"{fmt(kk) if kk is not None else '—'} | {r.get('proof_bytes') or '—'} | {r.get('load_start', '—')} |")
    # suite statistics
    L += ["", "## Suite statistics per stage", "", "| stage | subject | n | median | p95 | min | max |", "|---|---|---:|---:|---:|---:|---:|"]
    for subj, name in ((("program", "rsl-call"), "R_SL calls"), (("standin:A",), "stand-in A")):
        st = stats(rows, subj)
        for s in STAGES:
            v = st[s]
            if v["n"]:
                L.append(f"| {LABEL[s]} | {name} | {v['n']} | {fmt(v['median'])} | {fmt(v['p95'])} | {fmt(v['min'])} | {fmt(v['max'])} |")
    b = [r for r in rows if r["subject"] in ("build", "suite")]
    if b:
        L += ["", "## One-off builds and suite-level steps (not part of any call)", "", "| box | step | seconds |", "|---|---|---:|"]
        for r in b:
            L.append(f"| {r['box']} | {r['call'] if r['subject'] == 'build' else r['stage']} | {fmt(r['seconds'])} |")
    return "\n".join(L) + "\n"


def per_program_by_box(rows, calls, by):
    """Final table: per program and proving box, median prove (chain stages), wrap, ledger check; Preview tx, blocks."""
    keys = sorted({(pc[0], pc[2]) for pc in calls if pc[2] != "muninn"})
    if not any(by.get(("rsl-call", *pc, "lfr")) and by.get(("rsl-call", *pc, "lfr"))["seconds"] is not None for pc in calls):
        return []
    L = ["", "## Per program by box (final run)", "",
         "Prove = the chain process stages R_SL exec to L_Fr (median per call); wrap = E5JSON prove; ledger check = "
         "ledger 9.1.0.1 verify on muninn; Preview tx = included transactions (deploy + acceptRun) of the settled calls; "
         "blocks = acceptRun inclusion blocks.", "",
         "| program | box | calls | proved | prove median s | wrap median s | ledger check median s | Preview tx | blocks |",
         "|---|---|---:|---:|---:|---:|---:|---:|---|"]
    settle = [r for r in rows if r["stage"] in ("preview_deploy", "preview_submit")]
    for p, b in keys:
        pcs = [pc for pc in calls if pc[0] == p and pc[2] == b]
        pv = []
        for pc in pcs:
            xs = [by.get(("rsl-call", *pc, s)) for s in PROVE]
            if all(x and x["seconds"] is not None for x in xs):
                pv.append(sum(x["seconds"] for x in xs))
        med = lambda st: [by.get(("rsl-call", *pc, st))["seconds"] for pc in pcs
                          if by.get(("rsl-call", *pc, st)) and by.get(("rsl-call", *pc, st))["seconds"] is not None]
        w, lc = med("wrap"), med("ledger_check")
        cs = {(pc[0], pc[1]) for pc in pcs}
        tx = [r for r in settle if (r["program"], r["call"]) in cs and r.get("included")]
        blocks = sorted({str(r["block"]) for r in tx if r["stage"] == "preview_submit" and r.get("block")})
        L.append(f"| `{p}` | {b} | {len(pcs)} | {len(pv)} | {fmt(statistics.median(pv)) if pv else '—'} | "
                 f"{fmt(statistics.median(w)) if w else '—'} | {fmt(statistics.median(lc)) if lc else '—'} | "
                 f"{len(tx) if tx else '—'} | {', '.join(blocks) if blocks else '—'} |")
    return L


def settle_section(rows, calls, meta):
    settled = {(r["program"], r["call"]) for r in rows if r["stage"] == "preview_deploy"}
    st = [r for r in rows if r["stage"] in ("preview_deploy", "preview_submit") and (r["program"], r["call"]) in settled]
    if not st:
        return []
    L = ["", "## Preview settle (SherlockLedger, one ledger per settled call)", "",
         "| program | call | deploy tx | deploy block | acceptRun tx | acceptRun block | submit to inclusion s | refused (not included) |",
         "|---|---|---|---:|---|---:|---:|---:|"]
    pcs = sorted({(r["program"], r["call"]) for r in st})
    for pc in pcs:
        d = next((r for r in st if (r["program"], r["call"]) == pc and r["stage"] == "preview_deploy"), {})
        a = next((r for r in st if (r["program"], r["call"]) == pc and r["stage"] == "preview_submit"), {})
        sh = lambda x: f"`{x[:12]}…`" if x else "—"
        L.append(f"| `{pc[0]}` | `{pc[1]}` | {sh(d.get('tx'))} | {d.get('block') or '—'} | {sh(a.get('tx'))} | "
                 f"{a.get('block') or '—'} | {fmt(a.get('seconds'))} | {a.get('refused_submissions') or 0} |")
    inc = sum(1 for r in st if r.get("included"))
    ref = sum(r.get("refused_submissions") or 0 for r in st if r["stage"] == "preview_submit")
    proved = len({(pc[0], pc[1]) for pc in calls if pc[2] != "muninn"})
    L += ["", f"Preview tx MEASURED: {inc} included ({len(pcs)} settled calls), {ref} refused submissions (no fee, not included).",
          f"Preview tx DERIVED for the full suite at one ledger per call (acceptRun inserts one claim per tx): "
          f"{meta.get('derived_tx') or 2 * proved} included tx for {proved} proved calls."]
    return L


def write(rows, outdir, title, meta):
    outdir = pathlib.Path(outdir)
    outdir.mkdir(parents=True, exist_ok=True)
    with open(outdir / "bench.jsonl", "w") as f:
        for r in rows:
            f.write(json.dumps(r, sort_keys=True) + "\n")
    (outdir / "BENCH.md").write_text(render(rows, title, meta))


# ---------------------------------------------------------------- self test
def selftest():
    fx = pathlib.Path(__file__).resolve().parent / "fixtures" / "bench"
    ok = True

    def check(name, cond):
        nonlocal ok
        print(("PASS " if cond else "FAIL ") + name)
        ok &= bool(cond)

    p = marker((fx / "rsl-probe.log").read_text().splitlines(), "RSLPROBE")
    check("RSLPROBE parsed (odin r10 log)", len(p) == 2 and p[0]["app_prove_s"] > 0 and p[0]["leaf_prove_s"] > 0)
    c = marker((fx / "chain.log").read_text().splitlines(), "T7CHAIN")
    check("T7CHAIN times parsed", c and all(c[0][f"n{i}_prove"] > 0 for i in (1, 2, 3)))
    t = marker((fx / "t10call-key.log").read_text().splitlines(), "T10CALL")
    check("T10CALL parsed (t10 key-B log)", t and t[0]["l_prove_s"] > 0 and t[0]["lfr_prove_s"] > 0)
    w = marker((fx / "wrap.log").read_text().splitlines(), "E5JSON")
    check("E5JSON wrap prove parsed", w and w[0]["phase"] == "prove" and w[0]["proof_bytes"] == 2912)
    lg = marker((fx / "ledger.log").read_text().splitlines(), "E5JSON")
    check("E5JSON ledger 9.1.0.1 parsed", lg and lg[0]["result"] == "Ok(())" and lg[0]["verify_seconds"] > 0)
    check("p95 nearest rank", p95(list(range(1, 21))) == 19 and p95([5]) == 5 and p95([]) is None)
    # a synthetic box dir built from the real lines: times.tsv rows point at the fixture logs
    import shutil
    import tempfile
    with tempfile.TemporaryDirectory() as td:
        d = pathlib.Path(td)
        (d / "logs").mkdir()
        (d / "out" / "calls-t02-counter-example").mkdir(parents=True)
        (d / "out" / "calls-t02-counter-example" / "t02-counter-example.increment.increment.0.rsl.json").write_text("{}")
        (d / "out" / "exec-t02-counter-example.increment.increment.0.json").write_text(json.dumps({"calls": [{"status": "executed"}]}))
        for f in ("rsl-probe.log", "chain.log", "t10call-key.log", "wrap.log", "ledger.log"):
            shutil.copy(fx / f, d / "logs" / f)
        tsv = [("build", "-", "-", "CPU", "-", 300, 0, "build-ovm", "logs/x.log"),
               ("compactc", "t02-counter-example", "-", "CPU", "-", 1.5, 0, "compactc-t02", "logs/x.log"),
               ("mycroft", "t02-counter-example", "-", "CPU", "-", 0.8, 0, "mycroft-t02", "logs/x.log"),
               ("rsl_exec", "t02-counter-example", "increment.increment.0", "CPU", "-", 2.0, 0, "exec", "logs/x.log"),
               ("rsl_app_leaf", "t02-counter-example", "increment.increment.0", "CPU", "cold", 60, 0, "probe", "logs/rsl-probe.log"),
               ("standin_chain", "A", "chain", "CPU", "cold", 500, 0, "chain-A", "logs/chain.log"),
               ("standin_lfr", "A", "call", "GPU", "warm", 900, 0, "lfr-call", "logs/t10call-key.log"),
               ("standin_wrap", "A", "call", "CPU", "cold", 70, 0, "wrap-call", "logs/wrap.log"),
               ("ledger_check", "A", "call", "CPU", "-", 3, 0, "ledger-odin", "logs/ledger.log"),
               ("rsl_exec", "s02-foreign", "keccak.keccak.0", "CPU", "-", 1.0, 101, "exec2", "logs/x.log")]
        (d / "times.tsv").write_text("".join("\t".join(map(str, (r[0], r[1], r[2], "odin", r[3], r[4], r[5], r[6],
                                                                 "s", "e", "1,1,1", "2,2,2", r[7], r[8]))) + "\n" for r in tsv))
        rows, calls = ingest_box("selftest", d)
        rows = fill_blocked(rows, calls, "selftest", "rehearsal: no Preview")
        by = {(r["subject"], r["program"], r["call"], r["stage"]): r for r in rows}
        check("ingest: R_SL exec, app, leaf rows", by[("rsl-call", "t02-counter-example", "increment.increment.0", "rsl_app")]["seconds"] == round(p[0]["app_prove_s"], 3))
        check("ingest: failed exec is null with its reason, never 0",
              by[("rsl-call", "s02-foreign", "keccak.keccak.0", "rsl_exec")]["seconds"] is None
              and "exit 101" in by[("rsl-call", "s02-foreign", "keccak.keccak.0", "rsl_exec")]["reason"])
        check("ingest: upper stages of an R_SL call are null with the block reason",
              by[("rsl-call", "t02-counter-example", "increment.increment.0", "n1")]["seconds"] is None
              and "not wired" in by[("rsl-call", "t02-counter-example", "increment.increment.0", "n1")]["reason"])
        check("ingest: stand-in N1..N3, L, L_Fr, wrap, ledger-check", all(by.get(("standin:A", "A", cl, s), {}).get("seconds")
              for cl, s in (("chain", "n1"), ("chain", "n2"), ("chain", "n3"), ("call", "l"), ("call", "lfr"), ("call", "wrap"), ("call", "ledger_check"))))
        check("ingest: wrap proof bytes and keygen kept apart", by[("standin:A", "A", "call", "wrap")]["proof_bytes"] == 2912
              and by[("standin:A", "A", "call", "wrap")]["keygen_s"] > 0)
        md = render(rows, "selftest")
        check("render: no total without every stage", "no call has every stage measured" in md and "| — |" in md)
        check("render: stand-in table and stats with n", "Universal upper chain" in md and "| N1 | stand-in A | 1 |" in md)
        # a complete call gets a total and the summary line
        full = [row("s", "program", "p", "-", s, 1.0, "odin", "CPU", "-", "x") for s in PROGRAM_STAGES]
        full += [row("s", "rsl-call", "p", "c", s, 2.0, "odin", "CPU", "warm", "x") for s in CALL_STAGES]
        md2 = render(full, "full")
        check("render: complete call total = sum of its stages", "1 of 1 calls complete" in md2 and f"| {hms(2.0 * len(CALL_STAGES) + 2)} |" in md2)
        res = {"seconds": {s: 1.0 for s in RESULT_SECONDS}, "device": {"lfr": "GPU"}, "keys": "warm", "wrap_proof_bytes": 2912}
        fr = from_result("s", "p", "c", "odin", res)
        check("final-run result maps seconds{} to stages", len(fr) == len(RESULT_SECONDS) and all(r["seconds"] == 1.0 for r in fr)
              and next(r for r in fr if r["stage"] == "lfr")["device"] == "GPU")
        pv = ingest_preview("s", fx / "preview-submission.json", {"A": "p/c"})
        check("preview submit seconds from t10a-state submissions", pv[0]["seconds"] == round(6.376753076999914, 3) and pv[0]["block"] == "115187")
        # T23 final run: a final-box.sh return on two boxes (one calibration call on both), a settle state
        for bx, labels in (("huginn", ["t02-counter-example.increment.increment.0", "d02-battleship.player1Shoot.04.0"]),
                           ("odin", ["t02-counter-example.increment.increment.0", "bad.x.y.0"])):
            fd = d / f"final-{bx}"
            tl = []
            for lb in labels:
                prog, call = lb.split(".", 1)
                cd = fd / "calls" / lb
                (cd / "wrap").mkdir(parents=True)
                bad = prog == "bad"
                if not bad:
                    (cd / "prove-result.partial.json").write_text(json.dumps(
                        {"seconds": {s_: 2.0 for s_ in PROVE}, "device": {s_: "GPU" for s_ in PROVE}, "keys": "cold"}))
                    (cd / "rsl-chain.json").write_text(json.dumps({"rsl_bucket": "Sha" if prog.startswith("d02") else "Plain"}))
                    (cd / "keys.out").write_text("T25SUITE EQUAL x bucket Plain (16 universal fields)\n")
                    shutil.copy(fx / "wrap.log", cd / "wrap.log")
                tl.append(("final_chain", prog, call, "GPU", "cold", 120.0, 101 if bad else 0, f"calls/{lb}/chain.log"))
                if not bad:
                    tl.append(("final_wrap", prog, call, "CPU", "cold", 108.0, 0, f"calls/{lb}/wrap.log"))
            (fd / "times.tsv").write_text("".join("\t".join(map(str, (r[0], r[1], r[2], bx, r[3], r[4], r[5], r[6], "s", "e",
                                                                     "1,1,1", "2,2,2", "step", r[7]))) + "\n" for r in tl))
        st = {"submissions": {"deploy-ledger": {"verdict": "included", "txHash": "0xdep", "blockHeight": 135999, "seconds": 5.1},
                              "A": {"verdict": "included", "txHash": "0xacc", "blockHeight": 136144, "seconds": 3.2},
                              "r1": {"verdict": "refused-by-node"}}}
        (d / "state.json").write_text(json.dumps(st))
        fr_rows, fr_calls = [], []
        for bx in ("huginn", "odin"):
            r_, c_ = ingest_final("f", d / f"final-{bx}")
            fr_rows += r_
            fr_calls += c_
        fr_rows += ingest_settle("f", d / "state.json", "t02-counter-example", "increment.increment.0")
        fr_rows = fill_blocked(fr_rows, fr_calls, "f", "not settled (DERIVED)", "not measured")
        k2 = [r for r in fr_rows if r["program"] == "t02-counter-example" and r["stage"] == "lfr"]
        check("final: a calibration call keeps one row per box", sorted(r["box"] for r in k2) == ["huginn", "odin"])
        check("final: wrap from E5JSON, bucket and key check kept",
              any(r["stage"] == "wrap" and r["seconds"] for r in fr_rows)
              and any(r.get("bucket") == "Sha" and r.get("keys_check") == "T25SUITE" for r in fr_rows))
        check("final: failed chain is null with exit reason", all(r["seconds"] is None and "exit 101" in r["reason"]
              for r in fr_rows if r["program"] == "bad" and r["stage"] in PROVE))
        check("final: unsettled call gets the DERIVED note", any(r["stage"] == "preview_submit" and r["program"].startswith("d02")
              and r["reason"] == "not settled (DERIVED)" for r in fr_rows))
        md3 = render(fr_rows, "final", {"derived_tx": "290"})
        check("render: per program by box with Preview tx and blocks",
              "## Per program by box" in md3 and "| `t02-counter-example` | odin | 1 | 1 | 16.0 |" in md3 and "| 2 | 136144 |" in md3)
        check("render: settle table and tx counts", "| 135999 |" in md3 and "2 included (1 settled calls), 1 refused" in md3
              and "290 included tx" in md3)
    return 0 if ok else 1


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    g = sub.add_parser("ingest")
    g.add_argument("--run", required=True)
    g.add_argument("--box-dir", action="append", default=[])
    g.add_argument("--muninn-dir", action="append", default=[])
    g.add_argument("--preview")
    g.add_argument("--map", action="append", default=[], help="TAG=PROG/CALL for --preview")
    g.add_argument("--final-dir", action="append", default=[], help="box return of final/final-box.sh")
    g.add_argument("--settle", action="append", default=[], help="STATE.json=PROG/CALL (t10a-state.json of one ledger)")
    g.add_argument("--derived-tx", help="full-suite included tx (DERIVED) for the settle section")
    g.add_argument("--upper-note", help="reason for an unmeasured upper stage (default: the rehearsal block)")
    g.add_argument("--preview-note", default="rehearsal: Preview not in scope")
    g.add_argument("--title", default="T23 Part B benchmark")
    g.add_argument("--meta", action="append", default=[], help="KEY=VALUE lines for the header")
    g.add_argument("--out", required=True)
    r = sub.add_parser("render")
    r.add_argument("jsonl")
    r.add_argument("--title", default="T23 Part B benchmark")
    sub.add_parser("selftest")
    a = ap.parse_args()
    if a.cmd == "selftest":
        return selftest()
    if a.cmd == "render":
        rows = [json.loads(l) for l in pathlib.Path(a.jsonl).read_text().splitlines() if l.strip()]
        (pathlib.Path(a.jsonl).parent / "BENCH.md").write_text(render(rows, a.title))
        return 0
    rows, calls = [], []
    for d in a.box_dir + a.muninn_dir:
        r_, c_ = ingest_box(a.run, d)
        rows += r_
        calls += c_
    for d in a.final_dir:
        r_, c_ = ingest_final(a.run, d)
        rows += r_
        calls += c_
    if a.preview:
        rows += ingest_preview(a.run, a.preview, dict(m.split("=", 1) for m in a.map))
    for sm in a.settle:
        st, pc = sm.split("=", 1)
        rows += ingest_settle(a.run, st, *pc.split("/", 1))
    rows = fill_blocked(rows, calls, a.run, a.preview_note, a.upper_note)
    meta = dict(m.split("=", 1) for m in a.meta)
    if a.derived_tx:
        meta["derived_tx"] = a.derived_tx
    write(rows, a.out, a.title, meta)
    print(f"T23BENCH rows {len(rows)} calls {len(calls)} -> {a.out}/bench.jsonl, {a.out}/BENCH.md")
    return 0


if __name__ == "__main__":
    sys.exit(main())
