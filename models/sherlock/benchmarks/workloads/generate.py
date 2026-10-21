"""Deterministic public benchmark data, never a proof/native-operation runner.

The Fenrir-derived profile is an explicitly neutral account/scalar projection.
Its transparent test identifiers are not wallet addresses. Its unknown tree roots
are not replaced with another hash. Use the separately declared owner adapter
for real Fenrir witnesses; these JSON records cannot be fed to stage_batch.
"""
from __future__ import annotations
import argparse
import copy
import hashlib
import json
from pathlib import Path

SEED = 'sherlock-public-fenrir-derived/1'
PREFIXES = (1, 2, 4, 6)
ADDRESSES = ('test:alice', 'test:bob', 'test:merchant', 'test:processor')
STATE_FIELDS = ('cm_root','cm_count','nf_root','nf_count','rh_root','rh_count',
                'wd_root','wd_count','height','tx_count','dep_consumed','fee_total')
KINDS = ('deposit','checkout_change','checkout_exact','p2p','sweep','withdrawal')
UNKNOWN_ROOT = {'value': None, 'status': 'unavailable', 'reason':
    'Cryptographic root requires independent field-hash/tree oracle and native witness adaptation; no SHA-256 substitute.'}


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode('utf-8')


def digest(value):
    return hashlib.sha256(canonical(value)).hexdigest()


def _nat(value, name):
    if type(value) is not int or value < 0:
        raise ValueError(f'{name} must be a nonnegative integer')
    return value


def _state():
    return {name: copy.deepcopy(UNKNOWN_ROOT) if name.endswith('_root') else
            (2 if name == 'cm_count' else 1 if name == 'nf_count' else 0)
            for name in STATE_FIELDS}


def _next_state(state, tx):
    result = copy.deepcopy(state)
    for field, increment in (('cm_count',tx['synthetic_note_outputs']),
                            ('nf_count',int(tx['kind'] != 'deposit')),
                            ('wd_count',int(tx['kind'] == 'withdrawal')),
                            ('tx_count',1), ('dep_consumed',int(tx['kind'] == 'deposit')),
                            ('fee_total',tx['fee'])):
        result[field] += increment
    result['height'] = tx['post_height']
    result['rh_count'] = tx['post_height']
    return result


def _plan(batch, slot, balances):
    if slot == 0: return 'deposit','test:synthetic-bridge','test:alice',100,0,1
    if slot == 1: return 'checkout_change','test:alice','test:merchant',10,1,2
    if slot == 2: return 'checkout_exact','test:bob','test:merchant',11,1,1
    if slot == 3: return 'p2p','test:alice','test:bob',12,1,2
    if slot == 4: return 'sweep','test:merchant','test:processor',balances['test:merchant']-1,1,1
    if slot == 5: return 'withdrawal','test:processor','test:synthetic-bridge',13,1,1
    sender, receiver = ('test:alice','test:bob') if slot % 2 == 0 else ('test:bob','test:alice')
    return 'p2p',sender,receiver,slot+1,1,2


