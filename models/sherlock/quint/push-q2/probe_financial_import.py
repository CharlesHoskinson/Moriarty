#!/usr/bin/env python3
"""Probe financialImport's and financialSegments' source projections using an existing local mori binary.

This diagnostic never builds Rust and never touches a real contract or witness.
An exit of zero means the probe ran, not that the implementation is safe. Inspect
the resealed-counter, zero-fuel-waiting and segment-index results.
"""
from __future__ import annotations

import copy
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
U128_MAX = str(2**128 - 1)

SOURCE = '''language "moriarty/2";
agreement F01PaidPrefixes {
  asset U = { network: "local", issuer: "f01-oracle", reference: "U", scale: 0 };
  state marker: UInt = 0;
  action open() {
    liability("pay-beneficiary", U, "reserve", "beneficiary", 40);
  }
  action pay() {
    fee(U, "reserve", "fee_recipient", 2);
    transfer(U, "reserve", "beneficiary", 30);
    discharge("pay-beneficiary", 30);
    await second(go: Bool) when go;
    fee(U, "reserve", "fee_recipient", 1);
    transfer(U, "reserve", "beneficiary", 10);
    discharge("pay-beneficiary", 10);
  }
}
'''

SCENARIO = {
    "balances": [
        {
            "account": "beneficiary",
            "amount": "0",
            "asset": {"issuer": "f01-oracle", "network": "local", "reference": "U", "scale": 0},
        },
        {
            "account": "fee_recipient",
            "amount": "0",
            "asset": {"issuer": "f01-oracle", "network": "local", "reference": "U", "scale": 0},
        },
        {
            "account": "reserve",
            "amount": "100",
            "asset": {"issuer": "f01-oracle", "network": "local", "reference": "U", "scale": 0},
        },
    ],
    "inputs": {},
    "liabilities": [
        {
            "amount": "40",
            "asset": {"issuer": "f01-oracle", "network": "local", "reference": "U", "scale": 0},
            "creditor": "beneficiary",
            "debtor": "reserve",
            "name": "pay-beneficiary",
        }
    ],
    "state": {"marker": {"type": "UInt", "value": "0"}},
}


def seal_fin_envelope(header_bytes: bytes, image_obj: dict) -> tuple[bytes, str]:
    canonical_json = json.dumps(image_obj, separators=(",", ":"), sort_keys=True).encode("utf-8")
    body = header_bytes + canonical_json
    prefix = b"SHK2CKP1" + bytes([1]) + struct.pack("<Q", len(body))
    digest = hashlib.sha256(
        b"SHK_CK2" + bytes([3]) + struct.pack("<H", 2) + prefix + body
    ).digest()
    return prefix + body + digest, digest.hex()


def unpack_fin_envelope(blob: bytes) -> tuple[bytes, dict, str]:
    assert blob[:8] == b"SHK2CKP1" and blob[8] == 1
    body_len = struct.unpack("<Q", blob[9:17])[0]
    assert len(blob) == 17 + body_len + 32
    body = blob[17 : 17 + body_len]
    capture_len = body[42]
    assert capture_len == 0
    header_bytes = body[:43]
    image_bytes = body[43:]
    image_obj = json.loads(image_bytes.decode("utf-8"))
    reencoded, ck_hex = seal_fin_envelope(header_bytes, image_obj)
    assert reencoded == blob
    return header_bytes, image_obj, ck_hex


