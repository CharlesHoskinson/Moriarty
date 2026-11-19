import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {copyFileSync,existsSync,mkdirSync,readFileSync,symlinkSync,writeFileSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';

const pkg=fileURLToPath(new URL('../',import.meta.url));
const language=resolve(pkg,'../../experiments/moriarty-language');
const sha=bytes=>createHash('sha256').update(bytes).digest('hex');

export function buildO8(output,{skipZk=false}={}) {
  const src=readFileSync(join(pkg,'orders/o8.mori'));
  const lowered=lowerCompact(src,readFileSync(join(language,'spec/bounds.json')));
  const stage=join(output,'source');
  mkdirSync(stage,{recursive:true});
  writeFileSync(join(stage,'kernel.compact'),lowered.source);
  const arithmetic=readFileSync(join(language,'compact/arithmetic.compact'),'utf8');
  const pragma='pragma language_version >= 0.23 && <= 0.23;';
  if(!arithmetic.startsWith(pragma)) throw Error('Unexpected arithmetic source version');
  writeFileSync(join(stage,'arithmetic.compact'),arithmetic.replace(pragma,'pragma language_version >= 0.23 && <= 0.28;'));
  copyFileSync(join(pkg,'contracts/atomic-exchange.compact'),join(stage,'atomic-exchange.compact'));
  const tools=process.env.COMPACTC?[]:['compile','+0.36.0'];
  const compiler=process.env.COMPACTC||'compact';
  const compilerVersion=execFileSync(compiler,[...tools,'--version'],{encoding:'utf8'}).trim();
  if(!compilerVersion.startsWith('0.36.0')) throw Error('Expected Compact 0.36.0');
  const modules=resolve(process.env.MORIARTY_ORDER_MODULES||join(pkg,'../../tools/sherlock-sdk/node_modules'));
  for(const [source,dir] of [[join(stage,'atomic-exchange.compact'),output],[join(pkg,'test/fixtures/o8-mint.compact'),join(output,'mint')]]){
    execFileSync(compiler,[...tools,...(skipZk?['--skip-zk']:[]),source,dir],{stdio:'inherit'});
    if(!existsSync(join(dir,'node_modules')))symlinkSync(modules,join(dir,'node_modules'),'dir');
  }
  const ir=readFileSync(join(output,'zkir/settle.zkir'));
  if(JSON.parse(ir).version.major!==3)throw Error('Expected ZKIRv3');
  const build={compilerVersion,programHash:lowered.metadata.programHash,sourceSha256:sha(src),kernelSha256:sha(lowered.source),
    boundsSha256:sha(readFileSync(join(language,'spec/bounds.json'))),wrapperSha256:sha(readFileSync(join(stage,'atomic-exchange.compact'))),
    arithmeticSha256:sha(readFileSync(join(stage,'arithmetic.compact'))),zkirSha256:sha(ir),resources:lowered.metadata.actions[0]};
  if(!skipZk)build.verifierSha256=sha(readFileSync(join(output,'keys/settle.verifier')));
  writeFileSync(join(output,'o8-build.json'),JSON.stringify(build,null,2)+'\n');
  return build;
}
if(process.argv[1] && resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  if(!process.argv[2])throw Error('usage: build-o8.mjs OUTPUT [--skip-zk]');
  buildO8(resolve(process.argv[2]),{skipZk:process.argv.includes('--skip-zk')});
}
