# Bob Engine

A high-performance game engine written in Rust, built around a custom Entity Component System (ECS) architecture.

## Overview

Bob Engine is designed with data-oriented principles, deterministic behavior, and modular systems to provide a flexible foundation for games and real-time simulations.

**Key Features:**

- Custom ECS architecture with optimized entity stores
- WGPU rendering integration for modern GPU-accelerated graphics
- 2D physics engine with joints and constraints
- Built-in asset management system
- RHAI scripting runtime support
- Type-safe component macros with serialization
- Modular design for easy extensibility

## Quick Start

```rust
use bob_engine::{runtime::Engine, AppContext};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let mut engine = Engine::new();
    let context = AppContext::default();

    // Initialize and run
    let app = App::new(engine, context, AssetStore::new());
    event_loop.run_app(&mut app)?;
    Ok(())
}
```

## Documentation

- [`USAGE.md`](./USAGE.md) — practical usage guide: quick start, ECS, systems, input, assets, rendering, GUI, physics, scenes, scripting, and gotchas
- [`FEATURES.md`](./FEATURES.md) — architecture details, core systems, build commands, and usage examples
