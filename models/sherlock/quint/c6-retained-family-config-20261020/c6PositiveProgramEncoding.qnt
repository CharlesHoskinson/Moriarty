module c6PositiveProgramEncoding {
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  // Valid host-fit/UTF8/token-tree/cap premises are external, not guards here.
  pure val c6PMagic:int = 18918541878711089
  pure val c6PVersion:int = 1
  pure val c6PProgramTag:int = 1347571527
  pure val c6PTableNames:List[str] = List("expressions","occurrences","occurrence_paths","children","functions","parameters","arms","patterns","pattern_children","types","type_args","constructors","ctor_fields","decl_param_counts","decl_ctor_counts","decl_templates","plan_nodes","plan_cases","field_paths","constants","constant_fields","names")
  pure def c6PBoolLane(boolArg:bool):int = if (boolArg) 1 else 0
  pure def c6PSliceLanes(sliceArg:C6PSlice):List[int] = List(sliceArg.start,sliceArg.len)
  pure def c6POptionLanes(optionArg:C6POption):List[int] = match optionArg {
    | C6PNone => List(0,0)
    | C6PSome(indexArg) => List(1,indexArg)
  }
  pure def c6PBinaryTag(opArg:C6PBinary):int = match opArg {
    | C6PAdd => 0 | C6PSub => 1 | C6PMul => 2 | C6PEq => 3
    | C6PLt => 4 | C6PCons => 5 | C6PBranch => 6
  }
  pure def c6PUnaryTag(opArg:C6PUnary):int = match opArg {
    | C6PLeaf => 0 | C6PNot => 1
  }
  pure def c6PTextLanes(textArg:List[int]):List[int] = List(textArg.length()).concat(textArg)
  pure def c6PTypeTokenLanes(tokenArg:C6PTypeToken):List[int] = match tokenArg {
    | C6PTUInt => List(0) | C6PTBool => List(1)
    | C6PTList => List(2) | C6PTTree => List(3)
    | C6PTNamed(nodeArg) => List(4).concat(c6PTextLanes(nodeArg.name))
    | C6PTParam(nodeArg) => List(5).concat(c6PTextLanes(nodeArg.name))
    | C6PTApply(nodeArg) => List(6).concat(c6PTextLanes(nodeArg.name)).append(nodeArg.arity)
  }
  pure def c6PTypeLanes(tokensArg:List[C6PTypeToken]):List[int] =
    range(0,tokensArg.length()).foldl(List(),(lanesArg,indexArg) =>
      lanesArg.concat(c6PTypeTokenLanes(tokensArg.nth(indexArg))))
  pure def c6PDeclFieldLanes(fieldArg:C6PDeclField):List[int] =
    c6PTextLanes(fieldArg.name).concat(c6PTypeLanes(fieldArg.ty))
  pure def c6PCtorDeclLanes(ctorArg:C6PCtorDecl):List[int] =
    range(0,ctorArg.fields.length()).foldl(
      c6PTextLanes(ctorArg.name).append(ctorArg.fields.length()),
      (lanesArg,indexArg) => lanesArg.concat(c6PDeclFieldLanes(ctorArg.fields.nth(indexArg))))
  pure def c6PDeclLanes(declArg:C6PDecl):List[int] = {
    val paramsArg = range(0,declArg.parameters.length()).foldl(
      c6PTextLanes(declArg.name).append(declArg.parameters.length()),
      (lanesArg,indexArg) => lanesArg.concat(c6PTextLanes(declArg.parameters.nth(indexArg))))
    range(0,declArg.ctors.length()).foldl(paramsArg.append(declArg.ctors.length()),
      (lanesArg,indexArg) => lanesArg.concat(c6PCtorDeclLanes(declArg.ctors.nth(indexArg))))
  }
  pure def c6PExpressionLanes(exprArg:C6PExpr):List[int] = match exprArg {
    | C6PELiteral(rowArg) => List(0,rowArg.constant)
    | C6PEVar(rowArg) => List(1,rowArg.binder)
    | C6PELet(rowArg) => List(2,rowArg.binder,rowArg.value,rowArg.body)
    | C6PEBinary(rowArg) => List(3,c6PBinaryTag(rowArg.op),rowArg.left,rowArg.right)
    | C6PEUnary(rowArg) => List(4,c6PUnaryTag(rowArg.op),rowArg.operand)
    | C6PEIf(rowArg) => List(5,rowArg.cond,rowArg.then_expr,rowArg.else_expr)
    | C6PECall(rowArg) => List(6,rowArg.function_name).concat(c6PSliceLanes(rowArg.args))
    | C6PEMatch(rowArg) => List(7,rowArg.scrutinee).concat(c6PSliceLanes(rowArg.arms)).concat(c6POptionLanes(rowArg.plan))
    | C6PEConstruct(rowArg) => List(8,rowArg.type_id,rowArg.ctor_id).concat(c6PSliceLanes(rowArg.fields)).concat(c6POptionLanes(rowArg.constant))
  }
  pure def c6PPatternLanes(patternArg:C6PPattern):List[int] = match patternArg {
    | C6PPVar(rowArg) => List(0,rowArg.x) | C6PPNil => List(1)
    | C6PPCons(rowArg) => List(2,rowArg.x,rowArg.y)
    | C6PPLeaf(rowArg) => List(3,rowArg.x)
    | C6PPBranch(rowArg) => List(4,rowArg.x,rowArg.y)
    | C6PWildcard => List(5)
    | C6PPConstructor(rowArg) => List(6,rowArg.type_id,rowArg.ctor_id).concat(c6PSliceLanes(rowArg.children))
  }
  pure def c6PPlanLanes(planArg:C6PPlan):List[int] = match planArg {
    | C6PSwitch(rowArg) => List(0,rowArg.source_match).concat(c6PSliceLanes(rowArg.path)).append(rowArg.type_id).concat(c6PSliceLanes(rowArg.cases)).append(rowArg.default)
    | C6PBind(rowArg) => List(1,rowArg.source_match).concat(c6PSliceLanes(rowArg.path)).concat(List(rowArg.binder,rowArg.next))
    | C6PSelect(rowArg) => List(2,rowArg.source_match,rowArg.arm)
    | C6PNoMatch(rowArg) => List(3,rowArg.source_match)
  }
  pure def c6PConstantLanes(constantArg:C6PConstant):List[int] = match constantArg {
    | C6PKUInt(rowArg) => List(0,c6PBoolLane(rowArg.marker_free),rowArg.n)
    | C6PKBool(rowArg) => List(1,c6PBoolLane(rowArg.marker_free),c6PBoolLane(rowArg.b))
    | C6PKNil(rowArg) => List(2,c6PBoolLane(rowArg.marker_free))
    | C6PKCons(rowArg) => List(3,c6PBoolLane(rowArg.marker_free)).concat(c6PSliceLanes(rowArg.fields))
    | C6PKLeaf(rowArg) => List(4,c6PBoolLane(rowArg.marker_free)).concat(c6PSliceLanes(rowArg.fields))
    | C6PKBranch(rowArg) => List(5,c6PBoolLane(rowArg.marker_free)).concat(c6PSliceLanes(rowArg.fields))
    | C6PKConstructor(rowArg) => List(6,c6PBoolLane(rowArg.marker_free),rowArg.type_id,rowArg.ctor_id).concat(c6PSliceLanes(rowArg.fields))
    | C6PKOutOfRange(rowArg) => List(7,c6PBoolLane(rowArg.marker_free))
  }
  pure def c6PTableCounts(programArg:C6PProgram):List[int] = List(
    programArg.expressions.length(),programArg.occurrences.length(),programArg.occurrence_paths.length(),programArg.children.length(),programArg.functions.length(),programArg.parameters.length(),programArg.arms.length(),programArg.patterns.length(),programArg.pattern_children.length(),programArg.types.length(),programArg.type_args.length(),programArg.constructors.length(),programArg.ctor_fields.length(),programArg.decl_param_counts.length(),programArg.decl_ctor_counts.length(),programArg.decl_templates.length(),programArg.plan_nodes.length(),programArg.plan_cases.length(),programArg.field_paths.length(),programArg.constants.length(),programArg.constant_fields.length(),programArg.names.length())
  // Table number is the declared22-table grammar, never a fixture case selector.
  pure def c6PRowLanes(programArg:C6PProgram,tableArg:int,indexArg:int):List[int] =
    if (tableArg == 0) c6PExpressionLanes(programArg.expressions.nth(indexArg))
    else if (tableArg == 1) {
      val rowArg = programArg.occurrences.nth(indexArg)
      List(c6PBoolLane(rowArg.owner_is_function),rowArg.function).concat(c6PSliceLanes(rowArg.path))
    } else if (tableArg == 2) List(programArg.occurrence_paths.nth(indexArg))
    else if (tableArg == 3) List(programArg.children.nth(indexArg))
    else if (tableArg == 4) {
      val rowArg = programArg.functions.nth(indexArg)
      List(rowArg.function_name).concat(c6PSliceLanes(rowArg.params)).concat(List(rowArg.return_type,rowArg.body))
    } else if (tableArg == 5) {
      val rowArg = programArg.parameters.nth(indexArg)
      List(rowArg.binder,rowArg.sort)
    } else if (tableArg == 6) {
      val rowArg = programArg.arms.nth(indexArg)
      List(rowArg.pattern,rowArg.body)
    } else if (tableArg == 7) c6PPatternLanes(programArg.patterns.nth(indexArg))
    else if (tableArg == 8) List(programArg.pattern_children.nth(indexArg))
    else if (tableArg == 9) {
      val rowArg = programArg.types.nth(indexArg)
      List(rowArg.name_id).concat(c6POptionLanes(rowArg.decl)).concat(c6PSliceLanes(rowArg.args)).concat(List(rowArg.ctor_start,rowArg.ctor_len))
    } else if (tableArg == 10) List(programArg.type_args.nth(indexArg))
    else if (tableArg == 11) {
      val rowArg = programArg.constructors.nth(indexArg)
      List(rowArg.owning_type,rowArg.name_id,rowArg.source_order,rowArg.field_start,rowArg.field_len)
    } else if (tableArg == 12) {
      val rowArg = programArg.ctor_fields.nth(indexArg)
      List(rowArg.name_id,rowArg.type_id)
    } else if (tableArg == 13) List(programArg.decl_param_counts.nth(indexArg))
    else if (tableArg == 14) List(programArg.decl_ctor_counts.nth(indexArg))
    else if (tableArg == 15) c6PDeclLanes(programArg.decl_templates.nth(indexArg))
    else if (tableArg == 16) c6PPlanLanes(programArg.plan_nodes.nth(indexArg))
    else if (tableArg == 17) c6POptionLanes(programArg.plan_cases.nth(indexArg))
    else if (tableArg == 18) List(programArg.field_paths.nth(indexArg))
    else if (tableArg == 19) c6PConstantLanes(programArg.constants.nth(indexArg))
    else if (tableArg == 20) List(programArg.constant_fields.nth(indexArg))
    else c6PTextLanes(programArg.names.nth(indexArg))
  pure def c6PEncodeProgram(programArg:C6PProgram):List[int] =
    range(0,22).foldl(List(c6PMagic,c6PVersion,c6PProgramTag,programArg.entry),
      (lanesArg,tableArg) => {
        val countArg = c6PTableCounts(programArg).nth(tableArg)
        range(0,countArg).foldl(lanesArg.append(countArg),
          (rowsArg,indexArg) => rowsArg.concat(c6PRowLanes(programArg,tableArg,indexArg)))
      })
}
