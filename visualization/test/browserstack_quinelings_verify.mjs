#!/usr/bin/env node
// ============================================================================
// test/browserstack_quinelings_verify.mjs
// Unified Local Playwright + BrowserStack Automate Cross-Browser Verification
// for Quinelings 2.0 & 3D Sherlock Stage:
//   1. /motion.html    — 12-Scene 90.000s Storyboard Film ("What Is Life?")
//   2. /slideshow.html — 12-Plate Interactive Mathematical Organisms Gallery
//   3. /index.html     — 3D Sherlock Processor Stage (Designer 8 WovenBody +
//                        Designer 2 6-Ring Well, Zero Three.js UV Warnings)
// ============================================================================
import fs from 'node:fs';
import path from 'node:path';
import https from 'node:https';
import http from 'node:http';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const ROOT_DIR = path.resolve(__dirname, '..');
const BS_CONFIG_PATH = path.join(ROOT_DIR, '.browserstack.json');

export const DEFAULT_LOCAL_BASE_URL =
  process.env.LOCAL_BASE_URL || 'http://localhost:4377';

export const BROWSERSTACK_QUINELINGS_MATRIX = [
  {
    id: 'macos_sonoma_chrome',
    label: 'macOS Sonoma · Chrome latest (1920x1080)',
    browserName: 'chrome',
    browserVersion: 'latest',
    bstackOptions: {
      os: 'OS X',
      osVersion: 'Sonoma',
      resolution: '1920x1080',
      projectName: 'Quinelings 2.0 & Sherlock 3D Stage',
      buildName: 'Quinelings 3-Page Cross-Browser Verification',
      sessionName: 'macOS Sonoma Chrome - motion + slideshow + index',
    },
  },
  {
    id: 'win11_chrome_1080p',
    label: 'Windows 11 · Chrome latest (1920x1080)',
    browserName: 'chrome',
    browserVersion: 'latest',
    bstackOptions: {
      os: 'Windows',
      osVersion: '11',
      resolution: '1920x1080',
      projectName: 'Quinelings 2.0 & Sherlock 3D Stage',
      buildName: 'Quinelings 3-Page Cross-Browser Verification',
      sessionName: 'Win11 Chrome 1080p - motion + slideshow + index',
    },
  },
  {
    id: 'win11_edge_1080p',
    label: 'Windows 11 · Edge latest (1920x1080)',
    browserName: 'edge',
    browserVersion: 'latest',
    bstackOptions: {
      os: 'Windows',
      osVersion: '11',
      resolution: '1920x1080',
      projectName: 'Quinelings 2.0 & Sherlock 3D Stage',
      buildName: 'Quinelings 3-Page Cross-Browser Verification',
      sessionName: 'Win11 Edge 1080p - motion + slideshow + index',
    },
  },
];