def fenrir_derived(batches):
    """Return a prefix of one fixed six-batch public synthetic scalar history."""
    if type(batches) is not int or batches not in PREFIXES:
        raise ValueError('supported batch prefixes are exactly 1, 2, 4, 6')
    state = _state()
    balances = dict(zip(ADDRESSES, (10000,10000,0,0)))
    initial = copy.deepcopy(state)
    records, batch_records = [], []
    for batch in range(1,batches+1):
        before = copy.deepcopy(state)
        ids = []
        for slot in range(16):
            kind,sender,receiver,amount,fee,outputs = _plan(batch,slot,balances)
            tx = {'id':f'public-synthetic-{len(records):04d}', 'batch':batch,
                  'slot':slot, 'kind':kind, 'sender':sender, 'receiver':receiver,
                  'token':'synthetic:USD', 'amount':amount, 'fee':fee,
                  'synthetic_note_outputs':outputs, 'pre_height':state['height'],
                  'post_height':(batch-1)*50+(50 if slot == 15 else 3*(slot+1)),
                  'state_before':copy.deepcopy(state)}
            if kind != 'deposit': balances[sender] -= amount+fee
            if kind != 'withdrawal': balances[receiver] += amount
            state = _next_state(state, tx)
            tx['state_after'] = copy.deepcopy(state)
            records.append(tx); ids.append(tx['id'])
        batch_records.append({'batch_no':batch,'h_first':(batch-1)*50+1,'h_last':batch*50,
             'n_tx':16,'state_pre':before,'state_post':copy.deepcopy(state),'ordered_transaction_ids':ids,
             'ordered_records_sha256':digest(records[-16:]),
             'sha256_scope':'canonical public corpus bytes, not Fenrir Poseidon in_root or state digest'})
    return {'schema':'sherlock-fenrir-derived-public-scalar/1','seed':SEED,
            'compatibility':'neutral scalar projection; not Fenrir witness/stage_batch input',
            'tree_root_status':'unavailable','headers_per_batch':50,'transactions_per_batch':16,
            'genesis_balances':dict(zip(ADDRESSES,(10000,10000,0,0))),
            'state_before':initial,'state_after':state,'transactions':records,'batches':batch_records}


def validate_fenrir(corpus):
    """Independently replay all public economics, ordering and scalar endpoints.

    Does not claim signature, commitment, Merkle root, or cryptographic validity.
    """
    if corpus.get('schema') != 'sherlock-fenrir-derived-public-scalar/1':
        raise ValueError('not the public neutral scalar corpus')
    batches = corpus.get('batches', [])
    count = len(batches)
    if count not in PREFIXES or len(corpus.get('transactions',[])) != 16*count:
        raise ValueError('requires a supported full sixteen-transaction prefix')
    if corpus.get('seed') != SEED or corpus.get('headers_per_batch') != 50 or corpus.get('transactions_per_batch') != 16:
        raise ValueError('changed corpus identity or shape')
    balances = dict(zip(ADDRESSES,(10000,10000,0,0)))
    if corpus.get('genesis_balances') != balances or corpus.get('state_before') != _state():
        raise ValueError('changed genesis')
    state = copy.deepcopy(corpus['state_before'])
    deposit = withdrawal = fees = 0
    ordered = []
    for position, tx in enumerate(corpus['transactions']):
        batch, slot = position//16+1, position%16
        if tx.get('id') != f'public-synthetic-{position:04d}' or tx.get('batch') != batch or tx.get('slot') != slot:
            raise ValueError('transaction order or identity changed')
        if tx.get('kind') not in KINDS or tx.get('token') != 'synthetic:USD':
            raise ValueError('unsupported transaction kind or token')
        if tx.get('sender') not in ADDRESSES+('test:synthetic-bridge',) or tx.get('receiver') not in ADDRESSES+('test:synthetic-bridge',):
            raise ValueError('not a transparent synthetic identifier')
        amount, fee = _nat(tx.get('amount'),'amount'), _nat(tx.get('fee'),'fee')
        outputs = _nat(tx.get('synthetic_note_outputs'),'synthetic_note_outputs')
        if tx.get('state_before') != state or tx.get('pre_height') != state['height']:
            raise ValueError('discontinuous predecessor')
        end_height = (batch-1)*50+(50 if slot == 15 else 3*(slot+1))
        if tx.get('post_height') != end_height: raise ValueError('changed height schedule')
        kind, sender, receiver = tx['kind'],tx['sender'],tx['receiver']
        if kind == 'deposit':
            if sender != 'test:synthetic-bridge' or receiver not in balances or fee != 0:
                raise ValueError('invalid synthetic deposit')
            deposit += amount
        else:
            if sender not in balances or balances[sender] < amount+fee or fee != 1:
                raise ValueError('insufficient synthetic balance or incorrect fee')
            balances[sender] -= amount+fee
        if kind == 'withdrawal':
            if receiver != 'test:synthetic-bridge': raise ValueError('invalid synthetic withdrawal')
            withdrawal += amount
        else:
            if receiver not in balances: raise ValueError('invalid synthetic recipient')
            balances[receiver] += amount
        if kind == 'sweep' and balances[sender] != 0: raise ValueError('sweep left residual')
        # Direct scalar equations, rather than generation's state update helper.
        state['cm_count'] += outputs
        state['nf_count'] += 0 if kind == 'deposit' else 1
        state['wd_count'] += 1 if kind == 'withdrawal' else 0
        state['dep_consumed'] += 1 if kind == 'deposit' else 0
        state['tx_count'] += 1
        state['fee_total'] += fee
        state['height'] = state['rh_count'] = end_height
        if tx.get('state_after') != state: raise ValueError('incorrect full scalar endpoint')
        fees += fee; ordered.append(tx['id'])
        if slot == 15:
            b = batches[batch-1]
            first = corpus['transactions'][position-15]
            if b.get('batch_no') != batch or b.get('n_tx') != 16 or b.get('h_first') != (batch-1)*50+1 or b.get('h_last') != batch*50:
                raise ValueError('changed batch shape')
            if b.get('state_pre') != first['state_before'] or b.get('state_post') != state or b.get('ordered_transaction_ids') != ordered[-16:] or b.get('ordered_records_sha256') != digest(corpus['transactions'][position-15:position+1]):
                raise ValueError('changed batch continuity or ordered records')
    if corpus.get('state_after') != state or sum(balances.values()) != 20000+deposit-withdrawal-fees:
        raise ValueError('incorrect final state or supply conservation')
    return {'schema':'sherlock-fenrir-derived-scalar-effects/1','batches':count,'state':state,
       'balances':balances,'totals':{'deposited':deposit,'withdrawn':withdrawal,'fees':fees,
       'genesis_supply':20000,'final_supply':sum(balances.values())},'ordered_transaction_ids':ordered,
       'independent_scope':'all scalar effects and ordered transaction IDs; four cryptographic roots unavailable'}


