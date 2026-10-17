module directedB {
 import helperChunks as C from "./helperChunks"
 var pc:int
 action init=all{C::initCase(5),pc'=0}
 action step=any{
  all{pc==0,C::helperEvent,pc'=1},all{pc==1,C::helperEvent,pc'=2},
  all{pc==2,C::cut,pc'=3},all{pc==3,C::resume,pc'=4},
  all{pc==4,C::helperEvent,pc'=5},all{pc==5,C::helperEvent,pc'=6},
  all{pc==6,C::cut,pc'=7},all{pc==7,C::resume,pc'=8},
  all{pc==8,C::helperEvent,pc'=9},all{pc==9,C::helperEvent,pc'=10},
  all{pc==10,C::probeNoNext,pc'=11},
 }
 val allInvariant=C::allInvariant and pc>=0 and pc<=11
 val helperCut=pc==3
 val pendingResultCut=pc==7
 val completed=pc==9
 val published=pc==10
 val noNext=pc==11
 val directedFinished=pc==11
}
