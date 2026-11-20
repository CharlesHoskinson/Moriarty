// Canonical JSON encoding for contract values that carry bigints and byte strings.
export function encodeValue(value) {
  if (typeof value === 'bigint') return {$bigint: value.toString()};
  if (value instanceof Uint8Array) return {$bytes: Buffer.from(value).toString('hex')};
  if (Array.isArray(value)) return value.map(encodeValue);
  if (value && typeof value === 'object') {
    return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, encodeValue(item)]));
  }
  return value;
}

export function decodeValue(value) {
  if (Array.isArray(value)) return value.map(decodeValue);
  if (value && typeof value === 'object') {
    if (Object.hasOwn(value, '$bigint')) return BigInt(value.$bigint);
    if (Object.hasOwn(value, '$bytes')) return Uint8Array.from(Buffer.from(value.$bytes, 'hex'));
    return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, decodeValue(item)]));
  }
  return value;
}
