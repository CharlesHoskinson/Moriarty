import {build} from 'esbuild';
import {mkdir,chmod} from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
await mkdir('dist',{recursive:true});
for(const [entry,outfile] of [['src/index.ts','dist/index.js'],['src/cli.ts','dist/cli.js']])await build({entryPoints:[entry],outfile,bundle:true,platform:'node',format:'esm',target:'node24',sourcemap:true});
await chmod('dist/cli.js',0o755);
execFileSync(process.execPath,['node_modules/typescript/bin/tsc','--declaration','--emitDeclarationOnly','--noEmit','false','--outDir','dist/types','--rootDir','../..'],{stdio:'inherit'});
