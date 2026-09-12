[Skip to main content](#__docusaurus_skipToContent_fallback)

[![My Site Logo](/docu/img/logo.png)![My Site Logo](/docu/img/logo.png)](https://tronweb.network/)[Home](https://tronweb.network/)[Documentation](/docu/docs/intro)[Community](https://tronweb.network/#/community)

[6.5.0](/docu/docs/intro)

* [6.5.0](/docu/docs/intro)
* [6.4.0](/docu/docs/6.4.0/intro)
* [6.3.0](/docu/docs/6.3.0/intro)
* [6.2.2](/docu/docs/6.2.2/intro)
* [6.2.0 - 6.2.1](/docu/docs/6.2.0/intro)
* [6.1.0 - 6.1.1](/docu/docs/6.1.0/intro)
* [6.0.4](/docu/docs/6.0.4/intro)
* [6.0.0](/docu/docs/6.0.0/intro)
* [5.3.2 - 5.3.5](/docu/docs/5.3.2/intro)
* [5.3.0 - 5.3.1](/docu/docs/5.3.0/intro)
* [5.2.0](/docu/docs/5.2.0/intro)
* [5.1.1](/docu/docs/5.1.1/intro)
* [5.1.0](/docu/docs/5.1.0/intro)
* [5.0.0](/docu/docs/5.0.0/intro)

Search

[GitHub](https://github.com/tronprotocol/tronweb)

* [Introduction](/docu/docs/intro)
* [Release Note](/docu/docs/Release%20Note)
* [Quickstart](/docu/docs/quickstart)
* [Learn](/docu/docs/Learn)
* [TronWeb and TronLink wallet](/docu/docs/TronWeb%20and%20TronLink%20wallet)
* [Core concepts](/docu/docs/Core%20concepts)
* [Interact with the contract](/docu/docs/Interact%20with%20contract)
* [Transfer Token](/docu/docs/Transfer%20Token)
* [Account creation](/docu/docs/Account%20creation)
* [Build a local node](/docu/docs/Build%20local%20node)
* [Plugins](/docu/docs/Plugins)
* [Sign and Verify Message](/docu/docs/Sign%20and%20Verify%20Message)
* [Migrating from Tronweb v5](/docu/docs/Migrating%20from%20v5)
* [Advanced](/docu/docs/category/advanced)

  + [Staking V2](/docu/docs/advanced/staking-v2)
  + [Multi-Signature](/docu/docs/advanced/multi-signature)
  + [Governance & Voting](/docu/docs/advanced/governance-voting)
  + [Energy & Bandwidth](/docu/docs/advanced/energy-bandwidth)
  + [Exchange Operations](/docu/docs/advanced/exchange-operations)
* [API List](/docu/docs/API%20List/)
* [GPG verification](/docu/docs/GPG%20verification)
* [License and Copyright](/docu/docs/License%20and%20Copyright)
* [Terminology](/docu/docs/Terminology)
* [Contributings](/docu/docs/Contributings)
* [CookBook](/docu/docs/CookBook)
* [References](/docu/docs/category/references)

* Introduction
Version: 6.5.0

On this page

Introduction
============

#### [release note](/docu/docs/Release%20Note)

Introduction
============

[TronWeb](https://tronweb.network) aims to deliver a unified, seamless development experience for TRON developers. We have taken the core ideas and expanded upon them to unlock the functionality of TRON's unique feature set along with offering new tools for integrating DApps in the browser, Node.js and IoT devices.

Getting Started
---------------

### Live code editor

TronWeb doc provides a live code editor for developers to write and execute TRON smart contract code. You can write your code in the editor and see the results in real time. You can also see the logs of the contract deployment and function calls.

index.ts

```
import { TronWeb } from 'tronweb';
const tronWeb = new TronWeb({
  fullHost: 'https://api.nileex.io',
});
(async () => {
  console.log(await tronWeb.trx.getCurrentBlock());
})()
```

Output:

### Installation

#### Node.js

```
npm install tronweb
```

or

```
yarn add tronweb
```

#### Browser

The easiest way to use TronWeb in a browser is to install it as above and copy the dist file to your working folder. For example:

```
cp node_modules/tronweb/dist/TronWeb.js ./js/tronweb.js
```

so that you can call it in your HTML page as

```
<script src="./js/tronweb.js"></script>
```

**Note**:
Starting from v6.4.0, TronWeb no longer depends on the Buffer module.

### Instantiation

Then, in your JavaScript file, define TronWeb:

```
const { TronWeb } = require('tronweb');
```

**Note**:
Starting from version 6.0.0, TronWeb supports both CommonJS and ES Modules (ESM) to ensure compatibility with different JavaScript environments.
Use `const { TronWeb } = require('tronweb')` to load the CommonJS version.
Use `import { TronWeb } from 'tronweb'` to load the ESM version.
This allows seamless integration whether you're working in a traditional Node.js project or using modern build tools like Vite, Webpack, or Rollup.

When you instantiate TronWeb you can set

* fullHost

Supposing you are using a server that provides everything, like TronGrid, you can instantiate TronWeb as:

```
const tronWeb = new TronWeb({  
  fullHost: 'https://api.trongrid.io',  
  headers: { 'TRON-PRO-API-KEY': 'your api key' },  
  privateKey: 'your private key'  
});
```

Like infura API Key, you can sign up for [TronGrid](https://www.trongrid.io/) and create your API key on the dashboard to access TRON Network data.

Note

* **Do not expose your private key** in any web browser environment.
* You can instantiate TronWeb **without privateKey**, if you only need to use some `utils` functions such as `TronWeb.utils`
* If you only want to **query information** from the TRON Network blockchain **without signing a transaction**, you have two options:
  + Pass a **placeholder private key** (e.g. `01`) when instantiating TronWeb. This is a convenient way to avoid passing an address manually in each API call.
  + Instantiate TronWeb **without any private key**. In this case, you need to pass the required address explicitly in the corresponding method parameters when calling on-chain APIs such as `triggerConstantContract`, `getTransactionInfo`.

Compatibility
-------------

* Version built for Node.js v14 and above
* Version built for browsers with more than 0.25% market share

TronWeb is also compatible with frontend frameworks such as:

* Angular
* React
* Vue

You can also ship TronWeb in a Chrome extension such as [TronLink extension](https://chrome.google.com/webstore/detail/tronlink/ibnejdfjmmkpcnlpebklmnkoeoihofec)

Interact with TRON wallets
--------------------------

To make it easier for DApp developers to connect with TRON wallets, the community provides a set of wallet adapters compatible with the TRON ecosystem. These adapters currently support TronLink Extension (Chrome and Firefox), TronLink APP (Android/iOS), Ledger, Walletconnnect and etc.

These adapters offer a unified API for interacting with different TRON-compatible wallets, enabling seamless integration across both desktop and mobile environments.

The wallet adapters are maintained by the community under the [tronweb3/tronwallet-adapter](https://github.com/tronweb3/tronwallet-adapter) project on GitHub.

[Next

Release Note](/docu/docs/Release%20Note)

* [Getting Started](#getting-started)
  + [Live code editor](#live-code-editor)
  + [Installation](#installation)
  + [Instantiation](#instantiation)
* [Compatibility](#compatibility)
* [Interact with TRON wallets](#interact-with-tron-wallets)

Ask AI...

![](/docu/img/footer-logo.png)![](/docu/img/footer-logo.png)

Learn

* [Introduction](/docu/docs/intro)
* [Quickstart](/docu/docs/quickstart)

Community

* [GitHub](https://github.com/tronprotocol/tronweb)
* [Telegram](https://t.me/TronOfficialDevelopersGroupEn)
* [Discord](https://discord.com/invite/hqKvyAM)
* [Help](https://github.com/tronprotocol/tronweb/issues/new)

Partners

* [TronBox](https://tronbox.io/)
* [Tron-IDE](https://www.tronide.io/)
* [TronWallet-Adapter](https://github.com/tronprotocol/tronwallet-adapter)