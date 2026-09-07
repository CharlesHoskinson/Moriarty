"""Validate the lossless Git-sized partition of the original shared runtime."""
import hashlib
import json
from pathlib import Path
base = Path(__file__).resolve().parent
parts = sorted(base.glob('runtime.tar.gz.part-*'))
assert len(parts) == 7
whole, records, total = hashlib.sha256(), [], 0
for index, part in enumerate(parts):
    assert part.name == f'runtime.tar.gz.part-{index:02}'
    raw = part.read_bytes()
    assert 0 < len(raw) <= 50*1024*1024
    whole.update(raw); total += len(raw)
    records.append({'path':part.name,'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()})
assert total == 315704660
assert whole.hexdigest() == 'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c'
print(json.dumps({'status':'lossless-partition-verified','bytes':total,'sha256':whole.hexdigest(),'parts':records},indent=2))
