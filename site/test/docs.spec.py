"""Check the built static reference: navigation, MathML, narrow layouts and no scripts."""
import os
from pathlib import Path
from playwright.sync_api import sync_playwright
base=os.environ.get('URL','http://localhost:8891/').rstrip('/')+'/docs/'
pages=['requirements','language']
with sync_playwright() as p:
 browser=p.chromium.launch(executable_path=os.environ.get('CHROME') or None)
 page=browser.new_page()
 for width in [375,1280]:
  page.set_viewport_size({'width':width,'height':900})
  for name in pages:
   response=page.goto(base+name+'.html',wait_until='networkidle')
   assert response.status==200,(name,response.status)
   assert page.locator('h1').count()==1,name
   assert page.locator('nav[aria-label="Documentation"] a').count()==2,name
   assert page.locator('nav a[aria-current="page"]').count()==1,name
   assert page.locator('script').count()==0,name
   assert page.evaluate('document.documentElement.scrollWidth <= window.innerWidth'),(name,width,'horizontal overflow')
   assert page.locator('[data-tex]').count()==0,name
   for link in page.locator('a').all():
    href=link.get_attribute('href')
    if href.startswith(('https:','mailto:')):continue
    if href.startswith('#'):
     assert page.locator(href).count(),(name,href)
     continue
    target=page.url.rsplit('/',1)[0]+'/'+href
    result=page.request.get(target.split('#')[0]);assert result.status==200,(name,href,result.status)
   if name=='language': assert page.locator('math').count()>80
   if name=='requirements':
    for n in range(1,17):assert page.locator(f'#zr{n:02}').count()==1
   if name=='requirements':
    for n in range(1,9):assert page.locator(f'#mnr{n:02}').count()==1
   if name=='requirements':
    assert page.locator('section.requirement').count()==59
    for n in range(1,36): assert page.locator(f'#mplr-{n:03} > p').count()==1
    for section in page.locator('section.requirement').all(): assert section.locator(':scope > p').count()==1
   print('PASS',width,name)
 page.goto(base+'language.html'); page.screenshot(path=os.environ.get('DOCS_SHOT','/tmp/moriarty-semantics.png'),full_page=False)
 browser.close()
print('PASS static reference browser checks')
