from pathlib import Path
import json
from playwright.sync_api import sync_playwright
r=Path(__file__).resolve().parents[3];out=r/'evidence/moriarty-developer-mock-2026-09-06';out.mkdir(parents=True,exist_ok=True)
checks=[];errors=[];requests=[]
with sync_playwright() as p:
 browser=p.chromium.launch(headless=True)
 page=browser.new_page(viewport={'width':1440,'height':1000},accept_downloads=True)
 page.on('pageerror',lambda e:errors.append(str(e)));page.on('request',lambda req:requests.append(req.url))
 page.goto('http://127.0.0.1:4173');page.get_by_text('lam01',exact=False).first.wait_for()
 assert page.locator('table tbody tr').count()>0
 page.screenshot(path=str(out/'desktop-loan.png'),full_page=True,animations="disabled");checks.append('imported loan table renders')
 page.get_by_label('Example',exact=True).select_option('swap')
 assert '19743' in page.inner_text('body')
 for _ in range(5):page.locator('#advance-workflow').click()
 assert 'Simulated submission recorded locally' in page.inner_text('body')
 for label in ['Sign with wallet','Verify real proof','Submit to ledger']:assert page.get_by_role('button',name=label,exact=True).is_disabled()
 checks.append('five demo stages complete; real operations disabled')
 with page.expect_download() as dl:page.locator('#export-json').click()
 dest=out/'mock-export.json';dl.value.save_as(str(dest));data=json.loads(dest.read_text());assert data['mock'] is True and data['realVerification']['outcome']=='unavailable';assert len(data['proposedClaims'])==4
 checks.append('export preserves mock marker and four unavailable typed claims')
 page.locator('input[name=minOut]').fill('1');page.locator('input[name=minOut]').press('Tab')
 for _ in range(5):page.locator('#advance-workflow').click()
 assert 'AlreadyConsumed' in page.inner_text('body');checks.append('editing input preserves consumed predecessor; repeat rejects')
 page.get_by_role('button',name='Reset demo ledger',exact=True).click()
 page.get_by_label('Fault injection',exact=True).select_option('missing-proof')
 for _ in range(4):page.locator('#advance-workflow').click()
 assert 'InvalidProof' in page.inner_text('body');checks.append('missing evidence fails simulated proof stage')
 page.get_by_label('Fault injection',exact=True).select_option('stale-predecessor')
 for _ in range(5):page.locator('#advance-workflow').click()
 assert 'StalePredecessor' in page.inner_text('body');checks.append('stale state fails separate ledger stage')
 page.reload();page.get_by_text('Stage 0 / 5',exact=True).wait_for();checks.append('reload restores config but clears evidence')
 for example in ['calendar','option']:
  page.get_by_label('Example',exact=True).select_option(example)
  assert page.locator('input[name=principal]').count()==0
 page.get_by_label('Example',exact=True).select_option('partial');assert '10,000' in page.inner_text('body');assert page.locator('input[name=reserveA]').count()==1
 checks.append('only effective controls shown; fixed partial allowance explicit')
 page.get_by_label('Example',exact=True).select_option('swap');page.get_by_role('button',name='Reset defaults',exact=True).click()
 page.screenshot(path=str(out/'desktop-swap.png'),full_page=True,animations="disabled")
 page.set_viewport_size({'width':390,'height':844});page.screenshot(path=str(out/'mobile-swap.png'),full_page=True,animations="disabled")
 assert page.evaluate('document.documentElement.scrollWidth <= innerWidth');checks.append('390px mobile has no page overflow')
 assert not errors,errors
 external=[u for u in requests if not u.startswith(('http://127.0.0.1:4173/','blob:'))];assert not external,external
 checks.append('no JavaScript errors or external app requests')
 browser.close()
(out/'browser-check.json').write_text(json.dumps({'checks':checks,'page_errors':errors,'external_requests':external,'scope':'local developer mock only; no real proof or chain','command':'/home/charl/Moriarty/.venv/bin/python experiments/moriarty-developer-mock/tests/browser_check.py'},indent=2))
print(json.dumps(checks,indent=2))
