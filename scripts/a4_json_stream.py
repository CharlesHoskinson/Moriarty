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
