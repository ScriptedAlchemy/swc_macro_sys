import { execSync } from 'node:child_process';

export default async function globalSetup() {
  try {
    // Proactively free ports used by host and remote before starting webServers
    execSync('npx -y kill-port 3001 3002', { stdio: 'ignore' });
  } catch {}
}


