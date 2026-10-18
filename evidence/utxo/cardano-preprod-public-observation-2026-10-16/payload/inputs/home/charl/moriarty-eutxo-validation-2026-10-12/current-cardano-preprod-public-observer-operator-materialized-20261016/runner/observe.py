#!/usr/bin/env python3
"""One-shot public Cardano Preprod observer via installed Scrapling (R2).

Exactly five sequential unauthenticated HTTPS calls, one attempt each:
  1 GET  /genesis
  2 GET  /tip
  3 POST /address_utxos {_addresses:[ADDR],_extended:true}
  4 GET  /epoch_params?_epoch_no=<epoch_no from step 2>
  5 GET  /tip
POST is a provider query, not a ledger mutation. No explicit auth, proxy
or credentials are passed in code. Every get/post passes
discard_cookies=True (forwarded per-request by Scrapling to curl_cffi):
a Set-Cookie may still arrive in the ephemeral response object, but it
is not stored in the persistent cookie jar and is not reused on later
calls. This source cannot disable trust_env at the FetcherSession
constructor (no such supported argument); freedom from proxy/CA
environment effects is NOT a code guarantee and requires the root exact
clean environment dict in EXECUTION-PROPOSAL.json. No faucet, submit,
signing, wallet keys, paging, retry or redirect.
Honest limits: Scrapling/curl_cffi buffers the whole body before this
code can measure it; the 2 MiB check is a post-buffer persist bound, not
a streaming/download cap, and controller-level five calls do not prove
an absolute DNS/wire request count inside curl_cffi. Partial RESULT
preservation is best effort: a physical write failure can prevent the
final receipt, in which case the root outer captured terminal output
is the authority, not this file.
"""
import argparse, hashlib, json, math, re, sys
from datetime import datetime, timezone
from pathlib import Path

BASE = "https://preprod.koios.rest/api/v1"
ADDR = "addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc"
MAX_BODY = 2 * 1024 * 1024
MAX_RESULT = 2 * 1024 * 1024  # receipt guard, well under 8 MiB per-file FSIZE
TIMEOUT = 15
HEX64 = re.compile(r"^[0-9a-f]{64}$")
DEC = re.compile(r"^(0|[1-9][0-9]*)$")
CR_RE = re.compile(r"^\s*(?:(\d+)-(\d+)|\*)\s*/\s*(\d+|\*)\s*$")
POST_BODY = json.dumps({"_addresses": [ADDR], "_extended": True},
                       separators=(",", ":"), allow_nan=False).encode()
# Frozen from public Preprod Shelley genesis capture (provider contract check
# only, never ledger authentication): magic 1/Testnet, 2022-06-01 start.
GENESIS_INT_STR = {"networkmagic": "1", "epochlength": "432000",
    "slotlength": "1", "maxlovelacesupply": "45000000000000000",
    "slotsperkesperiod": "129600", "maxkesrevolutions": "62",
    "securityparam": "2160", "updatequorum": "5"}
SYSTEM_START = 1654041600  # 2022-06-01T00:00:00Z
EPOCH_LEN = 432000
PARAM_INT_FIELDS = ("max_tx_size", "max_block_size", "max_bh_size",
    "min_fee_a", "min_fee_b", "max_tx_ex_mem", "max_tx_ex_steps",
    "max_block_ex_mem", "max_block_ex_steps")

class Abort(Exception):
    pass

def utc():
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")

def is_int(x):
    return type(x) is int  # bool is not int here

def parse_strict(raw):
    def hook(pairs):
        out = {}
        for k, v in pairs:
            if k in out:
                raise Abort(f"duplicate JSON key: {k}")
            out[k] = v
        return out
    def bad_const(x):
        raise Abort(f"non-JSON constant: {x}")
    def finite_float(s):
        try:
            v = float(s)
        except Exception:
            raise Abort(f"invalid JSON number: {s}")
        if not math.isfinite(v):  # rejects 1e999 overflow and inf/nan forms
            raise Abort(f"non-finite JSON number: {s}")
        return v
    try:
        return json.loads(raw.decode("utf-8"), object_pairs_hook=hook,
                          parse_constant=bad_const, parse_float=finite_float)
    except Abort:
        raise
    except Exception as e:
        raise Abort(f"strict JSON parse failed: {type(e).__name__}") from e

