# pipelonk PLONK/utils/zk_function.cu (ae branch)

> Source: https://raw.githubusercontent.com/zlyber/pipelonk/ae/PLONK/utils/zk_function.cu
> Collected: 2026-10-26
> Published: Unknown

#include "function.cuh"
bool gt\_zkp(SyncedMemory a, SyncedMemory b){
int64\_t numel = a.size()/sizeof(uint64\_t);
uint64\_t\* a\_ = reinterpret\_cast(a.mutable\_cpu\_data());
uint64\_t\* b\_ = reinterpret\_cast(b.mutable\_cpu\_data());
bool gt = false;
for(int64\_t i = numel-1; i >= 0; i--){
if(a\_[i] > b\_[i]){
gt = true;
break;
}
else if(a\_[i] < b\_[i]){
gt = false;
break;
}
else{
continue;
}
}
return gt;
}