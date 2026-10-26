# Recorded A3 and A4 runs; not new measurements
> Source: file:///home/charl/code/Moriarty-perf/qa/sherlock/runs.jsonl
> Collected: 2026-10-26
> Published: Unknown

```json
[
  {
    "run": "20261025T000955Z-A3",
    "id": "A3",
    "cmd": "scripts/qa/check-a3.sh",
    "hypothesis": "The ported parent and native test compile on stock0.8.3; explicit BLS12 SRS typing fixes the build error, and preflight is now clear.",
    "exit": 0,
    "wallSeconds": 315.88,
    "maxRssBytes": 15760023552,
    "trace": "qa/sherlock/traces/20261025T000955Z-A3.log",
    "preflight": {
      "ok": true,
      "skipped": false,
      "diskFreeGiB": 1119.15,
      "minFreeGiB": 100.0,
      "load1": 10.83,
      "maxLoad": 20.0,
      "cpus": 20,
      "lockHeld": false,
      "lock": "/home/charles/.cache/doctorwatson/heavy.lock",
      "reasons": []
    },
    "head": "865f064023351c5b4da448cc953747edc1cba908",
    "at": "2026-10-25T00:15:11Z"
  },
  {
    "run": "20261025T003236Z-A4",
    "id": "A4",
    "cmd": "scripts/qa/check-a4.sh",
    "hypothesis": "The required reference flat arithmetic module is included; now test the bound-communication terminal relation against the exact live installed key before proving.",
    "exit": 101,
    "wallSeconds": 211.989,
    "maxRssBytes": 4677103616,
    "trace": "qa/sherlock/traces/20261025T003236Z-A4.log",
    "preflight": {
      "ok": true,
      "skipped": false,
      "diskFreeGiB": 1116.99,
      "minFreeGiB": 100.0,
      "load1": 0.85,
      "maxLoad": 20.0,
      "cpus": 20,
      "lockHeld": false,
      "lock": "/home/charles/.cache/doctorwatson/heavy.lock",
      "reasons": []
    },
    "head": "9d485819b8657f586e56c0c27ae3fd4d095e4e66",
    "at": "2026-10-25T00:36:08Z"
  },
  {
    "run": "20261025T012355Z-A4",
    "id": "A4",
    "cmd": "scripts/qa/check-a4.sh",
    "hypothesis": "The archived installed parent is afc4ec3a while current A3 is801467c8; reject that exact mismatch before keygen and preserve the installed verifier requirement",
    "exit": 101,
    "wallSeconds": 15.338,
    "maxRssBytes": 784191488,
    "trace": "qa/sherlock/traces/20261025T012355Z-A4.log",
    "preflight": {
      "ok": true,
      "skipped": false,
      "diskFreeGiB": 1101.49,
      "minFreeGiB": 150.0,
      "load1": 5.91,
      "maxLoad": 20.0,
      "cpus": 20,
      "lockHeld": false,
      "lock": "/home/charles/.cache/doctorwatson/heavy.lock",
      "reasons": []
    },
    "head": "f19f77e83de5b6f7f3517543f93ee1ce28bf3121",
    "at": "2026-10-25T01:24:10Z"
  },
  {
    "run": "20261025T024457Z-A4",
    "id": "A4",
    "cmd": "scripts/qa/check-a4.sh",
    "hypothesis": "The owner-authorized replacement verifier now installs the current parent/template decider; independent indexer state and Rust raw-key readback match the new pins.",
    "exit": 0,
    "wallSeconds": 324.739,
    "maxRssBytes": 16487976960,
    "trace": "qa/sherlock/traces/20261025T024457Z-A4.log",
    "preflight": {
      "ok": true,
      "skipped": false,
      "diskFreeGiB": 1051.77,
      "minFreeGiB": 150.0,
      "load1": 19.56,
      "maxLoad": 20.0,
      "cpus": 20,
      "lockHeld": false,
      "lock": "/home/charles/.cache/doctorwatson/heavy.lock",
      "reasons": []
    },
    "head": "6d752e7fbd30da16e5db7e9b78520121b623b107",
    "at": "2026-10-25T02:50:22Z"
  },
  {
    "run": "20261026T044131Z-A3",
    "id": "A3",
    "cmd": "scripts/qa/check-a3.sh",
    "hypothesis": null,
    "exit": 0,
    "wallSeconds": 382.278,
    "maxRssBytes": 15723253760,
    "trace": "qa/sherlock/traces/20261026T044131Z-A3.log",
    "preflight": {
      "ok": true,
      "skipped": false,
      "diskFreeGiB": 621.72,
      "minFreeGiB": 150.0,
      "load1": 12.57,
      "maxLoad": 20.0,
      "cpus": 20,
      "lockHeld": false,
      "lock": "/tmp/heavy-proxy-s1.lock",
      "reasons": []
    },
    "head": "c7d3c48a3d998a16112cab5f4a6aa621a47fa79e",
    "at": "2026-10-26T04:47:53Z"
  },
  {
    "run": "20261026T044937Z-A4",
    "id": "A4",
    "cmd": "scripts/qa/check-a4.sh",
    "hypothesis": null,
    "exit": 101,
    "wallSeconds": 2.201,
    "maxRssBytes": 88272896,
    "trace": "qa/sherlock/traces/20261026T044937Z-A4.log",
    "preflight": {
      "ok": true,
      "skipped": false,
      "diskFreeGiB": 618.94,
      "minFreeGiB": 150.0,
      "load1": 12.25,
      "maxLoad": 20.0,
      "cpus": 20,
      "lockHeld": false,
      "lock": "/tmp/heavy-proxy-s1.lock",
      "reasons": []
    },
    "head": "dbffef95ac7995ad9471e1daaa892a28e4183f1f",
    "at": "2026-10-26T04:49:39Z"
  }
]
```
