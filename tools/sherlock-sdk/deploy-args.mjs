// deploy-current.mjs arguments: SDK_ROOT BUILD PREPARATION OUTPUT [--instance-id N] [--submit]
// --instance-id replaces constructor argument 0 (instance_id) only; every other argument and both verifier
// keys stay those of the reviewed preparation. N is decimal or 0x-prefixed big-endian hex.
export const FR = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001n;
const USAGE = 'usage: deploy-current.mjs SDK_ROOT BUILD PREPARATION OUTPUT [--instance-id N] [--submit]';

const parseInstance = s => {
  if (typeof s !== 'string' || !/^(0x[0-9a-fA-F]+|[0-9]+)$/.test(s)) throw Error(`instance id must be decimal or 0x hex: ${s}`);
  const v = BigInt(s);
  if (v === 0n || v >= FR) throw Error('instance id must be a nonzero field element');
  return v;
};

export function parseDeployArgs(argv) {
  const positional = [], out = {submit: false, instanceId: undefined};
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--submit') out.submit = true;
    else if (a === '--instance-id') {
      if (i + 1 >= argv.length) throw Error('--instance-id needs a value');
      out.instanceId = parseInstance(argv[++i]);
    } else if (a.startsWith('--')) throw Error(USAGE);
    else positional.push(a);
  }
  if (positional.length !== 4) throw Error(USAGE);
  const [sdkRoot, build, preparation, output] = positional;
  return {sdkRoot, build, preparation, output, ...out};
}

export function constructorArgs(expected, instanceId) {
  const args = expected.constructor.map(BigInt);
  if (instanceId !== undefined) args[0] = instanceId;
  return args;
}
