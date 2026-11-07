import { promises as fs } from 'node:fs';

const manifest = JSON.parse(await fs.readFile('sidecars/manifest.json', 'utf8'));
for (const [target, assets] of Object.entries(manifest)) {
  for (const [name, asset] of Object.entries(assets)) {
    if (!/^[0-9a-f]{64}$/.test(asset.sha256)) {
      throw new Error(`Invalid SHA-256 for ${target}/${name}`);
    }
  }
}
console.log('All sidecar SHA-256 values are well formed.');
