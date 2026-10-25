# Bob Engine Documentation

## Overview

Bob Engine is a high-performance game engine written in Rust, built around a custom **Entity Component System (ECS)** architecture. The engine is designed with data-oriented principles, deterministic behavior, and modular systems to provide a flexible foundation for games and real-time simulations.

---

## Core Features

### 1. Entity Component System (ECS)

## Dynamic World API Guide

The DynamicWorld is the core ECS runtime that manages all entities, components, and singleton instances in Bob Engine. It provides a type-safe API for entity lifecycle management, component registration, and query operations.

### Structure

- `DynamicWorld`: Main entity store managing entities, singletons, and component registrations
- `Entity`: Unique identifier for entities (0-indexed internal storage)
- `ComponentID`: Unique ID for registered component types

#### DynamicWorld Fields

- `singletons`: Collection of unique singleton instances in the world
- `storages`: Map from ComponentID to entity component storage arrays
- `alive`: HashSet tracking currently alive entity IDs
- `entities_count`: Total count of entities in the system
- `static_type_ids`: Pre-defined static type mappings for components
- **Entity Lifecycle Management**: Create, destroy, and manage entity states
- **Singleton Stores**: Unique resources with dedicated singletons collection
- **Components**: Type-safe components with built-in serialization support
- **Component Registration & Queries**: Dynamic component management with flexible querying
- **Systems**: Base and group-based system architecture for game logic

## Entity Lifecycle Management

### 2. Rendering System

- **WGPU Integration**: Modern hardware-accelerated rendering via WGPU (v29.0.0)
- **Renderer Component**: Handles window management, camera updates, and rendering loop
- **Dynamic Window Support**: Resizes automatically on window resize events

### 3. Asset Management

- **AssetStore**: Centralized asset management system
- **Embedded Assets**: Rust-embed support for bundled assets (e.g., PNG images)
- **Multiple Formats**: Supports images (PNG, Aseprite), scenes (.bscene), tiles, and textures

### 4. Physics System

- **2D Physics Engine**: Custom physics implementation
- **Joints and Constraints**: Support for rigid body dynamics with joint systems

### 5. Input Handling

- **Keyboard Input**: Full keyboard event processing
- **Mouse Input**: Button press/release and cursor position tracking
- **Window Events**: Resize detection and camera updates on resize

### 6. Scripting Runtime

- **Rhai Integration**: Embedded scripting language support (RHAI v1.19.0)
- **Macro Extensions**: Custom `Component!` derive macro for efficient component definition
- **Serialization**: Built-in serde support for component persistence

---

## Technical Stack

### Primary Dependencies

| Dependency | Version      | Purpose                               |
| ---------- | ------------ | ------------------------------------- |
| RHAI       | 1.19.0       | Embedded scripting runtime            |
| WGPU       | 29.0.0       | Modern GPU rendering backend          |
| Pollster   | 0.4.0        | Async runtime for blocking operations |
| winit      | 0.30         | Window and event loop system          |
| bytemuck   | 1.0 + derive | Zero-cost trait derivations           |

### Secondary Dependencies

| Dependency       | Purpose                        |
| ---------------- | ------------------------------ |
| paste            | Macro utilities                |
| serde/serde_json | Serialization/deserialization  |
| seahash          | Fast hashing for component IDs |
| rust-embed       | Embedded assets from resources |
| image            | Image format processing        |
| rand             | Random number generation       |
| serde_with       | Advanced serialization formats |
| rustc-hash       | Hash map lookups               |

### Development Profile Settings

```toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = true             # Link-time optimization
codegen-units = 1      # Single codegen unit for OOP (data-oriented)
panic = "abort"        # Fast failure on panic
strip = "symbols"      # Strip debug symbols for size
```

---

## Architecture Overview

### Component Structure

