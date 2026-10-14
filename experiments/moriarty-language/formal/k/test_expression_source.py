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
if __name__=='__main__':unittest.main()
