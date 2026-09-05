# Candidate A Streaming JSON Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans. Execute only the root-assigned task; root owns independent review, evidence admission and commits.

**Goal:** Provide a strict standard-library-only streaming JSON transport primitive without importing or implementing authority, agreement, producer or checker semantics.

**Architecture:** A strict incremental UTF-8 reader parses one top-level object's selected array item at a time. The standard JSON decoder validates each bounded item with duplicate-key and nonfinite-number rejection; an incremental writer and chunked SHA256 helper avoid whole-document strings. Producer and checker may share this transport utility only; their schemas, fixed inventories, semantic expected records and transition judgments remain independent.

**Tech Stack:** Existing pinned Python3.13 environment and standard library only. No new package, tool upgrade, Node/Quint patch or semantic-source change.

## Global constraints

This is PLAN ONLY until root admits its complete code and controls. It implements CT007/008/009 of the case-sharded addendum without changing schema3 package fields, schema2 inventory bytes/hash,78cases/1557events or any semantic obligation. Source ownership is exactly new `scripts/a4_json_stream.py` and new `tests/test_a4_json_stream.py`. No prior checker/producer source is edited in these utility tasks. Integration into their bounded task plans is separate.

Shared transport means producer/checker are not independent at the JSON lexical/Unicode utility layer. This dependency must be disclosed in final reports and pinned in both complete source closures. No semantic function, fixture, oracle, event inventory or expected state may be imported by this module or its tests. Tests use literal synthetic JSON, not accepted package data masquerading as generator evidence.

## Requirements and OpenSpec acceptance scenarios

**ST001 — Bounded selected array.** When a caller selects a top-level array field, the reader SHALL yield each element independently, retaining no preceding element. Scenario: WHEN the caller consumes the first item of a long stream, THEN the reader has not read the full document and no later item is required in memory.

**ST002 — Strict object fields.** When duplicate top-level or nested object keys occur, the reader SHALL reject them, including keys whose JSON escapes decode to the same string. Scenario: WHEN `"x"` and `"\u0078"` occur in the same object, THEN duplicate-key failure occurs regardless of chunk boundaries.

**ST003 — Strict UTF-8 and JSON syntax.** The reader SHALL reject malformed/truncated UTF-8, BOM-prefixed input, incomplete containers/strings, illegal controls, invalid escapes, invalid numbers, nonfinite values, trailing commas and trailing non-whitespace data. Scenario: WHEN invalid UTF-8 is split between chunks or trailing garbage follows the root object, THEN completion is not emitted.

**ST004 — Numeric type preservation.** For valid JSON, integers SHALL remain Python int, decimal/exponent numbers float, and booleans bool; float overflow SHALL be rejected. Scenario: WHEN an item contains `0,1.0,true,1e2`, THEN decoded types are int,float,bool,float; leading-zero integers and Infinity/NaN are rejected. This utility does not apply authority-specific bigint or finite-domain rules.

**ST005 — Complete document judgment.** The reader SHALL require exactly one selected field, whose value is an array, and emit its end event only after the entire root object and trailing whitespace are consumed. Other top-level fields may precede or follow the selected array and SHALL be yielded in source order. Scenario: WHEN a duplicate field or malformed suffix occurs after valid array items, THEN an exception occurs and no end event indicates success.

**ST006 — Incremental output and hashing.** The writer SHALL serialize array elements incrementally with allow_nan=False and UTF-8, preserve item ordering, reject duplicate/reserved top-level fields, and handle short binary writes. Hashing SHALL read fixed positive chunks and return SHA256 plus byte count. Scenario: WHEN the sink accepts only three bytes per write, THEN the resulting document still parses identically and its digest equals the reference bytes.

**ST007 — Explicit resource limits.** When a single value exceeds the configured character limit, the reader SHALL raise ResourceLimit rather than pretend the data is semantically invalid. Scenario: WHEN a small configured limit is exceeded, THEN ResourceLimit is distinguishable from ordinary JSON failure and cannot count as a semantic mutant kill. The default limit is64Mi characters per value, configurable by the caller; changing this limit never permits case omission.

