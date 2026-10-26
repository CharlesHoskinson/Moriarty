# Circom witnesscalc
> Source: https://raw.githubusercontent.com/iden3/circom-witnesscalc/main/README.md
> Collected: 2026-10-26
> Published: Unknown

# circom-witnesscalc
## Description
This crate provides a fast witness generator for Circom circuits, serving as a drop-in replacement for Circom's witness generator.
It was created in response to the slow performance of Circom's WASM generator for larger circuits, which also necessitates a WASM runtime, often a cumbersome requirement.
Although the native C++ generator is faster, it requires embedding itself into a compiled binary, which is not always desirable.
The idea is to have a universal library that can calculate the witness for any Circom circuit without the need for a WASM runtime or embedding a C++ binary.
`circom-witnesscalc` comes with two executables:
1. `build-circuit` command to build a Circom circuit. As a result, you will get a binary file of graph operations to calculate the witness for a circuit.
2. `calc-witness` command uses the generated binary file and circuit inputs to generate a witness. This functionality is also availabe as a Rust or C library API.
The project originally inspired by [circom-witness-rs](https://github.com/philsippl/circom-witness-rs).
## CVM (Circom Virtual Machine) Witness Generator
This project also includes an alternative witness calculation method using the Circom Virtual Machine (CVM). The CVM approach provides a different compilation pipeline for witness generation.
\*\*Note\*\*: The CVM feature is not production ready and is currently in active development. Users are welcome to file bug reports if they encounter circuits where it does not work correctly.
For detailed documentation on using the CVM-based witness calculator, including:
- Building and using the `cvm-compile` tool
- Single-step vs two-step compilation modes
- Testing with the `test\_circuits\_vm2.sh` script
- Integration with the circom\_cvm fork
See \*\*[cvm.md](cvm.md)\*\* for complete instructions.
## Unimplemented features
There are some Circom features that are not yet implemented. If you need these features,
please open an issue with the Circom circuit that doesn't work.
In the near future, we plan to add support for using input signals in the loop condition to accommodate circuits that uses the `long\_div` function from the [zk-email](https://github.com/zkemail/zk-email-verify/blob/8685d35f9137ea566e0a07f6609fde0123d15f51/packages/circuits/lib/bigint-func.circom#L169) project.
## Compile a circuit and build the witness graph
To create a circuit graph file from a Circom 2 program, run the following command:
```shell
# Using compiled binary
./build-circuit  [-l ]\* [-i ]
# Or using `cargo` from the root of the repository
cargo run --package circom-witnesscalc --bin build-circuit  [-l ]\* [-i ]
```
Run `./build-circuit --help` to see the available options.
To build on Ubuntu you need to install `clang-19` and `protobuf-compiler` packages.
## Calculate witness from circuit graph created on previous step
To generate a witness file from a circuit graph and inputs, run the following command.
```shell
# Using compiled binary
./calc-witness 
# Or using `cargo` from the root of the repository
cargo run --package circom-witnesscalc --bin calc-witness 
```
## Run circuits tests
To run circuits tests, we need to make some manual setup
```shell
# Update the git submodules to checkout all required dependencies
git submodule update --init --recursive
# Install dependencies for iden3 authV2 circuits
pushd test\_deps/iden3-circuits-authV2
npm install
popd
# Install dependencies for master branch of iden3 circuits
pushd test\_deps/iden3-circuits-master
npm install
popd
```
Also, you need to have the following commands installed: `circom`, `snarkjs`,
`curl`, `cargo`, `node` and `cmp`.
Now run the `test\_circuits.sh` script.
```shell
./test\_circuits.sh
```
### Testing backward compatibility with graph\_v1
To verify that circuits work with the older graph format (v1), you need to test
against the `build-circuit` from version v0.1.1.
1. Clone and build the v0.1.1 version:
```shell
git clone --branch build-circuit/v0.1.1 https://github.com/iden3/circom-witnesscalc.git ~/src/circom-witnesscalc-build-circuit-v0.1.1
cd ~/src/circom-witnesscalc-build-circuit-v0.1.1
cargo build --release -p build-circuit
```
2. Run tests with the old build-circuit and the `graph\_v1` tag:
```shell
./test\_circuits.sh -b ~/src/circom-witnesscalc-build-circuit-v0.1.1/target/release/build-circuit -t graph\_v1
```
Circuits not compatible with graph\_v1 (tagged with `!graph\_v1`) will be skipped.
## Build for iOS & iOS Simulator
```shell
cargo build --target aarch64-apple-ios --release
cargo build --target aarch64-apple-ios-sim --release
install\_name\_tool -id @rpath/libcircom\_witnesscalc.dylib $PWD/target/aarch64-apple-ios/release/libcircom\_witnesscalc.dylib
install\_name\_tool -id @rpath/libcircom\_witnesscalc.dylib $PWD/target/aarch64-apple-ios-sim/release/libcircom\_witnesscalc.dylib
```
## Build for Android
You should have ANDROID\_NDK\_ROOT environment variable set to the path to the Android NDK.
It may be somewhere like `~/Library/Android/sdk/ndk/26.2.11394342`.
```shell
CC=${ANDROID\_NDK\_ROOT}/toolchains/llvm/prebuilt/darwin-x86\_64/bin/aarch64-linux-android29-clang \
CARGO\_TARGET\_AARCH64\_LINUX\_ANDROID\_LINKER=${CC} \
CLANG\_PATH=${CC} \
cargo build --target aarch64-linux-android --release
CC=${ANDROID\_NDK\_ROOT}/toolchains/llvm/prebuilt/darwin-x86\_64/bin/x86\_64-linux-android29-clang \
CARGO\_TARGET\_X86\_64\_LINUX\_ANDROID\_LINKER=${CC} \
CLANG\_PATH=${CC} \
cargo build --target x86\_64-linux-android --release
```
### Alternative way to build for Android
If you have cargo-ndk installed, you can build the project, for example,
with the following command:
```shell
cargo ndk --target aarch64-linux-android build --release
```
## Mobile wrappers. Releases with graph version only
| Wrapper | Repository Link | Version |
| ------------ |-----------------------------------------| ------- |
| React Native | https://github.com/iden3/react-native-circom-witnesscalc | 0.0.1-alpha.4 |
| Flutter | https://github.com/iden3/circom-witnesscalc-flutter | 0.0.1-alpha.3 |
| iOS | https://github.com/iden3/circom-witnesscalc-swift | 0.0.1-alpha.3 |
| Android | https://github.com/iden3/circom-witnesscalc-android | 0.0.1-alpha.3 |