def one_array(data, what):
    if not isinstance(data, list) or len(data) != 1 or not isinstance(data[0], dict):
        raise Abort(f"{what}: expected array of exactly one object")
    return data[0]

def req_int(obj, field, what, lo=0, hi=None):
    if field not in obj:
        raise Abort(f"{what}: missing {field}")
    v = obj[field]
    if not is_int(v) or v < lo or (hi is not None and v > hi):
        raise Abort(f"{what}: {field} must be non-bool integer in range")
    return v

def check_genesis(data):
    o = one_array(data, "genesis")
    for f, want in GENESIS_INT_STR.items():
        v = o.get(f)
        # Documented contract: these are strings; coerce only such strings.
        if not isinstance(v, str) or not DEC.match(v) or v != want:
            raise Abort(f"genesis: {f} must be documented string {want!r}")
    if o.get("networkid") != "Testnet":
        raise Abort("genesis: networkid must be exactly 'Testnet'")
    if req_int(o, "systemstart", "genesis", 0) != SYSTEM_START:
        raise Abort("genesis: systemstart != frozen Preprod start")
    if not isinstance(o.get("activeslotcoeff"), str):
        raise Abort("genesis: activeslotcoeff must be documented string")
    if not isinstance(o.get("alonzogenesis"), str):
        raise Abort("genesis: alonzogenesis must be opaque string")
    return o

def check_tip(data, what):
    o = one_array(data, what)
    if not isinstance(o.get("hash"), str) or not HEX64.match(o["hash"]):
        raise Abort(f"{what}: hash must be 64 lowercase hex chars")
    if not isinstance(o.get("era"), str) or not o["era"]:
        raise Abort(f"{what}: era must be non-empty string")
    req_int(o, "epoch_no", what); req_int(o, "abs_slot", what)
    es = req_int(o, "epoch_slot", what)
    req_int(o, "block_time", what, SYSTEM_START)
    if es >= EPOCH_LEN:
        raise Abort(f"{what}: epoch_slot >= frozen epoch length")
    if "block_no" in o and not is_int(o["block_no"]):
        raise Abort(f"{what}: deprecated block_no must be non-bool int if present")
    return o

def check_params(data, chosen, flags):
    o = one_array(data, "epoch_params")
    if req_int(o, "epoch_no", "epoch_params") != chosen:
        raise Abort("epoch_params: epoch_no != explicitly chosen epoch")
    if not isinstance(o.get("block_hash"), str) or not HEX64.match(o["block_hash"]):
        raise Abort("epoch_params: block_hash must be 64 lowercase hex chars")
    # Raw-observed budgets: known numeric shape, nullable stays explicit.
    for f in PARAM_INT_FIELDS:
        if f not in o:
            flags.append(f"params_missing:{f}")
        elif o[f] is None:
            flags.append(f"params_null:{f}")
        elif not is_int(o[f]) or o[f] < 0:
            raise Abort(f"epoch_params: {f} must be null or non-bool int >= 0")
    cm = o.get("cost_models", None)
    if cm is None:
        flags.append("cost_models_null:format_unknown_cannot_cost_evaluate")
    elif isinstance(cm, dict):
        flags.append("cost_models_opaque_object:format_unknown_cannot_cost_evaluate")
    else:
        flags.append("cost_models_unsupported_shape:cannot_cost_evaluate")
    return o

def utxo_completeness(content_range, n, flags):
    """True only when Content-Range exactly covers all n returned rows.

    PostgREST forms handled: 0-2/3 (complete iff start=0, end=total-1,
    n=total), */0 (complete empty iff n=0), 0-2/* and */* (total unknown:
    bounded/incomplete), absent or malformed (completeness unknown).
    """
    if content_range is None:
        flags.append("utxo_content_range_absent:completeness_unknown_bounded_result")
        return False
    m = CR_RE.match(str(content_range))
    if not m:
        flags.append("utxo_content_range_malformed:completeness_unknown_bounded_result")
        return False
    start_s, end_s, total_s = m.groups()
    if total_s == "*":
        flags.append("utxo_content_range_unknown_total:bounded_incomplete_or_unknown")
        return False
    total = int(total_s)
    if start_s is None:  # */total: no row range was returned
        if total == 0 and n == 0:
            flags.append("utxo_content_range_complete_empty:total_zero")
            return True
        flags.append("utxo_content_range_no_row_range:bounded_incomplete_or_unknown")
        return False
    start, end = int(start_s), int(end_s)
    if end < start:
        flags.append("utxo_content_range_malformed:completeness_unknown_bounded_result")
        return False
    if n != (end - start + 1):
        flags.append("utxo_content_range_row_count_mismatch:bounded_incomplete_or_unknown")
        return False
    if start == 0 and end == total - 1 and n == total:
        flags.append("utxo_content_range_complete:range_covers_all_rows")
        return True
    flags.append("utxo_content_range_partial_range:bounded_incomplete")
    return False

