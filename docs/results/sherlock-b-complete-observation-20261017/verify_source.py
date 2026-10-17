"""Ordinary source/grammar inventory, not Rust execution or transition checking."""
from pathlib import Path
import re,json,hashlib
from tree_sitter import Language,Parser
import tree_sitter_rust
R=Path(__file__).resolve().parent;W=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016');C=R/'candidate'
p=C/'crates/mori-core/src/sherlock/helper_machine/complete_observation.rs';s=p.read_text();host=(W/'crates/mori-core/src/sherlock/helper_machine/shared/owner_host.rs').read_text();shared=(W/'crates/mori-core/src/sherlock/helper_machine/shared.rs').read_text();raw=(W/'crates/mori-core/src/sherlock/helper_machine/raw.rs').read_text();checks=[]
def check(name,passed,detail):checks.append({'predicate':name,'passed':bool(passed),'detail':detail})
def body(source,name,kind='struct'):
 m=re.search(r'pub '+kind+' '+name+r"(?:<'a>)?\s*\{",source);start=m.end();i=start;depth=1
 while depth:
  if source[i]=='{':depth+=1
  elif source[i]=='}':depth-=1
  i+=1
 return source[start:i-1]
def fields(source,name):return re.findall(r'pub (\w+):',body(source,name))
for actual,owned,source in [('HostObservation','HostSnapshot',host),('AdmissionObservation','AdmissionSnapshot',host),('LayoutObservation','LayoutSnapshot',host),('WidthTableObservation','WidthsSnapshot',host),('ImageObservation','ImageSnapshot',host),('ProgramView','ProgramSnapshot',shared)]:
 old=fields(source,actual);new=fields(s,owned);check(actual+' complete public member coverage',old==new,{'source':old,'snapshot':new})
def variants(text):
 parts=[];depth=0;start=0
 for i,c in enumerate(text):
  if c in '({[<':depth+=1
  elif c in ')}]>':depth-=1
  elif c==',' and depth==0:parts.append(text[start:i]);start=i+1
 parts.append(text[start:])
 return [re.match(r'\s*(\w+)',part).group(1) for part in parts if part.strip()]
for actual,owned,source in [('ExpressionRowC','ExpressionSnapshot',shared),('ConstRow','ConstantSnapshot',raw),('DataRow','DataSnapshot',raw)]:
 old=variants(body(source,actual,'enum'));new=variants(body(s,owned,'enum'))
 arms=re.findall(actual+r'::(\w+)\s*(?:\{|=>)',s)
 check(actual+' explicit copy arms',old==new and set(old)==set(arms),{'source_variants':old,'snapshot_variants':new,'copy_arms':arms})
check('owned pre copied before one fixed owner step',s.index('let before = capture(admitted, image)')<s.index('match engine.step(admitted, image)') and s.count('engine.step(admitted, image)')==1,'No alternate engine, injected event, run loop or expected input')
check('raw StepEventC retained', 'pub event: StepEventC' in s and 'CompleteStepRecord { before, event, after }' in s,'Returned event moved into record without rewriting fields')
check('no JSON or Debug string substitutes','serde_json' not in s and 'format!(' not in s,'Owned concrete fields/enums, not formatted observations')
parser=Parser(Language(tree_sitter_rust.language()))
for f in sorted(C.rglob('*.rs')):check(str(f.relative_to(C))+' Rust grammar',not parser.parse(f.read_bytes()).root_node.has_error,'Grammar only, no traits/borrows/typecheck')
result={'classification':'source/hash/grammar only; all Rust compilation/test/step execution unrun','checks':checks,'all_passed':all(x['passed'] for x in checks)}
(R/'SOURCE-STRUCTURE.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'all_passed':result['all_passed'],'predicates':len(checks),'failures':[x for x in checks if not x['passed']]},indent=2));raise SystemExit(0 if result['all_passed'] else 1)
