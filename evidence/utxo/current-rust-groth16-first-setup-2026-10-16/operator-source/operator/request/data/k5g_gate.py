"""K5 host gate for step K5g (clauses K5-8 and K5-8A). PROPOSED SOURCE; never executed by its author.

Run only as:
  /usr/bin/python3 -I k5g_gate.py <W5>/out <W5>/keys <T5>/receipts/K5M-summary.json <stage-C K4b-descriptor.json>
                                  <T5>/receipts/P0-bin.sha256 <RR>/K2-LOCK-PIN.sha256 <K4H patched k2/src/lib.rs>
                                  <C k2/src/export.rs>

Reads: the host-written summary and binary receipt, four root-pinned files, the one setup record under <W5>/out
and the one proving-key file under <W5>/keys. Both directories are listed with lstat only; a record or key is
opened only when its own entry is a regular file and not a link, with O_NOFOLLOW, and the opened inode must equal
the listed one. The key is hashed in 1 MiB chunks: its bytes are never stored, printed or copied. The output holds
the VK digest, the record SHA-256 and the key's path, size, mode, link count and SHA-256, never VK or key bytes.

Verdicts, in this precedence (clause K5-5 maps them):
  STOP-CUSTODY     a link, non-regular, missing or extra entry, wrong mode, owner or link count, or a file that
                   changed while read, under <W5>/out or <W5>/keys                               -> BLOCKED(hygiene)
  STOP-IDENTITY    the record's relation or driver, the observed matrix digest, a pinned input, the gate's own
                   constants, or the key's size or SHA-256 differs from what it must equal       -> BLOCKED(identity)
  FAIL-PREDICTION  identities hold but a format, constant, VK-structure, digest or summary check fails
                                                                                                  -> FAILED(prediction)
  PASS             every check holds.
Each check runs inside its own guard. An exception there is appended to that check's list, so a data mismatch
never becomes a PASS and never hides a problem already collected (review finding R-K4H-02). Exit 0 whenever a
verdict was written; exit 2 with verdict REFUSED on a usage or interpreter-level fault (BLOCKED(environment)).
Subgroup, pairing and proof verification are deliberately absent: they are K6 scope, not this gate.
"""
import hashlib
import json
import os
import re
import stat
import sys

MAX_JSON = 1 << 20
CHUNK = 1 << 20
RECORD_NAME = "invoice-partial.setup.json"
KEY_NAME = "invoice-partial.pk"
STAGE_C_DESCRIPTOR_SHA256 = "7bae4fb1ded96b8c767acfabc135bb54aafab0c8a13a648686da32d38696e959"
EXPECTED_R1CS = "e458153e583e0a817fbab08f24f4ba16465e459969c1f8cd4e13b801e82694c0"
EXPECTED_LOCK = "39b7db9be3e68b2a29cf281d53e54546a65c6862921771b9462cacb8f08de556"
TOOLCHAIN = "1.99.0-x86_64-unknown-linux-gnu"
DESCRIPTOR_FIELDS = 29
# Typed here from export.rs and the K4H lib.rs, and compared at run time with the pinned source text itself.
CONSTANTS = {
    "SETUP_FORMAT": "moriarty-k2-groth16-setup/1",
    "TRUST_PROFILE": "single-party-test-setup/1",
    "SETUP_NOTICE": (
        "TEST-ONLY single-party Groth16 setup sampled from OsRng; not a ceremony. Whoever controlled the setup "
        "process could forge proofs for this relation. VK provenance is declared by this record; no verifier can "
        "check that the VK was generated from r1csSha256."
    ),
    "CURVE": "BLS12-381",
    "SNARK": "Groth16 (ark-groth16 =0.4.0, LibsnarkReduction)",
    "ENCODING": "arkworks 0.4 compressed points (Zcash BLS12-381 layout); VK 488 bytes, proof 192 bytes",
    "DRIVER_ID": "mori-k2-driver/0.1.0-k2s1+k4h1",
}
# SetupRecord field order (export.rs, serde rename_all camelCase, deny_unknown_fields).
RECORD_KEYS = [
    "format", "trustProfile", "notice", "relation", "curve", "snark", "encoding", "verifyingKeyHex",
    "vkDigestBlake2b256", "provingKeySha256", "provingKeyBytes", "driver",
]
# The `mori-k2 setup` stdout summary (patched main.rs).
SUMMARY_KEYS = {"setupRecordSha256", "predicateDigest", "observedR1csSha256", "vkDigestBlake2b256", "provingKeySha256",
                "trustProfile"}
