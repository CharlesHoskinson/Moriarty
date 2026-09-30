-- Actual installed Neovim client; no user configuration or network calls.
local cli = assert(vim.env.MORIARTY_BETA_CLI, 'set MORIARTY_BETA_CLI to an absolute CLI path')
local source = {
  'profile "moriarty-beta/1";',
  'agreement Demo {',
  '  // 😀 Unicode comment',
  '  const amount = 1;',
  '  const total = amount + 1;',
  '}',
}
vim.cmd('enew')
vim.api.nvim_buf_set_name(0, vim.fn.tempname() .. '.mori')
vim.bo.filetype = 'moriarty'
vim.api.nvim_buf_set_lines(0, 0, -1, false, source)
local id = assert(dofile('editor/nvim/moriarty.lua').start({ 'node', cli, 'lsp' }))
assert(vim.wait(5000, function() local c = vim.lsp.get_client_by_id(id); return c and c.initialized end), 'LSP initialize timeout')
local client = assert(vim.lsp.get_client_by_id(id))
assert(client.offset_encoding == 'utf-16', 'UTF-16 encoding was not negotiated')
local uri = vim.uri_from_bufnr(0)
local function request(method, params)
  local reply, err = client:request_sync(method, params, 5000, 0)
  assert(reply and not reply.err, vim.inspect(err or (reply and reply.err)))
  return reply.result
end
local symbols = request('textDocument/documentSymbol', { textDocument = { uri = uri } })
assert(#symbols == 2 and symbols[1].name == 'amount', vim.inspect(symbols))
local def = request('textDocument/definition', { textDocument = { uri = uri }, position = { line = 4, character = 17 } })
assert(def.range.start.line == 3, vim.inspect(def))
local edits = request('textDocument/formatting', { textDocument = { uri = uri }, options = { tabSize = 2, insertSpaces = true } })
assert(#edits == 1, vim.inspect(edits))
vim.api.nvim_buf_set_lines(0, 4, 5, false, { '  const total = missing;' })
-- Neovim debounces didChange; requesting after its flush exercises the live client.
assert(vim.wait(5000, function() return #vim.diagnostic.get(0) > 0 end), 'didChange diagnostics timeout')
local invalid = request('textDocument/formatting', { textDocument = { uri = uri }, options = { tabSize = 2, insertSpaces = true } })
assert(#invalid == 0, 'invalid source must not format')
client:stop()
assert(vim.wait(5000, function() return client:is_stopped() end), 'shutdown timeout')
print('Neovim Moriarty stdio LSP smoke passed: initialize/open/definition/symbols/format/change/diagnostics/shutdown')
vim.cmd('qa!')
