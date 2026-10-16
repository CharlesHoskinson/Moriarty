'''Independent wire expectations for the host_codec prove-request APIs.

Author: claude-opus-5-5 security source-contract author; source text only, never run.
Not an audit, implementation or adoption. Contract: CORE-REQUEST-WIRE-CONTRACT.md.

Stdlib unittest only. host_codec.py is loaded solely from the exact regular sibling path via
spec_from_file_location after a missing/symlink assertion; a missing module is an ordinary RED
failure containing 'missing implementation'. No writes, subprocess, entropy, keys, signing,
curves, proofs, SDK or network. hashlib SHA-256 only pins the public sibling K4H-GOLD.json.

Gold crosschecks compare literal fixture data with that independent GOLD by label lookup only.
They are not Rust execution, compiler coverage, proof validity or a financial evaluator. Records
fixtures are shapes a future compiler consumes; native acceptance is not asserted. The validator
under test is structural and must NOT enforce action semantics.
'''
import copy
import hashlib
import importlib.util
import inspect
import json
import sys
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
CODEC_PATH = HERE / 'host_codec.py'
GOLD_PATH = HERE / 'K4H-GOLD.json'
GOLD_BYTES = 39353
GOLD_SHA256 = '65c90d3d13912b762b4977e08ea86212b2b83a16bb2b3ddcaa7a21c905075e46'
AUTH = 'CALLER_SUPPLIED_UNAUTHENTICATED'
U128_MAX = 2 ** 128 - 1
U64_MAX = 2 ** 64 - 1
DERIVED = 'COMPILER_DERIVED_NOT_IN_WIRE'
HEX_FIELDS = {'claimantVkh': 28, 'settleHash': 28, 'threadId': 32, 'consumedTxid': 32,
              'catalogueDigest': 32, 'intentsDigest': 32}
_LOADED = {}


def load_codec(case):
    case.assertFalse(CODEC_PATH.is_symlink(), 'refused: host_codec.py is a symlink')
    case.assertTrue(CODEC_PATH.is_file(), 'missing implementation: expected regular sibling host_codec.py')
    if 'codec' not in _LOADED:
        spec = importlib.util.spec_from_file_location('core_request_wire_host_codec', str(CODEC_PATH))
        module = importlib.util.module_from_spec(spec)
        sys.modules[spec.name] = module
        spec.loader.exec_module(module)
        _LOADED['codec'] = module
    case.assertEqual(Path(_LOADED['codec'].__file__), CODEC_PATH)
    return _LOADED['codec']


def load_gold(case):
    case.assertFalse(GOLD_PATH.is_symlink(), 'refused: K4H-GOLD.json is a symlink')
    case.assertTrue(GOLD_PATH.is_file(), 'missing staged gold: expected regular sibling K4H-GOLD.json')
    if 'gold' not in _LOADED:
        case.assertEqual(GOLD_PATH.stat().st_size, GOLD_BYTES)
        data = GOLD_PATH.read_bytes()
        case.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (GOLD_BYTES, GOLD_SHA256))
        _LOADED['gold'] = json.loads(data.decode('utf-8'))
    return _LOADED['gold']


def U(n):
    return {'type': 'UInt', 'value': str(n)}


def B(b):
    return {'type': 'Bool', 'value': b}


def asset(issuer, reference):
    return {'network': 'oracle-unit', 'issuer': issuer, 'reference': reference, 'scale': 0}


ASSETS = {'CASH': asset('invoice-issuer', 'cash'), 'A': asset('reserve-issuer', 'A'),
          'B': asset('reserve-issuer', 'B')}


def bal(a, account, amount):
    return {'account': account, 'asset': dict(ASSETS[a]), 'amount': str(amount)}


def duty(name, a, debtor, creditor, amount):
    return {'name': name, 'asset': dict(ASSETS[a]), 'debtor': debtor, 'creditor': creditor,
            'amount': str(amount)}


def move(kind, a, frm, to, amount):
    return {'kind': kind, 'asset': dict(ASSETS[a]), 'from': frm, 'to': to, 'amount': str(amount)}


def liab(name, a, debtor, creditor, before, after):
    return {'kind': 'Liability', 'name': name, 'asset': dict(ASSETS[a]), 'debtor': debtor,
            'creditor': creditor, 'before': str(before), 'after': str(after)}


def state(name, before, after):
    return {'kind': 'State', 'name': name, 'before': before, 'after': after}


def original_request():
    '''Transcription of fixtures/invoice-partial.request.json (3008 bytes, sha256
    aa1703e7b3aa2e0dd6288668cc871996fc1ee49ecc90861a3c4b40f02107f3cd): CoreReference in the
    original array order. Root must check json.loads(exact bytes) == this literal.'''
    return {
        'format': 'moriarty-k2-prove-request/1',
        'selector': {'format': 'moriarty-k2-relation-selector/1', 'language': 'moriarty/1',
                     'semantics': 'moriarty-core/1', 'action': 'partial', 'adapter': 'SettlementHashV2',
                     'cap': 'NotBound', 'accounting': 'GrossDebitAndFees', 'frame': 'TouchedOnly'},
        'transition': {'kind': 'CoreReference', 'scenario': {
            'balances': [bal('CASH', 'escrow', 109), bal('CASH', 'spectator', 41),
                         bal('CASH', 'fees', 0), bal('CASH', 'seller', 0)],
            'inputs': {'amount': U(35)},
            'liabilities': [duty('invoice', 'CASH', 'escrow', 'seller', 90),
                            duty('reserve', 'CASH', 'escrow', 'buyer', 14)],
            'state': {'marker': U(7), 'opened': B(True), 'paid': U(0)}}},
        'context': {
            'claimantVkh': '5' * 56, 'networkId': 0, 'settleHash': 'a2' * 28,
            'threadId': '7a7fafdd81303746cd8af0fc48add098ab6468e5c510042d41be69f33abcf29a',
            'consumedTxid': '2' * 64, 'consumedIx': 0, 'seq': 1,
            'catalogueDigest': '37de4837bc1417d65df0c40b4829d796b04d97fd037bb128fc0814abb5646e18',
            'intentsDigest': 'b8e2f734456ad3d939e91441330cb0635707e09d70af45edead18923108fc308'}}


