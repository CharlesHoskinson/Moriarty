"""Read-only A9 check: stagenet rejection evidence against the public indexer.

Evidence is produced only by `qa/holdout/lib/stagenet/produce-a9.sh` under the heavy slot. This module
never opens a wallet or submits; it re-derives every claim it can from the indexer and the receipt.

Oracle (per case `stagenetCheck`):
  REJECT  = the node refused the attempt at submission, the attempt's transaction hash is absent from
            the indexer (no inclusion, so no effect), the native proof-verifying predictor agreed on the
            refusal class, and the same-session positive control finalized afterwards
            (or, for accept-level replay, the call finalized with an indexed post-state equal to its pre-state).
  The positive control is mandatory: without an accepted twin from the same harness, nothing passes.
"""
import hashlib
import json
import os
import urllib.request
from pathlib import Path

INDEXER = "https://indexer.stagenet.shielded.tools/api/v4/graphql"
PIN = "ac93b80e252f9cde794d22f76b2eb7d727f855f07ad77a0904afe5bac855388b"
INSTALLED_ACCEPT_VK = "10b2d9cfe7fd16df2616da4616f69c4f34f668e59f229e5855a1f6572fd46100"
COMPILED_ACCEPT_VK = "f2d10277b0e853e37e448e1a57c97ab6d3edcf6f5d15cab8673c8708136796f4"
A8_TX = "a2b71fa213437f7c8470f60be871eed20ff1c78209ee5dcf29a91198f8e3af07"
A8_STMT = "deffb4974d1eb0b62b1ffb6d709c439e142efd51a2bcc94405fa864186a5c25a"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def gql(query):
    req = urllib.request.Request(INDEXER, json.dumps({"query": query}).encode(), {"content-type": "application/json"})
    with urllib.request.urlopen(req, timeout=30) as r:
        body = json.load(r)
    if body.get("errors"):
        raise RuntimeError("indexer error: %r" % (body["errors"],))
    return body["data"]


def txs(h):
    return gql('{transactions(offset:{hash:"%s"}){hash raw block{height} ... on RegularTransaction{transactionResult{status}}}}' % h)["transactions"]


def action(h):
    return gql('{contractAction(address:"%s",offset:{transactionOffset:{hash:"%s"}}){__typename state '
               'transaction{hash} ... on ContractCall{entryPoint}}}' % (PIN, h))["contractAction"]


def find_evidence(repo):
    env = os.environ.get("DW_A9_EVIDENCE_DIR")
    cands = [Path(env) / "receipt.json"] if env else list((repo / "target" / "sherlock-a9").glob("run-*/receipt.json"))
    cands = [c for c in cands if c.is_file()]
    if not cands:
        return None, None
    path = max(cands, key=lambda p: p.stat().st_mtime_ns)
    return path, json.loads(path.read_text())


class Fail(Exception):
    pass


def need(cond, msg):
    if not cond:
        raise Fail(msg)


def proof_refusal(pred):
    return (pred is not None and pred.get("contractProofsSignaturesLimitsAccepted") is False
            and "failed to verify proof" in (pred.get("contractProfileFailure") or ""))


def attempts(rc):
    return {a["name"]: (i, a) for i, a in enumerate(rc.get("attempts") or [])}


def measured_run(repo, path, rc, run_id="A9produce"):
    """A node refusal leaves no public trace; bind each claimed outcome to a measured heavy run whose trace has it."""
    runs = repo / "qa" / "sherlock" / "runs.jsonl"
    lines = [json.loads(l) for l in runs.read_text().splitlines() if l.strip()] if runs.is_file() else []
    want = [f"[a9] output {path.parent.resolve()}"] + [
        f"[a9] {a['name']}: node {'ACCEPTED' if a['nodeOutcome'] == 'accepted' else 'refused'} {a['txSHA256']}" for a in rc["attempts"]]
    for r in reversed(lines):
        if r.get("id") != run_id or r.get("exit") != 0 or not r.get("trace"):
            continue
        t = repo / r["trace"]
        if t.is_file() and all(w in t.read_text(errors="replace") for w in want):
            return r["run"]
    raise Fail(f"no exit-0 {run_id} heavy run whose trace records this receipt's attempts")


