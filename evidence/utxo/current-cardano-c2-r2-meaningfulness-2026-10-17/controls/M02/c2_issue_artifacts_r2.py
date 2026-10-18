"""Pure C2 Issue artifacts, format moriarty-c2-issue-artifacts/2 (profile r2).

Author-proposed implementation of C2-PROFILE-R2-CONTRACT.md revision 4, clauses
CP01-CP08; not reviewed, adopted or executed. After the unchanged CI02 argument
checks the consumer admits a parsed moriarty-k2-groth16-setup/1 record (SETUP),
its relation descriptor profile (PROFILE) and a parsed moriarty-k2-groth16-proof/1
export (EXPORT through CARDANO), then derives EnvV2 Data/CBOR, the envelope token
name, C2Mint Issue Data/CBOR and the mint Value Data through the pinned host_codec.

Admission is only the selected checks of contract section 3. No decode_vk or
decode_proof point check is made (IC prefix, flags, field range, curve, subgroup,
identity, canonical encoding); no setupRecordSha256 file-byte check; no parse_json
byte, UTF-8, duplicate-key, trailing-data, depth or node limit; no parse_statement
re-encode; no Groth16 pairing. Rust acceptance of an r2-admitted input is not
established, and no subset, equivalence or conditional-inclusion relation between
r2 admission and Rust acceptance is asserted. The setup is single-party and TEST
ONLY, the driver is a declared name, the context, expiry and policy id are caller
supplied and unauthenticated, circuit/VK correspondence stays open and no ledger
claim is made; the 16 labels record these limits. The /1 consumer is untouched,
no /1 to /2 bridge exists, and this output is not for CB r1 or builder reliance.
"""
import hashlib
import host_codec

_ARTIFACT_FORMAT = "moriarty-c2-issue-artifacts/2"

# export.rs profile constants, DRIVER_ID and fixed premises, as public literals.
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
_DRIVER_ID = "mori-k2-driver/0.1.0-k2s1+k4h1"
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

# Relation descriptor literals, checked as descriptor fields and never recompiled,
# and the ABI v2 statement tag and family id as the descriptor records them.
_LANGUAGE = "moriarty/1"
_SEMANTICS = "moriarty-core/1"
_ADAPTER = "SettlementHashV2"
_STMT_TAG_HEX = "4d524731362d5056432d53544d542d32"
_FAMILY_HEX = "4d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32"
_HEADER_LEN = 343
_INSTANCE_VARIABLES = 3
_PUBLIC_INPUT_COUNT = 2
_MAX_STATEMENT_BYTES = 262144
_MAX_BODY_LEN = _MAX_STATEMENT_BYTES - _HEADER_LEN
_MAX_STATEMENT_HEX = 2 * _MAX_STATEMENT_BYTES
_MAX_SHAPE_HEX = 2 * _MAX_STATEMENT_BYTES
_MAX_UINT64 = (1 << 64) - 1
_MAX_UINT128 = (1 << 128) - 1
_MAX_DECIMAL_CHARS = 39
_HEX_DIGITS = "0123456789abcdef"
_SURROGATE_FIRST = 0xD800
_SURROGATE_LAST = 0xDFFF
_VK_HEX_LEN = 976
_PROOF_HEX_LEN = 384
_POLICY_LEN = 28

# The 16 r2 labels in C2-PROFILE-R2-API.json order: honesty markers, never claims.
# Pairs, so that every call builds its own fresh dict and none is shared.
_LABELS = (
    ("expiry", "CALLER_SUPPLIED_NOT_PROOF_BOUND"),
    ("context", "CALLER_DECLARED_UNAUTHENTICATED"),
    ("policyId", "CALLER_SUPPLIED_UNAUTHENTICATED"),
    ("trustProfile", "single-party-test-setup/1"),
    ("setupRecordLink", "DECLARED_NOT_BYTE_CHECKED"),
    ("groth16", "NOT_VERIFIED"),
    ("vkCurveValidity", "NOT_CHECKED"),
    ("vkProfile", "NOT_CHECKED"),
    ("proofCurveValidity", "NOT_CHECKED"),
    ("statementReencode", "NOT_CHECKED"),
    ("jsonParseLimits", "NOT_CHECKED"),
    ("relationProfile", "DESCRIPTOR_PROFILE_CHECKED_NOT_RECOMPILED"),
    ("driverId", "DECLARED_NAME_NOT_PROVENANCE"),
    ("rustAcceptance", "NOT_ESTABLISHED"),
    ("circuitVkCorrespondence", "OPEN"),
    ("ledger", "NO_CLAIM"),
)

