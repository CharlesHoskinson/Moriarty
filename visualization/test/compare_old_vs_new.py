#!/usr/bin/env python3
import http.server
import json
import os
import posixpath
import threading
import urllib.parse
from playwright.sync_api import sync_playwright

DIST_ROOT = "/home/charles/repos/Moriarty/site/dist"
STATIC_PORT = 8897
WORKBENCH_URL = "http://127.0.0.1:4377/"
GHPAGES_URL = f"http://127.0.0.1:{STATIC_PORT}/Moriarty/visualization/index.html"


class SubpathStaticHandler(http.server.SimpleHTTPRequestHandler):
    def translate_path(self, path: str) -> str:
        parsed = urllib.parse.urlsplit(path).path
        rel = urllib.parse.unquote(parsed)
        if rel.startswith("/Moriarty/"):
            rel = rel[len("/Moriarty"):]
        elif rel == "/Moriarty":
            rel = "/index.html"
        if rel.endswith("/"):
            rel += "index.html"
        clean = posixpath.normpath(rel).lstrip("/")
        return os.path.join(DIST_ROOT, clean)

    def log_message(self, format: str, *args) -> None:
        pass


def inspect_all_presets_and_lenses(page, url: str, is_static_host: bool):
    console_errors = []
    page_errors = []
    failed_requests = []
    post_requests = []

    page.route("**/*fonts.g*", lambda route: route.abort())
    page.on("console", lambda msg: console_errors.append(msg.text) if msg.type == "error" and "ERR_FAILED" not in msg.text else None)
    page.on("pageerror", lambda err: page_errors.append(str(err)))
    page.on("request", lambda req: post_requests.append(req.url) if req.method == "POST" else None)
    page.on("response", lambda resp: failed_requests.append(f"{resp.status} {resp.url}") if resp.status >= 400 else None)

    page.goto(url, wait_until="domcontentloaded")
    page.wait_for_function("() => window.__sherlockApp?.state?.trace?.steps?.length > 0")

    preset_ids = [
        "factorial",
        "list_fold",
        "tree_fold",
        "mutual_even_odd",
        "lexical_let",
        "checked_underflow",
        "stack_overflow",
        "fuel_exhaustion",
    ]

    presets_snapshot = {}
    for pid in preset_ids:
        page.click(f'button.preset-btn[data-preset-id="{pid}"]', force=True)
        page.wait_for_function(
            """(expectedId) =>
                window.__sherlockApp?.state?.activePresetId === expectedId &&
                window.__sherlockApp?.state?.trace?.steps?.length > 0 &&
                window.__sherlockApp?.state?.appliedRequestSeq === window.__sherlockApp?.state?.requestSeq
            """,
            arg=pid,
        )
        snap = page.evaluate(
            """() => {
                const st = window.__sherlockApp.state;
                const tr = st.trace;
                return {
                    presetId: st.activePresetId,
                    programName: tr.programName,
                    stepCount: tr.steps.length,
                    outcomeKind: tr.outcome.kind,
                    terminalRule: tr.steps[tr.steps.length - 1]?.ruleId,
                    qdlMorphHtml: document.getElementById('telemetryQdlMorph')?.textContent?.trim() || '',
                };
            }"""
        )
        presets_snapshot[pid] = snap

    lenses = page.evaluate(
        """() => {
            const btns = Array.from(document.querySelectorAll('#cameraLensBar button'));
            const visited = [];
            for (const b of btns) {
                b.click();
                visited.push(b.dataset.preset);
            }
            return visited;
        }"""
    )

    if is_static_host:
        assert len(post_requests) == 0, f"Expected 0 POST requests on static host, got: {post_requests}"
    assert len(console_errors) == 0, f"Console errors on {url}: {console_errors}"
    assert len(page_errors) == 0, f"Page errors on {url}: {page_errors}"
    assert len(failed_requests) == 0, f"Failed requests on {url}: {failed_requests}"

    return {"presetsSnapshot": presets_snapshot, "lenses": lenses}


def main():
    httpd = http.server.ThreadingHTTPServer(("127.0.0.1", STATIC_PORT), SubpathStaticHandler)
    thread = threading.Thread(target=httpd.serve_forever, daemon=True)
    thread.start()

    try:
        chrome_bin = "/home/charles/.cache/ms-playwright/chromium-1243/chrome-linux64/chrome"
        launch_kwargs = {
            "headless": True,
            "args": ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-webgl"],
        }
        if os.path.exists(chrome_bin):
            launch_kwargs["executable_path"] = chrome_bin
        with sync_playwright() as p:
            browser = p.chromium.launch(**launch_kwargs)
            page_old = browser.new_page(viewport={"width": 1440, "height": 900})
            old_res = inspect_all_presets_and_lenses(page_old, WORKBENCH_URL, is_static_host=False)
            page_old.close()

            page_new = browser.new_page(viewport={"width": 1440, "height": 900})
            new_res = inspect_all_presets_and_lenses(page_new, GHPAGES_URL, is_static_host=True)
            page_new.close()

            assert new_res["lenses"] == old_res["lenses"], "Camera lens presets must match identically"

            for pid, old_snap in old_res["presetsSnapshot"].items():
                new_snap = new_res["presetsSnapshot"][pid]
                assert new_snap["programName"] == old_snap["programName"], f"programName mismatch on {pid}"
                assert new_snap["stepCount"] == old_snap["stepCount"], f"stepCount mismatch on {pid}"
                assert new_snap["outcomeKind"] == old_snap["outcomeKind"], f"outcomeKind mismatch on {pid}"
                assert new_snap["terminalRule"] == old_snap["terminalRule"], f"terminalRule mismatch on {pid}"
                assert new_snap["qdlMorphHtml"] == old_snap["qdlMorphHtml"], f"qdlMorphHtml mismatch on {pid}"

            print("PASS: Old workbench (127.0.0.1:4377) and New GitHub Pages build (/Moriarty/visualization/index.html) match 100% across all 8 presets and all camera lenses with 0 errors!")
            print(json.dumps(new_res["presetsSnapshot"], indent=2))
            browser.close()
    finally:
        httpd.shutdown()
        httpd.server_close()


if __name__ == "__main__":
    main()