def nominal_queue(size):
    """Scale only the closed entry of the actual public MapQueue program."""
    if type(size) is not int or not 0 <= size <= 32: raise ValueError('queue size must be 0..32')
    source = (Path(__file__).parent/'sources/map-queue.mori').read_text()
    start = source.index('  entry pack(')
    expr = 'construct Queue<UInt>::Empty()'
    items = [3+5*i for i in range(size)]
    queue_type = {'name':'Queue','args':['UInt']}
    value = {'type':queue_type,'ctor':'Empty','fields':[]}
    for item in reversed(items):
        expr = f'construct Queue<UInt>::Push({item}, {expr})'
        value = {'type':queue_type,'ctor':'Push','fields':[
            {'name':'z_item','type':'UInt','value':{'UInt':str(item+2)}},
            {'name':'a_tail','type':queue_type,'value':value}]}
    source = source[:start]+f'  entry pack({expr});\n}}\n'
    expected = {'type':{'name':'Envelope','args':[queue_type,'Bool']},'ctor':'Wrap','fields':[
        {'name':'z_items','type':queue_type,'value':value},
        {'name':'a_marker','type':'Bool','value':{'Bool':True}}]}
    return {'source':source,'input_items':items,'expected_value':expected}


def write_corpus(out):
    """Write bounded public data only; no execution or automatic repetitions."""
    out = Path(out); out.mkdir(parents=True, exist_ok=True)
    for batches in PREFIXES:
        value = fenrir_derived(batches)
        (out/f'fenrir-derived-{batches}.json').write_bytes(canonical(value)+b'\n')
    for size in (0,1,2,4,8,16):
        (out/f'map-queue-{size}.mori').write_text(nominal_queue(size)['source'])