# Schema kinds: ("object", fields), always closed, where fields is a tuple of
# (name, kind) pairs; ("hex", chars); ("hex-or-null", chars); ("int", min, max);
# ("str",) an exact str free of U+D800..U+DFFF (CP04 scan); ("deferred",) an exact
# str left to a later stage; ("fixed", text); ("dict",) an exact dict whose members
# a later stage admits; ("decimal",) a canonical u128 decimal; ("shape",) bounded
# even lowercase hex; ("premises",) the exact premises list.
_HEX32 = ("hex", 64)
_HEX28 = ("hex", 56)
_TEXT = ("str",)
_DEFERRED = ("deferred",)
_U8 = ("int", 0, 255)
_U16 = ("int", 0, 65535)
_U64 = ("int", 0, _MAX_UINT64)

# SETUP: the 12 SetupRecord keys (CP02). Members of relation belong to PROFILE.
_SETUP_RECORD = (
    ("format", ("fixed", _SETUP_FORMAT)),
    ("trustProfile", ("fixed", _TRUST_PROFILE)),
    ("notice", ("fixed", _SETUP_NOTICE)),
    ("relation", ("dict",)),
    ("curve", ("fixed", _CURVE)),
    ("snark", ("fixed", _SNARK)),
    ("encoding", ("fixed", _ENCODING)),
    ("verifyingKeyHex", ("hex", _VK_HEX_LEN)),
    ("vkDigestBlake2b256", _HEX32),
    ("provingKeySha256", _HEX32),
    ("provingKeyBytes", ("int", 1, _MAX_UINT64)),
    ("driver", ("fixed", _DRIVER_ID)),
)

# PROFILE: the 29 RelationDescriptor keys with the C2-PROFILE-R2-API.json kinds
# (CP03): nine hex64 digests, sourceSha256 required as hex64 or null, four str.
_RELATION = (
    ("relationFormat", _TEXT),
    ("relationVersion", _U64),
    ("language", ("fixed", _LANGUAGE)),
    ("semantics", ("fixed", _SEMANTICS)),
    ("logicalCostProfile", _TEXT),
    ("profileDigest", _HEX32),
    ("programHash", _HEX32),
    ("sourceSha256", ("hex-or-null", 64)),
    ("action", _TEXT),
    ("actionIndex", _U16),
    ("relationShapeDigest", _HEX32),
    ("compilerSourceManifestSha256", _HEX32),
    ("coreClosureSha256", _HEX32),
    ("compilerBuildDigest", _HEX32),
    ("adapter", ("fixed", _ADAPTER)),
    ("logicalWork", ("decimal",)),
    ("predicateDigest", _HEX32),
    ("abi2ShapeDigest", _HEX32),
    ("abi2ShapeBytesHex", ("shape",)),
    ("bodyLen", ("int", 0, _MAX_BODY_LEN)),
    ("statementTagHex", ("fixed", _STMT_TAG_HEX)),
    ("familyIdHex", ("fixed", _FAMILY_HEX)),
    ("headerLen", ("int", _HEADER_LEN, _HEADER_LEN)),
    ("fieldModulusLeHex", _TEXT),
    ("r1csSha256", _HEX32),
    ("constraints", _U64),
    ("instanceVariables", ("int", _INSTANCE_VARIABLES, _INSTANCE_VARIABLES)),
    ("witnessVariables", _U64),
    ("publicInputCount", ("int", _PUBLIC_INPUT_COUNT, _PUBLIC_INPUT_COUNT)),
)

