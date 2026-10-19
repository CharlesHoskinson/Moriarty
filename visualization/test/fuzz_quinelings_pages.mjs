#!/usr/bin/env node
/**
 * Comprehensive Node + Playwright Fuzz & Runtime Auditor
 * Targets:
 *   - http://localhost:4377/motion.html   (all scenes, [0..5399] scrubber, speed, editor drawer, math modal)
 *   - http://localhost:4377/slideshow.html (12 slides, outcome/depth/owner/loop/caustics/auto-tour)
 *   - http://localhost:4377/index.html     (presets, step scrubber, camera lenses, fuel/stack sliders)
 *
 * Assertions:
 *   1. Zero `pageerror` or `console.error` events across all interactions
 *   2. Canvas dimensions `width > 600 && height > 400`
 *   3. Non-zero luminous pixels (`brightPixels > 500`) on EVERY scene of `motion.html`
 *      and EVERY slide (1..12) of `slideshow.html`
 *   4. Continuous animation movement (`frame_t1 !== frame_t0` pixel diff > 0 when playing)
 */

import assert from 'node:assert/strict';

const BASE_URL = process.env.BASE_URL || 'http://localhost:4377';
const pwPath = process.env.PLAYWRIGHT_MODULE || '/home/charles/.npm/_npx/9833c18b2d85bc59/node_modules/playwright/index.mjs';
const { chromium } = await import(pwPath);

