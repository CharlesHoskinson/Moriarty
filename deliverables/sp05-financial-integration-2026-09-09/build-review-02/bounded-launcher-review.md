# Independent bounded launcher review

**CHANGES REQUESTED.** Two disk-boundary defects were reproduced in the original launcher. The five supplied offline tests pass.

Reviewed only `run-build-bounded.py` SHA-256 `539b99a45554dfe6a42038c555ebf73c756d4eff9eedd04656f627968104cc73` and its test file SHA-256 `8bbef402b33b0e79e62349e81d5a594e539cc0d26ca1986e017d959f83b1bcc9`. These findings do not approve subsequent edits. Reviewer: independent GPT-6 Astra, `/root/sp05_integration_audit`.

1. **High: retention defeats the disk ceiling** (`run-build-bounded.py:100-113`). `copyfileobj` materializes sparse holes and copies each hard-link path independently to the unbounded host mirror. In an actual 64 KiB tmpfs, an inert child created a 262,144-byte sparse file, one 16,384-byte file and eight hard links. `quota_run` returned zero and retained **409,600 allocated host bytes** across ten files. Add a cumulative retention ceiling, an explicit sparse/hardlink policy and growth-safe streaming enforcement before dispatch. Preflight size checks alone do not protect against source changes during copying.
2. **High: child can write directly to the host mirror** (`run-build-bounded.py:126-132`). The original host output directory is mounted at the mirror before the child starts in the same namespace. An inert child wrote **131,072 host bytes** directly to this mirror under a 65,536-byte tmpfs quota and returned zero. A private mount namespace changes mount visibility; it does not restrict writable host paths. Prevent child access to unbounded host destinations if relying on a hard disk confinement claim. Otherwise explicitly narrow the claim and obtain the applicable resource decision; fixing retention alone does not address this path.

Both probes used the same `unshare --user --map-root-user --mount --fork` arrangement as the supplied test, calling `quota_run(parent, mirror, inertPythonCommand,65536)`. Sparse payload: `f.truncate(262144)` on `parent/sparse`, write 16,384 bytes to `parent/linked`, then `os.link` it eight times. Direct-write payload: `(mirror/'direct-host-output').write_bytes(b'x'*131072)`. Both were reproduced offline and temporary files removed. The companion JSON records exact results.

The supplied command `python3 experiments/moriarty-midnight-financial/ledger/run-build-bounded.test.py` passed all five tests, including actual ordinary-write ENOSPC and same-path retention. The output tmpfs works for ordinary cooperative writes to that mount.

Cgroup source configuration checks 4 GiB memory, zero swap, 330 seconds and control-group termination; no actual cgroup exhaustion/kill experiment was run here. Terminal empty-cgroup checks and the external 720-second aggregate across serial loan/swap attempts remain required. A zero wrapper exit correctly keeps `accepted:false` and `terminalCgroupVerified:false`; JSON bindings do not authenticate reviewers. Abrupt termination can lose volatile partial artifacts as documented.

No compiler, proof, wallet or network operation was performed. Neither launcher nor builder was edited by this reviewer. This review grants no resource, build or dispatch acceptance.
