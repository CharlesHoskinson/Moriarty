import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import katex from 'katex';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const source = path.join(root, 'docs');
const output = path.join(root, 'public/docs');
const pages = [['requirements','Requirements'],['language','Syntax &amp; semantics']];
const decode = value => value.replace(/&quot;/g,'"').replace(/&#x27;/g,"'").replace(/&lt;/g,'<').replace(/&gt;/g,'>').replace(/&amp;/g,'&');
await mkdir(output,{recursive:true});
let mathCount=0;
for (const [name,title] of pages) {
 let body=await readFile(path.join(source,`${name}.html`),'utf8');
 if ((body.match(/<h1[ >]/g)||[]).length !== 1) throw new Error(`${name}: expected one h1`);
 body=body.replace(/<(div|span) class="math-(block|inline)" data-tex="([^"]*)"><\/\1>/g,(_,tag,kind,tex)=>{
  mathCount++;
  return `<${tag} class="math-${kind}">${katex.renderToString(decode(tex),{displayMode:kind==='block',output:'mathml',throwOnError:true,strict:'error'})}</${tag}>`;
 });
 for (const [,href] of body.matchAll(/href="([^"#]+)(?:#[^"]*)?"/g)) {
  if (!/^(?:https?:|mailto:|\.\.\/)/.test(href) && href !== 'requirements-source.json' && !pages.some(([p])=>href===`${p}.html`)) throw new Error(`${name}: unrecognized local link ${href}`);
 }
 const nav=pages.map(([p,t])=>`<a href="${p}.html"${p===name?' aria-current="page"':''}>${t}</a>`).join('');
 await writeFile(path.join(output,`${name}.html`),`<!doctype html>\n<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><meta name="description" content="Moriarty language reference and proposed native backend requirements."><title>${title} · Moriarty</title><link rel="stylesheet" href="styles.css"></head><body><a class="skip" href="#content">Skip to content</a><header><a class="brand" href="../index.html">Moriarty</a><span>Language reference &amp; backend design</span></header><nav aria-label="Documentation">${nav}</nav><main id="content">${body}</main><footer>Research documentation · Scoped implementation and proposed requirements are distinguished throughout. <a href="https://github.com/CharlesHoskinson/Moriarty">Source repository</a></footer></body></html>\n`);
}
await writeFile(path.join(output,'styles.css'),await readFile(path.join(source,'styles.css')));
console.log(`Built ${pages.length} documentation pages; rendered ${mathCount} mathematical expressions as native MathML.`);

await writeFile(path.join(output,'requirements-source.json'),await readFile(path.join(source,'requirements-source.json')));
const aliases = {index:'requirements.html',syntax:'language.html#syntax',semantics:'language.html#semantics',kernel:'requirements.html#architecture','zkirv4-requirements':'requirements.html#zkir',recursion:'requirements.html#recursion'};
for (const [alias,target] of Object.entries(aliases)) {
 await writeFile(path.join(output,`${alias}.html`),`<!doctype html><html lang="en"><head><meta charset="utf-8"><meta http-equiv="refresh" content="0;url=${target}"><title>Moriarty reference</title></head><body><p>This reference has moved to <a href="${target}">${target}</a>.</p></body></html>`);
}
