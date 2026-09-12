> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Supported Assets

> All tokens supported by NEAR Intents, sourced live from the 1Click API.

export const AssetTable = () => {
  const [tokens, setTokens] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [search, setSearch] = useState("");
  const [sortBy, setSortBy] = useState("symbol");
  const CHAIN_NAMES = {
    adi: "ADI",
    aleo: "Aleo",
    aptos: "Aptos",
    arb: "Arbitrum",
    avax: "Avalanche",
    base: "Base",
    bch: "Bitcoin Cash",
    bera: "Berachain",
    bsc: "Binance Smart Chain",
    btc: "Bitcoin",
    cardano: "Cardano",
    dash: "Dash",
    doge: "Dogecoin",
    eth: "Ethereum",
    gnosis: "Gnosis Chain",
    ltc: "Litecoin",
    monad: "Monad",
    near: "NEAR",
    op: "Optimism",
    plasma: "Plasma",
    pol: "Polygon",
    scroll: "Scroll",
    sol: "Solana",
    starknet: "Starknet",
    stellar: "Stellar",
    sui: "Sui",
    ton: "TON",
    tron: "Tron",
    xlayer: "XLayer",
    xrp: "Ripple",
    zec: "ZCash"
  };
  const chainLabel = raw => CHAIN_NAMES[raw] ?? raw;
  useEffect(() => {
    fetch("https://1click.chaindefuser.com/v0/tokens").then(r => r.json()).then(data => {
      setTokens(data);
      setLoading(false);
    }).catch(() => {
      setError("Failed to load token data.");
      setLoading(false);
    });
  }, []);
  const filtered = tokens.filter(t => {
    const q = search.toLowerCase();
    return t.symbol.toLowerCase().includes(q) || chainLabel(t.blockchain).toLowerCase().includes(q);
  }).sort((a, b) => {
    const valA = sortBy === "blockchain" ? chainLabel(a.blockchain) : a.symbol;
    const valB = sortBy === "blockchain" ? chainLabel(b.blockchain) : b.symbol;
    return valA.localeCompare(valB);
  });
  return <div>
      <div className="flex gap-3 mb-5 flex-wrap items-center">
        <input type="text" placeholder="Search by symbol or blockchain…" value={search} onChange={e => setSearch(e.target.value)} className="px-3 py-2 rounded-md border border-white/15 text-sm flex-1 min-w-[200px] bg-black text-white outline-none" />

        <div className="flex items-center gap-2.5">
          <span className="text-sm text-gray-400 whitespace-nowrap">Sort by:</span>
          <div className="flex bg-white/6 rounded-lg p-[3px] gap-0.5">
            {["symbol", "blockchain"].map(opt => <button key={opt} onClick={() => setSortBy(opt)} className={`py-[5px] px-[14px] rounded-md border-0 cursor-pointer text-[13px] capitalize transition-all duration-150 ${sortBy === opt ? "font-semibold bg-orange-500 text-white" : "font-normal bg-transparent text-gray-400"}`}>
                {opt === "symbol" ? "Symbol" : "Blockchain"}
              </button>)}
          </div>
        </div>
      </div>

      {loading && <p>Loading tokens…</p>}
      {error && <p className="text-red-500">{error}</p>}

      {!loading && !error && <>
          <p className="text-[13px] text-gray-500 mb-3">
            {filtered.length} token{filtered.length !== 1 ? "s" : ""} found
          </p>
          <table className="w-full text-sm table-fixed">
            <thead>
              <tr className="border-b border-white/10">
                <th className="py-3 pr-4 text-left font-semibold">Symbol</th>
                <th className="py-3 px-4 text-left font-semibold">Blockchain</th>
                <th className="py-3 pl-4 text-left font-semibold w-1/2">Asset ID</th>
              </tr>
            </thead>
            <tbody>
              {filtered.map(t => <tr key={t.assetId} className="border-b border-white/10">
                  <td className="py-3 pr-4 font-medium">{t.symbol}</td>
                  <td className="py-3 px-4">{chainLabel(t.blockchain)}</td>
                  <td className="py-3 pl-4 max-w-0">
                    <div className="overflow-x-auto whitespace-nowrap">{chainLabel(t.assetId)}</div>
                  </td>
                </tr>)}
            </tbody>
          </table>
        </>}
    </div>;
};

The list below is sourced live from the [1Click API](https://1click.chaindefuser.com/v0/tokens) and reflects currently supported tokens. The list is continuously expanding — [join our Telegram](https://t.me/near_intents) for announcements.

<AssetTable />
