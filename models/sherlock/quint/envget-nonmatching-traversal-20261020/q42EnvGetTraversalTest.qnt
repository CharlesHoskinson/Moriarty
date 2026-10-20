module q42EnvGetTraversalTest {
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import q42EnvGetTraversalTypes.* from "./q42EnvGetTraversalTypes"
  import q42EnvGetTraversalRelation.* from "./q42EnvGetTraversalRelation"
  import q42EnvGetTraversalInputs.* from "./q42EnvGetTraversalInputs"
  import q42EnvGetTraversalManualExpected.* from "./q42EnvGetTraversalManualExpected"
  // Exact whole traces, not charge-only or endpoint-only comparisons.
  run q42PointerTraversalCompleteHostsAndEventsTest = and {
    q42ComputeTrace(q42OneHopBefore,3) == q42OneExpected,
    q42ComputeTrace(q42TwoHopBefore,4) == q42TwoExpected,
    q42TraceOwnerLaws(q42OneExpected),q42TraceOwnerLaws(q42TwoExpected),
  }
  run q42TraversalPrefixesAndFiveKeyWindowTest = and {
    q42ComputeTrace(q42OneHopBefore,1) == {
      hosts:List(q42OneHopBefore,q42OneAfter1),events:List(q42OneExpected.events.nth(0)),
      finished:false,within_slice:true},
    q42ComputeTrace(q42OneHopBefore,2) == {
      hosts:List(q42OneHopBefore,q42OneAfter1,q42OneAfter2),
      events:q42OneExpected.events.slice(0,2),finished:false,within_slice:true},
    q42ComputeTrace(q42TwoHopBefore,2) == {
      hosts:List(q42TwoHopBefore,q42TwoAfter1,q42TwoAfter2),
      events:q42TwoExpected.events.slice(0,2),finished:false,within_slice:true},
    q42ObserveMissHitExit(q42OneHopBefore) == Q42CompleteWindow({
      before:q42OneHopBefore,after_miss:q42OneAfter1,after_hit:q42OneAfter2,
      after_exit:q42OneAfter3,events:q42OneExpected.events}),
    q42ComputeTrace(q42OneHopBefore,5) == q42OneExpected,
    q42ComputeTrace(q42TwoHopBefore,6) == q42TwoExpected,
  }
}
