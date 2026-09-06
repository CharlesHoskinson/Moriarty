#!/usr/bin/env python3
"""Browser smoke for the executable R2 workspace."""

import json
import os
from urllib.parse import urlparse

from playwright.sync_api import sync_playwright


BASE = os.environ.get("MORIARTY_R2_URL", "http://127.0.0.1:4174").rstrip("/")


def expect_text(page, selector, text):
    page.locator(selector).wait_for(state="visible")
    locator = page.locator(selector)
    actual = locator.input_value() if locator.evaluate("node => node.matches('textarea, input')") else locator.inner_text()
    assert text in actual, (selector, actual)


with sync_playwright() as playwright:
    browser = playwright.chromium.launch()
    context = browser.new_context(accept_downloads=True)
    page = context.new_page()
    outside = []

    def request_seen(request):
        parsed = urlparse(request.url)
        base = urlparse(BASE)
        if parsed.hostname != base.hostname or parsed.port != base.port:
            outside.append(request.url)

    page.on("request", request_seen)
    page.goto(f"{BASE}/language", wait_until="networkidle")
    expect_text(page, "h1", "Run a bounded agreement")
    expect_text(page, "#source-status", "Elaborated")
    expect_text(page, "#quantization-note", "33.972602")

    page.locator("#evaluate").click()
    expect_text(page, "#evaluation-status", "Evaluated")
    expect_text(page, "#after-state", '"principalDue": "500000000"')
    expect_text(page, "#after-state", '"interestDue": "33972602"')
    original_action = page.locator("#action-json").input_value()
    page.locator("#action-json").fill(original_action + " ")
    expect_text(page, "#after-state", "—")
    expect_text(page, "#effects-json", "—")
    page.locator("#reset-example").click()
    page.locator("#evaluate").click()
    original_policy = page.locator("#policy-json").input_value()
    page.locator("#policy-json").fill(original_policy + " ")
    expect_text(page, "#after-state", "—")
    expect_text(page, "#effects-json", "—")
    page.locator("#reset-example").click()
    source = json.loads(page.locator("#source-json").input_value())
    source["terms"]["notional"] = "1000000000"
    page.locator("#source-json").fill(json.dumps(source, indent=2))
    assert page.locator("#source-json").evaluate("node => document.activeElement === node")
    expect_text(page, "#current-state", "—")
    expect_text(page, "#core-json", "—")
    page.locator("#elaborate").click()
    expect_text(page, "#quantization-note", "6794520 micro-USD")
    assert "33.972602" not in page.locator("#quantization-note").inner_text()
    page.locator("#reset-example").click()
    page.locator("#evaluate").click()
    page.locator("#check-policy").click()
    expect_text(page, "#policy-status", "Local intent/effect check passed")
    page.locator("#advance-local").click()
    expect_text(page, "#simulation-status", "Local simulation advanced")
    expect_text(page, "#action-json", '"name": "settle"')
    expect_text(page, "#action-json", '"amount": "533972602"')
    page.locator("#evaluate").click()
    page.locator("#check-policy").click()
    page.locator("#advance-local").click()
    expect_text(page, "#current-state", '"closed": "1"')
    expect_text(page, "#current-state", '"principalPaid": "500000000"')

    page.locator("#reset-example").click()
    page.locator("#try-profile-failure").click()
    page.locator("#evaluate").click()
    expect_text(page, "#evaluation-status", "GUARD_FAILED")
    page.locator("#reset-example").click()
    page.locator("#try-overflow").click()
    page.locator("#evaluate").click()
    expect_text(page, "#evaluation-status", "OVERFLOW")
    page.locator("#reset-example").click()
    page.locator("#try-malformed").click()
    page.locator("#evaluate").click()
    expect_text(page, "#evaluation-status", "INVALID_JSON")

    page.locator("#example").select_option("swap")
    page.locator("#evaluate").click()
    expect_text(page, "#evaluation-status", "Evaluated")
    expect_text(page, "#effects-json", '"amount": "19743"')
    action = json.loads(page.locator("#action-json").input_value())
    action["args"]["minOut"] = "19744"
    page.locator("#action-json").fill(json.dumps(action, indent=2))
    expect_text(page, "#evaluation-status", "Not evaluated")
    page.locator("#evaluate").click()
    expect_text(page, "#evaluation-status", "GUARD_FAILED")

    action["args"]["minOut"] = "19743"
    page.locator("#action-json").fill(json.dumps(action, indent=2))
    page.locator("#evaluate").click()
    policy = json.loads(page.locator("#policy-json").input_value())
    policy["minimumCredits"][0]["minAmount"] = "19744"
    page.locator("#policy-json").fill(json.dumps(policy, indent=2))
    page.locator("#evaluate").click()
    page.locator("#check-policy").click()
    expect_text(page, "#policy-status", "MinimumCreditNotMet")

    page.locator("#reset-example").click()
    page.locator("#evaluate").click()
    page.locator("#check-policy").click()
    page.locator("#sign-local").click()
    page.locator("#evaluate").click()
    page.wait_for_timeout(100)
    expect_text(page, "#signature-status", "No current signature")
    page.locator("#check-policy").click()
    signature_skipped = False
    try:
        page.locator("#sign-local").click()
        page.locator("#signature-status").wait_for(state="visible")
        page.wait_for_function("document.querySelector('#signature-status').textContent.includes('verified') || document.querySelector('#signature-status').textContent.includes('unavailable')")
        signature_text = page.locator("#signature-status").inner_text()
        if "unavailable" in signature_text.lower():
            signature_skipped = True
            print(f"SKIP signature: {signature_text}")
        else:
            assert "Ed25519 signature verified" in signature_text
            page.locator("#verify-claims").click()
            expect_text(page, "#claims-status", "Real PCD acceptance unavailable")
    except Exception as error:
        raise AssertionError(f"signature check did not complete or report an explicit skip: {error}") from error

    if not signature_skipped:
        page.locator("#action-json").fill(page.locator("#action-json").input_value() + " ")
        expect_text(page, "#signature-status", "No current signature")

    page.locator("#reset-example").click()
    page.locator("#evaluate").click()
    page.locator("#check-policy").click()
    if not signature_skipped:
        page.locator("#sign-local").click()
        page.wait_for_function("document.querySelector('#signature-status').textContent.includes('verified')")
        with page.expect_download() as download_info:
            page.locator("#export-plan").click()
        export = json.loads(open(download_info.value.path(), encoding="utf-8").read())
        assert "privateKey" not in json.dumps(export)

    page.set_viewport_size({"width": 360, "height": 780})
    page.reload(wait_until="networkidle")
    assert page.evaluate("document.documentElement.scrollWidth <= document.documentElement.clientWidth")
    assert not outside, outside
    browser.close()

print("language browser smoke passed")
