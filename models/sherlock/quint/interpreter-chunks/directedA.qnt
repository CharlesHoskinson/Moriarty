module directedA {
 import interpreterChunks as C from "./interpreterChunks"
 import aExpectations as E from "./aExpectations"
 var pc:int
 action init=all{C::initCase(E::n01State0),pc'=0}
 action step=any{
  all{pc==0,C::group(1,false),pc'=1},
  all{pc==1,C::cut(1),pc'=2},
  all{pc==2,C::projectBoundary,pc'=3},
  all{pc==3,C::resume,pc'=4},
  all{pc==4,C::startCall(1,false),pc'=5},
  all{pc==5,C::observeEvent,pc'=6},
  all{pc==6,C::startCall(1,false),pc'=7},
  all{pc==7,C::probeNoNext,pc'=8},
  all{pc==8,C::startCall(2,true),pc'=9},
  all{pc==9,C::probeNoNext,pc'=10},
  all{pc==10,C::cut(1),pc'=11},
  all{pc==11,C::projectBoundary,pc'=12},
 }
 val allInvariant=C::allInvariant and pc>=0 and pc<=12
 val pendingCut=pc==2
 val resumed=pc==4
 val terminalEvent=pc==6
 val repeatedNoNext=pc==10
 val scalarProjected=C::s.projection==C::ScalarProjected
 val directedFinished=pc==12
}
