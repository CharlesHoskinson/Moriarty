/** Staged source entrypoint; do not execute before separately reviewed authorization. */
import {readFileSync,statSync} from 'node:fs';
import {prepareExactNativeTransfer} from './handoff.ts';
const a=process.argv.slice(2);
if(a.length!==6)throw new Error('Expected sourcePath action scenarioPath signaturePath absoluteCryptoBinary absoluteNewOutput');
function text(path:string){if(statSync(path).size>65536)throw new Error('input bound');return new TextDecoder('utf-8',{fatal:true}).decode(readFileSync(path));}
const receipt=await prepareExactNativeTransfer({source:text(a[0]),action:a[1],scenarioText:text(a[2]),signatureText:text(a[3]),cryptoBinary:a[4],newOutput:a[5]});
process.stdout.write(JSON.stringify(receipt)+'\n');
