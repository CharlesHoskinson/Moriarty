"""Positive declaration-level Core helper/result field port; no lane evaluation.
Exact externally tagged native carriers are a declared input premise. This is
not admission, a parser/security policy, or a transition evaluator. No files,
K tool, subprocess, model or expected data are read or executed by this module.
The full source document is retained alongside all ordered query terms.
"""
HELPER_FIELDS = {
  "Scalar": [
    [
      "common",
      "CHCommon"
    ],
    [
      "mode",
      "CHMode"
    ],
    [
      "operand_a",
      "CTRef"
    ],
    [
      "operand_b",
      "CHOptRef"
    ],
    [
      "expected",
      "CHType"
    ],
    [
      "phase",
      "CHScalarPhase"
    ],
    [
      "reg_a",
      "Int"
    ],
    [
      "staged",
      "CHStaged"
    ]
  ],
  "EnvGet": [
    [
      "common",
      "CHCommon"
    ],
    [
      "binder",
      "Int"
    ],
    [
      "cursor",
      "CHIndex"
    ],
    [
      "found",
      "CHOptRef"
    ],
    [
      "status",
      "CHTri"
    ]
  ],
  "EnvPut": [
    [
      "common",
      "CHCommon"
    ],
    [
      "binder",
      "Int"
    ],
    [
      "value",
      "CTRef"
    ],
    [
      "base",
      "CTEnv"
    ],
    [
      "new_root",
      "CHIndex"
    ],
    [
      "status",
      "CHTri"
    ]
  ],
  "Find": [
    [
      "common",
      "CHCommon"
    ],
    [
      "function_name",
      "Int"
    ],
    [
      "row_cursor",
      "Int"
    ],
    [
      "found",
      "CHIndex"
    ],
    [
      "status",
      "CHTri"
    ]
  ],
  "Bind": [
    [
      "common",
      "CHCommon"
    ],
    [
      "function",
      "Int"
    ],
    [
      "params",
      "CTSlice"
    ],
    [
      "args",
      "CTSlice"
    ],
    [
      "p_cursor",
      "Int"
    ],
    [
      "a_cursor",
      "Int"
    ],
    [
      "partial",
      "CTEnv"
    ],
    [
      "dup_cursor",
      "CHIndex"
    ],
    [
      "phase",
      "CHBindPhase"
    ],
    [
      "status",
      "CHBindStatus"
    ]
  ],
  "Append": [
    [
      "common",
      "CHCommon"
    ],
    [
      "src",
      "CTSlice"
    ],
    [
      "value",
      "CTRef"
    ],
    [
      "src_cursor",
      "Int"
    ],
    [
      "dest_start",
      "Int"
    ],
    [
      "staged",
      "CHOptRef"
    ],
    [
      "phase",
      "CHAppendPhase"
    ]
  ],
  "Validate": [
    [
      "common",
      "CHCommon"
    ],
    [
      "root",
      "CTRef"
    ],
    [
      "status",
      "CHTri"
    ]
  ],
  "Equal": [
    [
      "common",
      "CHCommon"
    ],
    [
      "left",
      "CTRef"
    ],
    [
      "right",
      "CTRef"
    ],
    [
      "status",
      "CHTri"
    ]
  ],
  "Match": [
    [
      "common",
      "CHCommon"
    ],
    [
      "scrutinee",
      "CTRef"
    ],
    [
      "arm",
      "Int"
    ],
    [
      "saved_env",
      "CTEnv"
    ],
    [
      "partial",
      "CTEnv"
    ],
    [
      "pat_tag",
      "CHPattern"
    ],
    [
      "x",
      "Int"
    ],
    [
      "y",
      "Int"
    ],
    [
      "field_idx",
      "Int"
    ],
    [
      "staged",
      "CHOptRef"
    ],
    [
      "phase",
      "CHMatchPhase"
    ],
    [
      "status",
      "CHTri"
    ]
  ],
  "Plan": [
    [
      "common",
      "CHCommon"
    ],
    [
      "source_match",
      "Int"
    ],
    [
      "scrutinee",
      "CTRef"
    ],
    [
      "saved_env",
      "CTEnv"
    ],
    [
      "partial",
      "CTEnv"
    ],
    [
      "node",
      "Int"
    ],
    [
      "phase",
      "CHPlanPhase"
    ],
    [
      "after_walk",
      "CHAfterWalk"
    ],
    [
      "cur",
      "CTRef"
    ],
    [
      "path",
      "CTSlice"
    ],
    [
      "path_cursor",
      "Int"
    ],
    [
      "arm",
      "CHIndex"
    ],
    [
      "status",
      "CHTri"
    ]
  ],
  "Build": [
    [
      "common",
      "CHCommon"
    ],
    [
      "type_id",
      "Int"
    ],
    [
      "ctor_id",
      "Int"
    ],
    [
      "src",
      "CTSlice"
    ],
    [
      "src_cursor",
      "Int"
    ],
    [
      "field_start",
      "Int"
    ],
    [
      "staged",
      "CHOptRef"
    ],
    [
      "node",
      "CHOptRef"
    ],
    [
      "phase",
      "CHBuildPhase"
    ]
  ]
}
RESULT_FIELDS = {
  "Validated": [
    [
      "ok",
      "Bool"
    ]
  ],
  "EnvFound": [
    [
      "value",
      "CTRef"
    ]
  ],
  "EnvMissing": [],
  "EnvUpdated": [
    [
      "cell",
      "Int"
    ]
  ],
  "FindFound": [
    [
      "function",
      "Int"
    ]
  ],
  "FindMissing": [],
  "BindOk": [
    [
      "env",
      "CTEnv"
    ]
  ],
  "BindBad": [],
  "AppendDone": [
    [
      "seq",
      "CTSlice"
    ]
  ],
  "ScalarValue": [
    [
      "value",
      "CTRef"
    ]
  ],
  "ScalarError": [
    [
      "code",
      "Int"
    ]
  ],
  "ScalarSelector": [
    [
      "sel",
      "CHSelector"
    ]
  ],
  "ScalarTypeTest": [
    [
      "ok",
      "Bool"
    ]
  ],
  "EqualDone": [
    [
      "value",
      "CTRef"
    ]
  ],
  "MatchMatched": [
    [
      "env",
      "CTEnv"
    ]
  ],
  "MatchNoMatch": [],
  "PlanMatched": [
    [
      "arm",
      "Int"
    ],
    [
      "env",
      "CTEnv"
    ]
  ],
  "PlanNoMatch": [],
  "BuildDone": [
    [
      "value",
      "CTRef"
    ]
  ]
}
ENUM_VARIANTS = {
  "CHSite": [
    "EvalLiteral",
    "EvalVar",
    "EvalLet",
    "EvalBinary",
    "EvalUnary",
    "EvalIf",
    "EvalMatch",
    "EvalCall",
    "Push",
    "RetLet",
    "RetLeft",
    "RetRight",
    "RetUnary",
    "RetIf",
    "RetMatch",
    "RetArgs",
    "RetCall",
    "RetScope",
    "Invoke",
    "Enter",
    "Bound",
    "Cases",
    "CaseResult",
    "Prim",
    "EvalConstruct",
    "RetCtorFields",
    "Materialize"
  ],
  "CHMode": [
    "Add",
    "Sub",
    "Mul",
    "Lt",
    "Cons",
    "Branch",
    "Leaf",
    "Not",
    "TestBool",
    "TestType"
  ],
  "CHScalarPhase": [
    "First",
    "Second",
    "Done"
  ],
  "CHNode": [
    "Cons",
    "Leaf",
    "Branch"
  ],
  "CHSelector": [
    "True",
    "False",
    "NotBool"
  ],
  "CHTri": [
    "Pending",
    "Positive",
    "Negative"
  ],
  "CHBindPhase": [
    "Pairing",
    "UnwindType",
    "UnwindDup",
    "UnwindInsert",
    "Propagate",
    "Finished"
  ],
  "CHBindStatus": [
    "Ok",
    "Bad"
  ],
  "CHAppendPhase": [
    "Copy",
    "Tail",
    "Done"
  ],
  "CHMatchPhase": [
    "Arm",
    "Tag",
    "Distinct",
    "Field",
    "Bind",
    "Ready",
    "NoMatchReady",
    "Done"
  ],
  "CHPlanPhase": [
    "Walk",
    "Test",
    "Bind",
    "Ready",
    "NoMatchReady",
    "Done"
  ],
  "CHAfterWalk": [
    "Test",
    "Bind"
  ],
  "CHBuildPhase": [
    "Copy",
    "Done"
  ],
  "CHPattern": [
    "PVar",
    "PNil",
    "PCons",
    "PLeaf",
    "PBranch",
    "WildcardExtensionReserved",
    "ConstructorExtensionReserved"
  ]
}
STAGED_FIELDS = {
    'None': [], 'U128Value': [('n','Int')], 'BoolValue': [('b','Bool')],
    'PendingNode': [('tag','CHNode')], 'PrimError': [('code','Int')],
    'Selector': [('sel','CHSelector')], 'TypeTest': [('ok','Bool')],
}

