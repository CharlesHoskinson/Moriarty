"""Count the exact caller prefix plus all local instructions and primary input."""
from pathlib import Path
import argparse,json,hashlib
parser=argparse.ArgumentParser()
parser.add_argument('prefix',type=Path,help='Exact root-owned external token prefix file; read only')
args=parser.parse_args()
root=Path(__file__).resolve().parent
prefix=args.prefix.read_bytes()
instructions=(root/'DISPATCH-INSTRUCTIONS.md').read_bytes()
primary=(root/'PRIMARY.md').read_bytes()
body=b'\n\n'.join([prefix,instructions,primary])
report={'prefixBytes':len(prefix),'prefixSha256':hashlib.sha256(prefix).hexdigest(),'dispatchInstructionBytes':len(instructions),'primaryBytes':len(primary),'separatorBytes':4,'aggregateBytes':len(body),'strictlyBelow125000':len(body)<125000,'aggregateSha256':hashlib.sha256(body).hexdigest(),'scope':'This exact byte composition only; any extra caller text must be included in prefix before counting.'}
print(json.dumps(report,indent=2))
if len(body)>=125000:raise SystemExit('Combined review input exceeds strict size bound')
