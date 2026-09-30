# Moriarty editor assets

The TextMate grammar and snippets provide lexical assistance for `.mori` files.
The VS Code extension uses the bundled CLI's actual stdio LSP. That server offers
Full synchronization, UTF-16 diagnostics, lexical completion, hover, definition,
document symbols and strict formatting. Semantic tokens and debugging are absent.
Financial relations outside local S0 remain open.

CR, LF and CRLF line endings use the same UTF-16 position rules. A Full change
over the source or aggregate byte limit records its new version and resource
diagnostic. That generation retains no source or editor results until a newer
valid Full change arrives; it cannot reuse the previous successful analysis.

From `packages/moriarty-beta`, prepare and package the VS Code extension:

```sh
npm run build
node editor/vscode/prepare.mjs
cd editor/vscode
npm install --omit=dev --ignore-scripts
npm exec --yes --package @vscode/vsce@3.6.2 -- vsce package --allow-missing-repository
```

The package includes its own `server/cli.js`; it has no repository metadata
requirement. Install the resulting VSIX through VS Code's local extension menu.
The server requires Node >=24 on PATH, or the explicit `moriarty.nodePath` setting.
Packaging and protocol tests do not establish activation in VS Code. This checkout
contains no global editor configuration changes.

For Neovim 0.11+, load `nvim/moriarty.lua` explicitly from project configuration.
Pass an absolute path to the installed `mori` executable, or Node and the installed
CLI. The returned client ID identifies the attached server:

```lua
vim.bo.filetype = 'moriarty'
dofile('/path/to/editor/nvim/moriarty.lua').start({'node', '/path/to/dist/cli.js', 'lsp'})
```

Run the actual headless client smoke test from the beta package directory:

```sh
MORIARTY_BETA_CLI="$PWD/dist/cli.js" nvim --headless -u NONE -l editor/nvim/smoke.lua
```

Other LSP clients may use `mori lsp` with stdio Full synchronization and UTF-16
positions; their activation has not been tested by these assets.