```
bob-engine/
├── src/
│   ├── app/              # Application framework (winit integration)
│   ├── app_context.rs    # Shared application state context
│   ├── constants.rs      # Global engine constants
│   ├── lib.rs            # Library entry point and macros
│   └── runtime/          # Core engine systems
│       ├── assets/       # Asset loading and management
│       ├── ecs/          # Entity Component System modules
│       │   ├── core_components/      # Base components (Transform, etc.)
│       │   ├── entities.rs           # Entity storage
│       │   ├── query.rs               # Query system
│       │   ├── singleton_store.rs     # Unique component stores
│       │   └── system_base.rs         # System abstraction
│       ├── engine/        # Engine orchestration
│       ├── rendering/     # WGPU renderer
│       ├── phys/          # 2D physics systems
│       ├── input/         # Input handling
│       ├── math/          # Mathematical utilities
│       ├── scene/         # Scene management
│       ├── gui/           # GUI rendering
│       └── scripting/     # RHAI scripting integration
```

### Usage Flow

1. **Initialize**: Create `Engine::new()` and set up the application context
2. **Load Assets**: Use `include_asset!` macro to embed assets into AssetStore
3. **Create App**: Instantiate `App::new(engine, context, asset_store)`
4. **Run Loop**: Let winit handle events, which triggers `engine.run()`

---

## Key Macros

### Component Derive Macro

```rust
#[derive(Component!)]
struct PlayerPosition {
    x: f32,
    y: f32,
}
```

This auto-generates:

- Clone, Default implementations
- `StableID` for component identification
- Copy trait implementation
- serde Serialize/Deserialize support
- RHAI CustomType support

---

### Example Application Setup

```rust
use bob_engine::{runtime::Engine, runtime::assets::AssetStore, AppContext};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize event loop
    let event_loop = EventLoop::new()?;

    // Create asset store and load assets
    let mut asset_store = AssetStore::new();
    include_asset!(&mut asset_store, "assets/Tux.png");

    // Setup app context
    let context = AppContext::default(); // 800x600 window

    // Initialize engine and app
    let mut engine = Engine::new();
    let mut app = App::new(engine, context, asset_store);

    // Run the event loop
    event_loop.run_app(&mut app)?;

    Ok(())
}
```

### Creating Components with Derive Macros

```rust
use bob_engine::*;

#[derive(Component!)]
struct PlayerPosition {
    x: f32,
    y: f32,
}

#[derive(Component!)]
struct PlayerVelocity {
    vx: f32,
    vy: f32,
}

// Custom components can also be created manually:
use bob_engine::runtime::ecs::component_store::ComponentStore;

fn create_custom_component<T>() -> ComponentStore<T> {
    ComponentStore::<T>::new()
}
```

### Manual Component Registration and Insertion

```rust
// First, register the component type in the world
world.register_component::<PlayerPosition>();

// Then insert it into a specific entity
let entity = world.create_entity();
world.add_component(entity, PlayerPosition { x: 0.0, y: 0.0 });

// Or use add_component_safe (registers automatically)
world.add_component_safe::<PlayerVelocity>(entity, PlayerVelocity { vx: 1.0, vy: 0.0 });

// Get components
let pos = world.get_component::<PlayerPosition>(entity);
let mut pos_mut = world.get_component_mut::<PlayerPosition>(entity);
pos_mut.x += 0.1;
```

### Creating Systems with SystemBase

```rust
use bob_engine::runtime::ecs::{SystemBase, DynamicWorld};
use std::sync::Arc;

struct MySystem {
    accumulator: f32,
}

impl SystemBase for MySystem {
    fn on_start(&mut self, world: &Arc<DynamicWorld>) {
        println!("MySystem started");
        // Access world data to initialize your system
        let count = world.entity_count();
        self.accumulator = count as f32 * 0.5;
    }

    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        // Get entities with specific components
        for entity in world.for_each::<PlayerPosition>() {
            if let Some(pos) = world.get_component::<PlayerPosition>(entity).ok()
            {
                println!("Entity {} at ({}, {})", entity, pos.x, pos.y);
            }
        }
        self.accumulator += 1.0;
    }

    fn on_destroy(&mut self, world: &Arc<DynamicWorld>) {
        println!("MySystem destroyed");
    }
}

// Register your system
let system = Box::new(MySystem { accumulator: 0.0 });
world.register_system(system);
```

