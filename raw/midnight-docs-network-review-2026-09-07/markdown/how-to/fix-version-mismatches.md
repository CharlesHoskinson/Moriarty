> For the complete documentation index, see [llms.txt](/llms.txt)

# Fix version mismatch errors

Learn how to resolve version compatibility issues between Midnight components that cause build failures and runtime errors.

## What causes version mismatches?[​](#what-causes-version-mismatches "Direct link to What causes version mismatches?")

Midnight consists of multiple components that must work together in compatible versions:

* **Compact toolchain**: Binaries, compiler, and formatter
* **Runtime libraries**: `@midnight-ntwrk/compact-runtime`, `@midnight-ntwrk/ledger-v8`, `@midnight-ntwrk/onchain-runtime-v3`, and related packages
* **JavaScript libraries**: `@midnight-ntwrk/midnight-js` (barrel package), `@midnight-ntwrk/dapp-connector-api`, `@midnight-ntwrk/wallet-sdk-facade`, and related packages
* **Proof server**: Zero-knowledge proof generation service
* **Indexer**: GraphQL API for querying blockchain data (optional)

When these components are out of sync, you may encounter build errors, deployment failures, or runtime issues.

Check the compatibility matrix

Always refer to the official [release compatibility matrix](/relnotes/support-matrix.md) to verify which versions work together.

## Check your current versions[​](#check-your-current-versions "Direct link to Check your current versions")

Before fixing version mismatches, identify which components are currently installed and compare them with the compatibility matrix.

1

### Check the compiler version[​](#check-the-compiler-version "Direct link to Check the compiler version")

```
compact --version
```

Compare the output with the [compatibility matrix](/relnotes/support-matrix.md).

2

### Check runtime package versions[​](#check-runtime-package-versions "Direct link to Check runtime package versions")

Run these commands to check the installed versions of runtime packages:

```
npm list @midnight-ntwrk/compact-runtime

npm list @midnight-ntwrk/ledger-v8

npm list @midnight-ntwrk/onchain-runtime-v3
```

Verify that all runtime packages use compatible version numbers as specified in the compatibility matrix.

3

### Check proof server version[​](#check-proof-server-version "Direct link to Check proof server version")

If running the proof server using Docker:

```
docker ps | grep proof-server

docker logs [proof-server-container-id] | head -20
```

Look for version information in the startup logs.

4

### Compare versions with the compatibility matrix[​](#compare-versions-with-the-compatibility-matrix "Direct link to Compare versions with the compatibility matrix")

Review the [release compatibility matrix](/relnotes/support-matrix.md) and verify your versions are compatible. Note any components that do not match the recommended versions.

## Align component versions[​](#align-component-versions "Direct link to Align component versions")

After identifying version mismatches, update all components to compatible versions. Always update related components together to maintain compatibility across your development environment.

1

### Consult the compatibility matrix[​](#consult-the-compatibility-matrix "Direct link to Consult the compatibility matrix")

Review the [release compatibility matrix](/relnotes/support-matrix.md) to identify the correct versions for your components:

* Compact toolchain.
* Runtime libraries.
* JavaScript libraries.
* Proof server.
* Indexer (if used).

2

### Update runtime packages[​](#update-runtime-packages "Direct link to Update runtime packages")

Update your `package.json` with compatible versions from the matrix:

package.json

```
{

  "dependencies": {

    "@midnight-ntwrk/compact-runtime": "x.x.x",

    "@midnight-ntwrk/ledger-v8": "x.x.x",

    "@midnight-ntwrk/onchain-runtime-v3": "x.x.x",

    "@midnight-ntwrk/wallet-sdk-facade": "x.x.x"

  }

}
```

Install the updated packages:

```
npm install
```

3

### Update proof server[​](#update-proof-server "Direct link to Update proof server")

If you use Docker, update your container image to a compatible version. Stop the current container, update your `docker-compose.yml` with the correct version, then restart:

```
# Stop current container

docker-compose down



# Update docker-compose.yml with compatible version, then restart

docker-compose up -d
```

4

### Update the compiler[​](#update-the-compiler "Direct link to Update the compiler")

