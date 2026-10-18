"""Pure C2 Issue artifacts from a parsed Rust proof export and its setup record.

Author-proposed implementation of C2-ISSUE-ARTIFACT-CONTRACT.md CI01-CI12; not
reviewed or adopted. The consumer checks the non-pairing consistency of an
already parsed moriarty-k2-groth16-proof/1 export against its public
moriarty-k2-groth16-setup/1 record, then derives EnvV2 Data/CBOR, the envelope
token name, C2Mint Issue Data/CBOR and the mint Value Data through the pinned
host_codec. Groth16 validity, VK or proof curve validity, setup-record byte
identity, ledger or provider authenticity, Preview/Preprod acceptance and
legacy-builder or UPLC compatibility are not claimed. The expiry and the policy
id are caller supplied and not bound by the proof.
"""
import hashlib
import host_codec

_ARTIFACT_FORMAT = "moriarty-c2-issue-artifacts/1"

# export.rs profile constants and fixed premises, as public literals.
_PROOF_FORMAT = "moriarty-k2-groth16-proof/1"
_SETUP_FORMAT = "moriarty-k2-groth16-setup/1"
_TRUST_PROFILE = "single-party-test-setup/1"
_SETUP_NOTICE = (
    "TEST-ONLY single-party Groth16 setup sampled from OsRng; not a ceremony. "
    "Whoever controlled the setup process could forge proofs for this relation. "
    "VK provenance is declared by this record; no verifier can check that the VK "
    "was generated from r1csSha256.")
_CURVE = "BLS12-381"
_SNARK = "Groth16 (ark-groth16 =0.4.0, LibsnarkReduction)"
_ENCODING = ("arkworks 0.4 compressed points (Zcash BLS12-381 layout); "
             "VK 488 bytes, proof 192 bytes")
_PREMISES = (
    "The proof shows, under setup honesty and Groth16 knowledge soundness, that the "
    "statement's constant header and body satisfy the named compiled moriarty-core/1 "
    "relation.",
    "Context bytes (vk digest, claimant, network, settle hash, thread, consumed outref, "
    "seq, catalogue digest, intents digest) are caller-declared premises: bound by the "
    "proof, not authenticated by it. S2 recomputes each from its authenticated "
    "ScriptContext and datum.",
    "Trust profile single-party-test-setup/1: whoever ran setup could forge proofs for "
    "this relation.",
    "Groth16 proofs are malleable: the proof bytes are not a unique claim identifier.",
    "Signed intents, debit authority, frame completeness, physical asset delta, replay "
    "and phase outcomes are target obligations outside this proof.",
    "No ledger admission, deployment, settlement, Preprod or Midnight result is implied.",
)

# ABI v2 statement tag and family id as the relation descriptor records them.
_STMT_TAG_HEX = "4d524731362d5056432d53544d542d32"
_FAMILY_HEX = "4d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32"
_HEADER_LEN = 343
_MAX_STATEMENT_BYTES = 262144
_MAX_UINT64 = (1 << 64) - 1
_HEX_DIGITS = "0123456789abcdef"
_VK_HEX_LEN = 976
_PROOF_HEX_LEN = 384
_POLICY_LEN = 28

_LABELS = {
    "expiry": "CALLER_SUPPLIED_NOT_PROOF_BOUND",
    "context": "CALLER_DECLARED_UNAUTHENTICATED",
    "policyId": "CALLER_SUPPLIED_UNAUTHENTICATED",
    "trustProfile": "single-party-test-setup/1",
    "setupRecordLink": "DECLARED_NOT_BYTE_CHECKED",
    "groth16": "NOT_VERIFIED",
    "vkCurveValidity": "NOT_CHECKED",
    "ledger": "NO_CLAIM",
}

# Schema kinds: ("object", fields, closed), ("hex", chars), ("hex-or-null", chars),
# ("int", minimum, maximum or None), ("str",), ("fixed", text), ("premises",).
_HEX32 = ("hex", 64)
_HEX28 = ("hex", 56)
_TEXT = ("str",)
_U8 = ("int", 0, 255)
_U16 = ("int", 0, 65535)
_U64 = ("int", 0, _MAX_UINT64)