def CTX():
    return original_request()['context']


def invoice_records():
    '''Complete invoice/partial Records claim from K4H GOLD, the original scenario and
    installment-invoice.mori: Fee 2, Transfer 35, Liability invoice 90->55, State paid 0->35;
    marker 7, opened true, reserve 14, spectator 41 unchanged. Arrays use Core canonical
    (asset, account)/(name) order: a derived array, not the original (R4-A03).'''
    def snap(escrow, fees, seller, invoice, paid):
        return {'state': {'marker': U(7), 'opened': B(True), 'paid': U(paid)},
                'balances': [bal('CASH', 'escrow', escrow), bal('CASH', 'fees', fees),
                             bal('CASH', 'seller', seller), bal('CASH', 'spectator', 41)],
                'liabilities': [duty('invoice', 'CASH', 'escrow', 'seller', invoice),
                                duty('reserve', 'CASH', 'escrow', 'buyer', 14)]}
    return {'kind': 'Records', 'inputs': {'amount': U(35)},
            'pre': snap(109, 0, 0, 90, 0), 'post': snap(72, 2, 35, 55, 35),
            'effects': [move('Fee', 'CASH', 'escrow', 'fees', 2),
                        move('Transfer', 'CASH', 'escrow', 'seller', 35),
                        liab('invoice', 'CASH', 'escrow', 'seller', 90, 55),
                        state('paid', U(0), U(35))]}


def reserve_records():
    '''reserve/allocate Records claim from K4H GOLD and conditional-reserve.mori. The original
    reserve request is unread: outside rows come from GOLD outsideStatement, order is Core
    canonical. Narrow wire/gold coverage only.'''
    def snap(cells, allocated, rounds, duties):
        return {'state': {'allocated': B(allocated), 'marker': U(77), 'rounds': U(rounds)},
                'balances': [bal(a, account, n) for a, account, n in cells], 'liabilities': duties}
    pre = [('A', 'alpha', 0), ('A', 'beta', 2), ('A', 'fees', 5), ('A', 'observer', 17), ('A', 'vault', 40),
           ('B', 'alpha', 0), ('B', 'beta', 4), ('B', 'fees', 0), ('B', 'observer', 9), ('B', 'vault', 30)]
    post = [('A', 'alpha', 14), ('A', 'beta', 9), ('A', 'fees', 7), ('A', 'observer', 17), ('A', 'vault', 17),
            ('B', 'alpha', 4), ('B', 'beta', 4), ('B', 'fees', 0), ('B', 'observer', 9), ('B', 'vault', 26)]
    return {'kind': 'Records',
            'inputs': {'approved': B(True), 'quotient': U(7), 'remainder': U(2), 'units': U(23)},
            'pre': snap(pre, False, 0, []),
            'post': snap(post, True, 1, [duty('allocation_reserve', 'A', 'vault', 'owner', 2),
                                         duty('rebate', 'B', 'vault', 'owner', 6)]),
            'effects': [move('Fee', 'A', 'vault', 'fees', 2), move('Transfer', 'A', 'vault', 'alpha', 14),
                        move('Transfer', 'A', 'vault', 'beta', 7), move('Transfer', 'B', 'vault', 'alpha', 4),
                        liab('allocation_reserve', 'A', 'vault', 'owner', 0, 2),
                        liab('rebate', 'B', 'vault', 'owner', 0, 6),
                        state('allocated', B(False), B(True)), state('rounds', U(0), U(1))]}


def literal_request(transition, action='partial', context=Ellipsis):
    r = original_request()
    r['selector']['action'] = action
    r['transition'] = copy.deepcopy(transition)
    if context is not Ellipsis:
        r['context'] = copy.deepcopy(context)
    return r


def plain(v):
    if isinstance(v, dict) and set(v) == {'type', 'value'} and v['type'] in ('UInt', 'Bool'):
        return v['value']
    return ('BAD', v)


def same(a, b):
    return type(a) is type(b) and a == b


def slot_value(inputs, snaps, effects, label):
    '''Label lookup only; no arithmetic. accounting.* rows are compiler-derived, not wire data.'''
    p = label.split('.')
    if p[0] == 'accounting':
        return DERIVED
    if p[0] == 'input':
        return plain(inputs.get(p[1]))
    if p[0] == 'effect':
        e = effects[int(p[1])] if int(p[1]) < len(effects) else {}
        if str(e.get('kind')).lower() != p[2]:
            return ('KIND', e.get('kind'))
        if p[2] in ('fee', 'transfer'):
            return e.get('amount') if [e.get('from'), e.get('to')] == p[3:5] else ('ENDS', e.get('from'))
        if e.get('name') != p[3]:
            return ('NAME', e.get('name'))
        return plain(e.get(p[4])) if p[2] == 'state' else e.get(p[4])
    snap = snaps[p[0]]
    if p[1] == 'state':
        return plain(snap['state'].get(p[2]))
    if p[1] == 'cell':
        rows = [b['amount'] for b in snap['balances'] if b['asset'] == ASSETS[p[2]] and b['account'] == p[3]]
    else:
        rows = [d['amount'] for d in snap['liabilities'] if d['name'] == p[2]]
        if p[3] == 'present':
            return {0: False, 1: True}.get(len(rows), ('ROWS', len(rows)))
        if not rows:
            return '0'  # absent named duty encodes (false, 0) (CLAIM-COMPARISON C09)
    return rows[0] if len(rows) == 1 else ('ROWS', len(rows))


