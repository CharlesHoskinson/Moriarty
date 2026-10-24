"""Finite six-Bool K-law diagnostic query renderer / Bool result decoder.

SOURCE ONLY. This module never launches K, Java, a subprocess, a runner,
an owner engine, or any admission procedure. It only:

  * constructs three PUBLIC UNADMITTED synthetic native snapshots,
  * renders six closed Bool law queries as typed KAST JSONv4 against the
    actual compiled-317 labels already validated by envget_runtime,
  * proves a complete typed query/snapshot inverse in Python transport,
  * decodes one printed Bool primitive per saved law print.

Python computes no miss/hit/Exit afterstate, counter update, event, or
law answer. K alone would evaluate the frozen law functions in a later,
separately reviewed Root operation using unchanged F03. Future Root may
manually compose F03; this module is not a caller/runner.

Synthetic occurrence identity: S_event extends only admission.view of the
unadmitted synthetic seed with owner-authored DATA literals
``occurrences``/``occurrence_paths`` sufficient for the actual K helpers
egSourceOccurrence/egSourceFromView/egSourceFromRow (plus egGet/egAt/
egDrop/egTake). These literals are proposed diagnostic inputs requiring
source review. They are not a genuine capture, not a Core canonical
context, not an admission, and not an independent semantic golden.
"""
import argparse
import copy
import json
from contextlib import contextmanager
from pathlib import Path
import sys

# Complete dependency binding: the two existing bounded-runtime modules in
# the same candidate source tree, reached relative to this file so no
# shared source is edited and no absolute Python path is invented.
_HERE = Path(__file__).resolve()
for _candidate in (_HERE.parent, _HERE.parents[1] / 'source', _HERE.parent / 'source'):
    if (_candidate / 'envget_runtime.py').exists():
        sys.path.insert(0, str(_candidate))
        break
else:
    raise ImportError('existing envget_runtime source tree not found')

import envget_ports as ports
import envget_runtime as runtime

base = runtime.base

# --------------------------------------------------------------------------
# Closed law declarations: copied exactly from the frozen definition
# qualification. Raw compiled labels are observed, never guessed.
# --------------------------------------------------------------------------
LAW_DECLARATIONS = (
    {
        'surface_name': 'egMissPreservesWholeCarrier',
        'sort': 'Bool',
        'argument_sorts': ('EGSnapshot',),
        'arity': 1,
        'role': 'function',
        'declaring_module': 'SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS',
        'compiled_klabel': 'egMissPreservesWholeCarrier(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGSnapshot',
        'input_variant': 'S',
    },
    {
        'surface_name': 'egMissPreviousNoneRemainsPending',
        'sort': 'Bool',
        'argument_sorts': ('EGSnapshot',),
        'arity': 1,
        'role': 'function',
        'declaring_module': 'SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS',
        'compiled_klabel': 'egMissPreviousNoneRemainsPending(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGSnapshot',
        'input_variant': 'S_none',
    },
    {
        'surface_name': 'egMissNonzeroToSomeZero',
        'sort': 'Bool',
        'argument_sorts': ('EGSnapshot',),
        'arity': 1,
        'role': 'function',
        'declaring_module': 'SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS',
        'compiled_klabel': 'egMissNonzeroToSomeZero(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGSnapshot',
        'input_variant': 'S',
    },
    {
        'surface_name': 'egMissEventCursorAndAccounting',
        'sort': 'Bool',
        'argument_sorts': ('EGSnapshot',),
        'arity': 1,
        'role': 'function',
        'declaring_module': 'SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS',
        'compiled_klabel': 'egMissEventCursorAndAccounting(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGSnapshot',
        'input_variant': 'S_event',
    },
    {
        'surface_name': 'egTraversalLinked',
        'sort': 'Bool',
        'argument_sorts': ('EGTraversalWindow',),
        'arity': 1,
        'role': 'function',
        'declaring_module': 'SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS',
        'compiled_klabel': 'egTraversalLinked(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGTraversalWindow',
        'input_variant': 'S_event',
    },
    {
        'surface_name': 'egMissOffChainPreviousDiscriminator',
        'sort': 'Bool',
        'argument_sorts': ('EGSnapshot',),
        'arity': 1,
        'role': 'function',
        'declaring_module': 'SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS',
        'compiled_klabel': 'egMissOffChainPreviousDiscriminator(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGSnapshot',
        'input_variant': 'S',
    },
)

