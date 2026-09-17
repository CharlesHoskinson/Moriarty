/** Indexer and wallet sync owners are Bech32m; native owners are hex.
 * Only public codec files are read here. No provider, wallet or private access.
 */
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
const root=PINNED_NM+'/@midnight-ntwrk/wallet-sdk-address-format';
const hash=path=>createHash('sha256').update(readFileSync(path)).digest('hex');
if(hash(root+'/package.json')!=='a176faaf101ac74f4ea6ee1918e4631c4cd7e8835d1f30159c03a8ce92c23c31'||hash(root+'/dist/index.js')!=='20c1669c47d99200ccd981508dee9dd9b99fb0ded3217232dded468482de81cd')throw Error('INDEXED_ADDRESS_CODEC_PIN');
const {MidnightBech32m,UnshieldedAddress}=await import(pathToFileURL(root+'/dist/index.js').href);
function decodeIndexedOwner(value,network){
 try{
  if(typeof value!=='string'||value.length>128)throw Error();
  const decoded=MidnightBech32m.parse(value).decode(UnshieldedAddress,network);
  // Roundtrip also rejects uppercase and extra HRP segments the SDK parser ignores.
  if(MidnightBech32m.encode(network,decoded).toString()!==value||!/^[0-9a-f]{64}$/.test(decoded.hexString))throw Error();
  return decoded.hexString;
 }catch{throw Error('INVALID_INDEXED_OWNER');}
}

export function decodeLocalIndexedOwner(value){return decodeIndexedOwner(value,'undeployed');}
export function decodePreviewIndexedOwner(value){return decodeIndexedOwner(value,'preview');}
/** Closed owner-encoding selection only; callers still bind chain and wallet identity. */
export function indexedOwnerDecoderForNetwork(network='undeployed'){
 if(network==='undeployed')return decodeLocalIndexedOwner;
 if(network==='preview')return decodePreviewIndexedOwner;
 throw Error('INVALID_INDEXED_OWNER_NETWORK');
}