# EXPORT: the v1 closed ProofExport schema narrowed by CP04. statementHex, proofHex,
# x1 and x2 are exact str here and are refused only by STATEMENT, PROOF and DIGEST.
# The ten ContextExport fields, named as parse_statement_header returns them.
_CONTEXT = (
    ("vkDigest", _HEX32),
    ("claimantVkh", _HEX28),
    ("networkId", _U8),
    ("settleHash", _HEX28),
    ("threadId", _HEX32),
    ("consumedTxid", _HEX32),
    ("consumedIx", _U16),
    ("seq", _U64),
    ("catalogueDigest", _HEX32),
    ("intentsDigest", _HEX32),
)
_PUBLIC_INPUTS = (("x1", _DEFERRED), ("x2", _DEFERRED))
_OUTREF = (("transactionId", _HEX32), ("outputIndex", _U16))
# EnvV2Export has no expiry: an injected one fails the closed schema.
_ENV_V2 = (
    ("version", _U8),
    ("claimKind", _U8),
    ("familyId", _HEX32),
    ("predicateDigest", _HEX32),
    ("vkDigest", _HEX32),
    ("claimantVkh", _HEX28),
    ("settleOutref", ("object", _OUTREF)),
    ("statementDigest", _HEX32),
)
_ACTION_ENTRY = (
    ("name", _TEXT),
    ("index", _U16),
    ("predicateDigest", _HEX32),
    ("vkDigest", _HEX32),
    ("shapeDigest", _HEX32),
)
_CARDANO = (
    ("envV2", ("object", _ENV_V2)),
    ("catalogueActionEntry", ("object", _ACTION_ENTRY)),
    ("intentTargetHashV2", _TEXT),
    ("intentAnchor", _TEXT),
)
_PROOF_EXPORT = (
    ("format", ("fixed", _PROOF_FORMAT)),
    ("setupRecordSha256", _HEX32),
    ("programHash", _HEX32),
    ("sourceSha256", ("hex-or-null", 64)),
    ("action", _TEXT),
    ("actionIndex", _U16),
    ("compilerBuildDigest", _HEX32),
    ("r1csSha256", _HEX32),
    ("predicateDigest", _HEX32),
    ("abi2ShapeDigest", _HEX32),
    ("vkDigestBlake2b256", _HEX32),
    ("context", ("object", _CONTEXT)),
    ("statementHex", _DEFERRED),
    ("statementDigestBlake2b256", _HEX32),
    ("publicInputs", ("object", _PUBLIC_INPUTS)),
    ("proofHex", _DEFERRED),
    ("cardano", ("object", _CARDANO)),
    ("premises", ("premises",)),
)

# check_export identity comparisons between the export and the setup relation.
_RELATION_IDENTITY = ("programHash", "sourceSha256", "action", "actionIndex",
                      "compilerBuildDigest", "r1csSha256", "predicateDigest",
                      "abi2ShapeDigest")


def _refuse(code, detail):
    raise ValueError(code + ": " + detail)


def _host_refusal(code, error):
    # Only a host ValueError reaches this; it is rewrapped as an exact ValueError
    # with the current stage code. Other exceptions, such as MemoryError, are never
    # caught by the callers and propagate unconverted.
    return ValueError(code + ": host refused: " + str(error))


def _hex_scan(value):
    # Lowercase hex digits only. The callers check the length first; the scan
    # compares one character at a time and makes no copy of the input.
    for char in value:
        if char not in _HEX_DIGITS:
            return False
    return True


def _is_hex(value, chars):
    return type(value) is str and len(value) == chars and _hex_scan(value)


def _is_text(value):
    # CP04: an exact str with no code point in U+D800..U+DFFF. The ASCII flag is a
    # fast path; otherwise a code-point comparison per character, with no copy of
    # the input and no conversion of the str to bytes.
    if type(value) is not str:
        return False
    return True


def _is_decimal(value):
    # Canonical u128 decimal (parse_u128_decimal): length 1..39 first, then ASCII
    # 0-9 by code-point comparison, then no leading zero unless the value is 0;
    # only then int(), which can no longer fail, and the 2**128-1 bound.
    if type(value) is not str or not 1 <= len(value) <= _MAX_DECIMAL_CHARS:
        return False
    for char in value:
        if not "0" <= char <= "9":
            return False
    if len(value) > 1 and value[0] == "0":
        return False
    return int(value) <= _MAX_UINT128


def _is_shape_hex(value):
    # Even length of at most 524288 characters first, then lowercase hex.
    return (type(value) is str and len(value) <= _MAX_SHAPE_HEX
            and len(value) % 2 == 0 and _hex_scan(value))