def control(rc):
    """Positive control: the same harness, in the same session, got a valid call finalized."""
    need(rc.get("schema") == "sherlock-a9-stagenet-rejections/1" and rc.get("submitted") is True and rc.get("finishedAt"),
         "receipt is not a completed submission run")
    need(rc.get("address") == PIN, "receipt is not for the pinned verifier contract")
    at = attempts(rc)
    need("twin" in at and at["twin"][1]["nodeOutcome"] == "accepted", "positive control twin was not accepted by the node")
    for name in ("s-a", "s-b"):
        need(name in at and at[name][0] < at["twin"][0], f"{name} attempt must precede the positive control")
    h = at["twin"][1]["txSHA256"]
    t = txs(h)
    need(len(t) == 1 and (t[0].get("transactionResult") or {}).get("status") == "SUCCESS", f"twin {h[:16]} not indexed once with SUCCESS")
    need(sha(bytes.fromhex(t[0]["raw"])) == h, "indexed twin bytes do not hash to the twin hash")
    a = action(h)
    need(a and a["__typename"] == "ContractCall" and a["entryPoint"] == "acceptValidity", "twin is not an acceptValidity ContractCall on the pin")
    a8 = txs(A8_TX)
    need(len(a8) == 1 and a8[0]["transactionResult"]["status"] == "SUCCESS", "A8 acceptance is not indexed with SUCCESS")
    return t[0], a


def refused_unincluded(rc, name):
    at = attempts(rc)
    need(name in at, f"no {name} attempt recorded")
    a = at[name][1]
    need(a["nodeOutcome"] == "refused" and a.get("refusal"), f"{name}: node did not refuse ({a['nodeOutcome']})")
    need(txs(a["txSHA256"]) == [], f"{name}: transaction {a['txSHA256'][:16]} is indexed (included)")
    return a


def check_s_a(rc, twin_tx, _, case):
    a = refused_unincluded(rc, "s-a")
    need(proof_refusal((rc.get("prediction") or {}).get("sA")), "s-a: native predictor did not refuse on the proof")
    f = rc["flip"]
    need((f["proofOffset"], f["mask"]) == (case["mutation"]["proofOffset"], case["mutation"]["mask"]), "s-a: receipt flip differs from the case")
    need(f["mask"] in (1, 2, 4, 8, 16, 32, 64, 128), "s-a: mutation is not a single bit")
    need(f["proofStart"] <= f["txOffset"] < f["proofStart"] + f["proofLength"], "s-a: flipped byte is outside the contract proof")
    raw = bytearray(bytes.fromhex(twin_tx["raw"]))
    need(sha(bytes(raw[f["proofStart"]:f["proofStart"] + f["proofLength"]])) == rc["twinProofSHA256"],
         "s-a: recorded proof region is not the twin's contract proof")
    raw[f["txOffset"]] ^= f["mask"]
    need(sha(bytes(raw)) == a["txSHA256"], "s-a: refused bytes are not the accepted twin with one proof bit flipped")
    need(rc.get("stateAfterMutantsSHA256") == rc["preStateSHA256"], "s-a: contract state changed across the refused attempts")
    return f"node refused one-bit proof flip (tx offset {f['txOffset']}); twin with identical other bytes finalized; " \
           f"refusal={a['refusal'][0]['tag']}: {a['refusal'][-1]['message'][:120]!r}"


def check_s_b(rc, _t, _a, case):
    a = refused_unincluded(rc, "s-b")
    p = (rc.get("prediction") or {}).get("sB")
    need(proof_refusal(p), "s-b: native predictor did not refuse on the proof")
    need(p["calls"] and p["calls"][0]["address"] == PIN and p["calls"][0]["entryPoint"] == "acceptValidity", "s-b: not an acceptValidity call on the pin")
    need(rc["stmtBTag"] == case["mutation"]["stmtTag"], "s-b: statement tag differs from the case")
    need(rc["installedAcceptVkSHA256"] == INSTALLED_ACCEPT_VK, "s-b: installed key is not the A4 decider key")
    need(rc["compiledAcceptVkSHA256"] == COMPILED_ACCEPT_VK != INSTALLED_ACCEPT_VK, "s-b: proving key is not the compiled acceptValidity key")
    need(rc.get("stateAfterMutantsSHA256") == rc["preStateSHA256"], "s-b: contract state changed across the refused attempts")
    return f"node refused a compiled-key acceptValidity proof for fresh stmt {rc['stmtB'][:16]}; not indexed; " \
           f"refusal={a['refusal'][0]['tag']}: {a['refusal'][-1]['message'][:120]!r}"


BAN = "1012: Transaction is temporarily banned"


