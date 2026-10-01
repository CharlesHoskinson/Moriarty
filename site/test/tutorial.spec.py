"""Smoke-check the built tutorial, including its project subpath and downloads.

URL is the build directory URL; CHROME may select a binary (empty uses
Playwright Chromium); SHOTS selects captures and retained downloaded files.
This gate does not run the beta CLI, sign, prove or submit transactions.
"""
import hashlib
import json
import os
import re
import sys
import tempfile
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urldefrag, urljoin, urlparse

from playwright.sync_api import expect, sync_playwright

BASE = os.environ.get("URL", "http://localhost:8891/").rstrip("/") + "/"
PAGE = BASE + "tutorial.html"
OUT = Path(os.environ.get("SHOTS", "/tmp/moriarty-site-shots"))
LESSONS = Path(__file__).resolve().parents[1] / "src/tutorial/lessons"
SECTIONS = {"overview", "getting-started", "transfer", "repayment", "testing", "syntax", "patterns", "amm", "lending", "stablecoins", "options", "oracles", "governance", "bridges", "staking", "trust", "architecture"}
DOMAINS = {"amm": "AMM", "lending": "Lending", "stablecoins": "Stablecoin", "options": "Options", "oracles": "Oracle", "governance": "Governance", "bridges": "Bridge", "staking": "Staking"}
results, errors, downloads = [], [], []
OUT.mkdir(parents=True, exist_ok=True)


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def check(name, action):
    try:
        action()
        results.append({"name": name, "passed": True})
        print(f"PASS {name}")
    except Exception as error:
        results.append({"name": name, "passed": False, "error": str(error)})
        print(f"FAIL {name}: {str(error).splitlines()[0]}")


def load(page, fragment=""):
    response = page.goto(PAGE + fragment, wait_until="networkidle")
    if response is None:  # Fragment changes may reuse an existing document.
        response = page.reload(wait_until="networkidle")
    require(response is not None and response.status == 200, "Tutorial must return HTTP 200")
    expect(page.locator("#overview")).to_be_attached()


def active(page, section):
    expect(page.locator(f'#guide-lessons nav[aria-label="Lessons"] a[href="#{section}"]')).to_have_attribute("aria-current", "location")


def overflow(page):
    require(page.evaluate("Math.max(document.body.scrollWidth,document.documentElement.scrollWidth)<=innerWidth+1"), "Page has horizontal overflow")


def clear_anchor(page, ident):
    page.wait_for_function("""id=>{
      const el=document.getElementById(id);if(!el)return false;
      const h=el.matches('section')?(el.querySelector('h1,h2,h3')||el):el;
      const r=h.getBoundingClientRect(),anchor=el.getBoundingClientRect();
      const b=[...document.querySelectorAll('header,.guide-bar')].filter(e=>{
        const s=getComputedStyle(e),r=e.getBoundingClientRect();
        return ['sticky','fixed'].includes(s.position)&&s.display!=='none'&&r.bottom>0&&r.top<innerHeight;
      }).map(e=>e.getBoundingClientRect().bottom);
      const ceiling=Math.max(0,...b),end=scrollY>=document.documentElement.scrollHeight-innerHeight-2;
      let current;
      for(const section of document.querySelectorAll('section.guide-section[id]')){
        const line=Math.max(ceiling+16,(parseFloat(getComputedStyle(section).scrollMarginTop)||0)+16);
        if(section.getBoundingClientRect().top<=line)current=section.id;else break;
      }
      const parent=el.closest('section.guide-section')?.id;
      return r.top>=ceiling-2&&r.top<innerHeight&&anchor.top>=ceiling-2&&anchor.top<innerHeight&&(end||anchor.top<=ceiling+24)&&(end||current===parent);
    }""", arg=ident)


def settled_layout(page):
    # Offscreen lazy images may stay unloaded when explicit dimensions reserve
    # their space. Require actual loading for eager/visible images, then stable
    # document and image geometry; do not force readiness or scroll the target.
    page.wait_for_function("""document.readyState==='complete'&&(!document.fonts||document.fonts.status==='loaded')&&[...document.querySelectorAll('main img')].every(i=>{if(i.complete)return true;const r=i.getBoundingClientRect();return i.loading==='lazy'&&Number(i.getAttribute('width'))>0&&Number(i.getAttribute('height'))>0&&(r.top>=innerHeight||r.bottom<=0);})""")
    page.evaluate("""()=>new Promise(resolve=>{let last='',steady=0;function tick(){const layout=JSON.stringify([document.documentElement.scrollHeight,...[...document.querySelectorAll('main img')].map(i=>{const r=i.getBoundingClientRect();return [r.top+scrollY,r.width,r.height]})]);steady=layout===last?steady+1:0;last=layout;if(steady>=40)resolve();else requestAnimationFrame(tick)}requestAnimationFrame(tick)})""")
    page.wait_for_timeout(1800)  # Outlast the transient 1.5-second place pin.