def slot_mismatches(gold, relation, inputs, snaps, effects, prefixes=('',)):
    bad, derived = [], []
    for row in gold['relations'][relation]['slots']:
        if row['label'].startswith(prefixes):
            got = slot_value(inputs, snaps, effects, row['label'])
            if got is DERIVED:
                derived.append(row['label'])
            elif not same(got, row['value']):
                bad.append((row['label'], row['value'], got))
    return bad, derived


def gold_mismatches(gold, relation, rec):
    rel = gold['relations'][relation]
    fin, effects = rel['financial'], rec['effects']
    bad, derived = slot_mismatches(gold, relation, rec['inputs'], rec, effects)
    if [e.get('kind') for e in effects] != rel['effects']:
        bad.append(('effectKinds',))
    duties = {d['duty']: d for d in fin['liabilities']}
    for g in fin['orderedEffects']:
        e = effects[g['index']] if g['index'] < len(effects) else {}
        if g['kind'] == 'State':
            ok = (e.get('kind') == 'State' and e.get('name') == g['name']
                  and same(plain(e.get('before')), g['before']) and same(plain(e.get('after')), g['after']))
        elif g['kind'] in ('Fee', 'Transfer'):
            ok = e == {'kind': g['kind'], 'asset': ASSETS[g['asset']], 'from': g['from'], 'to': g['to'],
                       'amount': g['amount']}
        else:
            d = duties[g['duty']]
            ok = e == {'kind': 'Liability', 'name': g['duty'], 'asset': ASSETS[d['asset']], 'debtor': d['debtor'],
                       'creditor': d['creditor'], 'before': g['before'], 'after': g['after']}
        if not ok:
            bad.append(('orderedEffect', g['index']))
    residual = [{'duty': d['name'], 'asset': d['asset'], 'debtor': d['debtor'], 'creditor': d['creditor'],
                 'amount': d['amount']} for d in rec['post']['liabilities']]
    if residual != [dict(r, asset=ASSETS[r['asset']]) for r in fin['residualDuties']]:
        bad.append(('residualDuties',))
    for o in fin['outsideStatement']:
        a, account = o['cell'].split('.')
        for phase in ('pre', 'post'):
            rows = [b['amount'] for b in rec[phase]['balances'] if b['asset'] == ASSETS[a] and b['account'] == account]
            if rows != [o['amount']]:
                bad.append(('outside', phase, o['cell']))
    return bad, derived


class TestGoldFixtures(unittest.TestCase):
    '''Literal fixtures versus the sha-pinned independent GOLD; no host_codec needed.'''

    def setUp(self):
        self.gold = load_gold(self)

    def test_gold_pin_and_all_75_rows(self):
        inv, res = (self.gold['relations'][k] for k in ('invoice/partial', 'reserve/allocate'))
        self.assertEqual((len(inv['slots']), len(res['slots'])), (29, 46))
        self.assertEqual(inv['slots'][0], {'label': 'input.amount', 'encoding': 'uint128', 'value': '35'})
        self.assertEqual(inv['slots'][-1], {'label': 'accounting.CASH.escrow.fees_paid', 'encoding': 'wide144', 'value': '2'})
        self.assertEqual(res['slots'][0], {'label': 'input.approved', 'encoding': 'bool', 'value': True})
        self.assertEqual(res['slots'][-1], {'label': 'accounting.B.vault.fees_paid', 'encoding': 'wide144', 'value': '0'})
        self.assertEqual(inv['effects'], ['Fee', 'Transfer', 'Liability', 'State'])
        self.assertEqual(res['effects'], ['Fee', 'Transfer', 'Transfer', 'Transfer', 'Liability', 'Liability', 'State', 'State'])
        for rel in (inv, res):
            labels = [s['label'] for s in rel['slots']]
            self.assertEqual(len(set(labels)), len(labels))
            # Historical synthetic VK digests; never a current VK or relation.
            self.assertTrue(rel['declaredVkDigestOrigin'].startswith('SYNTHETIC'))
            for s in rel['slots']:
                if s['encoding'] == 'bool':
                    self.assertIs(type(s['value']), bool)
                else:
                    v = s['value']
                    self.assertIn(s['encoding'], ('uint128', 'wide144'))
                    self.assertTrue(v.isascii() and v.isdigit() and (v == '0' or v[0] != '0'), s)

    def test_invoice_records_fixture_matches_gold(self):
        bad, derived = gold_mismatches(self.gold, 'invoice/partial', invoice_records())
        self.assertEqual(bad, [])
        self.assertEqual(derived, ['accounting.CASH.escrow.gross_debit', 'accounting.CASH.escrow.fees_paid'])

    def test_reserve_records_fixture_matches_gold(self):
        bad, derived = gold_mismatches(self.gold, 'reserve/allocate', reserve_records())
        self.assertEqual(bad, [])
        self.assertEqual(derived, ['accounting.A.vault.gross_debit', 'accounting.A.vault.fees_paid',
                                   'accounting.B.vault.gross_debit', 'accounting.B.vault.fees_paid'])

    def test_original_core_reference_inputs_and_pre_rows_match_gold(self):
        sc = original_request()['transition']['scenario']
        prefixes = ('input.', 'pre.')
        self.assertEqual(slot_mismatches(self.gold, 'invoice/partial', sc['inputs'], {'pre': sc}, [], prefixes), ([], []))
        rows = [s for s in self.gold['relations']['invoice/partial']['slots'] if s['label'].startswith(prefixes)]
        self.assertEqual(len(rows), 11)
        self.assertEqual([b['amount'] for b in sc['balances'] if b['account'] == 'spectator'], ['41'])

    def test_crosscheck_catches_each_old_broad_fixture_error(self):
        def reasset(t):
            other = {'network': 'moriarty-test', 'issuer': 'issuer', 'reference': 'invoice', 'scale': 0}
            rows = [r for ph in ('pre', 'post') for r in t[ph]['balances'] + t[ph]['liabilities']]
            for row in rows + t['effects']:
                if 'asset' in row:
                    row['asset'] = dict(other)
        for mutate in (lambda t: t['effects'].insert(0, t['effects'].pop(1)),
                       lambda t: t['effects'].pop(),
                       lambda t: [t[ph].__setitem__('balances', t[ph]['balances'][:1]) for ph in ('pre', 'post')],
                       lambda t: [t[ph]['liabilities'].pop() for ph in ('pre', 'post')],
                       lambda t: t.__setitem__('inputs', {}),
                       reasset):
            t = invoice_records()
            mutate(t)
            self.assertNotEqual(gold_mismatches(self.gold, 'invoice/partial', t)[0], [])

    def test_derived_records_order_is_a_new_array_not_the_original(self):
        sc, rec = original_request()['transition']['scenario'], invoice_records()
        key = lambda b: (b['account'], b['amount'])
        self.assertEqual(sorted(map(key, sc['balances'])), sorted(map(key, rec['pre']['balances'])))
        self.assertNotEqual(sc['balances'], rec['pre']['balances'])
        self.assertEqual((sc['state'], sc['liabilities'], sc['inputs']),
                         (rec['pre']['state'], rec['pre']['liabilities'], rec['inputs']))


