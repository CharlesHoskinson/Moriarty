[![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)

Hyperliquid Docs](/hyperliquid-docs)

`⌘Ctrl``k`

* [Hyperliquid Docs](/hyperliquid-docs)
* [Builder Tools](/hyperliquid-docs/builder-tools)
* [Support](/hyperliquid-docs/support)

For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). This page is also available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/onboarding/how-to-start-trading.md).

Copy

On this page

1. [Hyperliquid Docs](/hyperliquid-docs)
2. [Onboarding](/hyperliquid-docs/onboarding)

How to start trading
====================

### What do I need to trade on Hyperliquid?

You can trade on Hyperliquid with a normal defi wallet or by logging in with your email address.

If you choose to use a normal defi wallet, you need:

1. An EVM wallet

   * If you don’t already have an EVM wallet (e.g., Rabby, MetaMask, WalletConnect, Coinbase Wallet), you can set one up easily at <https://rabby.io/>.
   * After downloading a wallet extension for your browser, create a new wallet.
   * Your wallet has a secret recovery phrase – anyone with access to your private key or seed phrase can access your funds. Do not share these with anyone. Best practice is to record these and store them in a safe physical location.
2. Collateral

   * USDC and ETH (gas to deposit) on Arbitrum, or
   * BTC on Bitcoin, ETH/ENA on Ethereum, SOL/2Z/BONK/FARTCOIN/PUMP/SPX on Solana, MON on Monad, or XPL on Plasma which can be traded for USDC on Hyperliquid

### How do I onboard to Hyperliquid?

There are many different interfaces and apps you can use, including

* [Based](https://based.one/) (web, iOS, Android)
* [Dexari](https://dexari.com/) (iOS, Android)
* [MetaMask](https://metamask.io/) (iOS, Android)
* [Phantom](https://phantom.com/) (web extension, iOS, Android)
* [app.hyperliquid.xyz](https://app.hyperliquid.xyz/trade) (web)

If you choose to log in to app.hyperliquid.xyz with email:

1. Click the "Connect" button and enter your email address. After you press "Submit," within a few seconds, a 6 digit code will be sent to your email. Type in the 6 digit code to login.
2. Now you're connected. All that's left is to deposit. Click Deposit to open the deposit window:

   1. You can send USDC on Arbitrum/Ethereum/Base/Polygon to the deposit address shown (or scan the QR code) from a centralized exchange or another defi wallet. Your deposit arrives as USDC on HyperCore.
   2. You can also send BTC on Bitcoin, ETH/ENA on Ethereum, SOL/2Z/ANSEM/BONK/FARTCOIN/PUMP/SPX on Solana, MON on Monad, XPL on Plasma, AVAX on Avalanche, or ZEC on Zcash.

If you choose to connect to app.hyperliquid.xyz with a defi wallet:

1. Click the “Connect” button and choose a wallet to connect. A pop-up will appear in your wallet extension asking you to connect to Hyperliquid. Press “Connect.”
2. Click the “Enable Trading” button. A pop-up will appear in your wallet extension asking you to sign a gas-less transaction. Press "Sign."
3. Deposit to Hyperliquid, choosing from USDC on Arbitrum, BTC on Bitcoin, ETH/ENA on Ethereum, SOL/2Z/ANSEM/BONK/FARTCOIN/PUMP/SPX on Solana, MON on Monad, XPL on Plasma, AVAX on Avalanche, or ZEC on Zcash.

   1. For USDC: enter the amount you want to deposit and click “Deposit.” Confirm the transaction in your EVM wallet.
   2. For the others: send the spot asset to the destination address shown. Note that you will have to sell this asset for USDC, USDT, or whichever quote asset is used for the assets you're interested in trading.
4. You're now ready to trade.

### How do I trade perpetuals on Hyperliquid?

With perpetual contracts, you use USDC as collateral to long or short the token instead of buying the token itself, like in spot trading.

1. Using the token selector, choose a token that you want to open a position in.
2. Decide if you want to long or short that token. If you expect the token price to go up, you want to long. If you expect the token price to go down, you want to short.
3. Use the slider or type in the size of your position. Position size = your leverage amount \* your collateral
4. Lastly, click Place Order. Click Confirm in the modal that appears. You can check the “Don’t show this again” box so you don’t have to confirm each order in the future.

### How do I get USDC onto Hyperliquid?

1. Many CEXs and bridges support withdrawals on Hyperliquid, either via HyperCore or HyperEVM.
2. You can also transfer USDC from other chains via CCTP. For this, you will need USDC and the native gas token on your source chain. The gas is only for depositing. Trading on Hyperliquid is gas-free.
3. Arbitrum is a common chain to receive USDC, if your centralized exchange does not directly support Hyperliquid.

   1. To get USDC on Arbitrum, you can use various bridges, such as <https://bridge.arbitrum.io/>, <https://app.debridge.finance/>, <https://swap.mayan.finance/>, <https://app.across.to/bridge?>, <https://routernitro.com/swap>, <https://jumper.exchange/>, <https://synapseprotocol.com/>, and <https://relay.link/bridge>
   2. Alternatively, you can move funds directly to Arbitrum from a centralized exchange, if you’re already using one.
   3. Once you have ETH and USDC on Arbitrum, you can deposit by clicking the “Deposit” button on <https://app.hyperliquid.xyz/trade>

### How do I withdraw USDC from Hyperliquid?

1. On <https://app.hyperliquid.xyz/trade>, click the “Withdraw” button in the bottom right.
2. Follow the steps. Depending on the withdrawal chain and method, there may be small gas fees to process the withdrawal.

[PreviousOnboarding](/hyperliquid-docs/onboarding)[NextHow to use the HyperEVM](/hyperliquid-docs/onboarding/how-to-use-the-hyperevm)

Last updated 1 month ago

* [What do I need to trade on Hyperliquid?](#what-do-i-need-to-trade-on-hyperliquid)
* [How do I onboard to Hyperliquid?](#how-do-i-onboard-to-hyperliquid)
* [How do I trade perpetuals on Hyperliquid?](#how-do-i-trade-perpetuals-on-hyperliquid)
* [How do I get USDC onto Hyperliquid?](#how-do-i-get-usdc-onto-hyperliquid)
* [How do I withdraw USDC from Hyperliquid?](#how-do-i-withdraw-usdc-from-hyperliquid)