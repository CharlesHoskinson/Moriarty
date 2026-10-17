import {readFile,writeFile} from 'node:fs/promises';
import path from 'node:path';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
const app=path.dirname(fileURLToPath(import.meta.url));
const root=path.resolve(app,'..');
const html=await readFile(path.join(root,'index.html'),'utf8');
const lock=JSON.parse(await readFile(path.join(app,'package-lock.json'),'utf8'));
const registry=JSON.parse(await readFile(path.join(root,'DEPENDENCY-REGISTRY-PINS.json'),'utf8'));
const fixtures=JSON.parse(await readFile(path.join(root,'contributions/04-scenarios/invoice-fixtures.json'),'utf8'));
const checks=[];
const check=(name,pass)=>{checks.push({name,result:pass?'PASS':'FAIL'});if(!pass)throw new Error(name)};
const scriptStart=html.indexOf('<script>');
const scriptEnd=html.indexOf('</script>',scriptStart);
const outsideScript=html.slice(0,scriptStart)+html.slice(scriptEnd+9);
check('one inline script',scriptStart>=0&&scriptEnd>scriptStart&&!/<script(?:\s[^>]*)?>/i.test(outsideScript));
check('no script src or external style link',!/<script[^>]+src=|<link[^>]+rel=["']stylesheet/i.test(html));
check('no remote asset attributes',!/(?:src|href)=["'](?:https?:|\/\/|file:)/i.test(html));
check('no absolute local path strings',!html.includes('/home/charl'));
check('third-party license notices',html.includes('Third-party license notices')&&html.includes('MIT License'));
for(const x of registry.packages){const y=lock.packages['node_modules/'+x.package];check(`${x.package} exact registry version/integrity`,y.version===x.version&&y.integrity===x.distIntegrity)}
check('strict slider range',fixtures.partial.slider.min===1&&fixtures.partial.slider.max===99&&fixtures.partial.slider.default===35);
const expected={'source110-amount35':[73,65,10,75,2],'proposed112-amount35':[75,65,10,75,0]};
for(const [id,values] of Object.entries(expected)){const f=fixtures.fixtures.find(x=>x.id===id),a=f.arithmetic;check(`${id} frozen independent expected values`,JSON.stringify([a.closingCash,a.closingInvoiceDuty,a.closingReserveDuty,a.totalClosingDuties,a.backingGap])===JSON.stringify(values))}
const report={format:'moriarty-webs-static-artifact-check/1',checkedAt:new Date().toISOString(),artifactSha256:createHash('sha256').update(html).digest('hex'),scope:'Static packaging and frozen fixture checks. No browser/proof/source-contract/ledger behavior is established.',checks};
await writeFile(path.join(app,'ARTIFACT-CHECK.json'),JSON.stringify(report,null,2)+'\n');
console.log(JSON.stringify({result:'PASS',checks:checks.length,scope:report.scope},null,2));
