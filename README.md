[![.github/workflows/rust.yml](https://github.com/LukyLuke/mse_autmobros/actions/workflows/rust.yml/badge.svg)](https://github.com/LukyLuke/mse_autmobros/actions/workflows/rust.yml)
[![.github/workflows/clippy.yml](https://github.com/LukyLuke/mse_autmobros/actions/workflows/clippy.yml/badge.svg)](https://github.com/LukyLuke/mse_autmobros/actions/workflows/clippy.yml)

# MSE AutMobRoS - Autonomous Mobile Robotic Systems

Playground for MSE-Module TSM_AutMobRoS

## Pathfinder

### Grassfire

Two versions of the Grassfire-Algorithm to show how fast it can be if optimized.

* **v1:** This version loops through all rows and cols and updates the neighbours as soon as it finds a value. This is a 4-Neighborhood calculation.
* **v2:** In this version the fields which have to be calculated next are cached. So no looping thorugh all rows and columns. This is a 4-Neighborhood calculation.
* **v3:** This is an 8-Neighborhood calculation and pathfinding based on the optimized *v2*.
* **v4:** This is an 4-Neighborhood calculation (based on *v2*) and a 8-Neighborhood pathfinding (as in *v3*).

### A* - Algorithm

This is a implementation of an A*-Algorithm.
Each field has a value for the cost from the Start to the field and one value for the direct distance (Manhatten) to the end.
Each field can be estimated by `cost + distance` to check which one is the nearest.

On each run, the fields with the lowest estimation are processed and remembered.
Each field has a link to it's anchestor - similar to a reverse-linked-list.
If the end is reached, the path can be resolved by moving back to the ancestor until the start is reached.

* **v1:** This is a not very optimized version which can take some time if it ran into a wall.

**TODO:** Not very fast, probably when going back after a dead end there can be something be optimized. Even if there is no obstacle in it's way, it's slower than Grassfire::v1 :?


### Usage

```
$ target/release/pathfinder ROWS COLS START_ROW START_COL END_ROW END_COL [OBSTACLES]
$ target/release/pathfinder ROWS COLS START_ROW START_COL END_ROW END_COL [OBSTACLES]
```

* **ROWS** Number of rows of the area
* **COLS** Number of columns of the area
* **START_ROW** X-Position (row) of the robot
* **START_COL** Y-Position (column) of the robot
* **END_ROW** X-Position where to drive to
* **END_COL** Y-Position where to drive to
* **OBSTACLES** Optional number of obstacles/walls; default 100

#### Example: Start top left, end bottom right

```
$ target/release/pathfinder 1000 1000 12 99 800 750
Create Area: 13.234211ms
Field Size: 1000x1000
Obstacles:  100 max 100x100

Grassfire-V1 Calc: 38.070922s
Grassfire-V1 Path-Calculation: 222.449000µs
Grassfire-V1 Path length: 1440
Saved: grassfire_v1.png

Grassfire-V2 Calc: 97.991937ms
Grassfire-V2 Path-Calculation: 208.282000µs
Grassfire-V2 Path length: 1440
Saved: grassfire_v2.png

Grassfire-V3 Calc: 162.971837ms
Grassfire-V3 Path-Calculation: 179.437000µs
Grassfire-V3 Path length: 818
Saved: grassfire_v3.png

Grassfire-V4 Calc: 96.350969ms
Grassfire-V4 Path-Calculation: 189.376000µs
Grassfire-V4 Path length: 964
Saved: grassfire_v4.png
```

* *v1* is nearly 400 times slower than *v2* and 230 times slower than *v3*
* *v2* is around 40%-50% faster than *v3* becasue it has 4 calculations less than *v3*
* *v3* has nearly 50% less steps the robot has to move than in *v1* and *v2*
* *v4* has around the same number of steps for the robot as *v3* but with the calculation speed of *v2*

#### Example: Start bottom right, end top left

```
$ target/release/pathfinder 1000 1000 800 750 12 99
Create Area: 16.536774ms
Field Size: 1000x1000
Obstacles:  100 max 100x100

Grassfire-V1 Calc: 31.927361ms
Grassfire-V1 Path-Calculation: 200.247000µs
Grassfire-V1 Path length: 1440
Saved: grassfire_v1.png

Grassfire-V2 Calc: 89.073367ms
Grassfire-V2 Path-Calculation: 177.133000µs
Grassfire-V2 Path length: 1440
Saved: grassfire_v2.png

Grassfire-V3 Calc: 131.293040ms
Grassfire-V3 Path-Calculation: 146.385000µs
Grassfire-V3 Path length: 808
Saved: grassfire_v3.png

Grassfire-V4 Calc: 93.812297ms
Grassfire-V4 Path-Calculation: 100.288000µs
Grassfire-V4 Path length: 808
Saved: grassfire_v4.png
```

* *v1* is only 2-3 times slower, in most cases it is arount the same speed as *v2*. Under some circumstances it may be that *v1* is even faster because it can update all cells in one run.
* *v2* is around 40%-50% faster than *v3* becasue it has 4 calculations less than *v3*
* *v3* has nearly 50% less steps the robot has to move than in *v1* and *v2*
* *v4* has around the same number of steps for the robot as *v3* but with the calculation speed of *v2*
