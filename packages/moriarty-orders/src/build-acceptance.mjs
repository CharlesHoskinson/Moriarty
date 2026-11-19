import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {copyFileSync, existsSync, mkdirSync, readFileSync, symlinkSync, writeFileSync} from 'node:fs';
import {join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';

const pkg = fileURLToPath(new URL('../',import.meta.url));
const language = resolve(pkg,'../../experiments/moriarty-language');
const sha = bytes => createHash('sha256').update(bytes).digest('hex');

export function buildAcceptance(output,{skipZk=false,policy='o1'}={}) {
  const builds = {
    o1: {source:'o1', custody:'acceptance', operations:['fund','fill','cancel']},
    executors: {source:'o1', custody:'executors', operations:['fund','fill','cancel']},
    staged: {source:'o9', custody:'staged-exchange', includes:['continuation'], operations:['fund','first','enable','resume','refund','withdraw']},
    recovery: {source:'partial-recovery', custody:'recovery', operations:['fund','fill','cancel','depositSurplus']},
  };
  const config = builds[policy];
  if (!config) throw Error('Unknown order build policy: '+policy);
  const source = readFileSync(join(pkg,'orders',config.source+'.mori'));
  const mapping = lowerCompact(source,readFileSync(join(language,'spec/bounds.json')));
  const stage = join(output,'source');
  mkdirSync(stage,{recursive:true});
  writeFileSync(join(stage,'kernel.compact'),mapping.source);
  const arithmetic = readFileSync(join(language,'compact/arithmetic.compact'),'utf8');
  const pragma = 'pragma language_version >= 0.23 && <= 0.23;';
  if (!arithmetic.startsWith(pragma)) throw Error('Unexpected arithmetic source version');
  writeFileSync(join(stage,'arithmetic.compact'),arithmetic.replace(pragma,'pragma language_version >= 0.23 && <= 0.28;'));
  for (const include of config.includes ?? []) copyFileSync(join(pkg,'contracts',include+'.compact'),join(stage,include+'.compact'));
  const wrapper = join(stage,config.custody+'.compact');
  copyFileSync(join(pkg,'contracts',config.custody+'.compact'),wrapper);
  const pins = {programHash:mapping.metadata.programHash, sourceSha256:sha(source), kernelSha256:sha(mapping.source),
    custodySha256:sha(readFileSync(wrapper)), upstreamArithmeticSha256:sha(arithmetic), arithmeticSha256:sha(readFileSync(join(stage,'arithmetic.compact')))};
  if (config.includes) pins.includes = Object.fromEntries(config.includes.map(name => [name,sha(readFileSync(join(stage,name+'.compact')))]));
  const command = process.env.COMPACTC || 'compact';
  const selector = process.env.COMPACTC ? [] : ['compile', '+0.36.0'];
  pins.compilerVersion = execFileSync(command,[...selector,'--version'],{encoding:'utf8'}).trim();
  const legacy = pins.compilerVersion.startsWith('0.31.0');
  if (legacy && (!skipZk || policy !== 'o1')) throw Error('Native order builds require Compact 0.36.0');
  if (legacy) {
    writeFileSync(wrapper,readFileSync(wrapper,'utf8').replaceAll(' as JubjubScalar',''));
    pins.custodySha256 = sha(readFileSync(wrapper));
  }
  if (!legacy && !pins.compilerVersion.startsWith('0.36.0')) throw Error('Unsupported order Compact compiler');
  execFileSync(command,[...selector,...(legacy?['--feature-zkir-v3']:[]),...(skipZk?['--skip-zk']:[]),wrapper,output],{stdio:'inherit'});
  const modules = resolve(process.env.MORIARTY_ORDER_MODULES || (legacy ? join(language,'node_modules') : join(pkg,'../../tools/sherlock-sdk/node_modules')));
  if (!existsSync(join(output,'node_modules'))) symlinkSync(modules,join(output,'node_modules'),'dir');
  const operations = config.operations;
  pins.zkir = {};
  for (const op of operations) {
    const ir = readFileSync(join(output,'zkir',op+'.zkir'));
    const {version} = JSON.parse(ir);
    if (version.major !== 3) throw Error(op+' did not compile to ZKIRv3');
    pins.zkir[op] = {version,sha256:sha(ir)};
  }
  if (!skipZk) pins.verifiers = Object.fromEntries(operations.map(op => [op,sha(readFileSync(join(output,'keys',op+'.verifier')))]));
  writeFileSync(join(output,'acceptance-build.json'),JSON.stringify(pins,null,2)+'\n');
  return {...pins,runtimePath:join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')};
}
if (process.argv[1] && resolve(process.argv[1])===fileURLToPath(import.meta.url)) {
  if (!process.argv[2]) throw Error('usage: node build-acceptance.mjs OUTPUT [--skip-zk]');
  buildAcceptance(resolve(process.argv[2]),{skipZk:process.argv.includes('--skip-zk'),policy:process.argv.includes('--staged')?'staged':process.argv.includes('--recovery')?'recovery':process.argv.includes('--executors')?'executors':'o1'});
}