**ST008 — Transport-only independence.** The utility SHALL import only Python standard-library modules. Scenario: WHEN its AST imports are inspected, THEN there are no `scripts`, `moriarty`, producer, checker or Quint imports. Both consumers SHALL perform their own schema/semantic judgments after decoding.

**ST009 — Partial work is not acceptance.** A writer error may leave a partial staging stream; the caller SHALL not publish it as an admitted artifact. A reader consumer SHALL exhaust through the end event before declaring complete success. Scenario: WHEN a caller has observed one valid item followed by malformed JSON, THEN it cannot report a complete valid document.

## File structure and exact interfaces

`scripts/a4_json_stream.py` owns lexical streaming, no file-path authorization:

```text
JsonStreamError(ValueError)
ResourceLimit(JsonStreamError)
iter_array_document(stream: BinaryIO, target: str, *, chunk_size: int=65536,
                    max_value_chars: int=67108864) -> Iterator[tuple[str, str|int, object]]
write_array_document(stream: BinaryIO, target: str, items: Iterable[object], *,
                     before: Iterable[tuple[str,object]]=(),
                     after: Iterable[tuple[str,object]]=()) -> int
hash_stream(stream: BinaryIO, *, chunk_size: int=1048576) -> tuple[str,int]
```

Reader events are exactly `('field', name, decoded_value)`, `('item', zero_based_index, decoded_value)`, and one final `('end', target, item_count)`. The selected field itself is represented by item/end events, not a materialized array. Empty selected arrays produce end count0. The caller chooses acceptable field names and exact counts independently. Hash/write streams start at the caller's current position; helpers do not seek, close, open or authorize paths. The caller owns safe paths, exclusive staging files, pins and atomic publication.

The reader permits JSON-escaped unpaired surrogate code points as Python's standard decoder does; such strings are not valid UTF-8 writer output with ensure_ascii=False and therefore the writer rejects them explicitly as an encoding error. This is not an accepted authority identifier. Invalid actual UTF-8 bytes are always rejected. Unicode normalization is never performed.

### Task1: strict selected-array reader

**Files:** create both owned paths. **Consumes:** binary input and selected field name only. **Produces:** reader API and error classes above. Root archives the original test/source/environment closure for each command; reuse the existing Task1 Python environment by hash, with fresh bytecode paths.

- [ ] Add the reader tests below first. Run the explicit test file and retain the expected missing-module diagnostic; it is not behavioral RED.
- [ ] Add the complete reader code below with the single prescribed compiling RED: remove `if key in out: raise JsonStreamError('duplicate object key')` from `_object`. Run the focused escaped duplicate-key test, which must fail with DID NOT RAISE after imports succeed. Preserve original bytes and terminal streams.
- [ ] Restore that single line, run all reader tests, and obtain independent source/receipt review before Task2. Do not modify tests to accept duplicate data.

Complete reader source:

