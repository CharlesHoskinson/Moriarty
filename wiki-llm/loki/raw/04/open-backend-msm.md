#include "cuda\_msm.cuh"
/\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\* BACKEND REGISTRATION \*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*/
using namespace msm;
using namespace icicle;
REGISTER\_MSM\_BACKEND("CUDA", (msm\_cuda\_wrapper));
REGISTER\_MSM\_PRE\_COMPUTE\_BASES\_BACKEND("CUDA", (msm\_precompute\_bases\_cuda\_wrapper));