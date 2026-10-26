# Control How Your GPU Shares Work with Green Contexts
> Source: https://developer.nvidia.com/blog/control-how-your-gpu-shares-work-with-green-contexts/
> Collected: 2026-10-26
> Published: 2026-10-06

Control How Your GPU Shares Work with Green Contexts | NVIDIA Technical Blog


[![Home](https://developer-blogs.nvidia.com/wp-content/themes/nvidia/dist/images/nvidia-logo_28b633c7.svg)![Home](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20142%2032%22%3E%3C/svg%3E)](/ "Home")
[DEVELOPER](/ "Home")

* [Home](/ "Home")
* [Blog](/blog "Blog")
* [Forums](https://forums.developer.nvidia.com/ "Forums")
* [Docs](https://docs.nvidia.com/ "Docs")
* [Downloads](https://developer.nvidia.com/downloads "Downloads")
* [Training](https://www.nvidia.com/en-us/training/ "Training")

* [Join](https://developer.nvidia.com/login)

[Technical Blog](https://developer.nvidia.com/blog)

[Subscribe](https://developer.nvidia.com/email-signup)

[Related Resources](#main-content-end)

[Data Center / Cloud](https://developer.nvidia.com/blog/category/data-center-cloud/)



English中文

Control How Your GPU Shares Work with Green Contexts
====================================================

![](https://developer-blogs.nvidia.com/wp-content/uploads/2026/10/image1-3.webp)

Oct 06, 2026

By [Kyrylo Perelygin](https://developer.nvidia.com/blog/author/kperelygin/ "Posts by Kyrylo Perelygin"), [Shelton Dsouza](https://developer.nvidia.com/blog/author/sdsouza/ "Posts by Shelton Dsouza") and [Myrto Papadopoulou](https://developer.nvidia.com/blog/author/myrtop/ "Posts by Myrto Papadopoulou")

Like

[Discuss (0)](#entry-content-comments)

* [L](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fcontrol-how-your-gpu-shares-work-with-green-contexts%2F)
* [T](https://twitter.com/intent/tweet?text=Control+How+Your+GPU+Shares+Work+with+Green+Contexts+%7C+NVIDIA+Technical+Blog+https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fcontrol-how-your-gpu-shares-work-with-green-contexts%2F)
* [F](https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fcontrol-how-your-gpu-shares-work-with-green-contexts%2F)
* [R](https://www.reddit.com/submit?url=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fcontrol-how-your-gpu-shares-work-with-green-contexts%2F&title=Control+How+Your+GPU+Shares+Work+with+Green+Contexts+%7C+NVIDIA+Technical+Blog)
* [E](mailto:?subject=I'd%20like%20to%20share%20a%20link%20with%20you&body=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fcontrol-how-your-gpu-shares-work-with-green-contexts%2F)

AI-Generated Summary
--------------------

* [Green contexts](https://docs.nvidia.com/cuda/cuda-runtime-api/group__CUDART__EXECUTION__CONTEXT.html#group__CUDART__EXECUTION__CONTEXT) in CUDA 13.1 let applications explicitly partition GPU execution resources such as SMs and workqueues within a single process.
* By assigning dedicated SMs to a green context, a latency-sensitive kernel can run without waiting for bulk workloads to drain, reducing critical kernel latency from 0.140 ms to 0.007 ms on a Blackwell GPU with 148 SMs.
* The programming model creates streams from a green context handle rather than relying on implicit thread-local device state, while preserving the existing stream-based CUDA workflow.
* Green contexts are opt-in and additive, so existing applications can adopt them incrementally where finer control over concurrent workloads is needed.

### Next Steps

* Read the [CUDA Programming Guide green contexts section](https://docs.nvidia.com/cuda/cuda-programming-guide/04-special-topics/green-contexts.html) for detailed usage and examples.
* Review the [CUDA Driver API green contexts reference](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__GREEN__CONTEXTS.html#group__CUDA__GREEN__CONTEXTS) for low-level resource partitioning APIs.

Powered by NVIDIA Nemotron. AI-generated content may summarize information incompletely. Verify important information. [Learn more](https://www.nvidia.com/en-us/agreements/trustworthy-ai/terms/)

GPU applications increasingly consist of multiple independent components running at the same time within a single process: a latency-sensitive operator alongside a throughput-oriented background kernel; a data preprocessing stage alongside model inference; or multiple stages of a processing workflow sharing a single GPU.

Controlling how GPU resources are shared between them remains difficult. Components can interfere unpredictably, and existing tools offer limited ability to partition resources.

Green contexts address this by letting applications explicitly select a subset of GPU execution resources and target work to those resources directly. Traditional CUDA contexts weren’t designed for this usage model. They are heavyweight, incur hardware context-switch overhead, and reflect assumptions from an earlier era, when GPUs were smaller and applications typically ran as a single dominant workload.

Green contexts have been available in the [Driver API](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__GREEN__CONTEXTS.html#group__CUDA__GREEN__CONTEXTS) since NVIDIA CUDA 12.4. Starting with CUDA 13.1, they are also accessible through the [Runtime API](https://docs.nvidia.com/cuda/cuda-runtime-api/group__CUDART__EXECUTION__CONTEXT.html#group__CUDART__EXECUTION__CONTEXT), allowing an application, within its process, to explicitly define where work runs and how execution resources are divided.

How green contexts work
-----------------------

One primary use of green contexts is **SM partitioning**. By assigning a specific subset of SMs to a green context, applications can target work submitted through that green context to those SMs. This can enable multiple workloads to run concurrently on the GPU without competing for the same compute units.

In addition to SM partitioning, **green contexts can provision workqueue resources**. In the traditional model, independent stream-ordered workloads may map to the same underlying workqueues, introducing unintended serialization even when sufficient execution resources are available. By provisioning workqueues explicitly, green contexts allow applications to express expected concurrency and reduce false dependencies.

Green contexts are lightweight to create and destroy, and creating or destroying one doesn’t implicitly synchronize unrelated GPU work. They also provide a more explicit programming model where applications target work to a chosen green context rather than relying only on implicit/current device state.

Explicit programming model with green contexts
----------------------------------------------

Historically, CUDA Runtime applications typically selected a device with `cudaSetDevice()`, created streams, and submitted work to those streams. The stream’s execution target was inferred from the current device or context for the calling thread at the time the stream was created.

Green contexts make that targeting more explicit. An application creates a green context for a chosen set of resources, then creates streams from that green context. Work submitted to those streams is associated with the green context’s resources.

In the CUDA Runtime API, green contexts are represented with the `cudaExecutionContext_t` type, a Runtime abstraction for CUDA contexts. For green-context use, the important point is that `cudaGreenCtxCreate()` returns a handle that can be passed directly to APIs such as `cudaExecutionCtxStreamCreate()`, instead of relying on implicit thread-local device or context state.

For streams, this shift is reflected directly in how applications create them.

Note: For brevity, the following code snippets omit full error checking. Production code should check all CUDA Runtime API return values, use `cudaGetLastError()` or `cudaPeekAtLastError()` after kernel launches, and check synchronization calls such as `cudaStreamSynchronize()` for asynchronous execution errors.

**Traditional Runtime flow: stream target comes from current device state**

```
cudaSetDevice(device);
cudaStream_t s;
cudaStreamCreate(&s);
kernel<<<grid, block, 0, s>>>();
```

**Green-context flow: stream target is selected explicitly**

```
cudaExecutionContext_t greenCtx;
cudaGreenCtxCreate(&greenCtx, desc, device, 0);

cudaStream_t s;
cudaExecutionCtxStreamCreate(&s, greenCtx, 0, 0);
kernel<<<grid, block, 0, s>>>();
```

The code changes are minimal, but the mental model is clearer. Instead of relying on hidden thread-local device state, applications explicitly choose a green context and create streams for it. The rest of the application code can continue using the same stream-based CUDA programming model.

Applications that want to target the full device can continue using the traditional Runtime model. For APIs that take an explicit context handle, they can obtain the device-wide context with `cudaDeviceGetExecutionCtx()`.

Example
=======

GPU workloads may often require a small, latency-sensitive kernel to share a device with a larger throughput-oriented worker. A canonical example is communication/GEMM overlap in distributed training and inference, or latency-sensitive operators in AI sensor processing platforms like [NVIDIA Holoscan](https://docs.nvidia.com/holoscan/index.htm) that require starting as soon as possible.

One of the standard tools for prioritizing such critical work is [CUDA stream priority](https://docs.nvidia.com/cuda/cuda-programming-guide/03-advanced/advanced-host-programming.html#stream-priorities). Unfortunately, setting priority isn’t enough to guarantee immediate execution of a higher priority kernel, when the bulk kernel fully occupies all of the GPU’s SMs. If bulk kernels are queued up and the critical kernel arrives, the scheduler hands critical kernel threads blocks to the next SM(s) that free(s) up. Stream priorities can’t preempt a block that’s already executing on an SM, so it needs to wait for some of the blocks to drain.

Let’s create a sample and look at some results. The setup code is the new piece we need to write:

```
// 1. Query all SMs on the device.
cudaDevResource all_sm {};
cudaDeviceGetDevResource(dev, &all_sm, cudaDevResourceTypeSm);

// 2. Carve out one critical group (arch's coscheduled alignment).
cudaDevSmResourceGroupParams crit_params {};
crit_params.smCount = all_sm.sm.smCoscheduledAlignment;

cudaDevResource critical_res{}, remaining_res{};
cudaDevSmResourceSplit(&critical_res, 1, &all_sm, &remaining_res, 0, &crit_params);

// 3. Pack each partition with its own workqueue-config resource so queue
//    pressure is isolated too, then generate the descriptor.
cudaDevResource wq {};
wq.type = cudaDevResourceTypeWorkqueueConfig;
wq.wqConfig.device = dev;
wq.wqConfig.sharingScope = cudaDevWorkqueueConfigScopeGreenCtxBalanced;
wq.wqConfig.wqConcurrencyLimit = 2;


cudaDevResource crit_pack[2] = { critical_res, wq };
cudaDevResourceDesc_t crit_desc{};
cudaDevResourceGenerateDesc(&crit_desc, crit_pack, 2);

cudaDevResource bulk_pack[2] = { remaining_res, wq };
cudaDevResourceDesc_t bulk_desc{};
cudaDevResourceGenerateDesc(&bulk_desc, bulk_pack, 2);

// 4. Create each green context, then a stream on it.
cudaExecutionContext_t crit_ctx{};
cudaGreenCtxCreate(&crit_ctx, crit_desc, dev, 0);

cudaStream_t crit_stream{};
cudaExecutionCtxStreamCreate(&crit_stream, crit_ctx, cudaStreamNonBlocking, prio_high);

cudaExecutionContext_t bulk_ctx{};
cudaGreenCtxCreate(&bulk_ctx, bulk_desc, dev, 0);

cudaStream_t bulk_stream{};
cudaExecutionCtxStreamCreate(&bulk_stream, bulk_ctx, cudaStreamNonBlocking, prio_low); // define prio_low
```

While the rest of the code should look familiar:

```
// Saturate the bulk partition.
for (int i = 0; i < BULK_LAUNCHES; ++i) {
    bulk_kernel<<<bulk_grid, bulk_block, 0, bulk_stream>>>(
        d_bulk, N_BULK, BULK_ITERS);
}
// Launch the critical kernel on its own partition.
cudaEventRecord(t_start, crit_stream);
critical_kernel<<<crit_grid, crit_block, 0, crit_stream>>>(d_crit, N_CRIT);
cudaEventRecord(t_stop, crit_stream);
cudaStreamSynchronize(crit_stream);
float crit_ms = 0.0f;
cudaEventElapsedTime(&crit_ms, t_start, t_stop);
```

The stream carries the partition and the kernel sees only the SMs it’s allowed to run on.

We test three modes, same critical + bulk workload in each:

* **Mode A**: Green-ctx partition, high-priority critical stream
* **Mode B**: Default context, high-priority critical stream (no partition)
* **Mode C**: Default context, normal priority on both streams (no stream priorities; no partition)

The critical kernel is a tiny integer workload. The bulk kernel is 20 back-to-back launches of a 4M-thread `sqrtf` loop designed to saturate the device. We measure the wall-clock latency of the critical kernel while bulk is running.

On an NVIDIA Blackwell GPU with 148 SMs:

| Mode | Critical Kernel Latency |
| --- | --- |
| Green-ctx (8 SMs for critical / 140 SMs for bulk) | 0.007 ms |
| Default ctx, high-priority critical | 0.140 ms |
| Default ctx, equal priority | 3.727 ms |

*Table 1. Critical kernel latency by execution mode*

Stream priority alone is a huge win over not using it. Roughly 27x faster than equal-priority streams competing for the same SMs. But priority has a ceiling. An additional 20x is wasted waiting for bulk blocks to drain, even with the GPU scheduler prioritizing the critical kernel. Green contexts skip that wait entirely because the critical SMs are dedicated to that workload. The tradeoff is that less SMs are available for the bulk kernel’s execution.

The same skeleton can be used to achieve desired concurrency rather than latency improvement, such as overlapping a communication kernel with a GEMM kernel so both make progress. Advanced use-cases often use both green context partitions and stream priorities to achieve desired performance targets.

When to use green contexts
--------------------------

They are most useful when your application:

* Runs multiple independent workloads concurrently on the same GPU.
* Needs more predictable performance rather than best-effort stream scheduling
* Wants explicit control over how GPU execution resources are divided
* Sees degraded performance due to limited control over workqueues
* Is evolving toward pipeline-based or multi-component execution within a single process

Getting started
---------------

* Build your application with CUDA 13.1 or newer.
* Query device resources and create a green context for the resource partition you want to target.
* Create streams for that green context and submit work using the same stream-based CUDA programming model.
* For APIs that take an explicit context handle and should target the full device, use `cudaDeviceGetExecutionCtx()`.

Green contexts are opt-in and additive. Existing applications can continue to target the full device unchanged and adopt green contexts incrementally where finer control over execution is needed.

For additional details and examples, see the [CUDA Programming Guide](https://docs.nvidia.com/cuda/cuda-programming-guide/04-special-topics/green-contexts.html) and [CUDA Runtime API](https://docs.nvidia.com/cuda/cuda-runtime-api/group__CUDART__EXECUTION__CONTEXT.html#group__CUDART__EXECUTION__CONTEXT) documentation, and try green contexts today.

[Discuss (0)](#entry-content-comments)

Like

Tags
----

[Data Center / Cloud](https://developer.nvidia.com/blog/category/data-center-cloud/) | [General](https://developer.nvidia.com/blog/recent-posts/?industry=General) | [CUDA](https://developer.nvidia.com/blog/recent-posts/?products=CUDA) | [Intermediate Technical](https://developer.nvidia.com/blog/recent-posts/?learning_levels=Intermediate+Technical) | [Deep dive](https://developer.nvidia.com/blog/recent-posts/?content_types=Deep+dive) | [featured](https://developer.nvidia.com/blog/tag/featured/)

About the Authors
-----------------

![Avatar photo](https://developer.nvidia.com/blog/parallelforall/wp-content/uploads/2017/10/Kyrylo-Perelygin_avatar_1507176671.jpg)![Avatar photo](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20131%20131%22%3E%3C/svg%3E)

**About Kyrylo Perelygin**
  
Kyrylo Perelygin has worked on features such as CUDA's Multi-Process Service and the new Cooperative Groups since joining NVIDIA, where he first interned in 2013. Kyrylo graduated with a bachelor's from EPITECH and obtained a master's from CSULB.

[View all posts by Kyrylo Perelygin](https://developer.nvidia.com/blog/author/kperelygin/)

![Avatar photo](https://developer-blogs.nvidia.com/wp-content/uploads/2022/12/cropped-Shelton-Dsouza-131x131.jpg)![Avatar photo](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20131%20131%22%3E%3C/svg%3E)

**About Shelton Dsouza**
  
Shelton is a senior system software engineer at NVIDIA working on CUDA driver and runtime. His primary focus is on designing forward-looking improvements to the CUDA APIs and programming model and improving ease of use. He graduated with a master's degree from Carnegie Mellon University in 2016

[View all posts by Shelton Dsouza](https://developer.nvidia.com/blog/author/sdsouza/)

![Avatar photo](https://developer-blogs.nvidia.com/wp-content/uploads/2026/10/cropped-image2-1-131x131.webp)![Avatar photo](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20131%20131%22%3E%3C/svg%3E)

**About Myrto Papadopoulou**
  
Myrto Papadopoulou is an engineer on the Developer Technology team at NVIDIA, where she works on performance optimizations. Myrto holds a Ph.D. from the University of Toronto, Canada, in the area of computer architecture.

[View all posts by Myrto Papadopoulou](https://developer.nvidia.com/blog/author/myrtop/)

Comments
--------

Related posts
-------------

![Glowing green compute cube surrounded by translucent layers and smaller cubes, illustrating NVIDIA CUDA computing.](https://developer-blogs.nvidia.com/wp-content/uploads/2026/06/cuda-python-660x370.png)![Glowing green compute cube surrounded by translucent layers and smaller cubes, illustrating NVIDIA CUDA computing.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20660%20370%22%3E%3C/svg%3E)

### CUDA Toolkit 13.4 Adds Windows on Arm Support and Greater Control over Shared GPUs

[CUDA Toolkit 13.4 Adds Windows on Arm Support and Greater Control over Shared GPUs](https://developer.nvidia.com/blog/cuda-toolkit-13-4-adds-windows-on-arm-support-and-greater-control-over-shared-gpus/)

![Image of Nsight Systems report.](https://developer-blogs.nvidia.com/wp-content/uploads/2024/04/gpu-occupancy-workloads-featured-960x540.png)![Image of Nsight Systems report.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20658%20370%22%3E%3C/svg%3E)

### Measuring the GPU Occupancy of Multi-stream Workloads

[Measuring the GPU Occupancy of Multi-stream Workloads](https://developer.nvidia.com/blog/measuring-the-gpu-occupancy-of-multi-stream-workloads/)

![](https://developer-blogs.nvidia.com/wp-content/uploads/2023/11/nvidia-grace-hopper-960x540.png)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20658%20370%22%3E%3C/svg%3E)

### Simplifying GPU Programming for HPC with NVIDIA Grace Hopper Superchip

[Simplifying GPU Programming for HPC with NVIDIA Grace Hopper Superchip](https://developer.nvidia.com/blog/simplifying-gpu-programming-for-hpc-with-the-nvidia-grace-hopper-superchip/)

![](https://developer-blogs.nvidia.com/wp-content/uploads/2021/07/CUDA-malloc-FeaturedImage.jpg)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20540%20370%22%3E%3C/svg%3E)

### Using the NVIDIA CUDA Stream-Ordered Memory Allocator, Part 1

[Using the NVIDIA CUDA Stream-Ordered Memory Allocator, Part 1](https://developer.nvidia.com/blog/using-cuda-stream-ordered-memory-allocator-part-1/)

![](https://developer-blogs.nvidia.com/wp-content/uploads/2020/12/CUDA_3x2.jpg)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20555%20370%22%3E%3C/svg%3E)

### Enhancing Memory Allocation with New NVIDIA CUDA 11.2 Features

[Enhancing Memory Allocation with New NVIDIA CUDA 11.2 Features](https://developer.nvidia.com/blog/enhancing-memory-allocation-with-new-cuda-11-2-features/)

Related posts
-------------

![](https://developer-blogs.nvidia.com/wp-content/uploads/2026/10/image4-2-660x370-png.webp)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20660%20370%22%3E%3C/svg%3E)

### Design Proteins with RFD4-Proteina Using Reward-Guided Inference-Time Search

[Design Proteins with RFD4-Proteina Using Reward-Guided Inference-Time Search](https://developer.nvidia.com/blog/design-proteins-with-rfd4-proteina-using-reward-guided-inference-time-search/)

![Observatory dome beneath curved star trails in the night sky.](https://developer-blogs.nvidia.com/wp-content/uploads/2026/10/Satellite-Stars-e1791320112446-660x370.webp)![Observatory dome beneath curved star trails in the night sky.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20660%20370%22%3E%3C/svg%3E)

### Faster Scientific Image Analysis with NVIDIA cuPhoton

[Faster Scientific Image Analysis with NVIDIA cuPhoton](https://developer.nvidia.com/blog/faster-scientific-image-analysis-with-nvidia-cuphoton/)

![Laptop surrounded by application icons and a compact computing device, illustrating local AI applications built with C++ and NVIDIA TensorRT RTX.](https://developer-blogs.nvidia.com/wp-content/uploads/2026/09/din-deploy-featured-660x370-png.webp)![Laptop surrounded by application icons and a compact computing device, illustrating local AI applications built with C++ and NVIDIA TensorRT RTX.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20660%20370%22%3E%3C/svg%3E)

### Build Local AI Apps with C++ and NVIDIA TensorRT RTX Samples

[Build Local AI Apps with C++ and NVIDIA TensorRT RTX Samples](https://developer.nvidia.com/blog/build-local-ai-apps-with-c-and-nvidia-tensorrt-rtx-samples/)

![Colorful DNA double helix against a black background, illustrating biological foundation models.](https://developer-blogs.nvidia.com/wp-content/uploads/2026/09/DNA-Helix-660x370-jpg.webp)![Colorful DNA double helix against a black background, illustrating biological foundation models.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20660%20370%22%3E%3C/svg%3E)

### Efficient MoE Training for Biological Foundation Models

[Efficient MoE Training for Biological Foundation Models](https://developer.nvidia.com/blog/efficient-moe-training-for-biological-foundation-models/)

![](https://developer-blogs.nvidia.com/wp-content/uploads/2026/09/image4-1.webp)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20600%20338%22%3E%3C/svg%3E)

### High-Throughput Structure Prediction with BioNeMo Inference Runtime

[High-Throughput Structure Prediction with BioNeMo Inference Runtime](https://developer.nvidia.com/blog/high-throughput-structure-prediction-with-bionemo-inference-runtime/)

* [L](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fcontrol-how-your-gpu-shares-work-with-green-contexts%2F)
* [T](https://twitter.com/intent/tweet?text=Control+How+Your+GPU+Shares+Work+with+Green+Contexts+%7C+NVIDIA+Technical+Blog+https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fcontrol-how-your-gpu-shares-work-with-green-contexts%2F)
* [F](https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fcontrol-how-your-gpu-shares-work-with-green-contexts%2F)
* [R](https://www.reddit.com/submit?url=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fcontrol-how-your-gpu-shares-work-with-green-contexts%2F&title=Control+How+Your+GPU+Shares+Work+with+Green+Contexts+%7C+NVIDIA+Technical+Blog)
* [E](mailto:?subject=I'd%20like%20to%20share%20a%20link%20with%20you&body=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fcontrol-how-your-gpu-shares-work-with-green-contexts%2F)

![](https://developer.nvidia.com/akam/13/pixel_4b3bf5?a=dD1kNGRiZjMxMDJkZTExMDAwZGY5OTgwOGJjMGMwOTNmMTI0Mjk5MDAxJmpzPW9mZg==)