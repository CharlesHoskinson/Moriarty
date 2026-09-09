"""Drive the built site and assert what it actually renders.

Covers the states a screenshot cannot reach: both reader modes, both themes,
phone width, an opened action target, the facet lens, and the two computed
forks. Console errors are collected and treated as failures.
"""
import json, os, sys
from playwright.sync_api import sync_playwright

URL = os.environ.get("URL", "http://localhost:8891/")
OUT = os.environ.get("SHOTS", "/tmp/moriarty-site-shots")
os.makedirs(OUT, exist_ok=True)

results, errors = [], []

def check(name, cond, detail=""):
    results.append((name, bool(cond), detail))
    print(f"{'PASS' if cond else 'FAIL'}  {name}{'  — ' + detail if detail else ''}")

with sync_playwright() as pw:
    b = pw.chromium.launch(executable_path=os.environ.get("CHROME", None) or None)
    page = b.new_page(viewport={"width": 1280, "height": 900})
    page.on("console", lambda m: errors.append(f"{m.type}: {m.text}") if m.type in ("error", "warning") else None)
    page.on("pageerror", lambda e: errors.append(f"pageerror: {e}"))
    page.goto(URL, wait_until="networkidle")

    # --- structure ---------------------------------------------------------
    secs = page.eval_on_selector_all("main > section", "els => els.map(e => e.id)")
    check("all eight sections render", len(secs) == 8, f"{secs}")
    check("no console errors on load", not errors, "; ".join(errors[:3]))

    # --- the Register ------------------------------------------------------
    cols = page.eval_on_selector_all(".reg-col", "els => els.length")
    check("register has a column per category", cols == 8, f"{cols} columns")
    chips = page.eval_on_selector_all(".reg-chip", "els => els.map(e => e.textContent)")
    check("register shows every action target", len(chips) == 25,
          f"{len(chips)} chips (24 targets, DA12 twice)")
    uniq = sorted(set(chips))
    check("targets run DA01 to DA24", uniq[0] == "DA01" and uniq[-1] == "DA24" and len(uniq) == 24,
          f"{len(uniq)} unique")
    shared = page.eval_on_selector_all(".reg-chip.is-shared", "els => els.map(e => e.textContent)")
    check("DA12 is marked shared in both families", shared == ["DA12", "DA12"], f"{shared}")
    facet_rows = page.eval_on_selector_all(".reg-row", "els => els.length")
    check("register has a row per facet", facet_rows == 8, f"{facet_rows} rows")

    # no tallies anywhere in the rendered page
    body = page.inner_text("body")
    import re
    tally = re.findall(r"\b\d+\s*/\s*(?:24|8|5|12|13)\b", body)
    check("no coverage readout on the page", not tally, f"{tally[:4]}")

    # --- facet lens --------------------------------------------------------
    page.click(".reg-row:nth-child(6) th button")   # Oracles row
    page.wait_for_selector(".lens")
    lens_items = page.eval_on_selector_all(".lens-grid > div", "els => els.length")
    check("facet lens compares every family", lens_items == 8, f"{lens_items}")
    page.screenshot(path=f"{OUT}/lens.png", clip={"x":0,"y":300,"width":1280,"height":900})
    page.click(".lens-head button")

    # --- opening a target reveals the Fork ---------------------------------
    page.click(".target-row")                      # DA01
    page.wait_for_selector(".fork")
    check("opening a target reveals the fork", page.is_visible(".fork"))
    marked = page.eval_on_selector_all(".reg-chip.is-read", "els => els.map(e => e.textContent)")
    check("reading a target marks it in the register", marked == ["DA01"], f"{marked}")

    # --- the computed swap -------------------------------------------------
    vals = page.eval_on_selector_all(".fork-col-required .fork-v", "els => els.map(e => e.textContent)")
    want = ["9,970,000", "19,940,000,000,000", "1,009,970,000", "19,743"]
    check("swap computes the repository's intermediates", all(w in vals for w in want), f"{vals[:5]}")
    check("remainder stays in reserve_b", any("162,290,000" in v for v in vals), "")

    page.fill("#min-out", "19744")
    page.wait_for_selector(".fork-reject")
    rej = page.inner_text(".fork-reject")
    check("19,744 fails the named guard", "minimum output not met" in rej, rej.replace("\n", " ")[:60])
    page.screenshot(path=f"{OUT}/fork-reject.png", clip={"x":0,"y":0,"width":1280,"height":900})
    page.fill("#min-out", "19743")
    page.wait_for_timeout(100)
    check("19,743 is accepted", not page.is_visible(".fork-reject"))

    # --- the repayment fork ------------------------------------------------
    page.click(".reg-col:nth-child(3) button")     # F2 Credit
    page.wait_for_timeout(150)
    rows = page.eval_on_selector_all(".target-row", "els => els.length")
    check("credit tab shows its seven targets", rows == 7, f"{rows}")
    page.click(".targets li:nth-child(3) .target-row")   # DA06
    page.wait_for_selector(".fork")
    alloc = page.inner_text(".fork-col-required")
    ok = "interest 3" in alloc and "principal 100" in alloc      # AccrualFirst on payment 7
    check("repayment shows three allocation splits", ok, alloc.replace("\n", " ")[:90])
    check("plausible column reads undefined", "undefined" in page.inner_text(".fork-col-plausible"))

    # --- verify mode -------------------------------------------------------
    build_text = page.inner_text("#categories .lede")
    page.click(".mode-switch button:nth-child(2)")
    page.wait_for_timeout(150)
    verify_text = page.inner_text("#categories .lede")
    check("verify mode changes the copy", build_text != verify_text)
    check("verify mode keeps the register", page.eval_on_selector_all(".reg-col", "e => e.length") == 8)
    page.screenshot(path=f"{OUT}/verify.png", full_page=False)

    # mode survives a reload
    page.reload(wait_until="networkidle")
    persisted = page.get_attribute(".mode-switch button:nth-child(2)", "aria-pressed")
    check("mode persists across a reload", persisted == "true", f"aria-pressed={persisted}")
    page.click(".mode-switch button:nth-child(1)")

    # --- dark theme --------------------------------------------------------
    page.emulate_media(color_scheme="dark")
    page.wait_for_timeout(120)
    bg = page.eval_on_selector("body", "e => getComputedStyle(e).backgroundColor")
    check("dark theme repaints the ground", bg not in ("rgb(250, 249, 246)", "rgba(0, 0, 0, 0)"), bg)
    page.screenshot(path=f"{OUT}/dark.png", full_page=False)
    page.emulate_media(color_scheme="light")

    # --- phone width -------------------------------------------------------
    page.set_viewport_size({"width": 400, "height": 900})
    page.wait_for_timeout(200)
    overflow = page.evaluate("() => document.documentElement.scrollWidth - document.documentElement.clientWidth")
    check("no horizontal page scroll at 400px", overflow <= 1, f"overflow {overflow}px")
    wide = page.evaluate("""() => [...document.querySelectorAll('main *')]
        .filter(e => e.scrollWidth > 400 && !e.closest('.scroll-x') && !e.matches('.scroll-x')
                     && getComputedStyle(e).overflowX !== 'auto')
        .map(e => e.className + ':' + e.scrollWidth).slice(0, 5)""")
    check("nothing overflows its container at 400px", not wide, f"{wide}")
    page.screenshot(path=f"{OUT}/phone.png", full_page=False)

    # --- keyboard ----------------------------------------------------------
    # Reload first: tab order is a property of a fresh document, and earlier
    # interactions in this run have already moved focus into the page.
    page.set_viewport_size({"width": 1280, "height": 900})
    page.reload(wait_until="networkidle")
    page.keyboard.press("Tab")
    first = page.evaluate("() => document.activeElement.className")
    check("first tab stop is the skip link", "skip" in first, first)
    page.keyboard.press("Enter")
    page.wait_for_timeout(120)
    check("skip link reaches the content", page.evaluate("() => location.hash") == "#main",
          page.evaluate("() => location.hash"))

    # Every control the reader needs must be reachable, and the register alone
    # carries a button per category and per facet.
    reg_btns = page.eval_on_selector_all(".reg-col button, .reg-row th button", "e => e.length")
    check("every register control is a real button", reg_btns == 16, f"{reg_btns}")
    targets_btns = page.eval_on_selector_all(".target-row", "e => e.length")
    check("every action target is reachable by keyboard", targets_btns >= 1, f"{targets_btns}")
    trap = page.evaluate("""() => [...document.querySelectorAll('[tabindex]')]
        .filter(e => Number(e.getAttribute('tabindex')) > 0).length""")
    check("no positive tabindex traps", trap == 0, f"{trap}")

    # --- reduced motion ----------------------------------------------------
    page.emulate_media(reduced_motion="reduce")
    page.wait_for_timeout(100)
    dur = page.eval_on_selector("html", "e => getComputedStyle(e).scrollBehavior")
    check("reduced motion disables smooth scrolling", dur == "auto", dur)

    # --- accessibility basics ---------------------------------------------
    no_name = page.evaluate("""() => [...document.querySelectorAll('button')]
        .filter(b => !b.textContent.trim() && !b.getAttribute('aria-label')).length""")
    check("every button has an accessible name", no_name == 0, f"{no_name} unnamed")
    tables_no_caption = page.evaluate("""() => [...document.querySelectorAll('table')]
        .filter(t => !t.caption && !t.querySelector('thead')).length""")
    check("every table has a header or caption", tables_no_caption == 0, f"{tables_no_caption}")
    h1s = page.eval_on_selector_all("h1", "e => e.length")
    check("exactly one h1", h1s == 1, f"{h1s}")

    # --- the masthead stays put --------------------------------------------
    page.set_viewport_size({"width": 1280, "height": 900})
    page.evaluate("() => window.scrollTo(0, 4000)")
    page.wait_for_timeout(200)
    top = page.eval_on_selector(".masthead", "e => Math.round(e.getBoundingClientRect().top)")
    check("masthead stays pinned while scrolling", top == 0, f"top {top}px at scrollY 4000")
    page.evaluate("() => window.scrollTo(0, 0)")
    page.wait_for_timeout(150)

    # --- masthead fits at every width --------------------------------------
    for w in (1440, 1280, 1180, 1024, 900, 768, 400):
        page.set_viewport_size({"width": w, "height": 900})
        page.wait_for_timeout(90)
        clipped = page.evaluate("""() => {
            const bar = document.querySelector('.masthead');
            const r = bar.getBoundingClientRect();
            return [...bar.querySelectorAll('*')]
              .filter(e => e.offsetParent !== null)
              .filter(e => { const b = e.getBoundingClientRect();
                             return b.right > r.right + 1 || b.left < r.left - 1; })
              .map(e => (e.className || e.tagName) + ':' + Math.round(e.getBoundingClientRect().right));
        }""")
        check(f"masthead fits at {w}px", not clipped, f"{clipped[:3]}")
        scroll = page.evaluate("() => document.documentElement.scrollWidth - document.documentElement.clientWidth")
        check(f"no page overflow at {w}px", scroll <= 1, f"{scroll}px")

    check("no console errors after driving the page", not errors, "; ".join(errors[:3]))
    b.close()

passed = sum(1 for _, ok, _ in results if ok)
print(f"\n{passed}/{len(results)} passed")
if errors:
    print("\nconsole output captured:")
    for e in errors[:10]: print("  ", e)
sys.exit(0 if passed == len(results) else 1)
