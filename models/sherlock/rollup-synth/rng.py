"""Counter-based SHA256 streams for future synthetic choices; not cryptographic proof hashing."""
from common import Error
from frtf import th
import struct

class Stream:
    def __init__(self, seed, domain):
        if type(seed) is not int or not 0 <= seed < 2**64 or not isinstance(domain, str) or not domain:
            raise Error("stream requires u64 seed and nonempty domain")
        name = domain.encode("utf-8")
        self.prefix = struct.pack("<Q", seed) + name
        self.counter = 0
        self.buffer = b""
    def read(self, count):
        if type(count) is not int or not 0 <= count <= 2**24: raise Error("stream read outside 0..2^24")
        while len(self.buffer) < count:
            if self.counter >= 2**64: raise Error("stream counter exhausted")
            self.buffer += th("moriarty/fsyn/rng/v1", self.prefix, struct.pack("<Q", self.counter))
            self.counter += 1
        result, self.buffer = self.buffer[:count], self.buffer[count:]
        return result