def check_utxos(data, flags, content_range=None):
    if not isinstance(data, list):
        raise Abort("address_utxos: expected top-level array")
    if len(data) > 1000:
        raise Abort("address_utxos: more than documented 1000 rows")
    if len(data) == 1000:
        flags.append("utxo_rows_exactly_1000:bounded_set_only_possible_truncation")
    seen = set()
    for i, r in enumerate(data):
        if not isinstance(r, dict):
            raise Abort(f"utxo[{i}]: expected object")
        if r.get("address") != ADDR:
            raise Abort(f"utxo[{i}]: address != exact requested address")
        if not isinstance(r.get("tx_hash"), str) or not HEX64.match(r["tx_hash"]):
            raise Abort(f"utxo[{i}]: tx_hash must be 64 lowercase hex chars")
        # SQL source declares tx_index smallint: reject negative, cap 32767.
        ti = req_int(r, "tx_index", f"utxo[{i}]", 0, 32767)
        key = (r["tx_hash"], ti)
        if key in seen:
            raise Abort(f"utxo[{i}]: duplicate outref")
        seen.add(key)
        if r.get("is_spent") is not False:
            raise Abort(f"utxo[{i}]: is_spent must be exactly false")
        v = r.get("value")
        if not isinstance(v, str) or not DEC.match(v):
            raise Abort(f"utxo[{i}]: value must be raw non-negative decimal string")
        # asset_list / inline_datum / reference_script: retained raw only.
        if "asset_list" in r and r["asset_list"] is not None \
                and not isinstance(r["asset_list"], list):
            raise Abort(f"utxo[{i}]: asset_list must be array or null")
        for f in ("inline_datum", "reference_script"):
            if f in r and r[f] is not None and not isinstance(r[f], dict):
                raise Abort(f"utxo[{i}]: {f} must be object or null")
    complete = utxo_completeness(content_range, len(data), flags)
    return data, complete

def _raw_file(ledger, idx):
    for e in ledger:
        if e.get("step") == idx:
            return e.get("raw_file")
    return None

def _content_range(ledger, idx):
    for e in ledger:
        if e.get("step") == idx:
            return e.get("headers_allowlisted", {}).get("content-range")
    return None

