# mse_autmobros

Playground for MSE-Module AutMobRoS

## Pathfinder: Grassfire

[![.github/workflows/rust.yml](https://github.com/LukyLuke/mse_autmobros/actions/workflows/rust.yml/badge.svg)](https://github.com/LukyLuke/mse_autmobros/actions/workflows/rust.yml)
[![.github/workflows/clippy.yml](https://github.com/LukyLuke/mse_autmobros/actions/workflows/clippy.yml/badge.svg)](https://github.com/LukyLuke/mse_autmobros/actions/workflows/clippy.yml)

Two versions of the Grassfire-Algorithm to show how fast it can be if optimized.

* **v1:** This version loops through all rows and cols and updates the neighbours as soon as it finds a value. This is a 4-Neighborhood calculation.
* **v2:** In this version the fields which have to be calculated next are cached. So no looping thorugh all rows and columns. This is a 4-Neighborhood calculation.
* **v3:** This is an 8-Neighborhood calculation and pathfinding absed on the optimized *v2*.

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

### Example: Start top left, end bottom right

```
$ target/release/pathfinder 1000 1000 12 99 800 750
Create Area: 23.597266ms
Area 1 invalid: Start or End is inside an obstacle.
Create Area: 21.790536ms
Area 2 invalid: Start or End is inside an obstacle.
Create Area: 28.023848ms
Area 3 invalid: Start or End is inside an obstacle.
Create Area: 14.823762ms
Field Size: 1000x1000
Obstacles:  100 max 100x100

Grassfire-V1 Calc: 38.487738s
Grassfire-V1 Path-Calculation: 243.118000µs
Grassfire-V1 Path length: 1440
Saved: grassfire_v1.png

Grassfire-V2 Calc: 99.330146ms
Grassfire-V2 Path-Calculation: 207.100000µs
Grassfire-V2 Path length: 1440
Saved: grassfire_v2.png

Grassfire-V3 Calc: 177.441322ms
Grassfire-V3 Path-Calculation: 199.515000µs
Grassfire-V3 Path length: 855
Saved: grassfire_v3.png
```

* *v1* is nearly 400 times slower than *v2* and 215 times slower than *v3*
* *v2* is around 40%-50% faster than *v3* becasue it has 4 calculations less than *v3*
* *v3* has nearly 50% less steps the robot has to move than in *v1* and *v2*

### Example: Start bottom right, end top left

```
$ target/release/pathfinder 1000 1000 800 750 12 99
Create Area: 13.838047ms
Area 1 invalid: Start or End is inside an obstacle.
Create Area: 12.429355ms
Area 2 invalid: Start or End is inside an obstacle.
Create Area: 14.683338ms
Field Size: 1000x1000
Obstacles:  100 max 100x100

Grassfire-V1 Calc: 34.534503ms
Grassfire-V1 Path-Calculation: 191.280000µs
Grassfire-V1 Path length: 1440
Saved: grassfire_v1.png

Grassfire-V2 Calc: 90.071216ms
Grassfire-V2 Path-Calculation: 169.519000µs
Grassfire-V2 Path length: 1440
Saved: grassfire_v2.png

Grassfire-V3 Calc: 142.868994ms
Grassfire-V3 Path-Calculation: 155.412000µs
Grassfire-V3 Path length: 862
Saved: grassfire_v3.png
```

* *v1* is only 2.5 times slower, in most cases it is arount the same speed as *v2*. Under some circumstances it may be that *v1* is even faster because it can update all cells in one run.
* *v2* is around 40%-50% faster than *v3* becasue it has 4 calculations less than *v3*
* *v3* has nearly 50% less steps the robot has to move than in *v1* and *v2*
