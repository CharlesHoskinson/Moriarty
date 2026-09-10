/** Fixed financial build -> generated module -> CompiledContract. No compile/prove/network operations.
 * An externally admitted receipt hash is the provenance root, not an authentication mechanism.
 */
import {openSync, closeSync, fstatSync, lstatSync, readFileSync, readSync, readdirSync, constants} from 'node:fs';
import {createHash} from 'node:crypto';
import {dirname, resolve, join, relative, isAbsolute, sep} from 'node:path';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {registerHooks} from 'node:module';
import {inspectBuildSources, sourceManifestHash as hashSources} from './build-proven.mjs';
import {PINNED_NM} from './providers.mjs';

const CIRCUITS = {loan:['initialize','accrue','settle'],swap:['initialize','swap','close']};
const HEX = /^[a-f0-9]{64}$/;
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const PINS = {
  "@midnight-ntwrk/midnight-js-protocol": {
    "version": "4.1.1",
    "packageSha256": "bdfe30f046627f364fd057fdf4dd756655fb4a4309722b64bb3e46484bf9effd",
    "entries": {
      "dist/compact-js.mjs": "7c34e5ac44b1406080f1cbf9f514c1d887b7bc947327a3f83031d9794caa85b7"
    }
  },
  "@midnight-ntwrk/compact-js": {
    "version": "2.5.1",
    "packageSha256": "90b4bcb6c76218dc4f99ca2cdb5ce619e1df2932c6fb9e494f9f8bee1c240b51",
    "entries": {
      "dist/esm/index.js": "d4ab62942ce025b93ce67ffe880e081a91bda84bc7300f9c749dc87d3fe13d3f",
      "dist/esm/effect/CompiledContract.js": "72d13105fa558c4ce908df43acc6879b051e0b64fec15da2efa57989a2035d30"
    }
  },
  "@midnight-ntwrk/compact-runtime": {
    "version": "0.16.0",
    "packageSha256": "ac4f818510afca0d17758b4c38af613f7b1a489aec96633548d0d361683f124c",
    "entries": {
      "dist/index.js": "c55a8ab3e7533b3fa6e27b66ab742c783d912d5a00d6ecc23d9f6142ddb0ecde"
    }
  },
  "typescript": {
    "version": "6.0.3",
    "packageSha256": "9332e97c30d3e53ed54910b89207ed657fb444066484df6e5b6965bf130865e9",
    "entries": {
      "lib/typescript.js": "569177652966bd528c319171c7dd22860dbf72bde116cbc4f644f1d02bb12e39"
    }
  }
};
function requireThat(value, message) { if (!value) throw Error(message); }
function safePath(path) {
  requireThat(isAbsolute(path ?? '') && resolve(path) === path, 'normalized absolute path required');
  for (let current = path; ; current = dirname(current)) {
    const stat = lstatSync(current);
    requireThat(!stat.isSymbolicLink(), 'symlink asset/ancestor forbidden');
    if (current !== path) requireThat(stat.isDirectory(), 'asset ancestor must be a directory');
    if (dirname(current) === current) break;
  }
  return path;
}
function bytesAt(path) {
  safePath(path);
  const fd = openSync(path, constants.O_RDONLY | constants.O_NOFOLLOW);
  try { requireThat(fstatSync(fd).isFile(), 'regular file required'); return readFileSync(fd); }
  finally { closeSync(fd); }
}
function fingerprint(path) {
  safePath(path);
  const fd = openSync(path, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    const stat = fstatSync(fd); requireThat(stat.isFile() && stat.size > 0, 'empty or nonregular asset');
    const h = createHash('sha256'), buffer = Buffer.allocUnsafe(1024 * 1024);
    let length = 0, read;
    while ((read = readSync(fd, buffer, 0, buffer.length, null)) !== 0) { h.update(buffer.subarray(0, read)); length += read; }
    const end = fstatSync(fd); requireThat(length === stat.size && end.size === stat.size && end.mtimeMs === stat.mtimeMs, 'asset changed during inspection');
    return {bytes:length, sha256:h.digest('hex')};
  } finally { closeSync(fd); }
}
function checkedPin(name) {
  const pin = PINS[name], root = join(PINNED_NM, name);
  requireThat(pin && hash(bytesAt(join(root,'package.json'))) === pin.packageSha256, 'dependency package pin mismatch: ' + name);
  const metadata = JSON.parse(bytesAt(join(root,'package.json')));
  requireThat(metadata.version === pin.version, 'dependency version mismatch: ' + name);
  for (const [entry, digest] of Object.entries(pin.entries)) requireThat(hash(bytesAt(join(root,entry))) === digest, 'dependency entry pin mismatch: ' + name);
  return root;
}
async function loadParser() {
  const root = checkedPin('typescript');
  const module = await import(pathToFileURL(join(root,'lib/typescript.js')).href);
  return module.default ?? module;
}
function checkGeneratedImports(ts, path, bytes) {
  const source = ts.createSourceFile(path, bytes.toString('utf8'), ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  requireThat(source.parseDiagnostics.length === 0, 'generated module parse error');
  function visit(node) {
    if (ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) {
      if (node.moduleSpecifier) requireThat(ts.isStringLiteral(node.moduleSpecifier) && node.moduleSpecifier.text === '@midnight-ntwrk/compact-runtime', 'unsupported generated module import');
    }
    if (ts.isCallExpression(node)) {
      requireThat(node.expression.kind !== ts.SyntaxKind.ImportKeyword, 'lazy generated imports unsupported');
      requireThat(!(ts.isIdentifier(node.expression) && ['require','eval','Function'].includes(node.expression.text)), 'generated dynamic module loading unsupported');
    }
    requireThat(!(ts.isNewExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === 'Function'), 'generated dynamic code unsupported');
    ts.forEachChild(node, visit);
  }
  visit(source);
}
function checkCommands(receipt, outputDir) {
  const compact = '/home/charl/.local/bin/compact', stage = join(outputDir,'stage');
  const expected = [[compact,'--version'],[compact,'compile','--version'],[compact,'compile','--language-version'],[compact,'compile','--runtime-version'],[compact,'compile','--compact-path',stage,join(stage,receipt.case+'.compact'),receipt.assetsPath]];
  const versions = ['compact 0.5.2','0.31.1','0.23.0','0.16.0'];
  requireThat(Array.isArray(receipt.commands) && receipt.commands.length === expected.length, 'complete compiler commands required');
  for (let i=0;i<expected.length;i++) {
    const command=receipt.commands[i];
    requireThat(JSON.stringify(command.argv) === JSON.stringify(expected[i]), 'unexpected compiler command/skip-zk');
    requireThat(command.exit === 0 && command.signal === null && command.error === null && Number.isSafeInteger(command.timeoutMs) && command.timeoutMs > 0, 'compiler did not finish successfully');
    requireThat(typeof command.stdout === 'string' && typeof command.stderr === 'string' && hash(command.stdout) === command.stdoutSha256 && hash(command.stderr) === command.stderrSha256, 'compiler output hash mismatch');
    if (i < 4) requireThat(command.stdout.trim() === versions[i], 'compiler version output mismatch');
  }
  requireThat(JSON.stringify(receipt.versions) === JSON.stringify({compactWrapper:'compact 0.5.2',compiler:'0.31.1',language:'0.23.0',runtime:'0.16.0'}), 'build versions mismatch');
}
function inspectBytes(options, sourceTestOnly) {
  requireThat(Object.hasOwn(CIRCUITS, options.case), 'fixed financial case required');
  requireThat(HEX.test(options.receiptSha256 ?? '') && HEX.test(options.sourceManifestHash ?? ''), 'external receipt and source hash bindings required');
  const bytes = bytesAt(options.receiptPath);
  requireThat(hash(bytes) === options.receiptSha256, 'build receipt hash mismatch');
  const receipt = JSON.parse(bytes);
  requireThat(receipt.schema === 'moriarty.financial-proven-assets/1' && receipt.case === options.case, 'build receipt schema/case mismatch');
  const genuine = receipt.status === 'built' && receipt.synthetic === false && receipt.proven === true && receipt.inspectedProofAssets === true;
  const synthetic = receipt.status === 'source-test-only' && receipt.synthetic === true && receipt.proven === false && receipt.inspectedProofAssets === false;
  requireThat(sourceTestOnly ? synthetic : genuine, sourceTestOnly ? 'source inspection requires explicitly synthetic build' : 'genuine build required; synthetic/failed/unproven receipts reject');
  // The compiler builds proving assets; it does not produce transaction proof bodies.
  requireThat(receipt.proofsGenerated === false && !Object.hasOwn(receipt,'error'), 'unexpected build proof/error claim');
  requireThat(receipt.sourceManifestHash === options.sourceManifestHash && hashSources(inspectBuildSources()) === options.sourceManifestHash, 'stale build sources');
  const outputDir = dirname(safePath(options.receiptPath));
  requireThat(options.receiptPath === join(outputDir,'build-receipt.json'), 'unexpected build receipt location');
  requireThat(receipt.assetsPath === join(outputDir,options.case) && receipt.contractModulePath === join(receipt.assetsPath,'contract/index.js'), 'build asset path mismatch');
  checkCommands(receipt,outputDir);
  const attempt=JSON.parse(bytesAt(receipt.attemptFile));
  requireThat(attempt.schema === 'moriarty.financial-build-attempt/1' && attempt.synthetic === sourceTestOnly, 'build attempt synthetic/genuine mismatch');
  for (const [key,value] of Object.entries({resourceId:receipt.resourceId,admissionId:receipt.admissionId,sourceCandidateSha:options.sourceManifestHash,case:options.case,outputDir,attempts:1})) requireThat(attempt[key] === value, 'build attempt identity mismatch');
  requireThat(typeof receipt.admissionId === 'string' && receipt.admissionId.length > 0 && typeof receipt.resourceId === 'string' && receipt.resourceId.length > 0, 'build resource identity missing');
  requireThat(HEX.test(receipt.resourceRecord?.sha256 ?? '') && hash(bytesAt(receipt.resourceRecord.path)) === receipt.resourceRecord.sha256, 'build resource byte binding mismatch');
  requireThat(Array.isArray(receipt.artifacts), 'asset manifest required');
  const declared = new Map();
  for (const artifact of receipt.artifacts) {
    requireThat(artifact && Object.keys(artifact).sort().join(',') === 'bytes,path,sha256', 'unknown artifact field');
    const name=artifact.path;
    requireThat(typeof name === 'string' && name.length > 0 && !name.includes('\\') && !name.split('/').some(p => !p || p === '.' || p === '..') && !isAbsolute(name), 'unsafe relative asset path');
    requireThat(!declared.has(name) && HEX.test(artifact.sha256) && Number.isSafeInteger(artifact.bytes) && artifact.bytes > 0, 'duplicate or malformed artifact');
    declared.set(name,artifact);
  }
  const observed = new Map();
  function visit(directory) {
    safePath(directory); requireThat(lstatSync(directory).isDirectory(), 'asset directory required');
    for (const name of readdirSync(directory).sort()) {
      const path=join(directory,name), stat=lstatSync(path);
      requireThat(!stat.isSymbolicLink(), 'asset symlink forbidden');
      if (stat.isDirectory()) visit(path);
      else {
        const name=relative(receipt.assetsPath,path).split(sep).join('/'), pin=declared.get(name);
        requireThat(pin, 'unlisted asset: '+name);
        const actual=fingerprint(path);
        requireThat(actual.bytes === pin.bytes && actual.sha256 === pin.sha256, 'asset bytes/hash mismatch: '+name);
        observed.set(name,pin);
      }
    }
  }
  visit(receipt.assetsPath);
  requireThat(observed.size === declared.size, 'missing listed asset');
  for (const path of ['contract/index.js','compiler/contract-info.json',...CIRCUITS[options.case].flatMap(name=>[`keys/${name}.prover`,`keys/${name}.verifier`,`zkir/${name}.zkir`,`zkir/${name}.bzkir`])]) requireThat(observed.has(path), 'missing required proof asset: '+path);
  const metadata=JSON.parse(bytesAt(join(receipt.assetsPath,'compiler/contract-info.json')));
  for (const [key,value] of Object.entries({'compiler-version':'0.31.1','language-version':'0.23.0','runtime-version':'0.16.0'})) requireThat(metadata[key] === value, 'compiler metadata version mismatch');
  requireThat(Array.isArray(metadata.circuits), 'compiler circuit metadata missing');
  const circuits=metadata.circuits.filter(c=>c.proof === true && c.pure === false).map(c=>c.name).sort();
  requireThat(JSON.stringify(circuits) === JSON.stringify([...CIRCUITS[options.case]].sort()), 'compiler proof circuits mismatch');
  return {receipt, observed};
}
function compose(CompiledContract, generated, kind, path) {
  requireThat(typeof generated?.Contract === 'function' && typeof generated?.ledger === 'function', 'generated Contract/ledger exports required');
  requireThat(['make','withVacantWitnesses','withCompiledFileAssets'].every(name=>typeof CompiledContract?.[name] === 'function'), 'pinned CompiledContract interface required');
  return CompiledContract.make('moriarty-financial-'+kind,generated.Contract).pipe(CompiledContract.withVacantWitnesses,CompiledContract.withCompiledFileAssets(path));
}
/** Structural source inspection never grants executable/proven status or returns a contract. */
export async function inspectFinancialBuild(options) {
  options=Object.freeze({...options});
  const sourceTestOnly=options.sourceTestOnly === true;
  requireThat(options.moduleAdapter === undefined || (sourceTestOnly && typeof options.moduleAdapter === 'function'), 'module adapters are source-test-only');
  const {receipt,observed}=inspectBytes(options,sourceTestOnly), ts=await loadParser();
  for (const name of observed.keys()) if (/\.(?:js|mjs|cjs)$/.test(name)) checkGeneratedImports(ts,join(receipt.assetsPath,name),bytesAt(join(receipt.assetsPath,name)));
  if (options.moduleAdapter) {
    const adapter=await options.moduleAdapter(receipt.contractModulePath);
    compose(adapter.CompiledContract,adapter.generatedModule,options.case,receipt.assetsPath);
    inspectBytes(options,sourceTestOnly);
  }
  return Object.freeze({status:sourceTestOnly?'source-test-only':'assets-inspected',executable:false,case:options.case,receiptSha256:options.receiptSha256,sourceManifestHash:options.sourceManifestHash,zkConfigPath:receipt.assetsPath,contractModulePath:receipt.contractModulePath,artifactCount:observed.size});
}
/** Only externally bound genuine build receipts reach actual generated module import. */
export async function loadProvenFinancialContract(options) {
  options=Object.freeze({...options});
  requireThat(options.moduleAdapter === undefined && options.sourceTestOnly === undefined, 'executable loader forbids source-test adapters');
  const report=await inspectFinancialBuild(options);
  const protocolRoot=checkedPin('@midnight-ntwrk/midnight-js-protocol');
  checkedPin('@midnight-ntwrk/compact-js');
  const runtimeRoot=checkedPin('@midnight-ntwrk/compact-runtime');
  const runtimeUrl=pathToFileURL(join(runtimeRoot,'dist/index.js')).href;
  const assetRoot=report.zkConfigPath+sep;
  const belongs=url=>{try{return fileURLToPath(url).startsWith(assetRoot);}catch{return false;}};
  // Exact runtime mapping is outside hashed assets. Static-only imports allow immediate deregistration.
  const hooks=registerHooks({
    resolve(specifier,context,nextResolve) {
      if (context.parentURL && belongs(context.parentURL)) {
        requireThat(specifier === '@midnight-ntwrk/compact-runtime', 'unsupported generated module dependency');
        return {url:runtimeUrl,shortCircuit:true};
      }
      return nextResolve(specifier,context);
    },
    load(url,context,nextLoad) {
      if (belongs(url)) {
        const path=fileURLToPath(url), {observed}=inspectBytes(options,false), name=relative(report.zkConfigPath,path).split(sep).join('/');
        requireThat(observed.has(name) && /\.js$/.test(name), 'unlisted generated module');
        const source=bytesAt(path);requireThat(hash(source) === observed.get(name).sha256,'generated source changed before import');
        return {format:'module',source,shortCircuit:true};
      }
      return nextLoad(url,context);
    },
  });
  let generated, CompiledContract;
  try {
    ({CompiledContract}=await import(pathToFileURL(join(protocolRoot,'dist/compact-js.mjs')).href));
    generated=await import(pathToFileURL(report.contractModulePath).href+'?financialBuild='+options.receiptSha256);
  } finally { hooks.deregister(); }
  inspectBytes(options,false);
  const compiledContract=compose(CompiledContract,generated,options.case,report.zkConfigPath);
  let closed=false;
  const assertFresh=()=>{requireThat(!closed,'financial assets loader closed');inspectBytes(options,false);return true;};
  return Object.freeze({case:options.case,compiledContract,zkConfigPath:report.zkConfigPath,receiptSha256:report.receiptSha256,sourceManifestHash:report.sourceManifestHash,
    decodeState(state){assertFresh();requireThat(state !== undefined && state !== null,'observed contract state required');return generated.ledger(state);},
    assertFresh,cleanup(){closed=true;return {loaderHooksRemoved:true};}});
}
