"""Explicit normal positive integration consumer of unchanged R328 APIs. No K/Quint launch."""
import importlib.util
import json
from pathlib import Path
import sys
import unittest

W=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
P=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/251-f03-paired-model-operation-proposal')
OUT=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ021-F03-K-caller-materialized-review/F03-next-capture-and-projection/passive-outputs-01')
ASSEMBLER=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ021-F03-K-caller-materialized-review/F03-next-capture-and-projection/logged-input-chain-candidate-04/afterimages/models/sherlock/qa/assemble_k_f03_observations.py')
CANDIDATE_ROOT=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ021-F03-K-caller-materialized-review/F03-next-capture-and-projection/logged-input-chain-candidate-04/afterimages')
sys.path.insert(0,str(W))
sys.path.insert(0,str(CANDIDATE_ROOT))
spec=importlib.util.spec_from_file_location('f03_r328_actual_api',ASSEMBLER)
a=importlib.util.module_from_spec(spec)
spec.loader.exec_module(a)
assert Path(a.decode_capture.__code__.co_filename).resolve()==CANDIDATE_ROOT/'models/sherlock/qa/k_capture_observations.py'
receipt=json.loads((P/'actual/k/receipt.json').read_text())
receipts={row['stage']:row['child_receipt'] for row in receipt['stages'] if row['stage']!='initializer'}
stages=a.load_stage_artifacts_from_actual_dir(P/'actual/k',receipts_by_stage_id=receipts)
run_input=(P/'input/F03-fuel093-stack064.ui').read_bytes()
inventory=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/COMPILED-MAIN-RULE-SPANS-01.json').read_bytes()
commands=(P/'K-COMMANDS.json').read_bytes()
source_k=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/compile-input/models/sherlock/k/main.k').read_bytes()
FIELDS={'code_k','entry_k','control_k','environment_k','frames_top_first_k','depth','stack_limit','initial_fuel','remaining_fuel','charged_steps','output_k'}
DECODED=None

class ActualF03PassivePositiveTests(unittest.TestCase):
    def test_actual_jvm_unpinned_and_pinned_payloads_preserve_all_capture_bytes(self):
        self.assertEqual(set(stages),{'kast','initializer'}|{f'exec-{i:03d}' for i in range(97)}|{f'print-{i:03d}' for i in range(97)})
        base=a.assemble_f03_capture_payload(stages,run_input_bytes=run_input,rule_span_inventory_bytes=inventory,strict_stage_validation=True)
        pinned=a.assemble_f03_capture_payload(stages,run_input_bytes=run_input,rule_span_inventory_bytes=inventory,k_commands_bytes=commands,strict_stage_validation=True)
        self.assertEqual(base,pinned)
        self.assertEqual(len(pinned['captures']),95)
        for index,capture in enumerate(pinned['captures'],1):
            self.assertEqual(capture['configuration'],(P/f'actual/k/print-{index:03d}/stdout').read_text())
            if index>1:
                self.assertEqual(capture['incoming_log'],(P/f'actual/k/exec-{index:03d}/stderr').read_bytes())
                self.assertEqual(capture['state_kore'],(P/f'actual/k/exec-{index:03d}/state.kore').read_bytes())
                self.assertEqual(capture['engine_receipt'],receipts[f'exec-{index:03d}'])
                self.assertEqual(capture['print_receipt'],receipts[f'print-{index:03d}'])
        for key,index in [('startup_probe',1),('terminal_probe',96)]:
            self.assertEqual(pinned[key]['log'],(P/f'actual/k/exec-{index:03d}/stderr').read_bytes())
            self.assertEqual(pinned[key]['state_kore'],(P/f'actual/k/exec-{index:03d}/state.kore').read_bytes())
        self.assertEqual(pinned['pre_startup']['kast']['stdout_kore'],(P/'actual/k/kast/stdout').read_text())
        self.assertEqual(pinned['pre_startup']['initializer']['initializer_kore'],(P/'actual/k/initializer/initializer.kore').read_text())
        self.assertEqual(pinned['pre_startup']['print-000']['configuration'],(P/'actual/k/print-000/stdout').read_text())

    def test_actual_decoding_has_complete_eleven_field_94_record_contract(self):
        global DECODED
        DECODED=a.decode_f03_observations(stages,run_input_bytes=run_input,rule_span_inventory_bytes=inventory,source_k_bytes=source_k,k_commands_bytes=commands,strict_stage_validation=True)
        self.assertTrue(DECODED['bundle']['comparison_ready'])
        self.assertTrue(DECODED['decoded_run']['comparison_ready'])
        self.assertEqual(DECODED['decoded_run']['gaps'],[])
        run=DECODED['bundle']['observations']['runs'][0]
        self.assertEqual(len(run['records']),94)
        self.assertEqual([row['charge'] for row in run['records']],[1]*93+[0])
        self.assertEqual(run['records'][-1]['rule_start_line'],80)
        self.assertIs(run['terminal_has_next'],False)
        snapshots=[run['initial'],run['terminal']]+[row[key] for row in run['records'] for key in ['before','after']]
        for snapshot in snapshots:
            self.assertEqual(set(snapshot),FIELDS)
            self.assertEqual(snapshot['stack_limit'],64)
            self.assertEqual(snapshot['initial_fuel'],93)
        # Complete semantic values remain independently checked by existing paired comparator against frozen pre-run oracle.

if __name__=='__main__':
    result=unittest.TextTestRunner(verbosity=2).run(unittest.TestSuite([ActualF03PassivePositiveTests('test_actual_decoding_has_complete_eleven_field_94_record_contract')]))
    if not result.wasSuccessful():
        raise SystemExit(1)
    for name,value in [('k-observations.json',DECODED['bundle']),('k-decoded-run.json',DECODED['decoded_run']),('k-assembled-provenance.json',DECODED['assembled_provenance'])]:
        with (OUT/name).open('x') as stream:
            json.dump(value,stream,sort_keys=True,allow_nan=False)
            stream.write('\n')
    print(json.dumps({'normal_positive_tests':result.testsRun,'outputs':[str(OUT/name) for name in ['k-observations.json','k-decoded-run.json','k-assembled-provenance.json']],'scope':'actual488file passiveload/assembly/decode only; no K/Quint/model/evaluator execution; no security-negative test selection'}))