QUERY_ORDER = tuple((row['surface_name'], row['input_variant']) for row in LAW_DECLARATIONS)
LAW_BY_NAME = {row['surface_name']: row for row in LAW_DECLARATIONS}
NESTED_ENTRY_NAME = 'egThreeEventTraversal'
NESTED_ENTRY_DECL = ('EGTraversalWindow', ('EGSnapshot',))
PRIMITIVE_SORTS = ('Int', 'Bool', 'String')

RENDER_PASS = 'PASS_SIX_BOOL_DIAGNOSTIC_QUERIES_RENDERED'
LAW_PASS = 'LAW_PASS'
LAW_FAIL = 'LAW_FAIL'
PARSE_FAILURE = 'FAIL_BOOL_PRINT_PARSE'

DIAGNOSTIC_IDENTITY = (
    'PUBLIC UNADMITTED synthetic six-Bool law diagnostic; algebraic '
    'source inputs only; not BEFORE_PIN, not admitted, not a genuine '
    'finalVarx capture, no Python afterstate')

# Verify every closed declaration against the already-qualified runtime
# maps at import time. Any drift is a hard source error, never repaired.
for _row in LAW_DECLARATIONS:
    _name = _row['surface_name']
    if runtime.REG_ALL.get(_name) != (_row['sort'], _row['argument_sorts']):
        raise ValueError('six-law declaration differs from current317 registry: ' + _name)
    if runtime.REG_ROLE.get(_name) != 'function':
        raise ValueError('six-law role is not function in current317 registry: ' + _name)
    if runtime.LABELS.get(_name) != _row['compiled_klabel']:
        raise ValueError('six-law compiled label differs from actual317 map: ' + _name)
if runtime.REG_ALL.get(NESTED_ENTRY_NAME) != NESTED_ENTRY_DECL:
    raise ValueError('nested egThreeEventTraversal declaration differs')
if runtime.LABELS.get(NESTED_ENTRY_NAME) != (
        'egThreeEventTraversal(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-STEP'
        '_EGTraversalWindow_EGSnapshot'):
    raise ValueError('nested egThreeEventTraversal compiled label differs')

_BEFORE_PIN_AT_IMPORT = runtime.BEFORE_PIN


# --------------------------------------------------------------------------
# PUBLIC UNADMITTED synthetic seeds (source logic reused, not imported
# from the passed twelve-unit test file and never executed here).
# --------------------------------------------------------------------------
def synthetic_base_fixture():
    """Complete synthetic 23-field-image context; fresh copy per call."""
    active = dict(data=0, data_fields=0, env=1, value_slots=0)
    alloc = dict(data_nodes=0, data_fields=0, environment_cells=1)
    return dict(
        admission=dict(identity={'test': [False, 0, '0', None]},
                       bounds={}, view={'unused': []}, layout={}),
        image=dict(
            run_id={'test_run': ['immutable', 3]},
            control={'Eval': {'expr': 3}}, control_origin=3,
            env_root={'Some': 0}, stack_limit=8, outcome='Running',
            initial_fuel=128, remaining_work=120, consumed_work=8,
            helper_work=5, helper_visits=2, reference_cursor=6,
            alloc=alloc, active=active,
            helper={'Some': {'EnvGet': {
                'common': {'site': 'EvalVar', 'origin_reference_cursor': 6,
                           'parent_resume': {'Some': 0}},
                'binder': 0, 'cursor': {'Some': 0}, 'found': None,
                'status': 'Pending'}}},
            result=None,
            seg_base=dict(active=dict(active), alloc=dict(alloc),
                          consumed_work=4),
            data=[], data_fields=[],
            env=[{'binder': 0, 'value': {'Const': 0}, 'previous': None}],
            frames=[{'ScopeF': {'saved_env': None, 'origin': 0}},
                    {'LetF': {'binder': 1, 'body': 4,
                              'saved_env': {'Some': 0}, 'origin': 2}}],
            tasks=[], value_slots=[]))