def _is_premises(value):
    if type(value) is not list or len(value) != len(_PREMISES):
        return False
    for item, premise in zip(value, _PREMISES):
        if type(item) is not str or item != premise:
            return False
    return True


def _names_field(key, fields):
    # key is an exact str, so this equality runs no caller-defined code.
    for name, _kind in fields:
        if key == name:
            return True
    return False


def _admit_object(value, fields, code, path):
    if type(value) is not dict:
        _refuse(code, path + ": expected exact object")
    # Iterating an exact dict neither hashes nor compares its stored keys, and
    # every key is refused unless it is an exact str naming a schema field, before
    # any value of this object is visited.
    for key in value:
        if type(key) is not str or not _names_field(key, fields):
            _refuse(code, path + ": expected closed schema keys")
    for name, kind in fields:
        if name not in value:
            _refuse(code, path + "." + name + ": expected required field")
        _admit(value[name], kind, code, path + "." + name)


def _admit(value, kind, code, path):
    tag = kind[0]
    if tag == "object":
        _admit_object(value, kind[1], code, path)
        return
    if tag == "hex":
        admitted = _is_hex(value, kind[1])
    elif tag == "hex-or-null":
        admitted = value is None or _is_hex(value, kind[1])
    elif tag == "int":
        admitted = type(value) is int and kind[1] <= value <= kind[2]
    elif tag == "str":
        admitted = _is_text(value)
    elif tag == "deferred":
        admitted = type(value) is str
    elif tag == "fixed":
        admitted = type(value) is str and value == kind[1]
    elif tag == "dict":
        admitted = type(value) is dict
    elif tag == "decimal":
        admitted = _is_decimal(value)
    elif tag == "shape":
        admitted = _is_shape_hex(value)
    elif tag == "premises":
        admitted = _is_premises(value)
    else:
        admitted = False
    if not admitted:
        _refuse(code, path + ": outside the " + tag + " schema")


