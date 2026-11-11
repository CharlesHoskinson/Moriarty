"""Headless check of the Sherlock chip page: screenshots, console errors, animation state.

Serve visualization/ over HTTP first (the import map does not load from file://), then:
    python check_leaf.py http://127.0.0.1:8817 <out_dir> [tag]
"""
import json
import sys
from pathlib import Path

from playwright.sync_api import sync_playwright

base = sys.argv[1].rstrip("/")
out = Path(sys.argv[2])
tag = sys.argv[3] if len(sys.argv) > 3 else "run"
out.mkdir(parents=True, exist_ok=True)

STATE_JS = """() => {
  const v = window.__leafViews;
  if (!v) return null;
  const s = v.leaf.state;
  return { p: s.p, t: +s.t.toFixed(3), playing: s.playing, layout: s.layoutKey,
           view: document.documentElement.dataset.stageView,
           cap: document.querySelector('[data-leaf-cap-title]').textContent,
           docW: document.documentElement.scrollWidth, winW: window.innerWidth,
           wide: Array.from(document.querySelectorAll('body *')).filter(e => {
             const r = e.getBoundingClientRect(); return r.width > 0 && r.right > window.innerWidth + 1;
           }).slice(0, 4).map(e => (e.className && e.className.baseVal === undefined ? e.className : e.tagName) + ':' + Math.round(e.getBoundingClientRect().right)) };
}"""

report = {}
with sync_playwright() as p:
    browser = p.chromium.launch(args=["--enable-unsafe-swiftshader", "--use-angle=swiftshader"])
    for name, vp, reduce in [
        ("desktop", {"width": 1440, "height": 900}, False),
        ("tablet", {"width": 1100, "height": 900}, False),
        ("phone", {"width": 390, "height": 844}, False),
        ("phone-reduced", {"width": 390, "height": 844}, True),
    ]:
        ctx = browser.new_context(viewport=vp, device_scale_factor=2 if "phone" in name else 1,
                                  reduced_motion="reduce" if reduce else "no-preference")
        page = ctx.new_page()
        logs = []
        page.on("console", lambda m, logs=logs: logs.append(f"{m.type}: {m.text}") if m.type in ("error", "warning") else None)
        page.on("pageerror", lambda e, logs=logs: logs.append(f"pageerror: {e}"))
        page.goto(f"{base}/index.html", wait_until="load")
        page.wait_for_timeout(1500)
        page.locator("#pipelineStrip").screenshot(path=str(out / f"{tag}-{name}-pipeline.png"))
        card = page.locator(".stage-viewport-card")
        card.scroll_into_view_if_needed()
        # The loop pauses while the card is off screen, so sample after scrolling to it.
        page.wait_for_timeout(500)
        s1 = page.evaluate(STATE_JS)
        page.wait_for_timeout(2500)
        s2 = page.evaluate(STATE_JS)
        card.screenshot(path=str(out / f"{tag}-{name}-leaf.png"))
        prose = page.locator(".leaf-prose")
        prose.scroll_into_view_if_needed()
        prose.screenshot(path=str(out / f"{tag}-{name}-leaf-prose.png"))
        card.scroll_into_view_if_needed()
        page.click("[data-leaf-end]")
        page.wait_for_timeout(1200 if not reduce else 200)
        card.screenshot(path=str(out / f"{tag}-{name}-leaf-end.png"))
        # Step (pauses) through records, walk and digest to the end of the fold.
        for _ in range(4):
            page.click("[data-leaf-step]")
        page.wait_for_timeout(300)
        s3 = page.evaluate(STATE_JS)
        card.screenshot(path=str(out / f"{tag}-{name}-leaf-fold.png"))
        page.screenshot(path=str(out / f"{tag}-{name}-page.png"), full_page=False)
        page.click("#stageTabMachine")
        page.wait_for_timeout(2500)
        card.screenshot(path=str(out / f"{tag}-{name}-machine.png"))
        s4 = page.evaluate(STATE_JS)
        page.click("#stageTabLeaf")
        page.wait_for_timeout(300)
        report[name] = {"s1": s1, "s2": s2, "s3": s3, "s4": s4, "console": logs}
        ctx.close()
    ctx = browser.new_context(viewport={"width": 1440, "height": 900})
    page = ctx.new_page()
    logs = []
    page.on("console", lambda m: logs.append(f"{m.type}: {m.text}") if m.type == "error" else None)
    page.on("pageerror", lambda e: logs.append(f"pageerror: {e}"))
    page.goto(f"{base}/motion.html", wait_until="load")
    page.wait_for_timeout(3000)
    page.screenshot(path=str(out / f"{tag}-motion-desktop.png"))
    report["motion"] = {"console": logs}
    ctx.close()
    browser.close()

print(json.dumps(report, indent=1))
