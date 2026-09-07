> For the complete documentation index, see [llms.txt](/llms.txt)

# Neovim setup for Compact

[compact.vim](https://github.com/1NickPappas/compact.vim) is a community-driven plugin that provides Compact language support for Neovim.

## Features[​](#features "Direct link to Features")

The plugin provides the following capabilities for working with Compact source files:

* Syntax highlighting (regex-based and tree-sitter)
* Smart indentation
* Code folding
* Text objects (requires [nvim-treesitter-textobjects](https://github.com/nvim-treesitter/nvim-treesitter-textobjects))
* Import navigation via `gf`
* Compiler integration via `:make`
* Local scoping (variable references scoped per circuit/block)

![Syntax highlighting](/assets/images/compact-syntax-19f5693a997c34fd63b937000110cb69.png)

## Installation[​](#installation "Direct link to Installation")

Install with [lazy.nvim](https://github.com/folke/lazy.nvim):

```
{ "1NickPappas/compact.vim" }
```
