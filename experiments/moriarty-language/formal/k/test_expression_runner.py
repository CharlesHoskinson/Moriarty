"""Controlled existing-runner boundary checks; command is always intercepted."""
import pathlib,tempfile,unittest,json
from unittest.mock import patch
import run
class ExpressionRunner(unittest.TestCase):
 def test_explicit_suite_preserves_old_limits(self):
  self.assertIn('expression-v1',run.SUITES)
  self.assertEqual(run.SUITE_LIMITS['initial'],(16,512))
  self.assertEqual(run.SUITE_LIMITS['numeric'],(64,1512))
  self.assertEqual(run.SUITE_LIMITS['expression-v1'],(113,2512))
 def test_complete_fixture_loop_is_inert_and_deep_safe(self):
  run.select_suite('expression-v1')
  fixture=run.expression_codec._parse((run.HERE/run.SUITES['expression-v1']).read_text(),allow_space=True)['cases']
  expected={r['id']:r['expected'] for r in fixture}
  with tempfile.TemporaryDirectory() as tmp,patch.object(run,'BUILD',pathlib.Path(tmp)),patch.object(run,'evaluate',side_effect=lambda packet,label:expected[label]) as call:
   result=run.traces()
   self.assertEqual(len(result),113);self.assertEqual(call.call_count,113)
   self.assertTrue(all(r['matchesExpected'] for r in result))
  run.select_suite('initial')
 def test_real_evaluate_boundary_retains_attempt_before_dispatch(self):
  run.select_suite('expression-v1')
  with tempfile.TemporaryDirectory() as tmp,patch.object(run,'BUILD',pathlib.Path(tmp)),patch.object(run,'BINDING',pathlib.Path(tmp)/'binding.json'):
   run.BINDING.write_text(json.dumps({'sources':{'x':'fixed'},'artifacts':{'artifact':'fixed'},'krunInvocations':0}))
   c=run.expression_codec;packet={'schema':'{}','request':'{}'}
   out=c._apply('exValue',c._encode(['UInt128']),c._encode('7'),c._token('Int','9'))
   raw=c._canonical({'format':'KAST','version':4,'term':c._apply('<generatedTop>',c._apply('<out>',out),c._apply('<k>',{'node':'KSequence','arity':0,'items':[]}),c._apply('<sigma>',c._encode({})),c._apply('<request>',c._encode({})))})
   def controlled(argv,seconds,label):
    self.assertEqual(json.loads(run.BINDING.read_text())['krunInvocations'],1)
    self.assertEqual(argv[-2:],['--parser',str(run.HERE/'expression-parser.py')]);self.assertEqual(seconds,20)
    wire=c._parse(pathlib.Path(argv[1]).read_text(),allow_space=True)
    self.assertEqual(wire['format'],'KAST');self.assertEqual(wire['version'],4)
    return raw
   with patch.object(run,'toolchain',return_value={'executables':{'krun':{'path':'PINNED-KRUN'}}}),patch.object(run,'sources',return_value={'x':'fixed'}),patch.object(run,'artifacts',return_value={'artifact':'fixed'}),patch.object(run,'command',side_effect=controlled):
    self.assertEqual(run.evaluate(packet,'controlled')['value'],'7')
    self.assertTrue((run.BUILD/'trace-01.result.json').exists())
  run.select_suite('initial')
 def test_fresh_definition_and_structural_parser_argv(self):
  self.assertTrue(hasattr(run,'select_suite'))
  with tempfile.TemporaryDirectory() as tmp:
   root=pathlib.Path(tmp)
   with patch.object(run,'HERE',root):
    run.select_suite('expression-v1')
    self.assertEqual(run.BUILD,root/'.build-expression-v1')
    run.BUILD.mkdir();(run.BUILD/'expression-v1-kompiled').mkdir()
    lock={'executables':{'kompile':{'path':'PINNED-KOMPILE'},'krun':{'path':'PINNED-KRUN'}}}
    with patch.object(run,'toolchain',return_value=lock),patch.object(run,'sources',return_value={'x':'fixed'}),patch.object(run,'artifacts',return_value={'artifact':'fixed'}),patch.object(run,'command',return_value='') as cmd:
     run.compile_definition()
     argv=cmd.call_args.args[0];self.assertEqual(argv[0],'PINNED-KOMPILE');self.assertIn(str(root/'expression-v1.k'),argv)
     self.assertIn('MORIARTY-EXPRESSION-V1',argv)
     with self.assertRaisesRegex(run.RunnerError,'COMPILE_ALREADY_ATTEMPTED'):run.compile_definition()
     self.assertEqual(cmd.call_count,1)
  run.select_suite('initial')
if __name__=='__main__':unittest.main()
