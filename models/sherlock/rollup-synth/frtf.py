"""FRTF v3 canonical bytes and hash-free replay, using only Python's stdlib.

Layout and checks follow fenrir-rollup-envelope's envelope.rs. ``encode_view``
maps the public rollup-test-format generator's JSON views; it never follows a
path from a view. ``decode`` checks binary form; ``validate`` additionally
checks step links, positions, header closes and profile-B reference association.

This module does not recompute Poseidon state/record/segment roots, verify
proofs or signatures, authenticate history, or establish ledger acceptance.
SHA-256 here implements only the format's tagged envelope digest.
"""

import hashlib


MODULUS = int("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001", 16)
ENVELOPE_TAG = "fenrir/rtf-envelope/v3"
MAX_STEPS = 64
MAX_SLOTS = 4096
FULL_STEPS = 16
STATE_FIELDS = ("cm_root", "cm_count", "nf_root", "nf_count", "rh_root", "rh_count",
                "wd_root", "wd_count", "height", "tx_count", "dep_consumed", "fee_total")
KIND_CODES = {"checkout_change": 0, "checkout_exact": 1, "p2p": 2,
              "sweep": 3, "deposit": 4, "withdrawal": 5}
SUM_FIELDS = ("fee_sum", "wd_sum_usd", "wd_sum_night", "dep_sum_usd", "dep_sum_night")
SLOT_DIGEST_FIELDS = ("fsqo_core_hash", "effects_root", "fefx_digest", "fhdr4_digest")


class CodecError(ValueError):
    """An invalid view, noncanonical encoding, or failed hash-free check."""

    def __init__(self, code, detail=""):
        self.code = code
        super().__init__(code + (": " + detail if detail else ""))


def th(tag: str, *parts: bytes) -> bytes:
    """SHA256(u64-le UTF-8 tag byte length || UTF-8 tag || ordered parts)."""
    if not isinstance(tag, str) or any(not isinstance(p, bytes) for p in parts):
        raise CodecError("MALFORMED", "tagged hash arguments")
    try:
        encoded = tag.encode("utf-8")
    except UnicodeEncodeError as exc:
        raise CodecError("MALFORMED", "tag encoding") from exc
    digest = hashlib.sha256(len(encoded).to_bytes(8, "little") + encoded)
    for part in parts:
        digest.update(part)
    return digest.digest()


def _uint(value, width, label):
    if type(value) is not int or not 0 <= value < 1 << (8 * width):
        raise CodecError("MALFORMED", label + " integer range")
    return value


def _obj(value, label):
    if not isinstance(value, dict):
        raise CodecError("MALFORMED", label + " object")
    return value


def _get(obj, key):
    _obj(obj, "view")
    if key not in obj:
        raise CodecError("MALFORMED", "missing " + key)
    return obj[key]


def _array(value, limit, label, minimum=0):
    if not isinstance(value, list) or not minimum <= len(value) <= limit:
        raise CodecError("MALFORMED", label + " count")
    return value


def _hex32(value, label, field=False):
    if (not isinstance(value, str) or len(value) != 64
            or any(c not in "0123456789abcdefABCDEF" for c in value)):
        raise CodecError("MALFORMED", label + " hex32")
    result = bytes.fromhex(value)
    if field and int.from_bytes(result, "little") >= MODULUS:
        raise CodecError("NON_CANONICAL_FIELD", label)
    return result


class _Reader:
    def __init__(self, data):
        if not isinstance(data, bytes):
            raise CodecError("MALFORMED", "input must be bytes")
        self.data = data
        self.offset = 0

    def take(self, count):
        end = self.offset + count
        if end > len(self.data):
            raise CodecError("MALFORMED", "truncated")
        result = self.data[self.offset:end]
        self.offset = end
        return result

    def uint(self, width):
        return int.from_bytes(self.take(width), "little")

    def hex32(self, field=False):
        value = self.take(32)
        if field and int.from_bytes(value, "little") >= MODULUS:
            raise CodecError("NON_CANONICAL_FIELD")
        return value.hex()


