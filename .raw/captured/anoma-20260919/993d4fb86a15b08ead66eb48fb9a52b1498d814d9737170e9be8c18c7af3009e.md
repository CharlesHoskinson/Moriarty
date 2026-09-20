# Reproducibly generate proving and verifying keys (ELF and ImageID)

You may generate different ELFs and ImageIDs on different machines and environments. To reproduce the same output and publicly verify that the ELF and ImageID correspond to the generic call circuit source code, use the following tool and command.

```bash
cargo risczero build --manifest-path generic_call_circuit/methods/guest/Cargo.toml
```

will reproduce the output to:

```bash
ELFs ready at:
ImageID: <image-id>
generic_call_circuit/methods/guest/target/riscv32im-risc0-zkvm-elf/docker/generic-call-guest.bin
```

Note: The unstable feature of risc0-zkvm currently causes issues in circuits. This can be temporarily fixed by manually updating the tool. The problem will be fully resolved in the next release of RISC Zero.

```bash
cargo install --force --git https://github.com/risc0/risc0 --tag v3.0.3 -Fexperimental cargo-risczero
```
