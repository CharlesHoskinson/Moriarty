"""UIREQ-031: separate ConstraintPlan / WitnessPlan preparation report.

Pure model of the interfaces.md "Witness, constraints and exports" section.
Nothing is executed: no constraint is emitted, no witness evaluated, no proof
run. The report lists constraint export, witness-program export,
lookup/interact/custom-gate support and field/layout support as separate
dispositions and never lets constraint JSON stand in for witness computation.
"""
import hashlib
import json

FORMAT = 'sherlock-export-plans/1'
FIELD = 'bls12-381-scalar'
LAYOUT = 'sherlock-layout/1'
PROFILE = 'sherlock-k-a/1'

# (operationId, domain, witnessProgramKind, constraint component, feature class)
STEP = (
    ('FieldAdd', 'field', 'structured', 'component:field-add', 'basic'),
    ('FieldMul', 'field', 'structured', 'component:field-mul', 'basic'),
    ('AssertZero', 'field', 'structured', 'component:assert-zero', 'basic'),
    ('LimbExtract', 'u128', 'structured', 'component:limb-extract', 'basic'),
    ('CheckedU128Add', 'u128', 'structured', 'component:checked-u128-add', 'basic'),
    ('NativeWitnessClosure', 'host', 'native-closure', None, 'native-closure'),
    ('LookupRangeU16', 'u16', 'structured', 'component:lookup-u16', 'lookup'),
    ('InteractBus', 'bus', 'structured', 'component:interact-bus', 'interaction'),
)

BACKENDS = {
    # Plain constraint-JSON backend: constraints only, no witness programs.
    'constraint-json-basic': dict(
        sourceMembership='tooling', fieldId=FIELD, layoutVersions=[LAYOUT],
        formats=['constraint-json'],
        constraintOperations=['FieldAdd', 'FieldMul', 'AssertZero', 'LimbExtract', 'CheckedU128Add'],
        witnessOperations=[], lookup=False, interaction=False, customGates=False,
        structuredWitnessIR=False, nativeClosures=False, midnightExporter=False),
    # Structured witness backend; still no lookup/interaction/native closures.
    'structured-witness-basic': dict(
        sourceMembership='tooling', fieldId=FIELD, layoutVersions=[LAYOUT],
        formats=['constraint-json', 'structured-witness'],
        constraintOperations=['FieldAdd', 'FieldMul', 'AssertZero', 'LimbExtract', 'CheckedU128Add'],
        witnessOperations=['FieldAdd', 'FieldMul', 'AssertZero', 'LimbExtract', 'CheckedU128Add'],
        lookup=False, interaction=False, customGates=False,
        structuredWitnessIR=True, nativeClosures=False, midnightExporter=False),
    # Midnight/ZKIR exporter is not implemented: nothing is claimed.
    'midnight-zkir': dict(
        sourceMembership='tooling', fieldId=FIELD, layoutVersions=[LAYOUT],
        formats=[], constraintOperations=[], witnessOperations=[],
        lookup=False, interaction=False, customGates=False,
        structuredWitnessIR=False, nativeClosures=False, midnightExporter=False),
}


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def unsupported(feature, op, reason):
    return dict(status='Unsupported', feature=feature, operationId=op, reason=reason,
                locator=f'step[{[o[0] for o in STEP].index(op)}]:{op}')


def manifest():
    ops = []
    for index, (op, domain, kind, component, feature) in enumerate(STEP):
        ops.append(dict(operationId=op, domain=domain, ordering=index,
                        sourceLocators=[f'reference-step:{op}'],
                        witnessDependencies=[STEP[index - 1][0]] if index else [],
                        constraintComponentLocator=component, witnessProgramKind=kind,
                        supportedStatus='Specified' if feature == 'basic' else 'Unsupported'))
    body = dict(referenceDefinitionId='stepReference', profileId=PROFILE, layoutVersion=LAYOUT,
                operations=ops)
    return dict(manifestId=digest(body), **body,
                typedOperations=[dict(operationId=o[0], domain=o[1]) for o in STEP],
                orderedInputs=['pre_image', 'program_image', 'fuel'],
                orderedOutputs=['next_image', 'outcome'],
                sourceMembership='reference-step', componentMembership=sorted({o[3] for o in STEP if o[3]}),
                omissions=[o[0] for o in STEP if o[4] != 'basic'])


