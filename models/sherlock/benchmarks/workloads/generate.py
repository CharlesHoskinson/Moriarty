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
import math
import re
from pathlib import Path

SEED = 'sherlock-public-fenrir-derived/1'
PREFIXES = (1, 2, 4, 6)
ADDRESSES = ('test:alice', 'test:bob', 'test:merchant', 'test:processor')
STATE_FIELDS = ('cm_root','cm_count','nf_root','nf_count','rh_root','rh_count',
                'wd_root','wd_count','height','tx_count','dep_consumed','fee_total')
KINDS = ('deposit','checkout_change','checkout_exact','p2p','sweep','withdrawal')
UNKNOWN_ROOT = {'value': None, 'status': 'unavailable', 'reason':
    'Cryptographic root requires independent field-hash/tree oracle and native witness adaptation; no SHA-256 substitute.'}


def strict_json(text):
    """Reject duplicate object keys and non-finite JSON numbers."""
    def pairs(items):
        result = {}
        for key,value in items:
            if key in result: raise ValueError('duplicate JSON key')
            result[key] = value
        return result
    def finite(text):
        value = float(text)
        if not math.isfinite(value): raise ValueError('non-finite JSON number')
        return value
    def constant(_): raise ValueError('non-finite JSON constant')
    return json.loads(text,object_pairs_hook=pairs,parse_float=finite,parse_constant=constant)


def _object(value, keys, label):
    if type(value) is not dict or set(value) != set(keys):
        raise ValueError(f'{label} requires exact object fields')
    return value


def _state_shape(value, roots_unavailable):
    _object(value,STATE_FIELDS,'state')
    for name,item in value.items():
        if name.endswith('_root'):
            if roots_unavailable:
                if item != UNKNOWN_ROOT: raise ValueError('cryptographic root must remain unavailable')
            elif not isinstance(item,str) or not re.fullmatch('[0-9a-f]{64}',item):
                raise ValueError('root requires canonical field encoding')
        else: _nat(item,name)


