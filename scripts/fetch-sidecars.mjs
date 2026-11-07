import { promises as fs } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { installAsset } from './lib/asset-installer.mjs';
import { downloadLicenses } from './lib/licenses.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const manifestFile = path.join(root, 'sidecars', 'manifest.json');
const manifest = JSON.parse(await fs.readFile(manifestFile, 'utf8'));
const requested = process.argv[2];
const hostTarget = `${process.platform}-${process.arch}`;
const targets = requested === 'all' ? Object.keys(manifest) : [requested ?? hostTarget];

for (const target of targets) {
  if (!manifest[target]) throw new Error(`Unsupported sidecar target: ${target}`);
  for (const [name, asset] of Object.entries(manifest[target])) {
    await installAsset(root, target, name, asset);
  }
}
await downloadLicenses(root, targets);