# The ten ContextExport fields, named as parse_statement_header returns them.
_CONTEXT = {
    "vkDigest": _HEX32,
    "claimantVkh": _HEX28,
    "networkId": _U8,
    "settleHash": _HEX28,
    "threadId": _HEX32,
    "consumedTxid": _HEX32,
    "consumedIx": _U16,
    "seq": _U64,
    "catalogueDigest": _HEX32,
    "intentsDigest": _HEX32,
}
_PUBLIC_INPUTS = {"x1": _TEXT, "x2": _TEXT}
_OUTREF = {"transactionId": _HEX32, "outputIndex": _U16}
# EnvV2Export has no expiry: an injected one fails the closed schema (CI03).
_ENV_V2 = {
    "version": _U8,
    "claimKind": _U8,
    "familyId": _HEX32,
    "predicateDigest": _HEX32,
    "vkDigest": _HEX32,
    "claimantVkh": _HEX28,
    "settleOutref": ("object", _OUTREF, True),
    "statementDigest": _HEX32,
}
_ACTION_ENTRY = {
    "name": _TEXT,
    "index": _U16,
    "predicateDigest": _HEX32,
    "vkDigest": _HEX32,
    "shapeDigest": _HEX32,
}
_CARDANO = {
    "envV2": ("object", _ENV_V2, True),
    "catalogueActionEntry": ("object", _ACTION_ENTRY, True),
    "intentTargetHashV2": _TEXT,
    "intentAnchor": _TEXT,
}
_PROOF_EXPORT = ("object", {
    "format": ("fixed", _PROOF_FORMAT),
    "setupRecordSha256": _HEX32,
    "programHash": _HEX32,
    "sourceSha256": ("hex-or-null", 64),
    "action": _TEXT,
    "actionIndex": _U16,
    "compilerBuildDigest": _HEX32,
    "r1csSha256": _HEX32,
    "predicateDigest": _HEX32,
    "abi2ShapeDigest": _HEX32,
    "vkDigestBlake2b256": _HEX32,
    "context": ("object", _CONTEXT, True),
    "statementHex": _TEXT,
    "statementDigestBlake2b256": _HEX32,
    "publicInputs": ("object", _PUBLIC_INPUTS, True),
    "proofHex": _TEXT,
    "cardano": ("object", _CARDANO, True),
    "premises": ("premises",),
}, True)

# Only the relation keys this consumer reads; other descriptor keys stay open.
_RELATION = {
    "programHash": _HEX32,
    "sourceSha256": ("hex-or-null", 64),
    "action": _TEXT,
    "actionIndex": _U16,
    "compilerBuildDigest": _HEX32,
    "r1csSha256": _HEX32,
    "predicateDigest": _HEX32,
    "abi2ShapeDigest": _HEX32,
    "statementTagHex": ("fixed", _STMT_TAG_HEX),
    "familyIdHex": ("fixed", _FAMILY_HEX),
    "headerLen": ("int", _HEADER_LEN, _HEADER_LEN),
    "bodyLen": ("int", 0, _MAX_STATEMENT_BYTES - _HEADER_LEN),
    "publicInputCount": ("int", 2, 2),
}
_SETUP_RECORD = ("object", {
    "format": ("fixed", _SETUP_FORMAT),
    "trustProfile": ("fixed", _TRUST_PROFILE),
    "notice": ("fixed", _SETUP_NOTICE),
    "relation": ("object", _RELATION, False),
    "curve": ("fixed", _CURVE),
    "snark": ("fixed", _SNARK),
    "encoding": ("fixed", _ENCODING),
    "verifyingKeyHex": ("hex", _VK_HEX_LEN),
    "vkDigestBlake2b256": _HEX32,
    "provingKeySha256": _HEX32,
    "provingKeyBytes": ("int", 1, None),
    "driver": _TEXT,
}, True)

# check_export identity comparisons between the export and the setup relation.
_RELATION_IDENTITY = ("programHash", "sourceSha256", "action", "actionIndex",
                      "compilerBuildDigest", "r1csSha256", "predicateDigest",
                      "abi2ShapeDigest")


def _refuse(code, detail):
    raise ValueError(code + ": " + detail)


def _host(code, api, *args):
    # Host refusals carry the stage code; other exceptions propagate unconverted.
    try:
        return api(*args)
    except ValueError as error:
        raise ValueError(code + ": host refused: " + str(error)) from None