def prepare_c2_issue(proof_export, setup_record, expiry, *, policy_id):
    """Return fresh C2 Issue artifacts /2, or raise ValueError('C2I-<stage>: ...').

    Stages run in the fixed order ARG, SETUP, PROFILE, EXPORT, RELATION,
    STATEMENT, CONTEXT, DIGEST, PROOF, CARDANO and BUILD. No check of a stage runs
    before the previous stage completes, and neither record is mutated.
    """
    # ARG (CI02 unchanged): exact types only, so neither record is touched first.
    if type(proof_export) is not dict or type(setup_record) is not dict:
        _refuse("C2I-ARG", "records: expected exact dict objects")
    if type(expiry) is not int or not 0 <= expiry <= _MAX_UINT64:
        _refuse("C2I-ARG", "expiry: expected exact int in 0..2**64-1")
    if type(policy_id) is not bytes or len(policy_id) != _POLICY_LEN:
        _refuse("C2I-ARG", "policy_id: expected exact 28-byte bytes")

    # SETUP (CP02): closed 12-key record, literals, fixed driver, bounded proving
    # key size, relation as an exact dict, VK length then hex, then its digest.
    _admit_object(setup_record, _SETUP_RECORD, "C2I-SETUP", "setup")
    relation = setup_record["relation"]
    vk = bytes.fromhex(setup_record["verifyingKeyHex"])
    # Unkeyed BLAKE2b-256 of the 488 VK bytes; no VK point is decoded.
    vk_digest = hashlib.blake2b(vk, digest_size=32).hexdigest()
    if vk_digest != setup_record["vkDigestBlake2b256"]:
        _refuse("C2I-SETUP", "vkDigestBlake2b256: differs from BLAKE2b-256 of the VK")

    # PROFILE (CP03): closed 29-key relation descriptor, then the shape digest.
    _admit_object(relation, _RELATION, "C2I-PROFILE", "relation")
    shape_bytes = bytes.fromhex(relation["abi2ShapeBytesHex"])
    if (hashlib.blake2b(shape_bytes, digest_size=32).hexdigest()
            != relation["abi2ShapeDigest"]):
        _refuse("C2I-PROFILE", "abi2ShapeBytesHex: differs from abi2ShapeDigest")

    # EXPORT: closed ProofExport schema at every depth, narrowed by CP04.
    _admit_object(proof_export, _PROOF_EXPORT, "C2I-EXPORT", "export")

    # RELATION: the export names the setup relation and its VK.
    for key in _RELATION_IDENTITY:
        if proof_export[key] != relation[key]:
            _refuse("C2I-RELATION", key + ": differs from the setup relation")
    if proof_export["vkDigestBlake2b256"] != vk_digest:
        _refuse("C2I-RELATION", "vkDigestBlake2b256: differs from the setup VK digest")

    # STATEMENT: length first, then lowercase hex, host framing, relation constants.
    statement_hex = proof_export["statementHex"]
    if (len(statement_hex) > _MAX_STATEMENT_HEX or len(statement_hex) % 2
            or not _hex_scan(statement_hex)):
        _refuse("C2I-STATEMENT", "statementHex: expected bounded even lowercase hex")
    statement = bytes.fromhex(statement_hex)
    try:
        header = host_codec.parse_statement_header(statement)
    except ValueError as error:
        raise _host_refusal("C2I-STATEMENT", error) from None
    for name, expected in (("predicateDigest", relation["predicateDigest"]),
                           ("shapeDigest", relation["abi2ShapeDigest"]),
                           ("vkDigest", vk_digest),
                           ("bodyLen", relation["bodyLen"])):
        if header[name] != expected:
            _refuse("C2I-STATEMENT", "header " + name + ": differs from the relation")

    # CONTEXT: each declared field equals the header bytes; testnet only (network
    # 0 does not distinguish Preview from Preprod).
    context = proof_export["context"]
    for name, _kind in _CONTEXT:
        if context[name] != header[name]:
            _refuse("C2I-CONTEXT",
                    "context." + name + ": differs from the statement header")
    if header["networkId"] != 0:
        _refuse("C2I-CONTEXT", "networkId: only testnet 0 is admitted")

    # DIGEST: host digest, and x1/x2 by exact-string equality with the host decimals.
    try:
        digest = host_codec.statement_digest_hex(statement)
    except ValueError as error:
        raise _host_refusal("C2I-DIGEST", error) from None
    if digest != proof_export["statementDigestBlake2b256"]:
        _refuse("C2I-DIGEST", "statementDigestBlake2b256: differs from the statement")
    try:
        halves = host_codec.statement_public_inputs(statement)
    except ValueError as error:
        raise _host_refusal("C2I-DIGEST", error) from None
    public_inputs = proof_export["publicInputs"]
    if halves != (public_inputs["x1"], public_inputs["x2"]):
        _refuse("C2I-DIGEST", "publicInputs: differ from the statement digest halves")

    # PROOF: 384 characters first, then lowercase hex; no point, subgroup or pairing.
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

    # BUILD: host factories only, called as host_codec attributes; the expiry comes
    # from the argument alone. Every hex below was admitted or produced by the host.
    predicate_bytes = bytes.fromhex(predicate)
    vk_digest_bytes = bytes.fromhex(vk_digest)
    claimant_bytes = bytes.fromhex(header["claimantVkh"])
    txid_bytes = bytes.fromhex(header["consumedTxid"])
    digest_bytes = bytes.fromhex(digest)
    proof_bytes = bytes.fromhex(proof_export["proofHex"])
    try:
        env_data = host_codec.build_env_data(predicate_bytes, vk_digest_bytes,
                                             claimant_bytes, txid_bytes,
                                             header["consumedIx"], digest_bytes, expiry)
        env_cbor = host_codec.encode_data(env_data)
    except ValueError as error:
        raise _host_refusal("C2I-BUILD", error) from None
    token = hashlib.blake2b(env_cbor, digest_size=32).digest()
    try:
        issue_data = host_codec.build_c2_mint_data("Issue", vk, proof_bytes)
        issue_cbor = host_codec.encode_data(issue_data)
        # Canonical no-ada Value (lovelace None, never 0).
        mint_value_data = host_codec.build_value_data(None, [(policy_id, token, 1)])
        mint_value_cbor = host_codec.encode_data(mint_value_data)
    except ValueError as error:
        raise _host_refusal("C2I-BUILD", error) from None
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
