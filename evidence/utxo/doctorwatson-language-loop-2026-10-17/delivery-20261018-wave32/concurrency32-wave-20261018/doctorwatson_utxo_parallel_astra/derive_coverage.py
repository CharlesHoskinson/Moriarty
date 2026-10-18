"""Offline pinned-source/JSON inventory extraction; never runs the subject."""
import hashlib
import json
import re
import subprocess
from pathlib import Path

BASE = '64c6dedff1d99b33c23832330d7911a5fad9ed59'
REPO = '/home/charl/Moriarty'
OUT = Path(__file__).parent
CACHE, PINS = {}, {}

def read(path):
    if path not in CACHE:
        raw = subprocess.check_output(['git', '-C', REPO, 'show', BASE + ':' + path])
        CACHE[path] = raw.decode()
        PINS[path] = {'path': path, 'commit': BASE, 'gitBlob': subprocess.check_output(
            ['git', '-C', REPO, 'rev-parse', BASE + ':' + path], text=True).strip(),
            'sha256': hashlib.sha256(raw).hexdigest(), 'bytes': len(raw)}
    return CACHE[path]

def cite(path, needle):
    lines = read(path).splitlines()
    found = [(i + 1, line) for i, line in enumerate(lines) if needle in line]
    if not found:
        raise ValueError((path, needle))
    n, line = found[0]
    return {**PINS[path], 'line': n, 'literal': line}

def write(name, value):
    (OUT / name).write_text(json.dumps(value, indent=2) + '\n')

IR = 'crates/mori-core/src/ir.rs'
PARSER = 'crates/mori-core/src/parser.rs'
constructors = {}
for kind in ['Expr', 'BinaryOp', 'Statement']:
    body = re.search(r'pub enum ' + kind + r' \{(.*?)\n\}', read(IR), re.S).group(1)
    constructors[kind] = re.findall(r'^    (\w+)(?: \{|,)', body, re.M)
assert [len(constructors[k]) for k in constructors] == [6, 11, 10]

entries = [
    ('P1', 'program-ir-contract', 'counter', 'counter', 'crates/mori-core/tests/program_ir_contract.rs'),
    ('P2', 'program-ir-contract', 'empty-frame', 'empty-frame', 'crates/mori-core/tests/program_ir_contract.rs'),
    ('P3', 'financial-report-contract', 'installment-invoice', 'installment-invoice', 'crates/mori/tests/core_financial_report_contract.rs'),
    ('P4', 'financial-report-contract', 'conditional-reserve', 'conditional-reserve', 'crates/mori/tests/core_financial_report_contract.rs'),
    ('P5', 'core2-escrow-complete', 'escrow', 'escrow', 'crates/mori/tests/core2_escrow_complete.rs'),
    ('P6', 'core2-multi-condition-complete', 'input-custody/multi-condition', 'multi-condition', 'crates/mori/tests/core2_multi_condition_complete.rs'),
    ('P7', 'core2-partial-delivery-complete', 'partial-delivery', 'partial-delivery', 'crates/mori/tests/core2_partial_delivery_complete.rs'),
]
fixtures, presence = [], {1: {}, 2: {}}
for fid, directory, srcstem, expstem, test in entries:
    src = 'crates/fixtures/' + directory + '/' + srcstem + '.mori'
    expectation = 'crates/fixtures/' + directory + '/' + expstem + '.program.expected.json'
    source = read(src)
    program = json.loads(read(expectation))
    profile = int(program['language'].rsplit('/', 1)[1])
    positive_functions = re.findall(r'#\[test\]\s*fn (\w+)', read(test))
    matching = [f for f in positive_functions if 'program' in f or 'load' in f]
    if fid == 'P2': matching = [f for f in matching if 'empty' in f]
    elif fid == 'P1': matching = [f for f in matching if 'counter' in f]
    assert matching, test
    features = {}
    for i, expr in enumerate(program['expressions']):
        features.setdefault('Expr.' + expr['kind'], []).append(f'/expressions/{i}')
        if expr['kind'] == 'Binary':
            features.setdefault('BinaryOp.' + expr['op'], []).append(f'/expressions/{i}/op')
    for i, action in enumerate(program['actions']):
        for j, statement in enumerate(action['statements']):
            features.setdefault('Statement.' + statement['kind'], []).append(f'/actions/{i}/statements/{j}')
    for feature, pointers in features.items():
        presence[profile].setdefault(feature, []).append({'fixtureId': fid, 'pointers': pointers})
    fixtures.append({'id': fid, 'source': PINS[src], 'programExpectation': PINS[expectation],
        'sourceHeader': source.splitlines()[0], 'language': program['language'], 'semantics': program['semantics'],
        'positiveTest': cite(test, matching[0]), 'declaredFeatures': features,
        'scope': 'Complete Program oracle presence. Uninvoked declarations do not establish execution coverage.',
        'runtimeObserved': False})