export const VERIFY_MOTION_SCRIPT = `
  const done = arguments[arguments.length - 1];
  (async () => {
    const errors = window.__capturedErrors || [];
    const warnings = window.__capturedWarnings || [];
    const canvas = document.getElementById('motionCanvas');
    if (!canvas) {
      return done({ ok: false, page: 'motion.html', error: 'Missing #motionCanvas' });
    }

    const t0 = performance.now();
    while ((canvas.width <= 300 && canvas.height <= 150) && performance.now() - t0 < 3000) {
      await new Promise((r) => setTimeout(r, 50));
    }

    const scrubber = document.getElementById('frameScrubber');
    const initialFrame = Number(scrubber?.value || 0);

    const frameDeltas = [];
    let lastRaf = performance.now();
    await new Promise((resolve) => {
      function step(now) {
        const dt = now - lastRaf;
        lastRaf = now;
        if (dt > 0 && dt < 500) frameDeltas.push(dt);
        if (frameDeltas.length < 45) requestAnimationFrame(step);
        else resolve();
      }
      requestAnimationFrame((ts) => {
        lastRaf = ts;
        requestAnimationFrame(step);
      });
    });

    const afterFrame = Number(scrubber?.value || 0);
    const frameAdvanced = afterFrame !== initialFrame;

    if (scrubber) {
      scrubber.value = '2100';
      scrubber.dispatchEvent(new Event('input', { bubbles: true }));
    }
    await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));

    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;
    let brightPixels = 0;
    let nonVoidPixels = 0;
    if (ctx && w > 0 && h > 0) {
      const sampleW = Math.min(w, 320);
      const sampleH = Math.min(h, 320);
      const sx = Math.max(0, Math.floor((w - sampleW) * 0.5));
      const sy = Math.max(0, Math.floor((h - sampleH) * 0.5));
      const img = ctx.getImageData(sx, sy, sampleW, sampleH).data;
      for (let i = 0; i < img.length; i += 16) {
        const r = img[i], g = img[i + 1], b = img[i + 2];
        if (r > 20 || g > 25 || b > 30) nonVoidPixels += 1;
        if (r > 60 || g > 90 || b > 100) brightPixels += 1;
      }
    }

    const avgFrameMs = frameDeltas.reduce((s, x) => s + x, 0) / Math.max(1, frameDeltas.length);
    const avgFps = 1000 / Math.max(0.001, avgFrameMs);
    const sceneBlocksCount = document.querySelectorAll('#sceneStrip .scene-block').length;

    const ok =
      errors.length === 0 &&
      w > 300 &&
      h > 150 &&
      brightPixels > 25 &&
      sceneBlocksCount >= 8 &&
      frameAdvanced;

    done({
      ok,
      page: 'motion.html',
      canvasSize: \`\${w}x\${h}\`,
      sceneBlocksCount,
      frameAdvanced,
      brightPixels,
      nonVoidPixels,
      avgFps: Number(avgFps.toFixed(1)),
      avgFrameMs: Number(avgFrameMs.toFixed(2)),
      errors,
      warnings,
    });
  })().catch((err) => done({ ok: false, page: 'motion.html', error: String(err && err.stack ? err.stack : err) }));
`;

export const VERIFY_SLIDESHOW_SCRIPT = `
  const done = arguments[arguments.length - 1];
  (async () => {
    const errors = window.__capturedErrors || [];
    const warnings = window.__capturedWarnings || [];
    const canvas = document.getElementById('stageCanvas');
    if (!canvas) {
      return done({ ok: false, page: 'slideshow.html', error: 'Missing #stageCanvas' });
    }

    const t0 = performance.now();
    while ((canvas.width <= 300 && canvas.height <= 150) && performance.now() - t0 < 3000) {
      await new Promise((r) => setTimeout(r, 50));
    }

    const dots = Array.from(document.querySelectorAll('#slideDots .slide-dot'));
    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;

    let platesWithBrightPixels = 0;
    for (let idx = 0; idx < dots.length; idx += 1) {
      dots[idx].click();
      await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
      if (ctx && w > 0 && h > 0) {
        const sampleW = Math.min(w, 280);
        const sampleH = Math.min(h, 280);
        const sx = Math.max(0, Math.floor((w - sampleW) * 0.5));
        const sy = Math.max(0, Math.floor((h - sampleH) * 0.5));
        const img = ctx.getImageData(sx, sy, sampleW, sampleH).data;
        let bright = 0;
        for (let i = 0; i < img.length; i += 16) {
          if (img[i] > 55 || img[i + 1] > 80 || img[i + 2] > 90) bright += 1;
        }
        if (bright > 15) platesWithBrightPixels += 1;
      }
    }

    if (dots[0]) dots[0].click();
    const frameDeltas = [];
    let lastRaf = performance.now();
    await new Promise((resolve) => {
      function step(now) {
        const dt = now - lastRaf;
        lastRaf = now;
        if (dt > 0 && dt < 500) frameDeltas.push(dt);
        if (frameDeltas.length < 45) requestAnimationFrame(step);
        else resolve();
      }
      requestAnimationFrame((ts) => {
        lastRaf = ts;
        requestAnimationFrame(step);
      });
    });

    const avgFrameMs = frameDeltas.reduce((s, x) => s + x, 0) / Math.max(1, frameDeltas.length);
    const avgFps = 1000 / Math.max(0.001, avgFrameMs);

    const ok =
      errors.length === 0 &&
      w > 300 &&
      h > 150 &&
      dots.length === 12 &&
      platesWithBrightPixels === 12;

    done({
      ok,
      page: 'slideshow.html',
      canvasSize: \`\${w}x\${h}\`,
      slideCount: dots.length,
      platesWithBrightPixels,
      avgFps: Number(avgFps.toFixed(1)),
      avgFrameMs: Number(avgFrameMs.toFixed(2)),
      errors,
      warnings,
    });
  })().catch((err) => done({ ok: false, page: 'slideshow.html', error: String(err && err.stack ? err.stack : err) }));
`;

