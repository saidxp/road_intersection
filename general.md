# General Logic and Mathematics of the Road Intersection Simulation

This document outlines the mathematical and logical principles underlying the road intersection simulation.

## 1. Coordinate System and Geometry

### 1.1 Screen Space
The simulation runs in a 2D Cartesian coordinate system provided by the `macroquad` crate.
- **Origin (0, 0)**: Top-left corner of the window.
- **X-axis**: Increases to the right.
- **Y-axis**: Increases downwards.
- **Center**: Calculated dynamically as `(screen_width() / 2.0, screen_height() / 2.0)`.

### 1.2 Vectors and Position
Positions and velocities are represented as tuples `(f32, f32)`.
- **Position ($P$)**: $(x, y)$
- **Velocity ($V$)**: $(v_x, v_y)$

### 1.3 Movement Mathematics
Vehicle movement is simulated using discrete time steps (frame-based updates).
$$P_{new} = P_{old} + V$$
Where $V$ represents the displacement per frame.

### 1.4 Distance Calculation
Euclidean distance is used for collision detection and spawn safety checks.
$$d = \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2}$$
This is manually implemented to avoid dependencies on external vector libraries.

## 2. Logical Architecture

### 2.1 Simulation Loop
The core loop follows a standard Game Loop pattern:
1.  **Input Handling**: Check for user key presses to spawn cars.
2.  **Update Phase**:
    *   Update traffic light timers.
    *   Determine active green light based on traffic density.
    *   Update vehicle positions (if safe).
    *   Remove vehicles that have left the screen.
3.  **Render Phase**:
    *   Draw background and environment (roads).
    *   Draw traffic lights.
    *   Draw vehicles.

### 2.2 Vehicle Logic (State Machine)
Each vehicle operates as an independent agent with the following states (implicitly managed via boolean flags and position checks):
1.  **Spawned**: Created at the edge of the screen.
2.  **Moving**: Traveling straight along its initial vector.
3.  **Braking/Waiting**:
    *   **Traffic Light**: Stops if the light is RED and the vehicle is at the stop line.
    *   **Collision Avoidance**: Stops if another vehicle is immediately ahead (within `AHEAD_CHECK_DISTANCE`).
4.  **Turning**:
    *   Detects if it has reached the "turning point" (intersection center + offset).
    *   Changes velocity vector ($V$) to the new direction (e.g., $(0, s) \to (s, 0)$ for a left turn).
    *   Sets `turned = true` to ignore future turn checks.
5.  **Despawning**: Removed when coordinates exceed screen bounds + offset.

### 2.3 Traffic Control Algorithm
The traffic light system uses a **density-based adaptive algorithm** rather than a fixed timer.
1.  **Green Timer**: Ensures a light stays green for at least `MIN_GREEN_TIME`.
2.  **Switch Condition**:
    *   If the intersection center is empty AND the minimum green time has passed.
3.  **Selection Logic (Heuristic)**:
    *   Calculates "pressure" for each lane based on the count of waiting vehicles.
    *   Switches green light to the lane with the highest pressure (most waiting cars).

### 2.4 Spawning Logic
Spawning is controlled by `InputManager` and `Spawner`.
*   **Capacity Constraint**: Checks if the number of cars in a lane < `LANE_CAPACITY`.
*   **Safety Constraint**: Checks if the spawn point is clear (distance to nearest car > `SPAWN_DISTANCE_CHECK`).
*   **Randomization**: Randomly assigns `Route` (Straight, Left, Right) and `Color`.

## 3. Collision Detection Logic
Collision avoidance uses a simple ray-cast-like check:
*   **Forward Check**: A vehicle checks only vehicles that are "ahead" of it.
*   **Directional Filter**:
    *   If moving North ($v_y < 0$): Check cars with $y < y_{self}$.
    *   If moving South ($v_y > 0$): Check cars with $y > y_{self}$.
    *   Similar logic for East/West.
*   **Lateral Check**: Ensures the car ahead is in the same lane (lateral distance < `AHEAD_CHECK_LATERAL`).
