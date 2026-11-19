import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {copyFileSync, existsSync, mkdirSync, readFileSync, symlinkSync, writeFileSync} from 'node:fs';
import {join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';

const pkg = fileURLToPath(new URL('../',import.meta.url));
const language = resolve(pkg,'../../experiments/moriarty-language');
const hash = bytes => createHash('sha256').update(bytes).digest('hex');

export function buildO5(output, {skipZk = false} = {}) {
  const source = readFileSync(join(pkg,'orders/o5.mori'));
  const generated = lowerCompact(source,readFileSync(join(language,'spec/bounds.json')));
  const stage = join(output,'source');
  mkdirSync(stage,{recursive:true});
  writeFileSync(join(stage,'kernel.compact'),generated.source);
  const arithmetic = readFileSync(join(language,'compact/arithmetic.compact'),'utf8');
  const pragma = 'pragma language_version >= 0.23 && <= 0.23;';
  if (!arithmetic.startsWith(pragma)) throw Error('Unexpected Compact arithmetic source version');
  writeFileSync(join(stage,'arithmetic.compact'),arithmetic.replace(pragma,'pragma language_version >= 0.23 && <= 0.28;'));
  for (const name of ['o5.compact','feed-policy.compact']) copyFileSync(join(pkg,'contracts',name),join(stage,name));
  const selector = process.env.COMPACTC ? [] : ['compile','+0.36.0'];
  const compiler = process.env.COMPACTC || 'compact';
  const compilerVersion = execFileSync(compiler,[...selector,'--version'],{encoding:'utf8'}).trim();
  if (!compilerVersion.startsWith('0.36.0')) throw Error('O5 requires Compact 0.36.0');
  execFileSync(compiler,[...selector,...(skipZk?['--skip-zk']:[]),join(stage,'o5.compact'),output],{stdio:'inherit'});
  const modules = resolve(process.env.MORIARTY_ORDER_MODULES || join(pkg,'node_modules'));
  if (!existsSync(join(output,'node_modules'))) symlinkSync(modules,join(output,'node_modules'),'dir');
  const verifiers = {};
  const kernels = {};
  for (const op of ['fund','fill','cancel','publishFeedUpdate']) {
    const ir = readFileSync(join(output,'zkir',op+'.zkir'));
    if (JSON.parse(ir).version.major !== 3) throw Error('Expected ZKIRv3 for '+op);
    kernels[op] = hash(ir);
    if (!skipZk) verifiers[op] = hash(readFileSync(join(output,'keys',op+'.verifier')));
  }
  const pins = {programHash:generated.metadata.programHash,sourceSha256:hash(source),kernelSha256:hash(generated.source),
    observations:generated.metadata.observationFields,compilerVersion,kernels,verifiers,
    sourceFiles:Object.fromEntries(['kernel.compact','arithmetic.compact','o5.compact','feed-policy.compact'].map(name=>[name,hash(readFileSync(join(stage,name)))]))};
  writeFileSync(join(output,'o5-build.json'),JSON.stringify(pins,null,2)+'\n');
  return pins;
}
if (process.argv[1] && resolve(process.argv[1])===fileURLToPath(import.meta.url)) {
  if (!process.argv[2]) throw Error('usage: node build-o5.mjs OUTPUT [--skip-zk]');
  buildO5(resolve(process.argv[2]),{skipZk:process.argv.includes('--skip-zk')});
}