export const VERIFY_INDEX_STAGE_SCRIPT = `
  const done = arguments[arguments.length - 1];
  (async () => {
    const errors = window.__capturedErrors || [];
    const warnings = window.__capturedWarnings || [];
    const tStart = performance.now();
    while (
      (!window.__sherlockApp || !window.__sherlockApp.stage || !window.__sherlockApp.stage.ready) &&
      performance.now() - tStart < 22000
    ) {
      await new Promise((r) => setTimeout(r, 100));
    }

    const app = window.__sherlockApp;
    if (!app || !app.stage || !app.stage.ready) {
      return done({
        ok: false,
        page: 'index.html',
        error: 'window.__sherlockApp.stage.ready did not complete within 22s',
        errors,
        warnings,
      });
    }

    const stage = app.stage;
    const state = app.state;
    const canvas = stage.canvas || document.getElementById('sherlockCanvas');
    const qSub = stage.subsystems?.quineling;

    if (stage.setCameraPreset) stage.setCameraPreset('quineling');
    if (state) {
      state.playing = false;
      state.speed = 1.0;
    }

    const tissueGeom = qSub?.tissuePointsMesh?.geometry;
    const ribbonGeom = qSub?.sdkManifoldRibbonMesh?.geometry;
    const tissuePointsCount = tissueGeom?.getAttribute('position')?.count || 0;
    const tissueHasUvAttribute = Boolean(tissueGeom?.getAttribute('uv'));
    const ribbonsHaveUvAttribute = Boolean(ribbonGeom?.getAttribute('uv'));

    let tendrilsHaveUvAttribute = false;
    qSub?.coreRoot?.traverse?.((obj) => {
      if (obj.name === 'Quineling_AtriumTendrils') {
        tendrilsHaveUvAttribute = Boolean(obj.geometry?.getAttribute('uv'));
      }
    });

    const EXPECTED_PRESETS = [
      { id: 'factorial', mechanism: 'logarithmic-mantle' },
      { id: 'list_fold', mechanism: 'pleated-braid' },
      { id: 'tree_fold', mechanism: 'recursive-affine' },
      { id: 'mutual_even_odd', mechanism: 'lorenz-flow' },
      { id: 'lexical_let', mechanism: 'phyllotaxis-fan' },
      { id: 'checked_underflow', mechanism: 'clifford-flow' },
      { id: 'stack_overflow', mechanism: 'recursive-julia' },
      { id: 'fuel_exhaustion', mechanism: 'toroidal-weave' },
    ];

    let presetsVerified = 0;
    for (const spec of EXPECTED_PRESETS) {
      const btn = document.querySelector(\`#presetStrip button[data-preset-id="\${spec.id}"]\`);
      if (btn) btn.click();
      const tPreset = performance.now();
      while (performance.now() - tPreset < 2500) {
        const qdlText = document.getElementById('telemetryQdlMorph')?.textContent || '';
        if (qdlText.includes(spec.mechanism)) {
          presetsVerified += 1;
          break;
        }
        await new Promise((r) => setTimeout(r, 25));
      }
    }

    if (state) {
      state.playing = true;
    }

    const frameDeltas = [];
    let lastRaf = performance.now();
    await new Promise((resolve) => {
      function step(now) {
        const dt = now - lastRaf;
        lastRaf = now;
        if (dt > 0 && dt < 500) frameDeltas.push(dt);
        if (frameDeltas.length < 45) requestAnimationFrame(step);
        else resolve();
      }
      requestAnimationFrame((ts) => {
        lastRaf = ts;
        requestAnimationFrame(step);
      });
    });

    const avgFrameMs = frameDeltas.reduce((s, x) => s + x, 0) / Math.max(1, frameDeltas.length);
    const avgFps = 1000 / Math.max(0.001, avgFrameMs);
    const uvWarnings = warnings.filter((w) => /AttributeNode.*uv.*not found/i.test(w));

    const ok =
      errors.length === 0 &&
      uvWarnings.length === 0 &&
      tissuePointsCount >= 6144 &&
      presetsVerified >= 7 &&
      canvas &&
      canvas.width > 300 &&
      canvas.height > 150;

    done({
      ok,
      page: 'index.html',
      backend: stage.renderer?.backend?.constructor?.name || 'unknown',
      canvasSize: canvas ? \`\${canvas.width}x\${canvas.height}\` : 'missing',
      tissuePointsCount,
      tissueHasUvAttribute,
      ribbonsHaveUvAttribute,
      tendrilsHaveUvAttribute,
      presetsVerified,
      avgFps: Number(avgFps.toFixed(1)),
      avgFrameMs: Number(avgFrameMs.toFixed(2)),
      uvWarningsCount: uvWarnings.length,
      uvWarnings,
      errors,
      warningsCount: warnings.length,
    });
  })().catch((err) => done({ ok: false, page: 'index.html', error: String(err && err.stack ? err.stack : err) }));
`;