class CodecCase(unittest.TestCase):
    def setUp(self):
        self.c = load_codec(self)

    def build(self, transition, action='partial', context=Ellipsis, **pins):
        return self.c.build_prove_request(action, transition, CTX() if context is Ellipsis else context, **pins)

    def ok(self, transition, action='partial', context=Ellipsis, **pins):
        req = self.build(copy.deepcopy(transition), action, copy.deepcopy(context), **pins)
        self.assertEqual(req['transition'], transition)
        self.assertIs(self.c.validate_prove_request(req), True)
        return req

    def bad_request(self, request):
        with self.assertRaises(ValueError):
            self.c.validate_prove_request(request)

    def refuse(self, transition, action='partial'):
        with self.assertRaises(ValueError):
            self.build(copy.deepcopy(transition), action)
        self.bad_request(literal_request(transition, action))

    def refuse_context(self, ctx):
        with self.assertRaises(ValueError):
            self.build(invoice_records(), context=copy.deepcopy(ctx))
        self.bad_request(literal_request(invoice_records(), context=ctx))
        with self.assertRaises(ValueError):
            self.c.validate_context_input(copy.deepcopy(ctx))


class TestExactRequests(CodecCase):
    def test_frozen_signatures(self):
        params = inspect.signature(self.c.build_prove_request).parameters
        self.assertEqual(list(params), ['action', 'transition', 'context', 'expected_source_sha256', 'expected_program_hash'])
        self.assertIsNone(params['expected_source_sha256'].default)
        self.assertIsNone(params['expected_program_hash'].default)
        for name in ('validate_prove_request', 'validate_context_input'):
            self.assertEqual(len(inspect.signature(getattr(self.c, name)).parameters), 1)

    def test_original_core_reference_request_is_rebuilt_exactly(self):
        orig = original_request()
        req = self.c.build_prove_request('partial', orig['transition'], orig['context'])
        self.assertEqual(req, orig)
        self.assertIs(self.c.validate_prove_request(orig), True)
        scenario = req['transition']['scenario']
        self.assertEqual([b['account'] for b in scenario['balances']], ['escrow', 'spectator', 'fees', 'seller'])
        self.assertEqual([d['name'] for d in scenario['liabilities']], ['invoice', 'reserve'])
        # CoreReference carries no pre/post/effects: Core simulate proposes them inside Rust.
        self.assertEqual(set(req['transition']), {'kind', 'scenario'})

    def test_records_fixtures_are_built_verbatim(self):
        for action, fx in (('partial', invoice_records()), ('allocate', reserve_records())):
            with self.subTest(action=action):
                self.assertEqual(self.ok(fx, action), literal_request(fx, action))

    def test_action_is_any_1_to_1024_utf8_bytes_without_allowlist(self):
        for action in ('anotherProgramAction', 'allocate', 'x', 'a' * 1024, chr(0xe9) * 512):
            self.assertEqual(self.ok(invoice_records(), action)['selector']['action'], action)
        for action in ('', 'a' * 1025, chr(0xe9) * 513, chr(0xd800), None, 7, b'partial'):
            with self.subTest(action=action):
                self.refuse(invoice_records(), action)

    def test_pins_are_optional_selector_fields_and_source_text_is_never_embedded(self):
        src, prog = 'cc' * 32, 'bb' * 32
        req = self.ok(invoice_records(), expected_source_sha256=src, expected_program_hash=prog)
        self.assertEqual((req['selector']['expectedSourceSha256'], req['selector']['expectedProgramHash']), (src, prog))
        for key in ('source', 'sourceText', 'sourceBytes', 'program'):
            self.assertNotIn(key, req)
            self.assertNotIn(key, req['selector'])
        self.assertNotIn('expectedSourceSha256', self.ok(invoice_records(), expected_program_hash=prog)['selector'])
        nulls = original_request()
        nulls['selector'].update(expectedSourceSha256=None, expectedProgramHash=None)
        self.assertIs(self.c.validate_prove_request(nulls), True)  # serde Option + default
        for bad in ('CC' * 32, 'cc' * 31, 'cc' * 33, '', 'gg' * 32, ' ' + 'c' * 63, 1, True, bytes([0xcc]) * 32):
            for kw, field in (('expected_source_sha256', 'expectedSourceSha256'),
                              ('expected_program_hash', 'expectedProgramHash')):
                with self.subTest(pin=field, bad=bad):
                    with self.assertRaises(ValueError):
                        self.build(invoice_records(), **{kw: bad})
                    r = original_request()
                    r['selector'][field] = bad
                    self.bad_request(r)


