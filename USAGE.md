# Bob Engine — Usage Guide

A practical reference for using the `bob_engine` crate: setting up an application,
working with the ECS, registering systems, and driving the built-in input, asset,
rendering, GUI, physics, scene, and scripting systems.

All examples below use the public API as it exists in `src/`. Where a feature is
still a stub or partially implemented, that is called out explicitly.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Project Layout](#project-layout)
3. [Application Lifecycle](#application-lifecycle)
4. [Entity Component System (ECS)](#entity-component-system-ecs)
   - [Components](#components)
   - [Entities](#entities)
   - [Component Operations](#component-operations)
   - [Queries](#queries)
   - [Query Filters](#query-filters)
   - [Singletons](#singletons)
5. [Systems](#systems)
   - [The `SystemBase` Trait](#the-systembase-trait)
   - [System Groups](#system-groups)
   - [Registering Your Own Systems](#registering-your-own-systems)
   - [Built-in System Groups](#built-in-system-groups)
6. [Input](#input)
7. [Assets](#assets)
8. [Rendering](#rendering)
   - [Camera](#camera)
   - [Color](#color)
   - [Sprites](#sprites)
   - [Sprite Sheets & Animation](#sprite-sheets--animation)
   - [Tilemaps](#tilemaps)
   - [Batches & Pipelines](#batches--pipelines)
9. [GUI](#gui)
10. [Physics](#physics)
11. [Scenes & Serialization](#scenes--serialization)
12. [Scripting (RHAI)](#scripting-rhai)
13. [Math Utilities](#math-utilities)
14. [Engine Constants](#engine-constants)
15. [Building & Running](#building--running)
16. [Gotchas & Caveats](#gotchas--caveats)

---

## Quick Start

The minimal application (mirrors `src/main.rs`):

```rust
use bob_engine::runtime::Engine;
use bob_engine::runtime::assets::AssetStore;
use bob_engine::app::App;
use bob_engine::AppContext;
use bob_engine::EventLoop; // re-exported winit EventLoop

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;

    // 1. Build the asset store and embed assets at compile time
    let mut asset_store = AssetStore::new();
    include_asset!(&mut asset_store, "Tux.png");

    // 2. Create the engine and app
    let mut engine = Engine::new();
    let mut app = App::new(engine, AppContext::default(), asset_store);

    // 3. Run the winit event loop — the engine runs on each redraw
    event_loop.run_app(&mut app)?;
    Ok(())
}
```

What happens under the hood:

1. `EventLoop::new()` creates the winit event loop.
2. `AssetStore::new()` + `include_asset!` embeds asset bytes into the binary
   (`include_bytes!` under the hood, resolved relative to `assets/`).
3. `Engine::new()` creates the renderer, input state, and the main
   `DynamicWorld` (named `"main"`).
4. `App::new(...)` stores the engine + context + assets.
5. When the event loop resumes, `App` creates the window, initializes the
   renderer (WGPU), and calls `Engine::init`, which boots all built-in system
   groups.
6. Every `WindowEvent::RedrawRequested` calls `Engine::run()` (update + render),
   and `about_to_wait` requests the next redraw, so the loop runs continuously.

---

## Project Layout

```
bob-engine/
├── assets/                  # Game assets (embedded at compile time)
│   ├── cfg/asset_store.json # List of assets loaded at engine init
│   ├── default_ui/          # UI textures
│   ├── exp/                 # Sprite sheets (ships, projectiles)
│   ├── scenes/              # Saved .bscene files
│   └── *.png, *.jpg         # Textures, tilesets
├── macros/
│   └── component/           # `StableID` derive + `component` attribute macro
├── scripts/
│   └── hello_world.rhai     # Example RHAI script
├── stable_cmpt_id/          # (legacy) standalone StableID macro crate
└── src/
    ├── lib.rs               # Library entry, `Component!`/`Serializable!` aliases
    ├── main.rs              # Demo binary
    ├── app.rs               # winit ApplicationHandler (window + event pump)
    ├── app_context.rs       # Window size / fullscreen config
    ├── constants.rs         # World/group names, FIXED_DT, etc.
    ├── alias_macros.rs      # (legacy, unused) macro aliases
    ├── test.rs              # Demo systems (physics + GUI showcase)
    └── runtime/
        ├── engine.rs        # Engine orchestration + frame loop
        ├── ecs/             # DynamicWorld, entities, queries, systems
        ├── input/           # Keyboard/mouse state
        ├── assets/          # AssetStore, embedded assets, handles
        ├── rendering/       # WGPU renderer, camera, sprites, tilemaps, GUI
        ├── gui/             # GUI components (shape, transform, text box)
        ├── phys/            # 2D physics: rigid bodies, collisions, joints
        ├── scene/           # World serialization (.bscene)
        ├── scripting/       # RHAI scripting engine wrapper
        └── math/            # Float2/Int2 vectors, angles, RNG
```

---

## Application Lifecycle

### `AppContext`

Configures the window. Defined in `src/app_context.rs`:

```rust
use bob_engine::AppContext;

// 800x600 windowed (the default)
let context = AppContext::default();

// Custom size, optionally fullscreen
let context = AppContext::new((1280, 720), false);
```

The context is also available at runtime through the global
`bob_engine::app::GLOBAL_APP_CONTEXT` (`OnceLock<AppContext>`), e.g. to read the
window size for GUI anchoring:

```rust
let size = bob_engine::app::GLOBAL_APP_CONTEXT.get().unwrap().window_size;
```

### `App`

`App` (in `src/app.rs`) implements winit's `ApplicationHandler`:

- **`resumed`** — creates the window from `AppContext`, then calls
  `init_engine()`, which initializes the renderer and the engine.
- **`window_event`** — forwards mouse/keyboard events to `engine.input`,
  handles resize (renderer resize + camera update) and close.
- **`RedrawRequested`** — calls `engine.run()` and requests the next redraw.
- **`about_to_wait`** — requests a redraw so the loop never idles.

You normally don't call these yourself; `event_loop.run_app(&mut app)` drives
them.

### The Frame Loop

`Engine::run()` (called once per redraw) does:

1. `frame_count += 1`
2. `update()` — runs all system groups, then the built-in camera control loop,
   then flushes per-frame input state
3. `render()` — uploads instance buffers and draws all batches
4. Prints frame time; sleeps to pace the loop at `FIXED_DT` (1/60 s)

---

## Entity Component System (ECS)

The ECS lives in `src/runtime/ecs/`. The central type is `DynamicWorld`,
reachable from the engine as:

```rust
use bob_engine::constants::MAIN_WORLD;

let world: &Arc<DynamicWorld> = engine.entities.get_world(MAIN_WORLD)?;
```

### Components

Components are plain `Copy` structs tagged with the `Component!` derive macro
(alias defined in `src/lib.rs`):

```rust
#[derive(Component!)]
struct Velocity {
    pub vx: f32,
    pub vy: f32,
}
```

`#[derive(Component!)]` expands to:

```rust
#[derive(Clone, Default, component::StableID, Copy,
         serde::Serialize, serde::Deserialize, rhai::CustomType)]
```

- `StableID` generates a `StableTypeID` impl whose `ID: u64` is an FNV-1a hash
  of the type's full module path — a stable, serialization-friendly component
  identifier.
- Because `Copy` is derived, **components can only contain `Copy` fields**
  (primitives, `Float2`, arrays, other `Copy` structs).
- `Default` is required, so every component must have a sensible zero value.

There is also a `#[derive(Serializable!)]` alias for plain
`serde::Serialize + serde::Deserialize` types that are *not* components
(e.g. `Shape`, `PhysCxn`).

The core built-in component is `Transform`
(`bob_engine::runtime::ecs::core_components::Transform`):

```rust
pub struct Transform {
    pub position: Float2,
    pub rotation: f32, // radians
}

impl Transform {
    pub fn new(position: Float2, rotation: f32) -> Self;
}
```

### Entities

Entities are cheap `Entity(usize)` handles (0-indexed). Creating an entity does
not allocate components — components are added separately.

```rust
use bob_engine::runtime::ecs::Entity;

let e: Entity = world.create_entity();
world.destroy_entity(e);      // marks it dead (ID is never reused)
world.is_alive(e);            // bool
world.entity_count();         // number of alive entities
```

### Component Operations

All component methods are generic over the component type `T` (which must
satisfy the `Component!` bounds).

| Method | Description |
| --- | --- |
| `world.register_component::<T>()` | Registers storage for `T` in the world. Idempotent. |
| `world.add_component(e, value)` | Inserts a component. **Panics if `T` was never registered.** |
| `world.add_component_safe(e, value)` | Registers `T` if needed, then inserts. Use this by default. |
| `world.remove_component::<T>(e)` | Removes the component from the entity. |
| `world.get_clone::<T>(e) -> Option<T>` | Copies the component out. |
| `world.has_component::<T>(entity_id: usize) -> bool` | Presence check (takes a raw `usize`). |
| `world.get_component::<T, R>(e, \|c: &T\| -> R) -> Option<R>` | Read a component through a closure. |
| `world.get_component_mut::<T, R>(e, \|c: &mut T\| -> R) -> Option<R>` | Mutate a component through a closure. |

The `get_component` / `get_component_mut` methods are **closure-based** — the
closure runs while the storage lock is held and returns whatever you need:

```rust
use bob_engine::runtime::ecs::core_components::Transform;

// Read
let pos = world.get_component::<Transform, _>(entity, |t| t.position);

// Mutate
world.get_component_mut::<Transform, _>(entity, |t| {
    t.position.x += 0.1;
});
```

### Queries

Queries iterate all *alive* entities that have the requested components.
Immutable queries:

```rust
// 1 component
world.for_each::<Transform>(|entity, transform| {
    // transform: &Transform
});

// 2..10 components (for_each2 .. for_each10)
world.for_each2::<Transform, Velocity>(|entity, t, v| {
    // t: &Transform, v: &Velocity
});
```

Mutable queries (all listed components must be distinct types):

```rust
// Single mutable
world.for_each_mut::<Velocity>(|entity, v| {
    v.vx *= 0.99;
});

// All mutable: for_each2_mut_both .. for_each10_mut_all
world.for_each2_mut_both::<Transform, Velocity>(|entity, t, v| {
    t.position.x += v.vx;
    t.position.y += v.vy;
});
```

Hybrid mutable/immutable queries:

```rust
// A mutable, B immutable
world.for_each2_mut::<Transform, Velocity>(|entity, t, v| {
    t.position += *v; // v: &Velocity
});

// A and B mutable, C immutable
world.for_each3_mut::<Transform, Velocity, Sprite>(|entity, t, v, s| {
    // t: &mut Transform, v: &mut Velocity, s: &Sprite
});
```

Optional second component:

```rust
world.for_each_optional::<Transform, Velocity>(|entity, t, maybe_v| {
    // maybe_v: Option<&Velocity>
});
```

> Note: queries that reference an unregistered component type simply return
> without invoking the closure.

### Query Filters

`for_each_filtered` / `for_each_mut_filtered` take a `QueryFilter`
(`bob_engine::runtime::ecs::query`) that adds per-entity conditions on top of
the tuple query:

```rust
use bob_engine::runtime::ecs::query::{With, Without, And, NoFilter};

// Entities with Transform that also have Velocity
world.for_each_filtered::<Transform, _>(
    With::<Velocity>::new(),
    |entity, t| { /* ... */ },
);

// Entities with Transform but NOT Velocity
world.for_each_filtered::<Transform, _>(
    Without::<Velocity>::new(),
    |entity, t| { /* ... */ },
);

// Compose filters
world.for_each_filtered::<Transform, _>(
    And(With::<Velocity>::new(), Without::<Sprite>::new()),
    |entity, t| { /* ... */ },
);
```

Available filters: `With<T>`, `Without<T>`, `And(a, b)`, `Or(a, b)`, `NoFilter`.

### Singletons

Singletons are world-wide, non-entity components (one instance per type):

```rust
#[derive(Component!)]
struct GameState {
    pub score: u32,
}

world.create_singleton(GameState { score: 0 });
world.set_singleton(GameState { score: 10 }); // replaces
let state: GameState = world.get_singleton::<GameState>(); // by value
world.destroy_singleton::<GameState>();
```

---

## Systems

### The `SystemBase` Trait

All game logic systems implement `SystemBase`
(`bob_engine::runtime::ecs::SystemBase`):

```rust
use bob_engine::runtime::ecs::{DynamicWorld, SystemBase};
use std::sync::Arc;

struct MySystem {
    accumulator: f32,
}

impl SystemBase for MySystem {
    fn on_start(&mut self, world: &Arc<DynamicWorld>) {
        // Runs once when the system group starts. Good place to spawn
        // initial entities.
        let e = world.create_entity();
        world.add_component_safe(e, Transform::new(Float2::new(0.0, 0.0), 0.0));
    }

    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        // Runs every frame.
        world.for_each_mut::<Velocity>(|_e, v| {
            v.vx *= 0.99;
        });
        self.accumulator += 1.0;
    }

    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {
        // Runs when the system is destroyed.
    }
}
```

Systems may hold shared engine resources as fields — clone the `Arc`s from the
engine when constructing the system (this is the pattern used by the built-in
systems and by `TestSystem` in `src/test/test_system.rs`):

```rust
struct MySystem {
    input: Arc<RwLock<Input>>,
    asset_store: Arc<OnceLock<AssetStore>>,
}

impl MySystem {
    fn new(input: Arc<RwLock<Input>>, asset_store: Arc<OnceLock<AssetStore>>) -> Self {
        Self { input, asset_store }
    }
}
```

### System Groups

A `SystemGroup` owns an ordered list of systems that share one world:

```rust
use bob_engine::runtime::ecs::SystemGroup;
use bob_engine::runtime::ecs::system_group::SystemGroupThreading;

let mut group = SystemGroup::new(world, SystemGroupThreading::Main);

// `order` controls execution order: lower runs first.
group.register_system(Box::new(SystemA), 0);
group.register_system(Box::new(SystemB), 1);
group.register_system(Box::new(SystemC), 2);

group.start_systems(); // calls on_start on every system
group.update();        // calls on_update on every system, in order
group.destroy_system(1); // calls on_destroy + removes
group.destroy_systems(); // tears everything down
```

- Threading modes:
  - `SystemGroupThreading::Main` — systems run on the main thread.
  - `SystemGroupThreading::Parallel` — systems run on a freshly spawned worker
    thread that is joined at the end of `update()`. (This serializes the group
    on a worker thread; it does not overlap with the main thread.)
- Systems run in ascending `order` value.
- Up to 256 systems per group (`MAX_SYSTEMS_PER_GROUP`).
- The engine holds at most ~16 named system groups.

### Registering Your Own Systems

The engine starts and updates every group registered in `engine.entities`
(`bob_engine::runtime::ecs::entities::Entities`). Add your group **before**
`App::new` (or before `Engine::init` runs), and it will be started and updated
every frame automatically:

```rust
use bob_engine::constants::MAIN_WORLD;
use bob_engine::runtime::Engine;
use bob_engine::runtime::ecs::SystemGroup;
use bob_engine::runtime::ecs::system_group::SystemGroupThreading;

fn build_engine() -> Engine {
    let mut engine = Engine::new();

    let world = engine.entities.get_world(MAIN_WORLD).unwrap();
    let mut group = SystemGroup::new(world, SystemGroupThreading::Main);
    group.register_system(Box::new(MoveSystem), 0);
    group.register_system(Box::new(PhysicsDebugSystem), 1);

    engine.entities.add_system_group("my_group", group);
    engine
}
```

You can also fetch and mutate an existing group by name:

```rust
let group = engine.entities.get_system_group_mut("my_group")?;
group.register_system(Box::new(AnotherSystem), 2);
```

You can spawn entities directly on the world before the app starts:

```rust
let world = engine.entities.get_world(MAIN_WORLD).unwrap();
let e = world.create_entity();
world.add_component_safe(e, Transform::new(Float2::new(1.0, 2.0), 0.0));
```

### Built-in System Groups

`Engine::init` registers these groups (names from `src/constants.rs`):

| Group name | Threading | Systems (in order) |
| --- | --- | --- |
| `"render_group"` | Parallel | `BatchAllocator` → `SpriteRenderSystem` → `GuiTextRenderSystem` → `GuiShapeRenderSystem` |
| `"physics_connection_group"` | Parallel | `ConnectorSystem` (joints) |
| `"physics_group"` | Parallel | `PhysicsSystem` (integration + collisions) |

Because groups are stored in a `HashMap`, **the relative order between groups
is nondeterministic** — only the order *within* a group is guaranteed.

---

## Input

Input state lives in `engine.input: Arc<RwLock<Input>>`
(`bob_engine::runtime::input::Input`). The `App` forwards winit events into it
automatically.

```rust
use bob_engine::runtime::input::KeyCode;

let input = engine.input.read().unwrap();

// True only on the frame the key was first pressed
if input.get_key_pressed(KeyCode::Space) { /* ... */ }

// True every frame the key is held down
if input.get_key_down(KeyCode::KeyW) { /* ... */ }

// True only on the frame the key was released
if input.get_key_released(KeyCode::Escape) { /* ... */ }
```

Mouse:

```rust
use bob_engine::runtime::input::MouseButton;

let input = engine.input.read().unwrap();

if input.get_mouse_button_pressed(MouseButton::Left) { /* clicked this frame */ }
if input.get_mouse_button_down(MouseButton::Left) { /* held */ }
if input.get_mouse_button_released(MouseButton::Right) { /* released */ }

let (x, y): (u32, u32) = input.mouse_position;          // screen pixels
let world_pos: Float2 = input.mouse_world_position;     // updated by the engine each frame
```

Notes:

- `Input::flush()` clears the "just pressed/released" sets. The engine calls it
  at the end of `Engine::update()`, so you don't need to call it yourself.
- `KeyCode` is a physical-key enum: `KeyA`–`KeyZ`, `Digit0`–`Digit9`, `Space`,
  `Enter`, `Escape`, `Tab`, `Backspace`, `Delete`, `ArrowUp/Down/Left/Right`,
  `F1`–`F35`, numpad keys, etc. (see `src/runtime/input/keycodes.rs`).
- `MouseButton` has `Left`, `Right`, `Middle`, `None`.
- **Built-in camera controls:** `Engine::player_loop()` (run every frame) moves
  the camera with `A`/`D`/`W`/`S` and zooms with `1` (in) / `2` (out). It also
  converts the mouse position to world space into
  `input.mouse_world_position`.

---

## Assets

Assets are embedded into the binary at compile time and managed by
`AssetStore` (`bob_engine::runtime::assets::AssetStore`).

### Including Assets

```rust
use bob_engine::runtime::assets::AssetStore;

let mut asset_store = AssetStore::new();
include_asset!(&mut asset_store, "Tux.png");
include_asset!(&mut asset_store, "exp/ship_parts_s.png");
```

`include_asset!` embeds `assets/<path>` via `include_bytes!` and registers the
asset under its path string. Paths are relative to the `assets/` directory.

At engine init, `AssetStore::init()` additionally loads every asset listed in
`assets/cfg/asset_store.json` from the embedded `assets/` folder
(`rust-embed`). Keep that JSON in sync with the assets you want available at
runtime.

### Looking Up Assets

```rust
// By path -> handle
let handle: Option<AssetHandle> = asset_store.get_asset_handle_by_path("Tux.png");

// By path -> asset (hash + path + raw bytes)
let asset: Option<&Asset> = asset_store.get_asset_by_path("Tux.png");

// By hash
let hash = asset_store.get_asset_hash("Tux.png");
let asset = asset_store.get_asset_by_hash(hash);

// By handle
let asset = asset_store.get_asset_by_handle(handle);
```

- `AssetHandle { idx: usize, file_type: Option<AssetType> }` — the index used
  by the renderer and by `BatchHandle`.
- `AssetType`: `Png`, `Jpeg`, `Json`, `BScene` (inferred from the file
  extension).
- You can also read any embedded file directly at runtime:
  `AssetEmbedded::get("grass.png")` returns the raw bytes.

The engine exposes the initialized store as
`engine.asset_store: Arc<OnceLock<AssetStore>>` — clone the `Arc` into your
systems to look up assets.

---

## Rendering

The renderer (`bob_engine::runtime::rendering::Renderer`) is a WGPU
immediate-mode, instanced-quad renderer. You rarely touch it directly; the
built-in render systems turn ECS components into GPU instance buffers.

The renderer is reachable as `engine.renderer: Arc<RwLock<Renderer>>`.

### Camera

`Renderer.camera` is a 2D orthographic camera (`bob_engine::runtime::rendering::camera::Camera`):

```rust
let mut renderer = engine.renderer.write().unwrap();

renderer.camera.move_by(0.1, 0.0);          // pan
renderer.camera.zoom_by(0.99);              // zoom out (zoom clamped to 0.001..1.0)
renderer.camera.zoom_towards(1.01, x, y);   // zoom keeping world point (x, y) fixed

// Screen pixel -> world coordinates
let [wx, wy] = renderer.camera.screen_to_world(400, 300);
```

- `zoom == 1.0` shows 2.0 world units vertically (aspect-scaled horizontally).
- The camera matrix is re-uploaded every frame by `Renderer::render()`.
- Window resizes update the camera automatically.

### Color

`bob_engine::runtime::rendering::Color` stores RGBA in 0.0–1.0:

```rust
use bob_engine::runtime::rendering::Color;

Color::new(1.0, 0.0, 0.0, 1.0)      // normalized
Color::from_rgba(255, 0, 0, 255)    // 0-255
Color::from_rgb(255, 0, 0)          // opaque
Color::from_hex("#FF0000")?         // "#RRGGBB" or "#RRGGBBAA"
Color::from_u32(0xFF0000FF)         // 0xRRGGBBAA
Color::red() / .green() / .blue() / .white() / .black() / .yellow() / .cyan() / .magenta() / .transparent()
color.with_alpha(0.5)
```

### Sprites

To draw a sprite, an entity needs three components:

1. `Transform` — world position + rotation
2. `Sprite` — visibility + UV rect into the texture
3. `BatchHandle` — which texture/pipeline to draw with

```rust
use bob_engine::runtime::ecs::core_components::Transform;
use bob_engine::runtime::rendering::sprite_rendering::components::Sprite;
use bob_engine::runtime::rendering::{BatchHandle, PipelineKey};
use bob_engine::runtime::math::Float2;

let asset_handle = asset_store.get_asset_handle_by_path("Tux.png").unwrap();

let e = world.create_entity();
world.add_component_safe(e, Transform::new(Float2::new(0.0, 0.0), 0.0));
world.add_component_safe(e, Sprite::new(true, [0.0, 0.0], [1.0, 1.0]));
// uv_offset [0,0] + uv_scale [1,1] = whole texture
world.add_component_safe(e, BatchHandle::new(asset_handle, PipelineKey::Sprite));
```

How it works:

- `BatchAllocator` (first system in `render_group`) scans for
  `BatchHandle`s with an unassigned slot and allocates a GPU batch
  (texture + instance buffer) per (asset, pipeline) pair, up to 4095
  instances per batch.
- `SpriteRenderSystem` then copies each entity's
  `Transform + Sprite + BatchHandle` into a `SpriteInstance`
  (position, size, uv_offset, uv_scale, rotation) in the batch.
- `Renderer::render()` uploads all instance buffers and draws every batch.

To hide a sprite, set `Sprite.visible = false` (or remove the component).

### Sprite Sheets & Animation

For sprite sheets, use `SpriteSheetBinder`
(`bob_engine::runtime::rendering::sprite_rendering::sprite_sheet_binder`):

```rust
use bob_engine::runtime::rendering::sprite_rendering::sprite_sheet_binder::SpriteSheetBinder;
use bob_engine::runtime::rendering::sprite_rendering::SpriteAnimation;

// Sheet is 6 columns x 1 row
let binder = SpriteSheetBinder::new(6, 1, None);

// Create a sprite showing cell (2, 0) with correct UV scale
let sprite = binder.new_sprite_at_frame([2.0, 0.0]);
world.add_component_safe(e, sprite);

// Or update an existing sprite's frame
binder.set_sprite_frame([3.0, 0.0], &mut sprite);
```

For frame animations, build a `SpriteAnimation` from a list of sheet cells:

```rust
let animation = SpriteAnimation::new(vec![[0.0, 0.0], [1.0, 0.0], [2.0, 0.0]]);
let binder = SpriteSheetBinder::new(6, 1, Some(animation));

binder.start_animation(&mut sprite, &mut sprite_frame); // jump to frame 0
binder.run_animation(&mut sprite, &mut sprite_frame);   // advance one frame (wraps)
```

`SpriteFrame { frame: u16 }` (a component) tracks the current frame index.
The binder applies a small border inset (`BORDER_EDGE_WIDTH = 0.05`) to avoid
texture bleeding between cells.

### Tilemaps

Tilemaps are rendered by `TilemapRenderer` instances held in
`renderer.tilemaps`. Create one from a tileset image and a grid of tile IDs:

```rust
let tilemap: [u8; 64 * 64] = /* your grid, row-major, values < 256 */;
let grass_bytes = AssetEmbedded::get("grass.png").unwrap().data;

let mut renderer = engine.renderer.write().unwrap();
let idx = renderer.create_tilemap(&grass_bytes, &tilemap, 64, 64, 32);
// tile_size = 32 world units per tile

renderer.tilemaps[idx].move_by(0.0, -0.5);        // offset the map in world space
renderer.tilemaps[idx].flush_position(renderer.queue()); // push offset to GPU
```

- `create_tilemap(tex_bytes, tile_data, width, height, tile_size) -> usize`
  returns the index into `renderer.tilemaps`.
- `TilemapRenderer::move_to(x, y)` / `move_by(dx, dy)` set the world offset;
  call `flush_position(queue)` afterwards.
- `renderer.update_tilemap(idx, ...)` re-uploads changed tile data.
- Max 255 distinct tiles per texture; the tile buffer supports 256×256 tiles.
- Note: the built-in tilemap draw call is currently commented out in
  `record_render_pass` (see "Gotchas").

### Batches & Pipelines

Low-level batch API (usually handled by `BatchAllocator` for you):

```rust
let idx = renderer.create_batch(asset_handle, &instances, PipelineKey::Sprite);
renderer.set_batch_texture(idx, new_tex_bytes); // swap a batch's texture
```

`PipelineKey` selects the shader/blend mode:

| Key | Purpose |
| --- | --- |
| `PipelineKey::Sprite` | Standard alpha-blended sprite shader |
| `PipelineKey::Additive` | Additive blending (particles, glows) |
| `PipelineKey::GuiShape` | Screen-space UI shapes |
| `PipelineKey::GuiText` | Screen-space UI text |

Instance structs (`SpriteInstance`, `GuiShapeInstance`, `GuiCharInstance`)
must implement `bytemuck::Pod + VertexLayout` to be used with
`create_batch`.

---

## GUI

The GUI is screen-space (not affected by the world camera) and uses the same
ECS + batch pipeline as sprites. Components live in
`bob_engine::runtime::gui::components`.

### `GuiShape`

A filled rectangle with optional border and corner radius:

```rust
use bob_engine::runtime::gui::components::{GuiShape, GuiBorder};
use bob_engine::runtime::rendering::Color;

let shape = GuiShape::new(
    true,                                        // visible
    Color::from_hex("#00909E").unwrap(),         // fill color
    GuiBorder::Bordered(                         // border: color, width, corner radius
        Color::from_hex("#006884").unwrap(),
        20.0,
        20.0,
    ),
    [0.0, 0.0],                                 // uv_offset
    [1.0, 1.0],                                 // uv_scale
);
```

`GuiBorder` variants: `Borderless` (default) and `Bordered(Color, width, radius)`.

### `GuiTransform`

Position + size in screen pixels:

```rust
use bob_engine::runtime::gui::components::{GuiTransform, GuiAnchor};
use bob_engine::runtime::math::Float2;

// Absolute screen position
let t = GuiTransform::from_position(Float2::new(100.0, 50.0), Float2::new(200.0, 80.0));

// Or anchor to the window (uses GLOBAL_APP_CONTEXT window size)
let t = GuiTransform::at_anchor(GuiAnchor::BottomRight, Float2::new(200.0, 80.0));
```

`GuiAnchor` has the 9 standard anchors: `TopLeft`, `TopMiddle`, `TopRight`,
`MiddleLeft`, `Middle`, `MiddleRight`, `BottomLeft`, `BottomMiddle`,
`BottomRight`.

### `GuiTextBox`

Text rendered from a bitmap font sheet:

```rust
use bob_engine::runtime::gui::components::GuiTextBox;
use bob_engine::runtime::rendering::gui_rendering::font_binder::FontSheetBinding;
use bob_engine::runtime::rendering::Color;

let text_box = GuiTextBox::new(
    "Health: 100".to_string(),
    FontSheetBinding::Standard16Collum, // or NonStandardTestPallate (default)
    24.0,        // font size in pixels
    1.5,         // letter spacing multiplier
    Color::white(),
);
```

- Max 1024 characters per text box.
- `FontSheetBinding::Standard16Collum` maps ASCII to a 16×6 glyph grid;
  `NonStandardTestPallate` maps a limited character set to a 15×6 grid.

### Putting It Together

A GUI entity needs `GuiTransform` + a visual component (`GuiShape` or
`GuiTextBox`) + a `BatchHandle` with the matching pipeline:

```rust
let e = world.create_entity();
world.add_component_safe(e, GuiTransform::at_anchor(GuiAnchor::TopLeft, Float2::new(200.0, 60.0)));
world.add_component_safe(e, GuiShape::new(true, Color::from_hex("#00909E").unwrap(), GuiBorder::Borderless, [0.0, 0.0], [1.0, 1.0]));
world.add_component_safe(e, BatchHandle::new(ui_asset_handle, PipelineKey::GuiShape));
```

`GuiShapeRenderSystem` and `GuiTextRenderSystem` (in `render_group`) convert
these into `GuiShapeInstance` / `GuiCharInstance` batches each frame. Changes
are picked up via the `dirty` flag — set it to `true` after mutating a
`GuiShape`/`GuiTextBox` (constructors set it for you).

> `GuiEventSystem` (hit-testing / interaction) is currently an empty stub.

---

## Physics

The 2D physics engine lives in `src/runtime/phys/`. It runs in two built-in
groups: `ConnectorSystem` (joints) in `"physics_connection_group"`, then
`PhysicsSystem` (integration + collisions) in `"physics_group"`.

### Rigid Bodies

Add a `RigidBody` component to an entity (along with `Transform`):

```rust
use bob_engine::runtime::phys::{RigidBody, Shape};
use bob_engine::runtime::math::Float2;

// Dynamic circle
let body = RigidBody::new(
    Shape::Circle { radius: 0.5 },
    1.0,                       // mass (0 = static)
    Float2::new(0.0, 10.0),    // position
    0.0,                       // rotation (radians)
);
world.add_component_safe(e, body);

// Static rectangle (infinite mass)
let wall = RigidBody::new_static(
    Shape::Rect { half_w: 5.0, half_h: 0.5 },
    Float2::new(0.0, 0.0),
    0.0,
);
```

`RigidBody` fields: `position`, `rotation`, `velocity`, `angular_velocity`,
`inv_mass`, `inv_inertia`, `restitution` (default 0.4), `friction` (default
0.3), `shape`, `force`, `torque`, `is_static`.

### Forces & Impulses

Apply these in your systems before the physics step; `force`/`torque` are
reset each frame after integration:

```rust
world.get_component_mut::<RigidBody, _>(e, |b| {
    b.apply_force(Float2::new(0.0, -9.8));       // gravity-like force
    b.apply_torque(0.5);
    b.apply_force_at(f, world_point);            // force + resulting torque
    b.apply_impulse(impulse);                    // instantaneous
    b.apply_impulse_pt(impulse, r);              // impulse at offset r
    b.apply_angular_impulse(j);
    b.apply_angular_impulse_at(j, world_point);
});
```

### Collision Pipeline

Each frame `PhysicsSystem::step` (fixed 1/60 s) does:

1. **Integrate** — semi-implicit Euler on all `RigidBody`s (gravity from
   `PhysicsConfig`, default `(0, 0)`).
2. **Broad phase** — AABB overlap test.
3. **Narrow phase** — circle–circle, rect–rect (SAT), circle–rect.
4. **Impulse resolution** — iterative (5 iterations by default) with
   restitution + Coulomb friction.
5. **Positional correction** — Baumgarte-style (40% correction, 0.01 slop).
6. **Sync** — copies `RigidBody` position/rotation back into `Transform`.

Tune via `PhysicsSystem.config: PhysicsConfig { gravity, iterations, slop,
correction_percent }`.

### Joints

`PhysJoint` connects up to 4 other bodies with spring-damper constraints,
solved by `ConnectorSystem`:

```rust
use bob_engine::runtime::phys::connector::{PhysJoint, PhysCxn};

// Connect this body to `other_entity` at anchor (1.0, 0.0)
let joint = PhysJoint::new(
    10.0,  // linear connection strength
    10.0,  // angular connection strength
    [
        Some(PhysCxn::new(other_entity, Float2::new(1.0, 0.0))),
        None, None, None,
    ],
);
world.add_component_safe(e, joint);
```

- `PhysCxn { cxn: Entity, anc: Float2 }` — target entity + anchor point.
- `is_intact` — set to `false` to disable a joint; the solver can also mark
  joints broken under load.

---

## Scenes & Serialization

Worlds can be serialized to JSON scene files (`.bscene`):

```rust
use bob_engine::runtime::scene::world_serializer::create_scene_file_from_world;

// Writes assets/scenes/saved/<name>.bscene
create_scene_file_from_world("level1".to_string(), &world);
```

The format is `SerializedWorld { entity_count, component_defs }`, where
`component_defs` maps stable component IDs → entity ID → JSON component value.
All components are serialized through serde, which is why `Component!` derives
`Serialize`/`Deserialize`.

> **Status:** saving works; loading is not implemented yet
> (`world_deserializer::load_to_serialized_world` and
> `load_serialized_world_to_dynamic_world` are `todo!`/stubs), and
> `SceneManager` is an empty stub.

---

## Scripting (RHAI)

The engine embeds RHAI (`src/runtime/scripting/scripting_engine.rs`):

```rust
use bob_engine::runtime::scripting::scripting_engine::ScriptingEngine;

let mut scripting = ScriptingEngine::new();

// Compile a script (from a string or file contents)
scripting.include_script(include_str!("scripts/hello_world.rhai").to_string())
    .expect("script failed to compile");

// Evaluate a compiled script's AST
scripting.evaluate_ast(&scripting.compiled_scripts[0])?;
```

- `include_script` compiles and stores the AST; `evaluate_ast` runs it.
- Components derive `rhai::CustomType` (via `Component!`), so component types
  can be exposed to scripts once you register them with the RHAI engine.
- `scripts/hello_world.rhai` is a sample script (a recursive factorial).
- **Status:** the `ScriptingEngine` is standalone — it is not yet wired into
  the engine's frame loop.

---

## Math Utilities

`bob_engine::runtime::math`:

### `Float2`

SIMD-friendly 2D float vector with full operator support
(`+`, `-`, `*`, `/`, `*` scalar, `Neg`, `+=`, etc.):

```rust
use bob_engine::runtime::math::Float2;

let a = Float2::new(3.0, 4.0);
let b = Float2::ONE;

a.dot(b);            // dot product
a.length();          // magnitude
a.length_sq();
a.normalize();       // exact
a.normalize_fast();  // fast approximation
a.cross(b);          // 2D cross (scalar)
Float2::cross_scalar_vec(omega, r); // angular velocity x radius
a.rotate(angle);     // rotate by radians
a.lerp(b, 0.5);
a.clamp(min, max);
a.distance(b);
a.reflect(normal);
a.perp();            // perpendicular
a.floor(); a.ceil();
a.min(b); a.max(b); a.abs();

// Constants: Float2::ZERO, ONE, X, Y, INF, NEG_INF
// Conversions: From<[f32; 2]>, From<(f32, f32)>, and back
```

### `Int2`

Integer 2D vector with `dot`, `length_sq`, `min`/`max`, `abs`, `clamp`,
`cross`, `manhattan`, `chebyshev`, `to_float2`, and operator support.

### Angles

```rust
use bob_engine::runtime::math::{degrees_to_radians, radians_to_degrees, angle_to_point};

angle_to_point(from, to); // radians, direction from `from` to `to`
```

### RNG

```rust
use bob_engine::runtime::math::rng::random_float2;

let p = random_float2(Float2::new(-10.0, -10.0), Float2::new(10.0, 10.0));
```

---

## Engine Constants

`bob_engine::constants` (also re-exported from `runtime::engine`):

| Constant | Value | Meaning |
| --- | --- | --- |
| `MAIN_WORLD` | `"main"` | Name of the default world |
| `RENDER_GROUP` | `"render_group"` | Built-in render system group |
| `PHYSICS_GROUP` | `"physics_group"` | Built-in physics system group |
| `PHYSICS_CONNECTION_GROUP` | `"physics_connection_group"` | Built-in joint solver group |
| `RENDER_BATCH_SIZE` | `4096` | Nominal batch size |
| `FIXED_DT` | `1/60` | Target frame time (seconds) |
| `INV_FIXED_DT` | `60` | Inverse of `FIXED_DT` |

---

## Building & Running

```bash
cargo build                 # debug build
cargo build --release       # optimized release build
cargo run                   # run the demo binary (src/main.rs)
cargo test                  # run tests (e.g. Float2/Int2 unit tests)
```

Release profile (`Cargo.toml`): `opt-level = 3`, `lto = true`,
`codegen-units = 1`, `panic = "abort"`, `strip = "symbols"`.

The `simd` feature (enabled by default) enables SSE/NEON fast paths in
`Float2` (`dot`, `normalize_fast`).

---

## Gotchas & Caveats

- **Components must be `Copy`.** `Component!` derives `Copy`, so no
  `Vec`/`String`/`Option` fields inside components. (Use fixed-size arrays,
  `FixedStr`, or store data outside the ECS.)
- **`add_component` panics on unregistered types.** Use `add_component_safe`
  unless you've explicitly called `register_component`.
- **`get_component`/`get_component_mut` are closure-based** and return
  `Option<R>` — the closure only runs if the entity has the component.
- **`has_component` takes a raw `usize`**, not an `Entity`.
- **Entity IDs are never reused.** `destroy_entity` just clears the alive
  flag; `entity_count()` counts alive entities.
- **System group order is nondeterministic** (groups live in a `HashMap`);
  only intra-group ordering (by the `order` value) is guaranteed.
- **`Parallel` groups are not truly concurrent** — each `update()` spawns a
  thread and joins it.
- **`BatchHandle` slots are assigned asynchronously.** A freshly spawned
  entity with a `BatchHandle` won't render until `BatchAllocator` assigns it a
  slot (next frame).
- **Zoom is clamped to `0.001..1.0`** — you can zoom out, but not past the
  default 1.0 zoom level.
- **`GuiTextBox` is capped at 1024 characters.**
- **Tilemap rendering is currently disabled** in `record_render_pass`
  (the draw calls are commented out), even though `create_tilemap` works.
- **`cfg/asset_store.json` must stay in sync** with the assets you expect to
  be available at runtime — `AssetStore::init()` loads exactly the paths
  listed there.
- **Stub features:** `SceneManager`, world deserialization,
  `EntityChangeBuffer`, `GuiEventSystem`, and the RHAI integration into the
  frame loop are not implemented yet.
- **The built-in camera controls** (WASD + 1/2 zoom in `player_loop`) are
  hardcoded into the engine and will respond to input in every app.
