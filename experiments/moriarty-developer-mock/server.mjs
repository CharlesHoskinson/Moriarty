import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';

const routes = new Map([
  ['/', ['index.html', 'text/html; charset=utf-8']],
  ['/language', ['language.html', 'text/html; charset=utf-8']],
  ['/styles.css', ['styles.css', 'text/css; charset=utf-8']],
  ['/app.js', ['dist/app.js', 'text/javascript; charset=utf-8']],
  ['/language-app.js', ['dist/language-app.js', 'text/javascript; charset=utf-8']],
  ['/language/core.js', ['dist/language/core.js', 'text/javascript; charset=utf-8']],
  ['/language/packages.js', ['dist/language/packages.js', 'text/javascript; charset=utf-8']],
  ['/language/policy.js', ['dist/language/policy.js', 'text/javascript; charset=utf-8']],
  ['/language/claims.js', ['dist/language/claims.js', 'text/javascript; charset=utf-8']],
  ['/model.js', ['dist/model.js', 'text/javascript; charset=utf-8']],
  ['/fixtures.json', ['fixtures.json', 'application/json; charset=utf-8']],
]);
const port = Number(process.env.MORIARTY_MOCK_PORT || '4173');
if (!Number.isInteger(port) || port < 1024 || port > 65535) throw new Error('MORIARTY_MOCK_PORT must be an integer from 1024 to 65535.');
const server = createServer(async (req, res) => {
  const route = routes.get((req.url || '/').split('?')[0]);
  if (!route || !['GET', 'HEAD'].includes(req.method || '')) { res.writeHead(404); res.end('Not found'); return; }
  try {
    const data = await readFile(new URL(route[0], import.meta.url));
    res.writeHead(200, { 'Content-Type': route[1], 'Cache-Control': 'no-store',
      'X-Content-Type-Options': 'nosniff', 'Referrer-Policy': 'no-referrer',
      'Content-Security-Policy': "default-src 'self'; connect-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'none'; frame-ancestors 'none'" });
    res.end(req.method === 'HEAD' ? undefined : data);
  } catch { res.writeHead(503); res.end('Build the mock first with npm run build.'); }
});
server.listen(port, '127.0.0.1', () => process.stdout.write(`Moriarty developer mock: http://127.0.0.1:${port}\n`));