def prepare(backend_id='structured-witness-basic'):
    cap = BACKENDS[backend_id]
    man = manifest()
    unsup_c, unsup_w = [], []
    for op, _, kind, component, feature in STEP:
        if feature == 'native-closure':
            unsup_w.append(unsupported('native-witness-closure', op, 'host closure has no exportable witness program'))
            unsup_c.append(unsupported('native-witness-closure', op, 'no constraint component declared'))
            continue
        if feature in ('lookup', 'interaction'):
            ok = cap['lookup'] if feature == 'lookup' else cap['interaction']
            if not ok:
                unsup_c.append(unsupported(feature, op, f'backend {backend_id} has no {feature} support'))
                unsup_w.append(unsupported(feature, op, f'backend {backend_id} has no {feature} support'))
            continue
        if op not in cap['constraintOperations']:
            unsup_c.append(unsupported('constraint-operation', op, f'backend {backend_id} does not emit this component'))
        if op not in cap['witnessOperations']:
            unsup_w.append(unsupported('witness-operation', op, f'backend {backend_id} has no witness program export'))
    layout_ok = LAYOUT in cap['layoutVersions'] and cap['fieldId'] == FIELD
    cplan_body = dict(operationManifestId=man['manifestId'], layoutVersion=LAYOUT, fieldId=FIELD,
                      orderedPublicInputs=['program_image', 'fuel'], orderedPrivateInputs=['pre_image'],
                      orderedOutputs=['next_image', 'outcome'],
                      components=[dict(componentId=o[3], operationId=o[0], sourceLocator=f'reference-step:{o[0]}',
                                       parameters={}) for o in STEP if o[3] and o[4] == 'basic'],
                      requiredCapabilities=['constraint-json'],
                      omissions=[u['operationId'] for u in unsup_c])
    wplan_body = dict(operationManifestId=man['manifestId'], layoutVersion=LAYOUT,
                      inputBindings=['pre_image', 'program_image', 'fuel'],
                      nodes=[dict(nodeId=f'n{i}', operationId=o[0], typedInputs=[dict(name='in', type=o[1])],
                             typedOutputs=[dict(name='out', type=o[1])], dependencies=[f'n{i-1}'] if i else [])
                             for i, o in enumerate(STEP) if o[4] == 'basic'],
                      stableTopologicalOrder='declaration', outputBindings=['next_image', 'outcome'],
                      programKind='structured-witness', omissions=[u['operationId'] for u in unsup_w])
    cexport = 'Unsupported' if unsup_c or not layout_ok or not cap['formats'] else 'Supported'
    wexport = 'Unsupported' if unsup_w or not cap['structuredWitnessIR'] else 'Supported'
    return dict(
        format=FORMAT, status='prepared-with-unsupported' if (unsup_c or unsup_w) else 'prepared',
        backendId=backend_id, execution='none', acceptance='none',
        operationManifest=man,
        constraintPlan=dict(planId=digest(cplan_body), **cplan_body),
        witnessPlan=dict(planId=digest(wplan_body), **wplan_body),
        backendCapabilities=dict(backendId=backend_id, **{k: v for k, v in cap.items()}),
        disposition=dict(
            constraintExport=dict(status=cexport, unsupported=unsup_c),
            witnessProgramExport=dict(status=wexport, unsupported=unsup_w),
            lookup=dict(status='Supported' if cap['lookup'] else 'Unsupported'),
            interact=dict(status='Supported' if cap['interaction'] else 'Unsupported'),
            customGates=dict(status='Supported' if cap['customGates'] else 'Unsupported'),
            fieldLayout=dict(status='Supported' if layout_ok else 'Unsupported',
                             fieldId=FIELD, layoutVersion=LAYOUT)),
        claims=dict(constraintJsonSuppliesWitnessComputation=False,
                    nonMidnightFormatIsMidnightExporter=False,
                    midnightZkirExporter='not-implemented'),
        tooling_status='Unsupported' if (unsup_c or unsup_w) else 'Supported')


def report():
    return dict(format=FORMAT, status='prepared',
                backends=[prepare(b) for b in sorted(BACKENDS)], execution='none', acceptance='none')
