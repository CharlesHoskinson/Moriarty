"""Controlled existing-runner boundary checks; command is always intercepted."""
import json,pathlib,sys,tempfile,unittest
from unittest.mock import patch
import run
class ExpressionRunner(unittest.TestCase):
 def test_macro_free_guard_controls_actual_expression_krun_argv(self):
  run.select_suite('expression-v1')
  with tempfile.TemporaryDirectory() as tmp,patch.object(run,'DEFINITION',pathlib.Path(tmp)):
   (run.DEFINITION/'macros.kore').write_bytes(b'// macros\n')
   (run.DEFINITION/'syntaxDefinition.kore').write_text('alias weakExistsFinally{}() : SortK where top{}() := top{}()\nalias weakAlwaysFinally{}() : SortK where top{}() := top{}()\nalias allPathGlobally{}() : SortK where top{}() := top{}()\n')
   term=run.expression_codec.encode_request('{}','{}')
   argv=run.expression_krun_argv(['PINNED-KRUN','input','--definition',str(run.DEFINITION),'--output','json','--parser','parser'],term)
   self.assertEqual(argv[-1],'--no-expand-macros')
  run.select_suite('initial')
 def test_macro_free_guard_fails_closed_for_artifact_or_alias_change(self):
  run.select_suite('expression-v1')
  with tempfile.TemporaryDirectory() as tmp,patch.object(run,'DEFINITION',pathlib.Path(tmp)):
   (run.DEFINITION/'macros.kore').write_bytes(b'// macros\n')
   (run.DEFINITION/'syntaxDefinition.kore').write_text('alias weakExistsFinally{}() : SortK where top{}() := top{}()\nalias weakAlwaysFinally{}() : SortK where top{}() := top{}()\nalias allPathGlobally{}() : SortK where top{}() := top{}()\n')
   term=run.expression_codec.encode_request('{}','{}')
   (run.DEFINITION/'macros.kore').write_bytes(b'nonempty')
   with self.assertRaisesRegex(run.RunnerError,'MACRO_GUARD'):run.expression_krun_argv([],term)
   (run.DEFINITION/'macros.kore').write_bytes(b'// macros\n')
   alias_term=run.expression_codec._apply('weakExistsFinally')
   with self.assertRaisesRegex(run.RunnerError,'MACRO_GUARD'):run.expression_krun_argv([],alias_term)
  run.select_suite('initial')
 def test_macro_guard_failure_is_charged_but_never_dispatches(self):
  run.select_suite('expression-v1')
  with tempfile.TemporaryDirectory() as tmp,patch.object(run,'BUILD',pathlib.Path(tmp)),patch.object(run,'BINDING',pathlib.Path(tmp)/'binding.json'),patch.object(run,'DEFINITION',pathlib.Path(tmp)/'definition'):
   run.DEFINITION.mkdir();(run.DEFINITION/'macros.kore').write_bytes(b'nonempty')
   (run.DEFINITION/'syntaxDefinition.kore').write_text('alias weakExistsFinally{}() : SortK where top{}() := top{}()\nalias weakAlwaysFinally{}() : SortK where top{}() := top{}()\nalias allPathGlobally{}() : SortK where top{}() := top{}()\n')
   run.BINDING.write_text(json.dumps({'sources':{'x':'fixed'},'artifacts':{'artifact':'fixed'},'krunInvocations':0}))
   with patch.object(run,'toolchain',return_value={'executables':{'krun':{'path':'PINNED-KRUN'}}}),patch.object(run,'sources',return_value={'x':'fixed'}),patch.object(run,'artifacts',return_value={'artifact':'fixed'}),patch.object(run,'command') as command:
    with self.assertRaisesRegex(run.RunnerError,'MACRO_GUARD'):run.evaluate({'schema':'{}','request':'{}'},'guarded')
    self.assertEqual(json.loads(run.BINDING.read_text())['krunInvocations'],1);command.assert_not_called()
  run.select_suite('initial')
 def test_explicit_suite_preserves_old_limits(self):
  self.assertIn('expression-v1',run.SUITES)
  self.assertEqual(run.SUITE_LIMITS['initial'],(16,512))
  self.assertEqual(run.SUITE_LIMITS['numeric'],(64,1512))
  self.assertEqual(run.SUITE_LIMITS['expression-v1'],(115,2552))
 def test_complete_fixture_loop_is_inert_and_deep_safe(self):
  run.select_suite('expression-v1')
  fixture=run.expression_codec._parse((run.HERE/run.SUITES['expression-v1']).read_text(),allow_space=True)['cases']
  expected={r['id']:r['expected'] for r in fixture}
  with tempfile.TemporaryDirectory() as tmp,patch.object(run,'BUILD',pathlib.Path(tmp)),patch.object(run,'evaluate',side_effect=lambda packet,label:expected[label]) as call:
   result=run.traces()
   self.assertEqual(len(result),115);self.assertEqual(call.call_count,115)
   self.assertTrue(all(r['matchesExpected'] for r in result))
  run.select_suite('initial')
 def test_real_evaluate_boundary_retains_attempt_before_dispatch(self):
  run.select_suite('expression-v1')
  with tempfile.TemporaryDirectory() as tmp,patch.object(run,'BUILD',pathlib.Path(tmp)),patch.object(run,'BINDING',pathlib.Path(tmp)/'binding.json'),patch.object(run,'DEFINITION',pathlib.Path(tmp)/'definition'):
   run.DEFINITION.mkdir();(run.DEFINITION/'macros.kore').write_bytes(b'// macros\n')
   (run.DEFINITION/'syntaxDefinition.kore').write_text('alias weakExistsFinally{}() : SortK where top{}() := top{}()\nalias weakAlwaysFinally{}() : SortK where top{}() := top{}()\nalias allPathGlobally{}() : SortK where top{}() := top{}()\n')
   run.BINDING.write_text(json.dumps({'sources':{'x':'fixed'},'artifacts':{'artifact':'fixed'},'krunInvocations':0}))
   c=run.expression_codec;packet={'schema':'{}','request':'{}'}
   out=c._apply('exValue',c._encode(['UInt128']),c._encode('7'),c._token('Int','9'))
   raw=c._canonical({'format':'KAST','version':4,'term':c._apply('<generatedTop>',c._apply('<out>',out),c._apply('<k>',{'node':'KSequence','arity':0,'items':[]}),c._apply('<sigma>',c._encode({})),c._apply('<request>',c._encode({})))})
   def controlled(argv,seconds,label):
    self.assertEqual(json.loads(run.BINDING.read_text())['krunInvocations'],1)
    self.assertEqual(argv[-3:-1],['--parser',str(run.HERE/'expression-parser.py')]);self.assertEqual(argv[-1],'--no-expand-macros');self.assertEqual(seconds,20)
    wire=c._parse(pathlib.Path(argv[1]).read_text(),allow_space=True)
    self.assertEqual(wire['format'],'KAST');self.assertEqual(wire['version'],4)
    return raw
   with patch.object(run,'toolchain',return_value={'executables':{'krun':{'path':'PINNED-KRUN'}}}),patch.object(run,'sources',return_value={'x':'fixed'}),patch.object(run,'artifacts',return_value={'artifact':'fixed'}),patch.object(run,'command',side_effect=controlled):
    self.assertEqual(run.evaluate(packet,'controlled')['value'],'7')
    self.assertTrue((run.BUILD/'trace-01.result.json').exists())
  run.select_suite('initial')
 def test_deepest_frozen_request_reaches_mocked_macro_free_dispatch(self):
  run.select_suite('expression-v1')
  cases=run.expression_codec._parse((run.HERE/run.SUITES['expression-v1']).read_text(),allow_space=True)['cases']
  case=next(c for c in cases if c['id']=='metadata-depth-5947')
  packet={'schema':case['schema'],'request':case['request']};c=run.expression_codec
  with tempfile.TemporaryDirectory() as tmp,patch.object(run,'BUILD',pathlib.Path(tmp)),patch.object(run,'BINDING',pathlib.Path(tmp)/'binding.json'),patch.object(run,'DEFINITION',pathlib.Path(tmp)/'definition'):
   run.DEFINITION.mkdir();(run.DEFINITION/'macros.kore').write_bytes(b'// macros\n')
   (run.DEFINITION/'syntaxDefinition.kore').write_text('alias weakExistsFinally{}() : SortK where top{}() := top{}()\nalias weakAlwaysFinally{}() : SortK where top{}() := top{}()\nalias allPathGlobally{}() : SortK where top{}() := top{}()\n')
   run.BINDING.write_text(json.dumps({'sources':{'x':'fixed'},'artifacts':{'artifact':'fixed'},'krunInvocations':0}))
   out=c._apply('exRejected',c._string('INPUT_SCHEMA'),c._encode({'kind':'synthetic','start':'0','end':'0'}),c._apply('ejNil'),c._token('Int','0'))
   raw=c._canonical({'format':'KAST','version':4,'term':c._apply('<generatedTop>',c._apply('<out>',out),c._apply('<k>',{'node':'KSequence','arity':0,'items':[]}),c._apply('<sigma>',c.encode_json(packet['schema'])),c._apply('<request>',c.encode_json(packet['request'])) )})
   def controlled(argv,seconds,label):
    self.assertEqual(label,'trace-01');self.assertEqual(argv[-1],'--no-expand-macros');return raw
   with patch.object(run,'toolchain',return_value={'executables':{'krun':{'path':'PINNED-KRUN'}}}),patch.object(run,'sources',return_value={'x':'fixed'}),patch.object(run,'artifacts',return_value={'artifact':'fixed'}),patch.object(run,'command',side_effect=controlled) as command:
    self.assertEqual(run.evaluate(packet,case['id'])['code'],'INPUT_SCHEMA');self.assertEqual(command.call_count,1)
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
 def test_command_records_float_accounting_as_json(self):
  run.select_suite('expression-v1')
  with tempfile.TemporaryDirectory() as tmp,patch.object(run,'BUILD',pathlib.Path(tmp)):
   self.assertEqual(run.command([sys.executable,'-c','pass'],1,'accounting'),'')
   receipt=json.loads((run.BUILD/'accounting.command.json').read_text())
   self.assertIsInstance(receipt['elapsedSeconds'],float)
   self.assertIsInstance(receipt['aggregateSeconds'],float)
  run.select_suite('initial')
 def test_command_caps_combined_stdout_and_stderr_at_exact_boundary(self):
  run.select_suite('expression-v1')
  with tempfile.TemporaryDirectory() as tmp,patch.object(run,'BUILD',pathlib.Path(tmp)),patch.dict(run.OUTPUT_LIMITS,{'expression-v1':16}):
   exact='import sys;sys.stdout.write("x"*8);sys.stderr.write("y"*8)'
   over='import sys;sys.stdout.write("x"*8);sys.stderr.write("y"*9)'
   self.assertEqual(run.command([sys.executable,'-c',exact],1,'exact'),'x'*8)
   with self.assertRaisesRegex(run.RunnerError,'K_OUTPUT_BOUND'):
    run.command([sys.executable,'-c',over],1,'over')
  run.select_suite('initial')
if __name__=='__main__':unittest.main()
