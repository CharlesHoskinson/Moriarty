"""Carry an actual ground NGState between K queries without interpreting it.

This module launches nothing and reads no expected data. All state bytes come
from the final argument of a completed actual ngTape, never from its JSON wire.
"""
import hashlib
import json

OBSERVE = b"LblngObserve'LParUndsCommUndsRParUnds'SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS'Unds'NGTape'Unds'NGState'Unds'Int{}"
TAPE = b"LblngTape'LParUndsCommUndsCommUndsRParUnds'SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS'Unds'NGTape'Unds'NGState'Unds'NGStates'Unds'NGState{}"
STATE = b"LblngState'LParUndsCommUndsCommUndsCommUndsCommUndsRParUnds'SHERLOCK-NOMINAL-PRODUCT-SYNTAX'Unds'NGState'Unds'NGProgram'Unds'NGControl'Unds'Map'Unds'NGFrames'Unds'NGEvents{}"
INT = b"\\dv{SortInt{}}"


def _trim(data, left, right):
    while left < right and data[left] in b" \t\r\n":
        left += 1
    while right > left and data[right - 1] in b" \t\r\n":
        right -= 1
    return left, right


def application(data):
    """Return the exact head and argument byte spans of one Kore application.

    Parentheses/commas within strings and sort parameters are not delimiters.
    The ground term's children stay opaque; K remains its typed verifier.
    """
    if type(data) is not bytes:
        raise TypeError("Kore input must be bytes")
    left, right = _trim(data, 0, len(data))
    brace = 0
    opening = None
    for pos in range(left, right):
        char = data[pos]
        if char == 123:
            brace += 1
        elif char == 125:
            brace -= 1
            if brace < 0:
                raise ValueError("Unbalanced sort parameters")
        elif char == 40 and brace == 0:
            opening = pos
            break
    if opening is None or opening == left:
        raise ValueError("Expected one Kore application")
    head = data[left:opening].strip()
    depth, brace, quoted, escaped = 1, 0, False, False
    start, spans = opening + 1, []
    for pos in range(start, right):
        char = data[pos]
        if quoted:
            if escaped:
                escaped = False
            elif char == 92:
                escaped = True
            elif char == 34:
                quoted = False
            continue
        if char == 34:
            quoted = True
        elif char == 123:
            brace += 1
        elif char == 125:
            brace -= 1
            if brace < 0:
                raise ValueError("Unbalanced sort parameters")
        elif char == 40:
            depth += 1
        elif char == 41:
            depth -= 1
            if depth == 0:
                if brace or data[pos + 1:right].strip():
                    raise ValueError("Trailing term or unbalanced sorts")
                span = _trim(data, start, pos)
                if span[0] != span[1]:
                    spans.append(span)
                elif spans:
                    raise ValueError("Empty final argument")
                return head, spans
        elif char == 44 and depth == 1 and brace == 0:
            span = _trim(data, start, pos)
            if span[0] == span[1]:
                raise ValueError("Empty argument")
            spans.append(span)
            start = pos + 1
    raise ValueError("Unterminated application or string")


def arguments(data, head, arity):
    actual, spans = application(data)
    if actual != head or len(spans) != arity:
        raise ValueError("Unexpected Kore carrier or arity")
    return [data[left:right] for left, right in spans], spans


def _bound(value):
    if type(value) is not int or value < 0:
        raise ValueError("Observation bound must be a nonnegative integer")
    return INT + b'("' + str(value).encode("ascii") + b'")'


def seed_query(passing_kast, bound):
    """Replace only the outer observer bound of already verified query bytes."""
    parts, spans = arguments(passing_kast, OBSERVE, 2)
    old, _ = arguments(parts[1], INT, 1)
    if type(json.loads(old[0])) is not str:
        raise ValueError("Expected an integer domain value string")
    left, right = spans[1]
    result = passing_kast[:left] + _bound(bound) + passing_kast[right:]
    assert arguments(result, OBSERVE, 2)[0][0] == parts[0]
    return result


def resume_query(actual_tape_kore, bound):
    """Use the complete actual final state verbatim as the next initial state."""
    parts, spans = arguments(actual_tape_kore, TAPE, 3)
    final = parts[2]
    arguments(final, STATE, 5)
    result = OBSERVE + b"(" + final + b"," + _bound(bound) + b")"
    assert arguments(result, OBSERVE, 2)[0][0] == final
    return result, {
        "source": "third argument of actual raw ngTape",
        "actual_tape_sha256": hashlib.sha256(actual_tape_kore).hexdigest(),
        "actual_final_byte_start": spans[2][0],
        "actual_final_byte_end_exclusive": spans[2][1],
        "actual_final_bytes": len(final),
        "actual_final_sha256": hashlib.sha256(final).hexdigest(),
        "next_query_sha256": hashlib.sha256(result).hexdigest(),
        "all_five_NGState_payloads_preserved_verbatim": True,
        "state_generated_from_JSON_or_expected": False,
    }