def leaf(sort, value):
    # Preserve native scalar identity; do not coerce bool/int or truncate u128.
    return {'sort':sort, 'value':value}

def term(sort, constructor, *arguments):
    return {'sort':sort, 'constructor':constructor, 'arguments':list(arguments)}

def port_value(sort, value):
    if sort in ('Int','Bool','String'):
        return leaf(sort,value)
    if sort in ENUM_VARIANTS:
        # Exact public enum string is a declared positive input premise.
        ENUM_VARIANTS[sort].index(value)
        return term(sort, 'ch'+sort[2:]+value)
    if sort == 'CTRef':
        (variant,index), = value.items()
        return term(sort, {'Const':'ctConst','Data':'ctData'}[variant],leaf('Int',index))
    if sort in ('CTEnv','CHIndex','CHResume','CHType'):
        (variant,payload), = value.items()
        prefix={'CTEnv':'ctEnv','CHIndex':'chIndex','CHResume':'chResume','CHType':'chType'}[sort]
        if variant == 'None':
            return term(sort,prefix+'None')
        return term(sort, {'Some':prefix+'Some'}[variant],leaf('Int',payload))
    if sort == 'CHOptRef':
        (variant,payload), = value.items()
        if variant == 'None':
            return term(sort,'chRefNone')
        return term(sort,{'Some':'chRefSome'}[variant],port_value('CTRef',payload))
    if sort == 'CTSlice':
        return term(sort,'ctSlice',leaf('Int',value['start']),leaf('Int',value['len']))
    if sort == 'CHCommon':
        return term(sort,'chCommon',port_value('CHSite',value['site']),
                    leaf('Int',value['origin_reference_cursor']),port_value('CHResume',value['parent_resume']))
    if sort == 'CHStaged':
        (variant,payload), = value.items()
        fields=STAGED_FIELDS[variant]
        return term(sort,'chStaged'+variant,*(port_value(t,payload[k]) for k,t in fields))
    # No other declared positive field sort. Natural lookup refusal, no fallback.
    raise KeyError(sort)

def port_helper(value):
    if value is None:
        return term('CHHelperOption','chNoHelper')
    (variant,payload), = value.items()
    fields=HELPER_FIELDS[variant]
    helper=term('CHHelper','ch'+variant,*(port_value(t,payload[k]) for k,t in fields))
    return term('CHHelperOption','chSomeHelper',helper)

def port_result(value):
    if value is None:
        return term('CHResultOption','chNoResult')
    (variant,payload), = value.items()
    fields=RESULT_FIELDS[variant]
    result=term('CHResult','chResult'+variant,*(port_value(t,payload[k]) for k,t in fields))
    return term('CHResultOption','chSomeResult',result)

def port_document(source):
    tail=term('CHQueries','chNoQueries')
    for row in reversed(source['result_cases']):
        tail=term('CHQueries','chResultQuery',leaf('String',row['case_id']),port_result(row['result']),tail)
    for row in reversed(source['helper_cases']):
        tail=term('CHQueries','chHelperQuery',leaf('String',row['case_id']),port_helper(row['helper']),tail)
    return {'schema':'sherlock-c6-helper-field-port/1',
            'profile':'logical-C6-helper-record-positive-codec/1',
            'source_input':source, 'queries':tail}