def decode(data: bytes) -> dict:
    """Decode exactly one canonical envelope, without semantic root recomputation.

    State vectors contain twelve hex32 field encodings in ``STATE_FIELDS``
    order. All digest fields are lowercase hex32, other scalar fields integers.
    Profile A has ``body=None``; profile B has slots/tierc/sign_fold_root.
    Use ``validate`` to also check the hash-free semantic relation.
    """
    r = _Reader(data)
    if r.take(4) != b"FRTF":
        raise CodecError("MALFORMED", "magic")
    if r.uint(1) != 3:
        raise CodecError("MALFORMED", "version")
    profile = r.uint(1)
    workload = r.uint(1)
    if profile not in (1, 2):
        raise CodecError("MALFORMED", "profile")
    if workload not in (0, 1):
        raise CodecError("MALFORMED", "workload")
    if r.uint(1) != 0:
        raise CodecError("MALFORMED", "flags")
    e = {"format_version": 3, "profile": profile, "workload": workload}
    for key in ("seed", "batch_no", "h_first", "h_last"):
        e[key] = r.uint(8)
    if (e["batch_no"] == 0 or e["h_first"] == 0 or e["h_last"] < e["h_first"]
            or e["h_last"] >= 1 << 47):
        raise CodecError("MALFORMED", "batch range")
    for key in ("state_pre", "state_post"):
        e[key] = [r.hex32(field=True) for _ in STATE_FIELDS]
    for key in ("pre_digest", "post_digest", "in_root"):
        e[key] = r.hex32(field=True)
    e["dep_from"] = r.uint(8)
    e["dep_to"] = r.uint(8)
    if e["dep_to"] < e["dep_from"]:
        raise CodecError("MALFORMED", "deposit range")
    e["dep_acc_from"] = r.hex32(field=True)
    e["dep_acc_to"] = r.hex32(field=True)
    for key in SUM_FIELDS:
        e[key] = r.uint(8)
    e["prev_envelope_digest"] = r.hex32()
    count = r.uint(4)
    if not 2 <= count <= MAX_STEPS:
        raise CodecError("MALFORMED", "step count")
    if workload == 1 and count != FULL_STEPS:
        raise CodecError("MALFORMED", "full batch step count")
    steps = []
    for _ in range(count):
        step = {"record_hash": r.hex32(field=True), "pos": r.uint(8), "tx_index": r.uint(8),
                "pre_digest": r.hex32(field=True), "post_digest": r.hex32(field=True),
                "pre_closes": r.uint(4), "post_close_from": r.uint(8),
                "post_closes": r.uint(4), "kind": r.uint(1), "covenant": r.uint(1)}
        if step["kind"] > 5 or step["covenant"] > 1:
            raise CodecError("MALFORMED", "step kind")
        steps.append(step)
    e["steps"] = steps
    e["body"] = None
    if profile == 2:
        slot_count = r.uint(4)
        if slot_count > MAX_SLOTS:
            raise CodecError("MALFORMED", "slot count")
        slots = []
        for _ in range(slot_count):
            slot = {"height": r.uint(8)}
            for key in SLOT_DIGEST_FIELDS:
                slot[key] = r.hex32()
            slots.append(slot)
        tierc_count = r.uint(4)
        if tierc_count > count:
            raise CodecError("MALFORMED", "tierc count")
        tierc = [{"step": r.uint(4), "auth_digest": r.hex32(), "proof_digest": r.hex32()}
                 for _ in range(tierc_count)]
        e["body"] = {"slots": slots, "tierc": tierc, "sign_fold_root": r.hex32()}
    if r.offset != len(data):
        raise CodecError("TRAILING_BYTES")
    e["envelope_digest"] = th(ENVELOPE_TAG, data).hex()
    return e