def summarize_observation(state, ledger):
    """Whitelisted receipt summary only; full bodies stay as raw files."""
    obs = {}
    if "genesis" in state:
        g = state["genesis"]
        obs["genesis"] = {k: g.get(k) for k in
            ("networkmagic", "networkid", "epochlength", "slotlength",
             "systemstart", "slotsperkesperiod", "maxkesrevolutions",
             "securityparam", "updatequorum", "maxlovelacesupply") if k in g}
        obs["genesis"]["raw_file"] = _raw_file(ledger, 1)
    for key, idx in (("tip_before", 2), ("tip_after", 5)):
        if key in state:
            t = state[key]
            obs[key] = {k: t.get(k) for k in
                ("hash", "era", "epoch_no", "abs_slot", "epoch_slot",
                 "block_time", "block_no") if k in t}
            obs[key]["raw_file"] = _raw_file(ledger, idx)
    if "chosen_epoch" in state:
        obs["chosen_epoch"] = state["chosen_epoch"]
    if "utxo_count_bounded" in state:
        obs["address_utxos"] = {
            "row_count_bounded": state["utxo_count_bounded"],
            "completeness_proven": bool(state.get("utxo_completeness_proven")),
            "content_range": _content_range(ledger, 3),
            "raw_file": _raw_file(ledger, 3)}
    if "params" in state:
        p = state["params"]
        summ = {"epoch_no": p.get("epoch_no"), "block_hash": p.get("block_hash"),
                "raw_file": _raw_file(ledger, 4)}
        for f in PARAM_INT_FIELDS:
            if f in p:
                summ[f] = p[f]  # validated above: null or non-bool int >= 0
        cm = p.get("cost_models", None)
        summ["cost_models"] = {
            "shape": ("missing" if "cost_models" not in p else
                      "null" if cm is None else
                      "object" if isinstance(cm, dict) else type(cm).__name__),
            "format": "unknown_cannot_cost_evaluate",
            "raw_file": _raw_file(ledger, 4)}
        obs["epoch_params"] = summ
    return obs

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--output", required=True,
                    help="fresh output directory; must not exist")
    out = Path(ap.parse_args().output)
    if out.exists():
        print(f"BLOCKED: output already exists: {out}", file=sys.stderr)
        return 2
    try:
        import logging
        logging.disable(logging.CRITICAL)
        try:  # presence-checked only; no invented Scrapling logger API
            from scrapling.core.utils import log as _log
            if hasattr(_log, "disable"):
                _log.disable("scrapling")
            if hasattr(_log, "remove"):
                _log.remove()
        except Exception:
            pass
        from scrapling.fetchers.requests import FetcherSession
    except Exception as e:
        print(f"BLOCKED: installed Scrapling unavailable: {type(e).__name__}",
              file=sys.stderr)
        return 2
    out.mkdir(parents=True)
    (out / "steps").mkdir()
    ledger, flags, state = [], [], {}
    result = {"format": "cardano-preprod-public-observation/1",
              "status": "STARTED", "provider": BASE, "address": ADDR,
              "started_utc": utc(), "fund_status": "unknown",
              "ledger_authenticated": False, "steps": ledger, "flags": flags}
    def save():
        result["updated_utc"] = utc()
        text = json.dumps(result, indent=2, sort_keys=True,
                          allow_nan=False) + "\n"
        if len(text.encode("utf-8")) > MAX_RESULT:
            # Bounded compact fallback: raw bodies remain exact step files.
            fallback = {k: result.get(k) for k in
                ("format", "status", "provider", "address", "started_utc",
                 "updated_utc", "fund_status", "ledger_authenticated",
                 "blocked_boundary", "flags", "steps")}
            fallback["observation"] = {
                "receipt_bounded_fallback": True,
                "reason": "whitelisted_summary_exceeded_receipt_bound",
                "raw_files_preserved_as_separate_exact_files":
                    [e.get("raw_file") for e in ledger]}
            text = json.dumps(fallback, indent=2, sort_keys=True,
                              allow_nan=False) + "\n"
        (out / "RESULT.json").write_text(text)
    def do_step(idx, name, method, url, body=None):
        entry = {"step": idx, "name": name, "method": method, "url": url,
                 "attempts_controller": 1, "retries_arg": 1,
                 "transport_attempted": False, "response_received": False,
                 "request_body_sha256": hashlib.sha256(body).hexdigest()
                 if body else None, "requested_utc": utc(), "status": None,
                 "response_url": None, "size": None, "sha256": None,
                 "headers_allowlisted": {}, "raw_file": None, "error": None}
        ledger.append(entry)
        save()  # persist the in-flight step before any request is attempted
        try:
            kw = dict(timeout=TIMEOUT, retries=1, retry_delay=0,
                      follow_redirects=False, max_redirects=0, verify=True,
                      stealthy_headers=False, impersonate=None,
                      discard_cookies=True,
                      headers={"Accept": "application/json"})
            entry["transport_attempted"] = True
            if method == "GET":
                resp = session.get(url, **kw)
            else:
                kw["headers"] = {"Accept": "application/json",
                                 "Content-Type": "application/json"}
                resp = session.post(url, data=body, **kw)
            entry["response_received"] = True
        except Exception as e:  # never dump request env/headers/cookies
            entry["error"] = f"{type(e).__name__}"
            save()
            raise Abort(f"step{idx} {name}: transport failure {type(e).__name__}")
        raw = resp.body if isinstance(resp.body, (bytes, bytearray)) else bytes(resp.body)
        entry["status"] = int(resp.status)
        entry["response_url"] = str(getattr(resp, "url", url))
        entry["size"] = len(raw)
        entry["sha256"] = hashlib.sha256(raw).hexdigest()
        hdrs = {str(k).lower(): str(v) for k, v in dict(resp.headers).items()}
        entry["headers_allowlisted"] = {
            k: hdrs[k] for k in ("content-type", "content-range") if k in hdrs}
        if getattr(resp, "history", None):
            entry["error"] = "redirect_history_present"
            save(); raise Abort(f"step{idx} {name}: redirect history present")
        if len(raw) > MAX_BODY:  # post-buffer bound; do not persist body
            entry["error"] = "body_over_2MiB_not_persisted"
            save(); raise Abort(f"step{idx} {name}: body over 2MiB")
        rawname = f"steps/{idx:02d}-{name}.bin"
        (out / rawname).write_bytes(raw)  # raw step appended before validation
        entry["raw_file"] = rawname
        save()
        if entry["status"] != 200:
            raise Abort(f"step{idx} {name}: HTTP {entry['status']} (captured, not accepted)")
        if entry["response_url"] != url:
            raise Abort(f"step{idx} {name}: canonical URL differs from requested URL")
        ct = entry["headers_allowlisted"].get("content-type", "").split(";")[0].strip().lower()
        if ct != "application/json":
            raise Abort(f"step{idx} {name}: Content-Type not application/json")
        return parse_strict(raw)
    status, code = "COMPLETE_PROVIDER_OBSERVATION", 0
    try:
        save()  # initial STARTED receipt, before any session or request
        with FetcherSession(impersonate=None, stealthy_headers=False,
                retries=1, retry_delay=0, follow_redirects=False,
                max_redirects=0, verify=True, timeout=TIMEOUT,
                headers={"Accept": "application/json"}) as session:
            state["genesis"] = check_genesis(do_step(1, "genesis", "GET", f"{BASE}/genesis"))
            tip1 = check_tip(do_step(2, "tip_before", "GET", f"{BASE}/tip"), "tip_before")
            state["tip_before"] = tip1
            chosen = tip1["epoch_no"]  # chosen before step 4, from step-2 tip
            state["chosen_epoch"] = chosen
            rows, utxo_complete = check_utxos(do_step(3, "address_utxos", "POST",
                                       f"{BASE}/address_utxos", POST_BODY), flags,
                                       _content_range(ledger, 3))
            state["utxo_count_bounded"] = len(rows)
            state["utxo_completeness_proven"] = utxo_complete
            state["params"] = check_params(do_step(4, "epoch_params", "GET",
                f"{BASE}/epoch_params?_epoch_no={chosen}"), chosen, flags)
            tip2 = check_tip(do_step(5, "tip_after", "GET", f"{BASE}/tip"), "tip_after")
            state["tip_after"] = tip2
        if tip1 == tip2:
            flags.append("tip_bracket_same:provider_bracket_only_not_atomic_snapshot")
        else:
            flags.append("tip_bracket_changed:AMBIGUOUS_not_atomic_snapshot")
            status = "COMPLETE_PROVIDER_OBSERVATION_AMBIGUOUS_BRACKET"
        # Provider-report language only; never verified balance or ADA sum.
        if state["utxo_count_bounded"]:
            result["fund_status"] = "provider_reports_nonempty_unspent_utxo"
        elif state.get("utxo_completeness_proven"):
            result["fund_status"] = "provider_reports_no_unspent_utxo_valid_unfunded_observation"
        else:
            result["fund_status"] = "provider_reports_empty_bounded_result_completeness_unknown"
    except Abort as e:
        status, code = "BLOCKED_PARTIAL_PRESERVED", 3
        result["blocked_boundary"] = str(e)
        result["fund_status"] = "unknown"
    except Exception as e:  # session-enter and any other non-Abort fault
        status, code = "BLOCKED_PARTIAL_PRESERVED", 3
        result["blocked_boundary"] = f"unexpected_exception:{type(e).__name__}"
        result["fund_status"] = "unknown"
    result["status"] = status
    result["observation"] = summarize_observation(state, ledger)
    result["finished_utc"] = utc()
    try:
        save()
    except Exception:
        pass  # write failure: root outer captured terminal is the authority
    print(json.dumps({"status": status, "output": str(out)}, allow_nan=False))
    return code

if __name__ == "__main__":
    raise SystemExit(main())