def inspect_fenrir_history(path):
    """Read-only shape inspection for an owner-supplied synthetic v2 history.

    Returns metadata only. Does not export witnesses, party keys, signatures,
    addresses or payloads. Generator and oracle execution remains with owner.
    """
    path = Path(path)
    def read(name):
        try: return json.loads((path/name).read_text())
        except (OSError,ValueError) as exc: raise ValueError('missing or invalid owner-supplied history input') from exc
    params = read('params.json')
    if params.get('headers') != 300 or params.get('batch_headers') != 50 or params.get('tx_per_batch') != 16:
        raise ValueError('requires six full sixteen-transaction batches and fifty headers per batch')
    batches = read('batches.json')
    if not isinstance(batches,list) or len(batches) != 6: raise ValueError('requires six batches')
    previous = None
    tx_total = 0
    for i,batch in enumerate(batches,1):
        if batch.get('batch_no') != i or batch.get('n_tx') != 16 or batch.get('h_first') != (i-1)*50+1 or batch.get('h_last') != i*50:
            raise ValueError('unsupported batch shape')
        if set(batch.get('state_pre',{})) != set(STATE_FIELDS) or set(batch.get('state_post',{})) != set(STATE_FIELDS):
            raise ValueError('requires complete twelve-field state')
        if previous is not None and batch['state_pre'] != previous: raise ValueError('discontinuous batch state')
        previous = batch['state_post']
    for height in range(1,301):
        header = read(f'blocks/{height:06}.json')
        if header.get('height') != height or 'rh_append_path' not in header:
            raise ValueError('requires v2 per-header root-history witnesses; old v1 is incompatible')
        tx_total += len(header.get('txs',[]))
    if tx_total != 96: raise ValueError('requires exactly ninety-six source transactions')
    return {'schema':'sherlock-fenrir-source-shape/1','headers':300,'batches':6,'transactions':96,
        'supported_prefixes':[1,2,4,6], 'payload_exported':False,'validation_scope':'shape only; source oracle is correlated; no cryptographic or native qualification'}


def validate_catalog(root):
    root = Path(root)
    catalog = json.loads((root/'catalog.json').read_text())
    if catalog.get('schema') != 'sherlock-benchmark-workloads/1': raise ValueError('unsupported catalog schema')
    ids = set()
    generator_hash = hashlib.sha256((root/'generate.py').read_bytes()).hexdigest()
    for case in catalog['cases']:
        if case['source_identity']['generator_sha256'] != generator_hash:
            raise ValueError('workload generator identity changed')
        if case['workload_id'] in ids: raise ValueError('duplicate workload id')
        ids.add(case['workload_id'])
        for item in case['inputs']:
            rel = Path(item['path'])
            if rel.is_absolute() or '..' in rel.parts: raise ValueError('unsafe workload path')
            if hashlib.sha256((root/rel).read_bytes()).hexdigest() != item['sha256']:
                raise ValueError('workload input changed')
        if digest(case['inputs']) != case['workload_digest_sha256']: raise ValueError('workload digest changed')
        expected_path = Path(case['expected_effects_path'])
        if expected_path.is_absolute() or '..' in expected_path.parts: raise ValueError('unsafe expected path')
        raw = (root/expected_path).read_bytes()
        if hashlib.sha256(raw).hexdigest() != case['expected_effects_sha256'] or digest(json.loads(raw)) != case['expected_effects_canonical_sha256']:
            raise ValueError('expected effects changed')
    return {'case_count':len(ids),'native_or_stagenet_executed':False,'scope':'public data identity only'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command',required=True)
    sub.add_parser('validate')
    build = sub.add_parser('generate'); build.add_argument('--out',type=Path,required=True)
    inspect = sub.add_parser('inspect-owner-history'); inspect.add_argument('--history',type=Path,required=True)
    args = parser.parse_args()
    if args.command == 'generate': write_corpus(args.out)
    elif args.command == 'inspect-owner-history': print(json.dumps(inspect_fenrir_history(args.history),sort_keys=True))
    else: print(json.dumps(validate_catalog(Path(__file__).parent),sort_keys=True))

if __name__ == '__main__': main()
