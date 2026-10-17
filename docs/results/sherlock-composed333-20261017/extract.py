import json,sys
from pathlib import Path
def unique(pairs):
 out={}
 for key,value in pairs:
  if key in out:raise ValueError("duplicate member: "+key)
  out[key]=value
 return out
def no_number(token):raise ValueError("unexpected unquoted number: "+token)
p=Path(sys.argv[1]);v=json.loads((p/"capture.json").read_text(),object_pairs_hook=unique,parse_int=no_number,parse_float=no_number,parse_constant=no_number)
assert set(v)=={"schema","payload","ordered_non_context_payload","metadata"}
assert v["schema"]=="moriarty.sherlock-composed-real-host-capture/1"
for key,name in [("payload","payload.json"),("ordered_non_context_payload","ordered.json")]:
 with (p/name).open("x") as handle:json.dump(v[key],handle,indent=2,allow_nan=False);handle.write("\n")
