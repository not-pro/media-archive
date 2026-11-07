import { spawnSync } from 'node:child_process';

const executable = process.platform === 'win32' ? 'npx.cmd' : 'npx';
const args = ['tauri', 'build', '--config', '.tmp/bundle-config.json'];
if (process.env.TAURI_TARGET) {
  args.push('--target', process.env.TAURI_TARGET);
}
const result = spawnSync(executable, args, { stdio: 'inherit', shell: false });
if (result.error) throw result.error;
process.exitCode = result.status ?? 1;
