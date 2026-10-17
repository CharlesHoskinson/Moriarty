module bEventExpectations {
 import bPure.* from "./bPure"
 import bExpectations as E from "./bExpectations"
 // Manual event-field derivation from the fixed component contract. Not subject
 // eventOf/advance output; new literal addendum requiring nonauthor review.
 pure val limits5:Limits={initialFuel:5,maxHelperWork:8,environmentCells:2,helperTasks:1}
 pure val initial5:State={input:E::lookupInput,limits:limits5,machine:E::initialMachine5,lastEvent:NoEvent}
 pure val event1:Event={ruleId:"b.helper.EnvGet.entry",eventKind:HelperEnter,disposition:Applied,workDelta:1,helperWorkDelta:1,helperVisitsDelta:0,referenceCursorDelta:0,environmentCellsDelta:0,taskCursorBefore:0,taskCursorAfter:0,helperCursorBefore:Absent,helperCursorAfter:At(1),helperPhaseBefore:"absent",helperPhaseAfter:"Pending",originReferenceCursor:Absent}
 pure val state1:State={input:E::lookupInput,limits:limits5,machine:E::afterEntry5,lastEvent:StepEvent(event1)}
 pure val event2:Event={ruleId:"b.helper.EnvGet.cell",eventKind:HelperVisit,disposition:Applied,workDelta:1,helperWorkDelta:1,helperVisitsDelta:1,referenceCursorDelta:0,environmentCellsDelta:0,taskCursorBefore:0,taskCursorAfter:0,helperCursorBefore:At(1),helperCursorAfter:At(0),helperPhaseBefore:"Pending",helperPhaseAfter:"Pending",originReferenceCursor:At(0)}
 pure val state2:State={input:E::lookupInput,limits:limits5,machine:E::afterFirstCell5,lastEvent:StepEvent(event2)}
 pure val event3:Event={ruleId:"b.helper.EnvGet.cell",eventKind:HelperVisit,disposition:Applied,workDelta:1,helperWorkDelta:1,helperVisitsDelta:1,referenceCursorDelta:0,environmentCellsDelta:0,taskCursorBefore:0,taskCursorAfter:0,helperCursorBefore:At(0),helperCursorAfter:At(0),helperPhaseBefore:"Pending",helperPhaseAfter:"Positive",originReferenceCursor:At(0)}
 pure val state3:State={input:E::lookupInput,limits:limits5,machine:E::afterFound5,lastEvent:StepEvent(event3)}
 pure val event4:Event={ruleId:"b.helper.EnvGet.exit",eventKind:HelperExit,disposition:Applied,workDelta:1,helperWorkDelta:1,helperVisitsDelta:0,referenceCursorDelta:0,environmentCellsDelta:0,taskCursorBefore:0,taskCursorAfter:0,helperCursorBefore:At(0),helperCursorAfter:Absent,helperPhaseBefore:"Positive",helperPhaseAfter:"absent",originReferenceCursor:At(0)}
 pure val state4:State={input:E::lookupInput,limits:limits5,machine:E::afterExit5,lastEvent:StepEvent(event4)}
 pure val event5:Event={ruleId:"k.machine.eval.variable.present",eventKind:LegacyComplete,disposition:Applied,workDelta:1,helperWorkDelta:0,helperVisitsDelta:0,referenceCursorDelta:1,environmentCellsDelta:0,taskCursorBefore:0,taskCursorAfter:0,helperCursorBefore:Absent,helperCursorAfter:Absent,helperPhaseBefore:"absent",helperPhaseAfter:"absent",originReferenceCursor:Absent}
 pure val state5:State={input:E::lookupInput,limits:limits5,machine:E::afterComplete5,lastEvent:StepEvent(event5)}
 pure val event6:Event={ruleId:"k.admin.publish.value",eventKind:Publish,disposition:Published,workDelta:0,helperWorkDelta:0,helperVisitsDelta:0,referenceCursorDelta:0,environmentCellsDelta:0,taskCursorBefore:0,taskCursorAfter:0,helperCursorBefore:Absent,helperCursorAfter:Absent,helperPhaseBefore:"absent",helperPhaseAfter:"absent",originReferenceCursor:Absent}
 pure val state6:State={input:E::lookupInput,limits:limits5,machine:E::afterPublish5,lastEvent:StepEvent(event6)}
 pure val states5:List[State]=List(initial5,state1,state2,state3,state4,state5,state6)
}
