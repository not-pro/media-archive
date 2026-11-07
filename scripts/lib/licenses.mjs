import { promises as fs } from 'node:fs';
import path from 'node:path';

import { download } from './download.mjs';

const FFMPEG_RELEASE = 'https://github.com/eugeneware/ffmpeg-static/releases/download/b6.1.1';

export async function downloadLicenses(root, targets) {
  const directory = path.join(root, 'third-party-licenses');
  await fs.mkdir(directory, { recursive: true });
  const notices = [
    [
      'yt-dlp.txt',
      'https://raw.githubusercontent.com/yt-dlp/yt-dlp/2026.10.26/THIRD_PARTY_LICENSES.txt',
    ],
    ['deno.txt', 'https://raw.githubusercontent.com/denoland/deno/v2.9.8/LICENSE.md'],
  ];
  for (const target of targets) {
    const platform = target === 'win32-x64' ? 'win32-x64' : target;
    notices.push([`ffmpeg-${target}.txt`, `${FFMPEG_RELEASE}/${platform}.LICENSE`]);
    notices.push([`ffmpeg-${target}-README.txt`, `${FFMPEG_RELEASE}/${platform}.README`]);
  }
  for (const [name, url] of notices) {
    await fs.writeFile(path.join(directory, name), await download(url), {
      mode: 0o644,
    });
  }
  console.log('Third-party licenses downloaded.');
}