VK_HEX_LEN = 976
VK_IC_OFFSET = 336
VK_IC_PREFIX = bytes([3, 0, 0, 0, 0, 0, 0, 0])
# Compressed points of the 488-byte VK (encoding.rs): first byte carries the compression and infinity flags.
# This is a structure check only; on-curve and subgroup validation stays with the driver parser and future K6.
VK_POINTS = (("alpha", 0), ("beta", 48), ("gamma", 144), ("delta", 240), ("ic0", 344), ("ic1", 392), ("ic2", 440))
# Source-derived predictions (EXPECTATIONS.json). Recorded next to the observation; never gated.
PREDICTED_PK_BYTES = 98715456
PREDICTED_RECORD_BYTES = 3926
HEX64 = re.compile(r"[0-9a-f]{64}")


def refuse_duplicates(pairs):
    out = {}
    for k, v in pairs:
        if k in out:
            raise ValueError("duplicate key %r" % k)
        out[k] = v
    return out


def no_constant(name):
    raise ValueError("non-finite number %s" % name)


def no_float(text):
    raise ValueError("non-integer number %s" % text)


def strict_object(data, what):
    value = json.loads(data.decode("utf-8"), object_pairs_hook=refuse_duplicates, parse_constant=no_constant,
                       parse_float=no_float)
    if not isinstance(value, dict):
        raise ValueError("%s is not a JSON object" % what)
    return value


def canonical(value):
    """Order- and type-exact text of a JSON value (Python's True == 1 must not make true equal 1)."""
    return json.dumps(value, sort_keys=False, separators=(",", ":"))


def mode_of(st):
    return "%04o" % stat.S_IMODE(st.st_mode)


def open_listed(path, listed):
    """Opens a file lstat-ed as `listed` without following a link; refuses any change of inode or type."""
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    f = os.fdopen(fd, "rb")
    st = os.fstat(f.fileno())
    if not stat.S_ISREG(st.st_mode) or (st.st_dev, st.st_ino) != (listed.st_dev, listed.st_ino):
        f.close()
        raise ValueError("%s changed between listing and open" % path)
    return f


def read_small(path):
    st = os.lstat(path)
    if not stat.S_ISREG(st.st_mode):
        raise ValueError("%s is not a regular non-link file" % path)
    if st.st_size > MAX_JSON:
        raise ValueError("%s exceeds 1 MiB" % path)
    with open_listed(path, st) as f:
        data = f.read(MAX_JSON + 1)
    if len(data) != st.st_size:
        raise ValueError("%s changed while it was read" % path)
    return data


def rust_str_const(text, name):
    found = re.findall(r'^pub const %s: &str = "([^"\\]*)";$' % re.escape(name), text, re.M)
    if len(found) != 1:
        raise ValueError("%d definitions of %s in the pinned source" % (len(found), name))
    return found[0]


def listing(path):
    """lstat of a directory and of each entry in it. Follows no link and reads no file content."""
    st = os.lstat(path)
    if stat.S_ISLNK(st.st_mode) or not stat.S_ISDIR(st.st_mode):
        raise ValueError("%s is a link or not a directory" % path)
    entries = {}
    for name in sorted(os.listdir(path)):
        entries[name] = os.lstat(os.path.join(path, name))
    return st, entries


def describe(entries):
    out = []
    for name, st in entries.items():
        if stat.S_ISREG(st.st_mode):
            kind = "file"
        elif stat.S_ISLNK(st.st_mode):
            kind = "link"
        elif stat.S_ISDIR(st.st_mode):
            kind = "directory"
        else:
            kind = "other"
        out.append({"name": name, "type": kind, "mode": mode_of(st), "bytes": st.st_size, "nlink": st.st_nlink})
    return out


