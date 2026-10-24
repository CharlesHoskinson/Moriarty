module q11NominalIdentityTest {
  import q11NominalIdentity.* from "./q11NominalIdentity"
  import q11NominalIdentityTypes.* from "./q11NominalIdentityTypes"
  import q11NominalIdentityInputs.* from "./q11NominalIdentityInputs"
  import q11NominalIdentityExpected.* from "./q11NominalIdentityExpected"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import nominalPublicSimulation.* from "./nominalPublicSimulation"

  run q11CompleteInputBindingsTest = {
    val mapBindings=q11Bindings(q11MapView)
    val mirrorBindings=q11Bindings(q11MirrorView)
    and {
      mapBindings.types==q11MapExpectedTypes,
      mapBindings.constructors==q11MapExpectedCtors,
      mirrorBindings.types==q11MirrorExpectedTypes,
      mirrorBindings.constructors==q11MirrorExpectedCtors,
      q11DeclarationRelation(q11MapView),q11DeclarationRelation(q11MirrorView),
    }
  }
  run q11BothSourceReturnsUseRuntimeIndicesTest = {
    val mapRun=q11RunSource(q11MapView,256,256)
    val mirrorRun=q11RunSource(q11MirrorView,256,256)
    and {
      mapRun.outcome==Done(q11MapExpectedReturn),
      mirrorRun.outcome==Done(q11MirrorExpectedReturn),
      validValue(mapRun.program,q11MapExpectedReturn),
      validValue(mirrorRun.program,q11MirrorExpectedReturn),
      mapRun.environment==Map(),mapRun.frames==List(),
      mirrorRun.environment==Map(),mirrorRun.frames==List(),
      mapRun.helperWork==0,mirrorRun.helperWork==0,
    }
  }
  run q11SameTypedFieldOrderDistinguishesTest = {
    // Select the independently declared recursive binary constructor, not a
    // fixture-to-runtime ID replacement. Both field types are identical.
    val ctor=q11MirrorExpectedCtors.select(c=>and {
      c.raw.owning_type>=4,c.raw.field_len==2,c.raw.source_order==1}).head()
    val first=ctor.raw.field_start
    val original=q11MirrorView.ctor_fields
    val reordered=original.replaceAt(first,original.nth(first+1))
      .replaceAt(first+1,original.nth(first))
    val contrasting={...q11MirrorView,ctor_fields:reordered}
    and {
      original.nth(first).type_id==original.nth(first+1).type_id,
      reordered.length()==original.length(),
      q11DeclarationRelation(q11MirrorView),
      not(q11DeclarationRelation(contrasting)),
      q11Bindings(contrasting).retained_view==contrasting,
      q11Bindings(contrasting).constructors!=q11Bindings(q11MirrorView).constructors,
    }
  }
  run q11AllOrderedInputTablesAndOptionsRetainedTest = {
    val mapBindings=q11Bindings(q11MapView)
    val mirrorBindings=q11Bindings(q11MirrorView)
    val empty=q11MapExpectedCtors.select(c=>c.raw.owning_type>=4 and c.raw.field_len==0).head()
    and {
      mapBindings.retained_view==q11MapView,
      mirrorBindings.retained_view==q11MirrorView,
      q11MapView.types.nth(4).decl==C6PSome(0),
      q11MapView.types.nth(0).decl==C6PNone,
      C6PSome(0)!=C6PNone,
      empty.raw.field_start==5,empty.raw.field_len==0,
      mapBindings.constructors.nth(empty.ctor_id).raw==empty.raw,
      q11MapView.constants.length()==6,
      q11MapView.constants.nth(4)==q11MapView.constants.nth(5),
      mapBindings.retained_view.constants.length()==6,
      mapBindings.retained_view.occurrences==q11MapView.occurrences,
      mapBindings.retained_view.plan_cases==q11MapView.plan_cases,
      mapBindings.retained_view.decl_templates==q11MapView.decl_templates,
    }
  }
  run q11ConstantReferenceReaderPreservesSourceValuesTest = {
    val empty=q11MapExpectedCtors.select(c=>c.raw.owning_type>=4 and c.raw.field_len==0).head()
    val a=q11ReadConstant(q11MapView,4,64)
    val b=q11ReadConstant(q11MapView,5,64)
    val firstUInt=q11ReadConstant(q11MapView,0,64)
    val firstFalse=q11ReadConstant(q11MirrorView,1,64)
    and {
      a.pending==List(),b.pending==List(),
      a.tokens==List(ConstructorToken({typeId:empty.raw.owning_type,ctorId:empty.ctor_id,arity:0})),
      b.tokens==a.tokens,
      firstUInt.pending==List(),firstUInt.tokens==List(UIntToken(3)),
      firstFalse.pending==List(),firstFalse.tokens==List(BoolToken(false)),
      q11MapView.constant_fields==List(),
    }
  }
}
