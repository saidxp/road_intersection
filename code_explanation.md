# Code Documentation: Road Intersection Simulation

This document provides a detailed function-by-function explanation of the Rust source files in the project.

## 1. `src/main.rs`

### Overview
The entry point of the application. It initializes the game loop and the main `IntersectionManager`.

### Functions

#### `main`
- **Signature**: `async fn main()`
- **Purpose**: The asynchronous main function required by `macroquad`.
- **Logic**:
    1.  Sets the window title via the `#[macroquad::main]` attribute.
    2.  Initializes the `IntersectionManager`.
    3.  Enters the main game loop (`loop { ... }`).
    4.  Calls `intersection.update()` and `intersection.draw()`.
    5.  Waits for the next frame (`next_frame().await`).

## 2. `src/app_config.rs`

### Overview
Defines global constants and configuration settings for the simulation (e.g., lane capacity, car size, speeds, timer durations).

## 3. `src/types.rs`

### Overview
Defines core enums used throughout the application.

### Enums
- **Route**: `Right`, `Left`, `Straight`.
- **Origin**: `North`, `South`, `West`, `East`.

## 4. `src/entity/vehicle.rs`

### Overview
Defines the `Vehicle` struct and its behavior, including movement, rendering, and collision avoidance.

### Struct: `Vehicle`
- **Fields**:
    - `id`: Unique identifier (atomic counter).
    - `pos`: Current position `(x, y)`.
    - `speed`: Current velocity vector `(vx, vy)`.
    - `origin`: Direction from which the vehicle spawned.
    - `route`: Intended path.
    - `color`: Visual color.
    - `turned`: Boolean flag indicating if the vehicle has completed its turn.

### Functions

#### `new`
- **Purpose**: Constructor. Assigns a unique ID and initializes fields.

#### `draw`
- **Purpose**: Renders the vehicle as a rectangle with an outline.

#### `update`
- **Purpose**: Updates position based on speed and handles turning logic via `update_turning_direction`.

#### `should_despawn`
- **Purpose**: Returns `true` if the vehicle is far outside the screen bounds.

#### `update_turning_direction`
- **Purpose**: Checks if the vehicle reached its turning point (based on `route` and `origin`) and updates the velocity vector.

#### `is_safe_to_move`
- **Purpose**: Determines if the vehicle can move forward.
- **Logic**:
    1.  **Traffic Light**: Checks if the light is RED at the stop line.
    2.  **Collision**: Checks if another vehicle is immediately ahead in the same lane.

## 5. `src/entity/traffic_light.rs`

### Overview
Manages the traffic light logic using `TrafficLightSystem`.

### Struct: `TrafficLightSystem`
- **Fields**:
    - `active_green`: The direction (`Origin`) currently having a green light.
    - `green_timer`: Tracks duration of the current green light.

### Functions

#### `update`
- **Purpose**: Updates the timer and decides when to switch lights.
- **Logic**: Switches if the intersection center is empty AND the minimum green time has passed.

#### `decide_next_green`
- **Purpose**: Selects the next lane for a green light.
- **Logic**: Prioritizes the lane with the highest number of waiting vehicles (highest pressure).

#### `draw`
- **Purpose**: Renders the traffic lights at the intersection corners.

## 6. `src/manager/intersection.rs`

### Overview
Manages the overall simulation state (`IntersectionManager`).

### Struct: `IntersectionManager`
- **Fields**:
    - `vehicles`: List of active vehicles.
    - `traffic_system`: Instance of `TrafficLightSystem`.
    - `center`: Screen center coordinates.

### Functions

#### `update`
- **Purpose**: Main update loop.
- **Logic**:
    1.  Updates `center`.
    2.  Delegates input handling to `InputManager`.
    3.  Updates `traffic_system`.
    4.  Updates each vehicle (movement, despawning, safety checks).

#### `draw`
- **Purpose**: Renders environment, traffic lights, vehicles, and UI text.

## 7. `src/manager/input.rs`

### Overview
Handles user input (`InputManager`) to spawn vehicles.

### Functions

#### `handle_input`
- **Purpose**: Checks for arrow keys or 'R' key.
- **Logic**: Calls `Spawner::try_spawn` if the lane capacity allows.

## 8. `src/manager/spawner.rs`

### Overview
Handles creation of new vehicles (`Spawner`).

### Functions

#### `try_spawn`
- **Purpose**: Attempts to spawn a vehicle if the location is safe (no collision with existing cars).
- **Logic**: Randomizes `Route` and `Color` if spawning is successful.

## 9. `src/render/draw.rs`

### Overview
Provides the `Renderer` struct for drawing the static environment.

### Functions

#### `draw_environment`
- **Purpose**: Draws the background, roads, and lane markings.

## 10. `src/render/palette.rs`

### Overview
Defines the color palette constants used for rendering (Grass, Asphalt, Markings, Lights, Cars).