def _is_hex(value, chars):
    return (type(value) is str and len(value) == chars
            and set(value).issubset(_HEX_DIGITS))


def _admit_object(value, fields, closed, code, path):
    if type(value) is not dict:
        _refuse(code, path + ": expected exact object")
    # Iterating an exact dict neither hashes nor compares its stored keys, and
    # unknown fields are refused before any value of this object is visited.
    for key in value:
        if type(key) is not str:
            _refuse(code, path + ": expected exact string keys")
        if closed and key not in fields:
            _refuse(code, path + ": expected closed schema object")
    for key, kind in fields.items():
        if key not in value:
            _refuse(code, path + "." + key + ": expected required field")
        _admit(value[key], kind, code, path + "." + key)


def _admit(value, kind, code, path):
    tag = kind[0]
    if tag == "object":
        _admit_object(value, kind[1], kind[2], code, path)
        return
    if tag == "hex":
        admitted = _is_hex(value, kind[1])
    elif tag == "hex-or-null":
        admitted = value is None or _is_hex(value, kind[1])
    elif tag == "int":
        admitted = (type(value) is int and kind[1] <= value
                    and (kind[2] is None or value <= kind[2]))
    elif tag == "str":
        admitted = type(value) is str
    elif tag == "fixed":
        admitted = type(value) is str and value == kind[1]
    elif tag == "premises":
        admitted = (type(value) is list and len(value) == len(_PREMISES)
                    and all(type(item) is str and item == premise
                            for item, premise in zip(value, _PREMISES)))
    else:
        admitted = False
    if not admitted:
        _refuse(code, path + ": outside the " + tag + " schema")


def prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id):
    """Return fresh C2 Issue artifacts, or raise ValueError('C2I-<stage>: ...').

    Stages run in the fixed order ARG, SETUP, EXPORT, RELATION, STATEMENT,
    CONTEXT, DIGEST, PROOF, CARDANO and BUILD; neither record is mutated.
    """
    # ARG: exact types only, so neither record is touched before this passes.
    if type(proof_export) is not dict or type(setup_record) is not dict:
        _refuse("C2I-ARG", "records: expected exact dict objects")
    if type(expiry) is not int or not 0 <= expiry <= _MAX_UINT64:
        _refuse("C2I-ARG", "expiry: expected exact int in 0..2**64-1")
    if type(policy_id) is not bytes or len(policy_id) != _POLICY_LEN:
        _refuse("C2I-ARG", "policy_id: expected exact 28-byte bytes")

    # SETUP: closed record, profile literals, consumed relation keys, VK digest.
    _admit(setup_record, _SETUP_RECORD, "C2I-SETUP", "setup")
    relation = setup_record["relation"]
    vk = bytes.fromhex(setup_record["verifyingKeyHex"])
    # Unkeyed BLAKE2b-256 of the 488 VK bytes (length only; no curve check).
    vk_digest = hashlib.blake2b(vk, digest_size=32).hexdigest()
    if vk_digest != setup_record["vkDigestBlake2b256"]:
        _refuse("C2I-SETUP", "vkDigestBlake2b256: differs from BLAKE2b-256 of the VK")

    # EXPORT: closed ProofExport schema at every level.
    _admit(proof_export, _PROOF_EXPORT, "C2I-EXPORT", "export")

    # RELATION: the export names the setup relation and its VK.
    for key in _RELATION_IDENTITY:
        if proof_export[key] != relation[key]:
            _refuse("C2I-RELATION", key + ": differs from the setup relation")
    if proof_export["vkDigestBlake2b256"] != vk_digest:
        _refuse("C2I-RELATION", "vkDigestBlake2b256: differs from the setup VK digest")

    # STATEMENT: bounded lowercase hex, host header admission, relation constants.
    statement_hex = proof_export["statementHex"]
    if (len(statement_hex) > 2 * _MAX_STATEMENT_BYTES or len(statement_hex) % 2
            or not set(statement_hex).issubset(_HEX_DIGITS)):
        _refuse("C2I-STATEMENT", "statementHex: expected bounded even lowercase hex")
    statement = bytes.fromhex(statement_hex)
    header = _host("C2I-STATEMENT", host_codec.parse_statement_header, statement)
    for name, expected in (("predicateDigest", relation["predicateDigest"]),
                           ("shapeDigest", relation["abi2ShapeDigest"]),
                           ("vkDigest", vk_digest),
                           ("bodyLen", relation["bodyLen"])):
        if header[name] != expected:
            _refuse("C2I-STATEMENT", "header " + name + ": differs from the relation")

    # CONTEXT: declared fields equal the header bytes; testnet only (network 0
    # does not distinguish Preview from Preprod).
    if proof_export["context"] != {key: header[key] for key in _CONTEXT}:
        _refuse("C2I-CONTEXT", "context: differs from the statement header")
    if header["networkId"] != 0:
        _refuse("C2I-CONTEXT", "networkId: only testnet 0 is admitted")

    # DIGEST: host digest and public inputs as exact strings.
    digest = _host("C2I-DIGEST", host_codec.statement_digest_hex, statement)
    if digest != proof_export["statementDigestBlake2b256"]:
        _refuse("C2I-DIGEST", "statementDigestBlake2b256: differs from the statement")
    public_inputs = proof_export["publicInputs"]
    halves = _host("C2I-DIGEST", host_codec.statement_public_inputs, statement)
    if halves != (public_inputs["x1"], public_inputs["x2"]):
        _refuse("C2I-DIGEST", "publicInputs: differ from the statement digest halves")

    # PROOF: 192 bytes by length only; no curve, subgroup or pairing check.
    if not _is_hex(proof_export["proofHex"], _PROOF_HEX_LEN):
        _refuse("C2I-PROOF", "proofHex: expected exactly 384 lowercase hex")

    # CARDANO: the section equals the cardano_export derivation.
    predicate = relation["predicateDigest"]
    shape = relation["abi2ShapeDigest"]
    derived = {
        "envV2": {
            "version": 2,
            "claimKind": 0,
            "familyId": _FAMILY_HEX,
            "predicateDigest": predicate,
            "vkDigest": vk_digest,
            "claimantVkh": header["claimantVkh"],
            "settleOutref": {"transactionId": header["consumedTxid"],
                             "outputIndex": header["consumedIx"]},
            "statementDigest": digest,
        },
        "catalogueActionEntry": {
            "name": relation["action"],
            "index": relation["actionIndex"],
            "predicateDigest": predicate,
            "vkDigest": vk_digest,
            "shapeDigest": shape,
        },
        "intentTargetHashV2": predicate + ":" + vk_digest + ":" + shape,
        "intentAnchor": "cardano-plutus-v3:" + header["settleHash"],
    }
    if proof_export["cardano"] != derived:
        _refuse("C2I-CARDANO", "cardano: differs from the derived section")

    # BUILD: host factories only; the expiry comes from the argument alone.
    env_data = _host("C2I-BUILD", host_codec.build_env_data,
                     bytes.fromhex(predicate), bytes.fromhex(vk_digest),
                     bytes.fromhex(header["claimantVkh"]),
                     bytes.fromhex(header["consumedTxid"]), header["consumedIx"],
                     bytes.fromhex(digest), expiry)
    env_cbor = _host("C2I-BUILD", host_codec.encode_data, env_data)
    token = hashlib.blake2b(env_cbor, digest_size=32).digest()
    issue_data = _host("C2I-BUILD", host_codec.build_c2_mint_data, "Issue", vk,
                       bytes.fromhex(proof_export["proofHex"]))
    issue_cbor = _host("C2I-BUILD", host_codec.encode_data, issue_data)
    # F01: canonical no-ada Value (lovelace None, never 0).
    mint_value_data = _host("C2I-BUILD", host_codec.build_value_data, None,
                            [(policy_id, token, 1)])
    mint_value_cbor = _host("C2I-BUILD", host_codec.encode_data, mint_value_data)
    return {
        "format": _ARTIFACT_FORMAT,
        "envData": env_data,
        "envCborHex": env_cbor.hex(),
        "envelopeTokenNameHex": token.hex(),
        "issueData": issue_data,
        "issueCborHex": issue_cbor.hex(),
        "mintValueData": mint_value_data,
        "mintValueCborHex": mint_value_cbor.hex(),
        "policyIdHex": policy_id.hex(),
        "expiry": expiry,
        "setupRecordSha256Declared": proof_export["setupRecordSha256"],
        "labels": dict(_LABELS),
    }
