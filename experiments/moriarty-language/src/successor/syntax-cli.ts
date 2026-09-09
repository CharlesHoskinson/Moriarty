import * as fs from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseSuccessorSource, SuccessorSyntaxError, SYNTAX_BOUNDS } from './frontend.ts';
import { formatSuccessorSource } from './format.ts';

declare module 'node:fs' {
  interface FdStats {
    isFile(): boolean;
  }
  export function openSync(filePath: string, flags: number): number;
  export function readSync(
    fd: number,
    buffer: Uint8Array,
    offset: number,
    length: number,
    position: number,
  ): number;
  export function closeSync(fd: number): void;
  export function fstatSync(fd: number): FdStats;
  export const constants: {
    readonly O_RDONLY: number;
    readonly O_NONBLOCK?: number;
  };
}

type StdStream = { write(chunk: string): unknown };
type ProcessIo = {
  argv: string[];
  cwd(): string;
  exitCode: number;
  stdout: StdStream;
  stderr: StdStream;
};

class CliError extends Error {
  readonly code: string;
  constructor(code: string, message: string) {
    super(message);
    this.code = code;
    this.name = 'CliError';
  }
}

function proc(): ProcessIo {
  return process as unknown as ProcessIo;
}

function isDirectInvocation(): boolean {
  const entry = process.argv[1];
  if (typeof entry !== 'string' || entry.length === 0) {
    return false;
  }
  return path.resolve(entry) === path.resolve(fileURLToPath(import.meta.url));
}

function usageDetail(): string {
  return 'expected check-syntax FILE or format FILE';
}

function parseCliArgs(argv: string[]): { command: 'check-syntax' | 'format'; file: string } {
  if (argv.length !== 4) {
    throw new CliError('CLI_USAGE', usageDetail());
  }
  const command = argv[2];
  const file = argv[3];
  if (command !== 'check-syntax' && command !== 'format') {
    throw new CliError('CLI_USAGE', usageDetail());
  }
  if (typeof file !== 'string' || file.length === 0) {
    throw new CliError('CLI_USAGE', usageDetail());
  }
  return { command, file };
}

function openReadFlags(): number {
  return fs.constants.O_RDONLY | (fs.constants.O_NONBLOCK ?? 0);
}

function nodeErrno(error: unknown): string | undefined {
  if (typeof error === 'object' && error !== null && 'code' in error) {
    const code = (error as { code: unknown }).code;
    return typeof code === 'string' ? code : undefined;
  }
  return undefined;
}

function wrapIo(error: unknown): CliError {
  if (error instanceof CliError) {
    return error;
  }
  return new CliError('CLI_IO', 'failed to read file');
}

function readRegularFileBounded(filePath: string, limit: number): Uint8Array {
  let fd: number | undefined;
  try {
    try {
      fd = fs.openSync(filePath, openReadFlags());
    } catch (error) {
      const errno = nodeErrno(error);
      if (errno === 'EAGAIN' || errno === 'EWOULDBLOCK' || errno === 'ENXIO') {
        throw new CliError('CLI_IO', 'not a regular file');
      }
      throw new CliError('CLI_IO', 'failed to open file');
    }
    const stats = fs.fstatSync(fd);
    if (typeof stats.isFile !== 'function' || !stats.isFile()) {
      throw new CliError('CLI_IO', 'not a regular file');
    }
    const buf = new Uint8Array(limit + 1);
    let got = 0;
    while (got < buf.length) {
      let n: number;
      try {
        n = fs.readSync(fd, buf, got, buf.length - got, got);
      } catch (error) {
        const errno = nodeErrno(error);
        if (errno === 'EAGAIN' || errno === 'EWOULDBLOCK') {
          throw new CliError('CLI_IO', 'not a regular file');
        }
        throw new CliError('CLI_IO', 'failed to read file');
      }
      if (n === 0) {
        break;
      }
      if (n < 0) {
        throw new CliError('CLI_IO', 'failed to read file');
      }
      got += n;
    }
    if (got > limit) {
      throw new CliError('SOURCE_BOUND', `source exceeds ${limit} UTF-8 bytes`);
    }
    return buf.subarray(0, got);
  } catch (error) {
    throw wrapIo(error);
  } finally {
    if (fd !== undefined) {
      try {
        fs.closeSync(fd);
      } catch {
        // Descriptor close must not replace the primary CLI result.
      }
    }
  }
}

function decodeUtf8(bytes: Uint8Array): string {
  try {
    return new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(bytes);
  } catch {
    throw new CliError('INVALID_UTF8', 'input is not valid UTF-8');
  }
}

function reportFailure(error: unknown): void {
  const io = proc();
  io.exitCode = 1;
  if (error instanceof SuccessorSyntaxError) {
    io.stderr.write(`${error.code} ${error.start} ${error.end}\n`);
    return;
  }
  if (error instanceof CliError) {
    io.stderr.write(`${error.code}: ${error.message}\n`);
    return;
  }
  io.stderr.write('CLI_IO\n');
}

function run(): void {
  const io = proc();
  const args = parseCliArgs(io.argv);
  const filePath = path.resolve(args.file);
  const bytes = readRegularFileBounded(filePath, SYNTAX_BOUNDS.sourceUtf8Bytes);
  const source = decodeUtf8(bytes);
  if (args.command === 'check-syntax') {
    io.stdout.write(`${JSON.stringify(parseSuccessorSource(source))}\n`);
  } else {
    io.stdout.write(formatSuccessorSource(source));
  }
  io.exitCode = 0;
}

if (isDirectInvocation()) {
  try {
    run();
  } catch (error) {
    reportFailure(error);
  }
}