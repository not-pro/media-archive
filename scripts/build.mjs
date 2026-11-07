import { spawnSync } from 'node:child_process';
import path from 'node:path';

const cli = path.resolve('node_modules/@tauri-apps/cli/tauri.js');
const args = [cli, 'build', '--config', '.tmp/bundle-config.json'];
if (process.env.TAURI_TARGET) {
  args.push('--target', process.env.TAURI_TARGET);
}
const result = spawnSync(process.execPath, args, { stdio: 'inherit', shell: false });
if (result.error) throw result.error;
process.exitCode = result.status ?? 1;
