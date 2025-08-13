# Product Requirements Document: WebAssembly-Compatible Tree Shaking System

**Document Version**: 1.0  
**Date**: December 2024  
**Project**: SWC Macro System Tree Shaking with WASM Support  
**Repository**: github.com/ScriptedAlchemy/swc_macro_sys

---

## Executive Summary

This PRD documents the requirements, challenges, and implementation strategy for a WebAssembly-compatible tree shaking system for webpack bundles with Module Federation support. The system must optimize JavaScript bundle sizes by removing unused code while maintaining full compatibility with WASM runtimes in browser and Node.js environments.

---

## Problem Statement

### Current State (December 2024)

The tree shaking system experiences **critical runtime failures** when compiled to WebAssembly:

1. **WASM Runtime Panics**: The system triggers "RuntimeError: unreachable" errors when processing webpack bundles in WASM
2. **Parser Incompatibility**: SWC's parser and the webpack_analyzer_v2 crate cause uncatchable WASM runtime errors
3. **Thread-Local Storage Issues**: GLOBALS and thread-local storage mechanisms used by SWC are incompatible with WASM's execution model
4. **Debug Visibility**: Standard println!/eprintln! macros don't work in WASM, making debugging extremely difficult

### Historical Context

#### Original Working State (Pre-June 2025)

The system originally worked with a **simpler architecture**:

```rust
// Original simple approach (worked in WASM)
- Direct string manipulation for module removal
- Regex-based pattern matching for webpack structures
- No complex AST parsing in the analyzer
- Single-pass optimization
```

**Key Characteristics of Working Version**:
- Used basic regex patterns to identify webpack modules
- Performed string replacement to remove unused modules
- Avoided complex SWC parser initialization in analyzer
- All parsing wrapped in GLOBALS.set() at a single entry point

#### Evolution Timeline

1. **Initial Implementation** (Early 2025)
   - Simple regex-based tree shaking
   - Direct string manipulation
   - Worked reliably in WASM

2. **Architecture Expansion** (Mid-2025)
   - Added webpack_chunk_tree_shaker crate
   - Introduced complex AST analysis
   - Added multi-pass optimization
   - **WASM compatibility broke here**

3. **Consolidation Attempt** (November 2025)
   - Removed webpack_chunk_tree_shaker crate
   - Consolidated into swc_macro_wasm/src/optimize.rs
   - Added webpack_analyzer_v2 for chunk analysis
   - **WASM issues persisted**

4. **Emergency Fix** (Commit d046e5c)
   - Implemented simple_tree_shake.rs as fallback
   - Bypassed complex TreeShaker for WASM builds
   - Restored basic functionality

---

## Root Cause Analysis

### Why WASM Fails

The fundamental issue lies in **multiple nested parser initializations**:

```rust
// PROBLEMATIC PATTERN - Multiple GLOBALS contexts
optimize.rs:
  GLOBALS.set() {                    // Context 1
    Parser::new().parse_program()    // Works
    TreeShaker::optimize() {
      analyzer.analyze_chunk() {
        parse_source() {
          GLOBALS.set() {            // Context 2 - NESTED!
            Parser::new()            // WASM PANIC HERE
          }
        }
      }
    }
  }
```

**WASM Limitations**:
1. Cannot handle nested GLOBALS contexts
2. Thread-local storage doesn't work as expected
3. Panic unwinding mechanisms fail silently
4. Memory management differs from native execution

### Why It Worked Before

The original implementation **avoided the problem entirely**:

```rust
// WORKING PATTERN - Single parser context
optimize.rs:
  GLOBALS.set() {                    // Single context
    Parser::new().parse_program()    // Parse once
    // All subsequent operations on existing AST
    // No re-parsing of chunks
    // String manipulation for module removal
  }
```

---

## Requirements

### Functional Requirements

#### Core Functionality

1. **Tree Shaking**
   - Remove unused webpack modules from bundles
   - Support CommonJS, JSONP, and ESM chunk formats
   - Handle Module Federation shared chunks
   - Preserve entry modules and their dependencies