def _public_structure(corpus):
    _object(corpus,('schema','seed','compatibility','tree_root_status','headers_per_batch',
        'transactions_per_batch','genesis_balances','state_before','state_after','transactions','batches'),'corpus')
    if corpus['compatibility'] != 'neutral scalar projection; not Fenrir witness/stage_batch input' or corpus['tree_root_status'] != 'unavailable':
        raise ValueError('changed compatibility boundary')
    for field in ('headers_per_batch','transactions_per_batch'): _nat(corpus[field],field)
    if type(corpus['transactions']) is not list or type(corpus['batches']) is not list:
        raise ValueError('transactions and batches require arrays')
    _object(corpus['genesis_balances'],ADDRESSES,'genesis_balances')
    for value in corpus['genesis_balances'].values(): _nat(value,'balance')
    _state_shape(corpus['state_before'],True); _state_shape(corpus['state_after'],True)
    for tx in corpus['transactions']:
        _object(tx,('id','batch','slot','kind','sender','receiver','token','amount','fee',
            'synthetic_note_outputs','pre_height','post_height','state_before','state_after'),'transaction')
        for field in ('batch','slot','amount','fee','synthetic_note_outputs','pre_height','post_height'): _nat(tx[field],field)
        _state_shape(tx['state_before'],True); _state_shape(tx['state_after'],True)
    for batch in corpus['batches']:
        _object(batch,('batch_no','h_first','h_last','n_tx','state_pre','state_post',
            'ordered_transaction_ids','ordered_records_sha256','sha256_scope'),'batch')
        for field in ('batch_no','h_first','h_last','n_tx'): _nat(batch[field],field)
        if type(batch['ordered_transaction_ids']) is not list: raise ValueError('batch IDs require array')
        if batch['sha256_scope'] != 'canonical public corpus bytes, not Fenrir Poseidon in_root or state digest':
            raise ValueError('changed artifact hash scope')
        _state_shape(batch['state_pre'],True); _state_shape(batch['state_post'],True)


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False, allow_nan=False).encode('utf-8')


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
    _public_structure(corpus)
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
    """Validate owner-supplied v2 shape without exporting private payload.

    Structural consistency only; signatures, roots and native validity unobserved.
    Extra source fields are permitted because the source format is external.
    """
    path = Path(path)
    def read(name):
        try: return strict_json((path/name).read_text())
        except (OSError,ValueError) as exc: raise ValueError('missing or invalid owner-supplied history input') from exc
    params = read('params.json')
    if type(params) is not dict: raise ValueError('params must be object')
    for key,expected in (('headers',300),('batch_headers',50),('tx_per_batch',16)):
        if type(params.get(key)) is not int or params[key] != expected: raise ValueError('unsupported owner-history shape')
    genesis = read('genesis.json')
    if type(genesis) is not dict or 'state' not in genesis: raise ValueError('genesis state missing')
    _state_shape(genesis['state'],False)
    batches = read('batches.json')
    if type(batches) is not list or len(batches) != 6: raise ValueError('requires six batches')
    previous = genesis['state']
    for i,batch in enumerate(batches,1):
        if type(batch) is not dict: raise ValueError('batch must be object')
        for name,expected in (('batch_no',i),('n_tx',16),('h_first',(i-1)*50+1),('h_last',i*50)):
            if type(batch.get(name)) is not int or batch[name] != expected: raise ValueError('unsupported batch shape')
        _state_shape(batch.get('state_pre'),False); _state_shape(batch.get('state_post'),False)
        if batch['state_pre'] != previous: raise ValueError('discontinuous batch/genesis state')
        if batch['state_pre']['height'] != (i-1)*50 or batch['state_post']['height'] != i*50:
            raise ValueError('batch state height mismatch')
        if batch['state_post']['tx_count']-batch['state_pre']['tx_count'] != 16:
            raise ValueError('batch transaction counter mismatch')
        previous = batch['state_post']
    counts = [0]*6
    for height in range(1,301):
        header = read(f'blocks/{height:06}.json')
        if type(header) is not dict or type(header.get('height')) is not int or header['height'] != height:
            raise ValueError('invalid header height')
        append = header.get('rh_append_path')
        if type(append) is not list or len(append) != 16 or any(not isinstance(x,str) or not re.fullmatch('[0-9a-f]{64}',x) for x in append):
            raise ValueError('requires v2 per-header root-history append path')
        transactions = header.get('txs')
        if type(transactions) is not list or any(type(tx) is not dict for tx in transactions):
            raise ValueError('header transactions must be object array')
        counts[(height-1)//50] += len(transactions)
    if counts != [16]*6: raise ValueError('requires exactly sixteen transactions in each fifty-header batch')
    return {'schema':'sherlock-fenrir-source-shape/1','headers':300,'batches':6,'transactions':96,
        'supported_prefixes':[1,2,4,6], 'payload_exported':False,'validation_scope':'shape only; source oracle is correlated; no cryptographic or native qualification'}


ORACLE_CANONICAL_SHA256 = {'expected/fenrir-derived-1.json': '7f68fa8b947cacc0c727db73fa4a4870ef81afabf0287f2d53a1c1798a224d2d', 'expected/fenrir-derived-2.json': '8143ad0594701711a3a5fc07211e7c7147d5715eb9a88807a0ad6ce86ea4794f', 'expected/fenrir-derived-4.json': 'eff8501a73918a8897e80770dcb7a9eb7ebb84e5ccd01e7e91a8f3222cb412e0', 'expected/fenrir-derived-6.json': '30d155950c04078b8aada93baa73360bfdda1f870f243d398de3d3c12d457a69', 'expected/fenrir-derived.json': '2bdcee531314e3c2874db0a3f4d5f37f4c8e264adebe0dc1c297c90bf3dee2b7', 'expected/linked-two-step.json': 'b16154804910c272e7221a7b6d6b540061118a942d5244c3d290619d46cc634d', 'expected/map-queue.json': '786d477ca6ea5141657dfda41b5a65099bebfba3169473e7d6b759d539ce461c', 'expected/mirror-arbor.json': '0188491a8a788c541985ed083490b2cdeb65a0898058715ba84ea7e118b7e9b0'}

HOST_IDS = ('moriarty-map-queue','moriarty-mirror-arbor')
LINKED_ID = 'sherlock-linked-two-step'
FENRIR_IDS = tuple(f'fenrir-transfer-batches-{n}' for n in PREFIXES)
HOST_METRICS = tuple(f'{phase}.{resource}' for phase in
    ('compile.frontend','compile.lowering','host.handler')
    for resource in ('wall_seconds','cpu_seconds','peak_rss_bytes'))+('artifact.bytes',)
NATIVE_TOP_METRICS = tuple(f'compile.{phase}.{resource}' for phase in ('frontend','lowering','native')
    for resource in ('wall_seconds','cpu_seconds','peak_rss_bytes'))+(
    'transaction.deployment.bytes','transaction.call.bytes','fee.dust',
    'network.proof_ready_to_submission_seconds','network.submission_to_ack_seconds',
    'network.ack_to_inclusion_seconds','network.inclusion_to_readback_seconds',
    'network.submission_to_readback_seconds')
LAYER_METRICS = tuple(f'{phase}.{resource}' for phase in ('witness','keygen','prove','local_verify')
    for resource in ('wall_seconds','cpu_seconds','peak_rss_bytes'))+(
    'circuit.k','circuit.domain_capacity_rows','proof.bytes','proving_key.bytes',
    'verifying_key.bytes','instance.bytes','artifact.bytes')
STAGE_CASES = {'host-compiler-coverage':HOST_IDS, 'initial-deployed-baseline':(LINKED_ID,),
    'full-rollup-series':(LINKED_ID,)+FENRIR_IDS, 'full-suite':HOST_IDS+(LINKED_ID,)+FENRIR_IDS}


def case_minimum(workload_id):
    """Known immutable minimum; caller policy may only strengthen this contract."""
    if workload_id in HOST_IDS:
        kind, evidence, layers = 'moriarty-nominal-source','host',()
        metrics = HOST_METRICS
    elif workload_id == LINKED_ID:
        kind,evidence,layers = 'sherlock-recursive-interpreter','stagenet',('child-0','child-1','parent','decider')
        metrics = NATIVE_TOP_METRICS+tuple(f'layer:{layer}:{metric}' for layer in layers for metric in LAYER_METRICS)
    elif workload_id in FENRIR_IDS:
        kind,evidence,layers = 'fenrir-derived-public-scalar','stagenet',('leaf','parent','decider')
        metrics = NATIVE_TOP_METRICS+tuple(f'layer:{layer}:{metric}' for layer in layers for metric in LAYER_METRICS)
    else: raise ValueError('unknown required workload identity')
    roots = [] if evidence == 'host' else ['pre_state_root','post_state_root','ledger_state_root']
    if workload_id in FENRIR_IDS:
        roots += [f'{phase}_{root}' for phase in ('pre','post') for root in ('cm_root','nf_root','rh_root','wd_root')]
    return {'workload_id':workload_id,'kind':kind,'evidence_class':evidence,
        'required_effect_root_roles':roots,
        'required_layers':list(layers),'required_metrics':list(metrics),'min_successful_samples':1}


def stage_contract(catalog, stage):
    """Return copied catalog cases after enforcing known stage/case/metric minima.

    Metric IDs are fully qualified exactly once. This is a selection contract;
    validate_catalog additionally verifies on-disk source/oracle closure.
    """
    if stage not in STAGE_CASES: raise ValueError('unknown benchmark stage')
    if type(catalog) is not dict or catalog.get('schema') != 'sherlock-benchmark-workloads/2':
        raise ValueError('unsupported catalog schema')
    declarations = catalog.get('stages')
    if type(declarations) is not dict or set(declarations) != set(STAGE_CASES): raise ValueError('all known stages required')
    for name,ids in STAGE_CASES.items():
        declaration = declarations[name]
        if type(declaration) is not dict or declaration.get('required_case_ids') != list(ids) or declaration.get('min_successful_samples') != 1:
            raise ValueError('stage minimum case/repetition coverage changed')
    if type(catalog.get('cases')) is not list: raise ValueError('cases require array')
    cases = {}
    for case in catalog['cases']:
        if type(case) is not dict or case.get('workload_id') in cases: raise ValueError('invalid or duplicate case')
        minimum = case_minimum(case.get('workload_id'))
        for field in ('kind','evidence_class','required_layers','required_effect_root_roles'):
            if case.get(field) != minimum[field]: raise ValueError('case kind/evidence/layer identity differs')
        metrics = case.get('required_metrics')
        if type(metrics) is not list or len(set(metrics)) != len(metrics) or any(not isinstance(m,str) or not re.fullmatch(r'(?:layer:[A-Za-z0-9_-]+:)?[a-z_]+(?:\.[a-z_]+)+',m) for m in metrics):
            raise ValueError('metric IDs must be unique and qualified exactly once')
        if not set(minimum['required_metrics']) <= set(metrics): raise ValueError('required minimum metric coverage omitted')
        if 'required_layer_metrics' in case: raise ValueError('catalog uses only fully qualified required_metrics')
        cases[case['workload_id']] = case
    if set(cases) != set(STAGE_CASES['full-suite']): raise ValueError('catalog must retain all seven known cases')
    result = [copy.deepcopy(cases[name]) for name in STAGE_CASES[stage]]
    return {'stage':stage,'cases':result,'min_successful_samples':1,
        'required_stage_ids':list(STAGE_CASES)[:3] if stage == 'full-suite' else [stage],
        'full_goal_complete':False,'scope':'stage evidence consistency only; full Sherlock goal remains conjunctive'}


def validate_catalog(root, upstream_root=None):
    """Validate stage minima, every input, copied-source provenance and oracle.

    Returns a summary plus a validated deep copy. Upstream historical context
    is additionally checked when an explicit complete source root is supplied.
    """
    root = Path(root)
    catalog = strict_json((root/'catalog.json').read_text())
    stage_contract(catalog,'full-suite')
    def file(rel):
        rel = Path(rel)
        if rel.is_absolute() or '..' in rel.parts: raise ValueError('unsafe workload path')
        return root/rel
    def check_pin(pin):
        _object(pin,('path','sha256','bytes'),'file pin')
        raw = file(pin['path']).read_bytes()
        if type(pin['bytes']) is not int or len(raw) != pin['bytes'] or hashlib.sha256(raw).hexdigest() != pin['sha256']:
            raise ValueError('source/provenance pin changed')
        return raw
    provenance = strict_json(check_pin(catalog['provenance_pin']))
    if provenance.get('schema') != 'sherlock-benchmark-workload-provenance/2': raise ValueError('provenance schema differs')
    closure = provenance.get('local_closure')
    if type(closure) is not list: raise ValueError('local provenance closure required')
    names = set()
    for pin in closure:
        if pin['path'] in names: raise ValueError('duplicate provenance file')
        names.add(pin['path']); check_pin(pin)
    expected_closure = {'generate.py','expected/fenrir-derived.json'}
    generator_hash = hashlib.sha256((root/'generate.py').read_bytes()).hexdigest()
    for case in catalog['cases']:
        if case['source_identity']['generator_sha256'] != generator_hash: raise ValueError('workload generator identity changed')
        for item in case['inputs']: check_pin(item); expected_closure.add(item['path'])
        if digest(case['inputs']) != case['workload_digest_sha256']: raise ValueError('workload digest changed')
        rel = case['expected_effects_path']; expected_closure.add(rel)
        raw = file(rel).read_bytes(); expected = strict_json(raw)
        if hashlib.sha256(raw).hexdigest() != case['expected_effects_sha256'] or digest(expected) != case['expected_effects_canonical_sha256']:
            raise ValueError('expected effects changed')
        if digest(expected) != ORACLE_CANONICAL_SHA256.get(rel): raise ValueError('independent frozen oracle differs')
        if case['kind'] == 'fenrir-derived-public-scalar':
            corpus = strict_json(file(case['inputs'][0]['path']).read_text())
            if validate_fenrir(corpus) != expected: raise ValueError('corpus effects differ from independent scalar oracle')
            if case['expected_effects_independence'] != 'unavailable' or case['scalar_effects_independence'] != 'independent':
                raise ValueError('unavailable cryptographic oracle cannot be relabeled')
        elif case['workload_id'] == LINKED_ID and case['expected_effects_independence'] != 'unavailable':
            raise ValueError('unavailable native oracle cannot be relabeled')
    expected_closure.update(f'corpus/map-queue-{n}.mori' for n in (0,1,2,4,8,16))
    if names != expected_closure: raise ValueError('provenance closure incomplete or unexpected')
    for n in (0,1,2,4,8,16):
        if file(f'corpus/map-queue-{n}.mori').read_text() != nominal_queue(n)['source']: raise ValueError('scaled nominal source differs')
    copies = provenance.get('local_source_copies')
    if type(copies) is not list or len(copies) != 7: raise ValueError('copied source provenance incomplete')
    if {item['path'] for item in copies} != {f'sources/{name}' for name in ('map-queue.mori','mirror-arbor.mori','config.json','target.json','literal-seven.mori','literal-config.json','literal-target.json')}:
        raise ValueError('copied source provenance identities differ')
    for item in copies:
        check_pin({k:item[k] for k in ('path','sha256','bytes')})
        if not isinstance(item.get('upstream_path'),str) or not item['upstream_path']: raise ValueError('upstream source locator missing')
    upstream_checked = False
    if upstream_root is not None:
        for pin in provenance['public_sources']:
            path = Path(pin['path'])
            if path.is_absolute() or '..' in path.parts: raise ValueError('unsafe upstream source path')
            raw = (Path(upstream_root)/path).read_bytes()
            if len(raw) != pin['bytes'] or hashlib.sha256(raw).hexdigest() != pin['sha256']: raise ValueError('upstream source changed')
        for pin in copies:
            upstream = Path(pin['upstream_path'])
            if upstream.is_absolute() or '..' in upstream.parts: raise ValueError('unsafe upstream source copy path')
            raw = (Path(upstream_root)/upstream).read_bytes()
            if len(raw) != pin['bytes'] or hashlib.sha256(raw).hexdigest() != pin['sha256']: raise ValueError('upstream copied source changed')
        upstream_checked = True
    return {'case_count':len(catalog['cases']),'native_or_stagenet_executed':False,
        'scope':'public data identity, stage minima, copied-source closure and independent scalar oracle only',
        'upstream_context_checked':upstream_checked,'catalog':copy.deepcopy(catalog),
        'catalog_canonical_sha256':digest(catalog)}


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
