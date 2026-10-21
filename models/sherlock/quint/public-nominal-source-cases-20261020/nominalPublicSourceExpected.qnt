module nominalPublicSourceExpected {
  import nominalPublicSimulation.* from "./nominalPublicSimulation"
  // DATA literals mechanically corresponding to pre-code manual typed trees.
  // IDs/type arguments are a declared finite projection, not canonical compiler IDs.
  pure val q10MapExpected:Value = List(
    ConstructorToken({typeId:20,ctorId:0,arity:2}),
    ConstructorToken({typeId:10,ctorId:1,arity:2}),UIntToken(5),
    ConstructorToken({typeId:10,ctorId:1,arity:2}),UIntToken(10),
    ConstructorToken({typeId:10,ctorId:0,arity:0}),BoolToken(true))
  pure val q10MirrorExpected:Value = List(
    ConstructorToken({typeId:40,ctorId:0,arity:2}),
    ConstructorToken({typeId:30,ctorId:1,arity:2}),
    ConstructorToken({typeId:30,ctorId:1,arity:2}),
    ConstructorToken({typeId:30,ctorId:0,arity:1}),BoolToken(false),
    ConstructorToken({typeId:30,ctorId:0,arity:1}),BoolToken(true),
    ConstructorToken({typeId:30,ctorId:0,arity:1}),BoolToken(false),UIntToken(41))
  pure val q10MapArms:List[int] = List(1,1,0)
  pure val q10MirrorArms:List[int] = List(1,1,0,0,0)
}
