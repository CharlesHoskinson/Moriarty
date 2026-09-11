"""Static source contracts only. Passing does not show K compilation/evaluation."""
import json,pathlib,re,unittest
HERE=pathlib.Path(__file__).resolve().parent
SPEC=HERE.parents[1]/'spec/successor/expression-signatures.json'
class SourceContracts(unittest.TestCase):
 def test_every_constructor_has_exact_lexical_metadata(self):
  p=HERE/'expression-shape.k'
  self.assertTrue(p.exists(),'all40 structural admission rules missing')
  text=p.read_text();expected=json.loads(SPEC.read_text())['constructors']
  self.assertEqual(set(re.findall(r'exRoles\("(\w+)"\) =>',text)),{x['constructor'] for x in expected})
  for x in expected:
   row=next(l for l in text.splitlines() if f'exRoles("{x["constructor"]}") =>' in l)
   self.assertEqual(re.findall(r'exRole\("([^"]+)"',row),[o['name'] for o in x['operands']])
 def test_no_host_prechecked_boundary(self):
  text=(HERE/'expression_codec.py').read_text()
  self.assertNotIn('subprocess',text)
  self.assertNotIn('evaluate(',text)
  self.assertNotIn('validateSchema(',text)
 def test_schema_byte_bound_precedes_invalid_j_domain(self):
  text=(HERE/'expression-v1.k').read_text()
  rule=next(line for line in text.splitlines() if 'rule <k> exInput() =>' in line)
  self.assertLess(rule.index('exJsonBytes(S) <=Int 65536'),rule.index('exJValid(S)'))
  fixture=json.loads((HERE/'fixtures/expression-v1.json').read_text())['cases']
  controls={row['id']:row for row in fixture if row['id'].startswith('invalid-schema-domain-')}
  self.assertEqual(set(controls),{'invalid-schema-domain-65536','invalid-schema-domain-65537'})
  for size,row in ((65536,controls['invalid-schema-domain-65536']),(65537,controls['invalid-schema-domain-65537'])):
   self.assertEqual(len(row['schema'].encode()),size)
   self.assertEqual(json.loads(row['schema'])[-1],None)
   self.assertEqual(row['expected']['code'],'INPUT_SCHEMA' if size == 65536 else 'INPUT_BOUND')
if __name__=='__main__':unittest.main()
