import { createHash } from 'node:crypto';
import { promises as fs } from 'node:fs';

export async function download(url, sha256) {
  const response = await fetch(url, {
    headers: { 'User-Agent': 'media-archive-build' },
  });
  if (!response.ok) throw new Error(`${response.status} ${response.statusText}: ${url}`);
  const contents = Buffer.from(await response.arrayBuffer());
  const actual = createHash('sha256').update(contents).digest('hex');
  if (sha256 && actual !== sha256) {
    throw new Error(`SHA-256 mismatch for ${url}: expected ${sha256}, got ${actual}`);
  }
  return contents;
}

export async function fileHash(file) {
  return createHash('sha256')
    .update(await fs.readFile(file))
    .digest('hex');
}
