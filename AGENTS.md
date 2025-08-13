# Agent Development Guide

This guide provides all commands needed to build, test, and develop the SWC Macro System. All commands use `pnpm` for JavaScript/Node.js operations and `cargo` for Rust operations.

## Prerequisites

```bash
# Required tools
- Node.js 16+ with pnpm
- Rust toolchain (rustc, cargo)
- wasm-pack for WASM builds

# Install pnpm if not present
npm install -g pnpm

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Quick Start

```bash
# Install all dependencies
pnpm install

# Build everything (Rust + WASM + JS)
pnpm build

# Run all tests
pnpm test:ci  # Use this in CI environments (no npm-run-all required)
pnpm test     # Use this locally (requires npm-run-all)
```

## Build Commands

### Full Build Pipeline

```bash
# Build everything in correct order
pnpm build

# Individual build steps
pnpm build:rust   # Build Rust crates
pnpm build:wasm   # Build WASM module
pnpm build:js     # Build JavaScript examples
```

### Rust/Cargo Commands

```bash
# Build all Rust crates
cargo build --workspace

# Build release mode
cargo build --release --workspace

# Build specific crate
cargo build -p swc_macro_wasm
cargo build -p webpack_analyzer_v2
cargo build -p swc_macro_parser
cargo build -p swc_macro_condition_transform
```

### WASM Building

```bash
# Build WASM module (from root)
pnpm build:wasm

# Or directly with wasm-pack
cd crates/swc_macro_wasm && wasm-pack build --release

# Debug build (faster, with debug symbols)
cd crates/swc_macro_wasm && wasm-pack build --dev
```

### JavaScript/TypeScript Building

```bash
# Build all JavaScript examples
pnpm build:js

# Build specific examples
pnpm -C examples/module-federation-example build
pnpm -C examples/module-federation-react-example build
pnpm -C examples/module-federation-web-example build
pnpm -C examples/rsbuild-project build
```

## Test Commands

### All Tests

```bash
# Run all tests (local environment with npm-run-all)
pnpm test

# Run all tests (CI environment, no npm-run-all needed)
pnpm test:ci

# Alternative for CI
./ci-test.sh
```

### Rust Tests

```bash
# Run all Rust tests
pnpm test:rust
# or
cargo test --workspace

# Run tests for specific crate
cargo test -p swc_macro_wasm
cargo test -p webpack_analyzer_v2

# Run specific test
cargo test test_webpack_tree_shaking_integration

# Run tests with output
cargo test --workspace -- --nocapture

# Run release mode tests (optimized)
cargo test --release --workspace
```

### JavaScript/WASM Tests

```bash
# Run all JavaScript tests
pnpm test:js

# Run workspace tests
pnpm test:workspaces

# Run tests in specific package
pnpm -C examples/module-federation-example test
pnpm -C examples/module-federation-react-example test
pnpm -C examples/module-federation-web-example test

# Run test with coverage
pnpm test:coverage

# Run tests in watch mode
pnpm test:watch
```

### Unit vs Integration Tests

```bash
# Unit tests only
pnpm test:unit

# Integration tests only
pnpm test:integration

# E2E tests
pnpm test:e2e
```

## Development Commands

### Running Examples

```bash
# Run WASM example
pnpm start
# or
node --experimental-wasm-modules example_wasm_usage.js

# Run Module Federation examples in dev mode
pnpm -C examples/module-federation-example dev
pnpm -C examples/module-federation-react-example dev
```

### Optimization Testing

```bash
# Optimize Module Federation chunks
pnpm -C examples/module-federation-example optimize
pnpm -C examples/module-federation-react-example optimize

# Build and optimize
pnpm -C examples/module-federation-example build:optimized
```

### Debugging WASM Issues

```bash
# Test simple WASM functionality
node --experimental-wasm-modules test-simple-treeshaker.js
node --experimental-wasm-modules test-no-treeshake.js

# Debug with enhanced logging (when implemented)
RUST_LOG=debug pnpm test

