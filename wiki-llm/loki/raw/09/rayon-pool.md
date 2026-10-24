ThreadPoolBuilder in rayon\_core - Rust

[Docs.rs](/)

* [rayon-core-1.13.0](# "Core APIs for Rayon")

  + rayon-core 1.13.0
  + [Permalink](/rayon-core/1.13.0/rayon_core/struct.ThreadPoolBuilder.html "Get a link to this specific version")
  + [Docs.rs crate page](/crate/rayon-core/latest "See rayon-core in docs.rs")
  + [MIT](https://spdx.org/licenses/MIT) OR [Apache-2.0](https://spdx.org/licenses/Apache-2.0)
  + 11 September 2026

  + Links
  + [Repository](https://github.com/rayon-rs/rayon)
  + [crates.io](https://crates.io/crates/rayon-core "See rayon-core in crates.io")
  + [Source](/crate/rayon-core/latest/source/ "Browse source of rayon-core-1.13.0")

  + Owners
  + [nikomatsakis](https://crates.io/users/nikomatsakis)
  + [cuviper](https://crates.io/users/cuviper)

  + Dependencies
  + - [crossbeam-deque ^0.8.1
      *normal*](/crossbeam-deque/^0.8.1/)
    - [crossbeam-utils ^0.8.0
      *normal*](/crossbeam-utils/^0.8.0/)
    - [wasm\_sync ^0.1.0
      *normal*
      *optional*](/wasm_sync/^0.1.0/)
    - [rand ^0.9
      *dev*](/rand/^0.9/)
    - [rand\_xorshift ^0.4
      *dev*](/rand_xorshift/^0.4/)
    - [scoped-tls ^1.0
      *dev*](/scoped-tls/^1.0/)
    - [libc ^0.2
      *dev*](/libc/^0.2/)

  + Versions

  + [**100%**
    of the crate is documented](/crate/rayon-core/latest)
* [Platform](#)
  + [x86\_64-unknown-linux-gnu](/crate/rayon-core/latest/target-redirect/rayon_core/struct.ThreadPoolBuilder.html)
* [Feature flags](/crate/rayon-core/latest/features "Browse available feature flags of rayon-core-1.13.0")

* [docs.rs](#)
  + [About docs.rs](/about)
  + [Badges](/about/badges)
  + [Builds](/about/builds)
  + [Metadata](/about/metadata)
  + [Shorthand URLs](/about/redirections)
  + [Download](/about/download)
  + [Rustdoc JSON](/about/rustdoc-json)
  + [Build queue](/releases/queue)
  + [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

* [Rust](#)
  + [Rust website](https://www.rust-lang.org/)
  + [The Book](https://doc.rust-lang.org/book/)
  + [Standard Library API Reference](https://doc.rust-lang.org/std/)
  + [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  + [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  + [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[Skip to main content](#main-content)

[ThreadPoolBuilder](#)
----------------------

[rayon\_core](../rayon_core/index.html)1.13.0
---------------------------------------------

[ThreadPoolBuilder](#)
----------------------

### [Sections](#)

* [Creating a ThreadPool](#creating-a-threadpool "Creating a ThreadPool")

### [Associated Functions](#implementations)

* [new](#method.new "new")

### [Methods](#implementations)

* [breadth\_first](#method.breadth_first "breadth_first")
* [build](#method.build "build")
* [build\_global](#method.build_global "build_global")
* [build\_scoped](#method.build_scoped "build_scoped")
* [exit\_handler](#method.exit_handler "exit_handler")
* [num\_threads](#method.num_threads "num_threads")
* [panic\_handler](#method.panic_handler "panic_handler")
* [spawn\_handler](#method.spawn_handler "spawn_handler")
* [stack\_size](#method.stack_size "stack_size")
* [start\_handler](#method.start_handler "start_handler")
* [thread\_name](#method.thread_name "thread_name")
* [use\_current\_thread](#method.use_current_thread "use_current_thread")

### [Trait Implementations](#trait-implementations)

* [Debug](#impl-Debug-for-ThreadPoolBuilder%3CS%3E "Debug")
* [Default](#impl-Default-for-ThreadPoolBuilder "Default")

### [Auto Trait Implementations](#synthetic-implementations)

* [!RefUnwindSafe](#impl-RefUnwindSafe-for-ThreadPoolBuilder%3CS%3E "!RefUnwindSafe")
* [!Send](#impl-Send-for-ThreadPoolBuilder%3CS%3E "!Send")
* [!Sync](#impl-Sync-for-ThreadPoolBuilder%3CS%3E "!Sync")
* [!UnwindSafe](#impl-UnwindSafe-for-ThreadPoolBuilder%3CS%3E "!UnwindSafe")
* [Freeze](#impl-Freeze-for-ThreadPoolBuilder%3CS%3E "Freeze")
* [Unpin](#impl-Unpin-for-ThreadPoolBuilder%3CS%3E "Unpin")
* [UnsafeUnpin](#impl-UnsafeUnpin-for-ThreadPoolBuilder%3CS%3E "UnsafeUnpin")

### [Blanket Implementations](#blanket-implementations)

* [Any](#impl-Any-for-T "Any")
* [Borrow<T>](#impl-Borrow%3CT%3E-for-T "Borrow<T>")
* [BorrowMut<T>](#impl-BorrowMut%3CT%3E-for-T "BorrowMut<T>")
* [From<T>](#impl-From%3CT%3E-for-T "From<T>")
* [Into<U>](#impl-Into%3CU%3E-for-T "Into<U>")
* [Pointable](#impl-Pointable-for-T "Pointable")
* [TryFrom<U>](#impl-TryFrom%3CU%3E-for-T "TryFrom<U>")
* [TryInto<U>](#impl-TryInto%3CU%3E-for-T "TryInto<U>")

[In crate rayon\_core](index.html)
----------------------------------

[rayon\_core](index.html)

Struct ThreadPoolBuilder Copy item path
=======================================

[Source](../src/rayon_core/lib.rs.html#170-202)

```
pub struct ThreadPoolBuilder<S = DefaultSpawn> { /* private fields */ }
```

Expand description

Used to create a new [`ThreadPool`](struct.ThreadPool.html "struct rayon_core::ThreadPool") or to configure the global rayon thread pool.

### [§](#creating-a-threadpool)Creating a ThreadPool

The following creates a thread pool with 22 threads.

[ⓘ](# "This example is not tested on wasm")

```
let pool = rayon::ThreadPoolBuilder::new().num_threads(22).build().unwrap();
```

To instead configure the global thread pool, use [`build_global()`](struct.ThreadPoolBuilder.html#method.build_global "method rayon_core::ThreadPoolBuilder::build_global"):

[ⓘ](# "This example is not tested on wasm")

```
rayon::ThreadPoolBuilder::new().num_threads(22).build_global().unwrap();
```

Implementations[§](#implementations)
------------------------------------

[Source](../src/rayon_core/lib.rs.html#242-247)[§](#impl-ThreadPoolBuilder)

### impl [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")

[Source](../src/rayon_core/lib.rs.html#244-246)

#### pub fn [new](#method.new)() -> Self

Creates and returns a valid rayon thread pool builder, but does not initialize it.

[Source](../src/rayon_core/lib.rs.html#251-282)[§](#impl-ThreadPoolBuilder%3CS%3E)

### impl<S> [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<S> where S: ThreadSpawn,

Note: the `S: ThreadSpawn` constraint is an internal implementation detail for the
default spawn and those set by [`spawn_handler`](#method.spawn_handler).

[Source](../src/rayon_core/lib.rs.html#256-258)

#### pub fn [build](#method.build)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ThreadPool](struct.ThreadPool.html "struct rayon_core::ThreadPool"), [ThreadPoolBuildError](struct.ThreadPoolBuildError.html "struct rayon_core::ThreadPoolBuildError")>

Creates a new `ThreadPool` initialized using this configuration.

[Source](../src/rayon_core/lib.rs.html#277-281)

#### pub fn [build\_global](#method.build_global)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ThreadPoolBuildError](struct.ThreadPoolBuildError.html "struct rayon_core::ThreadPoolBuildError")>

Initializes the global thread pool. This initialization is
**optional**. If you do not call this function, the thread pool
will be automatically initialized with the default
configuration. Calling `build_global` is not recommended, except
in two scenarios:

* You wish to change the default configuration.
* You are running a benchmark, in which case initializing may
  yield slightly more consistent results, since the worker threads
  will already be ready to go even in the first iteration. But
  this cost is minimal.

Initialization of the global thread pool happens exactly
once. Once started, the configuration cannot be
changed. Therefore, if you call `build_global` a second time, it
will return an error. An `Ok` result indicates that this
is the first initialization of the thread pool.

[Source](../src/rayon_core/lib.rs.html#284-344)[§](#impl-ThreadPoolBuilder-1)

### impl [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")

[Source](../src/rayon_core/lib.rs.html#322-343)

#### pub fn [build\_scoped](#method.build_scoped)<W, F, R>( self, wrapper: W, with\_pool: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<R, [ThreadPoolBuildError](struct.ThreadPoolBuildError.html "struct rayon_core::ThreadPoolBuildError")> where W: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([ThreadBuilder](struct.ThreadBuilder.html "struct rayon_core::ThreadBuilder")) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&[ThreadPool](struct.ThreadPool.html "struct rayon_core::ThreadPool")) -> R,

Creates a scoped `ThreadPool` initialized using this configuration.

This is a convenience function for building a pool using [`std::thread::scope`](https://doc.rust-lang.org/nightly/std/thread/scoped/fn.scope.html "fn std::thread::scoped::scope")
to spawn threads in a [`spawn_handler`](struct.ThreadPoolBuilder.html#method.spawn_handler "method rayon_core::ThreadPoolBuilder::spawn_handler").
The threads in this pool will start by calling `wrapper`, which should
do initialization and continue by calling `ThreadBuilder::run()`.

##### [§](#examples)Examples

A scoped pool may be useful in combination with scoped thread-local variables.

[ⓘ](# "This example is not tested on wasm")

```
scoped_tls::scoped_thread_local!(static POOL_DATA: Vec<i32>);

fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    let pool_data = vec![1, 2, 3];

    // We haven't assigned any TLS data yet.
    assert!(!POOL_DATA.is_set());

    rayon::ThreadPoolBuilder::new()
        .build_scoped(
            // Borrow `pool_data` in TLS for each thread.
            |thread| POOL_DATA.set(&pool_data, || thread.run()),
            // Do some work that needs the TLS data.
            |pool| pool.install(|| assert!(POOL_DATA.is_set())),
        )?;

    // Once we've returned, `pool_data` is no longer borrowed.
    drop(pool_data);
    Ok(())
}
```

[Source](../src/rayon_core/lib.rs.html#346-664)[§](#impl-ThreadPoolBuilder%3CS%3E-1)

### impl<S> [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<S>

[Source](../src/rayon_core/lib.rs.html#433-449)

#### pub fn [spawn\_handler](#method.spawn_handler)<F>(self, spawn: F) -> [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<CustomSpawn<F>> where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([ThreadBuilder](struct.ThreadBuilder.html "struct rayon_core::ThreadBuilder")) -> [Result](https://doc.rust-lang.org/nightly/core/io/error/type.Result.html "type core::io::error::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)>,

Sets a custom function for spawning threads.

Note that the threads will not exit until after the pool is dropped. It
is up to the caller to wait for thread termination if that is important
for any invariants. For instance, threads created in [`std::thread::scope`](https://doc.rust-lang.org/nightly/std/thread/scoped/fn.scope.html "fn std::thread::scoped::scope")
will be joined before that scope returns, and this will block indefinitely
if the pool is leaked. Furthermore, the global thread pool doesn’t terminate
until the entire process exits!

##### [§](#examples-1)Examples

A minimal spawn handler just needs to call `run()` from an independent thread.

[ⓘ](# "This example is not tested on wasm")

```
fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    let pool = rayon::ThreadPoolBuilder::new()
        .spawn_handler(|thread| {
            std::thread::spawn(|| thread.run());
            Ok(())
        })
        .build()?;

    pool.install(|| println!("Hello from my custom thread!"));
    Ok(())
}
```

The default spawn handler sets the name and stack size if given, and propagates
any errors from the thread builder.

[ⓘ](# "This example is not tested on wasm")

```
fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    let pool = rayon::ThreadPoolBuilder::new()
        .spawn_handler(|thread| {
            let mut b = std::thread::Builder::new();
            if let Some(name) = thread.name() {
                b = b.name(name.to_owned());
            }
            if let Some(stack_size) = thread.stack_size() {
                b = b.stack_size(stack_size);
            }
            b.spawn(|| thread.run())?;
            Ok(())
        })
        .build()?;

    pool.install(|| println!("Hello from my fully custom thread!"));
    Ok(())
}
```

This can also be used for a pool of scoped threads like [`crossbeam::scope`](https://docs.rs/crossbeam/0.8/crossbeam/fn.scope.html),
or [`std::thread::scope`](https://doc.rust-lang.org/nightly/std/thread/scoped/fn.scope.html "fn std::thread::scoped::scope") introduced in Rust 1.63, which is encapsulated in
[`build_scoped`](struct.ThreadPoolBuilder.html#method.build_scoped "method rayon_core::ThreadPoolBuilder::build_scoped").

[ⓘ](# "This example is not tested on wasm")

```
fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    std::thread::scope(|scope| {
        let pool = rayon::ThreadPoolBuilder::new()
            .spawn_handler(|thread| {
                let mut builder = std::thread::Builder::new();
                if let Some(name) = thread.name() {
                    builder = builder.name(name.to_string());
                }
                if let Some(size) = thread.stack_size() {
                    builder = builder.stack_size(size);
                }
                builder.spawn_scoped(scope, || {
                    // Add any scoped initialization here, then run!
                    thread.run()
                })?;
                Ok(())
            })
            .build()?;

        pool.install(|| println!("Hello from my custom scoped thread!"));
        Ok(())
    })
}
```

[Source](../src/rayon_core/lib.rs.html#496-502)

#### pub fn [thread\_name](#method.thread_name)<F>(self, closure: F) -> Self where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String") + 'static,

Sets a closure which takes a thread index and returns
the thread’s name.

[Source](../src/rayon_core/lib.rs.html#529-532)

#### pub fn [num\_threads](#method.num_threads)(self, num\_threads: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> Self

Sets the number of threads to be used in the rayon thread pool.

If you specify a non-zero number of threads using this
function, then the resulting thread pools are guaranteed to
start at most this number of threads.

If `num_threads` is 0, or you do not call this function, then
the Rayon runtime will select the number of threads
automatically. At present, this is based on the
`RAYON_NUM_THREADS` environment variable (if set),
or the number of logical CPUs (otherwise).
In the future, however, the default behavior may
change to dynamically add or remove threads as needed.

**Future compatibility warning:** Given the default behavior
may change in the future, if you wish to rely on a fixed
number of threads, you should use this function to specify
that number. To reproduce the current default behavior, you
may wish to use [`std::thread::available_parallelism`](https://doc.rust-lang.org/nightly/std/thread/functions/fn.available_parallelism.html "fn std::thread::functions::available_parallelism")
to query the number of CPUs dynamically.

**Old environment variable:** `RAYON_NUM_THREADS` is a one-to-one
replacement of the now deprecated `RAYON_RS_NUM_CPUS` environment
variable. If both variables are specified, `RAYON_NUM_THREADS` will
be preferred.

[Source](../src/rayon_core/lib.rs.html#547-550)

#### pub fn [use\_current\_thread](#method.use_current_thread)(self) -> Self

Use the current thread as one of the threads in the pool.

The current thread is guaranteed to be at index 0, and since the thread is not managed by
rayon, the spawn and exit handlers do not run for that thread.

Note that the current thread won’t run the main work-stealing loop, so jobs spawned into
the thread pool will generally not be picked up automatically by this thread unless you
yield to rayon in some way, like via [`yield_now()`](fn.yield_now.html "fn rayon_core::yield_now"), [`yield_local()`](fn.yield_local.html "fn rayon_core::yield_local"), or [`scope()`](fn.scope.html "fn rayon_core::scope").

##### [§](#local-thread-pools)Local thread pools

Using this in a local thread pool means the registry will be leaked. In future versions
there might be a way of cleaning up the current-thread state.

[Source](../src/rayon_core/lib.rs.html#571-577)

#### pub fn [panic\_handler](#method.panic_handler)<H>(self, panic\_handler: H) -> Self where H: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([Box](https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html "struct alloc::boxed::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Normally, whenever Rayon catches a panic, it tries to
propagate it to someplace sensible, to try and reflect the
semantics of sequential execution. But in some cases,
particularly with the `spawn()` APIs, there is no
obvious place where we should propagate the panic to.
In that case, this panic handler is invoked.

If no panic handler is set, the default is to abort the
process, under the principle that panics should not go
unobserved.

If the panic handler itself panics, this will abort the
process. To prevent this, wrap the body of your panic handler
in a call to `std::panic::catch_unwind()`.

[Source](../src/rayon_core/lib.rs.html#585-588)

#### pub fn [stack\_size](#method.stack_size)(self, stack\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> Self

Sets the stack size of the worker threads

[Source](../src/rayon_core/lib.rs.html#618-621)

#### pub fn [breadth\_first](#method.breadth_first)(self) -> Self

👎Deprecated:

use `scope_fifo` and `spawn_fifo` for similar effect

**(DEPRECATED)** Suggest to worker threads that they execute
spawned jobs in a “breadth-first” fashion.

Typically, when a worker thread is idle or blocked, it will
attempt to execute the job from the *top* of its local deque of
work (i.e., the job most recently spawned). If this flag is set
to true, however, workers will prefer to execute in a
*breadth-first* fashion – that is, they will search for jobs at
the *bottom* of their local deque. (At present, workers *always*
steal from the bottom of other workers’ deques, regardless of
the setting of this flag.)

If you think of the tasks as a tree, where a parent task
spawns its children in the tree, then this flag loosely
corresponds to doing a breadth-first traversal of the tree,
whereas the default would be to do a depth-first traversal.

**Note that this is an “execution hint”.** Rayon’s task
execution is highly dynamic and the precise order in which
independent tasks are executed is not intended to be
guaranteed.

This `breadth_first()` method is now deprecated per [RFC #1](https://github.com/rayon-rs/rfcs/blob/main/accepted/rfc0001-scope-scheduling.md),
and in the future its effect may be removed. Consider using
[`scope_fifo()`](fn.scope_fifo.html "fn rayon_core::scope_fifo") for a similar effect.

[Source](../src/rayon_core/lib.rs.html#638-644)

#### pub fn [start\_handler](#method.start_handler)<H>(self, start\_handler: H) -> Self where H: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Sets a callback to be invoked on thread start.

The closure is passed the index of the thread on which it is invoked.
Note that this same closure may be invoked multiple times in parallel.
If this closure panics, the panic will be passed to the panic handler.
If that handler returns, then startup will continue normally.

[Source](../src/rayon_core/lib.rs.html#657-663)

#### pub fn [exit\_handler](#method.exit_handler)<H>(self, exit\_handler: H) -> Self where H: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Sets a callback to be invoked on thread exit.

The closure is passed the index of the thread on which it is invoked.
Note that this same closure may be invoked multiple times in parallel.
If this closure panics, the panic will be passed to the panic handler.
If that handler returns, then the thread will exit normally.

Trait Implementations[§](#trait-implementations)
------------------------------------------------

[Source](../src/rayon_core/lib.rs.html#791-829)[§](#impl-Debug-for-ThreadPoolBuilder%3CS%3E)

### impl<S> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<S>

[Source](../src/rayon_core/lib.rs.html#792-828)[§](#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](../src/rayon_core/lib.rs.html#226-240)[§](#impl-Default-for-ThreadPoolBuilder)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")

[Source](../src/rayon_core/lib.rs.html#227-239)[§](#method.default)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

Auto Trait Implementations[§](#synthetic-implementations)
---------------------------------------------------------

[§](#impl-RefUnwindSafe-for-ThreadPoolBuilder%3CS%3E)

### impl<S = DefaultSpawn> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<S>

[§](#impl-Send-for-ThreadPoolBuilder%3CS%3E)

### impl<S = DefaultSpawn> ![Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<S>

[§](#impl-Sync-for-ThreadPoolBuilder%3CS%3E)

### impl<S = DefaultSpawn> ![Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<S>

[§](#impl-UnwindSafe-for-ThreadPoolBuilder%3CS%3E)

### impl<S = DefaultSpawn> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<S>

[§](#impl-Freeze-for-ThreadPoolBuilder%3CS%3E)

### impl<S> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<S> where S: [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"),

[§](#impl-Unpin-for-ThreadPoolBuilder%3CS%3E)

### impl<S> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<S> where S: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[§](#impl-UnsafeUnpin-for-ThreadPoolBuilder%3CS%3E)

### impl<S> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [ThreadPoolBuilder](struct.ThreadPoolBuilder.html "struct rayon_core::ThreadPoolBuilder")<S> where S: [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin"),

Blanket Implementations[§](#blanket-implementations)
----------------------------------------------------

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)[§](#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)[§](#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)[§](#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)[§](#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)[§](#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)[§](#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)[§](#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)[§](#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)[§](#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)[§](#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
`From<T> for U` chooses to do.

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#194)[§](#impl-Pointable-for-T)

### impl<T> [Pointable](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html "trait crossbeam_epoch::atomic::Pointable") for T

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#195)[§](#associatedconstant.ALIGN)

#### const [ALIGN](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#associatedconstant.ALIGN): [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

The alignment of pointer.

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#197)[§](#associatedtype.Init)

#### type [Init](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#associatedtype.Init) = T

The type for initializers.

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#199)[§](#method.init)

#### unsafe fn [init](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.init)(init: <T as [Pointable](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html "trait crossbeam_epoch::atomic::Pointable")>::[Init](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#associatedtype.Init "type crossbeam_epoch::atomic::Pointable::Init")) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Initializes a with the given initializer. [Read more](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.init)

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#203)[§](#method.deref)

#### unsafe fn [deref](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.deref)<'a>(ptr: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Dereferences the given pointer. [Read more](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.deref)

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#207)[§](#method.deref_mut)

#### unsafe fn [deref\_mut](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.deref_mut)<'a>(ptr: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably dereferences the given pointer. [Read more](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.deref_mut)

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#211)[§](#method.drop)

#### unsafe fn [drop](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.drop)(ptr: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Drops the object pointed to by the given pointer. [Read more](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.drop)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#814-816)[§](#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)[§](#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [!](https://doc.rust-lang.org/nightly/std/primitive.never.html)

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#821)[§](#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [!](https://doc.rust-lang.org/nightly/std/primitive.never.html)>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#798-800)[§](#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#802)[§](#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#805)[§](#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")>

Performs the conversion.