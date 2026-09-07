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
