#!/usr/bin/env node

import { spawn } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(__dirname, '..');

console.log('🚀 Module Federation React E2E Test Runner\n');

// Check if applications are built
function checkBuilds() {
  const hostDist = path.join(projectRoot, 'host/dist');
  const remoteDist = path.join(projectRoot, 'remote/dist');
  
  if (!fs.existsSync(hostDist) || !fs.existsSync(remoteDist)) {
    console.log('❌ Missing build artifacts. Running build first...');
    return false;
  }
  
  console.log('✅ Build artifacts found');
  return true;
}

// Run command and return promise
function runCommand(command, args, options = {}) {
  return new Promise((resolve, reject) => {
    console.log(`Running: ${command} ${args.join(' ')}`);
    const child = spawn(command, args, {
      stdio: 'inherit',
      shell: true,
      ...options
    });
    
    child.on('close', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`Command failed with exit code ${code}`));
      }
    });
    
    child.on('error', reject);
  });
}

// Main test runner
async function runE2ETests() {
  try {
    // Step 1: Check builds
    if (!checkBuilds()) {
      console.log('Building applications...');
      await runCommand('pnpm', ['build'], { cwd: projectRoot });
      
      console.log('Running optimization...');
      await runCommand('pnpm', ['optimize'], { cwd: projectRoot });
    }
    
    // Step 2: Install Playwright if needed
    console.log('Ensuring Playwright is installed...');
    try {
      await runCommand('npx', ['playwright', 'install'], { cwd: projectRoot });
    } catch (error) {
      console.log('Installing Playwright...');
      await runCommand('pnpm', ['add', '-D', '@playwright/test'], { cwd: projectRoot });
      await runCommand('npx', ['playwright', 'install'], { cwd: projectRoot });
    }
    
    // Step 3: Run E2E tests
    console.log('\n🧪 Running Playwright E2E tests...');
    
    const testArgs = process.argv.slice(2);
    const playwrightArgs = ['playwright', 'test'];
    
    // Add common args
    if (!testArgs.includes('--headed') && !testArgs.includes('--ui')) {
      playwrightArgs.push('--reporter=html');
    }
    
    // Add user args
    playwrightArgs.push(...testArgs);
    
    await runCommand('npx', playwrightArgs, { cwd: projectRoot });
    
    console.log('\n✅ E2E tests completed successfully!');
    console.log('\n📊 View the test report at: playwright-report/index.html');
    
  } catch (error) {
    console.error('\n❌ E2E tests failed:', error.message);
    process.exit(1);
  }
}

// Handle different run modes
const mode = process.argv[2];

switch (mode) {
  case 'ui':
    console.log('Running tests in UI mode...');
    process.argv.push('--ui');
    break;
  case 'headed':
    console.log('Running tests in headed mode...');
    process.argv.push('--headed');
    break;
  case 'debug':
    console.log('Running tests in debug mode...');
    process.argv.push('--debug');
    break;
  default:
    console.log('Running tests in headless mode...');
}

// Run the tests
runE2ETests();