class TestClosedWire(CodecCase):
    def test_fixed_selector_values_and_formats(self):
        for field, value in (('format', 'moriarty-k2-relation-selector/2'), ('language', 'moriarty/2'),
                             ('language', 'Moriarty/1'), ('semantics', 'moriarty-core/2'),
                             ('adapter', 'DirectFieldV0'), ('adapter', 'ClaimantBoundAbiV1Pending'),
                             ('cap', 'PublicCap'), ('accounting', 'GrossDebit'), ('frame', 'Include'), ('frame', None)):
            r = original_request()
            r['selector'][field] = value
            self.bad_request(r)
        for value in ('moriarty-k2-prove-request/2', '', None):
            r = original_request()
            r['format'] = value
            self.bad_request(r)

    def test_closed_field_sets_camel_case_and_required_keys(self):
        for path, key in (((), 'source'), ((), 'sourceText'), ((), 'unexpected'),
                          (('selector',), 'expected_source_sha256'), (('selector',), 'mode'),
                          (('context',), 'vk_digest'), (('context',), 'authentication'),
                          (('transition',), 'mode'), (('transition',), 'core_ir'),
                          (('transition', 'scenario'), 'program')):
            r = original_request()
            node = r
            for p in path:
                node = node[p]
            node[key] = AUTH if key == 'authentication' else 'cc' * 32
            self.bad_request(r)
        for part in (None, 'selector', 'context', 'transition'):
            for key in list(original_request() if part is None else original_request()[part]):
                r = original_request()
                del (r if part is None else r[part])[key]
                self.bad_request(r)
        r = original_request()
        r[1] = 'x'
        self.bad_request(r)
        for value in (None, [], 'request', 1):
            self.bad_request(value)

    def test_transition_variants_are_exact(self):
        inv, sc = invoice_records(), original_request()['transition']['scenario']
        cases = [{'kind': 'CoreReference', 'scenario': sc, 'effects': inv['effects']},
                 {'kind': 'CoreReference', 'scenario': sc, 'pre': inv['pre'], 'post': inv['post']},
                 {'kind': 'CoreReference', 'scenario': sc, 'mode': 'simulate'},
                 {'kind': 'CoreReference', 'scenario': sc, 'core_ir': {}},
                 {'kind': 'CoreReference'}, {'scenario': sc}, {'kind': 'Reference', 'scenario': sc},
                 {'kind': 'coreReference', 'scenario': sc}, {'kind': 0, 'scenario': sc},
                 {'kind': 'Records', 'scenario': sc}, dict(inv, scenario=sc), dict(inv, kind='records'),
                 None, [], 'CoreReference']
        cases += [{k: v for k, v in inv.items() if k != drop} for drop in inv]
        for t in cases:
            self.refuse(t)

    def test_scenario_and_snapshot_fields_are_required_and_closed(self):
        # Scenario derives Default but has no #[serde(default)]: all four fields are required.
        for key in ('state', 'inputs', 'balances', 'liabilities'):
            t = original_request()['transition']
            del t['scenario'][key]
            self.refuse(t)
        t = original_request()['transition']
        t['scenario']['balances'][0]['amount'] = 109
        self.refuse(t)
        mutations = [lambda t, ph=ph, k=k: t[ph].pop(k) for ph in ('pre', 'post') for k in ('state', 'balances', 'liabilities')]
        mutations += [lambda t: t['pre'].__setitem__('inputs', {}),
                      lambda t: t['pre'].__setitem__('balances', {}),
                      lambda t: t['pre'].__setitem__('state', []),
                      lambda t: t['pre']['state'].__setitem__(1, U(1)),
                      lambda t: t['pre']['balances'][0].pop('asset'),
                      lambda t: t['pre']['balances'][0].__setitem__('extra', 1),
                      lambda t: t['pre']['balances'][0].__setitem__('account', 1),
                      lambda t: t['pre']['balances'][0]['asset'].pop('scale'),
                      lambda t: t['pre']['balances'][0]['asset'].__setitem__('name', 'CASH'),
                      lambda t: t['pre']['balances'][0]['asset'].__setitem__('issuer', None),
                      lambda t: t['post']['liabilities'][0].pop('creditor'),
                      lambda t: t['post']['liabilities'][0].__setitem__('kind', 'Liability'),
                      lambda t: t['post']['balances'].append(None),
                      lambda t: t.__setitem__('inputs', None),
                      lambda t: t['inputs'].__setitem__(None, U(1)),
                      lambda t: t.__setitem__('effects', {}),
                      lambda t: t['effects'].append(None),
                      lambda t: t.__setitem__('pre', [])]
        for m in mutations:
            t = invoice_records()
            m(t)
            self.refuse(t)


UINT_PLACES = (lambda t, v: t['inputs'].__setitem__('amount', {'type': 'UInt', 'value': v}),
               lambda t, v: t['pre']['balances'][0].__setitem__('amount', v),
               lambda t, v: t['post']['liabilities'][0].__setitem__('amount', v),
               lambda t, v: t['effects'][0].__setitem__('amount', v),
               lambda t, v: t['effects'][2].__setitem__('after', v))
VALUE_PLACES = (lambda t, v: t['inputs'].__setitem__('amount', v),
                lambda t, v: t['pre']['state'].__setitem__('opened', v),
                lambda t, v: t['post']['state'].__setitem__('marker', v),
                lambda t, v: t['effects'][3].__setitem__('before', v))


def mutated(place, v):
    t = invoice_records()
    place(t, v)
    return t