export function loadBrowserStackCredentials() {
  let username = process.env.BROWSERSTACK_USERNAME || '';
  let accessKey = process.env.BROWSERSTACK_ACCESS_KEY || '';
  let targetUrl = process.env.TARGET_URL || '';

  if (fs.existsSync(BS_CONFIG_PATH)) {
    const raw = JSON.parse(fs.readFileSync(BS_CONFIG_PATH, 'utf8'));
    username = username || raw.username || '';
    accessKey = accessKey || raw.accessKey || '';
    targetUrl = targetUrl || raw.targetUrl || '';
  }

  const baseUrl = targetUrl
    ? targetUrl.replace(/\/(index|motion|slideshow)\.html(\?.*)?$/i, '')
    : DEFAULT_LOCAL_BASE_URL;

  return { username, accessKey, baseUrl };
}

function checkLocalServer(baseUrl) {
  return new Promise((resolve) => {
    const req = http.get(baseUrl, { timeout: 2000 }, (res) => {
      res.resume();
      resolve(res.statusCode >= 200 && res.statusCode < 500);
    });
    req.on('error', () => resolve(false));
    req.on('timeout', () => {
      req.destroy();
      resolve(false);
    });
  });
}

async function ensureLocalServer(baseUrl) {
  if (await checkLocalServer(baseUrl)) return null;
  const urlObj = new URL(baseUrl);
  const port = urlObj.port || '4377';
  const child = spawn(process.execPath, [path.join(ROOT_DIR, 'server.js')], {
    cwd: ROOT_DIR,
    env: { ...process.env, PORT: String(port) },
    stdio: 'ignore',
  });
  for (let i = 0; i < 40; i += 1) {
    if (await checkLocalServer(baseUrl)) return child;
    await new Promise((r) => setTimeout(r, 150));
  }
  child.kill();
  throw new Error(`Failed to start local server.js on ${baseUrl}`);
}

export async function runLocalPlaywrightVerification(baseUrl = DEFAULT_LOCAL_BASE_URL) {
  const serverProc = await ensureLocalServer(baseUrl);
  let browser = null;
  try {
    const pwPath = process.env.PLAYWRIGHT_MODULE || '/home/charles/.npm/_npx/9833c18b2d85bc59/node_modules/playwright/index.mjs';
    const { chromium } = await import(pwPath);
    browser = await chromium.launch({ headless: true });
    const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });

    const pageSpecs = [
      { path: '/motion.html', script: VERIFY_MOTION_SCRIPT },
      { path: '/slideshow.html', script: VERIFY_SLIDESHOW_SCRIPT },
      { path: '/index.html', script: VERIFY_INDEX_STAGE_SCRIPT },
    ];

    const pageResults = [];
    for (const spec of pageSpecs) {
      const page = await context.newPage();
      const capturedErrors = [];
      const capturedWarnings = [];

      await page.addInitScript(() => {
        window.__capturedErrors = [];
        window.__capturedWarnings = [];
        const origWarn = console.warn.bind(console);
        console.warn = (...args) => {
          window.__capturedWarnings.push(args.map(String).join(' '));
          origWarn(...args);
        };
        const origErr = console.error.bind(console);
        console.error = (...args) => {
          window.__capturedErrors.push(args.map(String).join(' '));
          origErr(...args);
        };
        window.addEventListener('error', (e) => {
          window.__capturedErrors.push(String(e.message || e.error || e));
        });
        window.addEventListener('unhandledrejection', (e) => {
          window.__capturedErrors.push(String(e.reason || e));
        });
      });

      page.on('pageerror', (err) => capturedErrors.push(String(err?.message || err)));
      page.on('console', (msg) => {
        if (msg.type() === 'error') capturedErrors.push(msg.text());
        if (msg.type() === 'warning') capturedWarnings.push(msg.text());
      });

      const fullUrl = `${baseUrl.replace(/\/$/, '')}${spec.path}`;
      await page.goto(fullUrl, { waitUntil: 'domcontentloaded', timeout: 30000 });

      const res = await page.evaluate(
        new Function('return new Promise((resolve) => { const arguments = [resolve]; ' + spec.script + ' });')
      );
      res.playwrightErrors = capturedErrors;
      res.playwrightWarnings = capturedWarnings;
      if (capturedErrors.length > 0) res.ok = false;
      if (spec.path === '/index.html' && capturedWarnings.some((w) => /AttributeNode.*uv.*not found/i.test(w))) {
        res.ok = false;
      }

      pageResults.push(res);
      await page.close();
    }

    return {
      runner: 'local-playwright-chromium',
      baseUrl,
      allPassed: pageResults.every((r) => r.ok),
      pages: pageResults,
    };
  } finally {
    if (browser) await browser.close().catch(() => {});
    if (serverProc) serverProc.kill();
  }
}

