# Recorded wallet registration event timeline (selected non-secret fields)
> Source: file:///home/charl/code/stagenet-wallet/register-dust.log
> Collected: 2026-10-26
> Published: 2026-10-26

Existing log, not a new measurement. Transaction ID, account quantities and configuration omitted. The log also records seconds=435. No per-proof timer or proof-server request count is present in these events.

```json
[
  {
    "time": "2026-10-26T07:32:53.634Z",
    "msg": "wallet[stagenet-main]: syncing (timeout 900000ms)"
  },
  {
    "time": "2026-10-26T07:39:08.544Z",
    "msg": "wallet[stagenet-main]: synced (17576 state emissions)"
  },
  {
    "time": "2026-10-26T07:39:09.402Z",
    "msg": "[stagenet-main] building + submitting registration tx"
  },
  {
    "time": "2026-10-26T07:39:25.473Z",
    "msg": "registration submitted [transaction ID omitted]"
  },
  {
    "time": "2026-10-26T07:40:08.714Z",
    "msg": "registeredUtxos=1 dustCoins=1 [balance omitted]"
  },
  {
    "time": "2026-10-26T07:40:08.715Z",
    "msg": "DUST ready [balance omitted]"
  }
]
```
