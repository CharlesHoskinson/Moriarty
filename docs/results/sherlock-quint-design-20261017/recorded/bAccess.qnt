module bAccess {
 import bPure.* from "./bPure"
 // Pure observation adapter uses locally imported variant names. Qualified
 // patterns caused a recorded QNT000 unification failure in the consumer.
 pure def eventList(ss:State):List[Event]=match ss.lastEvent {
  | StepEvent(event)=>List(event)
  | NoEvent=>List()
 }
}