def synthetic_S():
    """S: binder31, cursor Some2, Pending, found None; env rows in order
    (31, Const0, None), (31, Data1, None), (47, Const2, Some0). Row1 is
    the off-chain matching-binder trap; visited row2 previous is Some0."""
    x = synthetic_base_fixture()
    alloc = dict(data_nodes=0, data_fields=0, environment_cells=3)
    active = dict(data=0, data_fields=0, env=3, value_slots=0)
    x['image']['alloc'] = alloc
    x['image']['active'] = active
    x['image']['seg_base'] = dict(active=dict(active), alloc=dict(alloc),
                                  consumed_work=4)
    x['image']['helper'] = {'Some': {'EnvGet': {
        'common': {'site': 'EvalVar', 'origin_reference_cursor': 6,
                   'parent_resume': {'Some': 0}},
        'binder': 31, 'cursor': {'Some': 2}, 'found': None,
        'status': 'Pending'}}}
    x['image']['env'] = [
        {'binder': 31, 'value': {'Const': 0}, 'previous': None},
        {'binder': 31, 'value': {'Data': 1}, 'previous': None},
        {'binder': 47, 'value': {'Const': 2}, 'previous': {'Some': 0}}]
    return x


def synthetic_S_none():
    """S_none: distinct source-only copy of S; only row2.previous=None."""
    x = synthetic_S()
    x['image']['env'][2]['previous'] = None
    return x


# Explicit DATA-only synthetic occurrence literals. control_origin is 3,
# so occurrences[3] is the selected row. Its path slice start=0, len=2
# selects occurrence_paths[0:2] == [0, 3]. Non-selected rows are complete
# rows of the same declared shape and are never consulted by the helpers
# for this snapshot. owner_is_function/function are the native Bool/Int
# the K row extractor requires; no value here is a real source identity.
SYNTHETIC_OCCURRENCE_PATHS = [0, 3]
SYNTHETIC_OCCURRENCES = [
    {'owner_is_function': False, 'function': 0,
     'path': {'start': 0, 'len': 0}},
    {'owner_is_function': False, 'function': 0,
     'path': {'start': 0, 'len': 0}},
    {'owner_is_function': False, 'function': 0,
     'path': {'start': 0, 'len': 0}},
    {'owner_is_function': False, 'function': 0,
     'path': {'start': 0, 'len': 2}},
]


def synthetic_S_event():
    """S_event: S plus the frozen occurrence extension in admission.view.
    All original view fields and every other carrier field are retained."""
    x = synthetic_S()
    view = dict(x['admission']['view'])
    view['occurrences'] = copy.deepcopy(SYNTHETIC_OCCURRENCES)
    view['occurrence_paths'] = list(SYNTHETIC_OCCURRENCE_PATHS)
    x['admission']['view'] = view
    return x


INPUT_BUILDERS = {'S': synthetic_S, 'S_none': synthetic_S_none,
                  'S_event': synthetic_S_event}


def input_for_variant(variant):
    if variant not in INPUT_BUILDERS:
        raise ValueError('unknown closed diagnostic input variant: ' + repr(variant))
    return INPUT_BUILDERS[variant]()


# --------------------------------------------------------------------------
# Finite law-root typed-tree support. Shared runtime policy is untouched:
# temporary codec registry bindings are always restored with finally and
# the exact prior registry object content is verified afterwards.
# --------------------------------------------------------------------------
@contextmanager
def _preserved_runtime_state():
    saved_reg = dict(base.codec.REG)
    saved_before = runtime.BEFORE_PIN
    try:
        yield
    finally:
        # _codec_reg below restores REG itself; verify exact restoration
        # and that the genuine binding was never touched, even on error.
        if dict(base.codec.REG) != saved_reg:
            base.codec.REG = saved_reg
        if runtime.BEFORE_PIN is not saved_before:
            raise ValueError('genuine BEFORE_PIN identity changed by diagnostic')


