> For the complete documentation index, see [llms.txt](/llms.txt)

# Troubleshoot Compact toolchain installation issue on NixOS

Compact is Midnight’s dedicated smart contract programming language designed for building secure, efficient and adaptable decentralized applications. Compact tools are command-line utility for installing, updating, managing, and running the compact toolchain and compiler. This guide walks you through installing Midnight compact tools on NixOS.

### Compiler version scope[​](#compiler-version-scope "Direct link to Compiler version scope")

note

Installation problems only concern versions of the compact compiler **0.26**. A fix has already been made and the guide will become obsolete upon official release of version 0.27.

By the end of the guide, you will:

* Understand why standard installations fail on NixOS.
* Install and run compact for Midnight development.
* Fix “No such file or directory” errors for `compactc`.

### Prerequisites[​](#prerequisites "Direct link to Prerequisites")

* NixOS version 23.11

  <!-- -->

  * 24.05
  * 25.11 **(recommended stable release)**

Familiarity with command-line interfaces.

***

## Why standard installations fail on NixOS[​](#why-standard-installations-fail-on-nixos "Direct link to Why standard installations fail on NixOS")

Standard installations fail on NixOS because unlike traditional Linux distributions it is not configured to follow the Filesystem Hierarchy Standard (FHS) which defines the directory structure and directory content in Linux operating systems. The main reason for this deviation is because it does not provide isolation, it instead uses the filesystem as a database and to store packages in isolation in a special directory called the Nix store. This causes issues when applications download binaries that expect a traditional FHS structure.

## Install the Compact compiler (FHS Environment)[​](#install-the-compact-compiler-fhs-environment "Direct link to Install the Compact compiler (FHS Environment)")

To install the compiler, use `buildFHSEnv` a tool that creates a lightweight sandbox environment that simulates a traditional FHS system. It creates an isolated root filesystem with the host’s `/nix/store`, so its footprint in terms of disk space is quite small. This allows you to run software which is hard or unfeasible to patch for NixOS.

### Create shell.nix[​](#create-shellnix "Direct link to Create shell.nix")

Create a file called `shell.nix` with the code snippet below:

```
{ pkgs ? import <nixpkgs> {} }:

pkgs.buildFHSEnv {

  name = "x-11";

  targetPkgs = pkgs: [ pkgs.bash pkgs.glibc pkgs.curl];

  runScript = "exec ${pkgs.bash}/bin/bash";

}
```

Alternative for macOS (mkShell) If you are on macOS, use this version of the file instead:

```
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {

  name = "x-11";

  buildInputs = with pkgs; [

    bash

    glibc

    curl

  ];

}
```

### Run nix-shell[​](#run-nix-shell "Direct link to Run nix-shell")

Run this command to enter the environment:

```
nix-shell
```

### Download the Compact compiler[​](#download-the-compact-compiler "Direct link to Download the Compact compiler")

Inside the shell, run the installer:

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/midnightntwrk/compact/releases/latest/download/compact-installer.sh | sh
```

### Update Compact[​](#update-compact "Direct link to Update Compact")

Once installed, update to the latest version:

```
compact update
```

## Fix “No such file or directory (os error 2)” error[​](#fix-no-such-file-or-directory-os-error-2-error "Direct link to Fix “No such file or directory (os error 2)” error")

Even within the FHS environment, the installed Compact wrapper script `compactc` often fails when `compact compile -help` is run because it contains a hardcoded shebang (`#!`) line pointing to `bin/bash`, which the installer does not correctly resolve at the exact path resulting to a `No such file or directory(os error 2)`error. **To fix this**: Modify the installed Compact wrapper script to correctly point to the `bash` binary that is accessible inside the FHS sandbox.

Create a file called `compact_util.sh` in the root directory. This file serves as the utility script:

```
#!/usr/bin/env bash

cd ./.compact/versions/0.26.0/x86_64-unknown-linux-musl

sed -i '1 i #!/usr/bin/env bash' compactc
```

Make script executable, run this command to change permissions:

```
chmod +x compact_util.sh
```

Run the utility script:

```
./compact_util.sh
```

Verify installation,check that the compiler is now accessible:

```
compact compile --help
```

## Next steps[​](#next-steps "Direct link to Next steps")

Midnight is a blockchain platform for building privacy-preserving DApps. It enables developers to define how data is isolated, verified, and shared through zero-knowledge (ZK) proofs and programmable confidentiality controls. [To get started with Midnight](/getting-started.md).