2. **Macro Processing**
   - Process @common:if/@common:endif conditional compilation
   - Evaluate conditions against configuration
   - Remove or preserve code blocks based on evaluation

3. **Module Federation Support**
   - Handle shared modules correctly
   - Preserve singleton patterns
   - Support remote entry points
   - Maintain version requirements

#### WASM Compatibility

1. **Runtime Stability**
   - Zero panics in WASM execution
   - Graceful error handling and recovery
   - Fallback to original source on failure

2. **Parser Management**
   - Single GLOBALS context for all parsing
   - No nested parser initialization
   - Reuse parsed AST throughout pipeline

3. **Debug Support**
   - WASM-compatible logging mechanism
   - Error context preservation
   - Performance metrics collection

### Non-Functional Requirements

#### Performance

1. **Optimization Targets**
   - Process 1.5MB vendor chunks in <2 seconds
   - Achieve 30-99% size reduction for unused libraries
   - Support incremental optimization passes

2. **Memory Constraints**
   - Stay within WASM memory limits (4GB max)
   - Efficient AST representation
   - Minimal string allocations

#### Compatibility

1. **Environment Support**
   - Node.js with --experimental-wasm-modules
   - Modern browsers (Chrome 90+, Firefox 89+, Safari 15+)
   - Webpack 5.x output formats
   - Rspack/Turbopack compatibility

2. **Bundle Format Support**
   - CommonJS async/sync patterns
   - JSONP push patterns
   - ESM export patterns
   - Module Federation runtimes

---

## Proposed Solution

### Architecture Design

```
┌─────────────────────────────────────────────────────────┐
│                    JavaScript Source                      │
└────────────────────┬───────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                  optimize() Entry Point                   │
│                  [Single GLOBALS Context]                 │
├───────────────────────────────────────────────────────────┤
│  1. Parse source ONCE with SWC Parser                    │
│  2. Extract macros from comments                         │
│  3. Apply condition transformation                       │
│  4. Create ChunkInfo from AST (no re-parsing)          │
└────────────────────┬───────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                  TreeShaker Pipeline                      │
├───────────────────────────────────────────────────────────┤
│  1. Build dependency graph from AST                      │
│  2. Identify unreachable modules                         │
│  3. Mutate AST to remove modules                        │
│  4. Apply DCE pass                                      │
│  5. Run fixer pass                                      │
└────────────────────┬───────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                  Emit Optimized Code                      │
└─────────────────────────────────────────────────────────┘
```

### Implementation Strategy

#### Phase 1: Fix WASM Parser Issues (Immediate)

1. **Remove Nested GLOBALS**
   ```rust
   // BEFORE (causes WASM panic)
   fn parse_source(&self, source: &str) -> Result<Program> {
       GLOBALS.set(&Default::default(), || {  // NESTED!
           Parser::new(...).parse_program()
       })
   }
   
   // AFTER (WASM-safe)
   fn analyze_from_ast(&self, program: &Program) -> Result<WebpackChunk> {
       // Work with existing AST, no re-parsing
   }
   ```

2. **Single Parse Architecture**
   - Parse source once at entry point
   - Pass AST through entire pipeline
   - Extract module info from AST nodes
   - Never re-parse chunk strings

#### Phase 2: Refactor webpack_analyzer_v2 (Week 1)

1. **AST-Based Analysis**
   ```rust
   impl WebpackAnalyzer {
       pub fn analyze_from_program(
           &self, 
           program: &Program,
           characteristics: ChunkCharacteristics
       ) -> Result<WebpackChunk> {
           // Extract modules from existing AST
           // Build dependency graph from AST
           // No string parsing required
       }
   }
   ```

2. **Remove String-Based Methods**
   - Delete `analyze_chunk(source: &str)`
   - Delete `parse_source()`
   - Work only with AST nodes

#### Phase 3: Implement WASM Logging (Week 1)

1. **Add web_sys Console Support**
   ```rust
   #[cfg(target_arch = "wasm32")]
   macro_rules! log {
       ($($t:tt)*) => {
           web_sys::console::log_1(&format!($($t)*).into());
       }
   }
   
   #[cfg(not(target_arch = "wasm32"))]
   macro_rules! log {
       ($($t:tt)*) => {
           println!($($t)*);
       }
   }
   ```

