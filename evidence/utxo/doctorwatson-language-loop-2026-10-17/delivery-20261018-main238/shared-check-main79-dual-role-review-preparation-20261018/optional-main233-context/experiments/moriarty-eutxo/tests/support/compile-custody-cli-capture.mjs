// Capture only the existing compileCustody options boundary of the real CLI.
import { registerHooks } from 'node:module';

const cliUrl = new URL('../../src/cli.mjs', import.meta.url).href;
const unused = (names) => names.map((name) =>
  `export function ${name}() { throw new Error('unused CLI operation: ${name}'); }`
).join('\n');
const probes = new Map([
  ['./compile-public.mjs', unused(['compilePublic'])],
  ['./compile-custody.mjs', `
    import { writeFileSync } from 'node:fs';
    export function compileCustody(options) {
      writeFileSync(process.env.MORI_CLI_ARGUMENT_CAPTURE, JSON.stringify({
        options,
        oracleResultsPathType: typeof options.oracleResultsPath,
      }));
      return { ok: false, code: 'CLI_ARGUMENT_MAPPING_CAPTURE', layer: 'argument-mapping-only' };
    }
  `],
  ['./designers.mjs', unused(['validateDesigners'])],
  ['./native-caller.mjs', unused(['runNativeCaller'])],
  ['./native-consumer.mjs', unused(['inspectLauncher', 'runNativeCase'])],
  ['./native-runtime.mjs', `
    export function resolveOutputPath(path, fallback) {
      return { ok: true, path: path ?? fallback };
    }
  `],
]);

registerHooks({
  resolve(specifier, context, nextResolve) {
    if (context.parentURL === cliUrl && probes.has(specifier)) {
      return {
        url: `data:text/javascript,${encodeURIComponent(probes.get(specifier))}`,
        shortCircuit: true,
      };
    }
    return nextResolve(specifier, context);
  },
});
