Rust FFI Bindings for Program | ICICLE Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

🚀 We just released **ICICLE v4** — featuring a more intuitive API with lattices. [Check out the migration guide](/start/integration-&-support/migrate_from_v3).

[![Ingonyama Logo](/img/icicledocslogo.png)![Ingonyama Logo](/img/icicledocslogo.png)](/)[Docs](/)[API](/apioverview)

[Go to ICICLE](https://github.com/ingonyama-zk/icicle)

[4.0.0](/api/rust-bindings/program)

* [4.0.0](/api/rust-bindings/program)
* [3.9.0](/3.9.0/api/rust-bindings/program)
* [3.8.0](/3.8.0)
* [3.7.0](/3.7.0)
* [3.6.0](/3.6.0)
* [3.5.0](/3.5.0)
* [3.4.0](/3.4.0)
* [3.3.0](/3.3.0)
* [3.2.0](/3.2.0)
* [3.1.0](/3.1.0)
* [3.0.0](/3.0.0)
* [2.8.0](/2.8.0)
* [1.10.1](/1.10.1)

[Community](#)

* [Discord](https://discord.gg/6vYrE7waPj)
* [LinkedIn](https://www.linkedin.com/company/ingonyama)
* [X/Twitter](https://x.com/Ingo_zk)
* [YouTube](https://www.youtube.com/@ingo_ZK)
* [Mailing List](https://wkf.ms/3LKCbdj)

[Leave Feedback](https://forms.monday.com/forms/7b51e0bdad766b71da8869704c301472?r=use1)

[![Ingonyama logo](/img/Ingologo.svg)](https://ingonyama.com "Visit Ingonyama")

Search

* [API Overview](/apioverview)
* [C++](/cppstart)

  + [C++ Overview](/cppstart)
  + [MSM](/api/cpp/msm)
  + [NTT / ECNTT](/api/cpp/ntt)
  + [Vector operations](/api/cpp/vec_ops)
  + [Program](/api/cpp/program)
  + [Polynomials](/api/cpp/polynomials/overview)
  + [Hash](/api/cpp/hash)
  + [Merkle-Tree](/api/cpp/merkle)
  + [Sumcheck](/api/cpp/sumcheck)
  + [FRI](/api/cpp/fri)
  + [Pairings](/icicle/primitives/pairings)
  + [PQC ML-KEM](/api/cpp/lattice/pqc_ml_kem)
  + [Serialization](/api/cpp/serialization)
  + [Matrix Operations](/api/cpp/matrix_ops)
* [Golang Bindings](/gooverview)

  + [Golang Overview](/gooverview)
  + [MSM](/api/golang-bindings/msm)
  + [MSM pre-computation](/api/golang-bindings/msm-pre-computation)
  + [NTT](/api/golang-bindings/ntt)
  + [EC-NTT](/api/golang-bindings/ecntt)
  + [Vector operations](/api/golang-bindings/vec-ops)
  + [Multi GPU Support](/api/golang-bindings/multi-gpu)
  + [Hash](/api/golang-bindings/hash)
  + [Merkle-Tree](/api/golang-bindings/merkle)
  + [PQC ML-KEM](/api/golang-bindings/lattice/pqc-ml-kem)
* [Rust Bindings](/rustoverview)

  + [Rust Overview](/rustoverview)
  + [MSM](/api/rust-bindings/msm)
  + [NTT](/api/rust-bindings/ntt)
  + [ECNTT](/api/rust-bindings/ecntt)
  + [Vector operations](/api/rust-bindings/vec-ops)
  + [Program](/api/rust-bindings/program)
  + [Polynomials](/api/rust-bindings/polynomials)
  + [Hash](/api/rust-bindings/hash)
  + [Merkle-Tree](/api/rust-bindings/merkle)
  + [Sumcheck](/api/rust-bindings/sumcheck)
  + [FRI](/api/rust-bindings/fri)
  + [Lattice SNARKs](/api/rust-bindings/lattice/lattice-snarks)
  + [PQC ML-KEM](/api/rust-bindings/lattice/pqc-ml-kem)
  + [Serialization](/api/rust-bindings/serialization)
  + [Pairings](/api/rust-bindings/pairing)
  + [Matrix Operations](/api/rust-bindings/matrix_ops)

* Rust Bindings
* Program
Version: 4.0.0

On this page

Rust FFI Bindings for Program
=============================

note

Please refer to the [Program overview](/api/cpp/program) page for additional detail. This section is a brief description of the Rust FFI bindings.

This documentation is designed to bring developers up to speed about the Rust API wrapping the cpp implementation of program.

Introduction[​](#introduction "Direct link to Introduction")
------------------------------------------------------------

Program is a class that lets users define expressions on vector elements, and have ICICLE compile it for the backends for a fused implementation. This solves memory bottlenecks and also lets users customize algorithms such as sumcheck. Program can create only element-wise lambda functions.

The following lists the implemented Rust functionality with some examples paralleling those given in the [original program overview](/api/cpp/program).

Symbol
======

Symbol is the basic (template) class that allows users to define their own program, representing an arithmetic operation. The [function](#defining-a-function-for-program) the user defines will operate on symbols.

`Symbol` Trait Definition[​](#symbol-trait-definition "Direct link to symbol-trait-definition")
-----------------------------------------------------------------------------------------------

The trait defines the functionality required by the user. The expected use-case of symbol is solely to be operated on to create the final arithmetic operation, which is reflected in the implemented functions and traits.

```
pub trait Symbol<T: IntegerRing>:  
    Add<Output = Self>  
    + Sub<Output = Self>  
    + Mul<Output = Self>  
    + Add<T, Output = Self>  
    + Sub<T, Output = Self>  
    + Mul<T, Output = Self>  
    + AddAssign  
    + SubAssign  
    + MulAssign  
    + AddAssign<T>  
    + SubAssign<T>  
    + MulAssign<T>  
    + for<'a> Add<&'a Self, Output = Self>  
    + for<'a> Sub<&'a Self, Output = Self>  
    + for<'a> Mul<&'a Self, Output = Self>  
    + for<'a> AddAssign<&'a Self>  
    + for<'a> SubAssign<&'a Self>  
    + for<'a> MulAssign<&'a Self>  
    + Clone  
    + Copy  
    + Sized  
    + Handle  
{  
    fn new_input(in_idx: u32) -> Result<Self, IcicleError>; // New input symbol for the execution function  
    fn from_constant(constant: T) -> Result<Self, IcicleError>; // New symbol from a ring element  
}
```

`RingSymbol` Struct[​](#ringsymbol-struct "Direct link to ringsymbol-struct")
-----------------------------------------------------------------------------

The `RingSymbol` struct is implemented for each of the supported Icicle fields, providing an implementation of the above trait for each specific field. The distinction between fields is relevant for input symbols and stored constants in the program. At its core, it is simply a handle to the C++ implementation.

note

Despite the name, `RingSymbol` can be instantiated for both rings and fields.

```
pub struct RingSymbol {  
    m_handle: SymbolHandle,  
}
```

### Traits implemented and key methods[​](#traits-implemented-and-key-methods "Direct link to Traits implemented and key methods")

Additional traits the struct implements to fulfill `Symbol<T>` trait that should be noted.

#### Arithmetic operations[​](#arithmetic-operations "Direct link to Arithmetic operations")

`RingSymbol` implements addition, subtraction, and multiplication (as well as the assign variants of them) with other symbols/references as well as field elements. Applying the operations will generate a new symbol (or overwrite the existing in the case of the assign operation) representing the arithmetic operations of the two operand symbols.

Program
=======

A program to be run on the various Icicle backends. It can be either a user-defined program, or one of the members of `PreDefinedProgram` enum. The program adheres to one of the following traits:

`ProgramImpl` Trait Definition[​](#programimpl-trait-definition "Direct link to programimpl-trait-definition")
--------------------------------------------------------------------------------------------------------------

The trait defines the base functionality required for the user, which in this case is creation and execution. It is used as a program for running a function that takes a vector of field elements (both inputs and outputs) and has no return value (output is written to the given vector). It is executed through Vector Operations.

```
pub trait ProgramImpl<T>: Sized + Handle  
where  
    T: IntegerRing,  
{  
    type ProgSymbol: Symbol<T>;  
  
    fn new(program_func: impl FnOnce(&mut Vec<Self::ProgSymbol>), nof_parameters: u32) -> Result<Self, IcicleError>;  
  
    fn new_predefined(pre_def: PreDefinedProgram) -> Result<Self, IcicleError>;  
  
    fn execute_program<Data>(&self, data: &mut Vec<&Data>, cfg: &VecOpsConfig) -> Result<(), IcicleError>  
    where  
        Data: HostOrDeviceSlice<T> + ?Sized;  
}
```

`ReturningValueProgramImpl` Trait Definition[​](#returningvalueprogramimpl-trait-definition "Direct link to returningvalueprogramimpl-trait-definition")
--------------------------------------------------------------------------------------------------------------------------------------------------------

This trait is for programs that return a value (symbol) as output.

```
pub trait ReturningValueProgramImpl: Sized + Handle {  
    type Ring: IntegerRing;  
    type ProgSymbol: Symbol<Self::Ring>;  
  
    fn new(  
        program_func: impl FnOnce(&mut Vec<Self::ProgSymbol>) -> Self::ProgSymbol,  
        nof_parameters: u32,  
    ) -> Result<Self, IcicleError>;  
  
    fn new_predefined(pre_def: PreDefinedProgram) -> Result<Self, IcicleError>;  
}
```

`Program` Struct[​](#program-struct "Direct link to program-struct")
--------------------------------------------------------------------

The `Program` struct is implemented for each of the supported icicle fields, implementing the above trait for the specific field (field distinction is relevant for the input symbols and stored constants in the program). At its core, it's just a handle to the cpp implementation.

```
pub struct Program {  
    m_handle: ProgramHandle,  
}
```

Usage
=====

This section will outline how to use Program and Symbol, mirroring the examples from the [cpp overview](/api/cpp/program). The program use-case splits to three steps:

1. Defining a function/lambda that describes the program to be run (or choosing one of the predefined list).
2. Creating a new program given the above function.
3. Executing the program using the Vector Operations API.

Defining a Function for Program[​](#defining-a-function-for-program "Direct link to Defining a Function for Program")
---------------------------------------------------------------------------------------------------------------------

A function operating on a vector of symbols, with outputs being written to said input vector. The input symbols in the vector represent inputs and outputs of field elements, and will be replaced by vectors of field elements when executed.

note

The defined function defines arithmetic operations to be done in series, and could be represented as a set of equations (for each output). Practically, control flow (e.g., loops, conditions) is not parsed, instead the computation follows the exact execution path taken during tracing, which determines the final computation that will be performed.

```
fn example_function<T, S>(vars: &mut Vec<S>)  
where  
    T: IntegerRing,  
    S: Symbol<T>,  
{  
    let a = vars[0];  
    let b = vars[1];  
    let c = vars[2];  
    let eq = vars[3];  
  
    vars[4] = eq * (a * b - c) + T::from_u32(9);  
    vars[5] = a * b - c; // For division, use an Invertible trait if implemented  
    vars[6] = vars[5];  
    vars[3] = (vars[0] + vars[1]) * T::from_u32(2); // all variables can be both inputs and outputs  
}
```

Creating a Program[​](#creating-a-program "Direct link to Creating a Program")
------------------------------------------------------------------------------

Applying the constructor with the lambda:

```
let program = Program::new(example_function, 7 /* number of parameters for lambda = vars.len() */)?;
```

Executing the Program[​](#executing-the-program "Direct link to Executing the Program")
---------------------------------------------------------------------------------------

Execution is done through the appropriate vecops function:

```
program.execute_program(&mut parameters, &cfg)?;
```

Where `parameters` is a `Vec<&Parameter>` and `cfg` is a `VecOpsConfig`.

### Examples[​](#examples "Direct link to Examples")

Example taken from check\_program in vec\_ops tests.

#### Program functionality with a custom function[​](#program-functionality-with-a-custom-function "Direct link to Program functionality with a custom function")

```
pub fn check_program<T, Prog>()  
where  
    T: IntegerRing,  
    Prog: ProgramImpl<T>,  
{  
    let example_lambda = |vars: &mut Vec<Prog::ProgSymbol>| {  
        let a = vars[0];  
        let b = vars[1];  
        let c = vars[2];  
        let d = vars[3];  
  
        vars[4] = d * (a * b - c) + T::from_u32(9);  
        vars[5] = a * b - c; // For division, use an Invertible trait if implemented  
        vars[6] = vars[5];  
        vars[3] = (vars[0] + vars[1]) * T::from_u32(2);  
    };  
  
    // Additional lines for initiating the slices of field elements for the parameters  
  
    let mut parameters = vec![a_slice, b_slice, c_slice, eq_slice, var4_slice, var5_slice, var6_slice];  
  
    let program = Prog::new(example_lambda, 7).unwrap();  
  
    let cfg = VecOpsConfig::default();  
    program.execute_program(&mut parameters, &cfg).expect("Program Failed");  
}
```

#### Program functionality with predefined programs[​](#program-functionality-with-predefined-programs "Direct link to Program functionality with predefined programs")

```
pub fn check_predefined_program<T, Prog>()  
where  
    T: IntegerRing,  
    Prog: ProgramImpl<T>,  
{  
    // Additional lines for initiating the slices of field elements for the parameters  
    let mut parameters = vec![a_slice, b_slice, c_slice, eq_slice, var4_slice];  
  
    let program = Prog::new_predefined(PreDefinedProgram::EQtimesABminusC).unwrap();  
  
    let cfg = VecOpsConfig::default();  
    program.execute_program(&mut parameters, &cfg).expect("Program Failed");  
}
```

[Edit this page](https://github.com/ingonyama-zk/icicle/tree/main/docs/versioned_docs/version-4.0.0/api/rust-bindings/program.md)

Last updated on **Aug 6, 2025**

[Previous

Vector operations](/api/rust-bindings/vec-ops)[Next

Polynomials](/api/rust-bindings/polynomials)

* [Introduction](#introduction)
* [`Symbol` Trait Definition](#symbol-trait-definition)
* [`RingSymbol` Struct](#ringsymbol-struct)
  + [Traits implemented and key methods](#traits-implemented-and-key-methods)
* [`ProgramImpl` Trait Definition](#programimpl-trait-definition)
* [`ReturningValueProgramImpl` Trait Definition](#returningvalueprogramimpl-trait-definition)
* [`Program` Struct](#program-struct)
* [Defining a Function for Program](#defining-a-function-for-program)
* [Creating a Program](#creating-a-program)
* [Executing the Program](#executing-the-program)
  + [Examples](#examples)

Copyright © 2025 Ingonyama, Inc. Built with Docusaurus.