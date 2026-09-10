import ast,json,pathlib,unittest
from terminal_predicate import require_exit_zero
O=pathlib.Path(__file__).resolve().parent
R=O.parents[2]
class Retention(unittest.TestCase):
 def test_loaded_exact_invocation(self):
  fields={"LoadState":"loaded","ActiveState":"active","SubState":"exited","Type":"exec","RemainAfterExit":"yes","Transient":"yes","MainPID":"0","Result":"success","ExecMainCode":"1","ExecMainStatus":"0","InvocationID":"1"*32}
  self.assertEqual(require_exit_zero(fields,"1"*32),{"status":"EXIT_ZERO_OBSERVED","exitCode":0,"invocationId":"1"*32,"externalContainmentEstablished":False})
  for key,value in [("LoadState","not-found"),("ExecMainCode","0"),("ExecMainStatus","1"),("ExecMainCode","2"),("MainPID","23"),("SubState","running"),("ActiveState","inactive"),("InvocationID","2"*32),("Transient","no"),("RemainAfterExit","no"),("Result","exit-code")]:
   with self.subTest(key=key,value=value),self.assertRaisesRegex(ValueError,"EXIT_ZERO_NOT_ESTABLISHED"):require_exit_zero({**fields,key:value},"1"*32)
  for key in fields:
   absent=dict(fields);del absent[key]
   with self.subTest(missing=key),self.assertRaisesRegex(ValueError,"EXIT_ZERO_NOT_ESTABLISHED"):require_exit_zero(absent,"1"*32)
 def test_actual_unloaded_observation_is_not_an_exit(self):
  raw=json.loads((O.parent/'preview-loan-exit-01/terminal-observation-01.json').read_text())['systemdShow']
  fields=dict(line.split('=',1) for line in raw.splitlines())
  with self.assertRaisesRegex(ValueError,"EXIT_ZERO_NOT_ESTABLISHED"):require_exit_zero(fields,'1'*32)
 def test_patch_only_retains_unit_and_invocation_without_changing_stop_timer(self):
  original=(O.parent/'preview-loan-exit-01/execute-once.py').read_text()
  changed=original.replace("'--property=Type=exec','--property=MemoryMax","'--property=Type=exec','--property=RemainAfterExit=yes','--property=MemoryMax").replace('ActiveState,SubState,MainPID,ControlGroup,ActiveEnterTimestampMonotonic','ActiveState,SubState,MainPID,ControlGroup,ActiveEnterTimestampMonotonic,InvocationID')
  def timer(s):
   return ast.dump(next(n for n in ast.parse(s).body if isinstance(n,ast.Assign) and any(isinstance(x,ast.Name) and x.id=='timerArgv' for x in n.targets)))
  self.assertEqual(timer(original),timer(changed));self.assertEqual(changed.count("'--property=RemainAfterExit=yes'"),1)
  self.assertNotIn('--wait',changed);self.assertNotIn('--collect',changed)
if __name__=='__main__':unittest.main()
