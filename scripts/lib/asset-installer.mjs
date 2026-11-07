import { spawnSync } from 'node:child_process';
import { promises as fs } from 'node:fs';
import path from 'node:path';

import { download, fileHash } from './download.mjs';

async function currentArchive(destination, sha256) {
  const stamp = `${destination}.source-sha256`;
  const current = await fs.readFile(stamp, 'utf8').catch(() => '');
  return current.trim() === sha256 && Boolean(await fs.stat(destination).catch(() => null));
}

async function extractArchive(directory, destination, asset, contents) {
  const archive = path.join(directory, `asset-${process.pid}.zip`);
  try {
    await fs.writeFile(archive, contents, { mode: 0o600 });
    const result = spawnSync('tar', ['-xf', archive, '-C', directory], {
      stdio: 'inherit',
    });
    if (result.error || result.status !== 0) {
      throw result.error ?? new Error(`tar exited with status ${result.status}`);
    }
    await fs.chmod(destination, 0o755);
    await fs.writeFile(`${destination}.source-sha256`, `${asset.sha256}\n`);
  } finally {
    await fs.rm(archive, { force: true });
  }
}

async function writeExecutable(destination, contents) {
  const temporary = `${destination}.${process.pid}.tmp`;
  try {
    await fs.writeFile(temporary, contents, { mode: 0o755 });
    await fs.chmod(temporary, 0o755);
    await fs.rename(temporary, destination);
  } finally {
    await fs.rm(temporary, { force: true });
  }
}

export async function installAsset(root, target, name, asset) {
  const directory = path.join(root, 'sidecars', target);
  const destination = path.join(directory, asset.file);
  await fs.mkdir(directory, { recursive: true });
  if (asset.archive && (await currentArchive(destination, asset.sha256))) {
    console.log(`[${target}] ${name} is current`);
    return;
  }
  if (!asset.archive && (await fs.stat(destination).catch(() => null))) {
    if ((await fileHash(destination)) === asset.sha256) {
      console.log(`[${target}] ${name} is current`);
      return;
    }
  }

  console.log(`[${target}] downloading ${name}`);
  const contents = await download(asset.url, asset.sha256);
  if (asset.archive) await extractArchive(directory, destination, asset, contents);
  else await writeExecutable(destination, contents);
}