```python
import codecs
import hashlib
import json
import math
from typing import BinaryIO, Iterable, Iterator


class JsonStreamError(ValueError):
    pass


class ResourceLimit(JsonStreamError):
    pass


def _object(pairs):
    out = {}
    for key, value in pairs:
        if key in out: raise JsonStreamError('duplicate object key')
        out[key] = value
    return out


def _constant(value):
    raise JsonStreamError('nonfinite JSON number')


def _float(value):
    result = float(value)
    if not math.isfinite(result): raise JsonStreamError('nonfinite JSON number')
    return result


def _int(value):
    try:
        return int(value)
    except ValueError as error:
        raise ResourceLimit('integer conversion limit') from error


def _decode(text):
    try:
        return json.loads(text, object_pairs_hook=_object,
                          parse_constant=_constant, parse_float=_float, parse_int=_int)
    except RecursionError as error:
        raise ResourceLimit('JSON nesting limit') from error
    except json.JSONDecodeError as error:
        raise JsonStreamError('invalid JSON value') from error


class _Reader:
    def __init__(self, stream, chunk_size, max_value_chars):
        if type(chunk_size) is not int or chunk_size <= 0:
            raise JsonStreamError('positive chunk size required')
        if type(max_value_chars) is not int or max_value_chars <= 0:
            raise JsonStreamError('positive value limit required')
        self.stream = stream
        self.chunk_size = chunk_size
        self.limit = max_value_chars
        self.decoder = codecs.getincrementaldecoder('utf-8')('strict')
        self.buffer = ''
        self.position = 0
        self.eof = False

    def peek(self):
        while self.position == len(self.buffer) and not self.eof:
            data = self.stream.read(self.chunk_size)
            if type(data) is not bytes: raise JsonStreamError('binary input required')
            self.eof = data == b''
            try:
                self.buffer = self.decoder.decode(data, final=self.eof)
            except UnicodeDecodeError as error:
                raise JsonStreamError('invalid UTF-8') from error
            self.position = 0
        return self.buffer[self.position] if self.position < len(self.buffer) else ''

    def take(self):
        value = self.peek()
        if value: self.position += 1
        return value

    def space(self):
        while self.peek() and self.peek() in ' \t\r\n': self.take()

    def expect(self, character):
        self.space()
        if self.take() != character: raise JsonStreamError('expected '+character)

    def value(self):
        self.space()
        first = self.peek()
        if not first: raise JsonStreamError('missing JSON value')
        chars = []

        def append(character):
            chars.append(character)
            if len(chars) > self.limit: raise ResourceLimit('JSON value character limit')

        if first in '{[' or first == '"':
            stack = []
            quoted = False
            escaped = False
            while True:
                character = self.take()
                if not character: raise JsonStreamError('incomplete JSON value')
                append(character)
                if quoted:
                    if escaped: escaped = False
                    elif character == '\\': escaped = True
                    elif character == '"': quoted = False
                elif character == '"': quoted = True
                elif character in '{[': stack.append(character)
                elif character in '}]':
                    if not stack or stack.pop() != ('{' if character == '}' else '['):
                        raise JsonStreamError('mismatched JSON container')
                if not quoted and not stack: break
        else:
            while self.peek() and self.peek() not in ' \t\r\n,]}': append(self.take())
        return _decode(''.join(chars))


def iter_array_document(stream: BinaryIO, target: str, *, chunk_size: int = 65536,
                        max_value_chars: int = 67108864) -> Iterator[tuple]:
    if type(target) is not str: raise JsonStreamError('target field string required')
    reader = _Reader(stream, chunk_size, max_value_chars)
    reader.expect('{')
    seen = set()
    count = 0
    reader.space()
    if reader.peek() != '}':
        while True:
            key = reader.value()
            if type(key) is not str: raise JsonStreamError('object key string required')
            if key in seen: raise JsonStreamError('duplicate top-level key')
            seen.add(key)
            reader.expect(':')
            if key == target:
                reader.expect('[')
                reader.space()
                if reader.peek() != ']':
                    while True:
                        value = reader.value()
                        yield ('item', count, value)
                        count += 1
                        reader.space()
                        delimiter = reader.take()
                        if delimiter == ']': break
                        if delimiter != ',': raise JsonStreamError('array separator required')
                else: reader.take()
            else:
                yield ('field', key, reader.value())
            reader.space()
            delimiter = reader.take()
            if delimiter == '}': break
            if delimiter != ',': raise JsonStreamError('object separator required')
    else: reader.take()
    reader.space()
    if reader.peek(): raise JsonStreamError('trailing JSON data')
    if target not in seen: raise JsonStreamError('selected array missing')
    yield ('end', target, count)
```

Complete reader tests:

