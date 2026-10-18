from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent
a=json.loads((p/'INVENTORY.json').read_text())
for name,digest in a.items():
 assert hashlib.sha256((p/name).read_bytes()).hexdigest()==digest,name
f=json.loads((p/'FREEZE.json').read_text())
assert hashlib.sha256((p/'INVENTORY.json').read_bytes()).hexdigest()==f['inventory_sha256']
print('Packet hashes verified. This is an abstention; no positive/runtime claim.')
