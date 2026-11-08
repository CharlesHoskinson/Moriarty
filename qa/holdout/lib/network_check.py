"""Read-only SherlockRun rejection check on a selected network (E12; A9p on Preview, A9m on Mainnet).

Evaluator-owned. Never opens a wallet or submits. The evidence is produced by the network's reject
operation (config/networks.json `rejectOperation`, e.g. OP-PV-REJECT: `scripts/doctorwatson heavy --id
OP-PV-REJECT -- node tools/sherlock-sdk/call-run.mjs --network preview --reject-suite`), and its receipt
must be committed at qa/sherlock/receipt-traces/<op>/receipt.json (SWF-005). Every claim the checker can
re-derive is re-derived from the selected network's public indexer; the rest is bound to the measured
heavy run whose trace records it.

Evidence interface (`sherlock-run-rejections/1`), written by the producer:
  {schema, network, genesisHash, submitted: true, finishedAt,
   address, otherProgramAddress, otherInitAddress,          # MAIN, OTHER-PROGRAM, OTHER-INIT deployments
   preActionTx, preStateSHA256, stateAfterMutantsSHA256,    # MAIN's latest action before the session
   attempts: [{name, txSHA256, nodeOutcome: accepted|refused, refusal: [{tag, message}]}],
   prediction: {<attempt>: {applied: bool, stage}},          # validate_call, proof-verifying ledger 9.1.0.1
   flip: {txOffset, mask, proofStart, proofLength}, twinProofSHA256}
  attempt names: flipped-proof, reused-proof, other-program, other-init (all before `twin`), twin (the
  same-session positive control: a fresh valid acceptRun on MAIN), replay (a fresh valid proof of the
  twin's stmt, after the twin).
  The trace of the measured run carries `[reject] output <receipt dir>` and one
  `[reject] <name>: node ACCEPTED|refused <txSHA256>` line per attempt.

Oracle (per case `networkCheck`): REJECT = the node refused the attempt at submission with a counted
ledger refusal (not a pool ban, fee, DUST, TTL or balance failure), its transaction hash is absent from
the indexer, the native proof-verifying predictor refused it at the case's stage, MAIN's state did not
change across the mutants, and the same-session positive control finalized as an acceptRun ContractCall
on the sealed MAIN address. Without the positive control nothing passes.
"""
import json
import re
import subprocess
from pathlib import Path

import networks

EXCLUDED = re.compile(r"1012|temporarily banned|dust|fee|balanc|insufficient|ttl|expir|outdated|stale|future", re.I)
BEFORE_TWIN = ("flipped-proof", "reused-proof", "other-program", "other-init")
SCHEMA = "sherlock-run-rejections/1"


class Fail(Exception):
    pass


def need(cond, msg):
    if not cond:
        raise Fail(msg)


def tracked_clean(repo, rel):
    if not (repo / rel).is_file():
        return False
    ls = subprocess.run(["git", "-C", str(repo), "ls-files", "--error-unmatch", rel], capture_output=True)
    if ls.returncode:
        return False
    return subprocess.run(["git", "-C", str(repo), "diff", "--quiet", "HEAD", "--", rel]).returncode == 0


def receipt_rel(cfg):
    return f"qa/sherlock/receipt-traces/{cfg['rejectOperation']}/receipt.json"


def attempts(rc):
    return {a["name"]: (i, a) for i, a in enumerate(rc.get("attempts") or [])}


def measured_run(repo, rc, rel, op):
    runs = repo / "qa" / "sherlock" / "runs.jsonl"
    lines = [json.loads(l) for l in runs.read_text().splitlines() if l.strip()] if runs.is_file() else []
    want = [f"[reject] output {(repo / rel).parent.resolve()}"] + [
        f"[reject] {a['name']}: node {'ACCEPTED' if a['nodeOutcome'] == 'accepted' else 'refused'} {a['txSHA256']}"
        for a in rc["attempts"]]
    copy = (repo / rel).parent / "trace.log"
    for r in reversed(lines):
        if r.get("id") != op or r.get("exit") != 0 or not r.get("trace"):
            continue
        for t in (repo / r["trace"], copy):
            if t.is_file() and all(w in t.read_text(errors="replace") for w in want):
                return r["run"]
    raise Fail(f"no exit-0 {op} heavy run whose trace records this receipt's attempts")


def control(ix, name, cfg, rc):
    sr = cfg["sherlockRun"]
    need(rc.get("schema") == SCHEMA and rc.get("submitted") is True and rc.get("finishedAt"),
         "receipt is not a completed submission run")
    need(rc.get("network") == name, f"receipt is for network {rc.get('network')!r}, not {name}")
    g = networks.check_genesis(cfg, rc.get("genesisHash"))
    need(g is None, f"receipt {g}")
    need(rc.get("address") == sr["address"], "receipt is not for the sealed SherlockRun address")
    at = attempts(rc)
    need("twin" in at and at["twin"][1]["nodeOutcome"] == "accepted", "positive control twin was not accepted by the node")
    h = at["twin"][1]["txSHA256"]
    t = ix.txs(h)
    need(len(t) == 1 and (t[0].get("transactionResult") or {}).get("status") == "SUCCESS", f"twin {h[:16]} not indexed once with SUCCESS")
    need(ix.sha(bytes.fromhex(t[0]["raw"])) == h, "indexed twin bytes do not hash to the twin hash")
    a = ix.action_at(sr["address"], h)
    need(a and a["__typename"] == "ContractCall" and a["entryPoint"] == "acceptRun", "twin is not an acceptRun ContractCall on MAIN")
    pre = ix.action_at(sr["address"], rc["preActionTx"])
    need(pre and ix.sha(bytes.fromhex(pre["state"])) == rc["preStateSHA256"], "preStateSHA256 is not MAIN's indexed state at preActionTx")
    need(rc.get("stateAfterMutantsSHA256") == rc["preStateSHA256"], "MAIN state changed across the refused attempts")
    need(ix.sha(bytes.fromhex(a["state"])) != rc["preStateSHA256"], "the twin did not change MAIN's state")
    return t[0], at


