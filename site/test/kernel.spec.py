"""Drive the built kernel page and assert what it actually does.

Covers the positive path to completion, the failure and unknown branches,
rejected candidates, evidence outcomes, the conflicting events, the evidence
inspector, keyboard operation, 320px, dark theme, reduced motion, print, the
no-script fallback, the failed-script fallback and every local route link.
Console errors and failed requests are collected and treated as failures.

Set URL to the directory the build is served from. The CI job serves the build
under /Moriarty/ so that the project subpath is what gets tested.
"""
import os, re, sys
from playwright.sync_api import sync_playwright

BASE = os.environ.get("URL", "http://localhost:8891/").rstrip("/") + "/"
PAGE = BASE + "kernel.html"
OUT = os.environ.get("SHOTS", "/tmp/moriarty-site-shots")
os.makedirs(OUT, exist_ok=True)

results, errors, failed_requests = [], [], []


def check(name, cond, detail=""):
    results.append((name, bool(cond), detail))
    print(f"{'PASS' if cond else 'FAIL'}  {name}{'  — ' + str(detail)[:160] if detail else ''}")


def account(page):
    """The rendered account as the reader sees it."""
    rows = page.eval_on_selector_all(
        ".k-account tr[data-account-row]",
        "els => Object.fromEntries(els.map(e => [e.dataset.accountRow, e.querySelector('td').textContent]))",
    )
    return rows


def announce(page):
    return page.inner_text(".k-announce")


def click(page, selector):
    page.click(selector)
    page.wait_for_timeout(60)


def fresh_run(page):
    """Reset and walk the shared prefix through the timeout."""
    click(page, "[data-event=reset]")
    click(page, "[data-candidate=route-direct]")
    click(page, "[data-event=provider-accepts-duties]")
    click(page, "[data-event=commit-candidate]")
    click(page, "[data-evidence=fresh]")
    click(page, "[data-event=finalize-fill]")
    click(page, "[data-event=reserve-fill]")
    click(page, "[data-event=timeout]")