witnesses = []
def witness(wid, profile, path, function, features, source=None):
    row = {'id': wid, 'profile': profile, 'test': cite(path, function), 'features': features,
           'evidenceClass': 'selected-positive-test-source-not-new-execution', 'runtimeObserved': False}
    if source:
        read(source)
        row['source'] = PINS[source]
    witnesses.append(row)

witness('W1', 1, 'crates/mori-core/tests/language.rs',
    'authored_counter_selects_actions_and_preserves_dependencies',
    ['Expr.Literal','Expr.Name','Expr.Binary','BinaryOp.Add','BinaryOp.Mul','BinaryOp.Le','BinaryOp.Eq',
     'Statement.Let','Statement.Requires','Statement.Next','Statement.Ensures','Statement.Emit'])
witness('W2', 1, 'crates/mori-core/tests/language.rs',
    'authored_payment_has_complete_ordered_effects_and_persistent_liability',
    ['Expr.Literal','Expr.Name','Expr.Binary','BinaryOp.And','BinaryOp.Gt','BinaryOp.Ge','BinaryOp.Add',
     'Statement.Requires','Statement.Let','Statement.Transfer','Statement.Fee','Statement.Liability',
     'Statement.Next','Statement.Ensures','Statement.Emit','Statement.Discharge'])
witness('W3', 1, 'crates/mori-core/tests/liability_reads.rs', 'missing_and_present_zero_reads_are_distinct',
    ['Expr.Not','Expr.LiabilityPresent','Expr.LiabilityAmount','Expr.Literal','Expr.Binary','BinaryOp.Eq',
     'Statement.Requires','Statement.Ensures','Statement.Discharge','Statement.Liability'])
witness('W4', 2, 'crates/mori-core/tests/continuations.rs',
    'reserve_two_awaits_preserve_captured_held_and_residual_five',
    ['Expr.Literal','Expr.Name','Expr.Binary','BinaryOp.Add','BinaryOp.Sub','BinaryOp.Ge','BinaryOp.Le',
     'Statement.Let','Statement.Next','Statement.Liability','Statement.Await','Statement.Transfer',
     'Statement.Discharge','Statement.Requires','Statement.Fee','Statement.Emit'],
    'examples/core/continuations/reserve.mori')
witness('W5', 2, 'crates/mori-core/tests/stage_reports.rs',
    'two_deliveries_full_reports_and_bytes_follow_actual_continuations',
    ['Expr.Literal','Expr.Name','Statement.Transfer','Statement.Next','Statement.Await','Statement.Emit'],
    'crates/fixtures/core-stage-reports/two-deliveries.mori')
witness('W6', 2, 'crates/mori-core/tests/stage_reports.rs',
    'quoted_fee_full_reports_and_bytes_preserve_actual_captured_amount',
    ['Expr.Literal','Expr.Name','Expr.Binary','BinaryOp.Add','Statement.Let','Statement.Await',
     'Statement.Fee','Statement.Next','Statement.Emit'], 'crates/fixtures/core-stage-reports/quoted-fee.mori')
witness('W7', 2, 'crates/mori-core/tests/stage_static_cost.rs',
    'h26_every_admitted_segment_meters_exactly_its_static_cost',
    ['Expr.Literal','Expr.Name','Expr.Binary','BinaryOp.Eq','BinaryOp.Add','Statement.Let','Statement.Next','Statement.Await'])
witness('W8', 2, 'crates/mori-core/tests/stage_static_cost.rs',
    'h26_zero_await_action_completes_with_its_single_static_segment',
    ['Expr.Literal','Expr.Name','Expr.Binary','BinaryOp.Mul','BinaryOp.Add','Statement.Let','Statement.Next'])

spellings = dict(zip(['Add','Sub','Mul','Eq','Ne','Lt','Le','Gt','Ge','And','Or'],
                    ['+','-','*','==','not =','<','<=','>','>=','and','or']))
rows = []
for kind, variants in constructors.items():
    for variant in variants:
        feature = kind + '.' + variant
        row = {'constructor': feature, 'declaration': cite(IR, '    ' + variant + (' {' if kind != 'BinaryOp' else ',')), 'profiles': {}}
        if kind == 'BinaryOp':
            row['sourceSpelling'] = spellings[variant]
            row['parser'] = cite(PARSER, '(BinaryOp::' + variant + ',')
        else:
            row['parser'] = cite(PARSER, kind + '::' + variant)
        for profile in [1, 2]:
            admitted = not (feature == 'Statement.Await' and profile == 1)
            oracle = presence[profile].get(feature, [])
            normal = [w['id'] for w in witnesses if w['profile'] == profile and feature in w['features']]
            row['profiles']['core' + str(profile)] = {
                'sourceAdmitted': admitted,
                'qualification': 'subject to typed finite source validation' if admitted else 'Core/2-only Await',
                'programOraclePresence': oracle, 'normalTestSourceWitnesses': normal,
                'coverage': 'not-applicable' if not admitted else ('positive-source-witness-present' if oracle or normal else 'no-positive-witness-in-inspected-corpus'),
                'executedCoverage': 'not-measured'}
        rows.append(row)

