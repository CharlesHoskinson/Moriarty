local M = {}
function M.start(command)
  return vim.lsp.start({
    name = 'moriarty-beta',
    cmd = command or { 'mori', 'lsp' },
    root_dir = vim.fn.getcwd(),
    filetypes = { 'moriarty' },
    capabilities = vim.lsp.protocol.make_client_capabilities(),
  })
end
return M