def check_s_e(rc, twin_tx, twin_action, case, evidence_dir=None, repo=None):
    need(rc["stmt"] == A8_STMT, "s-e: twin statement is not the A8-accepted statement")
    a8 = action(A8_TX)
    need(sha(bytes.fromhex(a8["state"])) == rc["preStateSHA256"], "s-e: pinned pre-state is not the indexed A8 post-state")
    need(sha(bytes.fromhex(twin_action["state"])) == rc["preStateSHA256"], "s-e: second acceptance of the statement changed indexed state")
    msg = f"(i) re-acceptance of the A8 stmt finalized with indexed post-state == pre-state ({rc['preStateSHA256'][:16]})"
    at = attempts(rc)
    need("s-e-identical-bytes" in at, "no identical-bytes resubmission recorded")
    i, a = at["s-e-identical-bytes"]
    need(i > at["twin"][0] and a["txSHA256"] == at["twin"][1]["txSHA256"], "resubmission is not the twin's exact bytes after acceptance")
    need(a["nodeOutcome"] == "refused", "identical-bytes resubmission was not refused")
    if not any(BAN in r["message"] for r in a["refusal"]):
        need(len(txs(a["txSHA256"])) == 1, "twin bytes are indexed more than once")
        return msg + f"; (ii) identical bytes refused ({a['refusal'][-1]['message'][:80]!r}) and indexed once", None
    # The pool's temporary ban is not the ledger's replay guard; require a resubmission after the ban window.
    rs = (evidence_dir / "resubmit.json") if evidence_dir else None
    if not rs or not rs.is_file():
        return msg, ("UNIMPLEMENTED: s-e (ii) the identical-bytes resubmission was refused only by the node pool's temporary ban; "
                     "needs `DW_ROLE=evaluator scripts/doctorwatson heavy --id A9resubmit -- qa/holdout/lib/stagenet/produce-a9.sh --resubmit "
                     f"{evidence_dir}` at least 35 min after the twin")
    r = json.loads(rs.read_text()); b = r["attempt"]
    need(r["twinSHA256"] == a["txSHA256"] == b["txSHA256"] and r["minutesAfterTwin"] >= 35, "resubmit.json is not a post-ban resubmission of the twin")
    need(b["nodeOutcome"] == "refused" and not any(BAN in x["message"] for x in b["refusal"]), "post-ban identical bytes were not refused by the ledger")
    measured_run(repo, rs, {"attempts": [b]}, run_id="A9resubmit")
    need(len(txs(b["txSHA256"])) == 1, "twin bytes are indexed more than once")
    return msg + f"; (ii) identical bytes refused after the ban window ({b['refusal'][-1]['message'][:80]!r}) and indexed once", None


CHECKS = {"proof-bit-flip": check_s_a, "wrong-verifying-key": check_s_b, "statement-replay": check_s_e}


def run(cases, repo):
    """Returns (pass, fail, unimplemented) lists of (id, message)."""
    res = ([], [], [])
    path, rc = find_evidence(repo)
    implemented = [c for c in cases if c.get("stagenetCheck", {}).get("kind") in CHECKS]
    for c in cases:
        if c not in implemented:
            res[2].append((c["id"], c["unimplemented"]))
    if not implemented:
        return res
    if rc is None:
        for c in implemented:
            res[2].append((c["id"], "UNIMPLEMENTED: needs A9 evidence: run `DW_ROLE=evaluator scripts/doctorwatson heavy --id A9produce -- "
                                    "qa/holdout/lib/stagenet/produce-a9.sh` (opens the stagenet wallet)"))
        return res
    print(f"evidence: {path}")
    try:
        twin_tx, twin_action = control(rc)
        print(f"measured run: {measured_run(repo, path, rc)}")
    except (Fail, RuntimeError, OSError, KeyError) as e:
        for c in implemented:
            res[1].append((c["id"], f"positive control failed, harness cannot be trusted to reject: {e}"))
        return res
    for c in implemented:
        try:
            if c["stagenetCheck"]["kind"] == "statement-replay":
                msg, pending = check_s_e(rc, twin_tx, twin_action, c, path.parent, repo)
            else:
                msg, pending = CHECKS[c["stagenetCheck"]["kind"]](rc, twin_tx, twin_action, c), None
        except (Fail, RuntimeError, OSError, KeyError, TypeError) as e:
            res[1].append((c["id"], f"{type(e).__name__}: {e}"))
            continue
        rest = " ; ".join(x for x in (pending, c.get("unimplemented")) if x)
        if rest:   # implemented sub-checks passed; the case still needs the remaining evidence or harness
            res[2].append((c["id"], rest))
            print(f"PARTIAL {c['id']}: implemented sub-checks pass: {msg}")
        else:
            res[0].append((c["id"], msg))
    return res