def fresh_lesson_urls(page):
    for ident in sorted(SECTIONS):
        load(page, "#" + ident)  # Same-document goto is reloaded by load().
        settled_layout(page)
        clear_anchor(page, ident)
        active(page, ident)


def architecture_refreshes(page):
    for ident in ["architecture", "architecture-kernel"]:
        load(page, "#" + ident)
        settled_layout(page)
        clear_anchor(page, ident)
        active(page, "architecture")
        response = page.reload(wait_until="networkidle")
        require(response is not None and response.status == 200, "Architecture refresh must return HTTP 200")
        settled_layout(page)
        clear_anchor(page, ident)
        active(page, "architecture")
    load(page, "#architecture")
    settled_layout(page)
    clear_anchor(page, "architecture")
    active(page, "architecture")
    image = page.locator('#architecture img[src*="pipeline"]')
    expect(image).to_have_count(1)
    require(image.evaluate("""e=>{const r=e.getBoundingClientRect();return e.complete&&e.naturalWidth>0&&Math.min(innerHeight,r.bottom)-Math.max(0,r.top)>=Math.min(80,r.height)&&r.right>0&&r.left<innerWidth}"""), "Pipeline image must be loaded and meaningfully visible")
    page.screenshot(path=str(OUT / f"tutorial-pipeline-{page.viewport_size['width']}.png"))


class Anchors(HTMLParser):
    def __init__(self):
        super().__init__()
        self.ids = set()

    def handle_starttag(self, tag, attrs):
        self.ids.update(value for key, value in attrs if value and (key == "id" or (tag == "a" and key == "name")))


def structure(page):
    ids = page.locator("section.guide-section[id]").evaluate_all("els=>els.map(e=>e.id)")
    require(set(ids) == SECTIONS and len(ids) == len(SECTIONS), f"Unexpected tutorial sections: {ids}")
    expect(page.locator("h1")).to_have_count(1)
    for ident, name in DOMAINS.items():
        link = page.locator(f'#guide-lessons nav[aria-label="Lessons"] a[href="#{ident}"]')
        expect(link).to_have_count(1)
        require(re.search(r"\b" + name, link.inner_text(), re.I), f"Missing {name} navigation label")
    local = page.locator("a[href]").evaluate_all("els=>els.map(e=>e.href)")
    cached = {}
    for href in set(local):
        parsed = urlparse(href)
        if parsed.netloc != urlparse(PAGE).netloc or parsed.scheme != urlparse(PAGE).scheme or not parsed.fragment:
            continue
        ident = unquote(parsed.fragment)
        document, _ = urldefrag(href)
        if document == PAGE:
            require(page.evaluate("id=>!!document.getElementById(id)", ident), f"Broken tutorial fragment: {href}")
        else:
            if document not in cached:
                response = page.request.get(document)
                require(response.status == 200, f"Local destination failed: {document}")
                parser = Anchors()
                parser.feed(response.text())
                cached[document] = parser.ids
            require(ident in cached[document], f"Broken local fragment: {href}")


def reveal(page, link):
    # Disclosures are opened through their real controls, before downloading.
    ancestors = link.evaluate("""e=>{const a=[];for(let p=e.parentElement;p;p=p.parentElement)if(p.tagName==='DETAILS'&&!p.open){if(!p.id)p.id='smoke-detail-'+Math.random().toString(36).slice(2);a.unshift(p.id)}return a;}""")
    for ident in ancestors:
        page.locator(f'#{ident} > summary').click()