def _validate_data_tree(tree):
    """Every node below a permitted function position must be DATA.

    This validator consumes the internal typed carrier. Node form, not
    result sort alone, selects the branch: primitive leaves are exactly
    {'sort', 'value'}, while constructor nodes are exactly
    {'sort', 'constructor', 'arguments'}. A DATA constructor whose
    result sort is Bool, Int, or String is therefore still validated as
    a constructor. Public wire trees use {'sort', 'literal'} leaves;
    validate_query_wire converts the extracted wire snapshot to this
    internal carrier before calling here.
    """
    if type(tree) is not dict or type(tree.get('sort')) is not str:
        raise ValueError('typed tree required')
    if 'constructor' in tree or 'arguments' in tree:
        runtime.keys(tree, ('sort', 'constructor', 'arguments'))
        name, args = tree['constructor'], tree['arguments']
        if type(name) is not str or name not in runtime.DATA_REG or type(args) is not list:
            raise ValueError('residual function refused in DATA position: ' + repr(name))
        sort, sorts = runtime.DATA_REG[name]
        if tree['sort'] != sort or len(args) != len(sorts):
            raise ValueError('DATA tree sort/arity differs: ' + repr(name))
        for child, child_sort in zip(args, sorts):
            if type(child) is not dict or child.get('sort') != child_sort:
                raise ValueError('DATA tree child sort differs: ' + repr(name))
            _validate_data_tree(child)
        return
    runtime.keys(tree, ('sort', 'value'))
    if tree['sort'] not in PRIMITIVE_SORTS:
        raise ValueError('internal primitive leaf sort differs: ' + tree['sort'])
    expected = {'Int': int, 'Bool': bool, 'String': str}[tree['sort']]
    if type(tree['value']) is not expected:
        raise ValueError('scalar native kind differs: ' + tree['sort'])


def _wire_to_internal_diag(tree):
    """Diagnostic-owned public-wire to internal typed-tree conversion.

    This is a carrier conversion only. It dispatches on exact node form,
    never on result sort alone: {'sort', 'constructor', 'arguments'} is
    a constructor node even when its sort is Bool, and {'sort',
    'literal'} is a primitive leaf only when its sort and native scalar
    kind are exact. Finite law-root, nested-position, DATA, and function
    admission remains the job of validate_query_wire and
    _from_tree_diag. The shared runtime converter is not widened and is
    not called on a diagnostic law root.
    """
    if type(tree) is not dict or type(tree.get('sort')) is not str:
        raise ValueError('typed wire tree required')
    if 'constructor' in tree or 'arguments' in tree:
        runtime.keys(tree, ('sort', 'constructor', 'arguments'))
        if type(tree['arguments']) is not list:
            raise ValueError('wire arguments list required')
        return {'sort': tree['sort'], 'constructor': tree['constructor'],
                'arguments': [_wire_to_internal_diag(x)
                              for x in tree['arguments']]}
    runtime.keys(tree, ('sort', 'literal'))
    if tree['sort'] not in PRIMITIVE_SORTS:
        raise ValueError('wire primitive leaf sort differs: ' + tree['sort'])
    expected = {'Int': int, 'Bool': bool, 'String': str}[tree['sort']]
    if type(tree['literal']) is not expected:
        raise ValueError('wire scalar native kind differs: ' + tree['sort'])
    return {'sort': tree['sort'], 'value': tree['literal']}


