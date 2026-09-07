> For the complete documentation index, see [llms.txt](/llms.txt)

# Set up Bun for Midnight development

Bun is a modern JavaScript runtime and toolkit that can offer significantly faster performance compared to Node.js and npm. This guide walks you through using Bun with Midnight Network development, covering installation, configuration, compatibility considerations, best practices, and much more.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

Before starting this guide, ensure you have:

* Basic knowledge of JavaScript/TypeScript
* Familiarity with command-line interfaces
* Understanding of package managers (npm, yarn)

note

Docker is not required for this tutorial. It's only needed when running actual Midnight proof servers.

Here's what your complete project will look like by the end of this guide:

![Image](https://github.com/user-attachments/assets/3841162f-4485-47cc-98eb-4f9c1c786629)

## Install Bun[​](#install-bun "Direct link to Install Bun")

In this section, you'll install Bun on your development machine.

### Install Bun on Mac[​](#install-bun-on-mac "Direct link to Install Bun on Mac")

1. Run the command below:

```
curl -fsSL https://bun.sh/install | bash
```

This script:

* Downloads the appropriate Bun binary for your system.
* Installs it to `~/.bun/bin`.

2. Verify the installation:

Close and reopen your terminal, then run:

```
bun --version
```

Example of expected output:

![Image](https://github.com/user-attachments/assets/2d817253-4b67-4c07-8562-c02a3db0e643)

If bun --version returns "command not found." manually add Bun to your PATH.

```
export PATH="$HOME/.bun/bin:$PATH”
```

### Install Bun on linux or WSL[​](#install-bun-on-linux-or-wsl "Direct link to Install Bun on linux or WSL")

Follow the steps below to install Bun on Linux or Windows Subsystem for Linux (WSL).

1. Install required dependencies

Bun’s installer requires unzip to extract the binary.

```
sudo apt update

sudo apt install unzip -y
```

2. Install Bun

Run the official Bun installation script:

```
curl -fsSL https://bun.sh/install | bash
```

3. Reload your shell configuration

Update your environment so the bun command is available:

```
source ~/.bashrc
```

4. Verify the installation

```
bun --version
```

You should see the installed Bun version printed to the terminal.

5. Fix “command not found” (if needed)

If bun is not found, add Bun’s install directory to your `PATH`:

```
export PATH="$HOME/.local/bin:$PATH"
```

## Install using the installer script[​](#install-using-the-installer-script "Direct link to Install using the installer script")

Here, you will install the Compact compiler. This is a standalone tool (separate from Bun) used to turn Compact smart contracts (written in the Compact language) into code that can run on the Midnight blockchain.

1. Install using the Compact installer:

```
curl --proto '=https' --tlsv1.2 -LsSf \

  https://github.com/midnightntwrk/compact/releases/latest/download/compact-installer.sh | sh
```

By default, the installer places the compact binary in `$HOME/.local/bin`

2. Update the compiler

After installation, update the Compact compiler to the version used in this tutorial:

```
compact update
```

3. Verify the installation

Confirm that Compact is installed and correctly configured:

```
compact check
```

If the installation is successful, you should see output similar to the following:

```
compact: aarch64-darwin -- Up to date -- 0.31.0
```

4. Fix “command not found” (PATH issue)

If you see an error such as:

```
compact: command not found
```

it means the directory where Compact was installed `($HOME/.local/bin)` is not included in your PATH.

Add it to your current terminal session:

```
export PATH="$HOME/.local/bin:$PATH"
```

## Install Midnight packages with Bun[​](#install-midnight-packages-with-bun "Direct link to Install Midnight packages with Bun")

With both Bun and the Compact compiler installed, you can now create a Midnight application project.

1. Initialize a new project:

```
mkdir my-midnight-app

cd my-midnight-app

bun init -y
```

This creates a basic `package.json`.

2. Create the required directories:

The project requires separate folders for smart contracts and application source code.

```
mkdir src contracts
```

3. Install the Midnight runtime package

Install a specific version to avoid compatibility issues:

```
bun add @midnight-ntwrk/compact-runtime@0.16.0
```

This gives you everything you need to:

* Run your smart contracts
* Manage your app's data
* Work with zero-knowledge proofs
* Get type definitions for TypeScript

info

Always refer to the [compatibility matrix](/relnotes/support-matrix.md) for the correct version of the runtime package to install.

4. Update the `tsconfig.json` file:

tsconfig.json

```
{

  "compilerOptions": {

    "target": "ES2022",

    "module": "ESNext",

    "moduleResolution": "node",

    "outDir": "./dist",

    "rootDir": "./src",

    "strict": true,

    "esModuleInterop": true,

    "skipLibCheck": true,

    "resolveJsonModule": true

  },

  "include": ["src/**/*"]

}
```

This configuration ensures:

* Bun can resolve imports correctly.
* TypeScript compiles cleanly.
* Midnight’s runtime packages work without extra setup.

## Configure the project[​](#configure-the-project "Direct link to Configure the project")

Update your `package.json` with the Bun-specific scripts below:

package.json

```
  "scripts": {

    "dev": "bun run --hot src/index.ts",

    "start": "bun run dist/index.js",

    "test": "bun test",

    "install:midnight": "bun install",

    "compile:contract": "compact compile contracts/message.compact contracts/managed",

    "build:contract": "bun run compile:contract && bun build src/index.ts --outdir dist"

  },
```

Script explanations:

* dev: Runs the app in development mode with hot reload on src/index.ts.

* start: Runs the compiled production app from dist/index.js.

* test: Runs all tests using Bun’s test runner.

* install:midnight: Installs project dependencies with Bun.

* compile:contract: Compiles the Compact smart contract `message.compact` into the contracts/managed output directory.

* build:contract: First compiles the smart contract, then bundles the app entry file into the dist folder for production.

## Create smart contract with Compact[​](#create-smart-contract-with-compact "Direct link to Create smart contract with Compact")

In this section, you will create a simple Compact smart contract that allows you to store and read a message on the Midnight blockchain. Then, you will compile it so it can be used in your TypeScript/Bun application.

1. Create the smart contract file.

Create a file named `message.compact` in the contracts directory:

```
touch contracts/message.compact
```

2. Add the code below to the file:

message.compact

```
pragma language_version 0.23;



import CompactStandardLibrary;



// Public ledger state - visible on blockchain

export ledger message: Opaque<"string">;



// Circuit to store a message on the blockchain

// The message will be publicly visible

export circuit storeMessage(customMessage: Opaque<"string">): [] {

    message = disclose(customMessage);

}
```

Explanation:

* `pragma language_version 0.23;` Specifies the exact Compact compiler version required for this smart contract. Using version 0.23 ensures compatibility with the runtime version 0.16.0 you installed earlier.
* `import CompactStandardLibrary;` Loads standard functions and types provided by Midnight for contracts.
* `export ledger message: Opaque<"string">;` Declares a public state variable called `message` on the blockchain. Its value will be visible to everyone.
* `export circuit storeMessage(customMessage: Opaque<"string">): [] { ... }` Defines a function (circuit) to store a message on the blockchain. `disclose()` makes the message publicly readable.

3. Compile with compact

```
compact compile contracts/message.compact contracts/managed
```

![Image](https://github.com/user-attachments/assets/b53eb5ae-3f37-489d-8b54-968f84748e7a)

What this does:

* Converts the human-readable `.compact` contract into a compiled module that your application can interact with.
* Saves the compiled contract in `contracts/managed`, making it ready for integration with your Bun/TypeScript app.

## Integrate compiled smart contracts with Bun[​](#integrate-compiled-smart-contracts-with-bun "Direct link to Integrate compiled smart contracts with Bun")

Now that your Compact contracts are compiled, the next step is to interact with them from a Bun-powered TypeScript application.

Here you’ll create a client that simulates storing and reading a message from your Midnight contract.

1. Create `src/message-client.ts`

```
touch src/message-client.ts
```

Add the following code:

message-client.ts

```
import { Contract, ledger } from "../contracts/managed/contract/index.js";



export class MessageClient {

  private contract: Contract<any>;



  constructor() {

    this.contract = new Contract({});

  }



  async storeMessage(customMessage: string) {

    console.log(`📦 (Simulated) Storing message: "${customMessage}"`);

    // Real implementation needs proper Midnight context

    return { success: true, message: customMessage };

  }



  async getMessage() {

    console.log("📥 (Simulated) Fetching message from ledger...");

    return "Hello Midnight!";

  }

}
```

What this does:

* Creates a `MessageClient` class.

* Loads your compiled Compact contract.

* Provides two methods:

  <!-- -->

  * `storeMessage()`: simulates writing data.
  * `getMessage()`: simulates reading data.

* Keeps things simple while preserving the project structure required by real Midnight apps.

2. Create `src/index.ts`

This file will serve as your application entry point.

```
touch src/index.ts
```

Add the following code:

index.ts

```
import { MessageClient } from "./message-client";



async function main() {

  console.log("🚀 Starting Midnight Message App...");



  const client = new MessageClient();



  console.log("📝 Storing message...");

  await client.storeMessage("Hello Midnight!");



  console.log("📖 Reading message...");

  const message = await client.getMessage();

  console.log("✅ Message:", message);

}



main().catch(console.error);
```

### What this file does:[​](#what-this-file-does "Direct link to What this file does:")

* Boot your Midnight app
* Creates an instance of `MessageClient`
* Calls the store and retrieves functions
* Prints the results nicely in your terminal

3. Run the app with Bun:

```
bun run --hot src/index.ts
```

![Image](https://github.com/user-attachments/assets/845d6cbf-fcf8-42e9-bfe7-a63a6859175a)

## Known limitations and workarounds[​](#known-limitations-and-workarounds "Direct link to Known limitations and workarounds")

While Bun offers significant performance improvements, there are some limitations you should be aware of when building Midnight applications. Here are the common issues and how to work around them.

### Native module compatibility[​](#native-module-compatibility "Direct link to Native module compatibility")

Issue: some npm packages with native Node.js addons may not work correctly with Bun.

Workaround:

```
# If a package fails, try running it with Node.js compatibility mode

bun --bun run your-script.ts



# Or fall back to Node.js for specific scripts

node your-script.js
```

### Package manager lock files[​](#package-manager-lock-files "Direct link to Package manager lock files")

Issue: mixing Bun and npm in the same project can cause conflicts with lock files.

Workaround:

```
# Pick one and stick with it. If using Bun, remove npm files

rm package-lock.json



# If using npm, remove Bun files

rm bun.lock
```

tip

Choose one package manager for your project and have your team use only that one.

### Environment variables[​](#environment-variables "Direct link to Environment variables")

Issue: bun automatically handles `.env` files, which might cause unexpected behavior if you're using other environment variable tools.

Workaround:

```
# Bun loads .env automatically, no library needed# If you need to disable this:

bun --env-file= run your-script.ts



# Or specify a different env file:

bun --env-file=.env.production run your-script.ts
```

## Migration guide (npm → Bun)[​](#migration-guide-npm--bun "Direct link to Migration guide (npm → Bun)")

If your Midnight project was originally set up using Node.js + npm, you can migrate to Bun with just a few steps. This guide walks you through the process safely and cleanly.

1. Install Bun

Make sure Bun is installed on your system.

To check

```
 bun --version
```

If not found, go to this step and [install Bun](#install-bun).

2. Remove npm Artifacts:

```
cd your-midnight-project

rm -rf node_modules package-lock.json
```

warning

Back up your `package-lock.json` first if you need to revert.

3. Install dependencies with Bun:

```
bun install
```

4. Verify the Migration

Test that your application works correctly with Bun:

```
bun run dev
```

## Troubleshoot common issues[​](#troubleshoot-common-issues "Direct link to Troubleshoot common issues")

1. Module Not Found Errors

Problem:

```
bun run --hot src/index.ts
```

Cannot find module `'@midnight-ntwrk/compact-runtime’`

Solution:

```
bun add @midnight-ntwrk/compact-runtime@0.16.0
```

2. Version mismatch errors

Problem:

```
CompactError: Version mismatch: compiled code expects 0.16.0, runtime is 0.9.0
```

note

The Compact compiler and runtime versions must match exactly. Always check [version compatibility](/relnotes/support-matrix.md) before compiling.

Solution:

Install the matching runtime version:

```
bun add @midnight-ntwrk/compact-runtime@0.16.0
```

Then recompile your smart contracts:

```
compact compile contracts/message.compact contracts/managed
```

3. Compact compiler language version errors

Problem:

```
Exception: message.compact line 1 char 1:

  language version 0.22.0 mismatch
```

Solution:

Update your smart contract to use the exact language version:

```
pragma language_version 0.23;
```

note

Using an exact version number (such as 0.22) instead of a range prevents compatibility issues with different compiler versions.

4. Compact compiler installation issues

Problem:

The Compact compiler fails to install or doesn't work after installation.

Solutions:

* Verify the installer script ran successfully:

```
compact --version
```

* If the command isn't found, manually add it to your PATH:

```
export PATH="$HOME/.compact/bin:$PATH"
```

* Try reinstalling:

```
curl --proto '=https' --tlsv1.2 -LsSf \

  https://github.com/midnightntwrk/compact/releases/latest/download/compact-installer.sh | sh
```

## Next steps[​](#next-steps "Direct link to Next steps")

Now that you have Bun installed and configured, you can start building DApps on Midnight. See the following guides for more information:

* [DApp connector](/api-reference/dapp-connector.md)
* [Deploy a contract](/guides/deploy-and-operate.md)
* [Interact with a contract](/guides/interact-with-mn-app)
