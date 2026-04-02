# CAELab Plugin SDK

## Overview

CAELab Plugin SDK provides a framework for extending CAELab's functionality through plugins. Plugins can register custom solvers, importers, exporters, and material definitions.

## Architecture

```
src-tauri/src/plugin/
  mod.rs          - Module definition
  interface.rs    - Core interfaces and data structures
  manager.rs      - Plugin lifecycle management
  api.rs          - Tauri commands for frontend communication
```

## Plugin Interfaces

### PluginInfo

```rust
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub loaded: bool,
    pub path: Option<String>,
    pub registered_solvers: Vec<String>,
    pub registered_importers: Vec<String>,
    pub registered_exporters: Vec<String>,
    pub registered_materials: Vec<String>,
}
```

### SolverInterface

Plugins can register solvers that support different analysis types:

```rust
pub struct SolverInput {
    pub solver_type: String,
    pub mesh_nodes: Vec<Vec<f64>>,
    pub mesh_elements: Vec<Vec<usize>>,
    pub materials: HashMap<String, Value>,
    pub boundary_conditions: Vec<Value>,
    pub loads: Vec<Value>,
    pub analysis_type: String,  // "static", "modal", "thermal"
    pub parameters: HashMap<String, Value>,
}

pub struct SolverOutput {
    pub success: bool,
    pub node_results: Option<Vec<Vec<f64>>>,
    pub element_results: Option<Vec<Vec<f64>>>,
    pub convergence: bool,
    pub iterations: usize,
    pub messages: Vec<String>,
    pub error: Option<String>,
}
```

### ImporterInterface / ExporterInterface

```rust
pub struct ImporterInput {
    pub file_path: String,
    pub options: HashMap<String, Value>,
}

pub struct ImporterOutput {
    pub success: bool,
    pub mesh_nodes: Vec<Vec<f64>>,
    pub mesh_elements: Vec<Vec<usize>>,
    pub metadata: HashMap<String, Value>,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}
```

## Frontend API

### Tauri Commands

| Command | Parameters | Returns |
|---------|-----------|---------|
| `list_plugins` | - | `Vec<PluginInfo>` |
| `load_plugin` | `path: String` | `Result<PluginInfo, String>` |
| `unload_plugin` | `name: String` | `Result<(), String>` |
| `get_plugin_info` | `name: String` | `Result<PluginInfo, String>` |

### Usage Example (TypeScript)

```typescript
import { invoke } from '@tauri-apps/api/core'

// List all plugins
const plugins = await invoke('list_plugins')

// Load a plugin
const info = await invoke('load_plugin', { path: '/path/to/plugin.so' })

// Unload a plugin
await invoke('unload_plugin', { name: 'my-plugin' })
```

## Third-Party Solver Integration Guide

### Step 1: Create Plugin Library

Create a new Rust library project:

```bash
cargo new --lib caelab-solver-custom
```

### Step 2: Implement Solver Interface

```rust
// lib.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[no_mangle]
pub extern "C" fn caelab_plugin_metadata() -> *const u8 {
    // Return JSON-serialized PluginMetadata
    todo!()
}

#[no_mangle]
pub extern "C" fn caelab_plugin_solve(input_json: *const u8, len: usize) -> *const u8 {
    // Parse input, run solver, return JSON output
    todo!()
}
```

### Step 3: Build

```bash
cargo build --release
```

The output will be a shared library:
- Linux: `libcaelab_solver_custom.so`
- macOS: `libcaelab_solver_custom.dylib`
- Windows: `caelab_solver_custom.dll`

### Step 4: Load in CAELab

Use the Plugin Manager UI or API to load the plugin file.

## Plugin Directory

Plugins are stored in the application data directory:

```
<app-data-dir>/plugins/
```

## Notes

- The current implementation provides the framework layer for plugin management
- Full dynamic library loading requires the `libloading` crate (planned for future versions)
- Plugin sandboxing and security policies will be added in future releases
- See `src/views/PluginManager.vue` for the frontend plugin management interface
