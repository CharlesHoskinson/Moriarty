"""Provisional pure PlutusData encoder for the frozen serialise_data profile."""

_MAX_DEPTH = 128
_MAX_VISITS = 65536
_MAX_LEAF_BYTES = 65536
_MAX_OUTPUT_BYTES = 1048576
_MAX_UINT64 = (1 << 64) - 1


def _append(out, fragment):
    if len(fragment) > _MAX_OUTPUT_BYTES - len(out):
        raise ValueError("encoded data exceeds the output bound")
    out.extend(fragment)


def _head(major, value):
    if value < 24:
        return bytes((major | value,))
    if value <= 255:
        width, marker = 1, 24
    elif value <= 65535:
        width, marker = 2, 25
    elif value <= 4294967295:
        width, marker = 4, 26
    else:
        width, marker = 8, 27
    return bytes((major | marker,)) + int.to_bytes(value, width, "big")


def _write_bytes(out, value):
    size = len(value)
    if size > _MAX_LEAF_BYTES:
        raise ValueError("bytes or integer magnitude exceeds the leaf bound")
    if size <= 64:
        _append(out, _head(0x40, size))
        _append(out, value)
        return
    _append(out, b"\x5f")
    for offset in range(0, size, 64):
        chunk = value[offset:offset + 64]
        _append(out, _head(0x40, len(chunk)))
        _append(out, chunk)
    _append(out, b"\xff")