def _internal_to_wire_diag(tree):
    """Exact diagnostic-owned inverse of _wire_to_internal_diag."""
    if type(tree) is not dict or type(tree.get('sort')) is not str:
        raise ValueError('typed internal tree required')
    if 'constructor' in tree or 'arguments' in tree:
        runtime.keys(tree, ('sort', 'constructor', 'arguments'))
        if type(tree['arguments']) is not list:
            raise ValueError('internal arguments list required')
        return {'sort': tree['sort'], 'constructor': tree['constructor'],
                'arguments': [_internal_to_wire_diag(x)
                              for x in tree['arguments']]}
    runtime.keys(tree, ('sort', 'value'))
    if tree['sort'] not in PRIMITIVE_SORTS:
        raise ValueError('internal primitive leaf sort differs: ' + tree['sort'])
    expected = {'Int': int, 'Bool': bool, 'String': str}[tree['sort']]
    if type(tree['value']) is not expected:
        raise ValueError('internal scalar native kind differs: ' + tree['sort'])
    return {'sort': tree['sort'], 'literal': tree['value']}


def validate_query_wire(wire_query):
    """Closed finite validation: one of six law roots; DATA snapshot
    below, except egTraversalLinked whose single argument may be exactly
    egThreeEventTraversal(DATA EGSnapshot) at that position only.

    The public wire carrier is honored as supplied: ports nodes carry
    {'sort', 'constructor', 'arguments'} and primitive leaves carry
    {'sort', 'literal'}. The extracted EGSnapshot wire subtree is in the
    shared converter's existing DATA domain and is converted with
    runtime.wire_to_internal before _validate_data_tree. The Bool law
    root itself is never passed to that shared converter.
    """
    runtime.keys(wire_query, ('sort', 'constructor', 'arguments'))
    name = wire_query['constructor']
    if type(name) is not str or name not in LAW_BY_NAME:
        raise ValueError('root is not one of the closed six law functions')
    decl = LAW_BY_NAME[name]
    if (wire_query['sort'] != 'Bool'
            or type(wire_query['arguments']) is not list
            or len(wire_query['arguments']) != 1):
        raise ValueError('law query root sort/arity differs: ' + name)
    arg = wire_query['arguments'][0]
    if name == 'egTraversalLinked':
        runtime.keys(arg, ('sort', 'constructor', 'arguments'))
        if (arg['constructor'] != NESTED_ENTRY_NAME
                or arg['sort'] != 'EGTraversalWindow'
                or type(arg['arguments']) is not list
                or len(arg['arguments']) != 1):
            raise ValueError('egTraversalLinked requires sole nested egThreeEventTraversal')
        snapshot = arg['arguments'][0]
    else:
        if type(arg) is dict and arg.get('constructor') == NESTED_ENTRY_NAME:
            raise ValueError('nested traversal permitted only under egTraversalLinked')
        snapshot = arg
    if type(snapshot) is not dict or snapshot.get('sort') != 'EGSnapshot':
        raise ValueError('law argument does not terminate in EGSnapshot DATA')
    _validate_data_tree(runtime.wire_to_internal(snapshot))
    return decl


def build_query_wire(law_name, native_snapshot):
    """Wire (dict) law query from a native snapshot; no evaluation."""
    if law_name not in LAW_BY_NAME:
        raise ValueError('unknown closed law function: ' + repr(law_name))
    snapshot = ports.from_snapshot(native_snapshot)
    if law_name == 'egTraversalLinked':
        inner = ports.node(NESTED_ENTRY_NAME, snapshot)
        wire = ports.node(law_name, inner)
    else:
        wire = ports.node(law_name, snapshot)
    validate_query_wire(wire)
    return wire


def _snapshot_wire_of_query(wire_query):
    arg = wire_query['arguments'][0]
    if wire_query['constructor'] == 'egTraversalLinked':
        return arg['arguments'][0]
    return arg