2. **Debug Instrumentation**
   - Add timing measurements
   - Track memory usage
   - Log optimization stages

#### Phase 4: Convergence & Safety (Week 2)

1. **Prevent Oscillation**
   - Hash AST after each iteration
   - Detect repeated states
   - Limit iterations to 5 max
   - Early exit on convergence

2. **Error Recovery**
   - Wrap all operations in Result types
   - Catch and log WASM-specific errors
   - Always return valid JavaScript

---

## Testing Requirements

### Unit Tests

1. **Parser Safety**
   - Verify single GLOBALS context
   - Test AST reuse patterns
   - Validate no nested parsing

2. **Tree Shaking Accuracy**
   - CommonJS module removal
   - JSONP chunk optimization
   - ESM export handling
   - Module Federation preservation

### Integration Tests

1. **WASM Execution**
   ```javascript
   // Node.js WASM test
   import { optimize } from './pkg/swc_macro_wasm.js';
   
   const result = optimize(webpackChunk, config);
   assert(result.includes('__webpack_require__'));
   assert(!result.includes('unused_module'));
   ```

2. **Real Bundle Tests**
   - lodash-es (200+ exports → 5 used)
   - React DOM (complex dependencies)
   - Module Federation remotes
   - Vendor chunks with circular deps

### Performance Benchmarks

| Test Case | Input Size | Target Time | Reduction |
|-----------|------------|-------------|-----------|
| lodash-es vendor | 1.5MB | <2s | >95% |
| React DOM bundle | 120KB | <500ms | >30% |
| MF shared chunk | 500KB | <1s | >60% |
| Small bundle | 50KB | <200ms | Any |

---

## Success Criteria

### Must Have (P0)

1. ✅ Zero WASM runtime panics
2. ✅ Tree shaking works for all chunk formats
3. ✅ Module Federation compatibility
4. ✅ Graceful error handling
5. ✅ Performance within targets

### Should Have (P1)

1. ⚠️ WASM debug logging
2. ⚠️ Optimization metrics
3. ⚠️ Convergence detection
4. ⚠️ Configuration validation

### Nice to Have (P2)

1. ❌ Visual bundle analysis
2. ❌ Incremental optimization
3. ❌ Custom transform plugins
4. ❌ Source map support

---

## Risk Assessment

### High Risk

1. **SWC Version Incompatibility**
   - Mitigation: Pin to known working version
   - Fallback: Implement custom minimal parser

2. **Memory Exhaustion in WASM**
   - Mitigation: Streaming processing
   - Fallback: Chunk size limits

### Medium Risk

1. **Performance Regression**
   - Mitigation: Comprehensive benchmarks
   - Fallback: Simple tree shake mode

2. **Breaking Changes in Webpack Output**
   - Mitigation: Format detection
   - Fallback: Conservative optimization

---

## Timeline

### Week 1: Foundation
- Fix nested GLOBALS issue
- Refactor analyzer to use AST
- Add WASM logging

### Week 2: Implementation
- Complete AST-based analysis
- Add convergence detection
- Implement error recovery

### Week 3: Testing
- Unit test coverage
- Integration tests
- Performance benchmarks

### Week 4: Optimization
- Performance tuning
- Memory optimization
- Documentation

---

## Appendix

### Related Documents
- TREE_SHAKING_DESIGN.md - Original design specification
- IMPLEMENTATION_AUDIT.md - Current implementation status
- CLAUDE.md - AI assistant guidelines

### Key Commits
- `d046e5c` - Emergency WASM fix with simple tree shaking
- `428d70e` - Removed webpack_chunk_tree_shaker crate
- `1642cbe` - Last known fully working WASM version

### Technical Constraints
- WASM memory limit: 4GB
- Node.js flag required: --experimental-wasm-modules
- SWC version: swc_core = "13.0.5"
- wasm-bindgen version: "0.2.100"

---

**Document Status**: DRAFT - Pending Review  
**Next Review**: January 2025  
**Owner**: Module Federation Team