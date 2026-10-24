/\*\*\*
Copyright (c) 2024, Snarkify, Inc. All rights reserved.
Dual licensed under the MIT License or the Apache License, Version 2.0.
See LICENSE for details.
Author(s): Niall Emmart
\*\*\*/
#include 
#include 
#include "asm.cu"
#include "Chain.cu"
#include "MP.cu"
#include "BLS12381\_fr.cu"
#include "Common/core.h"
#include "Common/host.h"
typedef storage,false> affine\_t;
typedef storage st\_t;
typedef impl impl\_t;
typedef impl impl\_mont\_t;
typedef host::ntt\_context ntt\_context\_t;
//typedef host::msm\_context msm\_context\_t;
//typedef host::poly\_context poly\_context\_t;
#define $CUDA(call) { int localEC=call; if(localEC!=cudaSuccess) { printf("\nCall \"" #call "\" failed from %s, line %d, error=%d\n", \_\_FILE\_\_, \_\_LINE\_\_, localEC); exit(1); } }
#include "ScanReduce.cu"
#include "PowerTable.cu"
#include "NTT/NTT.h"
#include "Montgomery.cu"
#include "Polynomial.cu"
#include "MSM/MSM.h"
#include "MathAPI.cu"
uint32\_t generator[24]={
0xDB22C6BB, 0xFB3AF00A, 0xF97A1AEF, 0x6C55E83F, 0x171BAC58, 0xA14E3A3F, /\* x \*/
0x9774B905, 0xC3688C4F, 0x4FA9AC0F, 0x2695638C, 0x3197D794, 0x17F1D3A7,
0x46C5E7E1, 0x0CAA2329, 0xA2888AE4, 0xD03CC744, 0x2C04B3ED, 0x00DB18CB, /\* y \*/
0xD5D00AF6, 0xFCF5E095, 0x741D8AE4, 0xA09E30ED, 0xE3AAA0F1, 0x08B3F481,
};
#define SMALL (1<<22)
#define LARGE (1<<25)
#define POW\_TAB 256\*49
\_\_managed\_\_ uint32\_t msm\_points[SMALL\*2\*12];
\_\_managed\_\_ uint32\_t roots[ROOT\_COUNT\*8];
\_\_managed\_\_ uint32\_t inv\_roots[ROOT\_COUNT\*8];
\_\_managed\_\_ uint32\_t pow\_tab[POW\_TAB];
\_\_managed\_\_ uint32\_t ipow\_tab[POW\_TAB];
\_\_managed\_\_ uint32\_t eval\_x[8];
\_\_managed\_\_ uint32\_t s\_res[SMALL\*8];
\_\_managed\_\_ uint32\_t s\_a[SMALL\*8];
\_\_managed\_\_ uint32\_t s\_b[SMALL\*8];
\_\_managed\_\_ uint32\_t s\_t1[SMALL\*8];
\_\_managed\_\_ uint32\_t s\_t2[SMALL\*8];
\_\_managed\_\_ uint32\_t l\_res[LARGE\*8];
\_\_managed\_\_ uint32\_t l\_a[LARGE\*8];
\_\_managed\_\_ uint32\_t l\_b[LARGE\*8];
\_\_managed\_\_ uint32\_t l\_t1[LARGE\*8];
\_\_managed\_\_ uint32\_t l\_t2[LARGE\*8];
MSM\* msm=NULL;
uint32\_t random\_word() {
uint32\_t r=rand() & 0xFFFF;
r=(r<<16) | (rand() & 0xFFFF);
return r;
}
void random\_words(uint32\_t\* data, uint32\_t count) {
for(int i=0;imsm(s\_a, SMALL);
else
printf("Unhandled op: %s\n", op);
}
void run(const char\* op, uint32\_t runs) {
cudaEvent\_t start, stop;
float time, total=0.0, min=0.0, max=0.0;
run\_once(op);
$CUDA(cudaEventCreate(&start));
$CUDA(cudaEventCreate(&stop));
for(int run=0;runtime) ? max : time;
}
total+=time;
}
printf("%s: min=%0.3f max=%0.3f ave=%0.3f\n", op, min, max, total/((float)runs));
$CUDA(cudaEventDestroy(start));
$CUDA(cudaEventDestroy(stop));
}
int main() {
random\_scalars(eval\_x, 1);
random\_scalars(s\_res, SMALL);
random\_scalars(s\_a, SMALL);
random\_scalars(s\_b, SMALL);
random\_scalars(l\_res, LARGE);
random\_scalars(l\_a, LARGE);
random\_scalars(l\_b, LARGE);
for(int i=0;iinitialize(msm\_points, SMALL, 1);
initialize\_ntt\_roots((bls12381\_fr\*)roots, (bls12381\_fr\*)inv\_roots);
generate\_power\_table(pow\_tab, eval\_x);
generate\_inv\_power\_table(ipow\_tab, eval\_x);
run("coset-ntt-25", 20);
run("coset-ntt-25", 20);
run("ntt-25", 20);
run("ntt-22", 20);
run("intt-22", 20);
run("polynomial-divide-22", 20);
run("polynomial-evaluate-22", 20);
run("polynomial-evaluate-25", 20);
run("batch-invert-22", 20);
run("msm-22", 20);
$CUDA(cudaDeviceSynchronize());
}