import * as fs from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { prepareSuccessor, INVOCATION_UTF8_BYTES } from './evaluate.ts';
import { SYNTAX_BOUNDS } from './frontend.ts';
import { fail, FundedSourceError } from './core.ts';
import type { RepaymentResult } from './repayment.ts';

/** Fixed-size reads reject devices, pipes, oversized files and invalid UTF-8. */
function readBounded(path: string, limit: number): string {
  let fd: number | undefined;
  try {
    fd = fs.openSync(path, fs.constants.O_RDONLY | (fs.constants.O_NONBLOCK ?? 0));
    if (!fs.fstatSync(fd).isFile()) fail('CLI_IO');
    const bytes = new Uint8Array(limit + 1);
    let count = 0;
    while (count < bytes.length) {
      const n = fs.readSync(fd, bytes, count, bytes.length - count, count);
      if (n === 0) break;
      count += n;
    }
    if (count > limit) fail('CLI_BOUND');
    try { return new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(bytes.subarray(0, count)); }
    catch { return fail('CLI_UTF8'); }
  } finally {
    if (fd !== undefined) fs.closeSync(fd);
  }
}
function main(): void {
  let result: RepaymentResult;
  try {
    if (process.argv.length !== 5 || process.argv[2] !== 'simulate') fail('CLI_USAGE');
    result = prepareSuccessor(readBounded(process.argv[3], SYNTAX_BOUNDS.sourceUtf8Bytes), readBounded(process.argv[4], INVOCATION_UTF8_BYTES));
  } catch (error) {
    result = { status: 'Rejected', code: error instanceof FundedSourceError ? error.code : 'CLI_IO', actionIndex: null };
  }
  const io = process as unknown as { stdout: { write(s: string): void } };
  io.stdout.write(JSON.stringify(result) + '\n');
  process.exitCode = result.status === 'Prepared' ? 0 : 1;
}
if (process.argv[1] && resolve(process.argv[1]) === resolve(fileURLToPath(import.meta.url))) main();
