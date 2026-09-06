import json
import os
from pathlib import Path

from playwright.sync_api import sync_playwright


URL = os.environ.get("MORIARTY_OUTCOME_URL", "http://127.0.0.1:4175")


def status(page, selector):
    return page.locator(selector).inner_text().lower()


def wait_status(page, selector, fragment):
    page.wait_for_function(
        "([selector, fragment]) => document.querySelector(selector)?.textContent.toLowerCase().includes(fragment)",
        arg=[selector, fragment.lower()],
    )


def main():
    errors = []
    requests = []
    out = Path("test-output")
    out.mkdir(exist_ok=True)
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page(accept_downloads=True)
        page.on("pageerror", lambda error: errors.append(str(error)))
        page.on("request", lambda request: requests.append(request.url))
        page.goto(URL + "/intents")
        page.wait_for_selector("#intent-json")

        # Outcome authority is signed before either concrete pool plan is chosen.
        page.click("#sign-intent")
        wait_status(page, "#signature-status", "signed outcome")
        signature = page.locator("#signed-intent").inner_text()
        page.click("#propose-route-0")
        wait_status(page, "#plan-status", "proposed")
        page.click("#preview-plan")
        wait_status(page, "#preview-status", "eligible")

        # Hold proposal hashing open: the prior route must disappear and execution must lock.
        page.evaluate("""
          () => {
            const original = crypto.subtle.digest.bind(crypto.subtle);
            let release;
            crypto.subtle.digest = (...args) => new Promise(resolve => {
              release = () => original(...args).then(resolve);
            });
            window.__releaseOutcomeDigest = () => {
              crypto.subtle.digest = original;
              release();
            };
          }
        """)
        page.click("#propose-route-1")
        assert page.locator("#plan-json").input_value() == ""
        assert page.locator("#simulate-plan").is_disabled()
        assert "proposal pending" in status(page, "#preview-status")
        page.evaluate("window.__releaseOutcomeDigest()")
        wait_status(page, "#plan-status", "route 1")
        assert page.locator("#signed-intent").inner_text() == signature
        assert "not checked" in status(page, "#preview-status")
        page.click("#preview-plan")
        wait_status(page, "#preview-status", "eligible")
        page.click("#verify-real")
        wait_status(page, "#claims-status", "real pcd acceptance unavailable")
        assert "ContractInvariant" in page.locator("#claims-status").inner_text()
        with page.expect_download() as pre_download:
            page.click("#export-outcome")
        pre_destination = out / "outcome-before-simulation.json"
        pre_download.value.save_as(str(pre_destination))
        assert json.loads(pre_destination.read_text())["snapshot"]["consumed"] == 0

        page.evaluate("""
          () => {
            const original = crypto.subtle.verify.bind(crypto.subtle);
            let release;
            crypto.subtle.verify = (...args) => new Promise(resolve => {
              release = () => original(...args).then(resolve);
            });
            window.__releaseOutcomeVerify = () => {
              crypto.subtle.verify = original;
              release();
            };
          }
        """)
        page.click("#simulate-plan")
        assert page.locator("#intent-json").is_disabled()
        assert page.locator("#propose-route-0").is_disabled()
        assert page.locator("#simulate-plan").is_disabled()
        assert "pending" in status(page, "#simulation-status")
        page.evaluate("window.__releaseOutcomeVerify()")
        wait_status(page, "#simulation-status", "simulated complete")
        page.click("#simulate-plan")
        wait_status(page, "#simulation-status", "rejected")

        # Source edits synchronously clear every stale artifact and malformed JSON stays visible.
        intent = page.locator("#intent-json")
        stricter = json.loads(intent.input_value())
        stricter["authority"][0]["maxDebit"] = "1"
        stricter["nonce"] = "1"
        intent.fill(json.dumps(stricter, indent=2))
        assert page.locator("#signed-intent").inner_text() == "—"
        page.click("#sign-intent")
        wait_status(page, "#signature-status", "signed outcome")
        page.click("#propose-route-0")
        wait_status(page, "#plan-status", "proposed route 0")
        page.click("#preview-plan")
        wait_status(page, "#preview-status", "rejected")
        assert "GrossDebitExceeded" in page.locator("#preview-status").inner_text()
        intent.fill("{ malformed")
        assert "invalid json" in status(page, "#intent-status")
        assert page.locator("#signed-intent").inner_text() == "—"
        assert page.locator("#plan-json").input_value() == ""
        page.click("#export-outcome")
        assert "export rejected" in status(page, "#intent-status")
        assert "fail" in page.locator("#intent-status").get_attribute("class")

        # A fresh loan demo uses the same authority flow and settles its due state locally.
        page.evaluate("""
          () => {
            const original = crypto.subtle.digest.bind(crypto.subtle);
            let release;
            crypto.subtle.digest = (...args) => new Promise(resolve => {
              release = () => original(...args).then(resolve);
            });
            window.__releaseDemoDigest = () => {
              crypto.subtle.digest = original;
              release();
            };
          }
        """)
        page.select_option("#example", "loan")
        assert page.locator("#example").is_disabled()
        assert page.locator("#intent-json").is_disabled()
        assert page.locator("#signed-intent").inner_text() == "—"
        assert page.locator("#plan-json").input_value() == ""
        page.evaluate("window.__releaseDemoDigest()")
        page.wait_for_function("document.querySelector('#intent-status')?.textContent.includes('Valid editable')")
        page.click("#sign-intent")
        wait_status(page, "#signature-status", "signed outcome")
        page.click("#propose-route-0")
        wait_status(page, "#plan-status", "proposed route 0")
        page.click("#simulate-plan")
        wait_status(page, "#simulation-status", "simulated complete")
        assert "DueSettled" in page.locator("#receipt-json").inner_text()

        # Time advances monotonically; expiry is a visible fail-closed rejection.
        page.select_option("#example", "swap")
        page.wait_for_function("document.querySelector('#intent-status')?.textContent.includes('Valid editable')")
        page.click("#sign-intent")
        wait_status(page, "#signature-status", "signed outcome")
        page.click("#propose-route-0")
        wait_status(page, "#plan-status", "proposed route 0")
        page.fill("#logical-time", "200")
        page.click("#advance-time")
        page.click("#preview-plan")
        wait_status(page, "#preview-status", "expired")

        # Export includes public evidence, never a private key.
        page.select_option("#example", "swap")
        page.wait_for_function("document.querySelector('#intent-status')?.textContent.includes('Valid editable')")
        page.click("#sign-intent")
        wait_status(page, "#signature-status", "signed outcome")
        page.click("#propose-route-0")
        wait_status(page, "#plan-status", "proposed route 0")
        with page.expect_download() as download:
            page.click("#export-outcome")
        destination = out / "outcome-export.json"
        download.value.save_as(str(destination))
        payload = destination.read_text()
        assert "privateKey" not in payload and "private-key" not in payload
        assert json.loads(payload)["signedIntent"]["kind"] == "SignedOutcomeIntent"

        mobile = browser.new_page(viewport={"width": 360, "height": 780})
        mobile.goto(URL + "/intents")
        mobile.wait_for_selector("#intent-json")
        assert mobile.evaluate("document.documentElement.scrollWidth <= document.documentElement.clientWidth")
        browser.close()

    assert not errors, errors
    assert all(url.startswith(URL) or url.startswith("blob:") for url in requests), requests
    print("Outcome intent browser smoke passed")


if __name__ == "__main__":
    main()
