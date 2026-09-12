> For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). Markdown versions of documentation pages are available by appending `.md` to page URLs; this page is available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/notation.md).

# Notation

The current v0 API currently uses some nonstandard notation. Relevant standardization will be batched into a breaking v1 API change.

<table><thead><tr><th width="231.33333333333331">Abbreviation</th><th>Full name</th><th>Explanation</th></tr></thead><tbody><tr><td>Px</td><td>Price</td><td></td></tr><tr><td>Sz</td><td>Size</td><td>In units of coin, i.e. base currency</td></tr><tr><td>Szi</td><td>Signed size </td><td>Positive for long, negative for short</td></tr><tr><td>Ntl</td><td>Notional</td><td>USD amount, Px * Sz </td></tr><tr><td>Side</td><td>Side of trade or book</td><td>B = Bid = Buy, A = Ask = Short. Side is aggressing side for trades.</td></tr><tr><td>Asset</td><td>Asset</td><td>An integer representing the asset being traded. See below for explanation</td></tr><tr><td>Tif</td><td>Time in force</td><td>GTC = good until canceled, ALO = add liquidity only (post only), IOC = immediate or cancel</td></tr></tbody></table>
