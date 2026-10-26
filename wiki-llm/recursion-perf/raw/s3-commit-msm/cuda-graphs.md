# NVIDIA CUDA graphs launch overhead
> Source: https://developer.nvidia.com/blog/constant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements/
> Collected: 2026-10-26
> Published: 2024-09-11

Constant Time Launch for Straight-Line CUDA Graphs and Other Performance Enhancements | NVIDIA Technical Blog


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

[Agentic AI / Generative AI](https://developer.nvidia.com/blog/category/generative-ai/)



English中文

Constant Time Launch for Straight-Line CUDA Graphs and Other Performance Enhancements
=====================================================================================

![Decorative image of light fields in green, purple, and blue.](https://developer-blogs.nvidia.com/wp-content/uploads/2023/12/hpc-featured-1024x576.jpg)

Sep 11, 2024

By [Houston Hoffman](https://developer.nvidia.com/blog/author/hhoffman/ "Posts by Houston Hoffman") and [Fred Oh](https://developer.nvidia.com/blog/author/fredoh/ "Posts by Fred Oh")

Like

[Discuss (1)](#entry-content-comments)

* [L](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fconstant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements%2F)
* [T](https://twitter.com/intent/tweet?text=Constant+Time+Launch+for+Straight-Line+CUDA+Graphs+and+Other+Performance+Enhancements+%7C+NVIDIA+Technical+Blog+https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fconstant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements%2F)
* [F](https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fconstant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements%2F)
* [R](https://www.reddit.com/submit?url=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fconstant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements%2F&title=Constant+Time+Launch+for+Straight-Line+CUDA+Graphs+and+Other+Performance+Enhancements+%7C+NVIDIA+Technical+Blog)
* [E](mailto:?subject=I'd%20like%20to%20share%20a%20link%20with%20you&body=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fconstant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements%2F)

AI-Generated Summary
--------------------

* CUDA Graphs group CUDA kernels and operations into a graph with a dependency tree, reducing CPU overhead by combining driver activities for kernel launches and API calls.
* Between CUDA Toolkit 11.8 and 12.6, graph instantiation time improved by up to 40%, first-launch CPU overhead dropped up to 66% for longer graphs, and repeat-launch device runtime decreased up to 15% on straight-line topologies.
* On NVIDIA Ampere architecture, repeat-launch CPU overhead for straight-line kernel graphs shifted from 2μs + 200 ns per node to a nearly constant 2.5 μs + ~1 ns per node for graphs with 10 or more nodes.
* CUDA Toolkit 12.0 introduced scheduling heuristics that respect node creation order, helping developers achieve more intuitive execution ordering and avoid unintended serialization of memory copy operations.
* The microbenchmark code used for these measurements is available in the [cudaGraphsPerfScaling sample](https://github.com/NVIDIA/cuda-samples/tree/master/Samples/6_Performance/cudaGraphsPerfScaling) for reproduction on other hardware.

### Next Steps

* Read the [CUDA C++ Programming Guide: CUDA Graphs](https://docs.nvidia.com/cuda/cuda-programming-guide/04-special-topics/cuda-graphs.html) for detailed API reference and update mechanisms.
* Explore [Getting Started with CUDA Graphs](https://developer.nvidia.com/blog/cuda-graphs/) for foundational concepts and usage patterns.
* Review [Constructing CUDA Graphs with Dynamic Parameters](https://developer.nvidia.com/blog/constructing-cuda-graphs-with-dynamic-parameters/) to learn advanced graph-building techniques.

Powered by NVIDIA Nemotron. AI-generated content may summarize information incompletely. Verify important information. [Learn more](https://www.nvidia.com/en-us/agreements/trustworthy-ai/terms/)

[CUDA Graphs](https://developer.nvidia.com/blog/cuda-graphs/) are a way to define and batch GPU operations as a graph rather than a sequence of stream launches. A CUDA Graph [groups](https://developer.nvidia.com/blog/constructing-cuda-graphs-with-dynamic-parameters/) a set of CUDA kernels and other CUDA operations together and executes them with a specified dependency tree. It speeds up the workflow by combining the driver activities associated with CUDA kernel launches and CUDA API calls. It also enforces the dependencies with hardware accelerations, instead of relying solely on CUDA streams and events, when possible.

CUDA Graphs are particularly important for AI frameworks, as they enable you to capture and replay a sequence of CUDA operations, reducing CPU overhead and improving performance. With the latest improvements, you can now take even greater advantage of CUDA Graphs to accelerate AI workloads.

NVIDIA has improved the performance of CUDA Graphs in several respects between CUDA Toolkit 11.8 and CUDA Toolkit 12.6 and the accompanying driver builds:

* Graph construction and instantiation time
* CPU launch overhead
* General performance in several edge cases

In this post, we present some microbenchmark numbers for various graphs and discuss the improvements since the initial release. The benchmark code is included. As always, performance characteristics depend on the CPU, GPU, and clock settings. For our testing, we used an Intel Xeon Silver 4208 Processor @ 2.10GHz and an NVIDIA GeForce RTX 3060 GPU. The OS was Ubuntu 22.04. The results in this post were collected using default clock settings.

This is a snapshot of performance improvements as of CUDA 12.6, so future CUDA releases may not demonstrate the same performance as documented in this post. This snapshot is presented assuming familiarity with CUDA Graphs and how their performance characteristics impact applications. For more information, see [Getting Started with CUDA Graphs](https://developer.nvidia.com/blog/cuda-graphs/).

Definitions
-----------

Here are some graph topology terms and performance characteristics discussed later in this post.

* **Straight line kernel graph:** A CUDA Graph made completely out of kernel nodes, where every node has a single dependent node except for the last node.
* **Parallel straight line:** A CUDA Graph that has width as the number of entry points. Each entry point is followed by its own set of lengths as the number of nodes arrayed in a straight-line formation. This is also referred to as *parallel chain* in the charts and source code.
* **First launch:** The first launch of a graph, which also includes uploading the instantiated graph to the device.
* **Repeat launch:** Launching a graph that has already been uploaded to the device.
* **CPU overhead for a launch:** CPU time between invoking `CUDAGraphLaunch` and the completion of the function.
* **Device side runtime:** The amount of time for the operation to run on the device.
* **Repeat launch device side runtime:** The amount of time it takes for a graph that was previously uploaded to run on the device.
* **Time to solution:** Time from the launch call to the completion of the graph running on the device.

Constant time CPU overhead for repeat launch
--------------------------------------------

One notable improvement highlighted is related to CPU overhead for repeat launch on the NVIDIA Ampere architecture. For straight-line kernel graphs, there has been a significant reduction in time taken during repeat launches. Specifically, the time decreased from 2μs + 200 ns extra time per node to a nearly constant time of 2.5us + (~1ns per node), showcasing consistent speed enhancements for graphs with 10 nodes or more.

Launch still takes \(O(n)\) time on pre-NVIDIA Ampere architecture hardware.

![Line chart shows that repeat launch CPU overhead had a linear dependency on the graph length in CUDA Toolkit 11.8.  It has a near-constant behavior in CUDA Toolkit 12.6. The line chart covers small straight line graph topologies up to 250 nodes in length where the overhead reaches 25 uS in 11.8 and is about 2 uS in 12.6.](https://developer-blogs.nvidia.com/wp-content/uploads/2024/09/cpu-overhead-small-straight-line-1024x609.png "Chart")![Line chart shows that repeat launch CPU overhead had a linear dependency on the graph length in CUDA Toolkit 11.8.  It has a near-constant behavior in CUDA Toolkit 12.6. The line chart covers small straight line graph topologies up to 250 nodes in length where the overhead reaches 25 uS in 11.8 and is about 2 uS in 12.6.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%201024%20609%22%3E%3C/svg%3E "Chart")


*Figure 1. CPU overhead of repeat launch of straight line graphs for small straight line topology*

![Line chart shows that the relationships from Figure 1 extend to larger straight-line graphs. The chart extends the relationship to graphs of 2000 nodes in size, where the overhead was 300 uS in 11.8.](https://developer-blogs.nvidia.com/wp-content/uploads/2024/09/cpu-overhead-large-straight-line-1024x609.png "Chart")![Line chart shows that the relationships from Figure 1 extend to larger straight-line graphs. The chart extends the relationship to graphs of 2000 nodes in size, where the overhead was 300 uS in 11.8.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%201024%20609%22%3E%3C/svg%3E "Chart")


*Figure 2. CPU overhead of repeat launch of straight-line graphs for large straight-line topology*

Because the parallel straight-line graphs measured here have four root nodes, the repeat launch with one node takes 6μs instead of 2μs. The impact of chain length on the CPU launch overhead is still negligible for this topology. The exact scaling of having multiple root nodes is not analyzed here, although the impact is expected to be linear with the number of root nodes. Similarly, graphs with non-kernel nodes would have different performance characteristics.

![Line chart shows that the relationships from Figure 1 extend to parallel chain topologies. The chart shows that a parallel chain topology with four chains and a length of 250 nodes has a launch overhead of >60 uS in 11.8 and takes close to 6 uS in 12.6.](https://developer-blogs.nvidia.com/wp-content/uploads/2024/09/cpu-overhead-four-parallel-chain-1024x609.png)![Line chart shows that the relationships from Figure 1 extend to parallel chain topologies. The chart shows that a parallel chain topology with four chains and a length of 250 nodes has a launch overhead of >60 uS in 11.8 and takes close to 6 uS in 12.6.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%201024%20609%22%3E%3C/svg%3E)


*Figure 3. CPU overhead of repeat launch of a graph with four parallel task chains*

Other measurable performance gains
----------------------------------

Several other aspects of performance can matter for applications benefitting from graphs:

* Instantiation time
* First launch CPU overhead
* Repeat launch device runtime
* End-to-end time for repeat graph launch into an empty stream

|  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- |
| **metric** | **topology** | **Topology length** | **11.8 (r520.61.05)** | **12.6 (r560.28.03)** | **%speedup** |
| **Instantiation** | **Straight line** | **10** | 20 uS | 16 uS | 25% |
| **100** | 168 uS | 127 uS | 32% |
| **1025** | 2143 uS | 1526 uS | 40% |
| **4 parallel chains** | **10** | 71 uS | 58 uS | 22% |
| **100** | 695 uS | 552 uS | 26% |
| **First Launch Cpu Overhead** | **Straight line** | **10** | 4 uS | 4uS | 0% |
| **100** | 25 uS | 15 uS | 66% |
| **1025** | 278 uS | 175 uS | 59% |
| **4 parallel chains** | **100** | 73 uS | 67 uS | 9% |
| **Repeat Launch Device Runtime** | **Straight line** | **10** | 7 uS | 7 uS | 0% better |
| **100** | 61 uS | 53 uS | 15% better |
| **1025** | 629 uS | 567 uS | 11% better |
| **End to end time for repeat graph launch into an empty stream** | **Straight line** | **10** | 12 uS | 9 uS | 30% better |
| **100** | 69 uS | 55 uS | 25% better |
| **1025** | 628 uS | 567 uS | 11% better |

*Table 1. Metric performance on CUDA Toolkit 11.8 and 12.6*

### Instantiation time

The amount of time to instantiate a graph dominates the runtime cost of adopting a graph. This is a one-time cost per graph instantiation. Graph creation time often appears on the critical path of application startup or for workflows that need to periodically create new graphs at times they do not have work running on the GPU.  
  
Improvements here can impact application startup time for applications that prepare many graphs at program start and minimize the end-to-end latency penalty to other applications that file to hide graph construction latency behind other work execution.

### First launch CPU overhead

As first launch is responsible for uploading the work descriptions for a CUDA Graph to the GPU, it still has an \(O(N)\) CPU cost. This cost is paid the first time and doesn’t need to be repaid except partially when the graph is updated. Most of this cost can be paid by performing a separate upload operation, which is not discussed in this post because it’s already available to CUDA Graphs users.

### Repeat launch device runtime

Running even empty kernels takes time. Optimizing some inter-kernel latency for straight-line graphs reduces device times by up to a noticeable 60 ns/node.

### End-to-end time for repeat graph launch into an empty stream

CUDA Graphs contains logic that may allow execution to begin before the launch operation processes all the nodes in a graph, hiding part of the launch cost. Due to this, the launch overhead benefits aren’t fully seen in the time to work completion.

Instead, you see a more modest improvement of about 60ns/node benefit in time to solution for straight line graphs.

Scheduling based on node creation order
---------------------------------------

In CUDA Toolkit 12.0, CUDA started paying attention to node creation order as a heuristic to scheduling decisions. CUDA started loosely preferring to schedule node operations for nodes created earlier over nodes created later.

Specifically, the order of the root nodes was fixed and a pass that ordered the nodes by a breadth-first traversal was removed. The logic originally was designed to keep root nodes at the start of the graph’s node list and to prioritize making parallel work available for the GPU to run.

In cases where memcopy operations would be assigned to the same copy engine and falsely serialize, the old heuristic would often wind up unintentionally serializing the memcopy operations needed for the first compute items after memcopy operations that were only needed for later compute tasks. This caused the graph to have bubbles in the work available to the compute engines. So, developers often created the graph nodes in a deliberate order: nodes needed early in the graph were created first and nodes needed later were created later.

By paying attention to node creation order when making some scheduling decisions, CUDA can give you scheduling that better matches your intuitive expectations.

Microbenchmark source code
--------------------------

The microbenchmarks code has been added to the [/NVIDIA/cuda-samples](https://github.com/NVIDIA/cuda-samples/tree/master/Samples/6_Performance/cudaGraphsPerfScaling) as the `cudaGraphsPerfScaling` application. Compile the app and run the `dataCollection.bash` script with your GPU name and CUDA driver version to generate the data.

Summary
=======

New optimizations to CUDA graphs make a compelling argument for seeing if they translate to real performance improvements in applications using graphs.

In this post, we discussed CUDA Graphs performance improvements after CUDA 11.8. These improvements include enhancements in graph construction and instantiation time, CPU launch overhead, and overall performance in various scenarios.

For more information, see the following resources:

* [Getting Started with CUDA Graphs](https://developer.nvidia.com/blog/cuda-graphs/)
* [CUDA C++ Programming Guide: CUDA Graphs](https://docs.nvidia.com/cuda/cuda-programming-guide/04-special-topics/cuda-graphs.html)
  + [Whole Graph Update](https://docs.nvidia.com/cuda/cuda-programming-guide/04-special-topics/cuda-graphs.html#whole-graph-update)
  + [Individual Node Update](https://docs.nvidia.com/cuda/cuda-programming-guide/04-special-topics/cuda-graphs.html#individual-node-update)
* [CUDA Runtime API: Graph Management](https://docs.nvidia.com/cuda/cuda-runtime-api/group__CUDART__GRAPH.html)
* [A Guide to CUDA Graphs in GROMACS](https://developer.nvidia.com/blog/a-guide-to-cuda-graphs-in-gromacs-2023/)
* [Employing CUDA Graphs in a Dynamic Environment](https://developer.nvidia.com/blog/employing-cuda-graphs-in-a-dynamic-environment/)
* [Constructing CUDA Graphs with Dynamic Parameters](https://developer.nvidia.com/blog/constructing-cuda-graphs-with-dynamic-parameters/)
* [CUDA Graphs (OLCF presentation)](https://www.olcf.ornl.gov/wp-content/uploads/2021/10/013_CUDA_Graphs.pdf)
* [Accelerating PyTorch with CUDA Graphs](https://pytorch.org/blog/accelerating-pytorch-with-cuda-graphs/)

[Discuss (1)](#entry-content-comments)

Like

Tags
----

[Agentic AI / Generative AI](https://developer.nvidia.com/blog/category/generative-ai/) | [HPC / Scientific Computing](https://developer.nvidia.com/blog/recent-posts/?industry=HPC+%2F+Scientific+Computing) | [CUDA](https://developer.nvidia.com/blog/recent-posts/?products=CUDA) | [HPC SDK](https://developer.nvidia.com/blog/recent-posts/?products=HPC+SDK) | [Intermediate Technical](https://developer.nvidia.com/blog/recent-posts/?learning_levels=Intermediate+Technical) | [Deep dive](https://developer.nvidia.com/blog/recent-posts/?content_types=Deep+dive) | [featured](https://developer.nvidia.com/blog/tag/featured/) | [Graph Algorithms](https://developer.nvidia.com/blog/tag/graph-algorithms/) | [Performance Optimization](https://developer.nvidia.com/blog/tag/optimization/) | [Supercomputing](https://developer.nvidia.com/blog/tag/cluster-supercomputing/)

About the Authors
-----------------

![Houston Hoffman](https://developer-blogs.nvidia.com/wp-content/uploads/2024/05/cropped-houston-hoffman-131x131.png)![Houston Hoffman](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20131%20131%22%3E%3C/svg%3E)

**About Houston Hoffman**
  
Houston Hoffman is a senior software engineer on the CUDA Driver team at NVIDIA focused on high performance work dispatch mechanisms, like CUDA Graphs. He holds a master’s degree in Wireless Embedded Systems from UCSD and a bachelor’s degree in computer engineering from Cal Poly.

[View all posts by Houston Hoffman](https://developer.nvidia.com/blog/author/hhoffman/)

![Avatar photo](https://developer-blogs.nvidia.com/wp-content/uploads/2021/06/Fred_Oh_A_sq_med-131x131.jpg)![Avatar photo](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20131%20131%22%3E%3C/svg%3E)

**About Fred Oh**
  
Fred is a senior product marketing manager for CUDA, CUDA on WSL, and CUDA Python. Fred has a B.S. in Computer Science and Math from UC Davis. He began his career as a UNIX software engineer porting kernel services and device drivers to x86 architectures. He loves Star Wars, Star Trek and the NBA Warriors.

[View all posts by Fred Oh](https://developer.nvidia.com/blog/author/fredoh/)

Comments
--------

Related posts
-------------

![Graphic of device-side work scheduler](https://developer-blogs.nvidia.com/wp-content/uploads/2022/12/image2-1.png)![Graphic of device-side work scheduler](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20658%20370%22%3E%3C/svg%3E)

### Enabling Dynamic Control Flow in CUDA Graphs with Device Graph Launch

[Enabling Dynamic Control Flow in CUDA Graphs with Device Graph Launch](https://developer.nvidia.com/blog/enabling-dynamic-control-flow-in-cuda-graphs-with-device-graph-launch/)

![](https://developer-blogs.nvidia.com/wp-content/uploads/2022/08/cuda-graphs-dynamic-parameters-1.png)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20658%20370%22%3E%3C/svg%3E)

### Constructing CUDA Graphs with Dynamic Parameters

[Constructing CUDA Graphs with Dynamic Parameters](https://developer.nvidia.com/blog/constructing-cuda-graphs-with-dynamic-parameters/)

![](https://developer-blogs.nvidia.com/wp-content/uploads/2021/06/CUDA-toolkit-featured.png)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20617%20370%22%3E%3C/svg%3E)

### Discovering New Features in CUDA 11.4

[Discovering New Features in CUDA 11.4](https://developer.nvidia.com/blog/discovering-new-features-in-cuda-11-4/)

![](https://developer-blogs.nvidia.com/wp-content/uploads/2021/04/CUDA-Blog-Image-1000x600-1.jpg)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20617%20370%22%3E%3C/svg%3E)

### Exploring the New Features of CUDA 11.3

[Exploring the New Features of CUDA 11.3](https://developer.nvidia.com/blog/exploring-the-new-features-of-cuda-11-3/)

![](https://developer-blogs.nvidia.com/wp-content/uploads/2019/09/CUDA-Graphs.png)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20273%20370%22%3E%3C/svg%3E)

### Getting Started with CUDA Graphs

[Getting Started with CUDA Graphs](https://developer.nvidia.com/blog/cuda-graphs/)

Related posts
-------------

![Observatory dome beneath curved star trails in the night sky.](https://developer-blogs.nvidia.com/wp-content/uploads/2026/10/Satellite-Stars-e1791320112446-660x370.webp)![Observatory dome beneath curved star trails in the night sky.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20660%20370%22%3E%3C/svg%3E)

### Faster Scientific Image Analysis with NVIDIA cuPhoton

[Faster Scientific Image Analysis with NVIDIA cuPhoton](https://developer.nvidia.com/blog/faster-scientific-image-analysis-with-nvidia-cuphoton/)

![](https://developer-blogs.nvidia.com/wp-content/uploads/2025/11/image3-660x370-jpg.webp)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20660%20370%22%3E%3C/svg%3E)

### Making GPU Clusters More Efficient with NVIDIA Data Center Monitoring Tools

[Making GPU Clusters More Efficient with NVIDIA Data Center Monitoring Tools](https://developer.nvidia.com/blog/making-gpu-clusters-more-efficient-with-nvidia-data-center-monitoring/)

![A decorative image.](https://developer-blogs.nvidia.com/wp-content/uploads/2025/08/Faster-modeling-660x370-jpg.webp)![A decorative image.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20660%20370%22%3E%3C/svg%3E)

### Less Coding, More Science: Simplify Ocean Modeling on GPUs With OpenACC and Unified Memory

[Less Coding, More Science: Simplify Ocean Modeling on GPUs With OpenACC and Unified Memory](https://developer.nvidia.com/blog/less-coding-more-science-simplify-ocean-modeling-on-gpus-with-openacc-and-unified-memory/)

![](https://developer-blogs.nvidia.com/wp-content/uploads/2025/04/composite-images.png)![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20600%20338%22%3E%3C/svg%3E)

### NVIDIA cuPyNumeric 25.03 Now Fully Open Source with PIP and HDF5 Support

[NVIDIA cuPyNumeric 25.03 Now Fully Open Source with PIP and HDF5 Support](https://developer.nvidia.com/blog/nvidia-cupynumeric-25-03-now-fully-open-source-with-pip-and-hdf5-support/)

![Chart of algorithm workflow.](https://developer-blogs.nvidia.com/wp-content/uploads/2024/09/quantum-algorithm-gpt-featured-2-660x370.png)![Chart of algorithm workflow.](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20660%20370%22%3E%3C/svg%3E)

### Advancing Quantum Algorithm Design with GPTs

[Advancing Quantum Algorithm Design with GPTs](https://developer.nvidia.com/blog/advancing-quantum-algorithm-design-with-gpt/)

Close


Previous

![]()![](data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%20210%20140%22%3E%3C/svg%3E)

![]()![]()

Next

* [L](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fconstant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements%2F)
* [T](https://twitter.com/intent/tweet?text=Constant+Time+Launch+for+Straight-Line+CUDA+Graphs+and+Other+Performance+Enhancements+%7C+NVIDIA+Technical+Blog+https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fconstant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements%2F)
* [F](https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fconstant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements%2F)
* [R](https://www.reddit.com/submit?url=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fconstant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements%2F&title=Constant+Time+Launch+for+Straight-Line+CUDA+Graphs+and+Other+Performance+Enhancements+%7C+NVIDIA+Technical+Blog)
* [E](mailto:?subject=I'd%20like%20to%20share%20a%20link%20with%20you&body=https%3A%2F%2Fdeveloper.nvidia.com%2Fblog%2Fconstant-time-launch-for-straight-line-cuda-graphs-and-other-performance-enhancements%2F)

![](https://developer.nvidia.com/akam/13/pixel_4b3bf5?a=dD1kNGRiZjMxMDJkZTExMDAwZGY5OTgwOGJjMGMwOTNmMTI0Mjk5MDAxJmpzPW9mZg==)