# Install the toolchain

> For the complete documentation index, see [llms.txt](/llms.txt)

This guide covers the installation of core dependencies for developing decentralized applications (DApps) on Midnight.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

Before you begin developing DApps on Midnight, ensure you have:

* Google Chrome browser
* Visual Studio Code
* Docker Desktop: <https://www.docker.com/products/docker-desktop/>

Development is supported on Linux and Mac. Windows is not supported natively at this time, if you are using Windows, development through WSL is recommended.

## Install Compact[​](#install-compact "Direct link to Install Compact")

[Compact](https://github.com/midnightntwrk/compact) is Midnight's dedicated smart contract language for creating DApps that allows developers to control the level of data protection in their applications.

Use the following command to install the pre-built binaries:

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/midnightntwrk/compact/releases/latest/download/compact-installer.sh | sh
```

### Update your shell PATH[​](#update-your-shell-path "Direct link to Update your shell PATH")

The installer script automatically adds Compact to your shell's `PATH`. However, you need to restart your terminal or reload your shell configuration for the changes to take effect.

To reload your shell configuration without restarting, run the following command:

```
source ~/.zshrc # for zsh

# or

source ~/.bashrc # for bash
```

**Manual PATH configuration** (only if needed): If Compact is not found after restarting, manually add it to your PATH:

```
export PATH="$HOME/.compact/bin:$PATH"
```

Then reload your shell configuration as shown above.

### Update the compiler version[​](#update-the-compiler-version "Direct link to Update the compiler version")

Update to the latest compiler version:

```
compact update 0.31.1
```

This command downloads the latest version of the Compact compiler and sets it as the default version.

### Verify the Compact installation[​](#verify-the-compact-installation "Direct link to Verify the Compact installation")

Run these commands to verify your installation:

```
compact --version # print the version of Compact

compact compile --version # print the version of the compiler

which compact     # print the installation path
```

**Verification**: These commands return the Compact version number, the compiler version number, and the installation path.

## Set up the proof server[​](#set-up-the-proof-server "Direct link to Set up the proof server")

The proof server is required to generate zero-knowledge proofs for transactions locally. This section walks you through the process of running the proof server.

### Docker Desktop[​](#docker-desktop "Direct link to Docker Desktop")

The proof server runs as a background service using Docker. Ensure the Docker engine is running for the following step.

### Run the proof server[​](#run-the-proof-server "Direct link to Run the proof server")

Use the following command to start the proof server in your terminal:

```
docker run -p 6300:6300 midnightntwrk/proof-server:8.1.0 midnight-proof-server -v
```

This command occupies the terminal window while running.

**Verification**: The terminal displays logs indicating the server is running and listening at <http://localhost:6300>.

note

To use a local proof-server with the Lace Midnight wallet, go to **Settings » Midnight** and select `Local (http://localhost:6300)`. The local proof server is currently the only option supported within Lace.

## Install the Compact VS Code extension[​](#install-the-compact-vs-code-extension "Direct link to Install the Compact VS Code extension")

The Compact VS Code extension provides helpful syntax highlighting and code snippet completion. To install it, follow these steps:

1

Download the [VSIX package](https://raw.githubusercontent.com/midnight-ntwrk/releases/gh-pages/artifacts/vscode-extension/compact-0.2.13/compact-0.2.13.vsix).

2

In VS Code, go to **Extensions**, then **Install from VSIX** and select the extension file.

**Verification**: You now see the Compact Language Support extension in your installed VS Code extensions.

## Install Complete[​](#install-complete "Direct link to Install Complete")

Your development environment is now configured. You are ready to start building and interacting with privacy-preserving applications on Midnight.

In order to deploy and interact with Midnight DApps, both a Midnight Node and Indexer are required. These are provided as RPC endpoints and can be found in the [Latest Stable Release](/relnotes/overview.md#environments). Subsequent Tutorials will demonstrate their use, alternatively, the [Examples](/category/examples) section provides complete DApp demonstrations.

## Troubleshoot[​](#troubleshoot "Direct link to Troubleshoot")

This section covers common issues that you might encounter during installation and their solutions.

### Compact binary not found[​](#compact-binary-not-found "Direct link to Compact binary not found")

If you see this error:

```
compact: command not found
```

This means the Compact binary is not in your `PATH`. Follow the instructions in [Update your shell PATH](#update-your-shell-path) to add it to your `PATH`.

### Docker Desktop not running[​](#docker-desktop-not-running "Direct link to Docker Desktop not running")

If you encounter connection errors when starting the proof server, ensure Docker Desktop is running:

1. Open Docker Desktop application.
2. Wait for the Docker engine to start (the Docker icon in your system tray should be steady, not animated).
3. Try running the proof server command again.

### Port already in use[​](#port-already-in-use "Direct link to Port already in use")

If you see an error that port 6300 is already in use:

```
Error: bind: address already in use
```

Either stop the process using port 6300, or run the proof server on a different port:

```
docker run -p 6301:6300 midnightntwrk/proof-server:8.1.0 midnight-proof-server -v
```

Remember to update your application configuration to use the new port.

## Next steps[​](#next-steps "Direct link to Next steps")

Now that your development environment is set up, you can:

* [Create your first Midnight contract](/getting-started/hello-world.md)
* [Explore the Compact language](/compact.md)
