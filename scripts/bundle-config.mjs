import { promises as fs } from 'node:fs';
import path from 'node:path';

const target = process.env.SIDECAR_TARGET || `${process.platform}-${process.arch}`;
const bundle = {
  resources: {
    '../third-party-licenses/*': 'third-party-licenses/',
  },
};
const source = path.resolve('sidecars', target);
if (process.platform === 'darwin') {
  const triple = target === 'darwin-arm64' ? 'aarch64-apple-darwin' : 'x86_64-apple-darwin';
  const staging = path.resolve('.tmp/sidecars');
  await fs.mkdir(staging, { recursive: true });
  bundle.externalBin = await Promise.all(
    ['yt-dlp', 'deno', 'ffmpeg', 'ffprobe'].map(async (name) => {
      const destination = path.join(staging, name);
      await fs.copyFile(path.join(source, name), `${destination}-${triple}`);
      return destination;
    }),
  );
} else {
  bundle.resources[`${source}/*`] = 'sidecars/';
}
await fs.mkdir('.tmp', { recursive: true });
await fs.writeFile('.tmp/bundle-config.json', JSON.stringify({ bundle }, null, 2));