def _from_tree_diag(tree, law_name, allowed):
    # allowed: True only at the law root and, for egTraversalLinked only,
    # at its single traversal child position. Node form selects the
    # branch: a Bool constructor node is not a Bool primitive leaf.
    if type(tree) is not dict or type(tree.get('sort')) is not str:
        raise ValueError('typed tree required')
    if 'constructor' in tree or 'arguments' in tree:
        runtime.keys(tree, ('sort', 'constructor', 'arguments'))
        name, args = tree['constructor'], tree['arguments']
        if type(name) is not str or name not in runtime.ALL_REG or type(args) is not list:
            raise ValueError('unknown typed constructor')
        if name not in runtime.DATA_REG and not allowed:
            raise ValueError('residual/entry function refused as DATA: ' + name)
        actual, sorts = runtime.ALL_REG[name]
        if tree['sort'] != actual or len(args) != len(sorts):
            raise ValueError('typed tree sort/arity differs')
        children = []
        for index, child in enumerate(args):
            child_allowed = (
                law_name == 'egTraversalLinked' and name == 'egTraversalLinked'
                and index == 0)
            children.append(_from_tree_diag(child, law_name, child_allowed))
        if tuple(child.sort for child in children) != sorts:
            raise ValueError('child typed sorts differ')
        return base.codec.C(name, *children)
    runtime.keys(tree, ('sort', 'value'))
    if tree['sort'] not in PRIMITIVE_SORTS:
        raise ValueError('internal primitive leaf sort differs: ' + tree['sort'])
    expected = {'Int': int, 'Bool': bool, 'String': str}[tree['sort']]
    if type(tree['value']) is not expected:
        raise ValueError('scalar native kind differs: ' + tree['sort'])
    return base.encode(tree['value'], tree['sort'])


def _encode_node_diag(term, law_name, allowed):
    if term.name in ('@Int', '@Bool', '@String'):
        expected = {'Int': int, 'Bool': bool, 'String': str}
        if term.sort not in expected or type(term.args[0]) is not expected[term.sort]:
            raise ValueError('scalar kind differs')
        return {'node': 'KToken',
                'sort': {'node': 'KSort', 'name': term.sort, 'params': []},
                'token': base.codec.render(term)}
    if term.name not in runtime.ALL_REG or term.name not in runtime.LABELS:
        raise ValueError('outside closed carrier profile')
    if term.name not in runtime.DATA_REG and not allowed:
        raise ValueError('residual/entry function refused as DATA: ' + term.name)
    sort, sorts = runtime.ALL_REG[term.name]
    if term.sort != sort or tuple(x.sort for x in term.args) != sorts:
        raise ValueError('typed constructor differs')
    return {'node': 'KApply',
            'label': {'node': 'KLabel', 'name': runtime.LABELS[term.name],
                      'params': []},
            'arity': len(term.args),
            'args': [_encode_node_diag(
                child, law_name,
                law_name == 'egTraversalLinked' and term.name == 'egTraversalLinked')
                for child in term.args]}


def _decode_node_diag(value, sort, law_name, allowed):
    if type(value) is not dict:
        raise ValueError('K node object required')
    if value.get('node') == 'KToken':
        runtime.keys(value, ('node', 'sort', 'token'))
        s = runtime.keys(value['sort'], ('node', 'name', 'params'))
        if (s != {'node': 'KSort', 'name': sort, 'params': []}
                or sort not in PRIMITIVE_SORTS
                or type(value['token']) is not str):
            raise ValueError('exact primitive KSort/token required')
        term = base.codec.parse_printed(value['token'], sort)
        if (type(term.args[0]) is not {'Int': int, 'Bool': bool, 'String': str}[sort]
                or base.codec.render(term) != value['token']):
            raise ValueError('noncanonical scalar lexeme')
        return term
    runtime.keys(value, ('node', 'label', 'arity', 'args'))
    if value['node'] != 'KApply':
        raise ValueError('outside KApply/KToken carrier profile')
    label = runtime.keys(value['label'], ('node', 'name', 'params'))
    if (label['node'] != 'KLabel' or label['params'] != []
            or type(label['name']) is not str
            or label['name'] not in runtime.REVERSE_LABELS):
        raise ValueError('unknown or parametric label')
    name = runtime.REVERSE_LABELS[label['name']]
    if name not in runtime.DATA_REG and not allowed:
        raise ValueError('residual/entry function refused as DATA: ' + name)
    actual, sorts = runtime.ALL_REG[name]
    if (actual != sort or type(value['arity']) is not int
            or value['arity'] != len(sorts) or type(value['args']) is not list
            or len(value['args']) != len(sorts)):
        raise ValueError('exact KApply sort/arity differs')
    return base.codec.C(name, *(
        _decode_node_diag(child, child_sort, law_name,
                          law_name == 'egTraversalLinked' and name == 'egTraversalLinked')
        for child, child_sort in zip(value['args'], sorts)))


