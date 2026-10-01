"""Smoke-check the built documentation pages under their project subpath.

The documentation is five pages, one per Diátaxis need: a hub (documentation.html),
Tutorials (tutorial.html), How-to guides (how-to.html), Reference (reference.html)
and Explanation (explanation.html). This suite opens each one, follows its
anchors, checks the old single-page tutorial links still arrive, and checks layout
at desktop and phone widths in both colour schemes.

URL is the build directory URL; CHROME may select a binary (empty uses
Playwright Chromium); SHOTS selects where captures go.
This gate does not run the beta CLI, sign, prove or submit transactions.
"""
import json
import os
import re
import sys
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urldefrag, urljoin, urlparse

from playwright.sync_api import expect, sync_playwright

BASE = os.environ.get("URL", "http://localhost:8891/").rstrip("/") + "/"
OUT = Path(os.environ.get("SHOTS", "/tmp/moriarty-site-shots"))
SRC = Path(__file__).resolve().parents[1] / "src/guide"

# Section ids each page must have, in any order. Other pages link to these ids.
PAGES = {
    "documentation.html": {"start", "compass", "examples", "status", "other"},
    "tutorial.html": {"overview", "getting-started", "transfer", "repayment", "testing", "next-steps"},
    "how-to.html": {"install-archive", "restore-shell", "editor", "write-test", "test-rejection", "change-amount", "diagnose",
                    "cap-with-fee", "format-and-inspect", "write-amm", "write-lending", "write-stablecoins", "write-options",
                    "write-oracles", "write-governance", "write-bridges", "write-staking", "check-family", "sign-intent"},
    "reference.html": {"cli", "language", "results", "diagnostics", "support-labels", "support-matrix", "premises-bindings",
                       "scenario-format", "test-format", "family-amm", "family-lending", "family-stablecoins", "family-options",
                       "family-oracles", "family-governance", "family-bridges", "family-staking", "example-files", "release",
                       "reading-code-blocks"},
    "explanation.html": {"purpose", "names-ids-symbols", "explicit-terms", "three-questions", "architecture",
                         "why-derive-expectations", "patterns", "federation", "stipulation", "stance", "amm", "lending",
                         "stablecoins", "options", "oracles", "governance", "bridges", "staking", "horizon"},
}

# Old single-page tutorial links and where they must land now: (hash on tutorial.html, page, id).
LEGACY = [
    ("overview", "tutorial.html", "overview"),
    ("transfer", "tutorial.html", "transfer"),
    ("install", "tutorial.html", "install"),
    ("core-rejection", "tutorial.html", "core-rejection"),
    ("lending-repay", "tutorial.html", "repayment"),
    ("commands", "reference.html", "cli"),
    ("overview-legend", "reference.html", "support-labels"),
    ("overview-areas", "documentation.html", "examples"),
    ("syntax", "reference.html", "language"),
    ("syntax-failures", "reference.html", "results"),
    ("trust", "reference.html", "support-matrix"),
    ("install-archive", "how-to.html", "install-archive"),
    ("options-pattern", "how-to.html", "write-options"),
    ("options-amounts", "reference.html", "family-options"),
    ("patterns", "explanation.html", "patterns"),
    ("architecture", "explanation.html", "architecture"),
    ("architecture-kernel", "explanation.html", "architecture-kernel"),
    ("what-a-pass-means", "explanation.html", "stipulation"),
    ("amm", "explanation.html", "amm"),
    ("staking", "explanation.html", "staking"),
    ("governance-hostile", "explanation.html", "governance"),
    ("oracles-horizon", "explanation.html", "horizon"),
]

# Themed figures: page, figure image stem.
FIGURES = [("documentation.html", "quadrant"), ("explanation.html", "hero"), ("explanation.html", "pipeline")]

results, errors = [], []
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


def load(page, path, fragment=""):
    response = page.goto(BASE + path + fragment, wait_until="networkidle")
    if response is None:  # Fragment changes may reuse an existing document.
        response = page.reload(wait_until="networkidle")
    require(response is not None and response.status in (200, 304), f"{path} must return HTTP 200")
    expect(page.locator("section.guide-section").first).to_be_attached()


