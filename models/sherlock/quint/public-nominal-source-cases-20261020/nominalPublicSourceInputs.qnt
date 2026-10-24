module nominalPublicSourceInputs {
  import nominalPublicSimulation.* from "./nominalPublicSimulation"
  // Exact declaration field order, deliberately z_* before a_*.
  // These labels/type-argument lists are public binding metadata; evaluator
  // uses these same monomorphic IDs/layout order, not lexical field sorting.
  type Q10TypeBinding = {typeId:int,name:str,args:List[str]}
  pure val q10TypeBindings:List[Q10TypeBinding] = List(
    {typeId:10,name:"Queue",args:List("UInt")},
    {typeId:20,name:"Envelope",args:List("Queue<UInt>","Bool")},
    {typeId:30,name:"Arbor",args:List("Bool")},
    {typeId:40,name:"Stamp",args:List("Arbor<Bool>","UInt")})
  pure val q10FieldNames:(int,int) -> List[str] = Map(
    (10,0)->List(),(10,1)->List("z_item","a_tail"),
    (20,0)->List("z_items","a_marker"),(30,0)->List("z_value"),
    (30,1)->List("z_left","a_right"),(40,0)->List("z_tree","a_code"))
  pure val q10MapProgram:Program = {
    entry:0,
    layouts:Map((10,0)->{typeId:10,ctorId:0,fields:List()},
      (10,1)->{typeId:10,ctorId:1,fields:List(0,10)},
      (20,0)->{typeId:20,ctorId:0,fields:List(10,1)}),
    functions:Map(
      0->{name:0,parameters:List(100),parameterTypes:List(10),resultType:10,body:10},
      1->{name:1,parameters:List(200),parameterTypes:List(10),resultType:20,body:6}),
    expressions:Map(
      0->Call({function:1,arguments:List(1)}),
      1->Constructor({typeId:10,ctorId:1,occurrence:1,fields:List(2,3)}),
      2->Literal(List(UIntToken(3))),
      3->Constructor({typeId:10,ctorId:1,occurrence:3,fields:List(4,5)}),
      4->Literal(List(UIntToken(8))),
      5->Constructor({typeId:10,ctorId:0,occurrence:5,fields:List()}),
      6->Constructor({typeId:20,ctorId:0,occurrence:6,fields:List(7,8)}),
      7->Call({function:0,arguments:List(9)}),8->Literal(List(BoolToken(true))),
      9->Variable(200),
      10->Match({value:11,arms:List(
        {pattern:ConstructorPattern({typeId:10,ctorId:0,binders:List()}),body:12},
        {pattern:ConstructorPattern({typeId:10,ctorId:1,binders:List(101,102)}),body:13})}),
      11->Variable(100),
      12->Constructor({typeId:10,ctorId:0,occurrence:12,fields:List()}),
      13->Constructor({typeId:10,ctorId:1,occurrence:13,fields:List(14,15)}),
      14->Binary({op:Add,left:16,right:17}),15->Call({function:0,arguments:List(18)}),
      16->Variable(101),17->Literal(List(UIntToken(2))),18->Variable(102))}
  pure val q10MirrorProgram:Program = {
    entry:0,
    layouts:Map((30,0)->{typeId:30,ctorId:0,fields:List(1)},
      (30,1)->{typeId:30,ctorId:1,fields:List(30,30)},
      (40,0)->{typeId:40,ctorId:0,fields:List(30,0)}),
    functions:Map(
      0->{name:0,parameters:List(300),parameterTypes:List(30),resultType:30,body:13},
      1->{name:1,parameters:List(400),parameterTypes:List(30),resultType:40,body:9}),
    expressions:Map(
      0->Call({function:1,arguments:List(1)}),
      1->Constructor({typeId:30,ctorId:1,occurrence:1,fields:List(2,4)}),
      2->Constructor({typeId:30,ctorId:0,occurrence:2,fields:List(3)}),
      3->Literal(List(BoolToken(true))),
      4->Constructor({typeId:30,ctorId:1,occurrence:4,fields:List(5,7)}),
      5->Constructor({typeId:30,ctorId:0,occurrence:5,fields:List(6)}),
      6->Literal(List(BoolToken(false))),
      7->Constructor({typeId:30,ctorId:0,occurrence:7,fields:List(8)}),
      8->Literal(List(BoolToken(true))),
      9->Constructor({typeId:40,ctorId:0,occurrence:9,fields:List(10,11)}),
      10->Call({function:0,arguments:List(12)}),11->Literal(List(UIntToken(41))),
      12->Variable(400),
      13->Match({value:14,arms:List(
        {pattern:ConstructorPattern({typeId:30,ctorId:0,binders:List(301)}),body:15},
        {pattern:ConstructorPattern({typeId:30,ctorId:1,binders:List(302,303)}),body:18})}),
      14->Variable(300),
      15->Constructor({typeId:30,ctorId:0,occurrence:15,fields:List(16)}),
      16->BooleanNot(17),17->Variable(301),
      18->Constructor({typeId:30,ctorId:1,occurrence:18,fields:List(19,20)}),
      19->Call({function:0,arguments:List(21)}),20->Call({function:0,arguments:List(22)}),
      21->Variable(303),22->Variable(302))}
}