def _envelope_diag(term, law_name):
    return {'format': 'KAST', 'version': 4,
            'term': _encode_node_diag(term, law_name, True)}


def _decode_document_diag(document, law_name):
    runtime.keys(document, ('format', 'version', 'term'))
    if (document['format'] != 'KAST' or type(document['version']) is not int
            or document['version'] != 4):
        raise ValueError('KAST JSONv4 required')
    return _decode_node_diag(document['term'], 'Bool', law_name, True)


def build_query_term(law_name, native_snapshot):
    """Return (codec term, KAST envelope, wire query) with a complete
    typed inverse proof. Registry and BEFORE_PIN are preserved."""
    decl = LAW_BY_NAME.get(law_name)
    if decl is None:
        raise ValueError('unknown closed law function: ' + repr(law_name))
    with _preserved_runtime_state():
        wire_query = build_query_wire(law_name, native_snapshot)
        internal = _wire_to_internal_diag(wire_query)
        if not runtime.same_whole(_internal_to_wire_diag(internal), wire_query):
            raise ValueError('law query wire/internal typed inverse differs')
        with runtime._codec_reg(dict(runtime.ALL_REG)):
            term = _from_tree_diag(internal, law_name, True)
            envelope = _envelope_diag(term, law_name)
            decoded = _decode_document_diag(envelope, law_name)
            if not base.same(base.tree(term) if False else _codec_tree(term),
                             _codec_tree(decoded)):
                raise ValueError('whole law query/KJSON typed inverse differs')
        if not runtime.same_whole(
                ports.to_snapshot(_snapshot_wire_of_query(wire_query)),
                native_snapshot):
            raise ValueError('snapshot field-port inverse differs')
        if runtime.BEFORE_PIN is not _BEFORE_PIN_AT_IMPORT:
            raise ValueError('genuine BEFORE_PIN changed during construction')
        return term, envelope, wire_query


def _codec_tree(term):
    """Typed codec term -> runtime internal dict, under ALL registry."""
    with runtime._codec_reg(dict(runtime.ALL_REG)):
        return runtime.base.tree(term)


def build_all_queries():
    """Six closed queries in QUERY_ORDER; fresh seed per entry."""
    entries = []
    for law_name, variant in QUERY_ORDER:
        native = input_for_variant(variant)
        term, envelope, wire_query = build_query_term(law_name, native)
        entries.append({
            'law': law_name,
            'input_variant': variant,
            'compiled_klabel': LAW_BY_NAME[law_name]['compiled_klabel'],
            'root_sort': 'Bool',
            'native_input': native,
            'wire_query': wire_query,
            'query_envelope': envelope,
        })
    return entries


# --------------------------------------------------------------------------
# Complete Bool print decoding (existing <k> framing/parser convention).
# Reads no expected/oracle body. False is preserved as semantic failure;
# residual, unknown, trailing, multiple-term, or wrong-scalar prints are
# parse failures. The printed text itself is never repaired or truncated.
# --------------------------------------------------------------------------
def parse_bool_print(text):
    if type(text) is not str:
        raise ValueError('printed Bool text required')
    with _preserved_runtime_state():
        with runtime._codec_reg(dict(runtime.DATA_REG)):
            term = base.codec.parse_printed(base.complete_term(text), 'Bool')
    if term.sort != 'Bool' or term.name != '@Bool' or type(term.args[0]) is not bool:
        raise ValueError('complete Bool primitive required')
    if base.codec.render(term) not in ('true', 'false'):
        raise ValueError('noncanonical Bool lexeme')
    return term.args[0]


