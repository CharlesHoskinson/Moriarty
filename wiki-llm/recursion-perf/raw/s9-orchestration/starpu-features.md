# StarPU runtime features
> Source: https://starpu.gitlabpages.inria.fr/features.html
> Collected: 2026-10-26
> Published: Unknown

Features | StarPU
[StarPU](index.html)




* [Home](/index.html)
* [Features](/features.html)
* [Timeline](/timeline.html)
* [News](/news.html)
* [Publications](/publications.html)
* [Contributors](/contributors.html)
* [Download](/download.html)
* [Documentation](#)

  [Quick Start Guide](/quickGuide.html)
  [Full documentation](/doc.html)
  [Contributing](/involved.html)
  [Tutorials](/tutorials.html)
* [Ressources](#)

  [Software](/software.html)
  [Task Graph Market](/market.html)
  [Benchmarks](https://starpu.gitlabpages.inria.fr/benchmarks/)
* [Repositories](#)

  [GitLab](https://gitlab.inria.fr/starpu/starpu)
  [GitHub mirror](https://github.com/starpu-runtime/starpu)
* [Help](/help.html)
* [Intranet](https://gitlab.inria.fr/starpu/starpu-intranet/-/wikis/home)

Features
========

A good overview is available in the following
[Research Report ‘StarPU: a Runtime System for Scheduling Tasks over Accelerator-Based Multicore Machines’](./publications.html#AugThiNamWac10RR7240).

Portability
-----------

Portability is obtained by the means of a
[unified abstraction of the machine](https://files.inria.fr/starpu/doc/html/Glossary.html).
StarPU offers a unified offloadable task abstraction named [*codelet*](https://files.inria.fr/starpu/doc/html/BasicExamples.html#DefiningACodelet).
Rather than rewriting the entire code, programmers can encapsulate existing functions
within codelets. In case a codelet can run on heterogeneous architectures,
[**it is possible to specify one function for each architecture**](https://files.inria.fr/starpu/doc/html/BasicExamples.html#VectorScalingOnAnHybridCPUGPUMachine)
(e.g. one function for CUDA and one function for CPUs). StarPU takes care of
[scheduling and executing those codelets as efficiently as possible] (https://inria.hal.science/inria-00523937v1)
over the entire machine, including multiple GPUs, even of different brands and interface at the same time (e.g. CUDA + OpenCL).
One can even specify
[**several functions for each architecture**](https://files.inria.fr/starpu/doc/html/AdvancedTasksInStarPU.html#UsingMultipleImplementationsOfACodelet)
(new in v1.0) as well as
[**parallel implementations**](https://files.inria.fr/starpu/doc/html/ParallelWorker.html)
(e.g. in OpenMP), and StarPU will
automatically determine which version is best for each input size (new in v0.9).
StarPU can execute them concurrently, e.g. one per socket, provided that the
[task implementations support it](https://files.inria.fr/starpu/doc/html/ParallelWorker.html#ExampleOfConstrainingOpenMP) (which is the case for MKL, but unfortunately
most often not for OpenMP).

Genericity
----------

The StarPU programming interface is very generic. For instance, various
[data structures](https://files.inria.fr/starpu/doc/html/DataManagement.html#DataInterface)
are supported mainline (vectors, dense matrices, CSR/BCSR/COO sparse matrices, …),
but [application-specific data structures are also supported](https://files.inria.fr/starpu/doc/html/AdvancedDataManagement.html#DefiningANewDataInterface), provided that
the application describes how data is to be transferred (e.g. a series of
contiguous blocks). That was for instance used for hierarchically-compressed
matrices (h-matrices).

Data Transfers
--------------

To relieve programmers from the burden of explicit data transfers, a high-level
data management library enforces memory coherency over the machine: before a
codelet starts (e.g. on an accelerator), all its **data are automatically made
available on the compute resource**. Data are also kept on e.g. GPUs as long as
they are needed for further tasks. When a device runs out of memory, StarPU uses
an LRU strategy to **evict unused data**. StarPU also takes care of **automatically
prefetching** data, which thus permits to **overlap data transfers with computations**
(including **GPU-GPU direct transfers**) to achieve the most of the architecture.
Support is included for for matrices, vectors, n-dimension tensors, compressed
matrices (CSR, BCSR, COO), and the application can define its own data
interface for arbitrary data representation (that can even be e.g. a C++ or Python object).

Asynchronism
------------

StarPU behaves completely asynchronously, provided that the application-provided kernels are properly using e.g.
[the non-default kernel stream](https://files.inria.fr/starpu/doc/html/CheckListWhenPerformanceAreNotThere.html#CUDA-specificOptimizations)
provided by StarPU. Data transfers and computation kernels are submitted to GPUs
in a pipeline and completion is polled to keep the pipeline flowing and never
let GPUs idle.

Dependencies
------------

Dependencies between tasks can be given either of several ways, to provide the
programmer with the best flexibility:

* [**implicitly**](https://files.inria.fr/starpu/doc/html/AdvancedTasksInStarPU.html#SequentialConsistency) from RAW, WAW, and WAR data dependencies.
* [explicitly through **tags**](https://files.inria.fr/starpu/doc/html/AdvancedTasksInStarPU.html#TasksAndTagsDependencies) which act as rendez-vous points between tasks (thus including tasks which have not been created yet),
* [**explicitly**](https://files.inria.fr/starpu/doc/html/AdvancedTasksInStarPU.html#TasksAndTagsDependencies) between pairs of tasks,

These dependencies are computed in a completely decentralized way, and can be
introduced completely dynamically as tasks get submitted by the application
while tasks previously submitted are being executed.

StarPU also supports an [OpenMP-like reduction](https://files.inria.fr/starpu/doc/html/DataManagement.html#DataReduction)
access mode (new in v0.9).

It also supports a [commute access mode](https://files.inria.fr/starpu/doc/html/DataManagement.html#DataCommute)
to allow data access commutativity (new in v1.2).

It also supports transparent dependencies tracking between hierarchical sub-pieces of data
through [asynchronous partitioning](https://files.inria.fr/starpu/doc/html/DataManagement.html#AsynchronousPartitioning),
allowing seamless concurrent read access to different partitioning levels and even different partitioning schemes for the same data (new in v1.3).

Heterogeneous Scheduling
------------------------

StarPU obtains
portable performances by efficiently (and easily) using all computing resources
at the same time. StarPU also takes advantage of the **heterogeneous** nature of a
machine, for instance by using [scheduling strategies](https://files.inria.fr/starpu/doc/html/Scheduling.html#TaskSchedulingPolicy)
based on [auto-tuned performance models](https://files.inria.fr/starpu/doc/html/CheckListWhenPerformanceAreNotThere.html#PerformanceModelCalibration).
These determine the relative performance achieved
by the different processing units for the various kinds of task,
including for [irregular tasks](https://inria.hal.science/hal-01474556v1)
, and thus
permits to **automatically let processing units execute the tasks they are the best for**.
[Various strategies and variants are available](https://files.inria.fr/starpu/doc/html/Scheduling.html#TaskSchedulingPolicy).
Some of them are centralized, but
most of them are **completely distributed** to properly scale with
large numbers of cores. /dmdas/ (a data-locality-aware MCT strategy,
thus similar to heft but starts executing tasks before the whole task graph is
submitted, thus allowing dynamic task submission and a decentralized scheduler,
as well as an energy optimizing extension), /eager/ (dumb centralized
queue), /lws/ (distributed locality-aware work-stealing), …
The overhead per task is typically around the order of
magnitude of a microsecond. Tasks should thus be a few orders of magnitude
bigger, [such as 100 microseconds or 1 millisecond](https://files.inria.fr/starpu/doc/html/CheckListWhenPerformanceAreNotThere.html#CheckTaskSize),
to make the overhead negligible. The application can also provide
[its own scheduler](https://files.inria.fr/starpu/doc/html/HowToDefineANewSchedulingPolicy.html),
to better fit its own needs.

Data Allocation
---------------

StarPU optimizes data allocation in main memory as well as in accelerator
memory. It features an allocation cache for objects of the same size, and a
suballocator for objects with a small size. This avoids repeated expensive
allocation/release operations (on-accelerator allocation or main-memory
pinning).

Clusters
--------

To deal with clusters, StarPU can nicely [integrate with MPI](https://files.inria.fr/starpu/doc/html/MPISupport.html), through explicit or implicit
support, according to the application’s preference.

* [Explicit network communication](https://files.inria.fr/starpu/doc/html/MPISupport.html#PointToPointCommunication)
  requests can be emitted, which will
  then be **automatically combined and overlapped** with the intra-node
  data transfers and computation,
* The application can also just provide the whole task graph, a data
  distribution over MPI nodes, and StarPU will automatically determine
  which MPI node should execute which task, and
  [**automatically generate all required MPI communications**](https://files.inria.fr/starpu/doc/html/MPISupport.html#MPIInsertTaskUtility)
  accordingly (new in v0.9).
  We have gotten [excellent scaling on a 256-node cluster with GPUs](https://inria.hal.science/hal-01618526v1), we
  have not yet had the opportunity to test on a yet larger cluster. We
  have however measured that with naive task submission, it should
  scale to a thousand nodes, and with [pruning-tuned task submission](https://files.inria.fr/starpu/doc/html/MPISupport.html#MPIInsertPruning),
  it should scale to about a **million nodes**.
* The data distribution provided by the application can be changed on the fly
  during the submission according to partial execution results.
* Starting with v1.3, the application can also choose to just provide the whole
  task graph, and let StarPU decide the data distribution and task
  distribution dynamically, thanks to the
  [MPI master-slave](https://files.inria.fr/starpu/doc/html/MPISupport.html#MPIMasterSlave)
  and [TCP/IP master-slave](https://files.inria.fr/starpu/doc/html/TCPIPSupport.html)
  mechanisms. This will however by nature have a more limited scalability than the fully distributed
  paradigm mentioned above.
* Starting with v1.3, StarPU can leverage the NewMadeleine communication library
  and its fully-threaded progression support.
* Starting with v1.4 and in the NewMadeleine network support case, StarPU
  automatically detects data broadcasts performed by the application, and
  [**automatically optimizes the broadcast**](https://inria.hal.science/hal-02872765v1).

Out of Core
-----------

When memory is not big enough for the working set, one may have to resort to
using disks. StarPU makes this seamless thanks to its [out of core support](https://files.inria.fr/starpu/doc/html/OutOfCore.html) (new in v1.2).
StarPU will **automatically evict** data from the main memory in advance, and
**prefetch back** required data before it is needed for tasks.

Fortran Interface
-----------------

StarPU comes with native [Fortran](https://files.inria.fr/starpu/doc/html/NativeFortranSupport.html) bindings and examples.

OpenMP 4 -compatible Interface
------------------------------

[K’Star](http://kstar.gitlabpages.inria.fr/) provides an OpenMP
4 -compatible interface on top of StarPU. This allows to just rebuild OpenMP
applications with the K’Star source-to-source compiler, then build it with the
usual compiler, and the result will use the StarPU runtime.

K’Star also provides some extensions to the OpenMP 4 standard, to let the
StarPU runtime perform online optimizations.

OpenCL-compatible Interface
---------------------------

StarPU provides an [OpenCL-compatible interface, SOCL](https://files.inria.fr/starpu/doc/html/SOCLOpenclExtensions.html)
which allows to simply run OpenCL applications on top of StarPU (new in v1.0).

Simulation Support
------------------

StarPU can very accurately [simulate an application execution](https://files.inria.fr/starpu/doc/html/SimGridSupport.html)
and measure the resulting performance thanks to using the
[SimGrid simulator](http://simgrid.gforge.inria.fr) (new in v1.1). This allows
to quickly experiment with various scheduling heuristics, various application
algorithms, and even various platforms (available GPUs and CPUs, available
bandwidth)!

All in All
----------

All that means that the following sequential source code of a
[tiled version of the classical Cholesky factorization algorithm](https://inria.hal.science/inria-00547847v1)
using BLAS is also (an almost) valid StarPU code, possibly running on all the CPUs and GPUs, and given a data
distribution over MPI nodes, it is even a distributed version!

```
for (k = 0; k < tiles; k++) {
  potrf(A[k,k])
  for (m = k+1; m < tiles; m++)
    trsm(A[k,k], A[m,k])
  for (m = k+1; m < tiles; m++)
    syrk(A[m,k], A[m, m])
  for (m = k+1, m < tiles; m++)
    for (n = k+1, n < m; n++)
      gemm(A[m,k], A[n,k], A[m,n])
}
```

The complete example can be found inside the StarPU source,
[examples/cholesky/cholesky\_implicit.c](https://gitlab.inria.fr/starpu/starpu/-/tree/master/examples/cholesky)
for single-node, and
[mpi/examples/matrix\_decomposition/mpi\_cholesky\_codelets.c](https://gitlab.inria.fr/starpu/starpu/-/tree/master/mpi/examples/matrix_decomposition)
for distributed execution.

And Even More
-------------

StarPU provides a [Python interface](https://files.inria.fr/starpu/doc/html/PythonInterface.html)
(new in v1.4), a [Java interface](https://files.inria.fr/starpu/doc/html/StarPUJavaInterface.html)
(new in v1.4), an [Eclipse plugin](https://files.inria.fr/starpu/doc/html/OfflinePerformanceTools.html#EclipsePlugin) (new in v1.4)

Technical Details
-----------------

### Supported Architectures

Arbitrary mixtures of the following architectures are supported (except CUDA+HIP, just because [ROCm refuses to support it](https://github.com/ROCm/HIP/issues/2703)):

* SMP/Multicore Processors (x86, PPC, ARM, RiscV, … all Debian architecture have been tested)
* CUDA NVIDIA GPUs (e.g. heterogeneous multi-GPU), with pipelined and concurrent kernel execution support (new in v1.2), GPU-GPU direct transfers (new in v1.1), and GPU-NIC direct transfers (new in v1.4)
* HIP AMD and NVIDIA GPUs (new in v1.4)
* OpenCL devices
* Maxeler FPGA (new in v1.4)
* Intel SCC (experimental, new in v1.2, removed in v1.3)
* Intel MIC / Xeon Phi (new in v1.2, removed in v1.4)

### Supported Operating Systems

* GNU/Linux
* Mac OS X
* Windows

### Stability

StarPU is checked every night with

* Valgrind / Helgrind
* gcc’ Address/Leak/Thread/Undefined Sanitizers
* cppcheck
* Coverity

Performance Analysis Tools
--------------------------

### ViTE

In order to understand the performance obtained by StarPU, it is helpful to
[visualize the actual behavior](https://files.inria.fr/starpu/doc/html/OfflinePerformanceTools.html#GeneratingTracesWithFxT) of the applications running on complex
heterogeneous multicore architectures. StarPU therefore makes it possible to
generate Pajé traces that can be visualized thanks to the
[**ViTE** (Visual Trace Explorer) open source tool](http://vite.gforge.inria.fr/).

**Example:** LU decomposition on 3 CPU cores and a GPU using a very simple
greedy scheduling strategy. The green (resp. red) sections indicate when the
corresponding processing unit is busy (resp. idle). The number of ready tasks
is displayed in the curve on top: it appears that with this scheduling policy,
the algorithm suffers a certain lack of parallelism. **Measured speed: 175.32 GFlop/s**

![](/images/greedy-lu-16k-fx5800.png)

This second trace depicts the behavior of the same application using a
scheduling strategy (dmda) trying to minimize load imbalance thanks to auto-tuned
performance models and to keep data locality as high as possible. In this
example, the Pajé trace clearly shows that this scheduling strategy outperforms
the previous one in terms of processor usage. *Measured speed: 239.60 GFlop/s*

![](/images/dmda-lu-16k-fx5800.png)

### Temanejo

[Temanejo](http://www.hlrs.de/temanejo) can be used to [debug the task graph](https://files.inria.fr/starpu/doc/html/DebuggingTools.html#UsingTheTemanejoTaskDebugger), as shown below (new in v1.1).

![](/images/temanejo.png)

### StarVZ

[StarVZ](https://github.com/schnorr/starvz) ([paper](https://hal.inria.fr/hal-02960848)) can be used to investigate performance, as shown below

![](/images/starvz.png)