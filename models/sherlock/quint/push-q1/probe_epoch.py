#!/usr/bin/env python3
"""Probe resumeEpoch's source projection using an existing local mori binary.

This diagnostic never builds Rust and never touches a real contract or witness.
An exit of zero means the probe ran, not that the implementation is safe. Inspect
the maximum-epoch result; the current source projection predicts a debug panic.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SOURCE = '''language "moriarty-sherlock-draft/2";
program PendingInner {
  fn inner(n: UInt): UInt { await reply(v: UInt) when v == n + 1; v }
  fn outer(): UInt { 10 + inner(3) }
  entry outer();
}
'''


def main() -> int:
    binary = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else (
        Path.home() / "repos/Moriarty-wt-factory-u/target/debug/mori")
    result = {
        "scope": "Executed host diagnostic on a public synthetic checkpoint; no native proof or financial operation.",
        "binary": str(binary),
        "binarySha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "source": SOURCE,
        "cases": [],
    }
    with tempfile.TemporaryDirectory(prefix=".epoch-probe-", dir=HERE) as directory:
        work = Path(directory)
        config = json.loads((ROOT / "qa/holdout/config/draft-host-config.json").read_text())
        config["profile"] = "sherlock-helper-microstep/2"
        result["config"] = config
        (work / "config.json").write_text(json.dumps(config))
        (work / "source.mori").write_text(SOURCE)
        (work / "target.json").write_text("{}")
        base = [str(binary), "simulate", "source.mori", "--execution-profile",
                "sherlock-helper-microstep/2", "--source-profile",
                "moriarty-sherlock-draft/2", "--execution-config", "config.json",
                "--target", "target.json"]

        def invoke(label: str, arguments: list[str]) -> dict:
            started = time.monotonic()
            command = base + arguments
            process = subprocess.run(command, cwd=work, text=True, capture_output=True,
                                     timeout=20, check=False)
            row = {"label": label, "command": command, "exit": process.returncode,
                   "stdout": process.stdout, "stderr": process.stderr,
                   "wallSeconds": round(time.monotonic() - started, 4)}
            result["cases"].append(row)
            return row

        start = invoke("start", ["--entry", "--checkpoint-out", "waiting.bin"])
        assert start["exit"] == 0 and json.loads(start["stdout"])["status"] == "waiting"
        original = (work / "waiting.bin").read_bytes()
        assert original[:8] == b"SHK2CKP1" and original[8] == 0
        assert len(original) == 17 + struct.unpack("<Q", original[9:17])[0] + 32
        for label, epoch, value in (
            ("ordinaryAccept", 0, "4"),
            ("lastRepresentableAccept", 2**64 - 2, "4"),
            ("maximumEpochFalseGuard", 2**64 - 1, "5"),
            ("maximumEpochTrueGuard", 2**64 - 1, "4"),
        ):
            blob = bytearray(original)
            # Pure body begins with table digest[32], then Header.epoch[8].
            blob[49:57] = struct.pack("<Q", epoch)
            digest = hashlib.sha256(b"SHK_CK2" + bytes([3]) + struct.pack("<H", 2)
                                    + blob[:17] + blob[17:-32]).digest()
            blob[-32:] = digest
            (work / "input.bin").write_bytes(blob)
            observation = {"format": "sherlock-observation/2", "checkpoint": digest.hex(),
                           "site": 1, "label": "reply", "values": [{"name": "v",
                           "value": {"type": "UInt", "value": value}}]}
            (work / "observation.json").write_text(json.dumps(observation))
            output = work / "output.bin"
            output.unlink(missing_ok=True)
            row = invoke(label, ["--resume", "input.bin", "--observation", "observation.json",
                                 "--checkpoint-out", "output.bin"])
            row.update({"epoch": str(epoch), "observationValue": value,
                        "inputUnchanged": (work / "input.bin").read_bytes() == blob,
                        "outputExists": output.exists(),
                        "outputEqualsInput": output.exists() and output.read_bytes() == blob})
    (HERE / "epoch-rust-probe.json").write_text(json.dumps(result, indent=2) + "\n")
    for row in result["cases"]:
        print(f"{row['label']}: exit={row['exit']} wall={row['wallSeconds']}s")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
