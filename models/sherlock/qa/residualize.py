"""UIREQ-033: bounded static residualization model producing ResidualStepIR.

A finite memoized worklist keyed by (reference-rule location, source occurrence,
finite static layout facts) folds decoding/static dispatch only. Control, data,
environments, frames/depth, fuel/work, capacity checks and outcomes stay
dynamic; both successors of a dynamic condition remain; recursion stays a
runtime call (a back edge, never unrolled); over the residual-size limit the
specializer emits generic dispatch rather than application exhaustion.
Nothing is evaluated and circuit cost is not measured.
"""
import hashlib
import json

FORMAT = 'sherlock-residual-step-ir/1'
LAYOUT = 'sherlock-layout/1'
PROFILE = 'sherlock-k-a/1'
DYNAMIC_INPUTS = ('control', 'input-data', 'environment', 'frames', 'depth', 'fuel-work',
                  'helper-cursor', 'outcome', 'capacity-checks')
# Reference rules: id -> (source occurrence, kind, successors, charge)
# kind: dispatch (static opcode decode), cond (dynamic condition), call, return
RULES = {
    'R_fetch': ('occ:0', 'dispatch', ('R_cond', 'R_call'), 1),
    'R_cond': ('occ:1', 'cond', ('R_then', 'R_else'), 1),
    'R_then': ('occ:2', 'return', (), 1),
    'R_else': ('occ:3', 'return', (), 1),
    'R_call': ('occ:4', 'call', ('R_fetch',), 1),   # recursion: call back into R_fetch
}
ENTRY = 'R_fetch'
FAMILIES = {'arith': ('R_fetch', 'R_cond', 'R_then', 'R_else'),
            'recursive': tuple(RULES)}
IMAGES = {'img-sum': dict(imageId='img-sum', rules=('R_fetch', 'R_cond', 'R_then', 'R_else', 'R_call'))}


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def specialize(mask='empty', family=None, image=None, limit=64):
    if mask == 'empty':
        live, static = set(RULES), {}
    elif mask == 'family':
        if family not in FAMILIES:
            raise ValueError('unknown family')
        live, static = set(FAMILIES[family]), dict(family=family)
    elif mask == 'image':
        if image not in IMAGES:
            raise ValueError('unknown image')
        live, static = set(IMAGES[image]['rules']), dict(programImageId=image)
    else:
        raise ValueError('unknown mask')
    blocks, seen, pending, fallbacks = {}, set(), [ENTRY], []
    while pending:
        rule = pending.pop(0)
        key = (rule, RULES[rule][0], LAYOUT)
        if key in seen:
            continue
        seen.add(key)
        occ, kind, succ, charge = RULES[rule]
        if len(blocks) >= limit:
            fallbacks.append(dict(rule=rule, occurrence=occ, reason='residual-size-limit',
                                  fallback='generic-dispatch'))
            continue
        # Static folding applies to decode/dispatch only; successors are never dropped.
        if kind == 'dispatch':
            folded = [s for s in succ if s in live] if mask != 'empty' else list(succ)
            body = dict(kind='static-dispatch', targets=folded, dynamicSelector='control')
            if mask != 'empty' and len(folded) != len(succ):
                body['foldedAway'] = [s for s in succ if s not in live]
        elif kind == 'cond':
            body = dict(kind='dynamic-branch', targets=list(succ), dynamicSelector='input-data')
        elif kind == 'call':
            body = dict(kind='runtime-call', targets=list(succ), backEdge=True, unrolled=False,
                        frameCheck='dynamic')
        else:
            body = dict(kind='return', targets=[], outcome='dynamic')
        body.update(rule=rule, occurrence=occ, charge=charge, capacityCheck='dynamic', fuelCheck='dynamic')
        blocks[rule] = body
        pending.extend(t for t in body['targets'] if t in RULES)
    body = dict(referenceDefinitionId='stepReference', profileId=PROFILE, layoutVersion=LAYOUT,
                programImageId=static.get('programImageId'), realization=mask,
                blocks=[blocks[k] for k in sorted(blocks)], dynamicInputs=list(DYNAMIC_INPUTS),
                staticBindings=static,
                ruleBoundaryMap={k: dict(occurrence=b['occurrence'], charge=b['charge']) for k, b in sorted(blocks.items())},
                sourceOccurrenceMap={b['occurrence']: b['rule'] for b in blocks.values()},
                genericFallbacks=fallbacks,
                omissions=['circuit cost unmeasured', 'adoption unmeasured'])
    return dict(irId=digest(body), **body)


def report(limit=64):
    runs = [specialize('empty', limit=limit), specialize('family', family='arith', limit=limit),
            specialize('family', family='recursive', limit=limit),
            specialize('image', image='img-sum', limit=limit)]
    return dict(format=FORMAT, status='residualized', execution='none', acceptance='none',
                residualSizeLimit=limit, referenceDefinitionId='stepReference',
                realizations=runs,
                invariants=dict(
                    foldedOnly='immutable code/declarations/layout facts',
                    dynamic=list(DYNAMIC_INPUTS),
                    recursion='runtime calls; no depth/fuel/local-value specialization or unrolling',
                    sizeLimit='generic dispatch fallback; no application exhaustion',
                    circuitCost='unmeasured'),
                small_limit_probe=specialize('empty', limit=2))
