# Explicit Entry Point Handling in webpack_analyzer_v2

This document describes the new explicit entry point handling functionality that has been added to the webpack_analyzer_v2 crate.

## Overview

The explicit entry point handling feature allows you to define entry points through configuration without any filename pattern matching, heuristics, or inference. This ensures precise control over which modules are considered entry points.

## Key Components

### ShareUsageConfig

A new configuration structure that contains explicit entry point information:

```rust
use webpack_analyzer_v2::ShareUsageConfig;
use swc_core::atoms::Atom;

let config = ShareUsageConfig {
    entry_module_ids: vec![
        Atom::from("./src/index.js"),
        Atom::from("./src/entry.js"),
    ],
};
```

### ChunkCharacteristics Enhancement

The `ChunkCharacteristics` struct now includes an optional `entry_module_id` field:

```rust
let characteristics = ChunkCharacteristics {
    // ... other fields
    entry_module_id: Some("./src/main.js".to_string()),
};
```

## Methods

### extract_explicit_entry_points

Extracts entry points using ONLY explicit configuration. Returns empty Vec if no explicit entry points are found.

```rust
use webpack_analyzer_v2::{WebpackChunk, ShareUsageConfig};

let entry_points = chunk.extract_explicit_entry_points(&config);
// Returns: Vec<ModuleId> containing only explicitly configured entry points
```

**Key characteristics:**
- NO filename pattern matching
- NO heuristics or inference
- Only returns entry points that exist in both the configuration and the chunk
- Returns empty Vec if no explicit entry points are configured

### extract_explicit_entry_points_strict

Similar to `extract_explicit_entry_points` but with strict error handling for missing entry points.

```rust
let result = chunk.extract_explicit_entry_points_strict(&config);
match result {
    Ok(entry_points) => {
        // All configured entry points were found
        println!("Found {} entry points", entry_points.len());
    }
    Err(e) => {
        // Some configured entry points were missing from the chunk
        eprintln!("Error: {}", e);
    }
}
```

**Error conditions:**
- Returns error if any configured entry points are missing from the chunk
- Returns error if no entry points are configured at all

## Integration with DependencyGraph

The extracted entry points work seamlessly with the existing `DependencyGraph::get_reachable_from_multiple()` method:

```rust
use webpack_analyzer_v2::{DependencyGraph, ShareUsageConfig};

// Extract explicit entry points
let config = ShareUsageConfig {
    entry_module_ids: vec![Atom::from("./src/index.js")],
};
let entry_points = chunk.extract_explicit_entry_points(&config);

// Use with dependency graph
let reachable_modules = dependency_graph.get_reachable_from_multiple(&entry_points);
```

## Usage Examples

### Basic Usage

```rust
use webpack_analyzer_v2::{WebpackChunk, ShareUsageConfig, ChunkType};
use swc_core::atoms::Atom;

// Create a chunk with modules
let mut chunk = WebpackChunk::new(ChunkType::WebpackModules, "source code".to_string());
// ... add modules to chunk

// Configure explicit entry points
let config = ShareUsageConfig {
    entry_module_ids: vec![
        Atom::from("./src/index.js"),
        Atom::from("./src/worker.js"),
    ],
};

// Extract entry points
let entry_points = chunk.extract_explicit_entry_points(&config);
println!("Found {} entry points", entry_points.len());
```

### With Error Handling

```rust
// Use strict version for error handling
match chunk.extract_explicit_entry_points_strict(&config) {
    Ok(entry_points) => {
        // Process entry points
        for entry_point in &entry_points {
            println!("Entry point: {}", entry_point);
        }
    }
    Err(e) => {
        eprintln!("Failed to extract entry points: {}", e);
        // Handle missing entry points
    }
}
```

### Integration with Tree Shaking

```rust
// Extract explicit entry points
let entry_points = chunk.extract_explicit_entry_points(&config);

// Build dependency graph
let mut graph = DependencyGraph::new();
for (_, module) in &chunk.modules {
    graph.add_module(module.clone());
}

// Find reachable modules from entry points
let reachable = graph.get_reachable_from_multiple(&entry_points);

// Find orphaned modules that can be tree-shaken
let orphaned = graph.find_orphaned_modules(&entry_points);
println!("Can remove {} orphaned modules", orphaned.len());
```

## Important Notes

1. **No Inference**: The methods perform NO filename pattern matching or heuristic-based inference
2. **Explicit Only**: Only modules explicitly listed in the configuration are considered entry points
3. **Existence Check**: Entry points must exist in the chunk to be returned
4. **Error Handling**: Use the strict version when you need to catch configuration errors
5. **Integration**: Works seamlessly with existing `DependencyGraph` methods

## Backward Compatibility

This enhancement is fully backward compatible. Existing code continues to work unchanged, and the new functionality is opt-in through the new methods and configuration structures.