def main() -> int:
    binary = (
        Path(sys.argv[1]).resolve()
        if len(sys.argv) > 1
        else (Path.home() / "repos/Moriarty-wt-factory-u/target/debug/mori")
    )
    engine_src = Path.home() / "repos/Moriarty-wt-factory-u/crates/mori-core/src/stage/sherlock2.rs"
    adapter_src = (
        Path.home()
        / "repos/Moriarty-wt-factory-u/crates/mori-core/src/sherlock/compile_artifact/suspension2_fin.rs"
    )
    result = {
        "scope": "Executed read-only host diagnostic on synthetic financial checkpoints; no Rust build, native proof or chain operation.",
        "binary": str(binary),
        "binarySha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "engineSha256": hashlib.sha256(engine_src.read_bytes()).hexdigest(),
        "reportAdapterSha256": hashlib.sha256(adapter_src.read_bytes()).hexdigest(),
        "source": SOURCE,
        "scenario": SCENARIO,
        "cases": [],
    }
    with tempfile.TemporaryDirectory(prefix=".fin-import-probe-", dir=HERE) as directory:
        work = Path(directory)
        config = json.loads((ROOT / "qa/holdout/config/draft-host-config.json").read_text())
        config["profile"] = "sherlock-helper-microstep/2"
        config["fuel"] = {"kind": "Nat", "value": "12"}
        result["config"] = config
        (work / "config.json").write_text(json.dumps(config))
        (work / "source.mori").write_text(SOURCE)
        (work / "scenario.json").write_text(json.dumps(SCENARIO))
        (work / "target.json").write_text("{}")

        def invoke(label: str, arguments: list[str], cfg_path: str = "config.json") -> dict:
            started = time.monotonic()
            command = [
                str(binary),
                "simulate",
                "source.mori",
                "--execution-profile",
                "sherlock-helper-microstep/2",
                "--source-profile",
                "moriarty/2",
                "--execution-config",
                cfg_path,
                "--target",
                "target.json",
                *arguments,
            ]
            process = subprocess.run(
                command, cwd=work, text=True, capture_output=True, timeout=20, check=False
            )
            row = {
                "label": label,
                "command": command,
                "exit": process.returncode,
                "stdout": process.stdout,
                "stderr": process.stderr,
                "wallSeconds": round(time.monotonic() - started, 4),
            }
            result["cases"].append(row)
            return row

        start = invoke(
            "start",
            [
                "--action",
                "pay",
                "--scenario",
                "scenario.json",
                "--phase-policy",
                "retain-guaranteed-fees/1",
                "--checkpoint-out",
                "waiting.bin",
            ],
        )
        assert start["exit"] == 0 and json.loads(start["stdout"])["status"] == "waiting"
        original_blob = (work / "waiting.bin").read_bytes()
        header_bytes, base_image, base_ck = unpack_fin_envelope(original_blob)

        def run_resume_case(
            label: str,
            mutations: dict,
            go_value: bool,
            policy: str = "retain-guaranteed-fees/1",
        ) -> None:
            image = copy.deepcopy(base_image)
            image.update(mutations)
            blob, ck_hex = seal_fin_envelope(header_bytes, image)
            (work / "input.bin").write_bytes(blob)
            observation = {
                "format": "sherlock-observation/2",
                "checkpoint": ck_hex,
                "site": 1,
                "label": "second",
                "values": [{"name": "go", "value": {"type": "Bool", "value": go_value}}],
            }
            (work / "observation.json").write_text(json.dumps(observation))
            output = work / "output.bin"
            output.unlink(missing_ok=True)
            row = invoke(
                label,
                [
                    "--phase-policy",
                    policy,
                    "--resume",
                    "input.bin",
                    "--observation",
                    "observation.json",
                    "--checkpoint-out",
                    "output.bin",
                ],
            )
            row.update(
                {
                    "mutations": mutations,
                    "observationGo": go_value,
                    "inputUnchanged": (work / "input.bin").read_bytes() == blob,
                    "outputExists": output.exists(),
                    "outputEqualsInput": output.exists() and output.read_bytes() == blob,
                }
            )

        run_resume_case("ordinaryAccept", {}, True)
        run_resume_case(
            "malformedCountersAccepted",
            {"initial_fuel": "0", "remaining_work": "5", "consumed_work": "0"},
            True,
        )
        run_resume_case(
            "maximumConsumedFalseGuard",
            {"initial_fuel": U128_MAX, "remaining_work": "1", "consumed_work": U128_MAX},
            False,
        )
        run_resume_case(
            "maximumConsumedTrueGuard",
            {"initial_fuel": U128_MAX, "remaining_work": "1", "consumed_work": U128_MAX},
            True,
        )
        run_resume_case("outOfBoundsSegmentFalseGuard", {"segment": 2}, False)
        run_resume_case("outOfBoundsSegmentTrueGuard", {"segment": 2}, True)
        run_resume_case("staleSegmentZeroRetainFeesTrueGuard", {"segment": 0}, True)
        run_resume_case(
            "malformedCapturesAccepted",
            {
                "captures": [
                    {"name": "injected", "value": {"type": "UInt", "value": "99"}}
                ]
            },
            True,
        )
        run_resume_case(
            "nonEmptyDiagnosticsLeakedToHalted",
            {
                "discarded_effects": [base_image["committed_effects"][0]],
                "retained_by_policy": [base_image["committed_effects"][0]],
            },
            True,
        )

        # Await-entry fuel=4 check (SI-29 repaired: publishes Exhausted(Fuel) instead of Waiting,
        # while rolling back tentative / retaining fee_stage per E2x finding).
        config_fuel4 = copy.deepcopy(config)
        config_fuel4["fuel"] = {"kind": "Nat", "value": "4"}
        (work / "config-fuel4.json").write_text(json.dumps(config_fuel4))
        start4 = invoke(
            "zeroFuelAwaitEntryExhausted",
            [
                "--action",
                "pay",
                "--scenario",
                "scenario.json",
                "--phase-policy",
                "retain-guaranteed-fees/1",
                "--checkpoint-out",
                "waiting-fuel4.bin",
            ],
            cfg_path="config-fuel4.json",
        )
        assert start4["exit"] == 1 and json.loads(start4["stdout"])["status"] == "exhausted"
        _, image4, _ = unpack_fin_envelope((work / "waiting-fuel4.bin").read_bytes())
        start4["checkpointRemainingWork"] = image4["remaining_work"]
        start4["checkpointConsumedWork"] = image4["consumed_work"]

        # Resealed remaining_work=0 with valid observation -> refused at R0 (SI-29).
        run_resume_case(
            "zeroRemainingRefusedAtR0",
            {"initial_fuel": "4", "remaining_work": "0", "consumed_work": "4"},
            True,
        )

        # Resealed remaining_work=0 with malformed observation -> reports R3 before R0 (r0LateAfterR3Observed).
        image_r0_r3 = copy.deepcopy(base_image)
        image_r0_r3.update({"initial_fuel": "4", "remaining_work": "0", "consumed_work": "4"})
        blob_r0_r3, ck_r0_r3 = seal_fin_envelope(header_bytes, image_r0_r3)
        (work / "input-r0-r3.bin").write_bytes(blob_r0_r3)
        bad_obs = {
            "format": "wrong-format",
            "checkpoint": ck_r0_r3,
            "site": 1,
            "label": "second",
            "values": [{"name": "go", "value": {"type": "Bool", "value": True}}],
        }
        (work / "bad-obs.json").write_text(json.dumps(bad_obs))
        invoke(
            "zeroRemainingBadObsReportsR3BeforeR0",
            [
                "--phase-policy",
                "retain-guaranteed-fees/1",
                "--resume",
                "input-r0-r3.bin",
                "--observation",
                "bad-obs.json",
            ],
        )

        # Resealed header with resume_epoch = u64::MAX -> refused at R0 (epochMaxRefused).
        max_epoch_header = struct.pack("<Q", 2**64 - 1) + header_bytes[8:]
        blob_epoch, ck_epoch = seal_fin_envelope(max_epoch_header, copy.deepcopy(base_image))
        (work / "input-epoch-max.bin").write_bytes(blob_epoch)
        obs_epoch = {
            "format": "sherlock-observation/2",
            "checkpoint": ck_epoch,
            "site": 1,
            "label": "second",
            "values": [{"name": "go", "value": {"type": "Bool", "value": True}}],
        }
        (work / "obs-epoch.json").write_text(json.dumps(obs_epoch))
        invoke(
            "epochMaxRefusedAtR0",
            [
                "--phase-policy",
                "retain-guaranteed-fees/1",
                "--resume",
                "input-epoch-max.bin",
                "--observation",
                "obs-epoch.json",
            ],
        )

        # Resealed committed_effects overflowing u128::MAX -> refused at accounting (U017 checked_add).
        overflow_effect = copy.deepcopy(base_image["committed_effects"][1])
        overflow_effect["amount"] = U128_MAX
        run_resume_case(
            "checkedAccountingOverflowRefused",
            {"committed_effects": [overflow_effect, copy.deepcopy(overflow_effect)]},
            True,
        )

    (HERE / "financial-import-rust-probe.json").write_text(json.dumps(result, indent=2) + "\n")
    for row in result["cases"]:
        print(f"{row['label']}: exit={row['exit']} wall={row['wallSeconds']}s")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
