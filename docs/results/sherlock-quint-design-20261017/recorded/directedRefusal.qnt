module directedRefusal {
 import interpreterChunks as C from "./interpreterChunks"
 import aExpectations as E from "./aExpectations"
 var pc:int
 action init=all{C::initCase(E::returnZeroDone),pc'=0}
 action step=any{all{pc==0,C::cut(0),pc'=1},all{pc==1,C::projectBoundary,pc'=2}}
 val allInvariant=C::allInvariant and pc>=0 and pc<=2
 val projectionRefused=C::s.projection==C::ProjectionCapacityRefused
 val directedFinished=pc==2
}
