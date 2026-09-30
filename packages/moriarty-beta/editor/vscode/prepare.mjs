import { copyFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
const dir = fileURLToPath(new URL('.', import.meta.url));
mkdirSync(`${dir}server`, { recursive: true });
copyFileSync(fileURLToPath(new URL('../../dist/cli.js', import.meta.url)), `${dir}server/cli.js`);
writeFileSync(`${dir}server/package.json`, '{"type":"module"}\n');
