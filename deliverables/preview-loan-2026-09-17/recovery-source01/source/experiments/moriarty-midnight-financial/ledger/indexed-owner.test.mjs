import assert from 'node:assert/strict';
import test from 'node:test';
import {pathToFileURL} from 'node:url';
import {createRequire} from 'node:module';
import {PINNED_NM} from './providers.mjs';
import * as owners from './indexed-owner.mjs';
const codec=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/wallet-sdk-address-format/dist/index.js').href);
const require=createRequire(PINNED_NM+'/@midnight-ntwrk/wallet-sdk-address-format/package.json');
const {bech32m}=await import(pathToFileURL(require.resolve('@scure/base')).href);
const raw=Buffer.from('ab'.repeat(32),'hex');
const encode=network=>codec.MidnightBech32m.encode(network,new codec.UnshieldedAddress(raw)).toString();
test('actual pinned SDK owner roundtrip is explicitly local or Preview',()=>{
 assert.equal(typeof owners.decodePreviewIndexedOwner,'function');
 assert.equal(typeof owners.indexedOwnerDecoderForNetwork,'function');
 for(const network of ['undeployed','preview']){
  const encoded=encode(network),decode=owners.indexedOwnerDecoderForNetwork(network);
  assert.equal(decode(encoded),raw.toString('hex'));
  assert.equal(codec.MidnightBech32m.encode(network,codec.MidnightBech32m.parse(encoded).decode(codec.UnshieldedAddress,network)).toString(),encoded);
  assert.throws(()=>decode(encode(network==='preview'?'undeployed':'preview')),{message:'INVALID_INDEXED_OWNER'});
 }
 assert.equal(owners.indexedOwnerDecoderForNetwork(),owners.decodeLocalIndexedOwner);
 assert.equal(owners.indexedOwnerDecoderForNetwork('preview'),owners.decodePreviewIndexedOwner);
});
test('Preview rejects wrong type, network, length, prefix and noncanonical SDK encodings',()=>{
 assert.equal(typeof owners.decodePreviewIndexedOwner,'function');
 const good=encode('preview');
 const invalid=[encode('undeployed'),encode('mainnet'),good.toUpperCase(),good.replace('preview','Preview'),good+'q',raw.toString('hex'),null,{},'x'.repeat(129),new codec.MidnightBech32m('dust','preview',raw).toString()];
 for(const [hrp,bytes] of [['mn_addr_preview_extra',raw],['other_addr_preview',raw],['mn_addr_preview',raw.subarray(1)],['mn_addr_preview',Buffer.concat([raw,Buffer.from([0])])]])invalid.push(bech32m.encode(hrp,bech32m.toWords(bytes),false));
 for(const value of invalid)assert.throws(()=>owners.decodePreviewIndexedOwner(value),{message:'INVALID_INDEXED_OWNER'});
});
