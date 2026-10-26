# NVIDIA CUB DeviceScan source
> Source: https://raw.githubusercontent.com/NVIDIA/cccl/main/cub/cub/device/device_scan.cuh
> Collected: 2026-10-26
> Published: Unknown

// SPDX-FileCopyrightText: Copyright (c) 2011, Duane Merrill. All rights reserved.
// SPDX-FileCopyrightText: Copyright (c) 2011-2026, NVIDIA CORPORATION. All rights reserved.
// SPDX-License-Identifier: BSD-3
//! @file
//! cub::DeviceScan provides device-wide, parallel operations for computing a prefix scan across a sequence of data
//! items residing within device-accessible memory.
#pragma once
#include 
#ifndef CCCL\_DISABLE\_NVRTC\_COMPATIBILITY\_CHECK
# if \_CCCL\_COMPILER(NVRTC)
# error \
"Including  is not supported when compiling with NVRTC. Include block-, warp-, or thread-level primitives instead (e.g. ). You can define CCCL\_DISABLE\_NVRTC\_COMPATIBILITY\_CHECK to disable this warning."
# endif // \_CCCL\_COMPILER(NVRTC)
#endif // CCCL\_DISABLE\_NVRTC\_COMPATIBILITY\_CHECK
#if defined(\_CCCL\_IMPLICIT\_SYSTEM\_HEADER\_GCC)
# pragma GCC system\_header
#elif defined(\_CCCL\_IMPLICIT\_SYSTEM\_HEADER\_CLANG)
# pragma clang system\_header
#elif defined(\_CCCL\_IMPLICIT\_SYSTEM\_HEADER\_MSVC)
# pragma system\_header
#endif // no system header
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
CUB\_NAMESPACE\_BEGIN
namespace detail
{
template 
inline constexpr bool is\_single\_value\_wrapper\_v = false;
template 
inline constexpr bool is\_single\_value\_wrapper\_v<::cuda::args::immediate> = true;
template 
inline constexpr bool is\_single\_value\_wrapper\_v<::cuda::args::constant> = true;
template 
inline constexpr bool is\_single\_value\_wrapper\_v<::cuda::args::deferred> = true;
// Classify argument roles here; validate the wrapped count type after selecting the overload.
template 
inline constexpr bool is\_num\_items\_v = ::cuda::std::is\_integral\_v || is\_single\_value\_wrapper\_v;
} // namespace detail
//! @rst
//! DeviceScan provides device-wide, parallel operations for computing a
//! prefix scan across a sequence of data items residing within
//! device-accessible memory.
//!
//! Overview
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! Given a sequence of input elements and a binary reduction operator, a
//! `prefix scan `\_ produces an output
//! sequence where each element is computed to be the reduction of the elements
//! occurring earlier in the input sequence. \*Prefix sum\* connotes a prefix scan
//! with the addition operator. The term \*inclusive\* indicates that the
//! \*i\*\ :sup:`th` output reduction incorporates the \*i\*\ :sup:`th` input.
//! The term \*exclusive\* indicates the \*i\*\ :sup:`th` input is not
//! incorporated into the \*i\*\ :sup:`th` output reduction. When the input and
//! output sequences are the same, the scan is performed in-place.
//!
//! In order to provide an efficient parallel implementation, the binary reduction operator must be associative. That
//! is, ``op(op(a, b), c)`` must be equivalent to ``op(a, op(b, c))`` for any input values ``a``, ``b``, and ``c``.
//!
//! As of CUB 1.0.1 (2013), CUB's device-wide scan APIs have implemented our
//! \*"decoupled look-back"\* algorithm for performing global prefix scan with
//! only a single pass through the input data, as described in our 2016 technical
//! report [1]\_. The central idea is to leverage a small, constant factor of
//! redundant work in order to overlap the latencies of global prefix
//! propagation with local computation. As such, our algorithm requires only
//! ``~2\*n\*`` data movement (``n`` inputs are read, ``n`` outputs are written), and
//! typically proceeds at "memcpy" speeds. Our algorithm supports inplace operations.
//!
//! .. [1] Duane Merrill and Michael Garland. `Single-pass Parallel Prefix Scan with Decoupled Look-back
//! `\_,
//! \*NVIDIA Technical Report NVR-2016-002\*, 2016.
//!
//! Usage Considerations
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! @cdp\_class{DeviceScan}
//! @determinism{not\_guaranteed}
//!
//! The problem size must be nonnegative, whether provided directly or through a deferred argument.
//!
//! Deferred problem sizes
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! ``ExclusiveSum``, ``ExclusiveScan``, ``InclusiveSum``, ``InclusiveScan``, and ``InclusiveScanInit`` allow
//! specifying the problem size from a value that resides in device memory through a single-value
//! ``cuda::args::deferred`` argument. The deferred source may be a device pointer, a one-element span, or a
//! random-access fancy iterator whose element is a 32- or 64-bit integer.
//!
//! The problem size is read in stream order by the scan kernels. Work that produces the count in the same stream is
//! ordered automatically; a producer in another stream requires an event or an equivalent dependency. The source and
//! its value must remain accessible and unchanged until all scan kernels complete. The value must be nonnegative and
//! ``[d\_in, d\_in + num\_items)`` must be accessible. A temporary-storage query does not dereference the deferred
//! source.
//!
//! Scans with deferred problem sizes are CUDA Graph capturable. The pointed-to count may change between graph
//! replays without updating or recapturing the graph.
//!
//! A deferred problem size currently needs to run the decoupled look-back algorithm. Default tuning selects look-back
//! automatically. A custom tuning policy must select look-back and provide valid look-back tuning parameters;
//! selecting lookahead with a deferred problem size is rejected.
//!
//! The :ref:`determinism guarantees ` below apply unchanged to a deferred problem size. Note that
//! reproducibility is promised across launches of the \*same\* configuration: for a non-associative operator such as
//! floating-point addition, a scan with a deferred problem size need not produce the same result as a scan with an
//! immediate problem size over the same values, because the two group the reduction differently.
//!
//! Since the problem size is not available to the host, temporary storage and grid dimensions are derived from the
//! upper bound of the deferred argument. Compile-time and runtime bounds are accepted as caller preconditions; a
//! bound tight enough to keep the worst case within a single batch of tiles lets CUB size the launch exactly, which
//! makes a scan with a deferred problem size as cheap as one with an immediate problem size. Without such a bound,
//! CUB falls back to a reusable-batch look-back that claims tiles from a persistent grid, which keeps temporary storage
//! at a fixed size and costs a small constant overhead over a launch with an immediate problem size.
//!
//! Determinism
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! ``cub::DeviceScan`` supports the :ref:`determinism guarantees ` as follows; the
//! default is ``not\_guaranteed``.
//!
//! - ``run\_to\_run`` is supported for integral types with a known CUDA binary operator, and for floating-point
//! types with ``cuda::std::plus``. The floating-point ``plus`` case engages a stable, fixed reduction order so
//! results are reproducible across runs on the same GPU.
//! - ``gpu\_to\_gpu`` is supported for integral types with a known CUDA binary operator. (These are exactly
//! associative, so the result is already identical across GPUs.)
//! - Other combinations under ``run\_to\_run``/``gpu\_to\_gpu`` are rejected at compile time.
//!
//! Performance
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! @linear\_performance{prefix scan}
//!
//! Tuning
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! All non-ByKey algorithms in DeviceScan that accept an environment can be tuned by passing a custom :ref:`policy
//! selector ` that returns a :cpp:struct:`cub::ScanPolicy`, as shown in the example below:
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-sum-policy-selector
//! :end-before: example-end exclusive-sum-policy-selector
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-sum-tuning
//! :end-before: example-end exclusive-sum-tuning
//!
//! All ByKey algorithms in DeviceScan that accept an environment can be tuned by passing a custom :ref:`policy
//! selector ` that returns a :cpp:struct:`cub::ScanByKeyPolicy`, as shown in the example below:
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_by\_key\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-sum-by-key-policy-selector
//! :end-before: example-end exclusive-sum-by-key-policy-selector
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_by\_key\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-sum-by-key-tuning
//! :end-before: example-end exclusive-sum-by-key-tuning
//!
//! @endrst
struct DeviceScan
{
private:
//! @cond
template 
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t scan\_impl\_determinism(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
InputIteratorT d\_in,
OutputIteratorT d\_out,
ScanOpT scan\_op,
InitValueT init,
NumItemsT num\_items,
cudaStream\_t stream,
PolicySelectorT policy\_selector)
{
return detail::scan::dispatch(
d\_temp\_storage,
temp\_storage\_bytes,
d\_in,
d\_out,
scan\_op,
init,
detail::make\_num\_items\_dispatch\_arg(num\_items),
stream,
policy\_selector);
}
template 
CUB\_RUNTIME\_FUNCTION \_CCCL\_FORCEINLINE static cudaError\_t scan\_impl\_env(
InputIteratorT d\_in, OutputIteratorT d\_out, ScanOpT scan\_op, InitValueT init, NumItemsT num\_items, const EnvT& env)
{
static\_assert(!::cuda::std::execution::\_\_queryable\_with,
"Determinism should be used inside requires to have an effect.");
using requirements\_t = ::cuda::std::execution::
\_\_query\_result\_or\_t>;
using requested\_determinism\_t =
::cuda::std::execution::\_\_query\_result\_or\_t;
using accum\_t =
::cuda::std::\_\_accumulator\_t,
::cuda::std::\_If<::cuda::std::is\_same\_v,
cub::detail::it\_value\_t,
typename InitValueT::value\_type>>;
using offset\_t = detail::choose\_offset\_t::element\_type>;
constexpr bool is\_run\_to\_run\_required =
::cuda::std::is\_same\_v;
constexpr bool is\_gpu\_to\_gpu\_required =
::cuda::std::is\_same\_v;
constexpr bool is\_safe\_integral\_op =
::cuda::std::is\_integral\_v && detail::is\_cuda\_binary\_operator;
constexpr bool is\_fp\_plus\_op =
::cuda::std::is\_floating\_point\_v && ::cuda::\_\_is\_cuda\_std\_plus\_v;
// run\_to\_run determinism is supported only with integral types with known operators, or floating-point types with
// plus operator
static\_assert(!is\_run\_to\_run\_required || is\_safe\_integral\_op || is\_fp\_plus\_op,
"run\_to\_run deterministic scan requires either integral types with known operators, "
"or floating-point types with plus operator");
// gpu\_to\_gpu determinism is only supported with integral types with known operators
static\_assert(!is\_gpu\_to\_gpu\_required || is\_safe\_integral\_op,
"gpu\_to\_gpu deterministic scan requires integral types with known operators");
static constexpr bool stable\_reduction\_order = is\_run\_to\_run\_required && is\_fp\_plus\_op;
using default\_policy\_selector\_t = detail::scan::policy\_selector\_from\_types<
InputIteratorT,
OutputIteratorT,
accum\_t,
offset\_t,
ScanOpT,
stable\_reduction\_order,
::cuda::args::\_\_is\_deferred\_v>;
return detail::dispatch\_with\_env\_and\_tuning(
env, [&](auto policy\_selector, void\* storage, size\_t& bytes, auto stream) {
return scan\_impl\_determinism(
storage, bytes, d\_in, d\_out, scan\_op, init, num\_items, stream, policy\_selector);
});
}
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t scan\_by\_key\_impl(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
KeysInputIteratorT d\_keys\_in,
ValuesInputIteratorT d\_values\_in,
ValuesOutputIteratorT d\_values\_out,
EqualityOpT equality\_op,
ScanOpT scan\_op,
InitValueT init\_value,
NumItemsT num\_items,
cudaStream\_t stream)
{
using offset\_t = detail::choose\_offset\_t;
using accum\_t = ::cuda::std::\_\_accumulator\_t<
ScanOpT,
cub::detail::it\_value\_t,
::cuda::std::
\_If<::cuda::std::is\_same\_v, cub::detail::it\_value\_t, InitValueT>>;
using default\_policy\_selector\_t =
detail::scan\_by\_key::policy\_selector\_from\_types,
accum\_t,
cub::detail::it\_value\_t,
ScanOpT>;
using policy\_selector\_t =
::cuda::std::execution::\_\_query\_result\_or\_t;
// we would not need to override the accumulator type, but we must ensure it's the same as for the policy here
return detail::scan\_by\_key::dispatch(
d\_temp\_storage,
temp\_storage\_bytes,
d\_keys\_in,
d\_values\_in,
d\_values\_out,
equality\_op,
scan\_op,
init\_value,
static\_cast(num\_items),
stream,
policy\_selector\_t{});
}
//! @endcond
public:
//! @name Exclusive scans
//! @{
//! @rst
//! Computes a device-wide exclusive prefix sum.
//! The value of ``0`` is applied as the initial value, and is assigned to ``\*d\_out``.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place.
//! The range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix sum of an ``int``
//! device vector.
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_in; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! int \*d\_out; // e.g., [ , , , , , , ]
//! ...
//!
//! // Determine temporary device storage requirements
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::ExclusiveSum(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, num\_items);
//!
//! // Allocate temporary storage
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run exclusive prefix sum
//! cub::DeviceScan::ExclusiveSum(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, num\_items);
//!
//! // d\_out <-- [0, 8, 14, 21, 26, 29, 29]
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveSum(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
InputIteratorT d\_in,
OutputIteratorT d\_out,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::ExclusiveSum");
using init\_value\_t = cub::detail::it\_value\_t;
// Initial value
const init\_value\_t init\_value{};
return detail::scan::dispatch(
d\_temp\_storage,
temp\_storage\_bytes,
d\_in,
d\_out,
::cuda::std::plus<>{},
detail::InputValue(init\_value),
detail::make\_num\_items\_dispatch\_arg(num\_items),
stream);
}
//! @rst
//! Computes a device-wide exclusive prefix sum.
//! The value of ``0`` is applied as the initial value, and is assigned to ``\*d\_out``.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place.
//! The range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates a user-defined exclusive-scan of a
//! device vector of ``float`` data elements.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-sum-env-determinism
//! :end-before: example-end exclusive-sum-env-determinism
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t
ExclusiveSum(InputIteratorT d\_in, OutputIteratorT d\_out, NumItemsT num\_items, const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::ExclusiveSum");
using init\_value\_t = cub::detail::it\_value\_t;
const init\_value\_t init\_value{};
return scan\_impl\_env(
d\_in, d\_out, ::cuda::std::plus<>{}, detail::InputValue(init\_value), num\_items, env);
}
//! @rst
//! Computes a device-wide exclusive prefix sum in-place.
//! The value of ``0`` is applied as the initial value, and is assigned to ``\*d\_data``.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix sum of an ``int``
//! device vector.
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_data; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! ...
//!
//! // Determine temporary device storage requirements
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::ExclusiveSum(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_data, num\_items);
//!
//! // Allocate temporary storage
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run exclusive prefix sum
//! cub::DeviceScan::ExclusiveSum(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_data, num\_items);
//!
//! // d\_data <-- [0, 8, 14, 21, 26, 29, 29]
//!
//! @endrst
//!
//! @tparam IteratorT
//! \*\*[inferred]\*\* Random-access iterator type for reading scan inputs and wrigin scan outputs
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in,out] d\_data
//! Random-access iterator to the sequence of data items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_data`)
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveSum(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
IteratorT d\_data,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
return ExclusiveSum(d\_temp\_storage, temp\_storage\_bytes, d\_data, d\_data, num\_items, stream);
}
//! @rst
//! Computes a device-wide exclusive prefix sum in-place.
//! The value of ``0`` is applied as the initial value, and is assigned to ``\*d\_data``.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! This is an environment-based API that allows customization of:
//!
//! - Stream: Query via ``cuda::get\_stream``
//! - Memory resource: Query via ``cuda::mr::get\_memory\_resource``
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates an in-place exclusive prefix sum of an
//! ``int`` device vector using a stream environment.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-sum-inplace-env
//! :end-before: example-end exclusive-sum-inplace-env
//!
//! @endrst
//!
//! @tparam IteratorT
//! \*\*[inferred]\*\* Random-access iterator type for reading and writing scan data
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in,out] d\_data
//! Random-access iterator to the sequence of data items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_data`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t
ExclusiveSum(IteratorT d\_data, NumItemsT num\_items, const EnvT& env = {})
{
return ExclusiveSum(d\_data, d\_data, num\_items, env);
}
//! @rst
//! Computes a device-wide exclusive prefix scan using the specified
//! binary associative ``scan\_op`` functor. The ``init\_value`` value is applied as
//! the initial value, and is assigned to ``\*d\_out``.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place. The
//! range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix min-scan of an ``int`` device vector
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//! #include  // for INT\_MAX
//!
//! // CustomMin functor
//! struct CustomMin
//! {
//! template 
//! \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
//! T operator()(const T &a, const T &b) const {
//! return (b < a) ? b : a;
//! }
//! };
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_in; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! int \*d\_out; // e.g., [ , , , , , , ]
//! CustomMin min\_op;
//! ...
//!
//! // Determine temporary device storage requirements for exclusive
//! // prefix scan
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::ExclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, min\_op, (int) INT\_MAX, num\_items);
//!
//! // Allocate temporary storage for exclusive prefix scan
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run exclusive prefix min-scan
//! cub::DeviceScan::ExclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, min\_op, (int) INT\_MAX, num\_items);
//!
//! // d\_out <-- [2147483647, 8, 6, 6, 5, 3, 0]
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the `init\_value`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the exclusive scan (and is assigned to `\*d\_out`)
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveScan(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
InputIteratorT d\_in,
OutputIteratorT d\_out,
ScanOpT scan\_op,
InitValueT init\_value,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::ExclusiveScan");
return detail::scan::dispatch(
d\_temp\_storage,
temp\_storage\_bytes,
d\_in,
d\_out,
scan\_op,
detail::InputValue(init\_value),
detail::make\_num\_items\_dispatch\_arg(num\_items),
stream);
}
//! @rst
//! Computes a device-wide exclusive prefix scan using the specified
//! binary associative ``scan\_op`` functor. The ``init\_value`` value is applied as
//! the initial value, and is assigned to ``\*d\_out``.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place. The
//! range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates a user-defined exclusive-scan of a
//! device vector of ``float`` data elements.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-scan-env-determinism
//! :end-before: example-end exclusive-scan-env-determinism
//!
//! The code snippet below illustrates an exclusive-scan using a custom stream
//! and ``not\_guaranteed`` determinism.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-scan-env-stream
//! :end-before: example-end exclusive-scan-env-stream
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the `init\_value`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the exclusive scan (and is assigned to `\*d\_out`)
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveScan(
InputIteratorT d\_in,
OutputIteratorT d\_out,
ScanOpT scan\_op,
InitValueT init\_value,
NumItemsT num\_items,
const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::ExclusiveScan");
return scan\_impl\_env(d\_in, d\_out, scan\_op, detail::InputValue(init\_value), num\_items, env);
}
//! @rst
//! Computes a device-wide exclusive prefix scan using the specified
//! binary associative ``scan\_op`` functor. The ``init\_value`` value is applied as
//! the initial value, and is assigned to ``\*d\_data``.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix min-scan of an
//! ``int`` device vector:
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//! #include  // for INT\_MAX
//!
//! // CustomMin functor
//! struct CustomMin
//! {
//! template 
//! \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
//! T operator()(const T &a, const T &b) const {
//! return (b < a) ? b : a;
//! }
//! };
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_data; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! CustomMin min\_op;
//! ...
//!
//! // Determine temporary device storage requirements for exclusive
//! // prefix scan
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::ExclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_data, min\_op, (int) INT\_MAX, num\_items);
//!
//! // Allocate temporary storage for exclusive prefix scan
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run exclusive prefix min-scan
//! cub::DeviceScan::ExclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_data, min\_op, (int) INT\_MAX, num\_items);
//!
//! // d\_data <-- [2147483647, 8, 6, 6, 5, 3, 0]
//!
//! @endrst
//!
//! @tparam IteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs and writing scan outputs
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the `init\_value`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in,out] d\_data
//! Random-access iterator to the sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the exclusive scan (and is assigned to `\*d\_out`)
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveScan(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
IteratorT d\_data,
ScanOpT scan\_op,
InitValueT init\_value,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
return ExclusiveScan(d\_temp\_storage, temp\_storage\_bytes, d\_data, d\_data, scan\_op, init\_value, num\_items, stream);
}
//! @rst
//! Computes a device-wide exclusive prefix scan in-place using the specified
//! binary associative ``scan\_op`` functor. The ``init\_value`` value is applied as
//! the initial value, and is assigned to ``\*d\_data``.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! This is an environment-based API that allows customization of:
//!
//! - Stream: Query via ``cuda::get\_stream``
//! - Memory resource: Query via ``cuda::mr::get\_memory\_resource``
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates an in-place exclusive prefix scan of an
//! ``int`` device vector with a user-supplied initial value and a stream environment.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-scan-inplace-env
//! :end-before: example-end exclusive-scan-inplace-env
//!
//! @endrst
//!
//! @tparam IteratorT
//! \*\*[inferred]\*\* Random-access iterator type for reading and writing scan data
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the ``init\_value``
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in,out] d\_data
//! Random-access iterator to the sequence of data items
//!
//! @param[in] scan\_op
//! Binary scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the exclusive scan
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_data`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t
ExclusiveScan(IteratorT d\_data, ScanOpT scan\_op, InitValueT init\_value, NumItemsT num\_items, const EnvT& env = {})
{
return ExclusiveScan(d\_data, d\_data, scan\_op, init\_value, num\_items, env);
}
//! @rst
//! Computes a device-wide exclusive prefix scan using the specified
//! binary associative ``scan\_op`` functor. The ``init\_value`` value is provided as a future value.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place.
//! The range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix min-scan of an ``int`` device vector
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//! #include  // for INT\_MAX
//!
//! // CustomMin functor
//! struct CustomMin
//! {
//! template 
//! \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
//! T operator()(const T &a, const T &b) const {
//! return (b < a) ? b : a;
//! }
//! };
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_in; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! int \*d\_out; // e.g., [ , , , , , , ]
//! int \*d\_init\_iter; // e.g., INT\_MAX
//! CustomMin min\_op;
//!
//! auto future\_init\_value =
//! cub::FutureValue(d\_init\_iter);
//!
//! ...
//!
//! // Determine temporary device storage requirements for exclusive
//! // prefix scan
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::ExclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, min\_op, future\_init\_value, num\_items);
//!
//! // Allocate temporary storage for exclusive prefix scan
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run exclusive prefix min-scan
//! cub::DeviceScan::ExclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, min\_op, future\_init\_value, num\_items);
//!
//! // d\_out <-- [2147483647, 8, 6, 6, 5, 3, 0]
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the `init\_value`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in] d\_in
//! Pointer to the input sequence of data items
//!
//! @param[out] d\_out
//! Pointer to the output sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the exclusive scan (and is assigned to `\*d\_out`)
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveScan(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
InputIteratorT d\_in,
OutputIteratorT d\_out,
ScanOpT scan\_op,
FutureValue init\_value,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::ExclusiveScan");
return detail::scan::dispatch(
d\_temp\_storage,
temp\_storage\_bytes,
d\_in,
d\_out,
scan\_op,
detail::InputValue(init\_value),
detail::make\_num\_items\_dispatch\_arg(num\_items),
stream);
}
//! @rst
//! Computes a device-wide exclusive prefix scan using the specified binary associative ``scan\_op`` functor.
//! The ``init\_value`` value is provided as a future value.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix min-scan of an ``int`` device vector
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//! #include  // for INT\_MAX
//!
//! // CustomMin functor
//! struct CustomMin
//! {
//! template 
//! \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
//! T operator()(const T &a, const T &b) const {
//! return (b < a) ? b : a;
//! }
//! };
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_data; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! int \*d\_init\_iter; // e.g., INT\_MAX
//! CustomMin min\_op;
//!
//! auto future\_init\_value =
//! cub::FutureValue(d\_init\_iter);
//!
//! ...
//!
//! // Determine temporary device storage requirements for exclusive
//! // prefix scan
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::ExclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_data, min\_op, future\_init\_value, num\_items);
//!
//! // Allocate temporary storage for exclusive prefix scan
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run exclusive prefix min-scan
//! cub::DeviceScan::ExclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_data, min\_op, future\_init\_value, num\_items);
//!
//! // d\_data <-- [2147483647, 8, 6, 6, 5, 3, 0]
//!
//! @endrst
//!
//! @tparam IteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs and writing scan outputs
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the `init\_value`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in,out] d\_data
//! Pointer to the sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the exclusive scan (and is assigned to `\*d\_out`)
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveScan(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
IteratorT d\_data,
ScanOpT scan\_op,
FutureValue init\_value,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
return ExclusiveScan(d\_temp\_storage, temp\_storage\_bytes, d\_data, d\_data, scan\_op, init\_value, num\_items, stream);
}
//! @rst
//! Computes a device-wide exclusive prefix scan in-place using the specified
//! binary associative ``scan\_op`` functor. The ``init\_value`` value is provided as a future value.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! This is an environment-based API that allows customization of:
//!
//! - Stream: Query via ``cuda::get\_stream``
//! - Memory resource: Query via ``cuda::mr::get\_memory\_resource``
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates an in-place exclusive prefix scan of an
//! ``int`` device vector with a future initial value and a stream environment.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-scan-future-inplace-env
//! :end-before: example-end exclusive-scan-future-inplace-env
//!
//! @endrst
//!
//! @tparam IteratorT
//! \*\*[inferred]\*\* Random-access iterator type for reading and writing scan data
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the ``init\_value``
//!
//! @tparam InitValueIterT
//! \*\*[inferred]\*\* Iterator type for the future value
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in,out] d\_data
//! Random-access iterator to the sequence of data items
//!
//! @param[in] scan\_op
//! Binary scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the exclusive scan, provided as a future value
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_data`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveScan(
IteratorT d\_data,
ScanOpT scan\_op,
FutureValue init\_value,
NumItemsT num\_items,
const EnvT& env = {})
{
return ExclusiveScan(d\_data, d\_data, scan\_op, init\_value, num\_items, env);
}
//! @rst
//! Computes a device-wide exclusive prefix scan using the specified binary associative ``scan\_op`` functor.
//! The ``init\_value`` value is provided as a future value.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! - Can use a specific stream or cuda memory resource through the ``env`` parameter.
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place.
//! The range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix min-scan of an ``int`` device vector
//! using a future value for the initial value.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-scan-future-env
//! :end-before: example-end exclusive-scan-future-env
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the `init\_value`
//!
//! @tparam InitValueIterT
//! \*\*[inferred]\*\* Random-access iterator type used to access the initial value on device
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the exclusive scan (and is assigned to `\*d\_out`),
//! provided as a future value
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveScan(
InputIteratorT d\_in,
OutputIteratorT d\_out,
ScanOpT scan\_op,
FutureValue init\_value,
NumItemsT num\_items,
const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::ExclusiveScan");
return scan\_impl\_env(
d\_in, d\_out, scan\_op, detail::InputValue(init\_value), num\_items, env);
}
//! @}
//! @name Inclusive scans
//! @{
//! @rst
//! Computes a device-wide inclusive prefix sum.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place. The
//! range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix sum of an ``int`` device vector.
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_in; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! int \*d\_out; // e.g., [ , , , , , , ]
//! ...
//!
//! // Determine temporary device storage requirements for inclusive
//! // prefix sum
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::InclusiveSum(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, num\_items);
//!
//! // Allocate temporary storage for inclusive prefix sum
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run inclusive prefix sum
//! cub::DeviceScan::InclusiveSum(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, num\_items);
//!
//! // d\_out <-- [8, 14, 21, 26, 29, 29, 38]
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveSum(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
InputIteratorT d\_in,
OutputIteratorT d\_out,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::InclusiveSum");
return detail::scan::dispatch(
d\_temp\_storage,
temp\_storage\_bytes,
d\_in,
d\_out,
::cuda::std::plus<>{},
NullType{},
detail::make\_num\_items\_dispatch\_arg(num\_items),
stream);
}
//! @rst
//! Computes a device-wide inclusive prefix sum in-place.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix sum of an ``int`` device vector.
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_data; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! ...
//!
//! // Determine temporary device storage requirements for inclusive
//! // prefix sum
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::InclusiveSum(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_data, num\_items);
//!
//! // Allocate temporary storage for inclusive prefix sum
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run inclusive prefix sum
//! cub::DeviceScan::InclusiveSum(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_data, num\_items);
//!
//! // d\_data <-- [8, 14, 21, 26, 29, 29, 38]
//!
//! @endrst
//!
//! @tparam IteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs and writing scan outputs
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in,out] d\_data
//! Random-access iterator to the sequence of data items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveSum(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
IteratorT d\_data,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
return InclusiveSum(d\_temp\_storage, temp\_storage\_bytes, d\_data, d\_data, num\_items, stream);
}
//! @rst
//! Computes a device-wide inclusive prefix sum in-place.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! This is an environment-based API that allows customization of:
//!
//! - Stream: Query via ``cuda::get\_stream``
//! - Memory resource: Query via ``cuda::mr::get\_memory\_resource``
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates an in-place inclusive prefix sum of an
//! ``int`` device vector using a stream environment.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin inclusive-sum-inplace-env
//! :end-before: example-end inclusive-sum-inplace-env
//!
//! @endrst
//!
//! @tparam IteratorT
//! \*\*[inferred]\*\* Random-access iterator type for reading and writing scan data
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in,out] d\_data
//! Random-access iterator to the sequence of data items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_data`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t
InclusiveSum(IteratorT d\_data, NumItemsT num\_items, const EnvT& env = {})
{
return InclusiveSum(d\_data, d\_data, num\_items, env);
}
//! @rst
//! Computes a device-wide inclusive prefix sum.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place.
//! The range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix sum of an ``int`` device vector.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin inclusive-sum-env-determinism
//! :end-before: example-end inclusive-sum-env-determinism
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t
InclusiveSum(InputIteratorT d\_in, OutputIteratorT d\_out, NumItemsT num\_items, const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::InclusiveSum");
return scan\_impl\_env(d\_in, d\_out, ::cuda::std::plus<>{}, NullType{}, num\_items, env);
}
//! @rst
//! Computes a device-wide inclusive prefix scan using the specified binary associative ``scan\_op`` functor.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place. The
//! range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix min-scan of an ``int`` device vector.
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//! #include  // for INT\_MAX
//!
//! // CustomMin functor
//! struct CustomMin
//! {
//! template 
//! \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
//! T operator()(const T &a, const T &b) const {
//! return (b < a) ? b : a;
//! }
//! };
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_in; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! int \*d\_out; // e.g., [ , , , , , , ]
//! CustomMin min\_op;
//! ...
//!
//! // Determine temporary device storage requirements for inclusive
//! // prefix scan
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::InclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, min\_op, num\_items);
//!
//! // Allocate temporary storage for inclusive prefix scan
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run inclusive prefix min-scan
//! cub::DeviceScan::InclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, min\_op, num\_items);
//!
//! // d\_out <-- [8, 6, 6, 5, 3, 0, 0]
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveScan(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
InputIteratorT d\_in,
OutputIteratorT d\_out,
ScanOpT scan\_op,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::InclusiveScan");
return detail::scan::dispatch(
d\_temp\_storage,
temp\_storage\_bytes,
d\_in,
d\_out,
scan\_op,
NullType(),
detail::make\_num\_items\_dispatch\_arg(num\_items),
stream);
}
//! @rst
//! Computes a device-wide inclusive prefix scan using the specified binary associative ``scan\_op`` functor.
//! The result of applying the ``scan\_op`` binary operator to ``init\_value`` value and ``\*d\_in``
//! is assigned to ``\*d\_out``.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place. The
//! range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive max-scan of an ``int`` device vector.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin device-inclusive-scan
//! :end-before: example-end device-inclusive-scan
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the `init\_value`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to the size in bytes of the `d\_temp\_storage` allocation
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the inclusive scan (`scan\_op(init\_value, d\_in[0])`
//! is assigned to `\*d\_out`)
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! CUDA stream to launch kernels within.
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveScanInit(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
InputIteratorT d\_in,
OutputIteratorT d\_out,
ScanOpT scan\_op,
InitValueT init\_value,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::InclusiveScanInit");
return detail::scan::dispatch(
d\_temp\_storage,
temp\_storage\_bytes,
d\_in,
d\_out,
scan\_op,
detail::InputValue(init\_value),
detail::make\_num\_items\_dispatch\_arg(num\_items),
stream);
}
//! @rst
//! Computes a device-wide inclusive prefix scan using the specified binary associative ``scan\_op`` functor.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix min-scan of an ``int`` device vector.
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//! #include  // for INT\_MAX
//!
//! // CustomMin functor
//! struct CustomMin
//! {
//! template 
//! \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
//! T operator()(const T &a, const T &b) const {
//! return (b < a) ? b : a;
//! }
//! };
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_data; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! CustomMin min\_op;
//! ...
//!
//! // Determine temporary device storage requirements for inclusive
//! // prefix scan
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::InclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_data, min\_op, num\_items);
//!
//! // Allocate temporary storage for inclusive prefix scan
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run inclusive prefix min-scan
//! cub::DeviceScan::InclusiveScan(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_in, d\_out, min\_op, num\_items);
//!
//! // d\_data <-- [8, 6, 6, 5, 3, 0, 0]
//!
//! @endrst
//!
//! @tparam IteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs and writing scan outputs
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in] d\_data
//! Random-access iterator to the sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveScan(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
IteratorT d\_data,
ScanOpT scan\_op,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
return InclusiveScan(d\_temp\_storage, temp\_storage\_bytes, d\_data, d\_data, scan\_op, num\_items, stream);
}
//! @rst
//! Computes a device-wide inclusive prefix scan in-place using the specified
//! binary associative ``scan\_op`` functor.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! This is an environment-based API that allows customization of:
//!
//! - Stream: Query via ``cuda::get\_stream``
//! - Memory resource: Query via ``cuda::mr::get\_memory\_resource``
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates an in-place inclusive prefix scan of an
//! ``int`` device vector using a stream environment.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin inclusive-scan-inplace-env
//! :end-before: example-end inclusive-scan-inplace-env
//!
//! @endrst
//!
//! @tparam IteratorT
//! \*\*[inferred]\*\* Random-access iterator type for reading and writing scan data
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in,out] d\_data
//! Random-access iterator to the sequence of data items
//!
//! @param[in] scan\_op
//! Binary scan functor
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_data`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t
InclusiveScan(IteratorT d\_data, ScanOpT scan\_op, NumItemsT num\_items, const EnvT& env = {})
{
return InclusiveScan(d\_data, d\_data, scan\_op, num\_items, env);
}
//! @rst
//! Computes a device-wide inclusive prefix scan using the specified binary associative ``scan\_op`` functor.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place. The
//! range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix sum of an ``int`` device vector.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin inclusive-scan-env
//! :end-before: example-end inclusive-scan-env
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t
InclusiveScan(InputIteratorT d\_in, OutputIteratorT d\_out, ScanOpT scan\_op, NumItemsT num\_items, const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::InclusiveScan");
return scan\_impl\_env(d\_in, d\_out, scan\_op, NullType{}, num\_items, env);
}
//! @rst
//! Computes a device-wide inclusive prefix scan using the specified binary associative ``scan\_op`` functor.
//! The result of applying the ``scan\_op`` binary operator to ``init\_value`` value and ``\*d\_in``
//! is assigned to ``\*d\_out``.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place. The
//! range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix sum of an ``int`` device vector
//! with an initial value.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin inclusive-scan-init-env
//! :end-before: example-end inclusive-scan-init-env
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the `init\_value`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the inclusive scan (`scan\_op(init\_value, d\_in[0])`
//! is assigned to `\*d\_out`)
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveScanInit(
InputIteratorT d\_in,
OutputIteratorT d\_out,
ScanOpT scan\_op,
InitValueT init\_value,
NumItemsT num\_items,
const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::InclusiveScanInit");
return scan\_impl\_env(
d\_in, d\_out, scan\_op, detail::InputValue(init\_value), num\_items, env);
}
//! @rst
//! Computes a device-wide inclusive prefix scan using the specified binary associative ``scan\_op`` functor.
//! The result of applying the ``scan\_op`` binary operator to ``init\_value`` value and ``\*d\_in``
//! is assigned to ``\*d\_out``.
//!
//! .. versionadded:: 3.5.0
//! First appears in CUDA Toolkit 13.5.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place. The
//! range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix sum of an ``int`` device vector
//! with an initial value.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin inclusive-scan-future-init-env
//! :end-before: example-end inclusive-scan-future-init-env
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueIterT
//! \*\*[inferred]\*\* Random-access iterator type used to access the initial value on device
//!
//! @tparam InitValueBoundsT
//! \*\*[inferred]\*\* Static bounds on `init\_value`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the inclusive scan (`scan\_op(init\_value, d\_in[0])`
//! is assigned to `\*d\_out`), provided as deferred value. The deferred value must model an
//! iterator (i.e. the contained value should be dereferenceable to a value).
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
::cuda::std::enable\_if\_t, int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveScanInit(
InputIteratorT d\_in,
OutputIteratorT d\_out,
ScanOpT scan\_op,
const ::cuda::args::deferred& init\_value,
NumItemsT num\_items,
const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::InclusiveScanInit");
static\_assert(::cuda::std::indirectly\_readable, "The deferred value must model an iterator");
using \_\_init\_value\_type = typename ::cuda::std::remove\_cvref\_t::\_\_element\_type;
auto \_\_fut = FutureValue<\_\_init\_value\_type, InitValueIterT>{::cuda::args::\_\_unwrap(init\_value)};
return scan\_impl\_env(
d\_in, d\_out, scan\_op, detail::InputValue<\_\_init\_value\_type, InitValueIterT>(\_\_fut), num\_items, env);
}
//! @rst
//! Computes a device-wide inclusive prefix scan using the specified binary associative ``scan\_op`` functor.
//! The result of applying the ``scan\_op`` binary operator to ``init\_value`` value and ``\*d\_in``
//! is assigned to ``\*d\_out``.
//!
//! .. versionadded:: 3.5.0
//! First appears in CUDA Toolkit 13.5.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - When ``d\_in`` and ``d\_out`` are equal, the scan is performed in-place. The
//! range ``[d\_in, d\_in + num\_items)`` and ``[d\_out, d\_out + num\_items)``
//! shall not overlap in any other way.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive max-scan of an ``int`` device vector
//! with the initial value provided as a deferred device value.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin device-inclusive-scan-init-deferred
//! :end-before: example-end device-inclusive-scan-init-deferred
//!
//! @endrst
//!
//! @tparam InputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan inputs @iterator
//!
//! @tparam OutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueIterT
//! \*\*[inferred]\*\* Random-access iterator type used to access the initial value on device
//!
//! @tparam InitValueBoundsT
//! \*\*[inferred]\*\* Static bounds on `init\_value`
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to the size in bytes of the `d\_temp\_storage` allocation
//!
//! @param[in] d\_in
//! Random-access iterator to the input sequence of data items
//!
//! @param[out] d\_out
//! Random-access iterator to the output sequence of data items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the inclusive scan (`scan\_op(init\_value, d\_in[0])`
//! is assigned to `\*d\_out`), provided as deferred value. The deferred value must model an
//! iterator (i.e. the contained value should be dereferenceable to a value).
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_in`)
//!
//! @param[in] stream
//! CUDA stream to launch kernels within.
template 
CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveScanInit(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
InputIteratorT d\_in,
OutputIteratorT d\_out,
ScanOpT scan\_op,
const ::cuda::args::deferred& init\_value,
NumItemsT num\_items,
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::InclusiveScanInit");
static\_assert(::cuda::std::indirectly\_readable, "The deferred value must model an iterator");
using \_\_init\_value\_type = typename ::cuda::std::remove\_cvref\_t::\_\_element\_type;
auto \_\_fut = FutureValue<\_\_init\_value\_type, InitValueIterT>{::cuda::args::\_\_unwrap(init\_value)};
return detail::scan::dispatch(
d\_temp\_storage,
temp\_storage\_bytes,
d\_in,
d\_out,
scan\_op,
detail::InputValue<\_\_init\_value\_type, InitValueIterT>(\_\_fut),
detail::make\_num\_items\_dispatch\_arg(num\_items),
stream);
}
//! @}
//! @name Scans by key
//! @{
//! @rst
//! Computes a device-wide exclusive prefix sum-by-key with key equality
//! defined by ``equality\_op``. The value of ``0`` is applied as the initial
//! value, and is assigned to the beginning of each segment in ``d\_values\_out``.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - ``d\_keys\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_keys\_in, d\_keys\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - ``d\_values\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_values\_in, d\_values\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix sum-by-key of an ``int`` device vector.
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_keys\_in; // e.g., [0, 0, 1, 1, 1, 2, 2]
//! int \*d\_values\_in; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! int \*d\_values\_out; // e.g., [ , , , , , , ]
//! ...
//!
//! // Determine temporary device storage requirements
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::ExclusiveSumByKey(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_keys\_in, d\_values\_in, d\_values\_out, num\_items);
//!
//! // Allocate temporary storage
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run exclusive prefix sum
//! cub::DeviceScan::ExclusiveSumByKey(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_keys\_in, d\_values\_in, d\_values\_out, num\_items);
//!
//! // d\_values\_out <-- [0, 8, 0, 7, 12, 0, 0]
//!
//! @endrst
//!
//! @tparam KeysInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan keys inputs @iterator
//!
//! @tparam ValuesInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan values inputs @iterator
//!
//! @tparam ValuesOutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan values outputs @iterator
//!
//! @tparam EqualityOpT
//! \*\*[inferred]\*\* Functor type having member
//! `T operator()(const T &a, const T &b)` for binary operations that defines the equality of keys
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in] d\_keys\_in
//! Random-access input iterator to the input sequence of key items
//!
//! @param[in] d\_values\_in
//! Random-access input iterator to the input sequence of value items
//!
//! @param[out] d\_values\_out
//! Random-access output iterator to the output sequence of value items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_keys\_in` and `d\_values\_in`)
//!
//! @param[in] equality\_op
//! Binary functor that defines the equality of keys.
//! Default is cuda::std::equal\_to<>{}.
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template ,
typename NumItemsT = uint32\_t>
CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveSumByKey(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
KeysInputIteratorT d\_keys\_in,
ValuesInputIteratorT d\_values\_in,
ValuesOutputIteratorT d\_values\_out,
NumItemsT num\_items,
EqualityOpT equality\_op = EqualityOpT(),
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::ExclusiveSumByKey");
using init\_value\_t = cub::detail::it\_value\_t;
const init\_value\_t init\_value{};
return scan\_by\_key\_impl<::cuda::std::execution::env<>>(
d\_temp\_storage,
temp\_storage\_bytes,
d\_keys\_in,
d\_values\_in,
d\_values\_out,
equality\_op,
::cuda::std::plus<>{},
init\_value,
num\_items,
stream);
}
//! @rst
//! Computes a device-wide exclusive prefix scan-by-key using the
//! specified binary associative ``scan\_op`` functor. The key equality is defined by
//! ``equality\_op``. The ``init\_value`` value is applied as the initial
//! value, and is assigned to the beginning of each segment in ``d\_values\_out``.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - ``d\_keys\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_keys\_in, d\_keys\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - ``d\_values\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_values\_in, d\_values\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix min-scan-by-key of an ``int`` device vector
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//! #include  // for INT\_MAX
//!
//! // CustomMin functor
//! struct CustomMin
//! {
//! template 
//! \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
//! T operator()(const T &a, const T &b) const {
//! return (b < a) ? b : a;
//! }
//! };
//!
//! // CustomEqual functor
//! struct CustomEqual
//! {
//! template 
//! \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
//! T operator()(const T &a, const T &b) const {
//! return a == b;
//! }
//! };
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_keys\_in; // e.g., [0, 0, 1, 1, 1, 2, 2]
//! int \*d\_values\_in; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! int \*d\_values\_out; // e.g., [ , , , , , , ]
//! CustomMin min\_op;
//! CustomEqual equality\_op;
//! ...
//!
//! // Determine temporary device storage requirements for exclusive
//! // prefix scan
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::ExclusiveScanByKey(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_keys\_in, d\_values\_in, d\_values\_out, min\_op,
//! (int) INT\_MAX, num\_items, equality\_op);
//!
//! // Allocate temporary storage for exclusive prefix scan
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run exclusive prefix min-scan
//! cub::DeviceScan::ExclusiveScanByKey(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_keys\_in, d\_values\_in, d\_values\_out, min\_op,
//! (int) INT\_MAX, num\_items, equality\_op);
//!
//! // d\_values\_out <-- [2147483647, 8, 2147483647, 7, 5, 2147483647, 0]
//!
//! @endrst
//!
//! @tparam KeysInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan keys inputs @iterator
//!
//! @tparam ValuesInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan values inputs @iterator
//!
//! @tparam ValuesOutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan values outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the `init\_value`
//!
//! @tparam EqualityOpT
//! \*\*[inferred]\*\* Functor type having member
//! `T operator()(const T &a, const T &b)` for binary operations that defines the equality of keys
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in] d\_keys\_in
//! Random-access input iterator to the input sequence of key items
//!
//! @param[in] d\_values\_in
//! Random-access input iterator to the input sequence of value items
//!
//! @param[out] d\_values\_out
//! Random-access output iterator to the output sequence of value items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the exclusive scan (and is assigned to the
//! beginning of each segment in `d\_values\_out`)
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_keys\_in` and
//! `d\_values\_in`)
//!
//! @param[in] equality\_op
//! Binary functor that defines the equality of keys.
//! Default is cuda::std::equal\_to<>{}.
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template ,
typename NumItemsT = uint32\_t>
CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveScanByKey(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
KeysInputIteratorT d\_keys\_in,
ValuesInputIteratorT d\_values\_in,
ValuesOutputIteratorT d\_values\_out,
ScanOpT scan\_op,
InitValueT init\_value,
NumItemsT num\_items,
EqualityOpT equality\_op = EqualityOpT(),
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::ExclusiveScanByKey");
return scan\_by\_key\_impl<::cuda::std::execution::env<>>(
d\_temp\_storage,
temp\_storage\_bytes,
d\_keys\_in,
d\_values\_in,
d\_values\_out,
equality\_op,
scan\_op,
init\_value,
num\_items,
stream);
}
//! @rst
//! Computes a device-wide inclusive prefix sum-by-key with key equality defined by ``equality\_op``.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - ``d\_keys\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_keys\_in, d\_keys\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - ``d\_values\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_values\_in, d\_values\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix sum-by-key of an ``int`` device vector.
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_keys\_in; // e.g., [0, 0, 1, 1, 1, 2, 2]
//! int \*d\_values\_in; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! int \*d\_values\_out; // e.g., [ , , , , , , ]
//! ...
//!
//! // Determine temporary device storage requirements for inclusive prefix sum
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::InclusiveSumByKey(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_keys\_in, d\_values\_in, d\_values\_out, num\_items);
//!
//! // Allocate temporary storage for inclusive prefix sum
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run inclusive prefix sum
//! cub::DeviceScan::InclusiveSumByKey(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_keys\_in, d\_values\_in, d\_values\_out, num\_items);
//!
//! // d\_out <-- [8, 14, 7, 12, 15, 0, 9]
//!
//! @endrst
//!
//! @tparam KeysInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan keys inputs @iterator
//!
//! @tparam ValuesInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan values inputs @iterator
//!
//! @tparam ValuesOutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan values outputs @iterator
//!
//! @tparam EqualityOpT
//! \*\*[inferred]\*\* Functor type having member
//! `T operator()(const T &a, const T &b)` for binary operations that defines the equality of keys
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in] d\_keys\_in
//! Random-access input iterator to the input sequence of key items
//!
//! @param[in] d\_values\_in
//! Random-access input iterator to the input sequence of value items
//!
//! @param[out] d\_values\_out
//! Random-access output iterator to the output sequence of value items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_keys\_in` and `d\_values\_in`)
//!
//! @param[in] equality\_op
//! Binary functor that defines the equality of keys.
//! Default is cuda::std::equal\_to<>{}.
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template ,
typename NumItemsT = uint32\_t>
CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveSumByKey(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
KeysInputIteratorT d\_keys\_in,
ValuesInputIteratorT d\_values\_in,
ValuesOutputIteratorT d\_values\_out,
NumItemsT num\_items,
EqualityOpT equality\_op = EqualityOpT(),
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::InclusiveSumByKey");
return scan\_by\_key\_impl<::cuda::std::execution::env<>>(
d\_temp\_storage,
temp\_storage\_bytes,
d\_keys\_in,
d\_values\_in,
d\_values\_out,
equality\_op,
::cuda::std::plus<>{},
NullType{},
num\_items,
stream);
}
//! @rst
//! Computes a device-wide inclusive prefix scan-by-key using the
//! specified binary associative ``scan\_op`` functor. The key equality is defined by ``equality\_op``.
//!
//! .. versionadded:: 2.2.0
//! First appears in CUDA Toolkit 12.3.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - ``d\_keys\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_keys\_in, d\_keys\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - ``d\_values\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_values\_in, d\_values\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - @devicestorage
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix min-scan-by-key of an ``int`` device vector.
//!
//! .. code-block:: c++
//!
//! #include  // or equivalently 
//! #include  // for INT\_MAX
//!
//! // CustomMin functor
//! struct CustomMin
//! {
//! template 
//! \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
//! T operator()(const T &a, const T &b) const {
//! return (b < a) ? b : a;
//! }
//! };
//!
//! // CustomEqual functor
//! struct CustomEqual
//! {
//! template 
//! \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
//! T operator()(const T &a, const T &b) const {
//! return a == b;
//! }
//! };
//!
//! // Declare, allocate, and initialize device-accessible pointers for
//! // input and output
//! int num\_items; // e.g., 7
//! int \*d\_keys\_in; // e.g., [0, 0, 1, 1, 1, 2, 2]
//! int \*d\_values\_in; // e.g., [8, 6, 7, 5, 3, 0, 9]
//! int \*d\_values\_out; // e.g., [ , , , , , , ]
//! CustomMin min\_op;
//! CustomEqual equality\_op;
//! ...
//!
//! // Determine temporary device storage requirements for inclusive prefix scan
//! void \*d\_temp\_storage = nullptr;
//! size\_t temp\_storage\_bytes = 0;
//! cub::DeviceScan::InclusiveScanByKey(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_keys\_in, d\_values\_in, d\_values\_out, min\_op, num\_items, equality\_op);
//!
//! // Allocate temporary storage for inclusive prefix scan
//! cudaMalloc(&d\_temp\_storage, temp\_storage\_bytes);
//!
//! // Run inclusive prefix min-scan
//! cub::DeviceScan::InclusiveScanByKey(
//! d\_temp\_storage, temp\_storage\_bytes,
//! d\_keys\_in, d\_values\_in, d\_values\_out, min\_op, num\_items, equality\_op);
//!
//! // d\_out <-- [8, 6, 7, 5, 3, 0, 0]
//!
//! @endrst
//!
//! @tparam KeysInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan keys inputs @iterator
//!
//! @tparam ValuesInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan values inputs @iterator
//!
//! @tparam ValuesOutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan values outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam EqualityOpT
//! \*\*[inferred]\*\* Functor type having member
//! `T operator()(const T &a, const T &b)` for binary operations that defines the equality of keys
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @param[in] d\_temp\_storage
//! @devicestorage
//!
//! @param[in,out] temp\_storage\_bytes
//! Reference to size in bytes of `d\_temp\_storage` allocation
//!
//! @param[in] d\_keys\_in
//! Random-access input iterator to the input sequence of key items
//!
//! @param[in] d\_values\_in
//! Random-access input iterator to the input sequence of value items
//!
//! @param[out] d\_values\_out
//! Random-access output iterator to the output sequence of value items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_keys\_in` and `d\_values\_in`)
//!
//! @param[in] equality\_op
//! Binary functor that defines the equality of keys.
//! Default is cuda::std::equal\_to<>{}.
//!
//! @param[in] stream
//! @rst
//! \*\*[optional]\*\* CUDA stream to launch kernels within. Default is stream\ :sub:`0`.
//! @endrst
template ,
typename NumItemsT = uint32\_t>
CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveScanByKey(
void\* d\_temp\_storage,
size\_t& temp\_storage\_bytes,
KeysInputIteratorT d\_keys\_in,
ValuesInputIteratorT d\_values\_in,
ValuesOutputIteratorT d\_values\_out,
ScanOpT scan\_op,
NumItemsT num\_items,
EqualityOpT equality\_op = EqualityOpT(),
cudaStream\_t stream = nullptr)
{
\_CCCL\_NVTX\_RANGE\_SCOPE\_IF(d\_temp\_storage, "cub::DeviceScan::InclusiveScanByKey");
return scan\_by\_key\_impl<::cuda::std::execution::env<>>(
d\_temp\_storage,
temp\_storage\_bytes,
d\_keys\_in,
d\_values\_in,
d\_values\_out,
equality\_op,
scan\_op,
NullType{},
num\_items,
stream);
}
//! @rst
//! Computes a device-wide exclusive prefix sum-by-key with key equality
//! defined by ``equality\_op``. The value of ``0`` is applied as the initial
//! value, and is assigned to the beginning of each segment in ``d\_values\_out``.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - ``d\_keys\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_keys\_in, d\_keys\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - ``d\_values\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_values\_in, d\_values\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix sum-by-key of an ``int`` device vector.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_by\_key\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-sum-by-key-env
//! :end-before: example-end exclusive-sum-by-key-env
//!
//! @endrst
//!
//! @tparam KeysInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan keys inputs @iterator
//!
//! @tparam ValuesInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan values inputs @iterator
//!
//! @tparam ValuesOutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan values outputs @iterator
//!
//! @tparam EqualityOpT
//! \*\*[inferred]\*\* Functor type having member
//! `T operator()(const T &a, const T &b)` for binary operations that defines the equality of keys
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_keys\_in
//! Random-access input iterator to the input sequence of key items
//!
//! @param[in] d\_values\_in
//! Random-access input iterator to the input sequence of value items
//!
//! @param[out] d\_values\_out
//! Random-access output iterator to the output sequence of value items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_keys\_in` and `d\_values\_in`)
//!
//! @param[in] equality\_op
//! Binary functor that defines the equality of keys.
//! Default is cuda::std::equal\_to<>{}.
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
typename NumItemsT = uint32\_t,
typename EnvT = ::cuda::std::execution::env<>,
::cuda::std::enable\_if\_t<
!::cuda::std::is\_same\_v && !::cuda::std::is\_null\_pointer\_v
&& !::cuda::std::is\_same\_v,
int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveSumByKey(
KeysInputIteratorT d\_keys\_in,
ValuesInputIteratorT d\_values\_in,
ValuesOutputIteratorT d\_values\_out,
NumItemsT num\_items,
EqualityOpT equality\_op = EqualityOpT(),
const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::ExclusiveSumByKey");
using init\_value\_t = cub::detail::it\_value\_t;
return detail::dispatch\_with\_env(env, [&]([[maybe\_unused]] auto tuning, void\* storage, size\_t& bytes, auto stream) {
using tuning\_t = decltype(tuning);
return scan\_by\_key\_impl(
storage,
bytes,
d\_keys\_in,
d\_values\_in,
d\_values\_out,
equality\_op,
::cuda::std::plus<>{},
init\_value\_t{},
num\_items,
stream);
});
}
//! @rst
//! Computes a device-wide exclusive prefix scan-by-key using the
//! specified binary associative ``scan\_op`` functor. The key equality is defined by
//! ``equality\_op``. The ``init\_value`` value is applied as the initial
//! value, and is assigned to the beginning of each segment in ``d\_values\_out``.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - ``d\_keys\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_keys\_in, d\_keys\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - ``d\_values\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_values\_in, d\_values\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the exclusive prefix scan-by-key of an ``int`` device vector.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_by\_key\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin exclusive-scan-by-key-env
//! :end-before: example-end exclusive-scan-by-key-env
//!
//! @endrst
//!
//! @tparam KeysInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan keys inputs @iterator
//!
//! @tparam ValuesInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan values inputs @iterator
//!
//! @tparam ValuesOutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan values outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam InitValueT
//! \*\*[inferred]\*\* Type of the `init\_value`
//!
//! @tparam EqualityOpT
//! \*\*[inferred]\*\* Functor type having member
//! `T operator()(const T &a, const T &b)` for binary operations that defines the equality of keys
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_keys\_in
//! Random-access input iterator to the input sequence of key items
//!
//! @param[in] d\_values\_in
//! Random-access input iterator to the input sequence of value items
//!
//! @param[out] d\_values\_out
//! Random-access output iterator to the output sequence of value items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] init\_value
//! Initial value to seed the exclusive scan (and is assigned to the
//! beginning of each segment in `d\_values\_out`)
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_keys\_in` and `d\_values\_in`)
//!
//! @param[in] equality\_op
//! Binary functor that defines the equality of keys.
//! Default is cuda::std::equal\_to<>{}.
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
typename NumItemsT = uint32\_t,
typename EnvT = ::cuda::std::execution::env<>,
::cuda::std::enable\_if\_t<
!::cuda::std::is\_same\_v && !::cuda::std::is\_null\_pointer\_v
&& !::cuda::std::is\_same\_v,
int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t ExclusiveScanByKey(
KeysInputIteratorT d\_keys\_in,
ValuesInputIteratorT d\_values\_in,
ValuesOutputIteratorT d\_values\_out,
ScanOpT scan\_op,
InitValueT init\_value,
NumItemsT num\_items,
EqualityOpT equality\_op = EqualityOpT(),
const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::ExclusiveScanByKey");
return detail::dispatch\_with\_env(env, [&]([[maybe\_unused]] auto tuning, void\* storage, size\_t& bytes, auto stream) {
using tuning\_t = decltype(tuning);
return scan\_by\_key\_impl(
storage, bytes, d\_keys\_in, d\_values\_in, d\_values\_out, equality\_op, scan\_op, init\_value, num\_items, stream);
});
}
//! @rst
//! Computes a device-wide inclusive prefix sum-by-key with key equality defined by ``equality\_op``.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! - Supports non-commutative sum operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - ``d\_keys\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_keys\_in, d\_keys\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - ``d\_values\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_values\_in, d\_values\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix sum-by-key of an ``int`` device vector.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_by\_key\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin inclusive-sum-by-key-env
//! :end-before: example-end inclusive-sum-by-key-env
//!
//! @endrst
//!
//! @tparam KeysInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan keys inputs @iterator
//!
//! @tparam ValuesInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan values inputs @iterator
//!
//! @tparam ValuesOutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan values outputs @iterator
//!
//! @tparam EqualityOpT
//! \*\*[inferred]\*\* Functor type having member
//! `T operator()(const T &a, const T &b)` for binary operations that defines the equality of keys
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_keys\_in
//! Random-access input iterator to the input sequence of key items
//!
//! @param[in] d\_values\_in
//! Random-access input iterator to the input sequence of value items
//!
//! @param[out] d\_values\_out
//! Random-access output iterator to the output sequence of value items
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_keys\_in` and `d\_values\_in`)
//!
//! @param[in] equality\_op
//! Binary functor that defines the equality of keys.
//! Default is cuda::std::equal\_to<>{}.
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
typename NumItemsT = uint32\_t,
typename EnvT = ::cuda::std::execution::env<>,
::cuda::std::enable\_if\_t<
!::cuda::std::is\_same\_v && !::cuda::std::is\_null\_pointer\_v
&& !::cuda::std::is\_same\_v,
int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveSumByKey(
KeysInputIteratorT d\_keys\_in,
ValuesInputIteratorT d\_values\_in,
ValuesOutputIteratorT d\_values\_out,
NumItemsT num\_items,
EqualityOpT equality\_op = EqualityOpT(),
const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::InclusiveSumByKey");
return detail::dispatch\_with\_env(env, [&]([[maybe\_unused]] auto tuning, void\* storage, size\_t& bytes, auto stream) {
using tuning\_t = decltype(tuning);
return scan\_by\_key\_impl(
storage,
bytes,
d\_keys\_in,
d\_values\_in,
d\_values\_out,
equality\_op,
::cuda::std::plus<>{},
NullType{},
num\_items,
stream);
});
}
//! @rst
//! Computes a device-wide inclusive prefix scan-by-key using the
//! specified binary associative ``scan\_op`` functor. The key equality is defined by ``equality\_op``.
//!
//! .. versionadded:: 3.4.0
//! First appears in CUDA Toolkit 13.4.
//!
//! - Supports non-commutative scan operators.
//! - Results are not deterministic for pseudo-associative operators (e.g.,
//! addition of floating-point types). Results for pseudo-associative
//! operators may vary from run to run. Additional details can be found in
//! the @lookback description.
//! - ``d\_keys\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_keys\_in, d\_keys\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//! - ``d\_values\_in`` may equal ``d\_values\_out`` but the range
//! ``[d\_values\_in, d\_values\_in + num\_items)`` and the range
//! ``[d\_values\_out, d\_values\_out + num\_items)`` shall not overlap otherwise.
//!
//! Snippet
//! +++++++++++++++++++++++++++++++++++++++++++++
//!
//! The code snippet below illustrates the inclusive prefix scan-by-key of an ``int`` device vector.
//!
//! .. literalinclude:: ../../../cub/test/catch2\_test\_device\_scan\_by\_key\_env\_api.cu
//! :language: c++
//! :dedent:
//! :start-after: example-begin inclusive-scan-by-key-env
//! :end-before: example-end inclusive-scan-by-key-env
//!
//! @endrst
//!
//! @tparam KeysInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan keys inputs @iterator
//!
//! @tparam ValuesInputIteratorT
//! \*\*[inferred]\*\* Random-access input iterator type for reading scan values inputs @iterator
//!
//! @tparam ValuesOutputIteratorT
//! \*\*[inferred]\*\* Random-access output iterator type for writing scan values outputs @iterator
//!
//! @tparam ScanOpT
//! \*\*[inferred]\*\* Binary associative scan functor type having member `T operator()(const T &a, const T &b)`
//!
//! @tparam EqualityOpT
//! \*\*[inferred]\*\* Functor type having member
//! `T operator()(const T &a, const T &b)` for binary operations that defines the equality of keys
//!
//! @tparam NumItemsT
//! \*\*[inferred]\*\* An integral type representing the number of input elements
//!
//! @tparam EnvT
//! \*\*[inferred]\*\* Execution environment type providing stream, memory resource,
//! or determinism requirements. Default is ``cuda::std::execution::env<>``.
//!
//! @param[in] d\_keys\_in
//! Random-access input iterator to the input sequence of key items
//!
//! @param[in] d\_values\_in
//! Random-access input iterator to the input sequence of value items
//!
//! @param[out] d\_values\_out
//! Random-access output iterator to the output sequence of value items
//!
//! @param[in] scan\_op
//! Binary associative scan functor
//!
//! @param[in] num\_items
//! Total number of input items (i.e., the length of `d\_keys\_in` and `d\_values\_in`)
//!
//! @param[in] equality\_op
//! Binary functor that defines the equality of keys.
//! Default is cuda::std::equal\_to<>{}.
//!
//! @param[in] env
//! @rst
//! \*\*[optional]\*\* Execution environment. Default is ``cuda::std::execution::env{}``.
//! @endrst
template ,
typename NumItemsT = uint32\_t,
typename EnvT = ::cuda::std::execution::env<>,
::cuda::std::enable\_if\_t<
!::cuda::std::is\_same\_v && !::cuda::std::is\_null\_pointer\_v
&& !::cuda::std::is\_same\_v,
int> = 0>
[[nodiscard]] CUB\_RUNTIME\_FUNCTION static cudaError\_t InclusiveScanByKey(
KeysInputIteratorT d\_keys\_in,
ValuesInputIteratorT d\_values\_in,
ValuesOutputIteratorT d\_values\_out,
ScanOpT scan\_op,
NumItemsT num\_items,
EqualityOpT equality\_op = EqualityOpT(),
const EnvT& env = {})
{
\_CCCL\_NVTX\_RANGE\_SCOPE("cub::DeviceScan::InclusiveScanByKey");
return detail::dispatch\_with\_env(env, [&]([[maybe\_unused]] auto tuning, void\* storage, size\_t& bytes, auto stream) {
using tuning\_t = decltype(tuning);
return scan\_by\_key\_impl(
storage, bytes, d\_keys\_in, d\_values\_in, d\_values\_out, equality\_op, scan\_op, NullType{}, num\_items, stream);
});
}
//! @}
};
CUB\_NAMESPACE\_END