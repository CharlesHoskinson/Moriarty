"""Positive data conversion for already bound ProgramView23 inputs.

This module consumes Python DATA from the existing receiving boundary. It does
not parse files, authenticate admission, evaluate programs, or receive K output.
The old Port validation/cycle/refusal behavior is inherited byte-exact. New
conversion/AST functions are defined only on the stated positive source profile.
"""
import copy
import program_view_to_ngprogram as previous
PROGRAM_FIELDS=previous.PROGRAM_FIELDS
BUILTIN_TYPES={"UInt":0,"Bool":1,"List":2,"Tree":3}
BUILTIN_CTORS={(2,0):"Nil",(2,1):"Cons",(3,0):"Leaf",(3,1):"Branch"}

def source_numbers(value,key=None):
    """Positive wire translation; names/templates and U128 decimal payload stay raw.

    All other decimal strings in this ProgramView profile are usize indices or
    lengths. TypeIdC unit/variant carriers become their existing NG table index.
    This is not an accepting grammar for arbitrary JSON.
    """
    if key in ("names","decl_templates","n"):
        return copy.deepcopy(value)
    if type(value) is dict:
        if tuple(value)==("TypeId",):
            return int(value["TypeId"])
        return {k:source_numbers(v,k) for k,v in value.items()}
    if type(value) is list:
        return [source_numbers(v) for v in value]
    if type(value) is str:
        if value in BUILTIN_TYPES:
            return BUILTIN_TYPES[value]
        if value.isascii() and value.isdecimal():
            return int(value)
    return value

class PositivePort(previous.Port):
    """Existing positive Port plus the already implemented source09 Not rule."""
    def expression(self,index,constants,patterns):
        row=self.p["expressions"][index]
        if type(row) is dict and tuple(row)==("Unary",) and row["Unary"].get("op")=="Not":
            payload=previous.record(row["Unary"],("op","operand"),f"expressions/{index}")
            operand=self.ref("expressions",payload["operand"],f"expressions/{index}/operand")
            return {"id":index,"occurrence":index,"kind":{"Not":{"operand":operand}}}
        return super().expression(index,constants,patterns)

def selected(rows,start,length):
    return rows[start:start+length]

def type_metadata(view,index):
    """Generic arguments use the source TypeRow args slice, in original order."""
    if index<2:
        return {"kind":("UInt","Bool")[index]}
    row=view["types"][index]
    args=selected(view["type_args"],row["args"]["start"],row["args"]["len"])
    name=("List","Tree")[index-2] if index<4 else view["decl_templates"][row["decl"]]["name"]
    return {"kind":"Nominal","name":name,"arguments":[
        {"runtime_type_id":a,"type":type_metadata(view,a)} for a in args]}

def constructor_metadata(view,index,row):
    fields=selected(view["ctor_fields"],row["field_start"],row["field_len"])
    owner=row["owning_type"]
    if owner<4:
        name=BUILTIN_CTORS[(owner,row["source_order"])]
        # Builtin rows have placeholder name_id=0: these are ordinal labels,
        # not fictitious user source-field names. Raw fields remain in Source.
        names=["field"+str(i) for i in range(len(fields))]
    else:
        name=view["names"][row["name_id"]]
        names=[view["names"][f["name_id"]] for f in fields]
    return {"type_id":owner,"ctor_id":index,"name":name,"fields":names}

def port_positive(document):
    """Produce whole input identity + NGProgram + derived type/ctor metadata.

    Input/source authenticity, finite valid table references, acyclic generic
    type-argument expansion, and existing Port common-positive premises are
    external. Same-index identifiers are never equated across source documents.
    """
    raw=copy.deepcopy(document)
    normalized=source_numbers(document["program_view"])
    port=PositivePort(normalized)
    program=port.run()
    return {"schema":"sherlock-programview23-positive-reference-input/1",
        "source_document":raw,"normalized_program_view":normalized,
        "ng_program":program,"origins":port.origins,"unhandled":port.issues,
        "type_rows":[{"runtime_type_id":i,"type":type_metadata(normalized,i)}
                     for i in range(len(normalized["types"]))],
        "ctor_rows":[constructor_metadata(normalized,i,r)
                     for i,r in enumerate(normalized["constructors"])],
        "scope":"positive table identity and existing uncharged NG reference only"}

def source_roundtrip(ported):
    return copy.deepcopy(ported["source_document"])

def literal(sort,value):
    return {"sort":sort,"literal":value}

def node(sort,constructor,*arguments):
    return {"sort":sort,"constructor":constructor,"arguments":list(arguments)}

def linked(sort,empty,cons,rows,encode):
    result=node(sort,empty)
    for row in reversed(rows):
        result=node(sort,cons,*encode(row),result)
    return result

def json_ast(value):
    """Lossless DATA representation; distinguishes native Bool/Int/String/null."""
    if value is None:return node("NIJSON","niNull")
    if type(value) is bool:return node("NIJSON","niBool",literal("Bool",value))
    if type(value) is int:return node("NIJSON","niInt",literal("Int",value))
    if type(value) is str:return node("NIJSON","niString",literal("String",value))
    if type(value) is list:
        return node("NIJSON","niArray",linked("NIJItems","niNoItems","niItem",value,lambda v:[json_ast(v)]))
    return node("NIJSON","niObject",linked("NIJFields","niNoFields","niField",list(value.items()),
        lambda kv:[literal("String",kv[0]),json_ast(kv[1])]))