def refused_unincluded(ix, rc, at, name, stage):
    need(name in at, f"no {name} attempt recorded")
    i, a = at[name]
    if name in BEFORE_TWIN:
        need(i < at["twin"][0], f"{name} must precede the positive control")
    else:
        need(i > at["twin"][0], f"{name} must follow the positive control")
    need(a["nodeOutcome"] == "refused" and a.get("refusal"), f"{name}: node did not refuse ({a['nodeOutcome']})")
    msgs = " | ".join(r.get("message", "") for r in a["refusal"])
    need(not EXCLUDED.search(msgs), f"{name}: refusal is not a counted ledger refusal: {msgs[:200]!r}")
    p = (rc.get("prediction") or {}).get(name) or {}
    need(p.get("applied") is False and p.get("stage") == stage, f"{name}: native predictor {p!r}, want refusal at {stage!r}")
    need(ix.txs(a["txSHA256"]) == [], f"{name}: transaction {a['txSHA256'][:16]} is indexed (included)")
    return a


def check_flip(ix, rc, at, twin_tx, a, case):
    f = rc["flip"]
    need(f["mask"] in (1, 2, 4, 8, 16, 32, 64, 128), "flipped-proof: mutation is not a single bit")
    need(f["proofStart"] <= f["txOffset"] < f["proofStart"] + f["proofLength"], "flipped-proof: flipped byte is outside the contract proof")
    raw = bytearray(bytes.fromhex(twin_tx["raw"]))
    need(ix.sha(bytes(raw[f["proofStart"]:f["proofStart"] + f["proofLength"]])) == rc["twinProofSHA256"],
         "flipped-proof: recorded proof region is not the twin's contract proof")
    raw[f["txOffset"]] ^= f["mask"]
    need(ix.sha(bytes(raw)) == a["txSHA256"], "flipped-proof: refused bytes are not the accepted twin with one proof bit flipped")
    return f"one-bit flip at tx offset {f['txOffset']}"


def check_other(ix, rc, cfg, key):
    addr = cfg["sherlockRun"][key]
    need(rc.get(key) == addr, f"receipt {key} is not the sealed deployment")
    need(addr != cfg["sherlockRun"]["address"], f"{key} is MAIN")
    last = ix.latest(addr)
    need(last and last["__typename"] == "ContractDeploy", f"{key} has a contract action after its deployment")
    return f"{key} {addr[:16]} untouched since deployment"


def unimplemented_reason(name, cfg, case):
    sr = cfg["sherlockRun"]
    missing = [k for k in ["address"] + case["networkCheck"].get("needsAddresses", []) if not sr.get(k)]
    if missing:
        return (f"UNIMPLEMENTED: the evaluator has not sealed the {name} SherlockRun {', '.join(missing)} in "
                f"qa/holdout/config/networks.json (after the deploy operation and its A4-shape check)")
    return None


def run(name, cfg, cases, repo, ix=None):
    """Returns (pass, fail, unimplemented) lists of (id, message)."""
    res = ([], [], [])
    ix = ix or networks.Indexer(name, cfg)
    ready = []
    for c in cases:
        why = unimplemented_reason(name, cfg, c)
        (res[2].append((c["id"], why)) if why else ready.append(c))
    if not ready:
        return res
    rel = receipt_rel(cfg)
    if not tracked_clean(repo, rel):
        for c in ready:
            res[2].append((c["id"], f"UNIMPLEMENTED: needs the committed {cfg['rejectOperation']} receipt {rel} "
                                    f"(`scripts/doctorwatson heavy --id {cfg['rejectOperation']} -- node tools/sherlock-sdk/"
                                    f"call-run.mjs --network {name} --reject-suite`; owner-authorized operation)"))
        return res
    rc = json.loads((repo / rel).read_text())
    print(f"evidence: {rel}")
    try:
        twin_tx, at = control(ix, name, cfg, rc)
        print(f"measured run: {measured_run(repo, rc, rel, cfg['rejectOperation'])}")
    except (Fail, RuntimeError, KeyError, TypeError, ValueError) as e:
        for c in ready:
            res[1].append((c["id"], f"positive control failed, harness cannot be trusted to reject: {e}"))
        return res
    for c in ready:
        nc = c["networkCheck"]
        try:
            a = refused_unincluded(ix, rc, at, nc["attempt"], nc["stage"])
            extra = []
            if nc["attempt"] == "flipped-proof":
                extra.append(check_flip(ix, rc, at, twin_tx, a, c))
            for key in nc.get("needsAddresses", []):
                extra.append(check_other(ix, rc, cfg, key))
            res[0].append((c["id"], f"{nc['attempt']} {a['txSHA256'][:16]} refused at submission, not indexed, predictor "
                                    f"refused at {nc['stage']}; " + "; ".join(extra + [f"twin {at['twin'][1]['txSHA256'][:16]} finalized"])))
        except (Fail, RuntimeError, KeyError, TypeError, ValueError) as e:
            res[1].append((c["id"], f"{type(e).__name__}: {e}"))
    return res
