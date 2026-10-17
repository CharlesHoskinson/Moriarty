module aPrefixExpectations {
 import aPure.* from "./aPure"

  // Normal-prefix transposition from preexisting independent literal states only.
  // Charged normal states have ordinal work; publication free. No subject call.
  pure def expectedPrefix(states:List[Snapshot],fuel:int):List[Snapshot]={
    val work=states.length()-2
    val charged=if(fuel<work)fuel else work
    val body=range(0,charged+1).foldl(List(),(acc,i)=>acc.append({...states.nth(i),remaining:fuel-i,initial:fuel}))
    val endState=if(fuel<work){...states.nth(charged),control:EmptyControl,remaining:0,initial:fuel,outcome:Exhausted} else {...states.nth(work+1),remaining:fuel-work,initial:fuel}
    body.append(endState)
  }
}