```python
import ast
import io
import json
from pathlib import Path
import pytest
from scripts.a4_json_stream import JsonStreamError, ResourceLimit, iter_array_document


def events(data, chunk=65536, limit=67108864):
    return list(iter_array_document(io.BytesIO(data), 'states', chunk_size=chunk,
                                    max_value_chars=limit))


def test_reader_escaped_duplicate_key_red():
    with pytest.raises(JsonStreamError, match='duplicate object key'):
        events(b'{"states":[{"x":1,"\\u0078":2}]}', 1)


def test_reader_fields_types_and_every_chunk_size():
    raw = '{"head":{"v":2},"states":[0,1.0,true,1e2,"é😀\\\\\\\"[]{}",{"a":[null]}],"tail":"ok"}'.encode()
    whole = json.loads(raw)
    expected = [('field', 'head', whole['head'])]
    expected += [('item', i, value) for i, value in enumerate(whole['states'])]
    expected += [('field', 'tail', 'ok'), ('end', 'states', 6)]
    for chunk in range(1, len(raw)+1): assert events(raw, chunk) == expected
    items = [e[2] for e in events(raw, 1) if e[0] == 'item']
    assert [type(x) for x in items[:4]] == [int, float, bool, float]


@pytest.mark.parametrize('raw', [
    b'{"states":[] ,"states":[]}', b'{"states":[],"x":1,"x":2}',
    b'{"states":[{"a":1,"a":2}]}', b'{"states":0}', b'{}', b'[]',
    b'{"states":[01]}', b'{"states":[+1]}', b'{"states":[1.]}',
    b'{"states":[1e]}', b'{"states":[NaN]}', b'{"states":[Infinity]}',
    b'{"states":[-Infinity]}', b'{"states":[1e999]}',
    b'{"states":[1,]}', b'{"states":[],}', b'{"states":[}',
    b'{"states":["\\q"]}', b'{"states":["a\nb"]}',
    b'{"states":[]} false', b'\xef\xbb\xbf{"states":[]}',
    b'{"states":["\xff"]}', b'{"states":["\xc3"]}', b'{"states":[',
])
def test_reader_rejects_invalid_at_chunk_boundaries(raw):
    for chunk in (1, 2, 7, 65536):
        with pytest.raises(JsonStreamError): events(raw, chunk)


def test_reader_end_only_after_suffix_and_eof():
    iterator = iter_array_document(io.BytesIO(b'{"states":[1],"x":0,"x":1}'), 'states', chunk_size=1)
    assert next(iterator) == ('item', 0, 1)
    assert next(iterator) == ('field', 'x', 0)
    with pytest.raises(JsonStreamError): next(iterator)
    assert events(b'{"states":[]} \n') == [('end', 'states', 0)]


class BoundedInput(io.BytesIO):
    def __init__(self, value, bound):
        super().__init__(value)
        self.bound = bound

    def read(self, size=-1):
        assert 0 < size <= self.bound
        return super().read(size)


def test_reader_is_lazy_and_resource_limit_is_distinct():
    raw = b'{"states":[0,' + b'1,'*10000 + b'2]}'
    source = BoundedInput(raw, 8)
    iterator = iter_array_document(source, 'states', chunk_size=8)
    assert next(iterator) == ('item', 0, 0)
    assert source.tell() < len(raw)
    iterator.close()
    with pytest.raises(ResourceLimit): events(b'{"states":["abcdefghijkl"]}', 1, 10)
    with pytest.raises(JsonStreamError): events(b'{"states":[]}', 0)
    with pytest.raises(JsonStreamError): events(b'{"states":[]}', 1, 0)


def test_transport_has_no_semantic_imports():
    path = Path(__file__).resolve().parents[1]/'scripts/a4_json_stream.py'
    tree = ast.parse(path.read_text())
    names = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Import): names.extend(alias.name.split('.')[0] for alias in node.names)
        elif isinstance(node, ast.ImportFrom): names.append((node.module or '').split('.')[0])
    assert set(names) <= {'codecs', 'hashlib', 'json', 'math', 'typing'}
```