def _encode(data):
    out = bytearray()
    visits = 0
    # A continuation retains only one next child, even for a broad container.
    # Kind 0 visits a node; kinds 1 and 2 resume an array or map respectively.
    pending = [(0, data, 1, 0)]
    while pending:
        kind, value, depth, index = pending.pop()
        if kind == 1:
            if index == len(value):
                _append(out, b"\xff")
            else:
                pending.append((1, value, depth, index + 1))
                pending.append((0, value[index], depth, 0))
            continue
        if kind == 2:
            if index < 2 * len(value):
                pair = value[index // 2]
                if type(pair) is not tuple or len(pair) != 2:
                    raise ValueError("map entries must be exact pairs")
                pending.append((2, value, depth, index + 1))
                pending.append((0, pair[index % 2], depth, 0))
            continue

        visits += 1
        if visits > _MAX_VISITS:
            raise ValueError("data exceeds the expanded node visit bound")
        if depth > _MAX_DEPTH:
            raise ValueError("data exceeds the depth bound")
        if type(value) is not tuple or not value or type(value[0]) is not str:
            raise ValueError("a data node must be an exact tagged tuple")

        tag = value[0]
        if tag == "int":
            if len(value) != 2 or type(value[1]) is not int:
                raise ValueError("an integer node requires an exact integer")
            integer = value[1]
            if int.bit_length(integer) > _MAX_LEAF_BYTES * 8 + 1:
                raise ValueError("integer magnitude exceeds the leaf bound")
            magnitude = integer if integer >= 0 else -1 - integer
            if magnitude <= _MAX_UINT64:
                _append(out, _head(0x00 if integer >= 0 else 0x20, magnitude))
            else:
                size = (int.bit_length(magnitude) + 7) // 8
                if size > _MAX_LEAF_BYTES:
                    raise ValueError("integer magnitude exceeds the leaf bound")
                _append(out, b"\xc2" if integer >= 0 else b"\xc3")
                _write_bytes(out, int.to_bytes(magnitude, size, "big"))
            continue
        if tag == "bytes":
            if len(value) != 2 or type(value[1]) is not bytes:
                raise ValueError("a bytes node requires exact bytes")
            _write_bytes(out, value[1])
            continue
        if tag == "map":
            if len(value) != 2 or type(value[1]) is not list:
                raise ValueError("a map node requires an exact list of pairs")
            pairs = value[1]
            if len(pairs) > (_MAX_VISITS - visits) // 2:
                raise ValueError("map exceeds the expanded node visit bound")
            _append(out, _head(0xa0, len(pairs)))
            if pairs:
                pending.append((2, pairs, depth + 1, 0))
            continue
        if tag == "list":
            if len(value) != 2 or type(value[1]) is not list:
                raise ValueError("a list node requires an exact list")
            children = value[1]
        elif tag == "constr":
            if (len(value) != 3 or type(value[1]) is not int
                    or not 0 <= value[1] <= _MAX_UINT64
                    or type(value[2]) is not list):
                raise ValueError("a constructor requires a uint64 index and exact list")
            constructor = value[1]
            children = value[2]
            if constructor <= 6:
                _append(out, _head(0xc0, 121 + constructor))
            elif constructor <= 127:
                _append(out, _head(0xc0, 1280 + constructor - 7))
            else:
                _append(out, _head(0xc0, 102))
                _append(out, b"\x82")
                _append(out, _head(0x00, constructor))
        else:
            raise ValueError("unknown data tag")

        if len(children) > _MAX_VISITS - visits:
            raise ValueError("array exceeds the expanded node visit bound")
        if children:
            _append(out, b"\x9f")
            pending.append((1, children, depth + 1, 0))
        else:
            _append(out, b"\x80")
    return bytes(out)


def encode_data(data):
    """Return complete CBOR bytes, or reject data outside the frozen contract."""
    # Environmental failures such as MemoryError propagate unconverted (contract I2).
    return _encode(data)


# Provisional Rust request-wire APIs. These validate structure only; Core/Rust
# owns program semantics, and caller-supplied context is not authenticated.


def _wire_keys(value, path):
    if type(value) is not dict:
        raise ValueError(path + ": expected exact object")
    # Iterating an exact dict does not hash or compare its stored keys.
    for key in value:
        if type(key) is not str:
            raise ValueError(path + ": expected exact string keys")


def _wire_record(value, path, fields, optional=()):
    _wire_keys(value, path)
    # Refuse all unknown fields before looking at any of this object's values.
    for key in value:
        if key not in fields:
            raise ValueError(path + ": expected closed schema object")
    for key in fields:
        if key not in optional and key not in value:
            raise ValueError(path + "." + key + ": expected required field")
    return {key: fields[key](item, path + "." + key)
            for key, item in value.items()}


def _wire_tagged(value, path, tag, variants):
    _wire_keys(value, path)
    # Determine the admissible complete field shapes from keys alone. This
    # rejects unknown fields without visiting values, including the tag value.
    names = []
    fields = None
    for name, schema in variants.items():
        if len(value) == len(schema) and all(key in schema for key in value):
            names.append(name)
            fields = schema
    if fields is None:
        raise ValueError(path + ": expected closed tagged schema object")
    kind = value[tag]
    if type(kind) is not str or kind not in names:
        raise ValueError(path + "." + tag + ": expected schema tag string")
    return _wire_record(value, path, variants[kind])


def _wire_text(value, path):
    if type(value) is not str:
        raise ValueError(path + ": expected exact UTF-8 string")
    try:
        value.encode("utf-8")
    except UnicodeEncodeError:
        raise ValueError(path + ": expected exact UTF-8 string") from None
    return value


def _wire_action(value, path):
    if type(value) is not str or not 1 <= len(value) <= 1024:
        raise ValueError(path + ": expected action string of 1..1024 UTF-8 bytes")
    try:
        size = len(value.encode("utf-8"))
    except UnicodeEncodeError:
        raise ValueError(path + ": expected action string of 1..1024 UTF-8 bytes") from None
    if size > 1024:
        raise ValueError(path + ": expected action string of 1..1024 UTF-8 bytes")
    return value


def _wire_uint(value, path):
    if (type(value) is not str or not 1 <= len(value) <= 39
            or any(char < "0" or char > "9" for char in value)
            or (len(value) > 1 and value[0] == "0")):
        raise ValueError(path + ": expected canonical u128 decimal string")
    if int(value) > (1 << 128) - 1:
        raise ValueError(path + ": expected canonical u128 decimal string")
    return value


def _wire_bool(value, path):
    if type(value) is not bool:
        raise ValueError(path + ": expected exact boolean")
    return value


def _wire_integer(value, path, maximum):
    if type(value) is not int or not 0 <= value <= maximum:
        raise ValueError(path + ": expected unsigned integer in schema range")
    return value


def _wire_u8(value, path):
    return _wire_integer(value, path, 255)


def _wire_u16(value, path):
    return _wire_integer(value, path, 65535)


def _wire_u64(value, path):
    return _wire_integer(value, path, (1 << 64) - 1)


def _wire_hex(value, path, size):
    if (type(value) is not str or len(value) != size * 2
            or any(char not in "0123456789abcdef" for char in value)):
        raise ValueError(path + ": expected exact lowercase hexadecimal string")
    return value


def _wire_hex28(value, path):
    return _wire_hex(value, path, 28)


def _wire_hex32(value, path):
    return _wire_hex(value, path, 32)


def _wire_optional_hex32(value, path):
    if value is None:
        return None
    return _wire_hex32(value, path)


def _wire_value(value, path):
    return _wire_tagged(value, path, "type", {
        "UInt": {"type": _wire_text, "value": _wire_uint},
        "Bool": {"type": _wire_text, "value": _wire_bool},
    })


def _wire_map(value, path):
    _wire_keys(value, path)
    for key in value:
        _wire_text(key, path + ".*")
    return {key: _wire_value(item, path + ".*")
            for key, item in value.items()}


def _wire_array(value, path, item_validator):
    if type(value) is not list:
        raise ValueError(path + ": expected exact array")
    return [item_validator(item, path + "[]") for item in value]


def _wire_asset(value, path):
    return _wire_record(value, path, {
        "network": _wire_text,
        "issuer": _wire_text,
        "reference": _wire_text,
        "scale": _wire_u8,
    })


def _wire_balance(value, path):
    return _wire_record(value, path, {
        "account": _wire_text,
        "asset": _wire_asset,
        "amount": _wire_uint,
    })


def _wire_liability(value, path):
    return _wire_record(value, path, {
        "name": _wire_text,
        "asset": _wire_asset,
        "debtor": _wire_text,
        "creditor": _wire_text,
        "amount": _wire_uint,
    })


def _wire_balances(value, path):
    return _wire_array(value, path, _wire_balance)


def _wire_liabilities(value, path):
    return _wire_array(value, path, _wire_liability)


def _wire_snapshot(value, path):
    return _wire_record(value, path, {
        "state": _wire_map,
        "balances": _wire_balances,
        "liabilities": _wire_liabilities,
    })


def _wire_scenario(value, path):
    return _wire_record(value, path, {
        "state": _wire_map,
        "inputs": _wire_map,
        "balances": _wire_balances,
        "liabilities": _wire_liabilities,
    })


def _wire_effect(value, path):
    movement = {
        "kind": _wire_text,
        "asset": _wire_asset,
        "from": _wire_text,
        "to": _wire_text,
        "amount": _wire_uint,
    }
    return _wire_tagged(value, path, "kind", {
        "State": {
            "kind": _wire_text,
            "name": _wire_text,
            "before": _wire_value,
            "after": _wire_value,
        },
        "Transfer": movement,
        "Fee": movement,
        "Liability": {
            "kind": _wire_text,
            "name": _wire_text,
            "asset": _wire_asset,
            "debtor": _wire_text,
            "creditor": _wire_text,
            "before": _wire_uint,
            "after": _wire_uint,
        },
        "Emit": {"kind": _wire_text, "name": _wire_text, "value": _wire_value},
    })


def _wire_effects(value, path):
    return _wire_array(value, path, _wire_effect)


def _wire_transition(value, path):
    return _wire_tagged(value, path, "kind", {
        "CoreReference": {"kind": _wire_text, "scenario": _wire_scenario},
        "Records": {
            "kind": _wire_text,
            "inputs": _wire_map,
            "pre": _wire_snapshot,
            "post": _wire_snapshot,
            "effects": _wire_effects,
        },
    })


def _wire_context(value, path):
    return _wire_record(value, path, {
        "claimantVkh": _wire_hex28,
        "networkId": _wire_u8,
        "settleHash": _wire_hex28,
        "threadId": _wire_hex32,
        "consumedTxid": _wire_hex32,
        "consumedIx": _wire_u16,
        "seq": _wire_u64,
        "catalogueDigest": _wire_hex32,
        "intentsDigest": _wire_hex32,
        "vkDigest": _wire_optional_hex32,
    }, ("vkDigest",))


def _wire_selector(value, path):
    result = _wire_record(value, path, {
        "format": _wire_text,
        "language": _wire_text,
        "semantics": _wire_text,
        "action": _wire_action,
        "adapter": _wire_text,
        "cap": _wire_text,
        "accounting": _wire_text,
        "frame": _wire_text,
        "expectedSourceSha256": _wire_optional_hex32,
        "expectedProgramHash": _wire_optional_hex32,
    }, ("expectedSourceSha256", "expectedProgramHash"))
    expected = {
        "format": "moriarty-k2-relation-selector/1",
        "language": "moriarty/1",
        "semantics": "moriarty-core/1",
        "adapter": "SettlementHashV2",
        "cap": "NotBound",
        "accounting": "GrossDebitAndFees",
        "frame": "TouchedOnly",
    }
    for key, literal in expected.items():
        if result[key] != literal:
            raise ValueError(path + "." + key + ": expected fixed schema string")
    return result


def _wire_request(value, path):
    result = _wire_record(value, path, {
        "format": _wire_text,
        "selector": _wire_selector,
        "transition": _wire_transition,
        "context": _wire_context,
    })
    if result["format"] != "moriarty-k2-prove-request/1":
        raise ValueError(path + ".format: expected fixed schema string")
    return result


def build_prove_request(action, transition, context, expected_source_sha256=None,
                        expected_program_hash=None):
    """Return a detached structural Rust request; evaluate no financial rules."""
    selector = {
        "format": "moriarty-k2-relation-selector/1",
        "language": "moriarty/1",
        "semantics": "moriarty-core/1",
        "action": _wire_action(action, "request.selector.action"),
        "adapter": "SettlementHashV2",
        "cap": "NotBound",
        "accounting": "GrossDebitAndFees",
        "frame": "TouchedOnly",
    }
    if expected_source_sha256 is not None:
        selector["expectedSourceSha256"] = _wire_hex32(
            expected_source_sha256, "request.selector.expectedSourceSha256")
    if expected_program_hash is not None:
        selector["expectedProgramHash"] = _wire_hex32(
            expected_program_hash, "request.selector.expectedProgramHash")
    request = {
        "format": "moriarty-k2-prove-request/1",
        "selector": selector,
        "transition": _wire_transition(transition, "request.transition"),
        "context": _wire_context(context, "request.context"),
    }
    validate_prove_request(request)
    return request


def validate_prove_request(request):
    """Check wire structure only, without mutating or authenticating input."""
    _wire_request(request, "request")
    return True


def validate_context_input(context):
    """Return detached context labelled as caller supplied and unauthenticated."""
    result = _wire_context(context, "context")
    result["authentication"] = "CALLER_SUPPLIED_UNAUTHENTICATED"
    return result
