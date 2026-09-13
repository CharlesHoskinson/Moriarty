"""Independent entry-point controls; no native execution."""
import importlib.util
from pathlib import Path
import unittest
from unittest.mock import patch
HERE=Path(__file__).resolve().parent
SOURCE=HERE/'native-loader-smoke.py'
class EntryTests(unittest.TestCase):
    def load(self):
        self.assertTrue(SOURCE.is_file(), 'native loader smoke source is not implemented')
        spec=importlib.util.spec_from_file_location('ll02_smoke_entry_test',SOURCE)
        module=importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module
    def test_import_never_launches(self):
        with patch('os.execve') as execute, patch('subprocess.run') as run:
            self.load()
            execute.assert_not_called()
            run.assert_not_called()
    def test_unsupported_arguments_refuse_before_launch(self):
        module=self.load()
        with patch('os.execve') as execute, patch('subprocess.run') as run:
            self.assertEqual(module.main(['--unexpected']),31)
            execute.assert_not_called()
            run.assert_not_called()
if __name__=='__main__':
    unittest.main()
