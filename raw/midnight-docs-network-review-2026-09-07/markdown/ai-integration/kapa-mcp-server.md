> For the complete documentation index, see [llms.txt](/llms.txt)

# Kapa MCP server

The Kapa MCP server exposes the same Midnight knowledge base that powers the "Ask AI" button on this documentation site. It gives your AI coding assistant documentation-grounded answers about Midnight directly in your editor, so you can ask questions without switching to a browser tab.

For background on why the project moved to Kapa and how it fits alongside Midnight Expert, see the [migration blog post](/blog/migrating-to-kapa-and-midnight-expert).

## Install[​](#install "Direct link to Install")

### Claude Code[​](#claude-code "Direct link to Claude Code")

Run one command:

```
claude mcp add --transport http midnight https://midnight.mcp.kapa.ai
```

### Cursor and VS Code[​](#cursor-and-vs-code "Direct link to Cursor and VS Code")

Open the "Ask AI" button on this site, click "Use MCP", and choose "Add to Cursor" or "Add to VS Code". Both are one-click installs.

### Any other MCP client[​](#any-other-mcp-client "Direct link to Any other MCP client")

For any client that supports remote MCP servers in a JSON config, add the server manually:

```
{

  "mcpServers": {

    "midnight": {

      "type": "http",

      "url": "https://midnight.mcp.kapa.ai"

    }

  }

}
```

Some clients label this a remote or HTTP server and ask for a transport field; the URL is the same in every case. You can also click "Copy MCP URL" from the "Use MCP" menu and paste it wherever your client expects it.

## What it covers[​](#what-it-covers "Direct link to What it covers")

Kapa indexes a curated set of Midnight sources: the official documentation, hand-picked core repositories (the ledger, midnight.js, and the node), and standalone material such as the whitepaper. Ask how DUST generation works, how to structure a witness, or what a specific ledger type does, and you get an answer grounded in current documentation rather than stale training data.

## Midnight Expert[​](#midnight-expert "Direct link to Midnight Expert")

For hands-on development tasks like writing and validating Compact contracts, running a local devnet, or decoding error codes, use [Midnight Expert](/ai-integration/midnight-expert.md) alongside Kapa. See the [Midnight agent skills](/sdks/community/ai-tools/midnight-agent-skills.md) page or the [migration blog post](/blog/migrating-to-kapa-and-midnight-expert) for setup instructions.
