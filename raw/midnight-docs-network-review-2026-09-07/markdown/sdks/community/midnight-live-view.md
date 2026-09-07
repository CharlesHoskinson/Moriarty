> For the complete documentation index, see [llms.txt](/llms.txt)

# Midnight Live View

[Midnight Live View](https://github.com/Midnight-Scripts/Midnight-Live-View) is a terminal-based monitoring dashboard for Midnight validator nodes. It displays node health, block production, peer connections, and system resources in real time, refreshing every second.

The project consists of two shell scripts:

* **`LiveView.sh`**: a real-time dashboard inspired by [CNTool's gLiveView](https://cardano-community.github.io/guild-operators/Scripts/gliveview/) that shows node version, uptime, block data, peer count, and system metrics.
* **`simple_block_monitor.sh`**: a background process that tracks blocks persistently by tailing container logs and writing to a local JSON file. Block data survives Docker restarts.

Community-maintained, no license

[Midnight-Scripts](https://github.com/Midnight-Scripts) maintains this project, not the Midnight Foundation. The repository does not currently specify a license. The tool has not been audited. Evaluate independently before relying on it in any environment.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

Before using the tool, ensure that you have:

* A running Midnight validator node (Docker container named `midnight` by default).
* `jq` and `curl` installed (`sudo apt install jq curl` on Ubuntu/Debian, `brew install jq curl` on macOS).
* Docker installed and running.
* The `partner-chains-public-keys.json` file in the same directory as `LiveView.sh`.

## Compatibility[​](#compatibility "Direct link to Compatibility")

| Component           | Version      | Notes                                                          |
| ------------------- | ------------ | -------------------------------------------------------------- |
| Tool version        | `0.2.1`      | No formal releases; code accessed from `main` branch.          |
| Tested node version | `0.12.0`     | Tested against builds `cab67f3b` and `29935d2f`.               |
| Network             | Testnet      | No Mainnet support documented.                                 |
| Platforms           | Linux, macOS | Install commands reference Ubuntu/Debian and macOS (Homebrew). |

## Node configuration[​](#node-configuration "Direct link to Node configuration")

The dashboard reads data from your node's RPC endpoint. You need to modify the `APPEND_ARGS` variable in your `.envrc` file to expose the RPC interface.

Security: unsafe RPC methods

The configuration below enables `--unsafe-rpc-external` and `--rpc-methods=Unsafe`, which expose RPC methods that are not safe for public access. On a single-server setup, this is acceptable for local monitoring. On a multi-server setup, restrict RPC access to trusted IPs only.

For a single-server setup, add the following to your `APPEND_ARGS`:

```
--unsafe-rpc-external --rpc-methods=Unsafe --rpc-cors all
```

For two-server setups, see the [project README](https://github.com/Midnight-Scripts/Midnight-Live-View#readme) for the full configuration.

## Installation[​](#installation "Direct link to Installation")

Download both scripts and make them executable:

```
wget -O ./LiveView.sh https://raw.githubusercontent.com/Midnight-Scripts/Midnight-Live-View/refs/heads/main/LiveView.sh

wget -O ./simple_block_monitor.sh https://raw.githubusercontent.com/Midnight-Scripts/Midnight-Live-View/refs/heads/main/simple_block_monitor.sh

chmod +x LiveView.sh simple_block_monitor.sh
```

## Run the dashboard[​](#run-the-dashboard "Direct link to Run the dashboard")

For basic dashboard-only monitoring:

```
./LiveView.sh
```

For persistent block tracking (recommended), start the block monitor first:

```
./simple_block_monitor.sh start

./LiveView.sh
```

The block monitor runs in the background and writes to `all_blocks.json`. Manage it with `start`, `stop`, `status`, or `run` (interactive mode).

## Dashboard features[​](#dashboard-features "Direct link to Dashboard features")

The dashboard displays the following information, refreshed every second:

* **Node info**: version, uptime, and container start time.
* **Security**: node key (masked), port, and key status.
* **Registration**: current registration status.
* **Block data**: historic blocks (total since monitoring began), blocks produced (since Docker restart), latest and finalized block numbers, and sync status.
* **Network**: peer count with interactive peer details (press `p`).
* **System**: CPU, memory, and disk usage.

Press `q` to quit, `p` to toggle peer details.

## Configuration[​](#configuration "Direct link to Configuration")

You can override defaults with environment variables:

| Variable         | Default    | Purpose                                       |
| ---------------- | ---------- | --------------------------------------------- |
| `CONTAINER_NAME` | `midnight` | Docker container name for the validator node. |
| `PORT`           | `9944`     | RPC port the dashboard connects to.           |
| `USE_DOCKER`     | `true`     | Set to `false` for non-Docker node setups.    |

## Files created at runtime[​](#files-created-at-runtime "Direct link to Files created at runtime")

The block monitor creates three files in its working directory:

| File                 | Purpose                                              |
| -------------------- | ---------------------------------------------------- |
| `all_blocks.json`    | Persistent block database with duplicate prevention. |
| `block_monitor.log`  | Monitor activity log.                                |
| `.block_monitor.pid` | Process tracking file.                               |

## Additional resources[​](#additional-resources "Direct link to Additional resources")

* [Project repository](https://github.com/Midnight-Scripts/Midnight-Live-View): source, README, and configuration guide.
* [CNTool gLiveView](https://cardano-community.github.io/guild-operators/Scripts/gliveview/): the Cardano monitoring tool that inspired this dashboard.

## Report issues[​](#report-issues "Direct link to Report issues")

For issues with the tool, file on [Midnight-Scripts' tracker](https://github.com/Midnight-Scripts/Midnight-Live-View/issues).

For issues with this documentation page, file on the [Midnight docs repository](https://github.com/midnightntwrk/midnight-docs/issues).