class TestValues(CodecCase):
    def test_uint_is_canonical_ascii_decimal_up_to_2_128_minus_1(self):
        for place in UINT_PLACES:
            for v in ('0', '35', str(U128_MAX)):
                self.ok(mutated(place, v))
            for v in (str(U128_MAX + 1), '1' + '0' * 39, '00', '01', '0109', '-1', '+1', '1.0', '1e2', ' 1', '1 ',
                      '', '0x10', chr(0x661), chr(0xff11), chr(0xb2), '3' + chr(0x665), 35, 0, True, None, 1.0, ['1']):
                with self.subTest(v=v):
                    self.refuse(mutated(place, v))

    def test_value_is_type_value_tagged_and_bool_is_json_bool(self):
        for place in VALUE_PLACES:
            for v in (U(0), U(U128_MAX), B(True), B(False)):
                self.ok(mutated(place, v))
            for v in ({'type': 'Bool', 'value': 1}, {'type': 'Bool', 'value': 0}, {'type': 'Bool', 'value': 'true'},
                      {'type': 'Bool', 'value': None}, {'type': 'UInt', 'value': True}, {'type': 'UInt', 'value': 7},
                      {'type': 'UInt'}, {'value': '7'}, {'type': 'uint', 'value': '7'}, {'type': 'Int', 'value': '7'},
                      {'type': 'UInt', 'value': '7', 'extra': 1}, {'type': None, 'value': '7'}, '7', 7, True, None,
                      ['UInt', '7']):
                with self.subTest(v=v):
                    self.refuse(mutated(place, v))

    def test_asset_scale_is_wire_u8_not_program_bound_38(self):
        # Core validate refuses program scale > 38 (runtime.rs); the wire type is u8 (ir.rs).
        def scaled(s):
            t = invoice_records()
            for row in (t['pre']['balances'][0], t['effects'][1], t['post']['liabilities'][1]):
                row['asset']['scale'] = s
            return t
        for s in (0, 38, 39, 255):
            self.ok(scaled(s))
        for s in (256, -1, 1000, True, False, 1.0, '0', None):
            self.refuse(scaled(s))

    def test_every_effect_variant_and_typed_fields(self):
        good = [state('opened', B(True), B(False)), state('paid', U(0), U(U128_MAX)),
                move('Transfer', 'CASH', 'escrow', 'seller', 0), move('Fee', 'CASH', 'escrow', 'fees', U128_MAX),
                liab('invoice', 'CASH', 'escrow', 'seller', 90, 55),
                {'kind': 'Emit', 'name': 'evt', 'value': U(1)}, {'kind': 'Emit', 'name': 'evt', 'value': B(False)}]
        t = invoice_records()
        t['effects'] = good
        self.ok(t)
        st, tr, fe, li, em = good[1:6]

        def put(e, k, v):
            e = copy.deepcopy(e)
            e[k] = v
            return e
        bad = [{k: v for k, v in e.items() if k != drop} for e in (st, tr, fe, li, em) for drop in e]
        bad += [put(e, 'extra', 1) for e in (st, tr, fe, li, em)]
        bad += [put(st, 'before', '0'), put(st, 'after', 0), put(st, 'name', 1), put(tr, 'amount', 35),
                put(tr, 'amount', '035'), put(tr, 'from', None), put(tr, 'asset', 'CASH'), put(fe, 'to', ['fees']),
                put(fe, 'asset', dict(ASSETS['CASH'], scale=256)), put(li, 'before', True), put(li, 'after', '-1'),
                put(li, 'debtor', 0), put(li, 'amount', '35'), put(em, 'value', '1'),
                put(em, 'value', {'type': 'UInt', 'value': '1', 'x': 0}), put(st, 'kind', 'Liability'),
                put(li, 'kind', 'Transfer'), put(em, 'kind', 'State')]
        bad += [put(tr, 'kind', k) for k in ('Discharge', 'Accrue', 'Mint', 'transfer', 'TRANSFER', None)]
        for e in bad:
            t = invoice_records()
            t['effects'][0] = e
            with self.subTest(effect=e):
                self.refuse(t)


class TestContext(CodecCase):
    def test_context_ranges_endpoints_and_hex(self):
        for key, value in (('networkId', 0), ('networkId', 1), ('networkId', 255), ('consumedIx', 0),
                           ('consumedIx', 65535), ('seq', 0), ('seq', U64_MAX), ('threadId', '0f' * 32)):
            ctx = CTX()
            ctx[key] = value
            self.assertEqual(self.ok(invoice_records(), context=ctx)['context'], ctx)
        bad = [('networkId', v) for v in (256, -1, True, False, 1.0, '0', None)]
        bad += [('consumedIx', v) for v in (65536, -1, True, 0.0, '0')]
        bad += [('seq', v) for v in (U64_MAX + 1, -1, False, 1.0, '1')]
        for key, n in HEX_FIELDS.items():
            bad += [(key, v) for v in ('ab' * (n - 1), 'ab' * (n + 1), 'AB' * n, 'zz' * n, '0x' + 'ab' * (n - 1),
                                       'ab' * n + 'a', '', None, 1, bytes([0xab]) * n)]
        for key, value in bad:
            ctx = CTX()
            ctx[key] = value
            with self.subTest(key=key, value=value):
                self.refuse_context(ctx)

    def test_vk_digest_is_optional_omitted_or_null_and_never_invented(self):
        self.assertNotIn('vkDigest', self.ok(invoice_records())['context'])
        for value in (None, '5a' * 32):
            ctx = CTX()
            ctx['vkDigest'] = value
            self.assertEqual(self.ok(invoice_records(), context=ctx)['context']['vkDigest'], value)
        for value in ('5A' * 32, '5a' * 31, '5a' * 33, '', 0, False):
            ctx = CTX()
            ctx['vkDigest'] = value
            self.refuse_context(ctx)

    def test_validate_context_input_is_detached_and_unauthenticated(self):
        ctx = CTX()
        before = copy.deepcopy(ctx)
        out = self.c.validate_context_input(ctx)
        self.assertEqual(out, dict(before, authentication=AUTH))
        out['seq'] = 99
        self.assertEqual(ctx, before)
        for network in (0, 1, 2, 255):
            c = CTX()
            c['networkId'] = network
            self.assertEqual(self.c.validate_context_input(c), dict(c, authentication=AUTH))
        c = CTX()
        c['vkDigest'] = None
        self.assertEqual(self.c.validate_context_input(c), dict(c, authentication=AUTH))
        # The labelled result is not wire data: its extra key is an unknown ContextInput field.
        with self.assertRaises(ValueError):
            self.build(invoice_records(), context=self.c.validate_context_input(CTX()))
        for value in (None, [], 'context', dict(CTX(), authentication=AUTH)):
            with self.assertRaises(ValueError):
                self.c.validate_context_input(value)


