# Chunk Characteristics Schema Documentation

This document defines the required schema for `chunk_characteristics` in share-usage.json and tree-shake configuration files.

## Required Fields

Based on the source of truth file: `/Users/bytedance/dev/swc_macro_sys/examples/module-federation-react-example/host/dist/share-usage.json`

### Complete Schema Structure

```json
"chunk_characteristics": {
  "entry_module_id": "string",        // Path to the entry module (e.g., "../../../node_modules/.pnpm/react@18.3.1/node_modules/react/index.js")
  "is_runtime_chunk": boolean,        // Whether this is a runtime chunk (typically false for library chunks)
  "has_runtime": boolean,             // Whether chunk contains runtime code (typically false for library chunks)
  "is_entrypoint": boolean,           // Whether this is an entry point chunk (typically false for library chunks)
  "can_be_initial": boolean,          // Whether chunk can be loaded initially (typically false for library chunks)
  "is_only_initial": boolean,         // Whether chunk is only loaded initially (typically false for library chunks)
  "chunk_format": "string",           // Format type (e.g., "jsonp" for web bundles, "async-node" for node bundles)
  "chunk_loading_type": null|string,  // Loading mechanism (typically null)
  "runtime_names": ["string"],        // Array of runtime names (typically ["main"])
  "entry_name": null|string,          // Entry point name if applicable (typically null for library chunks)
  "has_async_chunks": boolean,        // Whether chunk has async dependencies (varies by library)
  "chunk_files": ["string"],          // Array of generated chunk file names
  "is_shared_chunk": boolean,         // Whether this is a shared chunk (typically false)
  "shared_modules": ["string"]        // Array of shared module identifiers (typically empty [])
}
```

## Common Values by Library Type

### React Libraries (react, react-dom, react-router-dom, react-redux)
- `has_async_chunks`: Often `true` due to lazy loading capabilities
- `chunk_format`: "jsonp"

### Utility Libraries (lodash-es, dayjs)
- `has_async_chunks`: Typically `false`
- `chunk_format`: "jsonp"

### UI Component Libraries (antd, @ant-design/icons, chart.js)
- `has_async_chunks`: Varies (true for antd, false for chart.js)
- `chunk_format`: "jsonp"

## File Locations Requiring This Schema

### Production/Build Output Files (Already Correct)
- `/examples/module-federation-react-example/host/dist/share-usage.json` ✅ (Source of Truth)
- `/examples/module-federation-react-example/remote/dist/share-usage.json`
- `/examples/module-federation-example/host/dist/share-usage.json`
- `/examples/module-federation-example/remote/dist/share-usage.json`
- `/examples/module-federation-web-example/host/dist/share-usage.json`
- `/examples/module-federation-web-example/remote/dist/share-usage.json`

### Test Case Files (Being Updated)
- `/test-cases/rspack-annotated-output/share-usage.json`
- `/test-cases/rspack-cjs-annotated-output/share-usage.json`

### Test Fixture Files (Need Update from simplified format)
- `/crates/swc_macro_wasm/tests/fixtures/standard_webpack_usage.json`
- `/crates/swc_macro_wasm/tests/fixtures/module_federation_usage.json`
- `/crates/swc_macro_wasm/tests/fixtures/module_federation_remote_usage.json`

Currently using simplified format: `{"chunk_format": "async-node"}` only

## Migration Notes

Files that currently have the simplified `chunk_characteristics` with only `"chunk_format": "async-node"` need to be updated to include all required fields as shown in the complete schema above.

## Validation Checklist

- [ ] All 14 required fields are present
- [ ] `entry_module_id` contains valid module path
- [ ] Boolean fields have appropriate values
- [ ] `chunk_format` is either "jsonp" or "async-node"
- [ ] `runtime_names` is an array (typically ["main"])
- [ ] `chunk_files` contains at least one file name
- [ ] `shared_modules` is an array (can be empty)