def main(argv):
    if len(argv) != 9:
        print(json.dumps({"format": "k5g-gate/1", "verdict": "REFUSED", "reason": "usage"}))
        return 2
    out_dir, keys_dir, summary_path, descriptor_path, bin_receipt, lock_pin, lib_rs, export_rs = argv[1:]
    custody, ident, pred = [], [], []
    rec = {}
    state = {}

    def guard(bucket, name, check):
        try:
            check()
        except Exception as e:  # recorded in its class, never dropped (R-K4H-02)
            bucket.append("%s: %s: %s" % (name, type(e).__name__, e))

    def constants():
        lib = read_small(lib_rs).decode("utf-8")
        exp = read_small(export_rs).decode("utf-8")
        pinned = {"DRIVER_ID": rust_str_const(lib, "DRIVER_ID")}
        for name in ("SETUP_FORMAT", "TRUST_PROFILE", "SETUP_NOTICE", "CURVE", "SNARK", "ENCODING"):
            pinned[name] = rust_str_const(exp, name)
        for name, value in pinned.items():
            if value != CONSTANTS[name]:
                ident.append("gate constant %s differs from the pinned driver source" % name)

    def descriptor():
        data = read_small(descriptor_path)
        if hashlib.sha256(data).hexdigest() != STAGE_C_DESCRIPTOR_SHA256:
            ident.append("stage-C descriptor bytes are not K4b-descriptor.json 7bae4fb1...")
        d = strict_object(data, "stage-C descriptor")
        if len(d) != DESCRIPTOR_FIELDS:
            ident.append("stage-C descriptor has %d fields" % len(d))
        if d.get("r1csSha256") != EXPECTED_R1CS or d.get("action") != "partial":
            ident.append("stage-C descriptor is not invoice/partial with r1csSha256 e458153e...")
        state["descriptor"] = d

    def out_listing():
        _, entries = listing(out_dir)
        rec["outEntries"] = describe(entries)
        if list(entries) != [RECORD_NAME]:
            custody.append("%s holds %r; expected exactly [%r]" % (out_dir, list(entries), RECORD_NAME))
        r = entries.get(RECORD_NAME)
        if r is None:
            return
        if not stat.S_ISREG(r.st_mode):
            custody.append("the setup record is not a regular non-link file")
            return
        if r.st_nlink != 1:
            custody.append("the setup record has %d hard links" % r.st_nlink)
        state["record_st"] = r

    def keys_listing():
        st, entries = listing(keys_dir)
        rec["keysDirMode"] = mode_of(st)
        rec["keysEntries"] = describe(entries)
        if stat.S_IMODE(st.st_mode) != 0o700:
            custody.append("keys directory mode %s; expected 0700" % mode_of(st))
        if list(entries) != [KEY_NAME]:
            custody.append("%s holds %r; expected exactly [%r]" % (keys_dir, list(entries), KEY_NAME))
        k = entries.get(KEY_NAME)
        if k is None:
            return
        if not stat.S_ISREG(k.st_mode):
            custody.append("the proving key is not a regular non-link file; it is not read")
            return
        if stat.S_IMODE(k.st_mode) != 0o600:
            custody.append("proving-key mode %s; expected 0600" % mode_of(k))
        if k.st_nlink != 1:
            custody.append("the proving key has %d hard links" % k.st_nlink)
        if k.st_uid != os.getuid():
            custody.append("the proving key is owned by uid %d, not the operator" % k.st_uid)
        state["key_st"] = k  # a regular non-link entry: hashing it is permitted (link guard satisfied)

    def record_read():
        r = state.get("record_st")
        if r is None:
            return  # already a custody problem
        if r.st_size > MAX_JSON:
            pred.append("the setup record has %d bytes, above 1 MiB; it is not parsed" % r.st_size)
            return
        path = os.path.join(out_dir, RECORD_NAME)
        with open_listed(path, r) as f:
            data = f.read(MAX_JSON + 1)
        if len(data) != r.st_size:
            custody.append("the setup record changed while it was read")
            return
        rec["setupRecord"] = {"path": path, "bytes": len(data), "mode": mode_of(r),
                              "sha256": hashlib.sha256(data).hexdigest()}
        state["record_bytes"] = data

    def record():
        data = state.get("record_bytes")
        if data is None:
            return
        doc = strict_object(data, "setup record")
        state["record"] = doc
        if list(doc) != RECORD_KEYS:
            pred.append("setup record keys %r; expected %r" % (list(doc), RECORD_KEYS))
        for key, name in (("format", "SETUP_FORMAT"), ("trustProfile", "TRUST_PROFILE"), ("notice", "SETUP_NOTICE"),
                          ("curve", "CURVE"), ("snark", "SNARK"), ("encoding", "ENCODING")):
            if doc.get(key) != CONSTANTS[name]:
                pred.append("setup record %s is not %s byte for byte" % (key, name))
        if doc.get("driver") != CONSTANTS["DRIVER_ID"]:
            ident.append("setup record driver %r; expected %r" % (doc.get("driver"), CONSTANTS["DRIVER_ID"]))

    def relation():
        doc, d = state.get("record"), state.get("descriptor")
        if doc is None or d is None:
            return
        rel = doc.get("relation")
        if not isinstance(rel, dict):
            ident.append("setup record relation is not a JSON object")
            return
        if canonical(rel) != canonical(d):
            keys = list(d) + [k for k in rel if k not in d]
            diff = [k for k in keys if canonical(rel.get(k)) != canonical(d.get(k))]
            ident.append("setup record relation differs from K4b-descriptor.json (fields %r; order equal: %s)"
                         % (diff, list(rel) == list(d)))

    def verifying_key():
        doc = state.get("record")
        if doc is None:
            return
        h = doc.get("verifyingKeyHex")
        if not isinstance(h, str) or not re.fullmatch(r"[0-9a-f]{%d}" % VK_HEX_LEN, h):
            pred.append("verifyingKeyHex is not %d lowercase hex characters (488 bytes)" % VK_HEX_LEN)
            return
        vk = bytes.fromhex(h)
        if vk[VK_IC_OFFSET:VK_IC_OFFSET + 8] != VK_IC_PREFIX:
            pred.append("VK bytes 336..344 are %s, not 0300000000000000" % vk[VK_IC_OFFSET:VK_IC_OFFSET + 8].hex())
        for name, at in VK_POINTS:
            if vk[at] & 0x80 == 0 or vk[at] & 0x40:
                pred.append("VK point %s lacks the compression flag or has the infinity flag" % name)
        digest = hashlib.blake2b(vk, digest_size=32).hexdigest()
        declared = doc.get("vkDigestBlake2b256")
        rec["vkDigestBlake2b256"] = declared if isinstance(declared, str) else None
        rec["vkDigestRecomputed"] = digest
        if declared != digest:
            pred.append("vkDigestBlake2b256 is not hashlib BLAKE2b-256 of the 488 VK bytes")

    def proving_key():
        doc = state.get("record")
        want_sha = doc.get("provingKeySha256") if doc is not None else None
        want_len = doc.get("provingKeyBytes") if doc is not None else None
        if doc is not None:
            if not (isinstance(want_sha, str) and HEX64.fullmatch(want_sha)):
                pred.append("provingKeySha256 is not 64 lowercase hex characters")
            if type(want_len) is not int or want_len <= 0:
                pred.append("provingKeyBytes is not a positive integer")
        k = state.get("key_st")
        if k is None:
            return
        path = os.path.join(keys_dir, KEY_NAME)
        h = hashlib.sha256()
        n = 0
        try:
            f = open_listed(path, k)
        except (OSError, ValueError) as e:
            custody.append("the proving key could not be opened as listed: %s" % e)
            return
        with f:
            while True:
                chunk = f.read(CHUNK)
                if not chunk:
                    break
                h.update(chunk)
                n += len(chunk)
            after = os.fstat(f.fileno())
        if n != k.st_size or after.st_size != k.st_size or after.st_mtime_ns != k.st_mtime_ns:
            custody.append("the proving key changed while it was hashed")
        rec["provingKey"] = {"path": path, "bytes": n, "mode": mode_of(k), "nlink": k.st_nlink,
                             "sha256": h.hexdigest()}
        if doc is None:
            return
        if n != want_len:
            ident.append("proving-key size %d differs from the record's provingKeyBytes %r" % (n, want_len))
        if h.hexdigest() != want_sha:
            ident.append("proving-key SHA-256 differs from the record's provingKeySha256")

    def summary_read():
        state["summary"] = strict_object(read_small(summary_path), "K5M stdout summary")

    def summary():
        s = state.get("summary")
        if s is None:
            return
        if set(s) != SUMMARY_KEYS:
            pred.append("summary keys %r; expected %r" % (sorted(s), sorted(SUMMARY_KEYS)))
        rec["observedR1csSha256"] = s.get("observedR1csSha256")
        if s.get("observedR1csSha256") != EXPECTED_R1CS:
            ident.append("observedR1csSha256 %r is not e458153e..." % s.get("observedR1csSha256"))
        d = state.get("descriptor")
        if d is not None:
            if s.get("observedR1csSha256") != d.get("r1csSha256"):
                ident.append("observedR1csSha256 differs from the stage-C descriptor's r1csSha256")
            if s.get("predicateDigest") != d.get("predicateDigest"):
                ident.append("summary predicateDigest differs from the stage-C descriptor")
        if s.get("trustProfile") != CONSTANTS["TRUST_PROFILE"]:
            pred.append("summary trustProfile %r" % s.get("trustProfile"))
        data = state.get("record_bytes")
        if data is not None and s.get("setupRecordSha256") != hashlib.sha256(data).hexdigest():
            pred.append("summary setupRecordSha256 is not the SHA-256 of the record file")
        doc = state.get("record")
        if doc is not None:
            for key in ("vkDigestBlake2b256", "provingKeySha256"):
                if s.get(key) != doc.get(key):
                    pred.append("summary %s differs from the setup record" % key)

    def provenance_inputs():
        b = read_small(bin_receipt).decode("ascii")
        m = re.fullmatch(r"([0-9a-f]{64})  mori-k2\n", b)
        if m is None:
            ident.append("P0-bin.sha256 is not exactly one mori-k2 line")
        state["bin"] = m.group(1) if m else None
        if read_small(lock_pin).decode("ascii") != EXPECTED_LOCK + "  Cargo.lock\n":
            ident.append("the lock pin is not 39b7db9b...  Cargo.lock")

    guard(ident, "driver constants", constants)
    guard(ident, "stage-C descriptor", descriptor)
    guard(custody, "out listing", out_listing)
    guard(custody, "keys listing", keys_listing)
    guard(custody, "setup record read", record_read)
    guard(pred, "setup record", record)
    guard(ident, "relation", relation)
    guard(pred, "verifying key", verifying_key)
    guard(ident, "proving key", proving_key)
    guard(ident, "K5M summary read", summary_read)
    guard(pred, "K5M summary", summary)
    guard(ident, "provenance inputs", provenance_inputs)

    verdict = "STOP-CUSTODY" if custody else ("STOP-IDENTITY" if ident else ("FAIL-PREDICTION" if pred else "PASS"))
    d = state.get("descriptor") or {}
    provenance = [
        {"link": "predicateDigest", "value": d.get("predicateDigest"),
         "basis": "stage-C descriptor K4b-descriptor.json (7bae4fb1...); the record's relation was compared with it field for field here"},
        {"link": "descriptor r1csSha256", "value": d.get("r1csSha256"),
         "basis": "stage-C probe synthesis, re-observed byte for byte by K5a"},
        {"link": "Cargo.lock", "value": EXPECTED_LOCK,
         "basis": "root lock pin used by the K4H release build (KH-1); declared, not rebuilt in K5"},
        {"link": "toolchain", "value": TOOLCHAIN,
         "basis": "toolchain of the K4H release build (KH-1); declared, not observed in K5"},
        {"link": "release mori-k2 SHA-256", "value": state.get("bin"),
         "basis": "K4H KH2 receipt KH-binaries-pre.sha256; re-hashed by P0 on the K4H file and on the T5 copy, and before each K5 use"},
        {"link": "observedR1csSha256", "value": rec.get("observedR1csSha256"),
         "basis": "DigestBound inside the K5M setup process (patches/k2/src/setup.rs), printed on its stdout"},
        {"link": "vkDigestBlake2b256", "value": rec.get("vkDigestRecomputed"),
         "basis": "hashlib BLAKE2b-256 of the record's 488 VK bytes, recomputed here"},
    ]
    out = {
        "format": "k5g-gate/1",
        "verdict": verdict,
        "custodyProblems": custody,
        "identityProblems": ident,
        "predictionProblems": pred,
        "records": rec,
        "trustProfile": CONSTANTS["TRUST_PROFILE"],
        "provenance": provenance,
        "consumerCanCheckVkToMatrixLink": False,
        "provenanceNote": (
            "This chain is the setup's recorded provenance, not something a VK holder can verify: nobody holding only "
            "the VK can check that it was generated from r1csSha256 (SETUP_NOTICE). Under single-party-test-setup/1 "
            "whoever controlled this setup process could forge proofs for invoice/partial."
        ),
        "recordedNotGated": {
            "predictedProvingKeyBytes": PREDICTED_PK_BYTES,
            "observedProvingKeyBytes": (rec.get("provingKey") or {}).get("bytes"),
            "predictedSetupRecordBytes": PREDICTED_RECORD_BYTES,
            "observedSetupRecordBytes": (rec.get("setupRecord") or {}).get("bytes"),
        },
        "neverRecorded": "verifying-key bytes, proving-key bytes, setup scalars",
    }
    print(json.dumps(out, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except Exception as e:  # interpreter-level refusal outside every guard: BLOCKED(environment) per K5-5
        print(json.dumps({"format": "k5g-gate/1", "verdict": "REFUSED", "reason": "%s: %s" % (type(e).__name__, e)}))
        sys.exit(2)