### Using System Groups for Organization

```rust
use bob_engine::runtime::ecs::{SystemGroup, SystemGroupThreading};

// Create a system group on the main thread
let mut main_group = SystemGroup::new(&world, SystemGroupThreading::Main);

// Register systems in order (lower index runs first)
main_group.register_system(Box::new(SystemA), 0);
main_group.register_system(Box::new(SystemB), 1);
main_group.register_system(Box::new(SystemC), 2);

// Create a parallel group for physics calculations
let mut phys_group = SystemGroup::new(&world, SystemGroupThreading::Parallel);
phys_group.register_system(Box::new(PhysicsSystem), 0);

// Update systems every frame
main_group.update();
phys_group.update();
```

---

## System Guides

### Input System Guide

The input system provides hardware abstraction for keyboard and mouse events, tracking state changes throughout each frame.

#### Basic Usage

```rust
use bob_engine::runtime::input::{Input, KeyCode, MouseButton};

fn update_input(input: &mut Input) {
    // Clear per-frame state at the start of each frame
    input.flush();

    // Check if a key was JUST pressed this frame
    if input.get_key_pressed(KeyCode::Space) {
        println!("Space was just pressed!");
    }

    // Check if a key is currently held down
    if input.get_key_down(KeyCode::W) {
        player.movement.y = 1.0;
    }

    // Check if a key was JUST released this frame
    if input.get_key_released(KeyCode::Escape) {
        game_over_menu.visible = true;
    }
}
```

#### Mouse Input

```rust
// Track mouse position (screen coordinates)
input.receive_mouse_moved(position);

// Handle button press/release events
input.receive_mouse_button_pressed(mouse_button);
input.receive_mouse_button_released(mouse_button);

// Check for button presses/releases in your system
if input.get_mouse_button_pressed(MouseButton::Left) {
    // Click event - used once per frame
}

if input.get_mouse_button_down(MouseButton::Left) {
    // Button held down - checked every frame
}
```

#### Complete Example

```rust
use bob_engine::runtime::{App, Engine, assets::AssetStore, AppContext};
use bob_engine::runtime::input::{Input, KeyCode, MouseButton};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;

    let mut asset_store = AssetStore::new();
    include_asset!(&mut asset_store, "assets/Tux.png");

    let context = AppContext::default();
    let mut engine = Engine::new();
    let mut app = App::new(engine, context, asset_store);

    // Initialize input receiver
    let mut input = Input::new();
    app.insert_component::<InputReceiver>();
    app.add_data(input.clone());

    event_loop.run_app(&mut app)?;
    Ok(())
}
```

#### KeyCode Reference

| Constant                    | Description        |
| --------------------------- | ------------------ |
| `KeyCode::Space`            | Space bar          |
| `KeyCode::Enter`            | Enter key          |
| `KeyCode::Escape`           | Escape key         |
| `KeyCode::Tab`              | Tab key            |
| `KeyCode::Backspace`        | Backspace          |
| `KeyCode::Delete`           | Delete key         |
| `KeyCode::ArrowUp`          | Up arrow           |
| `KeyCode::ArrowDown`        | Down arrow         |
| `KeyCode::ArrowLeft`        | Left arrow         |
| `KeyCode::ArrowRight`       | Right arrow        |
| `KeyCode::W`, `A`, `S`, `D` | WASD movement keys |

#### MouseButton Reference

| Constant              | Description                        |
| --------------------- | ---------------------------------- |
| `MouseButton::Left`   | Left mouse button                  |
| `MouseButton::Middle` | Middle mouse button (scroll wheel) |
| `MouseButton::Right`  | Right mouse button                 |

---

### Sprite System Guide

The sprite system handles rendering of images and animations using an atlas-based rendering approach.

#### Basic Usage

