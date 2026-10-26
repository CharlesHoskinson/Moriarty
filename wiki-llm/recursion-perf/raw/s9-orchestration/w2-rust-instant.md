# Rust Instant clock semantics
> Source: https://doc.rust-lang.org/std/time/struct.Instant.html
> Collected: 2026-10-26
> Published: Unknown

Instant in std::time - Rust[Skip to main content](#main-content)

[Instant](#)
------------

[![logo](../../static.files/rust-logo-9a9549ea.svg)](../../std/index.html)

[std](../../std/index.html)1.99.1
---------------------------------

(58cf8f9c3 2026-10-13)

[Instant](#)
------------

### [Sections](#)

* [OS-specific behaviors](#os-specific-behaviors "OS-specific behaviors")
* [Underlying System calls](#underlying-system-calls "Underlying System calls")
  + [Monotonicity](#monotonicity "Monotonicity")

### [Associated Functions](#implementations)

* [now](#method.now "now")

### [Methods](#implementations)

* [checked\_add](#method.checked_add "checked_add")
* [checked\_duration\_since](#method.checked_duration_since "checked_duration_since")
* [checked\_sub](#method.checked_sub "checked_sub")
* [duration\_since](#method.duration_since "duration_since")
* [elapsed](#method.elapsed "elapsed")
* [saturating\_duration\_since](#method.saturating_duration_since "saturating_duration_since")

### [Trait Implementations](#trait-implementations)

* [Add<Duration>](#impl-Add%3CDuration%3E-for-Instant "Add<Duration>")
* [AddAssign<Duration>](#impl-AddAssign%3CDuration%3E-for-Instant "AddAssign<Duration>")
* [Clone](#impl-Clone-for-Instant "Clone")
* [Copy](#impl-Copy-for-Instant "Copy")
* [Debug](#impl-Debug-for-Instant "Debug")
* [Eq](#impl-Eq-for-Instant "Eq")
* [Hash](#impl-Hash-for-Instant "Hash")
* [Ord](#impl-Ord-for-Instant "Ord")
* [PartialEq](#impl-PartialEq-for-Instant "PartialEq")
* [PartialOrd](#impl-PartialOrd-for-Instant "PartialOrd")
* [StructuralPartialEq](#impl-StructuralPartialEq-for-Instant "StructuralPartialEq")
* [Sub](#impl-Sub-for-Instant "Sub")
* [Sub<Duration>](#impl-Sub%3CDuration%3E-for-Instant "Sub<Duration>")
* [SubAssign<Duration>](#impl-SubAssign%3CDuration%3E-for-Instant "SubAssign<Duration>")

### [Auto Trait Implementations](#synthetic-implementations)

* [Freeze](#impl-Freeze-for-Instant "Freeze")
* [RefUnwindSafe](#impl-RefUnwindSafe-for-Instant "RefUnwindSafe")
* [Send](#impl-Send-for-Instant "Send")
* [Sync](#impl-Sync-for-Instant "Sync")
* [Unpin](#impl-Unpin-for-Instant "Unpin")
* [UnsafeUnpin](#impl-UnsafeUnpin-for-Instant "UnsafeUnpin")
* [UnwindSafe](#impl-UnwindSafe-for-Instant "UnwindSafe")

### [Blanket Implementations](#blanket-implementations)

* [Any](#impl-Any-for-T "Any")
* [Borrow<T>](#impl-Borrow%3CT%3E-for-T "Borrow<T>")
* [BorrowMut<T>](#impl-BorrowMut%3CT%3E-for-T "BorrowMut<T>")
* [CloneToUninit](#impl-CloneToUninit-for-T "CloneToUninit")
* [From<T>](#impl-From%3CT%3E-for-T "From<T>")
* [Into<U>](#impl-Into%3CU%3E-for-T "Into<U>")
* [ToOwned](#impl-ToOwned-for-T "ToOwned")
* [TryFrom<U>](#impl-TryFrom%3CU%3E-for-T "TryFrom<U>")
* [TryInto<U>](#impl-TryInto%3CU%3E-for-T "TryInto<U>")

[In std::time](index.html)
--------------------------

[std](../index.html)::[time](index.html)

Struct Instant Copy item path
=============================

1.8.0 · [Source](../../src/std/time.rs.html#156)

```
pub struct Instant(/* private fields */);
```

Expand description

A measurement of a monotonically nondecreasing clock.
Opaque and useful only with [`Duration`](struct.Duration.html "struct std::time::Duration").

Instants are always guaranteed, barring [platform bugs](struct.Instant.html#monotonicity "struct std::time::Instant"), to be no less than any previously
measured instant when created, and are often useful for tasks such as measuring
benchmarks or timing how long an operation takes.

Note, however, that instants are **not** guaranteed to be **steady**. In other
words, each tick of the underlying clock might not be the same length (e.g.
some seconds may be longer than others). An instant may jump forwards or
experience time dilation (slow down or speed up), but it will never go
backwards.
As part of this non-guarantee it is also not specified whether system suspends count as
elapsed time or not. The behavior varies across platforms and Rust versions.

Instants are opaque types that can only be compared to one another. There is
no method to get “the number of seconds” from an instant. Instead, it only
allows measuring the duration between two instants (or comparing two
instants).

The size of an `Instant` struct may vary depending on the target operating
system.

Example:

```
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
   let now = Instant::now();

   // we sleep for 2 seconds
   sleep(Duration::new(2, 0));
   // it prints '2'
   println!("{}", now.elapsed().as_secs());
}
```

[§](#os-specific-behaviors)OS-specific behaviors
------------------------------------------------

An `Instant` is a wrapper around system-specific types and it may behave
differently depending on the underlying operating system. For example,
the following snippet is fine on Linux but panics on macOS:

```
use std::time::{Instant, Duration};

let now = Instant::now();
let days_per_10_millennia = 365_2425;
let solar_seconds_per_day = 60 * 60 * 24;
let millennium_in_solar_seconds = 31_556_952_000;
assert_eq!(millennium_in_solar_seconds, days_per_10_millennia * solar_seconds_per_day / 10);

let duration = Duration::new(millennium_in_solar_seconds, 0);
println!("{:?}", now + duration);
```

For cross-platform code, you can comfortably use durations of up to around one hundred years.

[§](#underlying-system-calls)Underlying System calls
----------------------------------------------------

The following system calls are [currently](../io/index.html#platform-specific-behavior "mod std::io") being used by `now()` to find out
the current time:

| Platform | System call |
| --- | --- |
| SGX | [`insecure_time` usercall](https://edp.fortanix.com/docs/api/fortanix_sgx_abi/struct.Usercalls.html#method.insecure_time). More information on [timekeeping in SGX](https://edp.fortanix.com/docs/concepts/rust-std/#codestdtimecode) |
| UNIX | [clock\_gettime](https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html) with `CLOCK_MONOTONIC` |
| WASI | [clock\_gettime](https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html) with `CLOCK_MONOTONIC` |
| Darwin | [clock\_gettime](https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html) with `CLOCK_UPTIME_RAW` |
| VXWorks | [clock\_gettime](https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html) with `CLOCK_MONOTONIC` |
| SOLID | `get_tim` |
| Windows | [QueryPerformanceCounter](https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter) |

**Disclaimer:** These system calls might change over time.

> Note: mathematical operations like [`add`](struct.Instant.html#method.add "method std::time::Instant::add") may panic if the underlying
> structure cannot represent the new point in time.

### [§](#monotonicity)Monotonicity

On all platforms `Instant` will try to use an OS API that guarantees monotonic behavior
if available, which is the case for all [tier 1](https://doc.rust-lang.org/rustc/platform-support.html) platforms.
In practice such guarantees are – under rare circumstances – broken by hardware, virtualization
or operating system bugs. To work around these bugs and platforms not offering monotonic clocks
[`duration_since`](struct.Instant.html#method.duration_since "method std::time::Instant::duration_since"), [`elapsed`](struct.Instant.html#method.elapsed "method std::time::Instant::elapsed") and [`sub`](#impl-Sub-for-Instant) saturate to zero. In older
Rust versions this lead to a panic instead. [`checked_duration_since`](struct.Instant.html#method.checked_duration_since "method std::time::Instant::checked_duration_since") can be used to detect and
handle situations where monotonicity is violated, or `Instant`s are subtracted in the wrong order.

This workaround obscures programming errors where earlier and later instants are accidentally
swapped. For this reason future Rust versions may reintroduce panics.

Implementations[§](#implementations)
------------------------------------

[Source](../../src/std/time.rs.html#272-416)[§](#impl-Instant)

### impl [Instant](struct.Instant.html "struct std::time::Instant")

1.8.0 · [Source](../../src/std/time.rs.html#285-287)

#### pub fn [now](#method.now)() -> [Instant](struct.Instant.html "struct std::time::Instant")

Returns an instant corresponding to “now”.

##### [§](#examples)Examples

```
use std::time::Instant;

let now = Instant::now();
```

1.8.0 · [Source](../../src/std/time.rs.html#314-316)

#### pub fn [duration\_since](#method.duration_since)(&self, earlier: [Instant](struct.Instant.html "struct std::time::Instant")) -> [Duration](struct.Duration.html "struct std::time::Duration")

Returns the amount of time elapsed from another instant to this one,
or zero duration if that instant is later than this one.

##### [§](#panics)Panics

Previous Rust versions panicked when `earlier` was later than `self`. Currently this
method saturates. Future versions may reintroduce the panic in some circumstances.
See [Monotonicity](struct.Instant.html#monotonicity "struct std::time::Instant").

##### [§](#examples-1)Examples

```
use std::time::{Duration, Instant};
use std::thread::sleep;

let now = Instant::now();
sleep(Duration::new(1, 0));
let new_now = Instant::now();
println!("{:?}", new_now.duration_since(now));
println!("{:?}", now.duration_since(new_now)); // 0ns
```

1.39.0 · [Source](../../src/std/time.rs.html#340-342)

#### pub fn [checked\_duration\_since](#method.checked_duration_since)(&self, earlier: [Instant](struct.Instant.html "struct std::time::Instant")) -> [Option](../option/enum.Option.html "enum std::option::Option")<[Duration](struct.Duration.html "struct std::time::Duration")>

Returns the amount of time elapsed from another instant to this one,
or None if that instant is later than this one.

Due to [monotonicity bugs](struct.Instant.html#monotonicity "struct std::time::Instant"), even under correct logical ordering of the passed `Instant`s,
this method can return `None`.

##### [§](#examples-2)Examples

```
use std::time::{Duration, Instant};
use std::thread::sleep;

let now = Instant::now();
sleep(Duration::new(1, 0));
let new_now = Instant::now();
println!("{:?}", new_now.checked_duration_since(now));
println!("{:?}", now.checked_duration_since(new_now)); // None
```

1.39.0 · [Source](../../src/std/time.rs.html#361-363)

#### pub fn [saturating\_duration\_since](#method.saturating_duration_since)(&self, earlier: [Instant](struct.Instant.html "struct std::time::Instant")) -> [Duration](struct.Duration.html "struct std::time::Duration")

Returns the amount of time elapsed from another instant to this one,
or zero duration if that instant is later than this one.

##### [§](#examples-3)Examples

```
use std::time::{Duration, Instant};
use std::thread::sleep;

let now = Instant::now();
sleep(Duration::new(1, 0));
let new_now = Instant::now();
println!("{:?}", new_now.saturating_duration_since(now));
println!("{:?}", now.saturating_duration_since(new_now)); // 0ns
```

1.8.0 · [Source](../../src/std/time.rs.html#388-390)

#### pub fn [elapsed](#method.elapsed)(&self) -> [Duration](struct.Duration.html "struct std::time::Duration")

Returns the amount of time elapsed since this instant.

##### [§](#panics-1)Panics

Previous Rust versions panicked when the current time was earlier than self. Currently this
method returns a Duration of zero in that case. Future versions may reintroduce the panic.
See [Monotonicity](struct.Instant.html#monotonicity "struct std::time::Instant").

##### [§](#examples-4)Examples

```
use std::thread::sleep;
use std::time::{Duration, Instant};

let instant = Instant::now();
let three_secs = Duration::from_secs(3);
sleep(three_secs);
assert!(instant.elapsed() >= three_secs);
```

1.34.0 · [Source](../../src/std/time.rs.html#396-398)

#### pub fn [checked\_add](#method.checked_add)(&self, duration: [Duration](struct.Duration.html "struct std::time::Duration")) -> [Option](../option/enum.Option.html "enum std::option::Option")<[Instant](struct.Instant.html "struct std::time::Instant")>

Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as
`Instant` (which means it’s inside the bounds of the underlying data structure), `None`
otherwise.

1.34.0 · [Source](../../src/std/time.rs.html#404-406)

#### pub fn [checked\_sub](#method.checked_sub)(&self, duration: [Duration](struct.Duration.html "struct std::time::Duration")) -> [Option](../option/enum.Option.html "enum std::option::Option")<[Instant](struct.Instant.html "struct std::time::Instant")>

Returns `Some(t)` where `t` is the time `self - duration` if `t` can be represented as
`Instant` (which means it’s inside the bounds of the underlying data structure), `None`
otherwise.

Trait Implementations[§](#trait-implementations)
------------------------------------------------

1.8.0 · [Source](../../src/std/time.rs.html#419-430)[§](#impl-Add%3CDuration%3E-for-Instant)

### impl [Add](../ops/trait.Add.html "trait std::ops::Add")<[Duration](struct.Duration.html "struct std::time::Duration")> for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#427-429)[§](#method.add)

#### fn [add](../ops/trait.Add.html#tymethod.add)(self, other: [Duration](struct.Duration.html "struct std::time::Duration")) -> [Instant](struct.Instant.html "struct std::time::Instant")

##### [§](#panics-2)Panics

This function may panic if the resulting point in time cannot be represented by the
underlying data structure. See [`Instant::checked_add`](struct.Instant.html#method.checked_add "method std::time::Instant::checked_add") for a version without panic.

[Source](../../src/std/time.rs.html#420)[§](#associatedtype.Output)

#### type [Output](../ops/trait.Add.html#associatedtype.Output) = [Instant](struct.Instant.html "struct std::time::Instant")

The resulting type after applying the `+` operator.

1.9.0 · [Source](../../src/std/time.rs.html#433-437)[§](#impl-AddAssign%3CDuration%3E-for-Instant)

### impl [AddAssign](../ops/trait.AddAssign.html "trait std::ops::AddAssign")<[Duration](struct.Duration.html "struct std::time::Duration")> for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#434-436)[§](#method.add_assign)

#### fn [add\_assign](../ops/trait.AddAssign.html#tymethod.add_assign)(&mut self, other: [Duration](struct.Duration.html "struct std::time::Duration"))

Performs the `+=` operation. [Read more](../ops/trait.AddAssign.html#tymethod.add_assign)

1.8.0 · [Source](../../src/std/time.rs.html#153)[§](#impl-Clone-for-Instant)

### impl [Clone](../clone/trait.Clone.html "trait std::clone::Clone") for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#153)[§](#method.clone)

#### fn [clone](../clone/trait.Clone.html#tymethod.clone)(&self) -> [Instant](struct.Instant.html "struct std::time::Instant")

Returns a duplicate of the value. [Read more](../clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](../../src/core/clone.rs.html#245-247)[§](#method.clone_from)

#### fn [clone\_from](../clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](../clone/trait.Clone.html#method.clone_from)

1.8.0 · [Source](../../src/std/time.rs.html#153)[§](#impl-Copy-for-Instant)

### impl [Copy](../marker/trait.Copy.html "trait std::marker::Copy") for [Instant](struct.Instant.html "struct std::time::Instant")

1.8.0 · [Source](../../src/std/time.rs.html#476-480)[§](#impl-Debug-for-Instant)

### impl [Debug](../fmt/trait.Debug.html "trait std::fmt::Debug") for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#477-479)[§](#method.fmt)

#### fn [fmt](../fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](../fmt/struct.Formatter.html "struct std::fmt::Formatter")<'\_>) -> [Result](../fmt/type.Result.html "type std::fmt::Result")

Formats the value using the given formatter. [Read more](../fmt/trait.Debug.html#tymethod.fmt)

1.8.0 · [Source](../../src/std/time.rs.html#153)[§](#impl-Eq-for-Instant)

### impl [Eq](../cmp/trait.Eq.html "trait std::cmp::Eq") for [Instant](struct.Instant.html "struct std::time::Instant")

1.8.0 · [Source](../../src/std/time.rs.html#153)[§](#impl-Hash-for-Instant)

### impl [Hash](../hash/trait.Hash.html "trait std::hash::Hash") for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#153)[§](#method.hash)

#### fn [hash](../hash/trait.Hash.html#tymethod.hash)<\_\_H: [Hasher](../hash/trait.Hasher.html "trait std::hash::Hasher")>(&self, state: [&mut \_\_H](../primitive.reference.html))

Feeds this value into the given [`Hasher`](../hash/trait.Hasher.html "trait std::hash::Hasher"). [Read more](../hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](../../src/core/hash/mod.rs.html#234-236)[§](#method.hash_slice)

#### fn [hash\_slice](../hash/trait.Hash.html#method.hash_slice)<H>(data: &[Self], state: [&mut H](../primitive.reference.html)) where H: [Hasher](../hash/trait.Hasher.html "trait std::hash::Hasher"), Self: [Sized](../marker/trait.Sized.html "trait std::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](../hash/trait.Hasher.html "trait std::hash::Hasher"). [Read more](../hash/trait.Hash.html#method.hash_slice)

1.8.0 · [Source](../../src/std/time.rs.html#153)[§](#impl-Ord-for-Instant)

### impl [Ord](../cmp/trait.Ord.html "trait std::cmp::Ord") for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#153)[§](#method.cmp)

#### fn [cmp](../cmp/trait.Ord.html#tymethod.cmp)(&self, other: &[Instant](struct.Instant.html "struct std::time::Instant")) -> [Ordering](../cmp/enum.Ordering.html "enum std::cmp::Ordering")

This method returns an [`Ordering`](../cmp/enum.Ordering.html "enum std::cmp::Ordering") between `self` and `other`. [Read more](../cmp/trait.Ord.html#tymethod.cmp)

1.21.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](../../src/core/cmp.rs.html#1046-1048)[§](#method.max)

#### fn [max](../cmp/trait.Ord.html#method.max)(self, other: Self) -> Self where Self: [Sized](../marker/trait.Sized.html "trait std::marker::Sized"),

Compares and returns the maximum of two values. [Read more](../cmp/trait.Ord.html#method.max)

1.21.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](../../src/core/cmp.rs.html#1085-1087)[§](#method.min)

#### fn [min](../cmp/trait.Ord.html#method.min)(self, other: Self) -> Self where Self: [Sized](../marker/trait.Sized.html "trait std::marker::Sized"),

Compares and returns the minimum of two values. [Read more](../cmp/trait.Ord.html#method.min)

1.50.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](../../src/core/cmp.rs.html#1111-1113)[§](#method.clamp)

#### fn [clamp](../cmp/trait.Ord.html#method.clamp)(self, min: Self, max: Self) -> Self where Self: [Sized](../marker/trait.Sized.html "trait std::marker::Sized"),

Restrict a value to a certain interval. [Read more](../cmp/trait.Ord.html#method.clamp)

1.8.0 · [Source](../../src/std/time.rs.html#153)[§](#impl-PartialEq-for-Instant)

### impl [PartialEq](../cmp/trait.PartialEq.html "trait std::cmp::PartialEq") for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#153)[§](#method.eq)

#### fn [eq](../cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Instant](struct.Instant.html "struct std::time::Instant")) -> [bool](../primitive.bool.html)

Equality operator `==`. [Read more](../cmp/trait.PartialEq.html#tymethod.eq)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](../../src/core/cmp.rs.html#275)[§](#method.ne)

#### fn [ne](../cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](../primitive.reference.html)) -> [bool](../primitive.bool.html)

Inequality operator `!=`. [Read more](../cmp/trait.PartialEq.html#method.ne)

1.8.0 · [Source](../../src/std/time.rs.html#153)[§](#impl-PartialOrd-for-Instant)

### impl [PartialOrd](../cmp/trait.PartialOrd.html "trait std::cmp::PartialOrd") for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#153)[§](#method.partial_cmp)

#### fn [partial\_cmp](../cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[Instant](struct.Instant.html "struct std::time::Instant")) -> [Option](../option/enum.Option.html "enum std::option::Option")<[Ordering](../cmp/enum.Ordering.html "enum std::cmp::Ordering")>

This method returns an ordering between `self` and `other` values if one exists. [Read more](../cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](../../src/core/cmp.rs.html#1422)[§](#method.lt)

#### fn [lt](../cmp/trait.PartialOrd.html#method.lt)(&self, other: [&Rhs](../primitive.reference.html)) -> [bool](../primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](../cmp/trait.PartialOrd.html#method.lt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](../../src/core/cmp.rs.html#1440)[§](#method.le)

#### fn [le](../cmp/trait.PartialOrd.html#method.le)(&self, other: [&Rhs](../primitive.reference.html)) -> [bool](../primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the
`<=` operator. [Read more](../cmp/trait.PartialOrd.html#method.le)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](../../src/core/cmp.rs.html#1458)[§](#method.gt)

#### fn [gt](../cmp/trait.PartialOrd.html#method.gt)(&self, other: [&Rhs](../primitive.reference.html)) -> [bool](../primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>`
operator. [Read more](../cmp/trait.PartialOrd.html#method.gt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](../../src/core/cmp.rs.html#1476)[§](#method.ge)

#### fn [ge](../cmp/trait.PartialOrd.html#method.ge)(&self, other: [&Rhs](../primitive.reference.html)) -> [bool](../primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by
the `>=` operator. [Read more](../cmp/trait.PartialOrd.html#method.ge)

1.8.0 · [Source](../../src/std/time.rs.html#153)[§](#impl-StructuralPartialEq-for-Instant)

### impl [StructuralPartialEq](../marker/trait.StructuralPartialEq.html "trait std::marker::StructuralPartialEq") for [Instant](struct.Instant.html "struct std::time::Instant")

1.8.0 · [Source](../../src/std/time.rs.html#457-473)[§](#impl-Sub-for-Instant)

### impl [Sub](../ops/trait.Sub.html "trait std::ops::Sub") for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#470-472)[§](#method.sub)

#### fn [sub](../ops/trait.Sub.html#tymethod.sub)(self, other: [Instant](struct.Instant.html "struct std::time::Instant")) -> [Duration](struct.Duration.html "struct std::time::Duration")

Returns the amount of time elapsed from another instant to this one,
or zero duration if that instant is later than this one.

##### [§](#panics-3)Panics

Previous Rust versions panicked when `other` was later than `self`. Currently this
method saturates. Future versions may reintroduce the panic in some circumstances.
See [Monotonicity](struct.Instant.html#monotonicity "struct std::time::Instant").

[Source](../../src/std/time.rs.html#458)[§](#associatedtype.Output-1)

#### type [Output](../ops/trait.Sub.html#associatedtype.Output) = [Duration](struct.Duration.html "struct std::time::Duration")

The resulting type after applying the `-` operator.

1.8.0 · [Source](../../src/std/time.rs.html#440-447)[§](#impl-Sub%3CDuration%3E-for-Instant)

### impl [Sub](../ops/trait.Sub.html "trait std::ops::Sub")<[Duration](struct.Duration.html "struct std::time::Duration")> for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#441)[§](#associatedtype.Output-2)

#### type [Output](../ops/trait.Sub.html#associatedtype.Output) = [Instant](struct.Instant.html "struct std::time::Instant")

The resulting type after applying the `-` operator.

[Source](../../src/std/time.rs.html#444-446)[§](#method.sub-1)

#### fn [sub](../ops/trait.Sub.html#tymethod.sub)(self, other: [Duration](struct.Duration.html "struct std::time::Duration")) -> [Instant](struct.Instant.html "struct std::time::Instant")

Performs the `-` operation. [Read more](../ops/trait.Sub.html#tymethod.sub)

1.9.0 · [Source](../../src/std/time.rs.html#450-454)[§](#impl-SubAssign%3CDuration%3E-for-Instant)

### impl [SubAssign](../ops/trait.SubAssign.html "trait std::ops::SubAssign")<[Duration](struct.Duration.html "struct std::time::Duration")> for [Instant](struct.Instant.html "struct std::time::Instant")

[Source](../../src/std/time.rs.html#451-453)[§](#method.sub_assign)

#### fn [sub\_assign](../ops/trait.SubAssign.html#tymethod.sub_assign)(&mut self, other: [Duration](struct.Duration.html "struct std::time::Duration"))

Performs the `-=` operation. [Read more](../ops/trait.SubAssign.html#tymethod.sub_assign)

Auto Trait Implementations[§](#synthetic-implementations)
---------------------------------------------------------

[§](#impl-Freeze-for-Instant)

### impl [Freeze](../marker/trait.Freeze.html "trait std::marker::Freeze") for [Instant](struct.Instant.html "struct std::time::Instant")

[§](#impl-RefUnwindSafe-for-Instant)

### impl [RefUnwindSafe](../panic/trait.RefUnwindSafe.html "trait std::panic::RefUnwindSafe") for [Instant](struct.Instant.html "struct std::time::Instant")

[§](#impl-Send-for-Instant)

### impl [Send](../marker/trait.Send.html "trait std::marker::Send") for [Instant](struct.Instant.html "struct std::time::Instant")

[§](#impl-Sync-for-Instant)

### impl [Sync](../marker/trait.Sync.html "trait std::marker::Sync") for [Instant](struct.Instant.html "struct std::time::Instant")

[§](#impl-Unpin-for-Instant)

### impl [Unpin](../marker/trait.Unpin.html "trait std::marker::Unpin") for [Instant](struct.Instant.html "struct std::time::Instant")

[§](#impl-UnsafeUnpin-for-Instant)

### impl [UnsafeUnpin](../marker/trait.UnsafeUnpin.html "trait std::marker::UnsafeUnpin") for [Instant](struct.Instant.html "struct std::time::Instant")

[§](#impl-UnwindSafe-for-Instant)

### impl [UnwindSafe](../panic/trait.UnwindSafe.html "trait std::panic::UnwindSafe") for [Instant](struct.Instant.html "struct std::time::Instant")

Blanket Implementations[§](#blanket-implementations)
----------------------------------------------------

[Source](../../src/core/any.rs.html#141)[§](#impl-Any-for-T)

### impl<T> [Any](../any/trait.Any.html "trait std::any::Any") for T where T: 'static + ?[Sized](../marker/trait.Sized.html "trait std::marker::Sized"),

[Source](../../src/core/any.rs.html#142)[§](#method.type_id)

#### fn [type\_id](../any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](../any/struct.TypeId.html "struct std::any::TypeId")

Gets the `TypeId` of `self`. [Read more](../any/trait.Any.html#tymethod.type_id)

[Source](../../src/core/borrow.rs.html#212)[§](#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](../borrow/trait.Borrow.html "trait std::borrow::Borrow")<T> for T where T: ?[Sized](../marker/trait.Sized.html "trait std::marker::Sized"),

[Source](../../src/core/borrow.rs.html#214)[§](#method.borrow)

#### fn [borrow](../borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](../primitive.reference.html)

Immutably borrows from an owned value. [Read more](../borrow/trait.Borrow.html#tymethod.borrow)

[Source](../../src/core/borrow.rs.html#221)[§](#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](../borrow/trait.BorrowMut.html "trait std::borrow::BorrowMut")<T> for T where T: ?[Sized](../marker/trait.Sized.html "trait std::marker::Sized"),

[Source](../../src/core/borrow.rs.html#222)[§](#method.borrow_mut)

#### fn [borrow\_mut](../borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](../primitive.reference.html)

Mutably borrows from an owned value. [Read more](../borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](../../src/core/clone.rs.html#648)[§](#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](../clone/trait.CloneToUninit.html "trait std::clone::CloneToUninit") for T where T: [Clone](../clone/trait.Clone.html "trait std::clone::Clone"),

[Source](../../src/core/clone.rs.html#650)[§](#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](../clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](../primitive.pointer.html) [u8](../primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit` [#126799](https://github.com/rust-lang/rust/issues/126799))

Performs copy-assignment from `self` to `dest`. [Read more](../clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](../../src/core/convert/mod.rs.html#788)[§](#impl-From%3CT%3E-for-T)

### impl<T> [From](../convert/trait.From.html "trait std::convert::From")<T> for T

[Source](../../src/core/convert/mod.rs.html#791)[§](#method.from)

#### fn [from](../convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](../../src/core/convert/mod.rs.html#770-772)[§](#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](../convert/trait.Into.html "trait std::convert::Into")<U> for T where U: [From](../convert/trait.From.html "trait std::convert::From")<T>,

[Source](../../src/core/convert/mod.rs.html#780)[§](#method.into)

#### fn [into](../convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](../../src/alloc/borrow.rs.html#72-74)[§](#impl-ToOwned-for-T)

### impl<T> [ToOwned](../borrow/trait.ToOwned.html "trait std::borrow::ToOwned") for T where T: [Clone](../clone/trait.Clone.html "trait std::clone::Clone"),

[Source](../../src/alloc/borrow.rs.html#76)[§](#associatedtype.Owned)

#### type [Owned](../borrow/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](../../src/alloc/borrow.rs.html#77)[§](#method.to_owned)

#### fn [to\_owned](../borrow/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](../../src/alloc/borrow.rs.html#81)[§](#method.clone_into)

#### fn [clone\_into](../borrow/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](../primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../borrow/trait.ToOwned.html#method.clone_into)

[Source](../../src/core/convert/mod.rs.html#830-832)[§](#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](../convert/trait.TryFrom.html "trait std::convert::TryFrom")<U> for T where U: [Into](../convert/trait.Into.html "trait std::convert::Into")<T>,

[Source](../../src/core/convert/mod.rs.html#834)[§](#associatedtype.Error)

#### type [Error](../convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](../convert/enum.Infallible.html "enum std::convert::Infallible")

The type returned in the event of a conversion error.

[Source](../../src/core/convert/mod.rs.html#837)[§](#method.try_from)

#### fn [try\_from](../convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](../result/enum.Result.html "enum std::result::Result")<T, <T as [TryFrom](../convert/trait.TryFrom.html "trait std::convert::TryFrom")<U>>::[Error](../convert/trait.TryFrom.html#associatedtype.Error "type std::convert::TryFrom::Error")>

Performs the conversion.

[Source](../../src/core/convert/mod.rs.html#814-816)[§](#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](../convert/trait.TryInto.html "trait std::convert::TryInto")<U> for T where U: [TryFrom](../convert/trait.TryFrom.html "trait std::convert::TryFrom")<T>,

[Source](../../src/core/convert/mod.rs.html#818)[§](#associatedtype.Error-1)

#### type [Error](../convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](../convert/trait.TryFrom.html "trait std::convert::TryFrom")<T>>::[Error](../convert/trait.TryFrom.html#associatedtype.Error "type std::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](../../src/core/convert/mod.rs.html#821)[§](#method.try_into)

#### fn [try\_into](../convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](../result/enum.Result.html "enum std::result::Result")<U, <U as [TryFrom](../convert/trait.TryFrom.html "trait std::convert::TryFrom")<T>>::[Error](../convert/trait.TryFrom.html#associatedtype.Error "type std::convert::TryFrom::Error")>

Performs the conversion.