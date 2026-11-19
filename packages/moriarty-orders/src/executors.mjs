import {signOrder} from './authorization.mjs';

// Each credential authorizes only its slot in one contract and revision.
export function signExecutor(runtime,circuits,grant,role,secret) {
  const action = {primary:2n,fallback:3n}[role];
  if (typeof action !== 'bigint') throw Error('Unknown executor role');
  return signOrder(runtime,circuits,{...grant,owner:runtime.ecMulGenerator(secret),action},secret);
}