```rust
use bob_engine::{Component, App};
use bob_engine::runtime::rendering::sprite_rendering::{Sprite, SpriteFrame};
use bob_engine::runtime::ecs::{SystemBase, DynamicWorld};

struct MoveToRight;

impl Component for MoveToRight {}

struct Player;

impl Component for Player {}

#[derive(SystemBase)]
fn update_sprites(world: &Arc<DynamicWorld>) {
    // Find entities with sprites
    for entity in world.for_each::<Sprite>() {
        let sprite = world.get_component::<Sprite>(entity);
        if let Ok(sprite) = sprite.ok() {
            // Access sprite frame data for rendering
            let mut sprite_frame = \
                world.get_component_mut::<SpriteFrame>(sprite.frame).ok();

            // Update animation frame if needed
            update_animation(world, entity);
        }
    }
}
```

#### Creating Sprite Components

```rust
use bob_engine::runtime::rendering::sprite_rendering::{Sprite, SpriteFrame};

// Create a sprite component with an image reference
#[derive(Component!)]
struct MySprite {
    frame: u32,  // Reference to SpriteFrame in world
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;

    let mut asset_store = AssetStore::new();
    include_asset!(&mut asset_store, "assets/Tux.png");

    // Register the Sprite component in the world
    let mut world = engine.get_world();
    world.register_component::<MySprite>();

    // Create an entity with a sprite
    let entity = world.create_entity();
    world.add_component_safe::<MySprite>(entity, MySprite { frame: 0 });

    // Access the frame
    let sprite_frame = world.get_component::<SpriteFrame>(entity);
    Ok(())
}
```

#### Sprite Frame Reference

```rust
// Each SpriteFrame contains:
struct SpriteFrame {
    width: f32,           // Width of the image in pixels
    height: f32,          // Height of the image in pixels
    atlas_offset_x: f32,  // X offset within the atlas texture
    atlas_offset_y: f32,  // Y offset within the atlas texture
    atlas_width: f32,     // Total width of the atlas
    atlas_height: f32,    // Total height of the atlas
}

// Access frame data for custom rendering:
let frame = world.get_component::<SpriteFrame>(entity);
let x_offset = frame.atlas_offset_x;
let y_offset = frame.atlas_offset_y;
```

#### Sprite Animation System Example

```rust
struct SpriteAnimator {
    frame_index: u32,
    frame_duration: f32,
    last_time: f64,
}

#[derive(SystemBase)]
fn animate_sprite(world: &Arc<DynamicWorld>) {
    let current_time = std::time::Instant::now();

    for entity in world.for_each::<Sprite, SpriteFrame>() {
        let sprite = world.get_component::<Sprite>(entity);
        let mut frame_data = \
            world.get_component_mut::<SpriteFrame>(sprite.frame).ok();
        if !frame_data.ok().is_some() { continue; }

        let anim_time = current_time.duration_since(frame_data.last_time);
        frame_data.frame_index = (anim_time.as_secs_f32() * 60.0) as u32;
        frame_data.last_time = current_time;
    }
}
```

---

### GUI System Guide

The GUI system provides a component-based interface for creating on-screen text, shapes, and other graphical elements.

#### GUI Components Overview

| Component      | Purpose                                                        |
| -------------- | -------------------------------------------------------------- |
| `GuiShape`     | Draw rectangles with fill color, borders, and corner radius    |
| `GuiTextBox`   | Display editable text with font rendering                      |
| `GuiTransform` | Apply position, rotation, scale transformations to UI elements |

#### GuiShape Component