class TestStructuralNotFinancial(CodecCase):
    def test_wrong_financial_stories_remain_wire_valid(self):
        stories = []
        for mutate in (lambda t: t['effects'].insert(0, t['effects'].pop(1)),
                       lambda t: t['effects'].pop(),
                       lambda t: t['pre'].__setitem__('balances', t['pre']['balances'][:1]),
                       lambda t: t['post']['liabilities'].pop(),
                       lambda t: t['post']['balances'][0].__setitem__('amount', '73'),
                       lambda t: t['pre']['balances'].append(copy.deepcopy(t['pre']['balances'][0])),
                       lambda t: t['pre']['balances'].reverse(),
                       lambda t: t['effects'][2].__setitem__('after', '91'),
                       lambda t: t['pre']['balances'][0].__setitem__('account', ''),
                       lambda t: (t['pre'].__setitem__('state', {}), t.__setitem__('inputs', {}))):
            t = invoice_records()
            mutate(t)
            stories.append(('partial', t))
        stories += [('allocate', invoice_records()), ('partial', reserve_records()),
                    ('finish', original_request()['transition'])]
        for action, t in stories:
            with self.subTest(action=action):
                self.ok(t, action)

    def test_copy_independence_order_preservation_and_no_mutation(self):
        fx, ctx = invoice_records(), CTX()
        req = self.c.build_prove_request('partial', fx, ctx)
        frozen = copy.deepcopy(req)
        self.assertIsNot(req['transition']['pre']['balances'], fx['pre']['balances'])
        fx['pre']['balances'][0]['asset']['scale'] = 9
        fx['effects'].reverse()
        fx['inputs']['amount']['value'] = '36'
        ctx['seq'] = 2
        self.assertEqual(req, frozen)
        src = invoice_records()
        req = self.c.build_prove_request('partial', src, CTX())
        req['transition']['effects'].clear()
        req['transition']['pre']['state']['paid']['value'] = '9'
        req['context']['seq'] = 7
        self.assertEqual(src, invoice_records())
        for request in (original_request(), literal_request(reserve_records(), 'allocate')):
            before = copy.deepcopy(request)
            self.assertIs(self.c.validate_prove_request(request), True)
            self.assertEqual(json.dumps(request), json.dumps(before))  # key and array order untouched


def _hook(*args, **kwargs):
    raise AssertionError('caller-controlled method invoked before refusal')


class _Trap:
    '''Non-JSON object (R-01): ValueError must precede any method call on it.'''
    __deepcopy__ = __copy__ = __reduce__ = __reduce_ex__ = _hook
    __eq__ = __ne__ = __repr__ = __str__ = __format__ = __bool__ = __len__ = __iter__ = _hook


class _KeyTrap:
    '''Non-str dict key: hashed once on insertion, then armed.'''
    armed = False
    __eq__ = __lt__ = __repr__ = _hook

    def __hash__(self):
        if self.armed:
            raise AssertionError('non-str key hashed before refusal')
        return 7


class _Str(str):
    '''Lying str subclass: equal to everything, ASCII digits by its own account.'''
    __hash__ = str.__hash__

    def __eq__(self, other):
        return True

    def isascii(self):
        return True

    isdigit = isascii


class _Int(int):
    pass


class _List(list):
    pass


class _Dict(dict):
    pass


def _exact(node):
    if type(node) is dict:
        return all(type(k) is str and _exact(v) for k, v in node.items())
    if type(node) is list:
        return all(_exact(v) for v in node)
    return type(node) in (str, int, bool, type(None))


class TestR04CoreReferenceLeaves(CodecCase):
    def test_scenario_liabilities_values_keys_and_assets(self):
        sc = lambda t: t['scenario']
        lia = lambda t: sc(t)['liabilities'][0]
        asset0 = lambda t: sc(t)['balances'][0]['asset']
        muts = [lambda t, k=k: lia(t).pop(k) for k in ('name', 'asset', 'debtor', 'creditor', 'amount')]
        muts += [lambda t: lia(t).__setitem__('extra', '1'),
                 lambda t: asset0(t).__setitem__('scale', 256),
                 lambda t: asset0(t).__setitem__('name', 'CASH')]
        muts += [lambda t, v=v: lia(t).__setitem__('amount', v) for v in ('090', '-1', '', 90, str(U128_MAX + 1))]
        for m in ('state', 'inputs'):
            muts += [lambda t, m=m, v=v: sc(t)[m].__setitem__(next(iter(sc(t)[m])), v)
                     for v in ({'type': 'uint', 'value': '7'}, {'type': 'Bool', 'value': 1},
                               {'type': 'UInt', 'value': 7}, {'type': 'UInt'}, dict(U(1), x=0))]
            muts += [lambda t, m=m, k=k: sc(t)[m].__setitem__(k, U(1)) for k in (1, None, b'k', _Str('k'))]
        for i, mutate in enumerate(muts):
            t = original_request()['transition']
            mutate(t)
            with self.subTest(i=i):
                self.refuse(t)


