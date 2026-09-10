"""Run only exact archived function AST with inert schema dependency."""
import ast,json
from pathlib import Path
R=Path(__file__).resolve().parents[2]
p=R/'evidence/moriarty-completion-program-2026-09-07/SP01/independent-review-round-08/successor/candidate-04-partial/experiments/moriarty-language/spec/successor/signing-contract.test.py'
t=ast.parse(p.read_text());fn=next(n for n in t.body if isinstance(n,ast.FunctionDef) and n.name=='first_false_stage')
class V:
 def iter_errors(self,doc):return []
env={'schema_validator':lambda s:V()};exec(compile(ast.Module(body=[fn],type_ignores=[]),str(p),'exec'),env)
a=env['first_false_stage']({'intendedFailureStage':'authorization'},{'network':'local'}, {}, {})
print(json.dumps({'observation':a,'expectedWithoutMutation':None,'scope':'Exact archived function AST; inert schema validation only. Semantic label returned with no contextual checks.'}))
assert a is None,'archived function returns fixture label without mutation'