Download and install the compatible compiler version from the [Compact releases](https://github.com/midnightntwrk/compact/releases).

Verify installation:

```
compact --version
```

5

### Recompile contracts[​](#recompile-contracts "Direct link to Recompile contracts")

After updating components, recompile your smart contracts:

```
# Clean old artifacts

rm -rf contract/managed/



# Recompile using direct compact command

compact compile src/contract.compact contract/managed
```

Keep components in sync

Always check the compatibility matrix when updating components. Update all related components together to maintain compatibility across your development environment.

## Lock exact versions[​](#lock-exact-versions "Direct link to Lock exact versions")

Prevent automatic version updates by specifying exact version numbers in your project configuration. This ensures your project remains stable and avoids unexpected compatibility issues.

1

### Use exact version numbers[​](#use-exact-version-numbers "Direct link to Use exact version numbers")

In your `package.json`, specify exact versions without range operators such as `^` or `~`:

package.json

```
{

  "dependencies": {

    "@midnight-ntwrk/compact-runtime": "x.x.x",

    "@midnight-ntwrk/ledger-v8": "x.x.x"

  }

}
```

Avoid version ranges

Do not use version range operators such as `^x.x.x` or `~x.x.x`. Always specify exact versions like `x.x.x` to prevent unexpected updates.

2

### Use npm ci for installations[​](#use-npm-ci-for-installations "Direct link to Use npm ci for installations")

Use `npm ci` instead of `npm install` for reproducible builds:

```
# Clean install from lock file

rm -rf node_modules

npm ci
```

This installs exact versions from `package-lock.json`.

3

### Document your versions[​](#document-your-versions "Direct link to Document your versions")

Create a `VERSIONS.md` file in your project root to document the component versions you are using. This helps team members maintain consistency and troubleshoot version issues:

VERSIONS.md

```
# Component Versions



Last verified: 2025-10-17



## Versions in Use



- Compact compiler: x.x.x

- Runtime packages: x.x.x

- Proof server: x.x.x

- Node.js: xx.x.x



## Compatibility Reference



See: /relnotes/support-matrix
```

## Platform-specific considerations[​](#platform-specific-considerations "Direct link to Platform-specific considerations")

Different operating systems may require specific configuration or verification steps. Select your platform below for relevant guidance.

* Linux
* macOS
* Windows/WSL

Ensure all components are properly installed and accessible:

```
# Check compiler location

which compact



# Verify Node.js version

node --version
```

On Apple Silicon (M1/M2/M3), verify that Docker images use the correct architecture:

```
# Check Docker architecture

docker info | grep Architecture
```

When using Windows Subsystem for Linux (WSL), use native WSL paths rather than Windows mount paths for better performance and consistency:

```
# Use native WSL paths, not Windows mount paths like /mnt/c/

cd /home/username/project



# Verify compiler access

compact --version
```

## Create a version check script[​](#create-a-version-check-script "Direct link to Create a version check script")

Create a simple shell script to automate version checking across your development environment. This script helps you quickly verify that all components match the compatibility matrix.

check-versions.sh

```
#!/bin/bash



echo "=== Midnight Version Check ==="

echo ""



echo "Compiler:"

compact --version || echo "❌ Compiler not found"

echo ""



echo "Runtime packages:"

npm list --depth=0 | grep @midnight-ntwrk || echo "❌ No Midnight packages found"

echo ""



echo "Node.js:"

node --version

echo ""



echo "⚠️  Compare these versions with:"

echo "/relnotes/support-matrix"
```

Make the script executable and run it:

```
chmod +x check-versions.sh

./check-versions.sh
```

## Back up before upgrading[​](#back-up-before-upgrading "Direct link to Back up before upgrading")

Always back up your project files before upgrading components to prevent data loss if issues occur during the upgrade process:

```
# Back up package.json

cp package.json package.json.backup



# Back up contract artifacts

cp -r contract contract-backup



# Commit current state to git

git add -A

git commit -m "Backup before version upgrade"
```

## Verify after updates[​](#verify-after-updates "Direct link to Verify after updates")

After aligning component versions, test your setup to confirm everything works correctly:

```
# Test compilation

compact compile src/contract.compact contract/



# Run tests

npm test



# Check for errors

docker logs [proof-server-container-id]
```

## Common mistakes to avoid[​](#common-mistakes-to-avoid "Direct link to Common mistakes to avoid")

Avoid these common errors when managing component versions:

caution

* **Don't mix version ranges**: Use exact versions only.
* **Don't update one component**: Update all related components together.
* **Don't skip the compatibility matrix**: Always verify compatibility first.
* **Don't forget to recompile**: Recompile contracts after compiler updates.

## Get help[​](#get-help "Direct link to Get help")

If version issues persist after following these troubleshooting steps, use these resources:

1. **Check the compatibility matrix**: Review the [release compatibility matrix](/relnotes/support-matrix.md) for verified compatible versions.
2. **Review release notes**: Check component release notes for breaking changes and migration guidance.
3. **Ask for help**: Post in the #dev-chat channel on Discord with your version details and error messages.
