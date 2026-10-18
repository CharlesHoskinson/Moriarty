import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { execFile } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const PORT = Number(process.env.PORT || 4377);
const RUST_BIN = path.join(__dirname, 'rust-runner', 'sherlock-runner');

const MIME_TYPES = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.mjs': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.md': 'text/markdown; charset=utf-8',
  '.svg': 'image/svg+xml',
  '.png': 'image/png',
};

const server = http.createServer((req, res) => {
  const url = new URL(req.url || '/', `http://${req.headers.host || 'localhost'}`);

  if (req.method === 'POST' && url.pathname === '/api/trace') {
    let body = '';
    req.on('data', (chunk) => {
      body += chunk;
      if (body.length > 256 * 1024) req.destroy();
    });
    req.on('end', () => {
      try {
        const payload = JSON.parse(body || '{}');
        const source = String(payload.source || '');
        const fuel = Math.max(1, Math.min(2000, Number(payload.fuel || 200)));
        const stackLimit = Math.max(1, Math.min(64, Number(payload.stackLimit || 16)));
        const sliceWidth = Math.max(1, Math.min(32, Number(payload.sliceWidth || 4)));

        const child = execFile(
          RUST_BIN,
          ['--fuel', String(fuel), '--stack-limit', String(stackLimit), '--slice-width', String(sliceWidth)],
          { maxBuffer: 16 * 1024 * 1024, timeout: 4000 },
          (err, stdout) => {
            if (err && !stdout) {
              res.writeHead(500, { 'Content-Type': 'application/json' });
              res.end(JSON.stringify({ ok: false, error: err.message }));
              return;
            }
            res.writeHead(200, { 'Content-Type': 'application/json; charset=utf-8' });
            res.end(stdout);
          },
        );
        child.stdin?.write(source);
        child.stdin?.end();
      } catch (e) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ ok: false, error: String(e) }));
      }
    });
    return;
  }

  let reqPath = decodeURIComponent(url.pathname);
  if (reqPath === '/') reqPath = '/index.html';

  const safePath = path.normalize(path.join(__dirname, reqPath));
  if (!safePath.startsWith(__dirname)) {
    res.writeHead(403);
    res.end('Forbidden');
    return;
  }

  fs.stat(safePath, (err, stats) => {
    if (err || !stats.isFile()) {
      res.writeHead(404);
      res.end('Not Found');
      return;
    }
    const ext = path.extname(safePath).toLowerCase();
    res.writeHead(200, { 'Content-Type': MIME_TYPES[ext] || 'application/octet-stream' });
    fs.createReadStream(safePath).pipe(res);
  });
});

server.listen(PORT, '127.0.0.1', () => {
  console.log(`Sherlock Workbench running at http://127.0.0.1:${PORT}`);
});
