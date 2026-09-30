"""The subprocess contract is tested before the comparator exists."""
import json
import copy
import re
from pathlib import Path
import subprocess
import sys
import unittest
import compare as adapter

HERE = Path(__file__).resolve().parent


class ExecutableContract(unittest.TestCase):
    def test_three_observations_match(self):
        run = subprocess.run([sys.executable, str(HERE / 'compare.py')],
                             capture_output=True, text=True)
        self.assertEqual(run.returncode, 0, run.stderr)
        result = json.loads(run.stdout)
        self.assertEqual(result['scope'], 'three-fixed-local-positives')
        self.assertEqual([r['case'] for r in result['cases']],
                         ['T-10-1', 'R-30', 'R-near-bound'])
        self.assertTrue(all(r['equal'] for r in result['cases']))
        self.assertEqual(result['typescriptStatus'], 'PreparedUnqualified')
        self.assertEqual(result['normalization']['kAccountsByCase'], {
            'T-10-1': {'O': 'O', 'R': 'R', 'F': 'F'},
            'R-30': {'P': 'O', 'C': 'C'},
            'R-near-bound': {'P': 'O', 'C': 'C'},
        })
        self.assertEqual(result['normalization']['headsBySource'], {
            'K': {'h0': '0', 'h1': '1'}, 'TypeScript': {'h0': '0', 'h1': '1'},
            'Quint': {'0': '0', '1': '1'},
        })


class IndependentChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.k = json.loads(adapter.K_RECEIPT.read_text())
        cls.q = {name: json.loads((HERE / f'quint_{test}_{index}.itf.json').read_text())
                 for index, (name, test) in enumerate(adapter.CASES.items())}
        run = subprocess.run(['node', str(HERE / 'direct-core.mjs')], check=True,
                             capture_output=True, text=True)
        cls.ts = json.loads(run.stdout)

    def test_independent_literal_transfer_post(self):
        p = adapter.project_quint(self.q['T-10-1'], 'T-10-1')
        self.assertEqual(p['post'], {
            'balances': {'O': '89', 'R': '10', 'F': '1'},
            'allowances': {'O': {'remaining': '0', 'spent': '11'}},
            'obligations': {}, 'head': '1', 'round': '0',
            'consumedReplay': [['D', 'O', 'T-10-1']],
            'workRemaining': '0', 'workSpent': '1',
        })
        self.assertEqual([line['kind'] for line in p['effects']],
                         ['Debit', 'Credit', 'Credit', 'UseAllowance', 'UseReplay', 'AdvanceHead'])

    def test_independent_literal_repayment_and_bound(self):
        thirty = adapter.project_k(self.k[1])
        self.assertEqual(thirty['post']['balances'], {'O': '70', 'C': '30'})
        self.assertEqual(thirty['post']['obligations'], {'L': {
            'debtor': 'O', 'creditor': 'C', 'asset': 'A', 'principal': '980',
            'accrued': '0', 'outstanding': '980', 'status': 'Outstanding',
        }})
        near = adapter.project_k(self.k[2])
        self.assertEqual(near['post']['balances']['C'], '340282366920938463463374607431768211455')
        self.assertEqual(near['post']['allowances']['O']['spent'], '340282366920938463463374607431768211455')
        self.assertEqual(near['post']['obligations']['L']['principal'], '170141183460469231731687303715884105726')

    def test_expected_receipt_and_match_flags_are_not_oracles(self):
        rows = copy.deepcopy(self.k)
        for row in rows:
            row['expectedOut'] = 'rejected()'
            row['matched'] = False
            row['externalPreserved'] = False
        self.assertEqual(adapter.compare(rows, self.q, self.ts),
                         adapter.compare(self.k, self.q, self.ts))

    def test_coherent_k_output_mutation_fails_comparison(self):
        rows = copy.deepcopy(self.k)
        for field in ['observedOut', 'stdout']:
            rows[0][field] = rows[0][field].replace('"A" , 89', '"A" , 88')
        with self.assertRaisesRegex(ValueError, 'K/Quint post mismatch'):
            adapter.compare(rows, self.q, self.ts)

    def test_quint_fabricated_zero_allowance_fails(self):
        traces = copy.deepcopy(self.q)
        post = traces['T-10-1']['states'][-1]
        for field in ['allowanceRemaining', 'allowanceSpent']:
            key = next(k for k in post if k.endswith('::' + field))
            post[key]['#map'].append(['R', {'#bigint': '0'}])
        with self.assertRaisesRegex(ValueError, 'K/Quint post mismatch'):
            adapter.compare(self.k, traces, self.ts)

    def test_effect_order_mutation_fails(self):
        traces = copy.deepcopy(self.q)
        post = traces['T-10-1']['states'][-1]
        key = next(k for k in post if k.endswith('::lastEffects'))
        post[key][0], post[key][1] = post[key][1], post[key][0]
        with self.assertRaisesRegex(ValueError, 'K/Quint effects mismatch'):
            adapter.compare(self.k, traces, self.ts)

    def test_one_unit_near_bound_mutation_fails(self):
        rows = copy.deepcopy(self.ts)
        rows[2]['result']['candidatePost']['balances'][1]['amount'] = '340282366920938463463374607431768211454'
        with self.assertRaisesRegex(ValueError, 'K/TypeScript post mismatch'):
            adapter.compare(self.k, self.q, rows)

    def test_trailing_k_term_and_float_itf_fail_closed(self):
        with self.assertRaisesRegex(ValueError, 'Trailing K term'):
            adapter.parse_term('noEffects()noEffects()')
        with self.assertRaisesRegex(ValueError, 'exact #bigint'):
            adapter.decode_itf(340282366920938463463374607431768211455.0)

    def test_quint_foreign_suffix_cannot_mask_changed_balance(self):
        traces = copy.deepcopy(self.q)
        post = traces['T-10-1']['states'][-1]
        key = next(k for k in post if k.endswith('::balances'))
        original = copy.deepcopy(post[key])
        for owner, amount in post[key]['#map']:
            if owner == 'O':
                amount['#bigint'] = '88'
        post['foreign::balances'] = original
        with self.assertRaisesRegex(ValueError, 'Quint state namespace/footprint'):
            adapter.compare(self.k, traces, self.ts)

    def test_k_quoted_integer_is_not_numeric_observation(self):
        rows = copy.deepcopy(self.k)
        for field in ['observedOut', 'stdout']:
            rows[0][field] = rows[0][field].replace('"A" , 89', '"A" , "89"')
        with self.assertRaisesRegex(ValueError, 'K integer type'):
            adapter.compare(rows, self.q, self.ts)

    def test_quint_plain_numeric_string_is_not_bigint(self):
        traces = copy.deepcopy(self.q)
        post = traces['T-10-1']['states'][-1]
        key = next(k for k in post if k.endswith('::balances'))
        post[key]['#map'] = [[owner, '89' if owner == 'O' else amount]
                            for owner, amount in post[key]['#map']]
        with self.assertRaisesRegex(ValueError, 'Quint integer type'):
            adapter.compare(self.k, traces, self.ts)

    def test_itf_dual_tuple_set_cannot_mask_replay_mutation(self):
        traces = copy.deepcopy(self.q)
        post = traces['T-10-1']['states'][-1]
        key = next(k for k in post if k.endswith('::lastEffects'))
        value = next(line['value'] for line in post[key] if line['tag'] == 'UseReplay')
        original = copy.deepcopy(value['#tup'])
        value['#tup'][2] = 'changed-nonce'
        value['#set'] = original
        with self.assertRaisesRegex(ValueError, 'ITF wrapper exclusivity'):
            adapter.compare(self.k, traces, self.ts)

    def test_itf_tuple_cannot_mask_record_fields(self):
        traces = copy.deepcopy(self.q)
        post = traces['T-10-1']['states'][-1]
        key = next(k for k in post if k.endswith('::lastEffects'))
        value = next(line['value'] for line in post[key] if line['tag'] == 'UseReplay')
        value.update({'domain': 'changed-domain', 'signer': 'changed-signer',
                      'nonce': 'changed-nonce'})
        with self.assertRaisesRegex(ValueError, 'ITF wrapper exclusivity'):
            adapter.compare(self.k, traces, self.ts)

    def test_k_opaque_numeric_post_head_is_not_h1(self):
        rows = copy.deepcopy(self.k)
        old = rows[0]['observedOut']
        marker = 'head ( "h1" )'
        index = old.rfind(marker)
        self.assertGreaterEqual(index, 0)
        rows[0]['observedOut'] = old[:index] + old[index:].replace(marker, 'head ( "1" )', 1)
        rows[0]['stdout'] = re.sub(r'<out>.*?</out>',
                                   lambda _: '<out>' + rows[0]['observedOut'] + '</out>',
                                   rows[0]['stdout'], flags=re.S)
        with self.assertRaisesRegex(ValueError, 'K head representation'):
            adapter.compare(rows, self.q, self.ts)

    def test_k_unquoted_head_integer_is_not_opaque_head(self):
        rows = copy.deepcopy(self.k)
        for field in ['observedOut', 'stdout', 'request']:
            rows[0][field] = rows[0][field].replace('head ( "h0" )', 'head ( 0 )')
            rows[0][field] = rows[0][field].replace('head("h0")', 'head(0)')
        with self.assertRaisesRegex(ValueError, 'K head representation'):
            adapter.compare(rows, self.q, self.ts)

    def test_typescript_numeric_post_head_is_not_h1(self):
        rows = copy.deepcopy(self.ts)
        rows[0]['result']['candidatePost']['head'] = '1'
        with self.assertRaisesRegex(ValueError, 'TypeScript head representation'):
            adapter.compare(self.k, self.q, rows)

    def test_quint_empty_string_is_not_consumed_set(self):
        traces = copy.deepcopy(self.q)
        pre = traces['T-10-1']['states'][-2]
        key = next(k for k in pre if k.endswith('::consumed'))
        pre[key] = ''
        with self.assertRaisesRegex(ValueError, 'Quint consumed set type'):
            adapter.compare(self.k, traces, self.ts)

    def test_quint_consumed_plain_list_is_not_set(self):
        traces = copy.deepcopy(self.q)
        pre = traces['T-10-1']['states'][-2]
        key = next(k for k in pre if k.endswith('::consumed'))
        pre[key] = []
        with self.assertRaisesRegex(ValueError, 'Quint consumed set type'):
            adapter.compare(self.k, traces, self.ts)

    def test_quint_replay_entry_requires_tuple_wrapper(self):
        for replacement in [{'#set': ['D', 'O', 'T-10-1']}, ['D', 'O', 'T-10-1']]:
            with self.subTest(replacement=replacement):
                traces = copy.deepcopy(self.q)
                post = traces['T-10-1']['states'][-1]
                key = next(k for k in post if k.endswith('::consumed'))
                post[key]['#set'] = [replacement]
                with self.assertRaisesRegex(ValueError, 'Quint replay tuple type'):
                    adapter.compare(self.k, traces, self.ts)

    def test_quint_replay_effect_requires_tuple_wrapper(self):
        for replacement in [{'#set': ['D', 'O', 'T-10-1']}, ['D', 'O', 'T-10-1']]:
            with self.subTest(replacement=replacement):
                traces = copy.deepcopy(self.q)
                post = traces['T-10-1']['states'][-1]
                key = next(k for k in post if k.endswith('::lastEffects'))
                for line in post[key]:
                    if line['tag'] == 'UseReplay':
                        line['value'] = replacement
                with self.assertRaisesRegex(ValueError, 'Quint replay tuple type'):
                    adapter.compare(self.k, traces, self.ts)

    def test_quint_effect_debt_status_requires_tuple_wrapper(self):
        traces = copy.deepcopy(self.q)
        post = traces['R-30']['states'][-1]
        key = next(k for k in post if k.endswith('::lastEffects'))
        line = next(line for line in post[key] if line['tag'] == 'SetObligation')
        line['value']['value']['status']['value'] = {'#set': []}
        with self.assertRaisesRegex(ValueError, 'Quint debt status'):
            adapter.compare(self.k, traces, self.ts)

    def test_k_repayment_raw_o_cannot_alias_source_p(self):
        rows = copy.deepcopy(self.k)
        old = rows[1]['observedOut']
        marker = 'balance ( "P" , "A" , 70 )'
        self.assertIn(marker, old)
        rows[1]['observedOut'] = old.replace(marker, 'balance ( "O" , "A" , 70 )', 1)
        rows[1]['stdout'] = re.sub(r'<out>.*?</out>',
                                   lambda _: '<out>' + rows[1]['observedOut'] + '</out>',
                                   rows[1]['stdout'], flags=re.S)
        with self.assertRaisesRegex(ValueError, 'K account outside case-specific source domain'):
            adapter.compare(rows, self.q, self.ts)


if __name__ == '__main__':
    unittest.main()