def active(page, section):
    expect(page.locator(f'#guide-lessons a[href="#{section}"]')).to_have_attribute("aria-current", "location")


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
      return r.top>=ceiling-2&&r.top<innerHeight&&anchor.top>=ceiling-2&&anchor.top<innerHeight&&(end||anchor.top<=ceiling+24);
    }""", arg=ident)


def settled_layout(page):
    page.wait_for_function("""document.readyState==='complete'&&(!document.fonts||document.fonts.status==='loaded')""")
    page.evaluate("""()=>new Promise(resolve=>{let last=-1,steady=0;function tick(){const h=document.documentElement.scrollHeight;steady=h===last?steady+1:0;last=h;if(steady>=30)resolve();else requestAnimationFrame(tick)}requestAnimationFrame(tick)})""")
    page.wait_for_timeout(1800)  # Outlast the transient 1.5-second place pin.


class Anchors(HTMLParser):
    def __init__(self):
        super().__init__()
        self.ids = set()

    def handle_starttag(self, tag, attrs):
        self.ids.update(value for key, value in attrs if value and (key == "id" or (tag == "a" and key == "name")))


def rendered_ids(page, document):
    """Ids of a rendered page. The pages render client-side, so the HTML source alone has none of them."""
    other = page.context.new_page()
    try:
        response = other.goto(document, wait_until="networkidle")
        require(response is not None and response.status in (200, 304), f"Local destination failed: {document}")
        if other.locator("section.guide-section").count() == 0:
            parser = Anchors()
            parser.feed(other.content())
            return parser.ids
        return set(other.evaluate("[...document.querySelectorAll('[id]')].map(e=>e.id)"))
    finally:
        other.close()


def structure(page, path, cache):
    load(page, path)
    ids = page.locator("section.guide-section[id]").evaluate_all("els=>els.map(e=>e.id)")
    require(len(ids) == len(set(ids)), f"{path}: duplicate section ids {ids}")
    missing = PAGES[path] - set(ids)
    require(not missing, f"{path}: missing sections {sorted(missing)}")
    expect(page.locator("h1")).to_have_count(1)
    expect(page.locator(f'header nav a[href="{path}"]')).to_have_attribute("aria-current", "page")
    all_ids = page.evaluate("[...document.querySelectorAll('[id]')].map(e=>e.id)")
    dupes = sorted({i for i in all_ids if all_ids.count(i) > 1})
    require(not dupes, f"{path}: duplicate ids {dupes}")
    here = urljoin(BASE, path)
    for href in set(page.locator("a[href]").evaluate_all("els=>els.map(e=>e.href)")):
        parsed = urlparse(href)
        if parsed.netloc != urlparse(BASE).netloc or not parsed.fragment:
            continue
        ident = unquote(parsed.fragment)
        document, _ = urldefrag(href)
        if document == here:
            require(page.evaluate("id=>!!document.getElementById(id)", ident), f"{path}: broken fragment {href}")
        else:
            if document not in cache:
                cache[document] = rendered_ids(page, document)
            require(ident in cache[document], f"{path}: broken link {href}")


def fresh_urls(page, path):
    for ident in sorted(PAGES[path]):
        load(page, path, "#" + ident)
        settled_layout(page)
        clear_anchor(page, ident)
        active(page, ident)


def legacy(page):
    for old, path, ident in LEGACY:
        page.goto("about:blank")  # Arrive fresh, as a link from elsewhere does.
        page.goto(BASE + "tutorial.html#" + old, wait_until="networkidle")
        page.wait_for_function("p=>location.pathname.endsWith('/'+p)", arg=path)
        require(page.evaluate("()=>decodeURIComponent(location.hash.slice(1))") == ident, f"tutorial.html#{old} landed on {page.url}, expected {path}#{ident}")
        settled_layout(page)
        clear_anchor(page, ident)


def figures(page, scheme):
    for path, stem in FIGURES:
        load(page, path)
        figure = page.locator(f'figure:has(img[src*="{stem}-"])')
        expect(figure).to_have_count(1)
        figure.scroll_into_view_if_needed()
        page.wait_for_function("""s=>{const v=[...document.querySelectorAll('img[src*="'+s+'-"]')].filter(i=>i.getBoundingClientRect().height>0);return v.length===1&&v[0].complete&&v[0].naturalWidth>0}""", arg=stem)
        shown = page.evaluate("""s=>[...document.querySelectorAll('img[src*="'+s+'-"]')].filter(i=>i.getBoundingClientRect().height>0).map(i=>i.currentSrc)[0]""", stem)
        phone = page.viewport_size["width"] <= 640
        want = f"{stem}-mobile-{scheme}." if phone and stem == "pipeline" else f"{stem}-{scheme}."
        require(want in shown, f"{path}: {stem} shows {shown} in {scheme} scheme, expected {want}")


def keyboard_code(page):
    for path in PAGES:
        load(page, path)
        settled_layout(page)
        bad = page.evaluate("""[...document.querySelectorAll('pre')].filter(p=>p.getClientRects().length&&p.scrollWidth>p.clientWidth+1&&p.tabIndex<0).length""")
        require(bad == 0, f"{path}: {bad} overflowing code blocks cannot be reached by keyboard")


def mobile_navigation(page):
    response = page.goto(urljoin(BASE, "index.html"), wait_until="networkidle")
    require(response is not None and response.status == 200, "Home must return HTTP 200")
    tutorial = page.get_by_role("link", name=re.compile(r"tutorial", re.I)).first
    expect(tutorial).to_be_visible()
    tutorial.click()
    expect(page.locator("#overview")).to_be_attached()
    load(page, "tutorial.html", "#repayment")
    page.evaluate("scrollBy(0,400)")
    button = page.get_by_role("button", name=re.compile(r"^Contents\b", re.I))
    expect(button).to_be_visible()
    require(button.evaluate("e=>{const r=e.getBoundingClientRect();return r.top>=0&&r.bottom<=innerHeight}"), "Contents control is offscreen after scrolling")
    button.click()
    expect(page.get_by_role("searchbox")).to_be_focused()
    page.locator('#guide-lessons a[href="#transfer"]').first.click()
    expect(button).to_have_attribute("aria-expanded", "false")
    clear_anchor(page, "transfer")
    active(page, "transfer")
    require(page.evaluate("document.getElementById('transfer').contains(document.activeElement)"), "Selected section did not receive focus")


def mode_rules():
    """The Diátaxis split stays honest: each rule names text that belongs on one page only."""
    pages = {p.stem: p.read_text() for p in (SRC / "pages").glob("*.tsx")}
    for extra in ("tutorial", "howto", "reference", "explanation"):
        for p in (SRC / extra).glob("*.tsx") if (SRC / extra).is_dir() else []:
            pages[f"{extra}/{p.stem}"] = p.read_text()
    joined = "\n".join(pages.values())
    require("node dist/cli.js" not in joined, "Reader-facing commands use the mori function, not node dist/cli.js")
    require("Downloads" not in joined, "No step depends on a browser downloads folder")
    for name, text in pages.items():
        if name.startswith(("Tutorial", "tutorial/")):
            require("<Status" not in text, f"{name}: the tutorial uses no status chips")
            require('kind="horizon"' not in text, f"{name}: the tutorial shows no proposed syntax")
        if not name.startswith(("Explanation", "explanation/")):
            require('kind="horizon"' not in text, f"{name}: proposed syntax lives on the explanation page only")


def main():
    cache = {}
    with sync_playwright() as pw:
        browser = pw.chromium.launch(executable_path=os.environ.get("CHROME") or None)
        try:
            for scheme in ["light", "dark"]:
                context = browser.new_context(viewport={"width": 1440, "height": 1000}, color_scheme=scheme)
                context.on("page", lambda p: p.on("pageerror", lambda e: errors.append(str(e))))
                context.on("page", lambda p: p.on("console", lambda m: errors.append(m.text) if m.type == "error" else None))
                page = context.new_page()
                if scheme == "light":
                    for path in PAGES:
                        check(f"{path}: sections, ids and links", lambda path=path: structure(page, path, cache))
                        check(f"{path}: desktop containment", lambda: overflow(page))
                        page.screenshot(path=str(OUT / f"{path[:-5]}-desktop.png"))
                        check(f"{path}: fresh section URLs", lambda path=path: fresh_urls(page, path))
                    check("legacy tutorial links arrive on their new pages", lambda: legacy(page))
                    check("overflowing code is keyboard reachable", lambda: keyboard_code(page))
                check(f"themed figures show the {scheme} variant (desktop)", lambda scheme=scheme: figures(page, scheme))
                page.set_viewport_size({"width": 390, "height": 900})
                check(f"themed figures show the {scheme} variant (phone)", lambda scheme=scheme: figures(page, scheme))
                for path in PAGES:
                    for width in [390, 320]:
                        page.set_viewport_size({"width": width, "height": 900})
                        load(page, path)
                        check(f"{path}: {width}px containment ({scheme})", lambda: overflow(page))
                    page.screenshot(path=str(OUT / f"{path[:-5]}-mobile-{scheme}.png"))
                if scheme == "light":
                    page.set_viewport_size({"width": 390, "height": 900})
                    check("mobile home entry and drawer navigation", lambda: mobile_navigation(page))
                context.close()
            check("Diátaxis mode rules in page sources", mode_rules)
            check("no browser page or console errors", lambda: require(not errors, "; ".join(errors)))
        finally:
            browser.close()


if __name__ == "__main__":
    try:
        main()
    except Exception as error:
        results.append({"name": "browser harness", "passed": False, "error": str(error)})
        print(f"FAIL browser harness: {str(error).splitlines()[0]}")
    report = {"url": BASE, "results": results, "errors": errors}
    (OUT / "tutorial-results.json").write_text(json.dumps(report, indent=2) + "\n")
    passed = sum(result["passed"] for result in results)
    print(f"{passed}/{len(results)} checks pass; results: {OUT / 'tutorial-results.json'}")
    sys.exit(0 if results and passed == len(results) else 1)
