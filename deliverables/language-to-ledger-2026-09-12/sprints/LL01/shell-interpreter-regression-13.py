
import importlib.util
import copy
import unittest
from pathlib import Path
ROOT = Path('/home/charl/Moriarty')
spec = importlib.util.spec_from_file_location('diagnostic', ROOT / '.moriarty-dev/k-macro05-trace106-diagnostic.py')
adapter = importlib.util.module_from_spec(spec)
spec.loader.exec_module(adapter)
class ShellInterpreterContract(unittest.TestCase):
    def test_missing_actual_kast_interpreter_pin_rejected(self):
        pins = copy.deepcopy(adapter.load_pins())
        interpreter = Path(next(x['path'] for x in pins['selectedExecutables'] if x['role'] == 'kast-unwrapped')).read_text().splitlines()[0][2:]
        pins['selectedExecutables'] = [x for x in pins['selectedExecutables'] if x['path'] != interpreter]
        with self.assertRaises(adapter.PreflightError):
            adapter.verify_wrapper_and_parser(pins)
if __name__ == '__main__':
    unittest.main()
