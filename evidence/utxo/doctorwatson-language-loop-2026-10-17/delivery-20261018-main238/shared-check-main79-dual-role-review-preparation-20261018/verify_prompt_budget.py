"""Read-only counter for the exact role-specific textual prompt composition."""
from pathlib import Path
import argparse, hashlib, json
parser=argparse.ArgumentParser()
parser.add_argument('role', choices=['source','result'])
parser.add_argument('prefix',type=Path,help='Exact root-owned role-specific external caller/token prefix file')
args=parser.parse_args()
r=Path(__file__).resolve().parent
prefix=args.prefix.read_bytes(); role=(r/(args.role.upper()+'-PROMPT.md')).read_bytes(); common=(r/'COMMON-PRIMARY.md').read_bytes()
body=b'\n\n'.join([prefix,role,common])
report={'role':args.role,'prefixBytes':len(prefix),'prefixSha256':hashlib.sha256(prefix).hexdigest(),'rolePromptBytes':len(role),'commonPrimaryBytes':len(common),'joinBytes':4,'aggregateBytes':len(body),'aggregateSha256':hashlib.sha256(body).hexdigest(),'strictlyBelow125000':len(body)<125000,'scope':'Only this exact text composition. Include every extra textual wrapper/instruction/schema in prefix; structured response-schema transport is separate, not claimed already counted.'}
print(json.dumps(report,indent=2))
if len(body)>=125000:raise SystemExit('Exact role prompt exceeds strict125000byte bound')