function httpsJsonRequest(urlStr, { method = 'GET', auth = '', body = null, timeoutMs = 60000 } = {}) {
  return new Promise((resolve, reject) => {
    const url = new URL(urlStr);
    const payload = body ? JSON.stringify(body) : null;
    const headers = {
      Accept: 'application/json',
      'Content-Type': 'application/json',
    };
    if (auth) {
      headers.Authorization = 'Basic ' + Buffer.from(auth).toString('base64');
    }
    if (payload) {
      headers['Content-Length'] = Buffer.byteLength(payload);
    }

    const req = https.request(
      {
        hostname: url.hostname,
        port: url.port || 443,
        path: url.pathname + url.search,
        method,
        headers,
        timeout: timeoutMs,
      },
      (res) => {
        let data = '';
        res.on('data', (chunk) => {
          data += chunk;
        });
        res.on('end', () => {
          try {
            resolve({ statusCode: res.statusCode, body: data ? JSON.parse(data) : {} });
          } catch (_) {
            resolve({ statusCode: res.statusCode, body: { raw: data } });
          }
        });
      }
    );
    req.on('error', reject);
    req.on('timeout', () => {
      req.destroy(new Error(`Request timeout (${timeoutMs}ms): ${method} ${urlStr}`));
    });
    if (payload) req.write(payload);
    req.end();
  });
}

const CONSOLE_HOOK_PREAMBLE = `
  if (!window.__consoleHooked) {
    window.__consoleHooked = true;
    window.__capturedErrors = window.__capturedErrors || [];
    window.__capturedWarnings = window.__capturedWarnings || [];
    const origWarn = console.warn.bind(console);
    console.warn = function(...args) {
      window.__capturedWarnings.push(args.map(String).join(' '));
      return origWarn.apply(console, args);
    };
    const origErr = console.error.bind(console);
    console.error = function(...args) {
      window.__capturedErrors.push(args.map(String).join(' '));
      return origErr.apply(console, args);
    };
  }
`;

