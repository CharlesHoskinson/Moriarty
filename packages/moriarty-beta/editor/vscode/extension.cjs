const vscode = require('vscode');
const path = require('node:path');
const { LanguageClient } = require('vscode-languageclient/node');
let client;
async function activate(context) {
  const node = vscode.workspace.getConfiguration('moriarty').get('nodePath', 'node');
  const server = { command: node, args: [context.asAbsolutePath(path.join('server', 'cli.js')), 'lsp'], options: { shell: false } };
  client = new LanguageClient('moriarty', 'Moriarty Beta', server, { documentSelector: [{ scheme: 'file', language: 'moriarty' }], outputChannelName: 'Moriarty Beta' });
  context.subscriptions.push(client);
  await client.start();
}
async function deactivate() { if (client) await client.stop(); }
module.exports = { activate, deactivate };