def json_from_ast(term):
    """Inverse on json_ast's positive image only; NOT a K-output parser/guard."""
    name=term["constructor"];args=term["arguments"]
    if name=="niNull":return None
    if name in ("niBool","niInt","niString"):return args[0]["literal"]
    chain=args[0]; rows=[]
    if name=="niArray":
        while chain["constructor"]!="niNoItems":
            head,chain=chain["arguments"];rows.append(json_from_ast(head))
        return rows
    result={}
    while chain["constructor"]!="niNoFields":
        key,value,chain=chain["arguments"];result[key["literal"]]=json_from_ast(value)
    return result

def int_list(rows,sort,empty,cons):
    return linked(sort,empty,cons,rows,lambda x:[literal("Int",x)])

def value_ast(value):
    tag,payload=next(iter(value.items()))
    if tag=="UInt":return node("NGValue","ngUInt",literal("Int",payload))
    if tag=="Bool":return node("NGValue","ngBool",literal("Bool",payload))
    return node("NGValue","ngNominal",literal("Int",payload["type_id"]),literal("Int",payload["ctor_id"]),
        linked("NGValues","ngNoValues","ngValue",payload["fields"],lambda v:[value_ast(v)]))

def pattern_ast(value):
    tag,p=next(iter(value.items()))
    if tag=="Wildcard":return node("NGPattern","ngWildcard")
    if tag=="Bind":return node("NGPattern","ngBind",literal("Int",p))
    return node("NGPattern","ngConstructorPattern",literal("Int",p["type_id"]),literal("Int",p["ctor_id"]),
        linked("NGPatterns","ngNoPatterns","ngPattern",p["fields"],lambda x:[pattern_ast(x)]))

def expression_ast(row):
    tag,p=next(iter(row["kind"].items()));args=[literal("Int",row["occurrence"])]
    if tag=="Literal":return node("NGExpr","ngLiteral",*args,value_ast(p))
    names={"Variable":("ngVariable",("name_id",)),"Not":("ngNot",("operand",)),
        "Add":("ngAdd",("left","right")),"Construct":("ngConstruct",("type_id","ctor_id","fields")),
        "Call":("ngCall",("name_id","arguments")),"Match":("ngMatch",("scrutinee","arms")),
        "Let":("ngLet",("binder","bound_expr","body_expr")),"If":("ngIf",("condition_expr","yes_expr","no_expr"))}
    name,keys=names[tag]
    for k in keys:
        if k in ("fields","arguments"):
            args.append(int_list(p[k],"NGExprIds","ngNoExprs","ngExprId"))
        elif k=="arms":
            args.append(linked("NGArms","ngNoArms","ngArm",p[k],lambda a:[
                literal("Int",a["source_arm_id"]),pattern_ast(a["pattern"]),literal("Int",a["body"])]))
        else:args.append(literal("Int",p[k]))
    return node("NGExpr",name,*args)

def program_ast(p):
    def ctor(c):
        return node("NGCtor","ngCtor",literal("Int",c["type_id"]),literal("Int",c["ctor_id"]),
            literal("Int",c["source_order"]),int_list(c["field_types"],"NGTypes","ngNoTypes","ngType"))
    def decl(d):return [literal("Int",d["type_id"]),literal("Int",d["recursive_group"]),
        linked("NGCtors","ngNoCtors","ngCtorRow",d["ctors"],lambda c:[ctor(c)])]
    def function(f):return [node("NGFunction","ngFunction",literal("Int",f["name_id"]),
        linked("NGParams","ngNoParams","ngParam",f["parameters"],lambda x:[literal("Int",x["name_id"]),literal("Int",x["type_id"])]),
        literal("Int",f["return_type_id"]),literal("Int",f["body"]))]
    return node("NGProgram","ngProgram",linked("NGDecls","ngNoDecls","ngDecl",p["declarations"],decl),
        linked("NGNodes","ngNoNodes","ngNode",p["expressions"],lambda e:[literal("Int",e["id"]),expression_ast(e)]),
        linked("NGFunctions","ngNoFunctions","ngFunctionRow",p["functions"],function),literal("Int",p["entry"]))

def np_type_ast(t):
    if t["kind"]=="UInt":return node("NPType","npUIntType")
    if t["kind"]=="Bool":return node("NPType","npBoolType")
    return node("NPType","npNominalType",literal("String",t["name"]),
        linked("NPTypes","npNoTypes","npType",t["arguments"],lambda a:[np_type_ast(a["type"])]))

def query_ast(ported,observation_cut):
    """Closed constructor-name AST. Actual compiled labels bind AFTER compile.

    Complete Source is an explicit carried argument. niRun calls the existing
    npRunReference; this function only constructs DATA and evaluates no steps.
    """
    ts=linked("NPTypeRows","npNoTypeRows","npTypeRow",ported["type_rows"],lambda t:[
        literal("Int",t["runtime_type_id"]),np_type_ast(t["type"])])
    cs=linked("NPCtorRows","npNoCtorRows","npCtorRow",ported["ctor_rows"],lambda c:[
        literal("Int",c["type_id"]),literal("Int",c["ctor_id"]),literal("String",c["name"]),
        linked("NPFieldNames","npNoFieldNames","npFieldName",c["fields"],lambda f:[literal("String",f)])])
    return node("NIObservation","niRun",json_ast(ported["source_document"]),program_ast(ported["ng_program"]),
        ts,cs,literal("Int",observation_cut))
