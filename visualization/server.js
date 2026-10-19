import http from 'node:http';
import https from 'node:https';
import fs from 'node:fs';
import path from 'node:path';
import zlib from 'node:zlib';
import { execFile } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const PORT = Number(process.env.PORT || 4377);
const BS_PORT = Number(process.env.BS_PORT || 4388);
const RUST_BIN = path.join(__dirname, 'rust-runner', 'sherlock-runner');
const BS_CONFIG_PATH = path.join(__dirname, '.browserstack.json');

const MIME_TYPES = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.mjs': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.map': 'application/json; charset=utf-8',
  '.md': 'text/markdown; charset=utf-8',
  '.svg': 'image/svg+xml',
  '.png': 'image/png',
  '.mp3': 'audio/mpeg',
  '.wav': 'audio/wav',
  '.ttf': 'font/ttf',
  '.tgz': 'application/gzip',
};

const COMPRESSIBLE_EXTS = new Set(['.html', '.js', '.mjs', '.css', '.json', '.md', '.svg']);

function readBsConfig() {
  try {
    if (fs.existsSync(BS_CONFIG_PATH)) {
      return JSON.parse(fs.readFileSync(BS_CONFIG_PATH, 'utf8'));
    }
  } catch (_) {}
  return null;
}

function verifyBrowserStackPlan(username, accessKey) {
  return new Promise((resolve) => {
    const auth = Buffer.from(`${username}:${accessKey}`).toString('base64');
    const req = https.request(
      'https://api.browserstack.com/automate/plan.json',
      {
        method: 'GET',
        headers: {
          Authorization: `Basic ${auth}`,
          Accept: 'application/json',
        },
        timeout: 6000,
      },
      (res) => {
        const chunks = [];
        res.on('data', (c) => chunks.push(c));
        res.on('end', () => {
          try {
            const data = JSON.parse(Buffer.concat(chunks).toString('utf8'));
            if (res.statusCode === 200) {
              resolve({ verified: true, plan: data });
            } else {
              resolve({
                verified: false,
                verifyNote: data.message || `HTTP ${res.statusCode}`,
              });
            }
          } catch (e) {
            resolve({ verified: false, verifyNote: String(e) });
          }
        });
      }
    );
    req.on('error', (err) => resolve({ verified: false, verifyNote: err.message }));
    req.on('timeout', () => {
      req.destroy();
      resolve({ verified: false, verifyNote: 'Timeout contacting api.browserstack.com' });
    });
    req.end();
  });
}

// In-memory LRU cache for `/api/trace` responses (32 entries)
const traceCache = new Map();
function getCachedTrace(key) {
  if (!traceCache.has(key)) return null;
  const val = traceCache.get(key);
  traceCache.delete(key);
  traceCache.set(key, val);
  return val;
}
function setCachedTrace(key, val) {
  if (traceCache.size >= 32) {
    const oldest = traceCache.keys().next().value;
    traceCache.delete(oldest);
  }
  traceCache.set(key, val);
}

// In-memory static asset cache with mtime validation + precompressed Brotli/Gzip buffers
const staticAssetCache = new Map();

function serveStaticFile(req, res, safePath, stats) {
  const ext = path.extname(safePath).toLowerCase();
  const mime = MIME_TYPES[ext] || 'application/octet-stream';
  const mtimeMs = stats.mtimeMs;
  const size = stats.size;
  const etag = `W/"${size.toString(16)}-${Math.floor(mtimeMs).toString(16)}"`;

  if (req.headers['if-none-match'] === etag) {
    res.writeHead(304, { ETag: etag });
    res.end();
    return;
  }

  const isVendor = safePath.includes(`${path.sep}vendor${path.sep}`);
  const cacheControl = isVendor
    ? 'public, max-age=31536000, immutable'
    : 'public, max-age=0, must-revalidate';

  let cached = staticAssetCache.get(safePath);
  if (!cached || cached.mtimeMs !== mtimeMs || cached.size !== size) {
    const raw = fs.readFileSync(safePath);
    const shouldCompress = COMPRESSIBLE_EXTS.has(ext) && raw.length > 512;
    cached = {
      mtimeMs,
      size,
      etag,
      raw,
      br: shouldCompress
        ? zlib.brotliCompressSync(raw, {
            params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 5 },
          })
        : null,
      gzip: shouldCompress ? zlib.gzipSync(raw, { level: 6 }) : null,
    };
    staticAssetCache.set(safePath, cached);
  }

  const acceptEncoding = String(req.headers['accept-encoding'] || '');
  const headers = {
    'Content-Type': mime,
    ETag: etag,
    'Cache-Control': cacheControl,
    Vary: 'Accept-Encoding',
    'Accept-Ranges': 'bytes',
  };

  const rangeHeader = req.headers.range;
  if (rangeHeader && (ext === '.mp3' || ext === '.wav')) {
    const match = /bytes=(\d*)-(\d*)/.exec(rangeHeader);
    if (match) {
      const total = cached.raw.length;
      const start = match[1] ? parseInt(match[1], 10) : 0;
      const end = match[2] ? Math.min(total - 1, parseInt(match[2], 10)) : total - 1;
      if (start >= 0 && start <= end && end < total) {
        const chunk = cached.raw.subarray(start, end + 1);
        res.writeHead(206, {
          ...headers,
          'Content-Range': `bytes ${start}-${end}/${total}`,
          'Content-Length': chunk.length,
        });
        res.end(chunk);
        return;
      }
    }
  }

  if (cached.br && /\bbr\b/.test(acceptEncoding)) {
    headers['Content-Encoding'] = 'br';
    headers['Content-Length'] = cached.br.length;
    res.writeHead(200, headers);
    res.end(cached.br);
    return;
  }
  if (cached.gzip && /\bgzip\b/.test(acceptEncoding)) {
    headers['Content-Encoding'] = 'gzip';
    headers['Content-Length'] = cached.gzip.length;
    res.writeHead(200, headers);
    res.end(cached.gzip);
    return;
  }

  headers['Content-Length'] = cached.raw.length;
  res.writeHead(200, headers);
  res.end(cached.raw);
}