export async function runBrowserStackSession(spec, creds, pagesToTest) {
  const auth = `${creds.username}:${creds.accessKey}`;
  const hubBase = 'https://hub-cloud.browserstack.com/wd/hub';

  const createRes = await httpsJsonRequest(`${hubBase}/session`, {
    method: 'POST',
    auth,
    timeoutMs: 90000,
    body: {
      capabilities: {
        alwaysMatch: {
          browserName: spec.browserName,
          browserVersion: spec.browserVersion || 'latest',
          'bstack:options': spec.bstackOptions,
        },
      },
    },
  });

  const sessionId = createRes.body?.value?.sessionId || createRes.body?.sessionId || null;
  if (!sessionId) {
    throw new Error(`Failed to create BrowserStack session for ${spec.label}: ${JSON.stringify(createRes.body)}`);
  }

  try {
    await httpsJsonRequest(`${hubBase}/session/${sessionId}/timeouts`, {
      method: 'POST',
      auth,
      body: { script: 45000 },
    });

    await httpsJsonRequest(`${hubBase}/session/${sessionId}/window/maximize`, {
      method: 'POST',
      auth,
      body: {},
    }).catch(() => {});

    const pageOutputs = [];
    for (const p of pagesToTest) {
      await httpsJsonRequest(`${hubBase}/session/${sessionId}/url`, {
        method: 'POST',
        auth,
        body: { url: p.url },
      });

      const execRes = await httpsJsonRequest(`${hubBase}/session/${sessionId}/execute/async`, {
        method: 'POST',
        auth,
        timeoutMs: 55000,
        body: { script: CONSOLE_HOOK_PREAMBLE + '\n' + p.script, args: [] },
      });

      pageOutputs.push(execRes.body?.value || { ok: false, page: p.name, error: 'Empty execute/async response' });
    }

    return {
      sessionId,
      id: spec.id,
      browser: spec.label,
      ok: pageOutputs.every((p) => p.ok),
      pages: pageOutputs,
    };
  } finally {
    await httpsJsonRequest(`${hubBase}/session/${sessionId}`, {
      method: 'DELETE',
      auth,
      timeoutMs: 15000,
    }).catch(() => {});
  }
}

export async function main() {
  const args = new Set(process.argv.slice(2));
  const localOnly = args.has('--local');
  const bsOnly = args.has('--browserstack');

  const report = {
    timestamp: new Date().toISOString(),
    localPlaywright: null,
    browserStack: null,
  };

  if (!bsOnly) {
    console.log('▶ Running Local Playwright Verification (motion.html, slideshow.html, index.html)...');
    report.localPlaywright = await runLocalPlaywrightVerification(DEFAULT_LOCAL_BASE_URL);
    for (const p of report.localPlaywright.pages) {
      console.log(
        `  ${p.ok ? '✓' : '✗'} [Local] ${p.page}: ok=${p.ok}, canvas=${p.canvasSize}, avgFps=${p.avgFps}, errors=${p.playwrightErrors?.length || 0}`
      );
    }
  }

  if (!localOnly) {
    const creds = loadBrowserStackCredentials();
    if (creds.username && creds.accessKey) {
      const planRes = await httpsJsonRequest('https://api.browserstack.com/automate/plan.json', {
        auth: `${creds.username}:${creds.accessKey}`,
      });
      if (planRes.statusCode === 200) {
        const maxParallel = Math.max(
          1,
          Math.min(5, Number(planRes.body.parallel_sessions_max_allowed) || 1)
        );
        const base = creds.baseUrl.replace(/\/$/, '');
        const pagesToTest = [
          { name: 'motion.html', url: `${base}/motion.html`, script: VERIFY_MOTION_SCRIPT },
          { name: 'slideshow.html', url: `${base}/slideshow.html`, script: VERIFY_SLIDESHOW_SCRIPT },
          { name: 'index.html', url: `${base}/index.html`, script: VERIFY_INDEX_STAGE_SCRIPT },
        ];

        const queue = [...BROWSERSTACK_QUINELINGS_MATRIX];
        const bsResults = [];
        async function worker() {
          while (queue.length > 0) {
            const spec = queue.shift();
            if (!spec) break;
            try {
              const res = await runBrowserStackSession(spec, creds, pagesToTest);
              bsResults.push(res);
              console.log(`  ${res.ok ? '✓' : '✗'} [BrowserStack] ${spec.label}: ok=${res.ok}`);
            } catch (err) {
              bsResults.push({ id: spec.id, browser: spec.label, ok: false, error: err.message });
              console.error(`  ✗ [BrowserStack] ${spec.label}: ${err.message}`);
            }
          }
        }
        await Promise.all(Array.from({ length: maxParallel }, () => worker()));
        report.browserStack = {
          plan: planRes.body,
          baseUrl: base,
          results: bsResults,
        };
      }
    }
  }

  const outPath = path.join(ROOT_DIR, 'audits', 'perf', 'quinelings_verify_results.json');
  fs.mkdirSync(path.dirname(outPath), { recursive: true });
  fs.writeFileSync(outPath, JSON.stringify(report, null, 2));
  console.log(`\nSaved verification report to ${outPath}`);

  if (report.localPlaywright && !report.localPlaywright.allPassed) {
    process.exitCode = 1;
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}
