import { promises as fs } from 'node:fs';

const root = process.cwd();
const target = `${process.platform}-${process.arch}`;
const config = {
  bundle: {
    resources: {
      [`${root}/sidecars/${target}/*`]: 'sidecars/',
      [`${root}/third-party-licenses/*`]: 'third-party-licenses/',
    },
  },
};
await fs.mkdir('.tmp', { recursive: true });
await fs.writeFile('.tmp/bundle-config.json', JSON.stringify(config, null, 2));