class TestR04ExactTypes(CodecCase):
    def test_api_arguments_refuse_subclasses_and_foreign_objects(self):
        c, rec = self.c, invoice_records
        calls = (lambda: c.build_prove_request(_Str('partial'), rec(), CTX()),
                 lambda: c.build_prove_request('partial', _Dict(rec()), CTX()),
                 lambda: c.build_prove_request('partial', rec(), _Dict(CTX())),
                 lambda: c.build_prove_request('partial', rec(), CTX(), expected_source_sha256=_Str('cc' * 32)),
                 lambda: c.build_prove_request('partial', rec(), CTX(), expected_program_hash=_Str('bb' * 32)),
                 lambda: c.build_prove_request(_Trap(), rec(), CTX()),
                 lambda: c.build_prove_request('partial', _Trap(), CTX()),
                 lambda: c.validate_prove_request(_Dict(original_request())),
                 lambda: c.validate_prove_request(_Trap()),
                 lambda: c.validate_context_input(_Dict(CTX())),
                 lambda: c.validate_context_input(_Trap()))
        for i, call in enumerate(calls):
            with self.subTest(i=i):
                with self.assertRaises(ValueError):
                    call()

    def test_nested_subclasses_are_refused(self):
        muts = (lambda t: t['inputs'].__setitem__('amount', _Dict(U(35))),
                lambda t: t['inputs']['amount'].__setitem__('value', _Str('35')),
                lambda t: t['pre']['balances'][0].__setitem__('amount', _Str('109')),
                lambda t: t['pre']['balances'][0]['asset'].__setitem__('scale', _Int(0)),
                lambda t: t['pre'].__setitem__('balances', _List(t['pre']['balances'])),
                lambda t: t['pre'].__setitem__('state', _Dict(t['pre']['state'])),
                lambda t: t['pre']['state'].__setitem__(_Str('paid'), t['pre']['state'].pop('paid')),
                lambda t: t.__setitem__('effects', _List(t['effects'])),
                lambda t: t['effects'].__setitem__(1, _Dict(t['effects'][1])),
                lambda t: t['effects'][0].__setitem__('kind', _Str('Fee')),
                lambda t: t.__setitem__('kind', _Str('Records')))
        for i, mutate in enumerate(muts):
            t = invoice_records()
            mutate(t)
            with self.subTest(i=i):
                self.refuse(t)
        for key, value in (('seq', _Int(1)), ('networkId', _Int(0)), ('consumedIx', _Int(0)),
                           ('claimantVkh', _Str('5' * 56))):
            ctx = CTX()
            ctx[key] = value
            with self.subTest(key=key):
                self.refuse_context(ctx)


class TestR04UnknownValuesUntouched(CodecCase):
    def test_unknown_or_misplaced_objects_refuse_before_any_method(self):
        for i in range(10):
            r = literal_request(invoice_records())
            t = r['transition']
            node = (r, r['selector'], r['context'], t, t['pre'], t['post']['liabilities'][0], t['pre']['balances'][0],
                    t['pre']['balances'][0]['asset'], t['inputs']['amount'], t['effects'][2])[i]
            node['zzUnknown'] = _Trap()
            with self.subTest(i=i):
                self.bad_request(r)
                if i >= 2:
                    with self.assertRaises(ValueError):
                        self.c.build_prove_request('partial', t, r['context'])
                if i == 2:
                    with self.assertRaises(ValueError):
                        self.c.validate_context_input(r['context'])
        plants = (lambda r: r['transition'].__setitem__('zzUnknown', _Trap()),
                  lambda r: r['transition']['scenario'].__setitem__('zzUnknown', _Trap()),
                  lambda r: r['transition']['scenario']['liabilities'][1].__setitem__('zzUnknown', _Trap()),
                  lambda r: r['transition']['scenario']['balances'].__setitem__(0, _Trap()),
                  lambda r: r['transition']['scenario']['state'].__setitem__('paid', _Trap()),
                  lambda r: r['transition']['scenario']['inputs']['amount'].__setitem__('value', _Trap()),
                  lambda r: r['transition'].__setitem__('scenario', _Trap()),
                  lambda r: r['context'].__setitem__('seq', _Trap()))
        for i, plant in enumerate(plants):
            r = original_request()
            plant(r)
            with self.subTest(core=i):
                self.bad_request(r)
                with self.assertRaises(ValueError):
                    self.c.build_prove_request('partial', r['transition'], r['context'])

    def test_non_str_keys_refuse_before_hash_or_compare(self):
        for i in range(5):
            r = original_request()
            sc = r['transition']['scenario']
            key = _KeyTrap()
            (r, r['selector'], r['context'], sc['state'], sc['inputs'])[i][key] = U(1)
            key.armed = True
            with self.subTest(i=i):
                self.bad_request(r)


class TestR04BoundsAndCopy(CodecCase):
    def test_oversized_uint_and_action_raise_value_error(self):
        for place in UINT_PLACES:
            self.refuse(mutated(place, '9' * 100000))
        for action in ('a' * 1000000, chr(0xe9) * 600000, 'a' * 1023 + chr(0xdc00)):
            self.refuse(invoice_records(), action)

    def test_builder_copy_is_exact_detached_and_order_preserving(self):
        t = original_request()['transition']
        rows = t['scenario']['balances']
        rows[0] = dict(reversed(list(rows[0].items())))
        ctx = dict(reversed(list(CTX().items())), vkDigest=None)
        req = self.c.build_prove_request('partial', t, ctx)
        self.assertTrue(_exact(req))
        self.assertEqual((json.dumps(req['transition']), json.dumps(req['context'])), (json.dumps(t), json.dumps(ctx)))
        self.assertIsNot(req['transition']['scenario']['balances'][0], rows[0])
        self.assertIsNot(req['context'], ctx)


if __name__ == '__main__':
    unittest.main()