```rust
use bob_engine::{Component, App};
use bob_engine::runtime::rendering::{Color, gui_rendering::GuiBorder};

// Create a simple rectangular shape
#[derive(Component!)]
struct MyRectangle {
    transform: u32, // Transform component for positioning
    shape_component: u32, // GuiShape component index
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;

    let mut asset_store = AssetStore::new();
    include_asset!(&mut asset_store, "assets/Tux.png");

    let context = AppContext::default();
    let mut engine = Engine::new();
    let mut app = App::new(engine, context, asset_store);

    // Register GUI components
    world.register_component::<GuiShape>();
    world.register_component::<GuiTransform>();

    // Create entities with GUI elements
    // Each entity needs both GuiTransform (position) and GuiShape (visual)
    let rectangle_entity = world.create_entity();
    let transform_data = world.get_component_mut::<GuiTransform>(rectangle_entity);
    let shape_data = world.get_component_mut::<GuiShape>(rectangle_entity);

    // Set rectangle appearance
    *shape_data = GuiShape::new(
        visible: true,
        fill_color: Color::Srgba(255u8, 0u8, 0u8, 255u8), // Red color
        border: GuiBorder::Bordered(Color::Srgba(255u8, 255u8, 0u8, 255u8), 2.0, 5.0),
        uv_offset: [0.0, 0.0],
        uv_scale: [1.0, 1.0]
    );

    // Update transform to position rectangle
    let transform_data = world.get_component_mut::<GuiTransform>(rectangle_entity);
    transform_data.position.x = 100.0;
    transform_data.position.y = 50.0;

    Ok(())
}
```

#### GuiTextBox Component

```rust
use bob_engine::{Component, App};
use bob_engine::runtime::rendering::gui_rendering::font_binder;

// Create a text box with custom content
#[derive(Component!)]
struct MyTextBox {
    transform: u32,
    shape_component: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Register components
    world.register_component::<GuiShape>();
    world.register_component::<GuiTextBox>();

    let text_box_entity = world.create_entity();

    // Set up the shape component (background)
    let shape_data = world.get_component_mut::<GuiShape>(text_box_entity);
    *shape_data = GuiShape::new(
        true,
        Color::Srgba(255u8, 255u8, 255u8, 200u8), // Semi-transparent white
        GuiBorder::Borderless,
        [0.0, 0.0],
        [1.0, 1.0]
    );

    // Set up the text box with "Hello World" content
    let text_data = world.get_component_mut::<GuiTextBox>(text_box_entity);
    *text_data = GuiTextBox::new(
        "Hello World".to_string(),
        font_binder,
        24.0, // Font size in pixels
        1.5, // Letter spacing multiplier
        Color::Srgba(0u8, 0u8, 0u8, 255u8), // Black text
    );

    Ok(())
}
```

#### GUI Border Types

```rust
// Three border options:
enum GuiBorder {
    #[default]
    Borderless,              // No border
    Bordered(Color, f32, f32),  // Color, Width, Corner Radius
}
```

#### Complete GUI Example

```rust
use bob_engine::runtime::{App, Engine, assets::AssetStore, AppContext};
use bob_engine::runtime::rendering::{Color, gui_rendering::GuiBorder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;

    let mut asset_store = AssetStore::new();
    include_asset!(&mut asset_store, "assets/Tux.png");

    let context = AppContext::default();
    let mut engine = Engine::new();
    let mut app = App::new(engine, context, asset_store);

    // Register all GUI components
    world.register_component::<GuiShape>();
    world.register_component::<GuiTransform>();
    world.register_component::<GuiTextBox>();

    // Create a HUD element with text and border rectangle
    let hud_entity = world.create_entity();

    // Rectangle shape for HUD background
    let hud_shape_data = world.get_component_mut::<GuiShape>(hud_entity);
    *hud_shape_data = GuiShape::new(
        true,
        Color::Srgba(255u8, 0u8, 0u8, 150u8), // Semi-transparent red
        GuiBorder::Bordered(Color::Srgba(255u8, 255u8, 0u8, 255u8), 3.0, 10.0),
        [0.0, 0.0],
        [1.0, 1.0]
    );

    // Text box with health info
    let hud_text_data = world.get_component_mut::<GuiTextBox>(hud_entity);
    *hud_text_data = GuiTextBox::new(
        "Health: 100".to_string(),
        font_binder,
        24.0,
        1.5,
        Color::Srgba(255u8, 255u8, 255u8, 255u8),
    );

    Ok(())
}
```

---

## Development & Building

### Build Commands

```bash
cargo build                 # Debug build
cargo build --release       # Optimized release build
cargo test                  # Run tests
```

### Project Structure

- `assets/` - Game assets (images, scenes, textures)
- `macros/` - Custom derive macros (Component, Serializable)
- `scripts/` - RHAI script files
- `src/` - Core engine source code
