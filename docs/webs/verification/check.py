"""Ordinary offline browser checks of a frozen explanatory HTML artifact."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib
import json
import re
from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parent
HTML = ROOT / "index.html"
result = {"observedAt": datetime.now(timezone.utc).isoformat(),
          "artifactSha256": hashlib.sha256(HTML.read_bytes()).hexdigest(),
          "scope": "Frontend behavior and layout only; no proof, ledger or security correctness claim.",
          "checks": [], "pageErrors": [], "consoleErrors": [], "externalRequests": [], "screenshots": []}

def check(name, value, passed):
    result["checks"].append({"name": name, "observed": value, "pass": bool(passed)})

def shot(page, name, state):
    path = ROOT / (name + ".png")
    page.screenshot(path=str(path))
    result["screenshots"].append({"path": path.name, "state": state,
        "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})

with sync_playwright() as p:
    browser = p.chromium.launch(executable_path="/home/charl/.cache/ms-playwright/chromium-1223/chrome-linux64/chrome",
                              headless=True, args=["--no-sandbox"])
    result["browserVersion"] = browser.version
    context = browser.new_context(viewport={"width": 1440, "height": 1000}, reduced_motion="reduce")
    def route(request):
        if request.request.url.startswith(("file:", "data:", "about:")):
            request.continue_()
        else:
            result["externalRequests"].append(request.request.url)
            request.abort()
    context.route("**/*", route)
    page = context.new_page()
    page.on("pageerror", lambda e: result["pageErrors"].append(str(e)))
    page.on("console", lambda e: result["consoleErrors"].append(e.text) if e.type == "error" else None)
    page.goto(HTML.as_uri())
    shot(page, "desktop-top", "fresh default; invoice35/funding112; layer1; circuit counter default")
    check("one main heading", page.locator("h1").count(), page.locator("h1").count() == 1)
    for case in ["invoice", "dex", "issuance", "loan", "treasury"]:
        page.locator(f'[data-case="{case}"]').click()
        roles = page.locator("[data-strand]")
        check(case + " role count", roles.count(), roles.count() == 3)
        for i in range(roles.count()):
            roles.nth(i).click()
            title = page.locator(".inspector h3").inner_text()
            check(case + " role selection " + str(i), title, title in roles.nth(i).inner_text())
        page.get_by_test_id("omit-strand").check()
        status = page.get_by_test_id("assembly-status").inner_text()
        check(case + " omit", status, "2 of 3" in status)
        page.get_by_test_id("omit-strand").uncheck()
        status = page.get_by_test_id("assembly-status").inner_text()
        check(case + " restore without stray zero", status, "3 of 3" in status and not status.endswith("0"))
    page.locator('[data-case="invoice"]').click()
    for mode, opening in [("proposed112", 112), ("source110", 110)]:
        page.locator(f'[data-funding="{mode}"]').click()
        for amount in [1, 35, 99]:
            page.locator("#payment").fill(str(amount))
            expected = {"closing-cash": opening-amount-2, "invoice-duty": 100-amount,
                        "reserve-duty": 10, "total-duties": 110-amount, "gross-debit": amount+2}
            for key, value in expected.items():
                actual = page.get_by_test_id(key).inner_text()
                check(f"{mode}/{amount}/{key}", actual, actual.split()[0] == str(value))
    page.locator('[data-funding="proposed112"]').click()
    page.locator("#payment").fill("36")
    page.locator('[data-strand="invoice-payment"]').click()
    statement = page.get_by_test_id("role-statement").inner_text()
    check("invoice inspector tracks36/debit38", statement, "36" in statement and "38" in statement)
    page.locator("#payment").fill("35")
    page.locator("#worksheet-title").scroll_into_view_if_needed()
    shot(page, "worksheet-112-payment35", "fee-funded112; payment35; cash75; duties65+10")
    layers = page.locator("[data-layer]")
    check("seven selectable book layers", layers.count(), layers.count() == 7)
    for number in range(1, 8):
        control = page.locator(f'[data-layer="{number}"]')
        control.click()
        panel = page.locator("#selected-layer").inner_text()
        name = ["Setup", "Language", "Witness", "Arithmetization", "Proof System", "Cryptographic Bedrock", "Verification"][number-1]
        check("book layer " + str(number), panel[:600], control.get_attribute("aria-pressed") == "true" and name.casefold() in panel.casefold())
    page.locator("#seven-layers").scroll_into_view_if_needed()
    shot(page, "seven-layers-verification", "layer7 selected; other layer selectors available")
    dependencies = page.locator("[data-dependency]")
    if dependencies.count():
        number = dependencies.first.get_attribute("data-dependency")
        dependencies.first.click()
        check("layer dependency navigation", number, page.locator(f'[data-layer="{number}"]').get_attribute("aria-pressed") == "true")
    counter = page.locator("#circuit-count")
    counter.fill("8")
    text = page.locator(".setup-count-result").inner_text()
    check("hypothetical circuit count8", text, "8" in text and counter.input_value() == "8")
    page.get_by_role("button", name="More circuits", exact=True).click()
    check("increase circuit count", counter.input_value(), counter.input_value() == "9")
    page.get_by_role("button", name="Fewer circuits", exact=True).click()
    check("decrease circuit count", counter.input_value(), counter.input_value() == "8")
    page.locator("#groth16").scroll_into_view_if_needed()
    shot(page, "groth16-intro", "Groth16 explanations and setup stages; counter8 below")
    page.locator(".setup-counter").scroll_into_view_if_needed()
    shot(page, "groth16-counter8", "hypothetical8 compatible circuits; no actual circuit inventory count")
    links = page.locator('a[href^="#"]').evaluate_all('(es)=>es.map(e=>({label:e.innerText,href:e.getAttribute("href"),exists:!!document.getElementById(e.getAttribute("href").slice(1))}))')
    check("local anchor destinations", links, all(x["exists"] for x in links))
    for width in [1440, 768, 390, 320]:
        page.set_viewport_size({"width": width, "height": 844 if width < 500 else 1000})
        page.evaluate("scrollTo(0,0)")
        dw = page.evaluate("document.documentElement.scrollWidth")
        check("layout width" + str(width), dw, dw == width)
        small = page.locator(".site-nav a,.brand,[data-case],[data-strand],[data-funding],[data-layer],[data-dependency],.counter-input button").evaluate_all('(es)=>es.filter(e=>e.getClientRects().length).map(e=>({label:e.innerText,w:e.getBoundingClientRect().width,h:e.getBoundingClientRect().height})).filter(e=>e.w<43.99||e.h<43.99)')
        check("44px project targets width" + str(width), small, not small)
        if width == 390:
            shot(page, "mobile-top", "390px fresh top; same invoice35/funding112; circuit8")
    page.set_viewport_size({"width": 390, "height": 844})
    page.locator('[data-layer="4"]').click()
    page.evaluate("new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve)))")
    rect = page.locator("#selected-layer").bounding_box()
    check("mobile layer panel reveal", rect, -1 <= rect["y"] < 200)
    shot(page, "mobile-layer4", "390px layer4 selected")
    page.locator('[data-case="dex"]').click()
    page.locator('[data-strand="dex-pool"]').click()
    page.evaluate("new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve)))")
    rect = page.locator("#role-inspector").bounding_box()
    check("mobile case inspector reveal", rect, -1 <= rect["y"] < 200)
    page.locator('[data-case="invoice"]').focus()
    page.keyboard.press("Enter")
    check("keyboard case activation", page.locator('[data-case="invoice"]').get_attribute("aria-pressed"), page.locator('[data-case="invoice"]').get_attribute("aria-pressed") == "true")
    page.locator('[data-layer="3"]').focus()
    page.keyboard.press("Space")
    check("keyboard layer activation", page.locator('[data-layer="3"]').get_attribute("aria-pressed"), page.locator('[data-layer="3"]').get_attribute("aria-pressed") == "true")
    page.evaluate('document.querySelectorAll("details").forEach(e=>e.open=true)')
    page.add_style_tag(content="html{font-size:200% !important}")
    page.evaluate("scrollTo(0,0)")
    dw = page.evaluate("document.documentElement.scrollWidth")
    check("all disclosures open 200percent width390", dw, dw == 390)
    overflow = page.locator("h1,h2,h3,p,button,summary,output,dd").evaluate_all('(es)=>es.filter(e=>e.getClientRects().length&&getComputedStyle(e).visibility!=="hidden").map(e=>({text:e.innerText.slice(0,100),cw:e.clientWidth,sw:e.scrollWidth,x:e.getBoundingClientRect().x,right:e.getBoundingClientRect().right})).filter(e=>e.sw>e.cw+2||e.x<-.5||e.right>innerWidth+.5)')
    check("enlarged content wrapping", overflow, not overflow)
    shot(page, "mobile-enlarged", "390px rootfont200percent; all disclosures open")
    text = page.locator("body").inner_text()
    unresolved = re.findall(r"\{(?:K|amount|debit|cash|openingCash|invoiceDuty|reserveDuty|duties)\}", text)
    check("no unresolved displayed templates", unresolved, not unresolved)
    check("reduced motion", page.evaluate('getComputedStyle(document.documentElement).scrollBehavior'), page.evaluate('getComputedStyle(document.documentElement).scrollBehavior') != "smooth")
    browser.close()
result["pass"] = all(x["pass"] for x in result["checks"]) and not any(result[k] for k in ["pageErrors", "consoleErrors", "externalRequests"])
(ROOT / "RESULT.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({"pass": result["pass"], "checks": len(result["checks"]), "failures": [x for x in result["checks"] if not x["pass"]], "pageErrors": result["pageErrors"], "externalRequests": result["externalRequests"]}, indent=2))
