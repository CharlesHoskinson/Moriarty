> For the complete documentation index, see [llms.txt](/llms.txt)

# Fix package repository access failures

Resolve 403 Forbidden errors encountered when installing Midnight packages.

## Understand the cause of the error[​](#understand-the-cause-of-the-error "Direct link to Understand the cause of the error")

When installing Midnight packages, you might encounter a 403 Forbidden error. This error typically appears when your npm configuration points to an incorrect registry or when network restrictions block access:

```
npm install @midnight-ntwrk/compact-runtime



npm ERR! code E403

npm ERR! 403 Forbidden - GET https://registry.npmjs.org/@midnight-ntwrk/compact-runtime

npm ERR! 403 In most cases, you or one of your dependencies are requesting

npm ERR! 403 a package version that is forbidden by your security policy
```

The following solutions address the most common causes of this error, starting with the simplest fix.

## Solution 1: Reset npm registry[​](#solution-1-reset-npm-registry "Direct link to Solution 1: Reset npm registry")

The most common cause is an npm configuration pointing to the wrong registry. Reset the npm registry to the default public npm registry to resolve this issue.

1

### Check current registry[​](#check-current-registry "Direct link to Check current registry")

Check which registry npm is currently configured to use:

```
npm config get registry
```

If the output differs from `https://registry.npmjs.org/`, then the registry configuration is incorrect and needs resetting.

2

### Reset to default registry[​](#reset-to-default-registry "Direct link to Reset to default registry")

Reset the npm registry configuration to the default public registry, remove any scoped registry settings, and clear the npm cache:

```
npm config set registry https://registry.npmjs.org/

npm config delete @midnight-ntwrk:registry

npm cache clean --force
```

3

### Test installation[​](#test-installation "Direct link to Test installation")

```
npm install @midnight-ntwrk/compact-runtime
```

**Verification**: Package installs successfully without errors.

## Solution 2: Fix VPN and proxy issues[​](#solution-2-fix-vpn-and-proxy-issues "Direct link to Solution 2: Fix VPN and proxy issues")

If resetting the registry does not resolve the issue, VPN connections or corporate proxy servers may be blocking npm access. Try these steps to configure npm to work with your network setup.

1

### Disable VPN temporarily[​](#disable-vpn-temporarily "Direct link to Disable VPN temporarily")

Corporate Networks

On corporate networks, disabling a VPN connection may not be permitted.

Disconnect from the VPN and test:

```
npm install @midnight-ntwrk/compact-runtime
```

If the installation succeeds, VPN restrictions are blocking npm access. Configure proxy settings instead.

2

### Configure proxy (if VPN is required)[​](#configure-proxy-if-vpn-is-required "Direct link to Configure proxy (if VPN is required)")

If you must remain connected to a VPN, configure npm to use your corporate proxy server. Replace `proxy.company.com:8080` with your actual proxy address:

```
# Set proxy settings

npm config set proxy http://proxy.company.com:8080

npm config set https-proxy http://proxy.company.com:8080



# With authentication (if required)

npm config set proxy http://username:password@proxy.company.com:8080

npm config set https-proxy http://username:password@proxy.company.com:8080
```

3

### Test installation[​](#test-installation-1 "Direct link to Test installation")

```
npm install @midnight-ntwrk/compact-runtime
```

**Verification**: Package installs successfully through proxy.

## Solution 3: Clear configuration and start fresh[​](#solution-3-clear-configuration-and-start-fresh "Direct link to Solution 3: Clear configuration and start fresh")

If the previous solutions do not work, conflicting npm configurations may be causing the error. Clear all npm settings and restore default configurations to eliminate any misconfigurations.

1

### Back up and clear configuration[​](#back-up-and-clear-configuration "Direct link to Back up and clear configuration")

Important

Back up the npm configuration before clearing it, as this process removes all custom settings.

```
# Back up existing config

cp ~/.npmrc ~/.npmrc.backup



# Remove all settings

npm config delete registry

npm config delete proxy

npm config delete https-proxy

npm config delete @midnight-ntwrk:registry
```

2

### Set defaults and clean project[​](#set-defaults-and-clean-project "Direct link to Set defaults and clean project")

Data Loss Warning

The following commands delete node\_modules and package-lock.json. Ensure the correct project directory is active before proceeding.

```
# Set registry

npm config set registry https://registry.npmjs.org/



# Clean project

rm -rf node_modules package-lock.json

npm cache clean --force
```

3

### Test installation[​](#test-installation-2 "Direct link to Test installation")

```
npm install @midnight-ntwrk/compact-runtime
```

**Verification**: Package installs successfully.

## Solution 4: Authenticate to GitHub Package Registry[​](#solution-4-authenticate-to-github-package-registry "Direct link to Solution 4: Authenticate to GitHub Package Registry")

If the previous solutions do not resolve the issue, the cause may be GitHub Package Registry authentication rather than an npm registry misconfiguration. Midnight publishes `@midnight-ntwrk/*` packages to `https://npm.pkg.github.com/`, which requires authentication even for public packages. This solution applies to Yarn Berry (v2+) projects that use a `.yarnrc.yml` file.

Without a configured token, every install attempt fails with:

```
@midnight-ntwrk/ledger@npm:4.0.0::__archiveUrl=...: Invalid authentication (as an unknown user)
```

To resolve this, configure `npmAuthToken` in your `.yarnrc.yml`. You also need a GitHub personal access token with the `read:packages` scope, provided through the `GITHUB_TOKEN` environment variable.

1

### Generate a GitHub personal access token[​](#generate-a-github-personal-access-token "Direct link to Generate a GitHub personal access token")

