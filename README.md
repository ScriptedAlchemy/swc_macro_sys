## SWC Macro Sys

This project is a comprehensive macro system implementation for [swc macro proposal](https://github.com/swc-project/swc/issues/10519), which is used for parsing and transforming JavaScript code based on [swc](https://github.com/swc-project/swc). It includes advanced webpack tree shaking capabilities specifically designed for Module Federation and modern bundler outputs.

**Key Features:**
- **Advanced Tree Shaking**: Reduces bundle sizes by 30-70% for typical applications
- **Module Federation Support**: Optimized for micro-frontend architectures
- **AST-Based Analysis**: Uses SWC for precise JavaScript parsing and dependency tracking
- **Production Ready**: Tested with real-world webpack/rspack bundles

## Build & Setup

### Quick Start

```bash
# Install dependencies
pnpm install

# Build all packages (Rust + JavaScript)
pnpm build

# Run all tests
pnpm test

# Run tests in CI mode
pnpm test:ci
```

### WASM binding

```sh
# Once: Add WASM target
rustup target add wasm32-unknown-unknown

# Build the WASM binding
(cd crates/swc_macro_wasm && wasm-pack build --release)

# Your wasm file will be in `crates/swc_macro_wasm/pkg/`
```

**Requirements:**
- Node.js v20+ recommended for best WASM support
- Use `--experimental-wasm-modules` flag for WASM optimization to work

## Project Structure

```
swc_macro_sys/
├── crates/
│   ├── swc_macro_wasm/          # WASM bindings for JavaScript integration
│   ├── webpack_graph/           # Webpack bundle parsing and dependency graph
│   ├── webpack_analyzer_v2/     # Advanced webpack chunk analysis
│   └── webpack_chunk_tree_shaker/  # Tree shaking implementation
├── module-federation-example/    # Complete Module Federation demo
├── examples/                    # Various usage examples
├── test-cases/                  # Real-world test bundles
└── TESTING.md                   # Comprehensive testing guide
```

### Core Crates

- **`swc_macro_wasm`**: WASM bindings for JavaScript integration with macro processing
- **`webpack_graph`**: Webpack bundle parsing and dependency graph construction
- **`webpack_analyzer_v2`**: Advanced webpack chunk analysis with AST parsing
- **`webpack_chunk_tree_shaker`**: Tree shaking implementation for webpack bundles

## Examples

### Module Federation with Tree Shaking

Complete example demonstrating Module Federation with advanced tree shaking:

```bash
# Navigate to the example
cd module-federation-example

# Install dependencies
pnpm install

# Build and optimize shared chunks
pnpm build:optimized

# Run the demo applications
pnpm dev
```

This demonstrates:
- **Module Federation setup** with lodash-es sharing
- **Tree shaking optimization** reducing bundle size by 30-70%
- **Real-world usage** with host/remote applications
- **Bundle analysis** with detailed size reduction reports

### Tree-Shaking Demo

Test tree-shaking with real webpack bundles:

```bash
# Build WASM module first
pnpm build

# Run optimization on test cases
node --experimental-wasm-modules scripts/optimize-shared-chunks.js
```

This demonstrates:
- **AST-based analysis** using SWC for precise module detection
- **Dependency tracking** with webpack require analysis
- **Multiple webpack formats** (CommonJS, JSONP, WebpackModules)
- **Size analysis** showing bundle size reduction metrics

### Rust Transform Example

Check `crates/swc_macro_condition_transform` to see how this crate works to handle the macro annotations.

Run `cargo run --example transform` with the following input javascript code:

```js
/* @common:if [condition="featureFlags.enableNewFeature"] */
export function newFeature() {
  return "New feature is enabled!";
}
/* @common:endif */

const buildTarget =
  /* @common:define-inline [value="build.target" default="development"] */ "development";
```

The expected output is:

```js
const buildTarget = "production";
```

### Node.js JSX Demo

The `examples/jsx-test-server.mjs` demonstrates:

- **JSX Transformation**: Using SWC to transform JSX syntax to React.createElement calls
- **Macro Processing**: Applying conditional compilation and variable substitution
- **Component Rendering**: Server-side rendering of React components to HTML

Features demonstrated:
- Complex nested conditional blocks (`@common:if`/`@common:endif`)
- Platform-specific code paths (mobile/desktop)
- Feature flag conditional compilation
- A/B testing variants
- User type-based feature access
- Inline variable substitution (`@common:define-inline`)

Run the demo to see how the macro system can optimize bundle size by eliminating unused code paths at build time.

## Testing

This project includes comprehensive testing across Rust and JavaScript:

```bash
# Run all tests
pnpm test

# Run tests with coverage
pnpm test:coverage

# Run tests in watch mode
pnpm test:watch

# Run specific test types
pnpm test:unit        # Unit tests
pnpm test:integration # Integration tests
pnpm test:e2e         # End-to-end tests
```

For detailed testing information, see [TESTING.md](TESTING.md).

## Performance Results

Real-world performance improvements from the test suite:

| Bundle Type | Original Size | Optimized Size | Reduction |
|-------------|---------------|----------------|-----------|
| Lodash-ES Vendor | 5.3KB | 1.7KB | 67.9% |
| Module Federation | 1.4MB | 12.2KB | 99.2% |
| RSpack Bundle | 1.2MB | 69KB | 94.3% |

## API Reference

### JavaScript Usage (WASM)

```javascript
import { optimize } from 'swc_macro_wasm';

const optimizedCode = optimize(webpackBundle, {
  entryModules: {
    "lodash-es": "../../node_modules/lodash-es/lodash.js"
  }
});
```

### Rust Usage

```rust
use webpack_chunk_tree_shaker::*;
use webpack_analyzer_v2::WebpackAnalyzer;

let analyzer = WebpackAnalyzer::new();
let chunk = analyzer.analyze_chunk(webpack_source)?;
let shaker = WebpackTreeShaker::new();
let result = shaker.shake_tree(&chunk, &["entry-module"])?;
```