def validate(data: bytes) -> dict:
    """Decode and apply the Rust envelope's complete hash-free batch checks.

    This is structural replay only. A successful return makes no assertion
    about Poseidon roots, signatures, proof validity or authenticated history.
    """
    e = decode(data)
    steps = e["steps"]
    if (steps[0]["pre_digest"] != e["pre_digest"]
            or steps[-1]["post_digest"] != e["post_digest"]
            or any(a["post_digest"] != b["pre_digest"] for a, b in zip(steps, steps[1:]))):
        raise CodecError("STEP_CHAIN_BREAK")
    lo, hi = e["h_first"] * 65536, e["h_last"] * 65536 + 65535
    if any(not lo <= step["pos"] <= hi for step in steps):
        raise CodecError("WRONG_HEIGHT_RANGE")
    if any(a["pos"] >= b["pos"] or a["tx_index"] + 1 != b["tx_index"]
           for a, b in zip(steps, steps[1:])):
        raise CodecError("BATCH_ORDER_BROKEN")
    next_height = e["h_first"]
    for i, step in enumerate(steps):
        if i and step["pre_closes"] != 0:
            raise CodecError("STEP_CHAIN_BREAK", "later pre-closes")
        next_height += step["pre_closes"]
        if next_height >= 1 << 64:
            raise CodecError("WRONG_HEIGHT_RANGE")
        height = step["pos"] // 65536
        if height != next_height:
            raise CodecError("WRONG_HEIGHT_RANGE")
        if step["post_closes"]:
            if step["post_close_from"] != height:
                raise CodecError("WRONG_HEIGHT_RANGE")
            next_height = height + step["post_closes"]
            if next_height >= 1 << 64:
                raise CodecError("WRONG_HEIGHT_RANGE")
        elif step["post_close_from"] != 0:
            raise CodecError("MALFORMED", "post_close_from without closes")
    if next_height != e["h_last"] + 1:
        raise CodecError("WRONG_HEIGHT_RANGE")
    body = e["body"]
    if body is not None:
        slots = body["slots"]
        if (len(slots) != e["h_last"] - e["h_first"] + 1
                or any(s["height"] != e["h_first"] + i for i, s in enumerate(slots))):
            raise CodecError("MALFORMED", "slots")
        covenants = [i for i, step in enumerate(steps) if step["covenant"] == 1]
        if covenants != [entry["step"] for entry in body["tierc"]]:
            raise CodecError("MALFORMED", "tierc steps")
        if any(t["auth_digest"] != t["proof_digest"] for t in body["tierc"]):
            raise CodecError("TIERC_DIGEST_MISMATCH")
    elif any(step["covenant"] for step in steps):
        raise CodecError("MALFORMED", "covenant step in profile A")
    return e


def _state_bytes(state):
    state = _obj(state, "state")
    result = bytearray()
    for name in STATE_FIELDS:
        value = _get(state, name)
        if name.endswith("_root"):
            result.extend(_hex32(value, name, field=True))
        else:
            width = 4 if name in ("cm_count", "nf_count", "rh_count", "wd_count") else 8
            result.extend(_uint(value, width, name).to_bytes(32, "little"))
    return result


