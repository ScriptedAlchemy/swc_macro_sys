import { execSync } from 'node:child_process';

export default async function globalTeardown() {
  try {
    execSync('npx -y kill-port 3000 3001', { stdio: 'ignore' });
  } catch {}
}