profile_sources = [cite('crates/mori-core/src/stage/mod.rs', 'pub fn load('),
    cite('crates/mori-core/src/stage/profile.rs', 'pub fn require_core1('),
    cite('crates/mori-core/src/stage/validate.rs', 'pub(crate) fn validate_v2(')]
dispatch = []
for did, path, needle, finding in [
    ('check', 'crates/mori/src/commands/check_source.rs', 'fn dispatch<', 'check delegates to check_source::dispatch; it selects stage::load for moriarty/2, parse for Core/1, and separate beta frontend for the other header'),
    ('compile-core2', 'crates/mori/src/commands/compile.rs', 'mori_backend::stage::compile_staged', 'native-feature CLI explicitly routes Core/2 to staged artifact/4 compiler source'),
    ('compile-core1', 'crates/mori/src/commands/compile.rs', 'source_projection::compile_reported', 'native-feature Core/1 uses reported source-projection compiler and artifact/3'),
    ('inspect-core2', 'crates/mori/src/commands/inspect.rs', 'if args.target.is_none()', 'Core/2 source-only inspect differs from --target refusal; do not infer blanket Core/2 compiler absence'),
    ('expand', 'crates/mori/src/commands/expand.rs', 'core_io::program', 'Default expansion uses Core/1 program and reference execution; explicit Source/6 flag is a separate beta transfer proposal'),
    ('stage-types', 'crates/mori-backend/src/stage/mod.rs', '`/4` is a distinct type', 'Source-declared /4 type has no /3 artifact conversion; legacy proof/signed/native consumption remains separate'),
]:
    dispatch.append({'id': did, 'source': cite(path, needle), 'finding': finding, 'runtimeObserved': False})
read('crates/mori-core/Cargo.toml')
read('crates/mori/Cargo.toml')
read('crates/mori-core/README.md')
read('crates/mori/src/commands/simulate.rs')
coverage = {'format':'doctorwatson-utxo.ordinary-grammar-coverage/1', 'commit':BASE,
    'status':'SOURCE_ONLY_FINITE_COVERAGE_MAP', 'scope':'Seven complete Program oracle pairs plus eight selected normal test-source witnesses',
    'notWholeRepositoryTestExhaustiveness':True, 'runtimeObserved':False, 'acceptance':'none',
    'variantCounts':{**{k:len(v) for k,v in constructors.items()},'core1Statements':9,'core2Statements':10},
    'profileAdmissionSources':profile_sources, 'rows':rows, 'programOracleFixtures':fixtures,
    'supplementaryNormalWitnesses':witnesses, 'publicDispatch':dispatch,
    'nextPositiveTargets':[
        {'id':'N1','targets':['core1.BinaryOp.Ne','core1.BinaryOp.Lt','core1.BinaryOp.Or'],'sourceProposal':'proposal/crates/fixtures/positive-grammar-edges/core1.mori','test':'core1_positive_less_inequality_and_disjunction'},
        {'id':'N2','targets':['core2.Expr.Not','core2.BinaryOp.Ne','core2.BinaryOp.Lt','core2.BinaryOp.Or','core2.Statement.Ensures'],'sourceProposal':'proposal/crates/fixtures/positive-grammar-edges/core2.mori','test':'core2_positive_boolean_values_survive_await_and_ensure'},
        {'id':'N3','targets':['core2.Expr.LiabilityPresent','core2.Expr.LiabilityAmount','core2.BinaryOp.Gt'],'status':'specified-only','nextStep':'Extend an independently frozen normal staged duty specimen with current-snapshot reads and positive > guard. Freeze complete ordinary financial/state/work expectation first.'}],
    'rootCommand':{'argv':['cargo','test','--locked','-j1','-p','mori-core','--no-default-features','--test','positive_grammar_edges','--','--test-threads=1'],'cwd':'root-selected integration checkout after candidate adoption','executed':False,'inspection':'Proposed target uses ordinary parse/simulate/stage APIs; optional Midnight compatibility feature remains off.'},
    'retainedScope':'148 requirements/292 scenarios/20 families and all U/M/MC/SP/G/finance/composition/targets unchanged; no parent closure'}
for profile in [1,2]:
    coverage['core'+str(profile)+'UncoveredInInspectedCorpus'] = [r['constructor'] for r in rows if r['profiles']['core'+str(profile)]['coverage']=='no-positive-witness-in-inspected-corpus']
write('COVERAGE.json', coverage)
write('SOURCE-PINS.json', {'commit':BASE,'sources':list(PINS.values()),'inspection':'Exact pinned blobs; selected positive functions and public grammar/dispatch only; no new observations'})
print('Rows:',len(rows),'fixtures:',len(fixtures),'witnesses:',len(witnesses))
print('Core1 gaps:',coverage['core1UncoveredInInspectedCorpus'])
print('Core2 gaps:',coverage['core2UncoveredInInspectedCorpus'])