def classify_bool_print(law_name, text):
    if law_name not in LAW_BY_NAME:
        raise ValueError('unknown closed law function: ' + repr(law_name))
    try:
        value = parse_bool_print(text)
    except Exception as error:  # parse failure is data, not repaired
        return {'law': law_name, 'status': PARSE_FAILURE,
                'bool_value': None, 'error_type': type(error).__name__}
    return {'law': law_name,
            'status': LAW_PASS if value else LAW_FAIL,
            'bool_value': value, 'error_type': None}


# --------------------------------------------------------------------------
# Products. Pins are computed only from files actually written.
# --------------------------------------------------------------------------
def render(out_dir, manifest_path):
    """Write six native inputs and six KAST queries plus a closed
    manifest whose pins are real after-write pins. out_dir must not
    already exist. No engine is invoked."""
    out = Path(out_dir)
    manifest_file = Path(manifest_path)
    if out.exists() or manifest_file.exists():
        raise ValueError('fresh diagnostic output paths required')
    (out / 'inputs').mkdir(parents=True)
    (out / 'queries').mkdir()
    entries = build_all_queries()
    manifest_entries = []
    for index, entry in enumerate(entries, start=1):
        stem = '%02d-%s' % (index, entry['law'])
        input_pin = base.write(
            out / 'inputs' / (stem + '.input.json'),
            (json.dumps(entry['native_input'], indent=2,
                        allow_nan=False) + '\n').encode())
        query_pin = base.write(
            out / 'queries' / (stem + '.query.json'),
            (json.dumps(entry['query_envelope'], separators=(',', ':'),
                        allow_nan=False) + '\n').encode())
        manifest_entries.append({
            'index': index,
            'law': entry['law'],
            'input_variant': entry['input_variant'],
            'compiled_klabel': entry['compiled_klabel'],
            'root_sort': 'Bool',
            'input': input_pin,
            'query': query_pin,
        })
    manifest = {
        'status': RENDER_PASS,
        'identity': DIAGNOSTIC_IDENTITY,
        'closed_query_order': [list(pair) for pair in QUERY_ORDER],
        'entries': manifest_entries,
        'six_laws_executed': False,
        'genuine_runtime_before_pin': 'None/UNBOUND (unchanged)',
        'expected_body_reads': 0,
    }
    manifest_pin = base.write(
        manifest_file,
        (json.dumps(manifest, indent=2, allow_nan=False) + '\n').encode())
    print(json.dumps({'status': RENDER_PASS, 'manifest': manifest_pin}))
    return manifest


def decode(law_name, printed_path, report_path):
    """Decode one saved Bool print; write a classification report whose
    printed pin is real. Exit status distinguishes pass/fail/parse."""
    printed_file = Path(printed_path)
    printed_pin = base.pin(printed_file)
    text = base.checked(printed_pin).decode('utf-8')
    result = classify_bool_print(law_name, text)
    result.update({
        'identity': DIAGNOSTIC_IDENTITY,
        'printed': printed_pin,
        'printed_text_preserved': True,
        'expected_body_reads': 0,
    })
    report_pin = base.write(
        Path(report_path),
        (json.dumps(result, indent=2, allow_nan=False) + '\n').encode())
    print(json.dumps({'status': result['status'], 'report': report_pin}))
    return result


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    sub = cli.add_subparsers(dest='action', required=True)
    x = sub.add_parser('render')
    x.add_argument('out_dir')
    x.add_argument('manifest')
    x = sub.add_parser('decode')
    x.add_argument('law')
    x.add_argument('printed')
    x.add_argument('report')
    a = cli.parse_args()
    if a.action == 'render':
        render(a.out_dir, a.manifest)
    else:
        result = decode(a.law, a.printed, a.report)
        raise SystemExit(0 if result['status'] == LAW_PASS else 1)


if __name__ == '__main__':
    main()