# Check WASM module size
ls -lh crates/swc_macro_wasm/pkg/*.wasm
```

## Code Quality Commands

### Linting

```bash
# Lint Rust code
cargo clippy --workspace -- -D warnings

# Lint JavaScript
pnpm lint:js

# Lint everything
pnpm lint
```

### Formatting

```bash
# Format Rust code
cargo fmt --all

# Check formatting without changes
cargo fmt --all -- --check
```

### Clean

```bash
# Clean everything
pnpm clean

# Clean Rust build artifacts
cargo clean

# Clean JavaScript artifacts
pnpm clean:js

# Clean specific package
pnpm -C examples/module-federation-example clean
```

## CI-Specific Commands

```bash
# Install dependencies without scripts
pnpm install --frozen-lockfile --prefer-offline

# Build for CI
pnpm build

# Test for CI (doesn't require npm-run-all)
pnpm test:ci

# Or use the shell script
chmod +x ci-test.sh
./ci-test.sh
```

## Troubleshooting Commands

### Check Dependencies

```bash
# Check outdated packages
pnpm outdated

# Check Rust dependencies
cargo outdated

# Verify workspace setup
pnpm ls --depth 0

# Check WASM module exports
node -e "const wasm = require('./crates/swc_macro_wasm/pkg/swc_macro_wasm.js'); console.log(Object.keys(wasm));"
```

### Fix Common Issues

```bash
# Rebuild WASM after Rust changes
pnpm build:wasm

# Clear all caches and rebuild
pnpm clean && pnpm install && pnpm build

# Fix permission issues
chmod +x ci-test.sh
chmod +x examples/*/scripts/*.js

# Reinstall dependencies
rm -rf node_modules pnpm-lock.yaml && pnpm install
```

### Verify Installation

```bash
# Check Node version
node --version  # Should be 16+

# Check pnpm
pnpm --version

# Check Rust
rustc --version
cargo --version

# Check wasm-pack
wasm-pack --version

# Verify WASM support in Node
node --experimental-wasm-modules -e "console.log('WASM supported')"
```

## Git Commands

### Working with Remotes

```bash
# View all remotes
git remote -v

# Push to upstream (ScriptedAlchemy)
git push upstream federation-swc-optimization

# Push to origin (your fork)
git push origin federation-swc-optimization

# Push to all remotes
git push upstream federation-swc-optimization && git push origin federation-swc-optimization

# Fetch from upstream
git fetch upstream
git merge upstream/main
```

### Creating Commits

```bash
# Stage changes
git add -A

# Create commit with conventional format
git commit -m "feat: add new tree shaking optimization"
git commit -m "fix: resolve WASM panic in parser"
git commit -m "docs: update AGENTS.md with new commands"
git commit -m "test: add unit tests for webpack analyzer"
git commit -m "refactor: consolidate tree shaking logic"
```

## Package-Specific Commands

### swc_macro_wasm

```bash
# Build
cargo build -p swc_macro_wasm --release

# Test
cargo test -p swc_macro_wasm

# Build WASM
cd crates/swc_macro_wasm && wasm-pack build --release

# Run example
cargo run -p swc_macro_wasm --example transform
```

### webpack_analyzer_v2

```bash
# Build
cargo build -p webpack_analyzer_v2

# Test
cargo test -p webpack_analyzer_v2

# Run specific test suite
cargo test -p webpack_analyzer_v2 chunk_cases
```

### Module Federation Examples

```bash
# Install deps
pnpm -C examples/module-federation-example install

# Build
pnpm -C examples/module-federation-example build

# Optimize
pnpm -C examples/module-federation-example optimize

# Test
pnpm -C examples/module-federation-example test

# Dev mode
pnpm -C examples/module-federation-example dev
```

## Performance Testing

```bash
# Run benchmarks
cargo bench --workspace

# Test optimization performance
pnpm -C examples/module-federation-example test:performance

# Profile WASM execution
node --prof --experimental-wasm-modules example_wasm_usage.js
node --prof-process isolate-*.log
```

## Release Process

```bash
# 1. Run all tests
pnpm test:ci

# 2. Build release artifacts
cargo build --release --workspace
pnpm build:wasm

# 3. Verify WASM module
node --experimental-wasm-modules -e "const w = require('./crates/swc_macro_wasm/pkg/swc_macro_wasm.js'); console.log('WASM OK');"

# 4. Update version (if needed)
cargo update -p swc_macro_wasm

# 5. Commit and push
git add -A
git commit -m "chore: prepare release"
git push upstream main
```

## Environment Variables

```bash
# Enable debug logging
RUST_LOG=debug pnpm test

# Set Node options for WASM
NODE_OPTIONS="--experimental-wasm-modules" pnpm test

# Skip pre/post install scripts
SKIP_INSTALL_SCRIPTS=true pnpm install
```

## Docker Commands (if using Docker)

```bash
# Build Docker image
docker build -t swc-macro-system .

# Run tests in Docker
docker run --rm swc-macro-system pnpm test:ci

# Interactive shell
docker run --rm -it swc-macro-system /bin/bash
```

## Notes for CI Environments

1. **Always use `pnpm test:ci`** instead of `pnpm test` in CI
2. **Install with frozen lockfile**: `pnpm install --frozen-lockfile`
3. **No npm-run-all needed**: The `test:ci` command doesn't require it
4. **Use the ci-test.sh script** as an alternative

## Common Workflows

### After Pulling Changes

```bash
pnpm install
pnpm build:wasm
pnpm test:ci
```

### Before Pushing Changes

```bash
cargo fmt --all
cargo clippy --workspace -- -D warnings
pnpm test:ci
git add -A
git commit -m "feat: your change description"
git push upstream federation-swc-optimization
```

### Debugging WASM Panics

```bash
# 1. Check if it works without tree shaking
node --experimental-wasm-modules test-no-treeshake.js

# 2. Test with simple code
node --experimental-wasm-modules test-simple-treeshaker.js

# 3. Check debug output (when available)
RUST_LOG=debug node --experimental-wasm-modules test-treeshaker-debug.js

# 4. Rebuild WASM with debug symbols
cd crates/swc_macro_wasm && wasm-pack build --dev
```

---

**Remember**: Always use `pnpm` for JavaScript operations, never `npm` directly!