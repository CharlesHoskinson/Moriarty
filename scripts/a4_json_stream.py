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
