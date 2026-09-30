/** Finite stdio transports. Admission precedes allocation of a message body. */
export const LIMITS = Object.freeze({ header: 8192, body: 262144, response: 524288, source: 65536, documents: 32, aggregate: 1048576 });
const utf8 = new TextDecoder('utf-8', { fatal: true });
function decode(data: Buffer): unknown { return JSON.parse(utf8.decode(data)); }
export class FrameDecoder {
  private header: number[] = [];
  private body: Buffer | null = null;
  private written = 0;
  private failed = false;
  private receive: (value: unknown) => void;
  constructor(receive: (value: unknown) => void) { this.receive = receive; }
  push(chunk: Buffer): void {
    if (this.failed) throw new Error('closed framing decoder');
    try {
      let offset = 0;
      while (offset < chunk.length) {
        if (!this.body) {
          const b = chunk[offset++]!; this.header.push(b);
          if (this.header.length > LIMITS.header) throw new Error('protocol header limit');
          const h = this.header;
          if (h.length >= 4 && h[h.length - 4] === 13 && h[h.length - 3] === 10 && h[h.length - 2] === 13 && h[h.length - 1] === 10) {
            const text = Buffer.from(h).toString('ascii');
            if (h.some(v => v > 127)) throw new Error('non-ASCII protocol header');
            const rows = text.slice(0, -4).split('\r\n');
            let length: number | undefined;
            for (const row of rows) {
              const match = /^([A-Za-z-]+):[ \t]*(.*?)\s*$/.exec(row);
              if (!match) throw new Error('invalid protocol header');
              if (match[1]!.toLowerCase() === 'content-length') {
                if (length !== undefined || !/^(0|[1-9][0-9]{0,8})$/.test(match[2]!)) throw new Error('invalid Content-Length');
                length = Number(match[2]);
              } else if (match[1]!.toLowerCase() !== 'content-type') throw new Error('unknown protocol header');
            }
            if (length === undefined || length === 0 || length > LIMITS.body) throw new Error('protocol body limit');
            this.body = Buffer.alloc(length); this.written = 0; this.header = [];
          }
        } else {
          const count = Math.min(this.body.length - this.written, chunk.length - offset);
          chunk.copy(this.body, this.written, offset, offset + count); this.written += count; offset += count;
          if (this.written === this.body.length) { const body = this.body; this.body = null; this.receive(decode(body)); }
        }
      }
    } catch (error) { this.failed = true; throw error; }
  }
  finish(): void { if (this.body || this.header.length) throw new Error('truncated protocol frame'); }
}
/** MCP stdio is newline-delimited JSON, not LSP Content-Length framing. */
export class LineDecoder {
  private bytes: number[] = [];
  private receive: (value: unknown) => void;
  constructor(receive: (value: unknown) => void) { this.receive = receive; }
  push(chunk: Buffer): void {
    for (const byte of chunk) {
      if (byte === 10) { if (!this.bytes.length) throw new Error('empty MCP frame'); const data = Buffer.from(this.bytes); this.bytes = []; this.receive(decode(data)); }
      else { if (this.bytes.length >= LIMITS.body) throw new Error('protocol body limit'); this.bytes.push(byte); }
    }
  }
  finish(): void { if (this.bytes.length) throw new Error('truncated MCP frame'); }
}
function serialize(value: unknown): Buffer {
  const text = JSON.stringify(value);
  if (text === undefined || Buffer.byteLength(text) > LIMITS.response) throw new Error('serialized response limit');
  return Buffer.from(text);
}
export function encodeFrame(value: unknown): Buffer { const body = serialize(value); return Buffer.concat([Buffer.from(`Content-Length: ${body.length}\r\n\r\n`), body]); }
export function encodeLine(value: unknown): Buffer { return Buffer.concat([serialize(value), Buffer.from('\n')]); }