def encode_view(view: dict) -> bytes:
    """Reconstruct bytes from an actual generator view and bind its digest claim.

    Only encoded fields are consumed. Additional generator metadata (paths,
    Poseidon statements, full step-state openings and draft proof plans) is
    neither followed nor authenticated by this codec. Covenant bits are derived
    from the profile-B TierC step references, as in the generator.
    """
    view = _obj(view, "view")
    if _uint(_get(view, "format_version"), 1, "format_version") != 3:
        raise CodecError("MALFORMED", "version")
    profile_name = _get(view, "profile")
    workload_name = _get(view, "workload")
    if not isinstance(profile_name, str) or profile_name not in ("A", "B"):
        raise CodecError("MALFORMED", "profile")
    if not isinstance(workload_name, str) or workload_name not in ("minimal", "full"):
        raise CodecError("MALFORMED", "workload")
    profile = 1 if profile_name == "A" else 2
    workload = 0 if workload_name == "minimal" else 1
    seed = _get(view, "seed")
    if (not isinstance(seed, str) or not 3 <= len(seed) <= 18 or not seed.startswith("0x")
            or any(c not in "0123456789abcdefABCDEF" for c in seed[2:])):
        raise CodecError("MALFORMED", "seed hex u64")
    seed = _uint(int(seed[2:], 16), 8, "seed")
    steps = _array(_get(view, "steps"), MAX_STEPS, "steps", minimum=2)
    if workload == 1 and len(steps) != FULL_STEPS:
        raise CodecError("MALFORMED", "full batch step count")
    tierc = []
    slots = []
    if profile == 2:
        tierc = _array(_get(view, "tierc"), len(steps), "tierc")
        slots = _array(_get(view, "slots"), MAX_SLOTS, "slots")
    elif any(key in view for key in ("tierc", "slots", "sign_fold_root_draft")):
        raise CodecError("MALFORMED", "profile A body")
    covenant_steps = set()
    for entry in tierc:
        index = _uint(_get(entry, "step"), 4, "tierc step")
        if index >= len(steps):
            raise CodecError("MALFORMED", "tierc step range")
        covenant_steps.add(index)

    out = bytearray(b"FRTF" + bytes((3, profile, workload, 0)))

    def put_uint(value, width, label):
        out.extend(_uint(value, width, label).to_bytes(width, "little"))

    def put_hex(value, label, field=False):
        out.extend(_hex32(value, label, field=field))

    put_uint(seed, 8, "seed")
    for key in ("batch_no", "h_first", "h_last"):
        put_uint(_get(view, key), 8, key)
    out.extend(_state_bytes(_get(view, "state_pre")))
    out.extend(_state_bytes(_get(view, "state_post")))
    for key in ("pre", "post", "in_root"):
        put_hex(_get(view, key), key, field=True)
    for key in ("dep_from", "dep_to"):
        put_uint(_get(view, key), 8, key)
    for key in ("dep_acc_from", "dep_acc_to"):
        put_hex(_get(view, key), key, field=True)
    for key in SUM_FIELDS:
        put_uint(_get(view, key), 8, key)
    put_hex(_get(view, "prev_envelope_digest"), "prev_envelope_digest")
    put_uint(len(steps), 4, "step count")
    for i, step in enumerate(steps):
        put_hex(_get(step, "record_hash"), "record_hash", field=True)
        for key in ("pos", "tx_index"):
            put_uint(_get(step, key), 8, key)
        for key in ("pre_digest", "post_digest"):
            put_hex(_get(step, key), key, field=True)
        pre_closes = _obj(_get(step, "pre_closes"), "pre_closes")
        post_closes = _obj(_get(step, "post_closes"), "post_closes")
        pre_count = _uint(_get(pre_closes, "count"), 4, "pre_closes")
        expected_from = _get(view, "h_first") if pre_count else 0
        if _uint(_get(pre_closes, "from"), 8, "pre_closes from") != expected_from:
            raise CodecError("MALFORMED", "pre_closes from")
        put_uint(pre_count, 4, "pre_closes")
        put_uint(_get(post_closes, "from"), 8, "post_close_from")
        put_uint(_get(post_closes, "count"), 4, "post_closes")
        kind = _get(step, "kind")
        if not isinstance(kind, str) or kind not in KIND_CODES:
            raise CodecError("MALFORMED", "step kind")
        put_uint(KIND_CODES[kind], 1, "kind")
        put_uint(int(i in covenant_steps), 1, "covenant")
    if profile == 2:
        put_uint(len(slots), 4, "slot count")
        for slot in slots:
            put_uint(_get(slot, "height"), 8, "slot height")
            for key in ("fsqo_core_hash", "effects_root", "fefx_draft_digest", "fhdr4_draft_digest"):
                put_hex(_get(slot, key), key)
        put_uint(len(tierc), 4, "tierc count")
        for entry in tierc:
            put_uint(_get(entry, "step"), 4, "tierc step")
            put_hex(_get(entry, "auth_digest"), "auth_digest")
            put_hex(_get(entry, "proof_digest"), "proof_digest")
        put_hex(_get(view, "sign_fold_root_draft"), "sign_fold_root_draft")
    data = bytes(out)
    normalized = validate(data)
    claim = _hex32(_get(view, "envelope_digest"), "envelope_digest").hex()
    if claim != normalized["envelope_digest"]:
        raise CodecError("ENVELOPE_DIGEST_MISMATCH")
    return data
