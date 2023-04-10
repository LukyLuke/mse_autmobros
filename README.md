[![.github/workflows/rust.yml](https://github.com/LukyLuke/mse_autmobros/actions/workflows/rust.yml/badge.svg)](https://github.com/LukyLuke/mse_autmobros/actions/workflows/rust.yml)
[![.github/workflows/clippy.yml](https://github.com/LukyLuke/mse_autmobros/actions/workflows/clippy.yml/badge.svg)](https://github.com/LukyLuke/mse_autmobros/actions/workflows/clippy.yml)

# MSE AutMobRoS - Autonomous Mobile Robotic Systems

Playground for MSE-Module TSM_AutMobRoS

## Pathfinder

### Grassfire

This is a board/pixel bases Algorithm.

Grassfire starts at the end and fills all fields around it.
In the next round, all fields with a value are taken and all its neighbors are filled with a higher value than the field itself.
If a neighbor is already filled, it is not updated.

Two versions of the Grassfire-Algorithm to show how fast it can be if optimized.

* **v1:** This version loops through all rows and cols and updates the neighbours as soon as it finds a value. This is a 4-Neighborhood calculation.
* **v2:** In this version the fields which have to be calculated next are cached. So no looping thorugh all rows and columns. This is a 4-Neighborhood calculation.
* **v3:** This is an 8-Neighborhood calculation and pathfinding based on the optimized *v2*.
* **v4:** This is an 4-Neighborhood calculation (based on *v2*) and a 8-Neighborhood pathfinding (as in *v3*).


### A* - Algorithm

This is a board/pixel bases Algorithm.

This is a implementation of an A*-Algorithm.
Each field has a value for the cost from the Start to the field and one value for the direct distance (Manhatten) to the end.
Each field can be estimated by `cost + distance` to check which one is the nearest.

On each run, the fields with the lowest estimation are processed and remembered.
Each field has a link to it's anchestor - similar to a reverse-linked-list.
If the end is reached, the path can be resolved by moving back to the ancestor until the start is reached.

* **v1:** This is a not very optimized version which can take some time if it ran into a wall.
* **v2:** Using a hashmap for the fields cache and other vector formats for the processed fields increases the performance. But still not as fast as an optimized grassfire.


### RRT - Rapidly-Exploring Roadmap Tree

This is a vector-based algorithm.

This implementation of a `Rapidly-Exploring Roadmap Tree` starts at the start point and randomly chooses a point on the whole area.
That random point is used to define the direction in which the tree should grow.
For every already known node on the map, the distance is calculated to the direction random point.
From the node with the smallest distance, a new point is calculated in the direction to the random direction point.

If the new point is inside of a predefined area around the end, the end point is taken instead.

After a predefined maximum of nodes, the calculation stops.
If in there was no conenction made to the end, there seems to be no possible connection.

* **v1:** Standard algorithm with no special optimization.


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
Create Area: 2.251088ms
Field Size: 1000x1000
Obstacles:  100 max 100x100

Grassfire-V1 Calc: 2.367617s
Grassfire-V1 Path-Calculation: 29.596000µs
Grassfire-V1 Path length: 1440
Saved: grassfire_v1.png

Grassfire-V2 Calc: 6.038268ms
Grassfire-V2 Path-Calculation: 14.037000µs
Grassfire-V2 Path length: 1440
Saved: grassfire_v2.png

Grassfire-V3 Calc: 8.443215ms
Grassfire-V3 Path-Calculation: 12.845000µs
Grassfire-V3 Path length: 869
Saved: grassfire_v3.png

Grassfire-V4 Calc: 5.854472ms
Grassfire-V4 Path-Calculation: 9.558000µs
Grassfire-V4 Path length: 899
Saved: grassfire_v4.png

A*-Algorithm Calc: 21.032154ms
A*-Algorithm Path-Calculation: 20.408000µs
A*-Algorithm Path length: 899
Saved: a_star_calculate.png

RRT-V1 End Reached: 1.379851s
RRT-V1 End Reached: 8196
RRT-V1 Calc: 5.516622s
RRT-V1 Path-Calculation: 4.949000µs
RRT-V1 Path length: 167
RRT-V1 Tree Edges: 16383
Saved: rrt_v1.png
```

* *Grassfire-v1* is nearly 400 times slower than *v2* and 230 times slower than *v3*
* *Grassfire-v2* is around 40%-50% faster than *v3* becasue it has 4 calculations less than *v3*
* *Grassfire-v3* has nearly 50% less steps the robot has to move than in *v1* and *v2*
* *Grassfire-v4* has around the same number of steps for the robot as *v3* but with the calculation speed of *v2*
* *A-v1* slows down drastically as soon as it hits an obstacle, the direction does not matter
* *RRT-v1* has to process until the number of steps is reached. In each step it has to check for more nodes. It does not matter from which direction the tree is built.

#### Example: Start bottom right, end top left

```
$ target/release/pathfinder 1000 1000 800 750 12 99
Create Area: 1.678380ms
Field Size: 1000x1000
Obstacles:  100 max 100x100

Grassfire-V1 Calc: 4.084009ms
Grassfire-V1 Path-Calculation: 42.400000µs
Grassfire-V1 Path length: 1440
Saved: grassfire_v1.png

Grassfire-V2 Calc: 4.865951ms
Grassfire-V2 Path-Calculation: 14.818000µs
Grassfire-V2 Path length: 1440
Saved: grassfire_v2.png

Grassfire-V3 Calc: 7.077984ms
Grassfire-V3 Path-Calculation: 11.872000µs
Grassfire-V3 Path length: 870
Saved: grassfire_v3.png

Grassfire-V4 Calc: 4.924151ms
Grassfire-V4 Path-Calculation: 9.679000µs
Grassfire-V4 Path length: 911
Saved: grassfire_v4.png

A*-Algorithm Calc: 12.722611ms
A*-Algorithm Path-Calculation: 22.262000µs
A*-Algorithm Path length: 911
Saved: a_star_calculate.png

RRT-V1 End Reached: 1.166924s
RRT-V1 End Reached: 7494
RRT-V1 Calc: 1.394317s
RRT-V1 Path-Calculation: 4.739000µs
RRT-V1 Path length: 130
RRT-V1 Tree Edges: 8192
Saved: rrt_v1.png
```

* *Grassfire-v1* is only 2-3 times slower, in most cases it is arount the same speed as *v2*. Under some circumstances it may be that *v1* is even faster because it can update all cells in one run.
* *Grassfire-v2* is around 40%-50% faster than *v3* becasue it has 4 calculations less than *v3*
* *Grassfire-v3* has nearly 50% less steps the robot has to move than in *v1* and *v2*
* *Grassfire-v4* has around the same number of steps for the robot as *v3* but with the calculation speed of *v2*
* *A-v1* slows down drastically as soon as it hits an obstacle, the direction does not matter
* *RRT-v1* has to process until the number of steps is reached. In each step it has to check for more nodes. It does not matter from which direction the tree is built.
