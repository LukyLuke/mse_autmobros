# mse_autmobros

Playground for MSE-Module AutMobRoS

## Pathfinder: Grassfire

[![.github/workflows/rust.yml](https://github.com/LukyLuke/mse_autmobros/actions/workflows/rust.yml/badge.svg)](https://github.com/LukyLuke/mse_autmobros/actions/workflows/rust.yml)
[![.github/workflows/clippy.yml](https://github.com/LukyLuke/mse_autmobros/actions/workflows/clippy.yml/badge.svg)](https://github.com/LukyLuke/mse_autmobros/actions/workflows/clippy.yml)

Two versions of the Grassfire-Algorithm to show how fast it can be if optimized.

* **v1:** This version loops through all rows and cols and updates the neighbours as soon as it finds a value.
* **v2:** In this version the fields which have to be calculated next are cached. So no looping thorugh all rows and columns.

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
Field Size: 1000x1000
Obstacles:  100 max 100x100

Grassfire-V1 Calc: 130.348262s
Path-Calculation: 172.375000µs
Saved: grassfire_v1.png
Grassfire-V2 Calc: 212.476567ms
Path-Calculation: 153.879000µs
Saved: grassfire_v2.png
```

*Implementation v1 is ~600 times slower becuase it has to go through all rows and cols until the near end again and again.*

### Example: Start bottom right, end top left

```
$ target/release/pathfinder 1000 1000 800 750 12 99
Field Size: 1000x1000
Obstacles:  100 max 100x100

Grassfire-V1 Calc: 154.414003ms
Path-Calculation: 218.261000µs
Saved: grassfire_v1.png
Grassfire-V2 Calc: 196.079796ms
Path-Calculation: 198.043000µs
Saved: grassfire_v2.png
```

*Implementation v1 is not really slower becuase it can update the whole row in one loop as soon as it has found the first value.*
