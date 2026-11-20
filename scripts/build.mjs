import { spawnSync } from 'node:child_process';
import path from 'node:path';

const cli = path.resolve('node_modules/@tauri-apps/cli/tauri.js');
const args = [cli, 'build', '--config', '.tmp/bundle-config.json'];
if (process.platform === 'darwin') {
  args.push('--bundles', 'app,dmg');
}
if (process.env.TAURI_TARGET) {
  args.push('--target', process.env.TAURI_TARGET);
}
const env = { ...process.env };
for (const name of Object.keys(env)) {
  if (name.startsWith('APPLE_') && !env[name]) delete env[name];
}
const result = spawnSync(process.execPath, args, { stdio: 'inherit', shell: false, env });
if (result.error) throw result.error;
process.exitCode = result.status ?? 1;
