# WSL dependencies

Installed in Ubuntu 26.04 on WSL2. Activate the existing local tools with:

```bash
source ~/.config/moriarty/environment.sh
source /home/charl/Moriarty/.venv/bin/activate
```

The environment selects Node 24.21.0, TypeScript 5.9.3, OpenSpec 1.13.1 and Compact manager 0.5.2 with compiler 0.31.1. Python dependencies follow `uv.lock`; npm dependencies follow each active package lock. Chromium launches successfully inside WSL.

K 7.1.337, Java 21, LLVM/Clang 17, build dependencies and GDB are installed as Linux packages. Package versions and download digests are in `wsl-environment.json`. The K Debian package came from the official runtimeverification/k v7.1.337 release. Its Ubuntu Noble dependency libsecp256k1-1 came from archive.ubuntu.com and is installed alongside current libraries.

This installation is a new build provenance. It does not recover the historical Nix interpreter or establish native K conformance. Historical toolchain locks remain intact.