let bsMatrixRunning = false;
let bsMatrixLogs = [];

function startBrowserStackMatrixBackground() {
  if (bsMatrixRunning) return;
  bsMatrixRunning = true;
  bsMatrixLogs = [`[${new Date().toISOString()}] Launching 13-platform BrowserStack matrix...`];
  const scriptPath = path.join(__dirname, 'test', 'browserstack_mobile_matrix.mjs');
  const child = spawn(process.execPath, [scriptPath], { cwd: __dirname });
  child.stdout.on('data', (chunk) => {
    const lines = chunk.toString('utf8').split('\n').filter(Boolean);
    bsMatrixLogs.push(...lines);
  });
  child.stderr.on('data', (chunk) => {
    const lines = chunk.toString('utf8').split('\n').filter(Boolean);
    bsMatrixLogs.push(...lines);
  });
  child.on('close', (code) => {
    bsMatrixRunning = false;
    bsMatrixLogs.push(`[${new Date().toISOString()}] Matrix completed (exit code ${code}).`);
  });
}

function handleRequest(req, res, defaultToBs = false) {
  const url = new URL(req.url || '/', `http://${req.headers.host || 'localhost'}`);

  if (req.method === 'GET' && url.pathname === '/api/browserstack/status') {
    const cfg = readBsConfig();
    if (!cfg || (!cfg.username && !cfg.accessKey)) {
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({ configured: false }));
      return;
    }
    const keyStr = cfg.accessKey || '';
    const maskedKey =
      keyStr.length > 6 ? `${keyStr.slice(0, 3)}***${keyStr.slice(-3)}` : '***';
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(
      JSON.stringify({
        configured: Boolean(cfg.username && cfg.accessKey),
        hasAccessKey: Boolean(cfg.accessKey),
        username: cfg.username || '',
        accessKey: cfg.accessKey || '',
        maskedKey,
        targetUrl:
          cfg.targetUrl || 'https://charleshoskinson.github.io/Moriarty/visualization/index.html',
        verified: Boolean(cfg.verified),
        plan: cfg.plan || null,
        verifyNote: cfg.verifyNote || null,
        matrixRunning: bsMatrixRunning,
        matrixLogs: bsMatrixLogs.slice(-30),
      })
    );
    return;
  }

  if (req.method === 'POST' && url.pathname === '/api/browserstack/config') {
    const chunks = [];
    req.on('data', (c) => chunks.push(c));
    req.on('end', async () => {
      try {
        const prev = readBsConfig() || {};
        const body = JSON.parse(Buffer.concat(chunks).toString('utf8') || '{}');
        const clean = (s) =>
          String(s || '')
            .replace(/https?:\/\/127\.0\.0\.1(?::\d+)?\/?/gi, '')
            .replace(/^\/\/127\.0/, '')
            .replace(/\.0\.1$/, '')
            .trim();
        let username = clean(body.username);
        let accessKey = clean(body.accessKey);
        const targetUrl = String(
          body.targetUrl ||
            prev.targetUrl ||
            'https://charleshoskinson.github.io/Moriarty/visualization/index.html'
        ).trim();
        if (username.includes(':') && !accessKey) {
          const parts = username.split(':');
          username = clean(parts[0]);
          accessKey = clean(parts.slice(1).join(':'));
        }
        if (!accessKey && prev.accessKey) {
          accessKey = prev.accessKey;
        }
        if (!username && prev.username && prev.username !== 'http') {
          username = prev.username;
        }
        const check =
          username && accessKey
            ? await verifyBrowserStackPlan(username, accessKey)
            : { verified: false, verifyNote: 'Need both BrowserStack Username and Access Key' };
        const record = {
          username,
          accessKey,
          targetUrl,
          verified: check.verified,
          plan: check.plan || null,
          verifyNote: check.verifyNote || null,
          updatedAt: new Date().toISOString(),
        };
        fs.writeFileSync(BS_CONFIG_PATH, JSON.stringify(record, null, 2), { mode: 0o600 });
        if (check.verified) {
          startBrowserStackMatrixBackground();
        }
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(
          JSON.stringify({
            ok: true,
            ...check,
            matrixRunning: bsMatrixRunning,
            matrixLogs: bsMatrixLogs.slice(-30),
          })
        );
      } catch (e) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ ok: false, error: String(e) }));
      }
    });
    return;
  }

  if (req.method === 'POST' && url.pathname.endsWith('/api/trace')) {
    const chunks = [];
    let totalBytes = 0;
    let bodyTooLarge = false;
    req.on('data', (chunk) => {
      if (bodyTooLarge) return;
      totalBytes += chunk.length;
      if (totalBytes > 256 * 1024) {
        bodyTooLarge = true;
        res.writeHead(413, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ ok: false, error: 'Payload Too Large' }));
        req.destroy();
        return;
      }
      chunks.push(chunk);
    });
    req.on('end', () => {
      if (bodyTooLarge) return;
      try {
        const body = Buffer.concat(chunks, totalBytes).toString('utf8');
        const payload = JSON.parse(body || '{}');
        const source = String(payload.source || '');
        const fuel = Math.max(1, Math.min(2000, Number(payload.fuel || 200)));
        const stackLimit = Math.max(1, Math.min(64, Number(payload.stackLimit || 16)));
        const sliceWidth = Math.max(1, Math.min(32, Number(payload.sliceWidth || 4)));

        const traceKey = `${source}|F:${fuel}|S:${stackLimit}|W:${sliceWidth}`;
        const cachedStdout = getCachedTrace(traceKey);
        if (cachedStdout !== null) {
          res.writeHead(200, {
            'Content-Type': 'application/json; charset=utf-8',
            'X-Trace-Cache': 'HIT',
          });
          res.end(cachedStdout);
          return;
        }

        const child = execFile(
          RUST_BIN,
          ['--fuel', String(fuel), '--stack-limit', String(stackLimit), '--slice-width', String(sliceWidth)],
          { maxBuffer: 16 * 1024 * 1024, timeout: 4000 },
          (err, stdout) => {
            if (err || !stdout) {
              res.writeHead(500, { 'Content-Type': 'application/json' });
              res.end(JSON.stringify({ ok: false, error: err ? err.message : 'Empty runner output' }));
              return;
            }
            setCachedTrace(traceKey, stdout);
            res.writeHead(200, {
              'Content-Type': 'application/json; charset=utf-8',
              'X-Trace-Cache': 'MISS',
            });
            res.end(stdout);
          }
        );
        child.stdin?.on('error', () => {});
        child.stdin?.write(source);
        child.stdin?.end();
      } catch (e) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ ok: false, error: String(e) }));
      }
    });
    return;
  }

  let reqPath;
  try {
    reqPath = decodeURIComponent(url.pathname);
  } catch (_) {
    res.writeHead(400);
    res.end('Bad Request');
    return;
  }
  if (reqPath === '/quinelings') {
    res.writeHead(302, { Location: `/quinelings/${url.search || ''}` });
    res.end();
    return;
  }
  if (reqPath.startsWith('/quinelings/')) {
    const quinelingsRoot = '/home/charles/repos/quinelings';
    const subPath = reqPath === '/quinelings/' ? '/index.html' : reqPath.slice('/quinelings'.length);
    const qSafePath = path.resolve(quinelingsRoot, `.${subPath}`);
    if (qSafePath !== quinelingsRoot && !qSafePath.startsWith(quinelingsRoot + path.sep)) {
      res.writeHead(403);
      res.end('Forbidden');
      return;
    }
    fs.stat(qSafePath, (err, stats) => {
      if (err || !stats.isFile()) {
        res.writeHead(404);
        res.end('Not Found');
        return;
      }
      try {
        serveStaticFile(req, res, qSafePath, stats);
      } catch (_) {
        res.writeHead(500);
        res.end('Internal Server Error');
      }
    });
    return;
  }

  if (reqPath === '/browserstack' || reqPath === '/browserstack/') {
    reqPath = '/browserstack.html';
  } else if (reqPath === '/') {
    reqPath = defaultToBs ? '/browserstack.html' : '/index.html';
  }

  const safePath = path.resolve(__dirname, `.${reqPath}`);
  if (safePath !== __dirname && !safePath.startsWith(__dirname + path.sep)) {
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
    try {
      serveStaticFile(req, res, safePath, stats);
    } catch (_) {
      res.writeHead(500);
      res.end('Internal Server Error');
    }
  });
}

const server = http.createServer((req, res) => handleRequest(req, res, false));
server.listen(PORT, '127.0.0.1', () => {
  console.log(`Sherlock Workbench running at http://127.0.0.1:${PORT}`);
});

const bsServer = http.createServer((req, res) => handleRequest(req, res, true));
bsServer.on('error', () => {});
bsServer.listen(BS_PORT, '127.0.0.1', () => {
  console.log(`BrowserStack Lab Miniapp running at http://127.0.0.1:${BS_PORT}`);
});