def lesson_downloads(page):
    folder = Path(tempfile.mkdtemp(prefix="tutorial-downloads-", dir=OUT))
    for section, prefix, program in [("transfer", "transfer", "invoice.mori"), ("repayment", "repay", "repayment.mori")]:
        target = folder / section
        target.mkdir()
        for filename, source in [(program, prefix + ".mori"), ("scenario.json", prefix + ".scenario.json"), ("mori.tests.json", prefix + ".test.json")]:
            link = page.locator(f'#{section} a[download="{filename}"]')
            expect(link).to_have_count(1)
            reveal(page, link)
            with page.expect_download() as event:
                link.click()
            download = event.value
            require(download.failure() is None, f"Download failed: {section}/{filename}")
            path = target / filename
            download.save_as(str(path))
            data = path.read_bytes()
            require(data == (LESSONS / source).read_bytes(), f"Download differs from source bytes: {section}/{filename}")
            downloads.append({"path": str(path), "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()})


def mobile_navigation(page):
    response = page.goto(urljoin(BASE, "index.html"), wait_until="networkidle")
    require(response is not None and response.status == 200, "Home must return HTTP 200")
    tutorial = page.get_by_role("link", name=re.compile(r"tutorial", re.I)).first
    expect(tutorial).to_be_visible()
    require(tutorial.evaluate("e=>{const r=e.getBoundingClientRect();return r.top>=0&&r.bottom<=innerHeight}"), "Mobile home tutorial entry is offscreen")
    tutorial.click()
    expect(page.locator("#overview")).to_be_attached()
    load(page, "#options")
    page.evaluate("scrollBy(0,600)")
    button = page.get_by_role("button", name=re.compile(r"^Browse\b", re.I))
    expect(button).to_be_visible()
    require(button.evaluate("e=>{const r=e.getBoundingClientRect();return r.top>=0&&r.bottom<=innerHeight}"), "Browse control is offscreen after scrolling")
    button.click()
    expect(page.get_by_role("searchbox")).to_be_focused()
    page.get_by_role("navigation", name="Lessons", exact=True).locator('a[href="#transfer"]').click()
    expect(button).to_have_attribute("aria-expanded", "false")
    clear_anchor(page, "transfer")
    active(page, "transfer")
    title = page.locator('#guide-lessons nav[aria-label="Lessons"] a[href="#transfer"]').inner_text()
    expect(page.locator(".guide-current")).to_have_text(title)
    require(page.evaluate("document.getElementById('transfer').contains(document.activeElement)"), "Selected lesson did not receive focus")


def main():
    with sync_playwright() as pw:
        browser = pw.chromium.launch(executable_path=os.environ.get("CHROME") or None)
        try:
            context = browser.new_context(viewport={"width": 1440, "height": 1000}, accept_downloads=True)
            context.on("page", lambda p: p.on("pageerror", lambda e: errors.append(str(e))))
            context.on("page", lambda p: p.on("console", lambda m: errors.append(m.text) if m.type == "error" else None))
            page = context.new_page()
            load(page)
            check("tutorial sections, domains and local fragments", lambda: structure(page))
            check("desktop containment", lambda: overflow(page))
            page.screenshot(path=str(OUT / "tutorial-desktop.png"))
            def desktop_nav():
                page.get_by_role("navigation", name="Lessons", exact=True).locator('a[href="#transfer"]').click()
                clear_anchor(page, "transfer")
                active(page, "transfer")
                load(page, "#transfer")
                clear_anchor(page, "transfer")
                active(page, "transfer")
            check("desktop navigation and fresh transfer URL", desktop_nav)
            check("desktop all fresh lesson URLs after settled layout", lambda: fresh_lesson_urls(page))
            check("desktop architecture refreshes and visible pipeline", lambda: architecture_refreshes(page))
            check("six real downloads match versioned lesson bytes", lambda: lesson_downloads(page))
            page.set_viewport_size({"width": 390, "height": 1000})
            check("mobile home entry and deep-scroll lesson navigation", lambda: mobile_navigation(page))
            load(page, "#transfer")
            check("mobile fresh transfer URL", lambda: (clear_anchor(page, "transfer"), active(page, "transfer")))
            check("mobile all fresh lesson URLs after settled layout", lambda: fresh_lesson_urls(page))
            check("mobile architecture refreshes and visible pipeline", lambda: architecture_refreshes(page))
            for width in [390, 320]:
                page.set_viewport_size({"width": width, "height": 1000})
                check(f"mobile {width}px containment", lambda: overflow(page))
            page.set_viewport_size({"width": 390, "height": 1000})
            load(page)
            page.screenshot(path=str(OUT / "tutorial-mobile.png"))
            check("no browser page or console errors", lambda: require(not errors, "; ".join(errors)))
            context.close()
        finally:
            browser.close()


if __name__ == "__main__":
    try:
        main()
    except Exception as error:
        results.append({"name": "browser harness", "passed": False, "error": str(error)})
        print(f"FAIL browser harness: {str(error).splitlines()[0]}")
    report = {"url": PAGE, "results": results, "errors": errors, "downloads": downloads}
    (OUT / "tutorial-results.json").write_text(json.dumps(report, indent=2) + "\n")
    passed = sum(result["passed"] for result in results)
    print(f"{passed}/{len(results)} checks pass; results: {OUT / 'tutorial-results.json'}")
    sys.exit(0 if results and passed == len(results) else 1)