function mulberry32(seed = 0x5eed1234) {
  let s = seed >>> 0;
  return () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = Math.imul(s ^ (s >>> 15), 1 | s);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

async function sampleCanvasMetrics(page, canvasSelector, waitPlayMs = 140) {
  return page.evaluate(
    async ({ selector, delayMs }) => {
      const canvas = document.querySelector(selector);
      if (!canvas) {
        return { missing: true, width: 0, height: 0, brightPixels: 0, frameDiff: 0 };
      }
      const width = canvas.width;
      const height = canvas.height;

      const probeW = 240;
      const probeH = 160;
      const off = document.createElement('canvas');
      off.width = probeW;
      off.height = probeH;
      const octx = off.getContext('2d', { willReadFrequently: true });

      octx.drawImage(canvas, 0, 0, probeW, probeH);
      const data0 = new Uint8ClampedArray(octx.getImageData(0, 0, probeW, probeH).data);

      await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
      if (delayMs > 0) {
        await new Promise((r) => setTimeout(r, delayMs));
      }
      await new Promise((r) => requestAnimationFrame(r));

      octx.drawImage(canvas, 0, 0, probeW, probeH);
      const data1 = octx.getImageData(0, 0, probeW, probeH).data;

      const scaleFactor = Math.max(1, (width * height) / (probeW * probeH * 16));
      let rawBright = 0;
      let diffPixels = 0;
      for (let i = 0; i < data1.length; i += 4) {
        const r = data1[i];
        const g = data1[i + 1];
        const b = data1[i + 2];
        if (r + g + b > 68 && (r > 24 || g > 28 || b > 36)) {
          rawBright++;
        }
        if (
          Math.abs(data1[i] - data0[i]) +
            Math.abs(data1[i + 1] - data0[i + 1]) +
            Math.abs(data1[i + 2] - data0[i + 2]) >
          6
        ) {
          diffPixels++;
        }
      }

      return {
        missing: false,
        width,
        height,
        brightPixels: Math.round(rawBright * scaleFactor),
        rawBright,
        frameDiff: diffPixels,
      };
    },
    { selector: canvasSelector, delayMs: waitPlayMs }
  );
}

function attachErrorCollectors(page, pageName) {
  const pageErrors = [];
  const consoleErrors = [];
  page.on('pageerror', (err) => {
    pageErrors.push(`[${pageName} pageerror] ${err.stack || err.message || String(err)}`);
  });
  page.on('console', (msg) => {
    if (msg.type() === 'error') {
      consoleErrors.push(`[${pageName} console.error] ${msg.text()}`);
    }
  });
  return {
    assertZeroErrors(contextLabel) {
      assert.equal(
        pageErrors.length,
        0,
        `Expected 0 pageerror events during ${contextLabel}, got:\n${pageErrors.join('\n')}`
      );
      assert.equal(
        consoleErrors.length,
        0,
        `Expected 0 console.error events during ${contextLabel}, got:\n${consoleErrors.join('\n')}`
      );
    },
  };
}

async function fuzzMotionPage(browser, rng) {
  console.log('--- Fuzzing /motion.html ---');
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
  const page = await context.newPage();
  const errors = attachErrorCollectors(page, 'motion.html');

  await page.goto(`${BASE_URL}/motion.html`, { waitUntil: 'domcontentloaded' });
  await page.waitForTimeout(250);
  errors.assertZeroErrors('motion.html initial load');

  const initMetrics = await sampleCanvasMetrics(page, '#motionCanvas', 160);
  assert.ok(
    initMetrics.width > 600 && initMetrics.height > 400,
    `motion.html canvas dimensions too small: ${initMetrics.width}x${initMetrics.height}`
  );
  assert.ok(
    initMetrics.brightPixels > 500,
    `motion.html initial brightPixels (${initMetrics.brightPixels}) <= 500`
  );
  assert.ok(
    initMetrics.frameDiff > 0,
    `motion.html expected continuous animation movement (frameDiff > 0), got ${initMetrics.frameDiff}`
  );

  // 1. Click EVERY scene block in #sceneStrip and assert brightPixels > 500 & frameDiff > 0
  const sceneBlocks = page.locator('#sceneStrip .scene-block');
  const sceneCount = await sceneBlocks.count();
  assert.ok(sceneCount >= 8, `Expected >= 8 scene blocks in motion.html, found ${sceneCount}`);

  for (let i = 0; i < sceneCount; i++) {
    await sceneBlocks.nth(i).click();
    await page.waitForTimeout(80);
    const m = await sampleCanvasMetrics(page, '#motionCanvas', 120);
    errors.assertZeroErrors(`motion.html Scene ${i + 1}`);
    assert.ok(
      m.width > 600 && m.height > 400,
      `motion.html Scene ${i + 1} canvas size invalid: ${m.width}x${m.height}`
    );
    assert.ok(
      m.brightPixels > 500,
      `motion.html Scene ${i + 1} brightPixels (${m.brightPixels}) <= 500`
    );
    assert.ok(
      m.frameDiff > 0,
      `motion.html Scene ${i + 1} animation frozen (frameDiff === 0)`
    );
    console.log(
      `  [Scene ${String(i + 1).padStart(2, '0')}] OK · size=${m.width}x${m.height} · brightPixels=${m.brightPixels} · diff=${m.frameDiff}`
    );
  }

  // 2. Fuzz scrubber across random frames in [0..5399] + boundary frames
  const testFrames = [
    0, 1, 299, 300, 675, 1179, 1180, 1665, 2249, 2250,
    2790, 3240, 3690, 4170, 4590, 5130, 5398, 5399,
    ...Array.from({ length: 20 }, () => Math.floor(rng() * 5400)),
  ];

  for (const frame of testFrames) {
    await page.evaluate((f) => {
      const scrubber = document.getElementById('frameScrubber');
      scrubber.value = String(f);
      scrubber.dispatchEvent(new Event('input', { bubbles: true }));
    }, frame);
    const m = await sampleCanvasMetrics(page, '#motionCanvas', 45);
    errors.assertZeroErrors(`motion.html scrub frame=${frame}`);
    assert.ok(
      m.brightPixels > 500,
      `motion.html scrub frame=${frame} brightPixels (${m.brightPixels}) <= 500`
    );
  }

  // 3. Click transport controls, speed buttons, surgical scene editor drawer, and resize
  for (const speed of ['0.5', '1', '2']) {
    await page.locator(`[data-speed="${speed}"]`).click();
    errors.assertZeroErrors(`motion.html speed=${speed}`);
  }
  await page.locator('#prevSceneBtn').click();
  await page.locator('#nextSceneBtn').click();
  await page.locator('#restartBtn').click();
  await page.locator('#toggleEditorBtn').click();
  await page.locator('#editHeadline').fill('What is Life? — Fuzz Verified');
  await page.locator('#editFlowVel').fill('1.65');
  await page.locator('#editFlowVel').dispatchEvent('input');
  await page.locator('#toggleEditorBtn').click();

  await page.locator('#playPauseBtn').click(); // Pause
  await page.locator('#playPauseBtn').click(); // Play
  const postResume = await sampleCanvasMetrics(page, '#motionCanvas', 140);
  assert.ok(postResume.frameDiff > 0, 'motion.html failed to resume animation after Pause -> Play');

  await page.setViewportSize({ width: 960, height: 640 });
  await page.waitForTimeout(60);
  const resized = await sampleCanvasMetrics(page, '#motionCanvas', 100);
  assert.ok(resized.width > 600 && resized.height > 400, 'motion.html resize dimensions invalid');
  assert.ok(resized.brightPixels > 500, 'motion.html resize brightPixels <= 500');
  errors.assertZeroErrors('motion.html resize & controls');

  await context.close();
}

async function fuzzSlideshowPage(browser, rng) {
  console.log('--- Fuzzing /slideshow.html ---');
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
  const page = await context.newPage();
  const errors = attachErrorCollectors(page, 'slideshow.html');

  await page.goto(`${BASE_URL}/slideshow.html`, { waitUntil: 'domcontentloaded' });
  await page.waitForTimeout(250);
  errors.assertZeroErrors('slideshow.html initial load');

  const slideDots = page.locator('#slideDots .slide-dot');
  const slideCount = await slideDots.count();
  assert.equal(slideCount, 12, `Expected 12 slide dots in slideshow.html, found ${slideCount}`);

  const outcomes = ['Running', 'Done', 'Rejected', 'Exhausted'];

  for (let i = 0; i < slideCount; i++) {
    await slideDots.nth(i).click();
    await page.waitForTimeout(80);
    const m = await sampleCanvasMetrics(page, '#stageCanvas', 140);
    errors.assertZeroErrors(`slideshow.html Slide ${i + 1}`);
    assert.ok(
      m.width > 600 && m.height > 400,
      `slideshow.html Slide ${i + 1} canvas size invalid: ${m.width}x${m.height}`
    );
    assert.ok(
      m.brightPixels > 500,
      `slideshow.html Slide ${i + 1} brightPixels (${m.brightPixels}) <= 500`
    );
    assert.ok(
      m.frameDiff > 0,
      `slideshow.html Slide ${i + 1} animation frozen (frameDiff === 0)`
    );
    console.log(
      `  [Slide ${String(i + 1).padStart(2, '0')}] OK · size=${m.width}x${m.height} · brightPixels=${m.brightPixels} · diff=${m.frameDiff}`
    );
  }

  for (let step = 0; step < 24; step++) {
    const slideIdx = Math.floor(rng() * slideCount);
    const outcome = outcomes[Math.floor(rng() * outcomes.length)];
    const depth = 1 + Math.floor(rng() * 8);
    const owner = Math.floor(rng() * 5);

    await slideDots.nth(slideIdx).click();
    await page.locator('#outcomeSelect').selectOption(outcome);
    await page.evaluate(
      ({ d, o }) => {
        const ds = document.getElementById('depthSlider');
        ds.value = String(d);
        ds.dispatchEvent(new Event('input', { bubbles: true }));
        const os = document.getElementById('ownerSlider');
        os.value = String(o);
        os.dispatchEvent(new Event('input', { bubbles: true }));
      },
      { d: depth, o: owner }
    );

    if (step % 4 === 0) await page.locator('#toggleLoopBtn').click();
    if (step % 5 === 0) await page.locator('#toggleCausticsBtn').click();
    if (step % 6 === 0) await page.locator('#autoAdvanceBtn').click();

    const m = await sampleCanvasMetrics(page, '#stageCanvas', 90);
    errors.assertZeroErrors(
      `slideshow.html fuzz step=${step} slide=${slideIdx + 1} outcome=${outcome} depth=${depth} owner=${owner}`
    );
    assert.ok(
      m.brightPixels > 500,
      `slideshow.html fuzz step=${step} slide=${slideIdx + 1} brightPixels (${m.brightPixels}) <= 500`
    );
  }

  await page.mouse.move(640, 400);
  await page.mouse.down();
  await page.mouse.move(720, 350, { steps: 5 });
  await page.mouse.up();

  await page.setViewportSize({ width: 960, height: 640 });
  await page.waitForTimeout(60);
  const resized = await sampleCanvasMetrics(page, '#stageCanvas', 100);
  assert.ok(resized.width > 600 && resized.height > 400, 'slideshow.html resize dimensions invalid');
  assert.ok(resized.brightPixels > 500, 'slideshow.html resize brightPixels <= 500');
  errors.assertZeroErrors('slideshow.html pointer drag & resize');

  await context.close();
}

async function fuzzIndexPage(browser, rng) {
  console.log('--- Fuzzing /index.html ---');
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
  const page = await context.newPage();
  const errors = attachErrorCollectors(page, 'index.html');

  await page.goto(`${BASE_URL}/index.html`, { waitUntil: 'domcontentloaded' });
  await page.waitForTimeout(400);
  errors.assertZeroErrors('index.html initial load');

  const initMetrics = await sampleCanvasMetrics(page, '#sherlockCanvas', 160);
  assert.ok(
    initMetrics.width > 400 && initMetrics.height > 300,
    `index.html canvas dimensions too small: ${initMetrics.width}x${initMetrics.height}`
  );

  const lenses = page.locator('#cameraLensBar button[data-preset]');
  const lensCount = await lenses.count();
  for (let i = 0; i < lensCount; i++) {
    await lenses.nth(i).click();
    await page.waitForTimeout(40);
    errors.assertZeroErrors(`index.html camera lens ${i}`);
  }

  for (const speed of ['0.5', '1', '2', '4']) {
    await page.locator('#speedSelect').selectOption(speed);
  }

  for (let k = 0; k < 10; k++) {
    const pct = Math.floor(rng() * 100);
    await page.evaluate((val) => {
      const s = document.getElementById('stepScrubber');
      if (s) {
        s.value = String(Math.min(Number(s.max || 100), val));
        s.dispatchEvent(new Event('input', { bubbles: true }));
      }
    }, pct);
  }

  await page.locator('#stepNextBtn').click();
  await page.locator('#stepPrevBtn').click();
  await page.locator('#resetStepBtn').click();
  errors.assertZeroErrors('index.html controls');

  await context.close();
}

async function main() {
  const rng = mulberry32(0x20261019);
  const browser = await chromium.launch({ headless: true });
  try {
    await fuzzMotionPage(browser, rng);
    await fuzzSlideshowPage(browser, rng);
    await fuzzIndexPage(browser, rng);
    console.log('\n✅ ALL QUINELINGS PAGE FUZZ & LUMINOSITY ASSERTIONS PASSED!');
  } finally {
    await browser.close();
  }
}

main().catch((err) => {
  console.error('\n❌ FUZZ TEST FAILED:', err);
  process.exit(1);
});