with sync_playwright() as pw:
    b = pw.chromium.launch(executable_path=os.environ.get("CHROME", None) or None)
    ctx = b.new_context(viewport={"width": 1280, "height": 900})
    page = ctx.new_page()
    page.on("console", lambda m: errors.append(f"{m.type}: {m.text}") if m.type in ("error", "warning") else None)
    page.on("pageerror", lambda e: errors.append(f"pageerror: {e}"))
    page.on("requestfailed", lambda r: failed_requests.append(r.url))
    page.on("response", lambda r: failed_requests.append(f"{r.status} {r.url}") if r.status >= 400 else None)

    resp = page.goto(PAGE, wait_until="networkidle")
    check("kernel.html resolves directly", resp is not None and resp.status == 200, f"status {resp.status if resp else None}")
    check("no console errors on load", not errors, "; ".join(errors[:3]))
    check("no failed asset requests under this base", not failed_requests, "; ".join(failed_requests[:3]))

    # --- structure ---------------------------------------------------------
    check("exactly one h1", page.eval_on_selector_all("h1", "e => e.length") == 1)
    secs = page.eval_on_selector_all("main > section", "els => els.map(e => e.id)")
    check("seven sections in reading order",
          secs == ["agreement", "responsibilities", "follow", "unknown", "evidence", "solvers", "status"], f"{secs}")
    lanes = page.eval_on_selector_all(".k-lane", "e => e.length")
    check("responsibility map has five lanes", lanes == 5, f"{lanes}")
    css_links = page.eval_on_selector_all("link[rel=stylesheet]", "els => els.map(e => e.href)")
    for href in css_links:
        r = page.request.get(href)
        check(f"stylesheet resolves: {href.rsplit('/', 1)[-1]}", r.status == 200, f"{r.status}")
    bg = page.eval_on_selector("body", "e => getComputedStyle(e).backgroundColor")
    check("stylesheet actually applied", bg not in ("rgba(0, 0, 0, 0)", "rgb(255, 255, 255)"), bg)

    # --- enhancement ---------------------------------------------------------
    check("explorer mounted", page.get_attribute("#kernel-root", "data-enhanced") == "true")
    check("static transcript hidden only after enhancement", page.is_hidden("#kernel-static") and page.get_attribute("#kernel-static", "data-superseded") == "true")
    check("inspector mounted", page.get_attribute("#kernel-evidence-root", "data-enhanced") == "true")
    check("static evidence table hidden after enhancement", page.is_hidden("#kernel-evidence-static"))

    # --- initial account -----------------------------------------------------
    a = account(page)
    check("fresh illustration has zero debit and full custody",
          a["grossDebit"] == "0 A" and a["reserved"] == "0 A" and a["confirmedReceipt"] == "0 B of 20 B" and a["custody"] == "11 A", f"{a}")
    check("commit disabled before a candidate and consent", page.is_disabled("[data-event=commit-candidate]"))
    check("finalize disabled before commitment", page.is_disabled("[data-event=finalize-fill]"))

    # --- rejected candidates touch nothing --------------------------------------
    click(page, "[data-candidate=route-wrong-recipient]")
    check("wrong recipient verdict is a rejection", page.get_attribute(".k-verdict", "data-accepted") == "false")
    failing = page.eval_on_selector_all(".k-checks li[data-met=false] .k-check-name", "els => els.map(e => e.textContent)")
    check("wrong recipient fails the recipient check by name", failing == ["Delivery goes to the named recipient"], f"{failing}")
    click(page, "[data-event=provider-accepts-duties]")
    check("commit stays disabled for a rejected candidate", page.is_disabled("[data-event=commit-candidate]"))
    check("rejected candidate leaves the account untouched", account(page) == a, f"{account(page)}")
    page.screenshot(path=f"{OUT}/kernel-rejected.png", full_page=False)

    click(page, "[data-candidate=route-over-fee]")
    failing = page.eval_on_selector_all(".k-checks li[data-met=false] .k-check-name", "els => els.map(e => e.textContent)")
    check("excessive fee fails the fee check alone", failing == ["Fees within fee cap"], f"{failing}")
    gross_detail = page.inner_text(".k-checks li[data-met=true] .k-check-detail >> nth=0")
    check("excessive fee stays within gross cap", "11 A of 11 A" in page.inner_text(".k-checks"), gross_detail)

    # --- positive path -----------------------------------------------------------
    click(page, "[data-candidate=route-direct]")
    check("direct route accepted", page.get_attribute(".k-verdict", "data-accepted") == "true")
    check("commit enabled once consent and a compliant candidate exist", page.is_enabled("[data-event=commit-candidate]"))
    click(page, "[data-event=commit-candidate]")
    check("commitment spends nothing", account(page)["grossDebit"] == "0 A")
    check("candidate selector locked after commitment", page.is_disabled("[data-candidate=route-split]"))

    # evidence gate
    check("evidence starts unknown", page.get_attribute(".k-evidence-result", "data-status") == "unknown")
    click(page, "[data-event=finalize-fill]")
    check("release refused while evidence is missing", announce(page).startswith("Refused") and "unknown" in announce(page), announce(page))
    check("refused release moves nothing", account(page)["grossDebit"] == "0 A")
    click(page, "[data-evidence=stale]")
    check("stale evidence is unmet", page.get_attribute(".k-evidence-result", "data-status") == "unmet")
    click(page, "[data-evidence=unsupported]")
    check("unsupported evidence is neither met nor unmet", page.get_attribute(".k-evidence-result", "data-status") == "unsupported")
    click(page, "[data-evidence=fresh]")
    check("fresh evidence is met", page.get_attribute(".k-evidence-result", "data-status") == "met")

    click(page, "[data-event=finalize-fill]")
    a = account(page)
    check("first fill: 5.5 A gross, 0.5 A fees, 10 B confirmed",
          a["grossDebit"] == "5.5 A" and a["fees"] == "0.5 A" and a["reserved"] == "0 A" and a["confirmedReceipt"] == "10 B of 20 B", f"{a}")
    check("first fill moves custody", a["custody"] == "5.5 A", a["custody"])
    check("delivery duty persists after the prefix", page.get_attribute("[data-duty=deliver-remaining]", "data-status") == "accepted")

    click(page, "[data-event=reserve-fill]")
    a = account(page)
    check("reservation: reserved 5.5 A, debit unchanged", a["reserved"] == "5.5 A" and a["grossDebit"] == "5.5 A", f"{a}")
    check("reservation does not move custody", a["custody"] == "5.5 A", a["custody"])
    custody_label = page.inner_text(".k-account tr[data-account-row=custody] th")
    check("custody row is labeled as the last confirmed balance", "Last confirmed escrow balance" == custody_label, custody_label)
    custody_note = page.inner_text("[data-testid=custody-note]")
    check("custody note says the pending amount's current location is unknown while in flight", "current location of the 5.5 A" in custody_note and "unknown" in custody_note, custody_note)
    check("delivery duty discharge requires acceptance against all predicates", "accepted against all signed predicates" in page.inner_text("[data-duty=deliver-remaining] .k-duty-discharge"))
    check("spent plus reserved reaches the cap", a["exposure"] == "11 A of 11 A", a["exposure"])
    check("fee capacity separates free from encumbered while in flight", a["feeRemaining"] == "0 A free · 0.5 A encumbered by the pending attempt", a["feeRemaining"])
    click(page, "[data-event=second-solver-reserve]")
    check("second solver is refused in the record", "Refused" in page.inner_text(".k-log li:last-child"), page.inner_text(".k-log li:last-child .k-log-meaning"))
    check("second solver changes nothing", account(page) == a)

    click(page, "[data-event=timeout]")
    check("timeout leaves the exposure unchanged", account(page) == a)
    check("observation enabled after timeout", page.is_enabled("[data-observe=late-success]"))
    page.screenshot(path=f"{OUT}/kernel-inflight.png", full_page=False)

    click(page, "[data-observe=late-success]")
    a = account(page)
    check("late success completes: 11 A gross, 1 A fees, 0 reserved, 20 B",
          a["grossDebit"] == "11 A" and a["fees"] == "1 A" and a["reserved"] == "0 A" and a["confirmedReceipt"] == "20 B of 20 B", f"{a}")
    check("phase reads complete", "Complete" in page.inner_text("[data-testid=phase]"))
    check("delivery duty discharged", page.get_attribute("[data-duty=deliver-remaining]", "data-status") == "discharged")
    check("conditional remedy discharged by failure-free completion", page.get_attribute("[data-duty=remedy-on-failure]", "data-status") == "discharged")
    n_before = page.eval_on_selector_all(".k-log li", "e => e.length")
    click(page, "[data-event=duplicate-observation]")
    check("duplicate terminal observation is recorded", page.eval_on_selector_all(".k-log li", "e => e.length") == n_before + 1)
    check("duplicate produces no second discharge", account(page) == a and "no account change" in page.inner_text(".k-log li:last-child"))
    click(page, "[data-event=replay-fill]")
    check("replayed fill identifier changes nothing", account(page) == a)
    page.screenshot(path=f"{OUT}/kernel-complete.png", full_page=False)

    # --- authenticated failure branch ---------------------------------------------
    fresh_run(page)
    click(page, "[data-observe=auth-failure]")
    a = account(page)
    check("failure: 6 A gross, 1 A fees, reservation released, 10 B",
          a["grossDebit"] == "6 A" and a["fees"] == "1 A" and a["reserved"] == "0 A" and a["confirmedReceipt"] == "10 B of 20 B", f"{a}")
    check("failure leaves no fee capacity", a["feeRemaining"] == "0 A", a["feeRemaining"])
    check("remedy duty is pending after failure", page.get_attribute("[data-duty=remedy-on-failure]", "data-status") == "pending")
    check("delivery duty still open after failure", page.get_attribute("[data-duty=deliver-remaining]", "data-status") == "accepted")
    check("failure narrative states the remaining capacity from the numbers", "5 A of ordinary capacity and no fee capacity remain" in page.inner_text(".k-log li:last-child .k-log-meaning"), page.inner_text(".k-log li:last-child .k-log-meaning"))
    click(page, "[data-event=reserve-fill]")
    check("further fill refused by the fee cap", "fee cap" in announce(page), announce(page))
    check("refusal moves nothing", account(page) == a)
    check("attempt-2 listed once after failure", page.inner_text(".k-consumed").count("attempt-2") == 1, page.inner_text(".k-consumed"))

    # --- still unknown branch ---------------------------------------------------------
    fresh_run(page)
    before = account(page)
    click(page, "[data-observe=still-unknown]")
    check("still unknown changes nothing", account(page) == before)
    check("attempt remains in flight", "in flight" in page.inner_text("[data-testid=phase]"))
    click(page, "[data-event=premature-refund]")
    check("premature refund is refused", "Refused" in page.inner_text(".k-log li:last-child"))
    check("premature refund changes nothing", account(page) == before)
    click(page, "[data-observe=late-success]")
    check("a late result after unknown still resolves", account(page)["confirmedReceipt"] == "20 B of 20 B")

    # --- ordering: first fill local, second fill external (audit R2) ------------------
    click(page, "[data-event=reset]")
    click(page, "[data-candidate=route-direct]")
    click(page, "[data-event=provider-accepts-duties]")
    click(page, "[data-event=commit-candidate]")
    click(page, "[data-evidence=fresh]")
    # The UI does not offer out-of-order fills. React drops synthetic clicks on
    # a button whose disabled prop is set, so the reducer's own refusal of these
    # events is exercised in kernel-scenarios.test.mjs rather than here.
    check("reserve is not offered before the first local fill", page.is_disabled("[data-event=reserve-fill]"))
    check("finalize is offered for the first fill", page.is_enabled("[data-event=finalize-fill]"))
    click(page, "[data-event=finalize-fill]")
    check("finalize is not offered for the second fill", page.is_disabled("[data-event=finalize-fill]"))
    check("reserve is offered for the second fill", page.is_enabled("[data-event=reserve-fill]"))
    consumed = page.inner_text(".k-consumed")
    check("no identifier is listed twice", consumed.count("fill-1") == 1, consumed)

    # --- observed success with a missing predicate (audit R1) ----------------------------
    click(page, "[data-event=reset]")
    click(page, "[data-candidate=route-split]")
    click(page, "[data-event=provider-accepts-duties]")
    click(page, "[data-event=commit-candidate]")
    click(page, "[data-evidence=fresh]")
    click(page, "[data-event=finalize-fill]")
    click(page, "[data-evidence=wrong-issuer]")
    click(page, "[data-event=reserve-fill]")
    click(page, "[data-event=timeout]")
    click(page, "[data-observe=late-success]")
    a = account(page)
    check("observed success is accounted even without the document predicate",
          a["grossDebit"] == "11 A" and a["fees"] == "1 A" and a["reserved"] == "0 A" and a["confirmedReceipt"] == "20 B of 20 B", f"{a}")
    check("observed success is recorded, not refused", not announce(page).startswith("Refused") and "acceptance withheld" in announce(page).lower(), announce(page))
    check("phase reads observed, not complete", "Observed" in page.inner_text("[data-testid=phase]"), page.inner_text("[data-testid=phase]"))
    check("delivery duty stays open until acceptance", page.get_attribute("[data-duty=deliver-remaining]", "data-status") == "accepted")
    check("attempt identifier consumed, fill not yet", "attempt-2" in page.inner_text(".k-consumed") and "fill-2" not in page.inner_text(".k-consumed"), page.inner_text(".k-consumed"))
    check("acceptance not offered while the predicate is missing", page.is_disabled("[data-event=accept-observed-fill]"))
    check("no conflicting observation is offered", page.is_disabled("[data-observe=auth-failure]") and page.is_disabled("[data-observe=still-unknown]") and page.is_disabled("[data-observe=late-success]"))
    check("no further fill is offered while acceptance is pending", page.is_disabled("[data-event=reserve-fill]") and page.is_disabled("[data-event=finalize-fill]"))
    n_dup = page.eval_on_selector_all(".k-log li", "e => e.length")
    click(page, "[data-event=duplicate-observation]")
    check("duplicate success result for attempt-2 is refused in the record", "attempt-2" in page.inner_text(".k-log li:last-child .k-log-title") and page.eval_on_selector_all(".k-log li", "e => e.length") == n_dup + 1)
    check("duplicate changes nothing", account(page) == a)
    click(page, "[data-evidence=fresh]")
    check("restoring the predicate offers acceptance", page.is_enabled("[data-event=accept-observed-fill]"))
    click(page, "[data-event=accept-observed-fill]")
    check("acceptance moves no money", account(page) == a)
    check("acceptance completes the agreement", "Complete" in page.inner_text("[data-testid=phase]"))
    check("acceptance discharges both duties", page.get_attribute("[data-duty=deliver-remaining]", "data-status") == "discharged" and page.get_attribute("[data-duty=remedy-on-failure]", "data-status") == "discharged")
    check("acceptance is offered only once", page.is_disabled("[data-event=accept-observed-fill]"))
    page.screenshot(path=f"{OUT}/kernel-observed-accepted.png", full_page=False)

    # --- withheld signature or acceptance -------------------------------------------------
    click(page, "[data-event=reset]")
    click(page, "[data-candidate=route-direct]")
    click(page, "[data-event=provider-accepts-duties]")
    click(page, "[data-event=commit-candidate]")
    click(page, "[data-evidence=fresh]")
    click(page, "[data-condition=provider-signature][data-present=false]")
    click(page, "[data-event=finalize-fill]")
    check("withheld provider signature blocks release", "Provider signature is missing" in announce(page), announce(page))
    check("withheld signature moves nothing", account(page)["grossDebit"] == "0 A")
    click(page, "[data-condition=provider-signature][data-present=true]")
    click(page, "[data-condition=recipient-acceptance][data-present=false]")
    click(page, "[data-event=finalize-fill]")
    check("withheld recipient acceptance blocks release", "Recipient acceptance is missing" in announce(page), announce(page))
    click(page, "[data-condition=recipient-acceptance][data-present=true]")
    click(page, "[data-event=finalize-fill]")
    check("release proceeds once all three conditions hold", account(page)["grossDebit"] == "5.5 A")

    # --- solver label ----------------------------------------------------------------
    before = account(page)
    enabled_before = page.eval_on_selector_all("button.k-act", "els => els.map(e => e.disabled)")
    click(page, "[data-solver=ai]")
    check("AI label is pressed", page.get_attribute("[data-solver=ai]", "aria-pressed") == "true")
    check("AI label changes no account", account(page) == before)
    check("AI label changes no available action", page.eval_on_selector_all("button.k-act", "els => els.map(e => e.disabled)") == enabled_before)

    # --- reset ----------------------------------------------------------------------
    click(page, "[data-event=reset]")
    check("reset restores the fresh fixture", account(page)["grossDebit"] == "0 A" and page.eval_on_selector_all(".k-log li", "e => e.length") == 0)
    check("reset is announced as a new illustration, not a rollback", "New illustration" in announce(page) and "rolled back" in announce(page), announce(page))

    # --- evidence inspector ----------------------------------------------------------
    claim_zk = page.inner_text(".k-insp-claim")
    click(page, "[data-mech=threshold]")
    check("mechanism selector changes the claim", page.inner_text(".k-insp-claim") != claim_zk)
    check("threshold names its corruption assumption", "threshold" in page.inner_text(".k-insp-list").lower())
    click(page, "[data-mech=tee]")
    check("TEE names rollback", "rolled back" in page.inner_text(".k-insp-col:nth-child(1)").lower())
    click(page, "[data-binding=effects]")
    check("binding selector names the flattering-projection risk", "flattering" in page.inner_text(".k-insp-col:nth-child(2)"))
    check("signature-only destination shows the 3-of-5 bypass", "3-of-5" in page.inner_text("[data-bypass]"))
    check("signature-only shows the local limit", "cannot retroactively stop" in page.inner_text(".k-insp-col:nth-child(3)"))
    click(page, "[data-dest=checking]")
    check("hypothetical destination is flagged", page.is_visible(".k-insp-flag"))
    check("no generic proof-enforcing chain claim", "no generic proof-enforcing chain" in page.inner_text("[data-bypass]"))
    page.screenshot(path=f"{OUT}/kernel-inspector.png", full_page=False)

    # --- evidence prose (audit R6) -----------------------------------------------------
    click(page, "[data-mech=tee]")
    tee = page.inner_text(".k-insp-col:nth-child(1)")
    check("TEE claim is about the measured environment and report data", "identifies the measured code" in tee and "ran this code on this input and produced this output" not in page.inner_text(".k-insp-claim"), page.inner_text(".k-insp-claim"))
    check("TEE I/O claim is conditional on bindings and measured code", "binding its input and output" in tee, tee[:200])
    click(page, "[data-mech=threshold]")
    thr = page.inner_text(".k-insp-claim")
    check("threshold claim is a key-verification claim, not individual approval", "verifies under the group key" in thr and "approved exactly these bytes" not in thr, thr)
    check("threshold does-not names individual attribution", "attribute an informed approval" in page.inner_text(".k-insp-not"), page.inner_text(".k-insp-not"))
    static_ev = page.eval_on_selector("#kernel-evidence-static", "e => e.textContent")
    check("static evidence table makes the same TEE qualification", "binding its input and output" in static_ev)
    check("static evidence table makes the same threshold qualification", "verifies under the group key" in static_ev and "attributes no informed approval" in static_ev)
    check("correlation statement is narrow, not collapsing", "share a failure" in page.inner_text(".k-insp-fine") and "are one assumption" not in page.inner_text(".k-insp-fine"))
    check("threshold claim does not imply a threshold of honest signers", "not a threshold of honest signers" in thr and "t − f honest shares" in thr, thr)
    check("static threshold row makes the same t − f qualification", "t − f honest shares" in static_ev)
    evidence_explanation = " ".join(page.locator("#evidence .k-prose").all_inner_texts())
    check("evidence explanation treats shared operators as correlated and binding alone as insufficient",
          "count once" not in evidence_explanation and "correlated dependencies" in evidence_explanation and "common statement is not enough" in evidence_explanation, evidence_explanation)
    click(page, "[data-mech=zk]")
    zk_list = page.inner_text(".k-insp-list")
    check("zero-knowledge is qualified relative to the public statement", "relative to the public statement" in zk_list and "metadata" in zk_list, zk_list[:200])

    # --- four acceptance judgments (audit R7) ----------------------------------------------
    j = page.inner_text(".k-judgments")
    check("four acceptance judgments are explained on the page", all(k in j for k in ("Contract properties", "Intent refinement", "Transition validity", "History compliance")), j[:120])
    check("implementation section identifies the browser model and its proof boundary",
          "browser model" in page.inner_text("#status") and "no native proof or ledger settlement" in page.inner_text("#status"))
    check("explorer checks reference the four judgments", "contract properties" in page.inner_text(".k-hint-judgments") and "history compliance" in page.inner_text(".k-hint-judgments"))
    check("fixture names concrete asset identities", "fixture-issuer-a" in page.inner_text(".k-fixture-note") and "foreign-illustration" in page.inner_text(".k-fixture-note"))

    # --- disclosures ---------------------------------------------------------------
    check("three disclosures present", page.eval_on_selector_all("details.k-disclosure", "e => e.length") == 3)
    page.click("#solvers details.k-disclosure:nth-of-type(1) summary")
    check("service trace opens", page.is_visible(".k-trace"))
    check("retry keeps request identity", "same R1" in page.inner_text(".k-trace"))

    # --- accessibility basics ------------------------------------------------------
    no_name = page.evaluate("""() => [...document.querySelectorAll('button')]
        .filter(b => !b.textContent.trim() && !b.getAttribute('aria-label')).length""")
    check("every button has a text label", no_name == 0, f"{no_name}")
    trap = page.evaluate("() => [...document.querySelectorAll('[tabindex]')].filter(e => Number(e.getAttribute('tabindex')) > 0).length")
    check("no positive tabindex", trap == 0)
    tables_no_caption = page.evaluate("() => [...document.querySelectorAll('table')].filter(t => !t.caption && !t.querySelector('thead')).length")
    check("every table has a caption or header", tables_no_caption == 0)
    check("result is announced in a status region", page.get_attribute(".k-announce", "role") == "status" and page.get_attribute(".k-announce", "aria-live") == "polite")
    body = page.inner_text("body")
    check("nothing shows a verified badge or claims a native proof", not re.search(r"(✓|✔|proof verified|natively proven|settled on midnight)", body, re.I))
    check("page names itself an illustration", "illustration" in body.lower())

    # --- keyboard --------------------------------------------------------------------
    page.reload(wait_until="networkidle")
    page.keyboard.press("Tab")
    check("first tab stop is the skip link", "skip" in page.evaluate("() => document.activeElement.className"))
    page.keyboard.press("Enter")
    page.wait_for_timeout(80)
    check("skip link reaches main", page.evaluate("() => location.hash") == "#main")
    # Walk by keyboard from the first candidate to a completed first fill.
    page.focus("[data-candidate=route-direct]")
    page.keyboard.press("Enter")
    page.wait_for_timeout(60)
    check("Enter selects a candidate", page.get_attribute("[data-candidate=route-direct]", "aria-pressed") == "true")
    page.focus("[data-event=provider-accepts-duties]")
    page.keyboard.press("Space")
    page.wait_for_timeout(60)
    page.focus("[data-event=commit-candidate]")
    page.keyboard.press("Enter")
    page.wait_for_timeout(60)
    page.focus("[data-evidence=fresh]")
    page.keyboard.press("Enter")
    page.wait_for_timeout(60)
    page.focus("[data-event=finalize-fill]")
    page.keyboard.press("Enter")
    page.wait_for_timeout(60)
    check("keyboard alone reaches a finalized fill", account(page)["grossDebit"] == "5.5 A", account(page)["grossDebit"])
    outline = page.evaluate("""() => { const b = document.querySelector('[data-event=reserve-fill]'); b.focus();
        const s = getComputedStyle(b); return s.outlineStyle + ' ' + s.outlineWidth; }""")
    check("focused control shows a visible outline", "none" not in outline and "0px" not in outline, outline)
    # Tab order: controls precede the account, which follows in document order.
    order = page.evaluate("""() => { const a = document.querySelector('.k-controls'); const l = document.querySelector('.k-ledger');
        return a.compareDocumentPosition(l) & Node.DOCUMENT_POSITION_FOLLOWING ? 'controls-then-ledger' : 'other'; }""")
    check("reading order is controls then account", order == "controls-then-ledger", order)

    # --- widths ----------------------------------------------------------------------
    for w in (320, 400, 768, 1024, 1280):
        page.set_viewport_size({"width": w, "height": 900})
        page.wait_for_timeout(120)
        overflow = page.evaluate("() => document.documentElement.scrollWidth - document.documentElement.clientWidth")
        check(f"no page overflow at {w}px", overflow <= 1, f"{overflow}px")
        wide = page.evaluate(f"""() => [...document.querySelectorAll('main *')]
            .filter(e => e.offsetParent !== null && e.scrollWidth > {w} && !e.closest('.scroll-x') && !e.matches('.scroll-x') && !e.closest('.k-log') && !e.classList.contains('sr-only')
                         && getComputedStyle(e).overflowX !== 'auto')
            .map(e => (e.className || e.tagName) + ':' + e.scrollWidth).slice(0, 4)""")
        check(f"nothing overflows its container at {w}px", not wide, f"{wide}")
        clipped = page.evaluate("""() => { const bar = document.querySelector('.masthead'); const r = bar.getBoundingClientRect();
            return [...bar.querySelectorAll('*')].filter(e => e.offsetParent !== null)
              .filter(e => { const b = e.getBoundingClientRect(); return b.right > r.right + 1 || b.left < r.left - 1; })
              .map(e => (e.className || e.tagName)); }""")
        check(f"masthead fits at {w}px", not clipped, f"{clipped[:3]}")
    page.set_viewport_size({"width": 320, "height": 800})
    page.wait_for_timeout(100)
    click(page, "[data-event=reserve-fill]")
    check("controls work at 320px", account(page)["reserved"] == "5.5 A")
    small = page.evaluate("""() => [...document.querySelectorAll('.k-explorer button')].filter(b => !b.disabled && b.offsetParent)
        .filter(b => b.getBoundingClientRect().height < 24).length""")
    check("touch targets are not tiny at 320px", small == 0, f"{small}")
    page.screenshot(path=f"{OUT}/kernel-320.png", full_page=False)

    # 200% zoom approximated: half-width viewport with doubled device scale factor.
    page.set_viewport_size({"width": 640, "height": 600})
    page.evaluate("() => { document.documentElement.style.fontSize = '32px'; }")
    page.wait_for_timeout(120)
    overflow = page.evaluate("() => document.documentElement.scrollWidth - document.documentElement.clientWidth")
    check("account stays usable at doubled root font size", overflow <= 1 and page.is_visible(".k-account"), f"{overflow}px")
    page.evaluate("() => { document.documentElement.style.fontSize = ''; }")
    page.set_viewport_size({"width": 1280, "height": 900})

    # --- dark, reduced motion, print --------------------------------------------------
    page.emulate_media(color_scheme="dark")
    page.wait_for_timeout(100)
    bg_dark = page.eval_on_selector("body", "e => getComputedStyle(e).backgroundColor")
    check("dark theme repaints the ground", bg_dark != bg and bg_dark != "rgba(0, 0, 0, 0)", bg_dark)
    check("dark theme keeps the account readable", page.is_visible(".k-account"))
    page.screenshot(path=f"{OUT}/kernel-dark.png", full_page=False)
    page.emulate_media(color_scheme="light")
    page.emulate_media(reduced_motion="reduce")
    page.wait_for_timeout(60)
    check("reduced motion disables smooth scrolling", page.eval_on_selector("html", "e => getComputedStyle(e).scrollBehavior") == "auto")
    page.emulate_media(reduced_motion="no-preference")
    page.emulate_media(media="print")
    page.wait_for_timeout(60)
    check("print shows the static transcripts", page.eval_on_selector("#kernel-static", "e => getComputedStyle(e).display") == "block")
    check("print hides the masthead", page.eval_on_selector(".masthead", "e => getComputedStyle(e).display") == "none")
    page.emulate_media(media="screen")

    # --- route links -----------------------------------------------------------------
    hrefs = page.eval_on_selector_all("a[href]", "els => els.map(e => e.getAttribute('href'))")
    for href in sorted(set(hrefs)):
        if href.startswith(("https:", "mailto:")):
            continue
        if href.startswith("#"):
            check(f"anchor {href} exists", page.eval_on_selector_all(href, "e => e.length") == 1)
            continue
        r = page.request.get(BASE + href.split("#")[0])
        check(f"local link resolves: {href}", r.status == 200, f"{r.status}")
    r = page.request.get(BASE + "docs/kernel.html")
    check("existing docs/kernel.html redirect still resolves", r.status == 200 and "requirements.html#architecture" in r.text(), f"{r.status}")
    page.goto(BASE + "index.html", wait_until="networkidle")
    nav = page.get_attribute(".jump a[href='kernel.html']", "href")
    check("homepage navigation links to the kernel page", nav == "kernel.html", f"{nav}")

    check("no console errors after driving the page", not errors, "; ".join(errors[:3]))
    check("no failed requests after driving the page", not failed_requests, "; ".join(failed_requests[:3]))
    ctx.close()

    # --- no-script fallback -------------------------------------------------------------
    nojs = b.new_context(viewport={"width": 1280, "height": 900}, java_script_enabled=False)
    p2 = nojs.new_page()
    p2.goto(PAGE, wait_until="load")
    check("no-JS: article renders with its h1", p2.eval_on_selector_all("h1", "e => e.length") == 1)
    check("no-JS: static transcripts visible", p2.is_visible("#kernel-static") and p2.is_visible("[data-transcript=shared]"))
    check("no-JS: explorer root stays empty", p2.eval_on_selector("#kernel-root", "e => e.children.length") == 0)
    rows = p2.eval_on_selector_all("[data-transcript=shared] tbody tr", "e => e.length")
    branches = p2.eval_on_selector_all("[data-transcript=branches] tbody tr", "e => e.length")
    check("no-JS: shared path and three branches present", rows == 7 and branches == 3, f"{rows} shared, {branches} branches")
    check("no-JS: evidence table visible", p2.is_visible("#kernel-evidence-static"))
    check("no-JS: styles applied", p2.eval_on_selector("body", "e => getComputedStyle(e).backgroundColor") == bg)
    p2.set_viewport_size({"width": 320, "height": 800})
    p2.wait_for_timeout(80)
    ov = p2.evaluate("() => document.documentElement.scrollWidth - document.documentElement.clientWidth")
    check("no-JS: no overflow at 320px", ov <= 1, f"{ov}px")
    p2.screenshot(path=f"{OUT}/kernel-nojs.png", full_page=False)
    nojs.close()

    # --- failed-script fallback: the bundle never loads -------------------------------------
    broken = b.new_context(viewport={"width": 1280, "height": 900})
    p3 = broken.new_page()
    p3.route(re.compile(r".*/assets/kernel-.*\.js$"), lambda route: route.abort())
    p3.goto(PAGE, wait_until="load")
    p3.wait_for_timeout(200)
    check("script load failure: static transcripts remain visible", p3.is_visible("#kernel-static"))
    check("script load failure: root not marked enhanced", p3.get_attribute("#kernel-root", "data-enhanced") is None)
    broken.close()

    # --- failed-script fallback: the bundle loads but throws during initialization -----------
    throwing = b.new_context(viewport={"width": 1280, "height": 900})
    p4 = throwing.new_page()
    p4.route(re.compile(r".*/assets/kernel-.*\.js$"),
             lambda route: route.fulfill(status=200, content_type="text/javascript", body="throw new Error('illustrative initialization failure');"))
    p4.goto(PAGE, wait_until="load")
    p4.wait_for_timeout(200)
    check("initialization failure: static transcripts remain visible", p4.is_visible("#kernel-static"))
    check("initialization failure: evidence table remains visible", p4.is_visible("#kernel-evidence-static"))
    check("initialization failure: root not marked enhanced", p4.get_attribute("#kernel-root", "data-enhanced") is None)
    throwing.close()

    # --- component render failure on first render (audit R3) ---------------------------------
    # The production bundle loads and React schedules the explorer, but a
    # component throws while React creates its DOM. No debug switch exists in
    # the page; the fault is injected by the browser before any script runs.
    FAULT = """(tag) => { const orig = document.createElement.bind(document);
        document.createElement = function (name, ...rest) {
          if (String(name).toLowerCase() === tag) throw new Error('injected render fault: ' + tag);
          return orig(name, ...rest); }; }"""
    rf = b.new_context(viewport={"width": 1280, "height": 900})
    p5 = rf.new_page()
    p5_errors = []
    p5.on("pageerror", lambda e: p5_errors.append(str(e)))
    p5.add_init_script(f"({FAULT})('fieldset')")
    p5.goto(PAGE, wait_until="networkidle")
    p5.wait_for_timeout(300)
    check("render failure: static transcripts restored and visible", p5.is_visible("#kernel-static") and p5.is_visible("[data-transcript=shared]"))
    check("render failure: static block not marked superseded", p5.get_attribute("#kernel-static", "data-superseded") is None)
    check("render failure: root not marked enhanced", p5.get_attribute("#kernel-root", "data-enhanced") is None)
    check("render failure: root left empty", p5.eval_on_selector("#kernel-root", "e => e.children.length") == 0)
    check("render failure: inspector in its own root still works", p5.get_attribute("#kernel-evidence-root", "data-enhanced") == "true" and p5.is_hidden("#kernel-evidence-static"))
    p5.click("[data-mech=tee]")
    check("render failure: inspector still responds", "measured code" in p5.inner_text(".k-insp-claim"))
    p5.emulate_media(media="print")
    check("render failure: print shows the full static fallback", p5.eval_on_selector("#kernel-static", "e => getComputedStyle(e).display") == "block")
    p5.emulate_media(media="screen")
    p5.screenshot(path=f"{OUT}/kernel-render-failure.png", full_page=False)
    rf.close()

    # --- component render failure on a later update (audit R3) ------------------------------
    # The explorer mounts and hides the fallback. A fault is then injected so
    # that the next state change, which first renders a <code> element for a
    # consumed identifier, throws during React's update. The fallback must come back.
    uf = b.new_context(viewport={"width": 1280, "height": 900})
    p6 = uf.new_page()
    p6.goto(PAGE, wait_until="networkidle")
    check("update failure: explorer mounted first", p6.get_attribute("#kernel-root", "data-enhanced") == "true" and p6.is_hidden("#kernel-static"))
    p6.click("[data-candidate=route-direct]")
    p6.click("[data-event=provider-accepts-duties]")
    p6.click("[data-event=commit-candidate]")
    p6.click("[data-evidence=fresh]")
    p6.wait_for_timeout(60)
    check("update failure: no <code> element rendered yet", p6.eval_on_selector_all("#kernel-root code", "e => e.length") == 0)
    p6.evaluate(FAULT, "code")
    p6.click("[data-event=finalize-fill]")
    p6.wait_for_timeout(300)
    check("update failure: static transcripts restored", p6.is_visible("#kernel-static") and p6.get_attribute("#kernel-static", "data-restored") == "true")
    check("update failure: root un-enhanced and emptied", p6.get_attribute("#kernel-root", "data-enhanced") is None and p6.eval_on_selector("#kernel-root", "e => e.children.length") == 0)
    check("update failure: inspector unaffected", p6.get_attribute("#kernel-evidence-root", "data-enhanced") == "true")
    p6.emulate_media(media="print")
    check("update failure: print shows the full static fallback", p6.eval_on_selector("#kernel-static", "e => getComputedStyle(e).display") == "block")
    p6.emulate_media(media="screen")
    uf.close()

    b.close()

passed = sum(1 for _, ok, _ in results if ok)
print(f"\n{passed}/{len(results)} passed")
if errors:
    print("\nconsole output captured:")
    for e in errors[:10]:
        print("  ", e)
sys.exit(0 if passed == len(results) else 1)
