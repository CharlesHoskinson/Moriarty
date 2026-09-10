"""Evaluate the exact archived metadata branch predicate at reordered path."""
import ast,json
from pathlib import Path
R=Path(__file__).resolve().parents[2];p=R/'evidence/moriarty-completion-program-2026-09-07/SP01/independent-review-round-08/successor/candidate-04-partial/experiments/moriarty-language/spec/successor/generate-signing-examples.py'
t=ast.parse(p.read_text());guard=next(n.test for n in ast.walk(t) if isinstance(n,ast.If) and 'action.arguments.0.value.value' in ast.unparse(n.test));value=eval(compile(ast.Expression(guard),str(p),'eval'),{'__builtins__':{}},{'ctx':{'debt_id':'Loan01'},'path':'executionBody.action.arguments.1.value.value'})
print(json.dumps({'observedMetadataBranch':value,'expected':True,'scope':'Exact archived predicate only; enclosing named debt_id remains valid after argument reversal.'}));assert value is True,'archived metadata branch depends on array index0'
