Change limit of 50% for cudaHostAlloc pinned memory on Windows 10/11 - #8 by Robert\_Crovella - CUDA Programming and Performance - NVIDIA Developer Forums

[NVIDIA Developer Forums](/)

[Change limit of 50% for cudaHostAlloc pinned memory on Windows 10/11](/t/change-limit-of-50-for-cudahostalloc-pinned-memory-on-windows-10-11/228235)
=====================================================================================================================================================

[Accelerated Computing](/c/accelerated-computing/5)

[CUDA](/c/accelerated-computing/cuda/206)

[CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7)

[Robert\_Crovella](https://forums.developer.nvidia.com/u/Robert_Crovella)


September 19, 2022, 2:45am
8

The limit is entirely managed by Windows, and a typical limit is 50% of system memory. Sorry, I don’t have further details to share. No the limit is not 50% of system memory in all cases. I don’t have further details to share. I don’t have or know of any mechanisms to modify or override the observed limit, and the NVIDIA driver doesn’t control or set the limit.

I don’t have any permission to explain exactly how the driver pins memory, or confirm any of the speculation in this thread.

Because of additional WSL2->WDDM machinery that I don’t have permission to describe, the pinning limits on WSL2 are likely to be lower than what you would observe strictly on the windows side.

You’re welcome to file a bug to either 1. request additional documentation in this area, or 2. to “increase the limit”. Based on the information I have available to me, for the reasons already indicated, bugs along the line of 2 are unlikely to be actionable.

[show post in topic](/t/change-limit-of-50-for-cudahostalloc-pinned-memory-on-windows-10-11/228235#post_8)

### Related topics

| Topic |  | Replies | Views | Activity |
| --- | --- | --- | --- | --- |
| [Max amount of host pinned memory available for allocation](https://forums.developer.nvidia.com/t/max-amount-of-host-pinned-memory-available-for-allocation/56053)  [CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7) | 8 | 9007 | February 4, 2021 |
| [cudaHostRegister(): strange/unexpected behaviour under Windows 10](https://forums.developer.nvidia.com/t/cudahostregister-strange-unexpected-behaviour-under-windows-10/77439)  [CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7) | 4 | 1311 | October 22, 2019 |
| [Unexpected limit in cudaHostAlloc Failing to allocate large amounts of pinned/page-locked memory](https://forums.developer.nvidia.com/t/unexpected-limit-in-cudahostalloc-failing-to-allocate-large-amounts-of-pinned-page-locked-memory/19951)  [CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7) | 3 | 4326 | December 6, 2010 |
| [estimate an upper limit for pinned memory (windows, linux) - how ?](https://forums.developer.nvidia.com/t/estimate-an-upper-limit-for-pinned-memory-windows-linux-how/53134)  [CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7) | 4 | 1917 | September 5, 2017 |
| [amount of pinned memory](https://forums.developer.nvidia.com/t/amount-of-pinned-memory/6615)  [CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7) | 17 | 12792 | December 4, 2008 |
| [Pinned memory limit](https://forums.developer.nvidia.com/t/pinned-memory-limit/39777)  [CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7) | 16 | 14231 | May 1, 2016 |
| [Out Of Memory Error Allocating large chunks (> 1GB) of pinned-memory fails](https://forums.developer.nvidia.com/t/out-of-memory-error-allocating-large-chunks-1gb-of-pinned-memory-fails/22383)  [CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7) | 3 | 6044 | June 4, 2011 |
| [Significant decrease of available page-locked memory at Win7 x64 vs. Win7 x32](https://forums.developer.nvidia.com/t/significant-decrease-of-available-page-locked-memory-at-win7-x64-vs-win7-x32/23025)  [CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7) | 3 | 5802 | June 18, 2011 |
| [GTX 680 fails after large cudaHostAllocaPortable allocation](https://forums.developer.nvidia.com/t/gtx-680-fails-after-large-cudahostallocaportable-allocation/26888)  [CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7) | 11 | 4039 | June 13, 2012 |
| [Cudamalloc attempting to allocate more memory than it is supposed to](https://forums.developer.nvidia.com/t/cudamalloc-attempting-to-allocate-more-memory-than-it-is-supposed-to/319858)  [CUDA Programming and Performance](/c/accelerated-computing/cuda/cuda-programming-and-performance/7) [cuda](https://forums.developer.nvidia.com/tag/cuda/31) | 15 | 1311 | January 13, 2025 |

* [Home](/)
* [Categories](/categories)
* [Guidelines](/guidelines)
* [Terms of Service](https://www.nvidia.com/en-us/about-nvidia/legal-info/)
* [Privacy Policy](https://www.nvidia.com/en-us/about-nvidia/privacy-policy/)

Powered by [Discourse](https://www.discourse.org), best viewed with JavaScript enabled