Create a classic personal access token (PAT) at `https://github.com/settings/tokens/new` with the `read:packages` scope. A fine-grained token with the **Packages: Read** permission also works.

Organization access

For organization-owned packages, your token must also have access to the `midnightntwrk` organization.

2

### Update `.yarnrc.yml`[​](#update-yarnrcyml "Direct link to update-yarnrcyml")

Add the `npmAuthToken` field under the `midnight-ntwrk` scope, pointing at the `GITHUB_TOKEN` environment variable:

```
npmScopes:

  midnight-ntwrk:

    npmAlwaysAuth: true

    npmRegistryServer: "https://npm.pkg.github.com/"

    npmAuthToken: "${GITHUB_TOKEN}"
```

Token security

Yarn interpolates `${GITHUB_TOKEN}` from the environment at install time, so the token never lives in `.yarnrc.yml`.

3

### Export the token[​](#export-the-token "Direct link to Export the token")

Export the `GITHUB_TOKEN` variable so yarn can interpolate it at install time:

```
export GITHUB_TOKEN=<your-github-token>
```

If you prefer not to set the environment variable globally, prefix the install command on the same line: `GITHUB_TOKEN=<your-github-token> yarn install`.

4

### Test installation[​](#test-installation-3 "Direct link to Test installation")

```
yarn install @midnight-ntwrk/compact-runtime
```

**Verification**: Installation completes without `Invalid authentication` errors and `@midnight-ntwrk/*` packages resolve from `npm.pkg.github.com`.

## Platform-specific fixes[​](#platform-specific-fixes "Direct link to Platform-specific fixes")

Some npm access issues are specific to particular operating systems or environments. Use these platform-specific solutions if the general fixes above do not resolve your issue.

* Windows/WSL
* Linux
* macOS

On Windows Subsystem for Linux (WSL), line ending issues and cache conflicts between Windows and WSL can cause npm errors. Use these commands to resolve WSL-specific issues:

```
# Configure git line endings

git config --global core.autocrlf false



# Use native WSL paths (not /mnt/c/...)

cd /home/yourusername/project



# Clear Windows npm cache

rm -rf /mnt/c/Users/YourName/AppData/Roaming/npm-cache
```

Replace `yourusername` and `YourName` with your actual username.

On Linux, permission errors can occur when npm tries to install packages globally. Configure npm to use a directory in your home folder instead of requiring sudo:

```
# Configure npm without sudo

mkdir ~/.npm-global

npm config set prefix '~/.npm-global'



# Add to ~/.bashrc

export PATH=~/.npm-global/bin:$PATH

source ~/.bashrc
```

After running these commands, npm installs global packages to `~/.npm-global` without requiring elevated permissions.

On macOS, cached credentials in the system keychain can cause authentication issues with npm. Clear these credentials to force npm to authenticate fresh:

macOS security

The system may prompt for your password when running this command.

```
# Clear keychain credentials

security delete-generic-password -s 'npm' -a 'bearer'
```

If the command returns an error saying the password was not found, no credentials were stored and you can proceed with other solutions.

## Quick fix script[​](#quick-fix-script "Direct link to Quick fix script")

For convenience, you can create an automated shell script that applies multiple fixes at once. This script resets npm configuration, clears caches, and tests the installation:

fix-midnight-npm.sh

```
#!/bin/bash



echo "🔧 Fixing Midnight npm installation..."



# Clear configuration

npm config delete registry

npm config delete @midnight-ntwrk:registry

npm config delete proxy

npm config delete https-proxy



# Set defaults

npm config set registry https://registry.npmjs.org/

npm cache clean --force



# Clean project (only if package.json exists)

if [ -f "package.json" ]; then

    echo "📁 Cleaning project files..."

    rm -rf node_modules package-lock.json

else

    echo "⚠️  No package.json found - skipping project cleanup"

fi



# Test installation

echo "🧪 Testing package installation..."

npm install @midnight-ntwrk/compact-runtime



if [ $? -eq 0 ]; then

    echo "✅ Success! Midnight packages are accessible."

else

    echo "❌ Installation failed. Try disconnecting from VPN or checking firewall settings."

fi
```

Make the script executable and run it:

```
chmod +x fix-midnight-npm.sh

./fix-midnight-npm.sh
```

## Verify the fix[​](#verify-the-fix "Direct link to Verify the fix")

After applying the fixes, confirm that all configurations are correct and installations work properly:

```
# Check registry

npm config get registry

# Expected output: https://registry.npmjs.org/



# Test package access

npm view @midnight-ntwrk/compact-runtime

# Expected output: package information



# Verify installation

npm install @midnight-ntwrk/compact-runtime

# Expected result: successful installation
```

## Still not working?[​](#still-not-working "Direct link to Still not working?")

If none of the solutions above resolve the issue, use these advanced debugging techniques to identify the root cause.

**Enable verbose logging**

Display detailed error information by running npm with the verbose flag:

```
npm install @midnight-ntwrk/compact-runtime --verbose
```

Common issues revealed by verbose logging include:

* SSL certificate validation failures.
* Proxy configuration errors or authentication issues.
* DNS resolution failures when contacting the registry.

**Test direct connection**

Verify registry access:

```
curl https://registry.npmjs.org/@midnight-ntwrk/compact-runtime
```

A failed request indicates a network connectivity issue.

## Next steps[​](#next-steps "Direct link to Next steps")

After successfully installing Midnight packages, verify that your development environment is properly configured and the packages are working correctly:

```
# Check installed packages

npm list @midnight-ntwrk



# Verify compiler

compact --version



# Test runtime import

node -e "const runtime = require('@midnight-ntwrk/compact-runtime'); console.log('✅ Runtime loaded');"
```