Commands, each recorded separately with actual source/environment before/after pins:

```bash
/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_a4_json_stream.py::test_reader_escaped_duplicate_key_red
/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_a4_json_stream.py
```

The first command must fail only under the prescribed duplicate-key RED and pass when corrected. No package or authority acceptance is claimed by these tests.

### Task2: incremental writer and hashing

**Files:** append to the same two owned paths, preserving admitted Task1 bytes. **Consumes:** arbitrary JSON-compatible literal values/binary streams. **Produces:** writer/hash APIs, no semantic code.

- [ ] Append tests below before writer implementation, retain missing-function diagnostic separately.
- [ ] Append the complete writer/hash code with compiling RED: change `view = view[written:]` to `break` in `_write_all`. The short-write roundtrip test must fail because emitted bytes are truncated. Preserve original source/streams, then restore only that fault.
- [ ] Run all utility tests, explicit import/syntax checks, and independent byte/receipt review. Root commits only the admitted utility paths. Integrating either consumer is a separate task with its own source freeze and complete tests.

Complete append-only implementation:

```python


def _write_all(stream, data):
    view = memoryview(data)
    while view:
        written = stream.write(view)
        if type(written) is not int or written <= 0 or written > len(view):
            raise JsonStreamError('invalid binary write progress')
        view = view[written:]


def _emit(stream, value):
    encoder = json.JSONEncoder(ensure_ascii=False, allow_nan=False, separators=(',', ':'))

    def validate(item):
        if type(item) is dict:
            for key, child in item.items():
                if type(key) is not str: raise JsonStreamError('JSON object key string required')
                validate(child)
        elif type(item) is list:
            for child in item: validate(child)
        elif item is not None and type(item) not in (str, bool, int, float):
            raise JsonStreamError('JSON value type required')

    try:
        validate(value)
        for chunk in encoder.iterencode(value): _write_all(stream, chunk.encode('utf-8'))
    except RecursionError as error:
        raise ResourceLimit('JSON output nesting limit') from error
    except (TypeError, ValueError, UnicodeError) as error:
        raise JsonStreamError('JSON output encoding failed') from error


def write_array_document(stream: BinaryIO, target: str, items: Iterable[object], *,
                         before=(), after=()) -> int:
    if type(target) is not str: raise JsonStreamError('target field string required')
    seen = {target}
    first = True

    def field(name, value):
        nonlocal first
        if type(name) is not str or name in seen: raise JsonStreamError('duplicate or invalid output field')
        seen.add(name)
        if not first: _write_all(stream, b',')
        _emit(stream, name)
        _write_all(stream, b':')
        _emit(stream, value)
        first = False

    _write_all(stream, b'{')
    for name, value in before: field(name, value)
    if not first: _write_all(stream, b',')
    _emit(stream, target)
    _write_all(stream, b':[')
    count = 0
    for value in items:
        if count: _write_all(stream, b',')
        _emit(stream, value)
        count += 1
    _write_all(stream, b']')
    first = False
    for name, value in after: field(name, value)
    _write_all(stream, b'}\n')
    return count


def hash_stream(stream: BinaryIO, *, chunk_size: int = 1048576) -> tuple[str, int]:
    if type(chunk_size) is not int or chunk_size <= 0:
        raise JsonStreamError('positive hash chunk size required')
    digest = hashlib.sha256()
    count = 0
    while True:
        data = stream.read(chunk_size)
        if type(data) is not bytes: raise JsonStreamError('binary hash input required')
        if not data: break
        digest.update(data)
        count += len(data)
    return digest.hexdigest(), count
```

Complete append-only tests:

```python
import hashlib
from scripts.a4_json_stream import write_array_document, hash_stream


class ShortOutput(io.BytesIO):
    def write(self, data):
        return super().write(data[:3])


def test_writer_short_write_roundtrip_red():
    sink = ShortOutput()
    values = [{'text': 'é😀', 'nested': [1, True, None]}, [], 'tail']
    count = write_array_document(sink, 'states', iter(values), before=(('head', 2),), after=(('tail', False),))
    raw = sink.getvalue()
    assert json.loads(raw) == {'head': 2, 'states': values, 'tail': False}
    assert count == 3
    assert events(raw, 1)[-1] == ('end', 'states', 3)


def test_writer_lazy_items_empty_and_hash_chunks():
    sink = io.BytesIO()

    def values():
        assert sink.getvalue().endswith(b'[')
        yield 1
        assert sink.getvalue().endswith(b'1')
        yield 2

    assert write_array_document(sink, 'states', values()) == 2
    raw = sink.getvalue()
    for size in (1, 3, 64):
        assert hash_stream(BoundedInput(raw, size), chunk_size=size) == (hashlib.sha256(raw).hexdigest(), len(raw))
    empty = io.BytesIO()
    assert write_array_document(empty, 'states', ()) == 0
    assert events(empty.getvalue()) == [('end', 'states', 0)]


@pytest.mark.parametrize('before,after', [((('states', 1),), ()), ((('x', 1), ('x', 2)), ()), ((('x', 1),), (('x', 2),))])
def test_writer_rejects_duplicate_or_reserved_fields(before, after):
    with pytest.raises(JsonStreamError): write_array_document(io.BytesIO(), 'states', (), before=before, after=after)


@pytest.mark.parametrize('value', [float('nan'), float('inf'), float('-inf'), object(), '\ud800', {1: 'x', '1': 'y'}, (1, 2)])
def test_writer_rejects_unencodable_values(value):
    with pytest.raises(JsonStreamError): write_array_document(io.BytesIO(), 'states', (value,))


def test_writer_and_hash_reject_invalid_progress_or_streams():
    class Stalled:
        def write(self, value): return 0
    with pytest.raises(JsonStreamError): write_array_document(Stalled(), 'states', ())
    with pytest.raises(JsonStreamError): hash_stream(io.StringIO('not binary'))
    with pytest.raises(JsonStreamError): hash_stream(io.BytesIO(), chunk_size=True)
```

Commands:

```bash
/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_a4_json_stream.py::test_writer_short_write_roundtrip_red
/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_a4_json_stream.py
```

## Consumer integration and review obligations

- [ ] Before modifying producer/checker, freeze admitted utility bytes and add both utility/test paths to their exact source/tooling inventories. Retain the same Python runtime snapshot; no additional dependency archive is needed.
- [ ] Producer iterates target `states`; checker iterates raw `states` and case `events` in lockstep, enforcing schema/metadata/counters independently. Metadata fields after arrays remain validated before any complete success. Consumers reject unexpected top-level fields; the generic utility intentionally does not know package schemas.
- [ ] Consumers must not call list() on the item iterator in actual-package paths. They must release prior event/case references and require one final end event. Existing small synthetic unit tests may collect results for assertions; this does not authorize full-package materialization.
- [ ] Both callers perform semantic equality and ITF carrier checks themselves. Decoder output does not mean authority validity; writer output does not mean admitted generator evidence. ResourceLimit, syntax/UTF-8 failure, child crash and timeout are distinct from a successful rebound provenance check followed by semantic rejection.
- [ ] Native pilot/source-order tests and largest-case memory measurements remain required by the transport addendum. This utility alone does not remove Quint's whole-string cap; native case sharding is still mandatory.
- [ ] Root independently reviews original RED/GREEN source closures, streams and terminal statuses for each task. No claim of full A4 acceptance, cryptography, model checking or Council follows from these utility tests.

The writing-plans skill supplied exact code, interfaces, controls and task boundaries. Implementation is not authorized by this plan's creation; root adoption and separate dispatch are required.
