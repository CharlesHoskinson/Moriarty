module c6PositiveProgramDecoding {
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  // Positive complete images only: no malformed/UTF8/refusal/cap validator.
  type C6PReader = {cursor:int,text_spans:List[C6PTextSpan]}
  type C6PTextRead = {value:List[int],reader:C6PReader}
  type C6PTypeRead = {value:List[C6PTypeToken],reader:C6PReader}
  type C6PTypeWork = {value:List[C6PTypeToken],reader:C6PReader,pending:int}
  type C6PFieldRead = {value:C6PDeclField,reader:C6PReader}
  type C6PFieldsWork = {value:List[C6PDeclField],reader:C6PReader}
  type C6PCtorRead = {value:C6PCtorDecl,reader:C6PReader}
  type C6PCtorsWork = {value:List[C6PCtorDecl],reader:C6PReader}
  type C6PParamsWork = {value:List[List[int]],reader:C6PReader}
  type C6PDeclRead = {value:C6PDecl,reader:C6PReader}
  type C6PBuild = {program:C6PProgram,reader:C6PReader}
  type C6PRowsWork = {build:C6PBuild,spans:List[C6PRowSpan]}
  type C6PDecodeWork = {build:C6PBuild,table_counts:List[int],table_spans:List[C6PTableSpan]}
  pure val c6PDecoderTableNames:List[str] = List("expressions","occurrences","occurrence_paths","children","functions","parameters","arms","patterns","pattern_children","types","type_args","constructors","ctor_fields","decl_param_counts","decl_ctor_counts","decl_templates","plan_nodes","plan_cases","field_paths","constants","constant_fields","names")
  pure val c6PEmptyProgram:C6PProgram = {
    entry:0,expressions:List(),occurrences:List(),occurrence_paths:List(),children:List(),
    functions:List(),parameters:List(),arms:List(),patterns:List(),pattern_children:List(),
    types:List(),type_args:List(),constructors:List(),ctor_fields:List(),decl_param_counts:List(),
    decl_ctor_counts:List(),decl_templates:List(),plan_nodes:List(),plan_cases:List(),field_paths:List(),
    constants:List(),constant_fields:List(),names:List(),
  }
  pure def c6PReadSlice(lanesArg:List[int],atArg:int):C6PSlice =
    {start:lanesArg.nth(atArg),len:lanesArg.nth(atArg+1)}
  pure def c6PReadOption(lanesArg:List[int],atArg:int):C6POption =
    if (lanesArg.nth(atArg)==1) C6PSome(lanesArg.nth(atArg+1)) else C6PNone
  pure def c6PReadBinary(tagArg:int):C6PBinary =
    if(tagArg==0) C6PAdd else if(tagArg==1) C6PSub else if(tagArg==2) C6PMul
    else if(tagArg==3) C6PEq else if(tagArg==4) C6PLt else if(tagArg==5) C6PCons else C6PBranch
  pure def c6PReadUnary(tagArg:int):C6PUnary = if(tagArg==0) C6PLeaf else C6PNot

  pure def c6PReadText(lanesArg:List[int],readerArg:C6PReader,
      tableArg:str,rowArg:int,pathArg:List[C6PPathStep],budgetArg:str):C6PTextRead = {
    val lengthArg = lanesArg.nth(readerArg.cursor)
    val startArg = readerArg.cursor+1
    val endArg = startArg+lengthArg
    val bytesArg = range(0,lengthArg).foldl(List(),(valueArg,indexArg) => valueArg.append(lanesArg.nth(startArg+indexArg)))
    val spanArg:C6PTextSpan = {table:tableArg,row_index:rowArg,path:pathArg,budget:budgetArg,
      length_lane:readerArg.cursor,bytes_start:startArg,bytes_end:endArg,utf8_bytes:bytesArg}
    {value:bytesArg,reader:{cursor:endArg,text_spans:readerArg.text_spans.append(spanArg)}}
  }

  // One complete preorder tree. Pending counts unfinished nodes, not runtime work.
  // A token consumes >=1 lane, so the supplied positive stream length bounds this fold.
  pure def c6PReadType(lanesArg:List[int],readerArg:C6PReader,
      rowArg:int,pathArg:List[C6PPathStep]):C6PTypeRead = {
    val initialArg:C6PTypeWork = {value:List(),reader:readerArg,pending:1}
    val doneArg = range(0,lanesArg.length()).foldl(initialArg,(workArg,iterationArg) =>
      if(workArg.pending==0) workArg else {
        val tagArg = lanesArg.nth(workArg.reader.cursor)
        val afterTagArg:C6PReader = {...workArg.reader,cursor:workArg.reader.cursor+1}
        if(tagArg<4) {
          val tokenArg:C6PTypeToken = if(tagArg==0) C6PTUInt else if(tagArg==1) C6PTBool else if(tagArg==2) C6PTList else C6PTTree
          {value:workArg.value.append(tokenArg),reader:afterTagArg,pending:workArg.pending-1}
        } else {
          val nameArg = c6PReadText(lanesArg,afterTagArg,"decl_templates",rowArg,
            pathArg.concat(List(C6PIndexStep(workArg.value.length()),C6PFieldStep("name"))),"Template")
          val arityArg = if(tagArg==6) lanesArg.nth(nameArg.reader.cursor) else 0
          val tokenArg:C6PTypeToken = if(tagArg==4) C6PTNamed({name:nameArg.value})
            else if(tagArg==5) C6PTParam({name:nameArg.value}) else C6PTApply({name:nameArg.value,arity:arityArg})
          {value:workArg.value.append(tokenArg),
            reader:{...nameArg.reader,cursor:nameArg.reader.cursor+(if(tagArg==6) 1 else 0)},
            pending:workArg.pending-1+arityArg}
        }
      })
    {value:doneArg.value,reader:doneArg.reader}
  }
  pure def c6PReadDeclField(lanesArg:List[int],readerArg:C6PReader,
      rowArg:int,ctorArg:int,fieldArg:int):C6PFieldRead = {
    val baseArg = List(C6PFieldStep("ctors"),C6PIndexStep(ctorArg),C6PFieldStep("fields"),C6PIndexStep(fieldArg))
    val nameArg = c6PReadText(lanesArg,readerArg,"decl_templates",rowArg,baseArg.append(C6PFieldStep("name")),"Template")
    val typeArg = c6PReadType(lanesArg,nameArg.reader,rowArg,baseArg.append(C6PFieldStep("ty")))
    {value:{name:nameArg.value,ty:typeArg.value},reader:typeArg.reader}
  }
  pure def c6PReadCtorDecl(lanesArg:List[int],readerArg:C6PReader,rowArg:int,ctorArg:int):C6PCtorRead = {
    val nameArg = c6PReadText(lanesArg,readerArg,"decl_templates",rowArg,
      List(C6PFieldStep("ctors"),C6PIndexStep(ctorArg),C6PFieldStep("name")),"Template")
    val countArg = lanesArg.nth(nameArg.reader.cursor)
    val initialArg:C6PFieldsWork = {value:List(),reader:{...nameArg.reader,cursor:nameArg.reader.cursor+1}}
    val doneArg = range(0,countArg).foldl(initialArg,(workArg,indexArg) => {
      val fieldArg = c6PReadDeclField(lanesArg,workArg.reader,rowArg,ctorArg,indexArg)
      {value:workArg.value.append(fieldArg.value),reader:fieldArg.reader}
    })
    {value:{name:nameArg.value,fields:doneArg.value},reader:doneArg.reader}
  }
  pure def c6PReadDecl(lanesArg:List[int],readerArg:C6PReader,rowArg:int):C6PDeclRead = {
    val nameArg = c6PReadText(lanesArg,readerArg,"decl_templates",rowArg,List(C6PFieldStep("name")),"Template")
    val paramsCountArg = lanesArg.nth(nameArg.reader.cursor)
    val paramsInitialArg:C6PParamsWork = {value:List(),reader:{...nameArg.reader,cursor:nameArg.reader.cursor+1}}
    val paramsArg = range(0,paramsCountArg).foldl(paramsInitialArg,(workArg,indexArg) => {
      val textArg = c6PReadText(lanesArg,workArg.reader,"decl_templates",rowArg,
        List(C6PFieldStep("parameters"),C6PIndexStep(indexArg)),"Template")
      {value:workArg.value.append(textArg.value),reader:textArg.reader}
    })
    val ctorsCountArg = lanesArg.nth(paramsArg.reader.cursor)
    val ctorsInitialArg:C6PCtorsWork = {value:List(),reader:{...paramsArg.reader,cursor:paramsArg.reader.cursor+1}}
    val ctorsArg = range(0,ctorsCountArg).foldl(ctorsInitialArg,(workArg,indexArg) => {
      val ctorArg = c6PReadCtorDecl(lanesArg,workArg.reader,rowArg,indexArg)
      {value:workArg.value.append(ctorArg.value),reader:ctorArg.reader}
    })
    {value:{name:nameArg.value,parameters:paramsArg.value,ctors:ctorsArg.value},reader:ctorsArg.reader}
  }

  pure def c6PReadExpression(lanesArg:List[int],atArg:int):C6PExpr = {
    val tagArg=lanesArg.nth(atArg)
    if(tagArg==0) C6PELiteral({constant:lanesArg.nth(atArg+1)})
    else if(tagArg==1) C6PEVar({binder:lanesArg.nth(atArg+1)})
    else if(tagArg==2) C6PELet({binder:lanesArg.nth(atArg+1),value:lanesArg.nth(atArg+2),body:lanesArg.nth(atArg+3)})
    else if(tagArg==3) C6PEBinary({op:c6PReadBinary(lanesArg.nth(atArg+1)),left:lanesArg.nth(atArg+2),right:lanesArg.nth(atArg+3)})
    else if(tagArg==4) C6PEUnary({op:c6PReadUnary(lanesArg.nth(atArg+1)),operand:lanesArg.nth(atArg+2)})
    else if(tagArg==5) C6PEIf({cond:lanesArg.nth(atArg+1),then_expr:lanesArg.nth(atArg+2),else_expr:lanesArg.nth(atArg+3)})
    else if(tagArg==6) C6PECall({function_name:lanesArg.nth(atArg+1),args:c6PReadSlice(lanesArg,atArg+2)})
    else if(tagArg==7) C6PEMatch({scrutinee:lanesArg.nth(atArg+1),arms:c6PReadSlice(lanesArg,atArg+2),plan:c6PReadOption(lanesArg,atArg+4)})
    else C6PEConstruct({type_id:lanesArg.nth(atArg+1),ctor_id:lanesArg.nth(atArg+2),fields:c6PReadSlice(lanesArg,atArg+3),constant:c6PReadOption(lanesArg,atArg+5)})
  }
  pure def c6PExpressionWidth(tagArg:int):int =
    if(tagArg==0 or tagArg==1) 2 else if(tagArg==4) 3 else if(tagArg==7) 6 else if(tagArg==8) 7 else 4
  pure def c6PReadPattern(lanesArg:List[int],atArg:int):C6PPattern = {
    val tagArg=lanesArg.nth(atArg)
    if(tagArg==0) C6PPVar({x:lanesArg.nth(atArg+1)}) else if(tagArg==1) C6PPNil
    else if(tagArg==2) C6PPCons({x:lanesArg.nth(atArg+1),y:lanesArg.nth(atArg+2)})
    else if(tagArg==3) C6PPLeaf({x:lanesArg.nth(atArg+1)})
    else if(tagArg==4) C6PPBranch({x:lanesArg.nth(atArg+1),y:lanesArg.nth(atArg+2)})
    else if(tagArg==5) C6PWildcard
    else C6PPConstructor({type_id:lanesArg.nth(atArg+1),ctor_id:lanesArg.nth(atArg+2),children:c6PReadSlice(lanesArg,atArg+3)})
  }
  pure def c6PPatternWidth(tagArg:int):int =
    if(tagArg==1 or tagArg==5) 1 else if(tagArg==0 or tagArg==3) 2 else if(tagArg==6) 5 else 3
  pure def c6PReadPlan(lanesArg:List[int],atArg:int):C6PPlan = {
    val tagArg=lanesArg.nth(atArg)
    if(tagArg==0) C6PSwitch({source_match:lanesArg.nth(atArg+1),path:c6PReadSlice(lanesArg,atArg+2),type_id:lanesArg.nth(atArg+4),cases:c6PReadSlice(lanesArg,atArg+5),default:lanesArg.nth(atArg+7)})
    else if(tagArg==1) C6PBind({source_match:lanesArg.nth(atArg+1),path:c6PReadSlice(lanesArg,atArg+2),binder:lanesArg.nth(atArg+4),next:lanesArg.nth(atArg+5)})
    else if(tagArg==2) C6PSelect({source_match:lanesArg.nth(atArg+1),arm:lanesArg.nth(atArg+2)})
    else C6PNoMatch({source_match:lanesArg.nth(atArg+1)})
  }
  pure def c6PPlanWidth(tagArg:int):int = if(tagArg==0) 8 else if(tagArg==1) 6 else if(tagArg==2) 3 else 2
  pure def c6PReadConstant(lanesArg:List[int],atArg:int):C6PConstant = {
    val tagArg=lanesArg.nth(atArg)
    val markerArg=lanesArg.nth(atArg+1)==1
    if(tagArg==0) C6PKUInt({marker_free:markerArg,n:lanesArg.nth(atArg+2)})
    else if(tagArg==1) C6PKBool({marker_free:markerArg,b:lanesArg.nth(atArg+2)==1})
    else if(tagArg==2) C6PKNil({marker_free:markerArg})
    else if(tagArg==3) C6PKCons({marker_free:markerArg,fields:c6PReadSlice(lanesArg,atArg+2)})
    else if(tagArg==4) C6PKLeaf({marker_free:markerArg,fields:c6PReadSlice(lanesArg,atArg+2)})
    else if(tagArg==5) C6PKBranch({marker_free:markerArg,fields:c6PReadSlice(lanesArg,atArg+2)})
    else if(tagArg==6) C6PKConstructor({marker_free:markerArg,type_id:lanesArg.nth(atArg+2),ctor_id:lanesArg.nth(atArg+3),fields:c6PReadSlice(lanesArg,atArg+4)})
    else C6PKOutOfRange({marker_free:markerArg})
  }
  pure def c6PConstantWidth(tagArg:int):int = if(tagArg==0 or tagArg==1) 3 else if(tagArg==2 or tagArg==7) 2 else if(tagArg==6) 6 else 4

  // One row, selected by the fixed public22-table grammar, never by fixture identity.
  pure def c6PReadOneRow(lanesArg:List[int],buildArg:C6PBuild,tableArg:int,indexArg:int):C6PBuild = {
    val atArg=buildArg.reader.cursor
    if(tableArg==0) {
      program:{...buildArg.program,expressions:buildArg.program.expressions.append(c6PReadExpression(lanesArg,atArg))},
      reader:{...buildArg.reader,cursor:atArg+c6PExpressionWidth(lanesArg.nth(atArg))},
    } else if(tableArg==1) {
      program:{...buildArg.program,occurrences:buildArg.program.occurrences.append({owner_is_function:lanesArg.nth(atArg)==1,function:lanesArg.nth(atArg+1),path:c6PReadSlice(lanesArg,atArg+2)})},
      reader:{...buildArg.reader,cursor:atArg+4},
    } else if(tableArg==2) {
      program:{...buildArg.program,occurrence_paths:buildArg.program.occurrence_paths.append(lanesArg.nth(atArg))},
      reader:{...buildArg.reader,cursor:atArg+1},
    } else if(tableArg==3) {
      program:{...buildArg.program,children:buildArg.program.children.append(lanesArg.nth(atArg))},
      reader:{...buildArg.reader,cursor:atArg+1},
    } else if(tableArg==4) {
      program:{...buildArg.program,functions:buildArg.program.functions.append({function_name:lanesArg.nth(atArg),params:c6PReadSlice(lanesArg,atArg+1),return_type:lanesArg.nth(atArg+3),body:lanesArg.nth(atArg+4)})},
      reader:{...buildArg.reader,cursor:atArg+5},
    } else if(tableArg==5) {
      program:{...buildArg.program,parameters:buildArg.program.parameters.append({binder:lanesArg.nth(atArg),sort:lanesArg.nth(atArg+1)})},
      reader:{...buildArg.reader,cursor:atArg+2},
    } else if(tableArg==6) {
      program:{...buildArg.program,arms:buildArg.program.arms.append({pattern:lanesArg.nth(atArg),body:lanesArg.nth(atArg+1)})},
      reader:{...buildArg.reader,cursor:atArg+2},
    } else if(tableArg==7) {
      program:{...buildArg.program,patterns:buildArg.program.patterns.append(c6PReadPattern(lanesArg,atArg))},
      reader:{...buildArg.reader,cursor:atArg+c6PPatternWidth(lanesArg.nth(atArg))},
    } else if(tableArg==8) {
      program:{...buildArg.program,pattern_children:buildArg.program.pattern_children.append(lanesArg.nth(atArg))},
      reader:{...buildArg.reader,cursor:atArg+1},
    } else if(tableArg==9) {
      program:{...buildArg.program,types:buildArg.program.types.append({name_id:lanesArg.nth(atArg),decl:c6PReadOption(lanesArg,atArg+1),args:c6PReadSlice(lanesArg,atArg+3),ctor_start:lanesArg.nth(atArg+5),ctor_len:lanesArg.nth(atArg+6)})},
      reader:{...buildArg.reader,cursor:atArg+7},
    } else if(tableArg==10) {
      program:{...buildArg.program,type_args:buildArg.program.type_args.append(lanesArg.nth(atArg))},
      reader:{...buildArg.reader,cursor:atArg+1},
    } else if(tableArg==11) {
      program:{...buildArg.program,constructors:buildArg.program.constructors.append({owning_type:lanesArg.nth(atArg),name_id:lanesArg.nth(atArg+1),source_order:lanesArg.nth(atArg+2),field_start:lanesArg.nth(atArg+3),field_len:lanesArg.nth(atArg+4)})},
      reader:{...buildArg.reader,cursor:atArg+5},
    } else if(tableArg==12) {
      program:{...buildArg.program,ctor_fields:buildArg.program.ctor_fields.append({name_id:lanesArg.nth(atArg),type_id:lanesArg.nth(atArg+1)})},
      reader:{...buildArg.reader,cursor:atArg+2},
    } else if(tableArg==13) {
      program:{...buildArg.program,decl_param_counts:buildArg.program.decl_param_counts.append(lanesArg.nth(atArg))},
      reader:{...buildArg.reader,cursor:atArg+1},
    } else if(tableArg==14) {
      program:{...buildArg.program,decl_ctor_counts:buildArg.program.decl_ctor_counts.append(lanesArg.nth(atArg))},
      reader:{...buildArg.reader,cursor:atArg+1},
    } else if(tableArg==15) {
      val rowArg=c6PReadDecl(lanesArg,buildArg.reader,indexArg)
      {program:{...buildArg.program,decl_templates:buildArg.program.decl_templates.append(rowArg.value)},reader:rowArg.reader}
    } else if(tableArg==16) {
      program:{...buildArg.program,plan_nodes:buildArg.program.plan_nodes.append(c6PReadPlan(lanesArg,atArg))},
      reader:{...buildArg.reader,cursor:atArg+c6PPlanWidth(lanesArg.nth(atArg))},
    } else if(tableArg==17) {
      program:{...buildArg.program,plan_cases:buildArg.program.plan_cases.append(c6PReadOption(lanesArg,atArg))},
      reader:{...buildArg.reader,cursor:atArg+2},
    } else if(tableArg==18) {
      program:{...buildArg.program,field_paths:buildArg.program.field_paths.append(lanesArg.nth(atArg))},
      reader:{...buildArg.reader,cursor:atArg+1},
    } else if(tableArg==19) {
      program:{...buildArg.program,constants:buildArg.program.constants.append(c6PReadConstant(lanesArg,atArg))},
      reader:{...buildArg.reader,cursor:atArg+c6PConstantWidth(lanesArg.nth(atArg))},
    } else if(tableArg==20) {
      program:{...buildArg.program,constant_fields:buildArg.program.constant_fields.append(lanesArg.nth(atArg))},
      reader:{...buildArg.reader,cursor:atArg+1},
    } else {
      val rowArg=c6PReadText(lanesArg,buildArg.reader,"names",indexArg,List(),"Names")
      {program:{...buildArg.program,names:buildArg.program.names.append(rowArg.value)},reader:rowArg.reader}
    }
  }

  pure def c6PReadTable(lanesArg:List[int],workArg:C6PDecodeWork,tableArg:int):C6PDecodeWork = {
    val atArg=workArg.build.reader.cursor
    val countArg=lanesArg.nth(atArg)
    val initialArg:C6PRowsWork={
      build:{...workArg.build,reader:{...workArg.build.reader,cursor:atArg+1}},spans:List()}
    val rowsArg=range(0,countArg).foldl(initialArg,(rowsArg,indexArg) => {
      val nextArg=c6PReadOneRow(lanesArg,rowsArg.build,tableArg,indexArg)
      val spanArg:C6PRowSpan={row_index:indexArg,start:rowsArg.build.reader.cursor,end:nextArg.reader.cursor}
      {build:nextArg,spans:rowsArg.spans.append(spanArg)}
    })
    val tableSpanArg:C6PTableSpan={table:c6PDecoderTableNames.nth(tableArg),count_lane:atArg,
      row_count:countArg,rows_start:atArg+1,rows_end:rowsArg.build.reader.cursor,row_spans:rowsArg.spans}
    {build:rowsArg.build,table_counts:workArg.table_counts.append(countArg),
      table_spans:workArg.table_spans.append(tableSpanArg)}
  }
  pure def c6PDecodeProgramPositive(lanesArg:List[int]):C6PDecodeWork = {
    val initialArg:C6PDecodeWork={
      build:{program:{...c6PEmptyProgram,entry:lanesArg.nth(3)},reader:{cursor:4,text_spans:List()}},
      table_counts:List(),table_spans:List()}
    range(0,22).foldl(initialArg,(workArg,tableArg) => c6PReadTable(lanesArg,workArg,tableArg))
  }
}
