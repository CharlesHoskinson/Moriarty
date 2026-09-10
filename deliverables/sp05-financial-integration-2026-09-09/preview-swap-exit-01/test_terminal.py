import unittest
from terminal_predicate import require_exit_zero
class Terminal(unittest.TestCase):
 def test_retained_same_invocation(self):
  f=dict(LoadState='loaded',ActiveState='active',SubState='exited',Type='exec',RemainAfterExit='yes',Transient='yes',MainPID='0',Result='success',ExecMainCode='1',ExecMainStatus='0',InvocationID='103af569b7b74c81ac2147fe3a263348')
  self.assertEqual(require_exit_zero(f,f['InvocationID'])['exitCode'],0)
  for key,value in [('LoadState','not-found'),('ExecMainCode','0'),('ExecMainStatus','1'),('ActiveState','inactive'),('MainPID','12'),('InvocationID','a'*32)]:
   with self.subTest(key=key),self.assertRaises((ValueError,AssertionError)):require_exit_zero({**f,key:value},f['InvocationID'])
if __name__=='__main__':unittest.main()
