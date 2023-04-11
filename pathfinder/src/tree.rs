/// This module handles different Tree-Pathfinder algorithms
///
/// # Rapidly-Exploring RandomTree Algorithm
///
/// 1. A random point on the area is choosen.
/// 2. For each existing node, check if in the direction to the random point, a node can be added (no obstacle)
/// 3. Use the one new node which is nearest to the random point

use crate::{Instant, thread_rng, Rng};

const MAX_NODES: usize = 16383;
const STEP_DISTANCE: f32 = 10.0;
const END_POSITION: i32 = 5; // radius around the end to catch the end position
const REWIRE_DISTANCE_FACTOR: i32 = 2; // Check all nodes to rewire in the radius: `REWIRE_DISTANCE_FACTOR * STEP_DISTANCE`

/// The result of a tree algorithm
#[derive(Debug)]
pub struct TreeResult {
	/// The path from start to the end
	pub path: Vec<(usize, usize)>,
	/// The tree as lines/tuples: ((x0, y0), (x1, y1))
	pub tree: Vec<((usize, usize), (usize, usize))>,
}

/// Inner type representing a node
#[derive(Debug, Clone)]
struct Node {
	/// Coordinates
	pos: (usize, usize),
	// Parent node index
	parent: usize,
	// Distance to the start
	distance: f32,
}

/// Use the Rapidly-Exploring RandomTree Algorithm: RRT
///
/// # Arguments
///
/// * `area` - The play field as a one-dimensional vector
/// * `rows` - number of rows
/// * `cols` - number of cols
/// * `start` - start position (row, col)
/// * `end` - end position (row, col)
///
/// # Result
///
/// A tuple with:
/// * A Vector of tuples where each tuple represents a waypoint
/// * A Vector of tuples where each tuple represents an edge of the tree
pub fn rrt_v1(area: &mut [u64], rows: &usize, cols: &usize, start:(usize, usize), end:(usize, usize)) -> TreeResult {
	let benchmark = Instant::now();
	let mut rng = thread_rng();
	let mut found_end = false;

	// Configuration
	let max_nodes = usize::min(MAX_NODES, (area.len() as f32 / STEP_DISTANCE) as usize);
	let mut nodes: Vec<Node> = Vec::with_capacity(max_nodes);
	let finish_range = get_range(end, END_POSITION);

	// Initialize with the start node
	nodes.push(Node {
		pos: start,
		parent: 0,
		distance: 0.0,
	});

	loop {
		// Loop until we have filled the whole nodes vector
		if nodes.len() == max_nodes { break; }

		let mut last_distance = f32::MAX;
		let mut new_node = Node {
			pos: (0, 0),
			parent: 0,
			distance: 0.0,
		};

		// 1. Get a random point on the area where to give the direction
		let direction_node = (rng.gen_range(0..*rows), rng.gen_range(0..*cols));

		// 2. Find the nearest collision free Node
		nodes.iter().enumerate().for_each(|(key, node)| {
			// Do not grow the tree from the end
			if node.pos != end {
				// The new node to check
				let (new_pos, distance) = get_new_position(node.pos, direction_node, STEP_DISTANCE);

				// Check if the new node may be a valid one and update it
				if last_distance > distance && is_collision_free(area, rows, node, new_pos) {
					last_distance = distance;

					new_node.parent = key;
					new_node.pos = if is_in_range(new_pos, finish_range) {
						end
					} else {
						new_pos
					};
				}
			}
		});

		if last_distance < f32::MAX {
			nodes.push(new_node.clone());
			if new_node.pos == end {
				found_end = true;
				println!("RRT-V1 End reached within {}: {:.6?}", nodes.len(), benchmark.elapsed());
			}
		}
	}

	if !found_end { println!("RRT-V1 Calc: No conneciton found"); }
	println!("RRT-V1 Calc: {:.6?}", benchmark.elapsed());

	// For the return value we need only the position as a tuple ((x0, y0), (x1, y1)) to draw the tree/network
	let result_nodes = nodes.iter().map(|node| (node.pos, nodes[node.parent].pos)).collect();

	// Return the tuple of the path-vector and tree-vector
	TreeResult {
		path: find_path("RRT-V1", end, &nodes),
		tree: result_nodes,
	}
}

/// Use the Rapidly-Exploring RandomTree Algorithm: RRT*
///
/// Version 2:
///
/// Every new Node is connected to the neares Neighbour and not to the one from which directed node was calculated.
/// After connection the new node, every node in a given area around is checked if it may be rewired
/// with the new one to have a more direct conneciton to the start.
///
/// # Arguments
///
/// * `area` - The play field as a one-dimensional vector
/// * `rows` - number of rows
/// * `cols` - number of cols
/// * `start` - start position (row, col)
/// * `end` - end position (row, col)
///
/// # Result
///
/// A tuple with:
/// * A Vector of tuples where each tuple represents a waypoint
/// * A Vector of tuples where each tuple represents an edge of the tree
pub fn rrt_v2(area: &mut [u64], rows: &usize, cols: &usize, start:(usize, usize), end:(usize, usize)) -> TreeResult {
	let benchmark = Instant::now();
	let mut rng = thread_rng();
	let mut found_end = false;

	// Configuration
	let max_nodes = usize::min(MAX_NODES, (area.len() as f32 / STEP_DISTANCE) as usize);
	let mut nodes: Vec<Node> = Vec::with_capacity(max_nodes);
	let finish_range = get_range(end, END_POSITION);

	// Initialize with the start node
	nodes.push(Node {
		pos: start,
		parent: 0,
		distance: 0.0,
	});

	loop {
		// Loop until we have filled the whole nodes vector
		if nodes.len() == max_nodes { break; }

		let mut last_distance = f32::MAX;
		let mut new_node = Node {
			pos: (0, 0),
			parent: 0,
			distance: 0.0,
		};

		// 1. Get a random point on the area where to give the direction
		let direction_node = (rng.gen_range(0..*rows), rng.gen_range(0..*cols));

		// 2. Find the nearest collision free Node
		nodes.iter().for_each(|node| {
			// Do not grow the tree from the end
			if node.pos != end {
				// The new node to check
				let (new_pos, distance) = get_new_position(node.pos, direction_node, STEP_DISTANCE);

				// Check if the new node may be a valid one and update it
				if last_distance > distance && is_collision_free(area, rows, node, new_pos) {
					last_distance = distance;

					new_node.pos = if is_in_range(new_pos, finish_range) {
						end
					} else {
						new_pos
					};
				}
			}
		});

		if last_distance < f32::MAX {
			// Find the nearest node to connect to
			last_distance = f32::MAX;
			nodes.iter().enumerate().for_each(|(key, node)| {
				let (_, _, distance) = get_distances(new_node.pos, node.pos, nodes[key].distance);
				if last_distance > distance && is_collision_free(area, rows, node, new_node.pos) {
					last_distance = distance;
					new_node.parent = key;
					new_node.distance = distance;
				}
			});

			// Append the node to the list
			nodes.push(new_node.clone());
			let new_node_index = nodes.len() - 1;
			let check_range = get_range(new_node.pos, (STEP_DISTANCE as i32) * REWIRE_DISTANCE_FACTOR);

			// Rewire all nodes
			nodes.iter_mut().for_each(|node| {
				if node.pos != new_node.pos && is_in_range(node.pos, check_range) {
					let (_, _, distance) = get_distances(new_node.pos, node.pos, new_node.distance);
					if node.distance > distance && is_collision_free(area, rows, node, new_node.pos) {
						node.parent = new_node_index;
						node.distance = distance;
					}
				}
			});

			if new_node.pos == end {
				found_end = true;
				println!("RRT-V2 End reached within {}: {:.6?}", nodes.len(), benchmark.elapsed());
			}
		}
	}

	if !found_end { println!("RRT-V2 Calc: No conneciton found"); }
	println!("RRT-V2 Calc: {:.6?}", benchmark.elapsed());

	// For the return value we need only the position as a tuple ((x0, y0), (x1, y1)) to draw the tree/network
	let result_nodes = nodes.iter().map(|node| (node.pos, nodes[node.parent].pos)).collect();

	// Return the tuple of the path-vector and tree-vector
	TreeResult {
		path: find_path("RRT-V2", end, &nodes),
		tree: result_nodes,
	}
}

/// Get a range tuple around the given position
///
/// # Arguments:
///
/// * `pos` - The position to get the range around
/// * `range` - Dimension for the range +/- left/right and up/down
///
/// # Result
///
/// Two tuples representing the range: ((x0, x1), (y0, y1))
fn get_range(pos: (usize, usize), range: i32) -> ((usize, usize), (usize, usize)) {
	 (
		(
			i32::max(0, pos.0 as i32 - range) as usize,
			pos.0 + range as usize
		),
		(
			i32::max(0, pos.1 as i32 - range) as usize,
			pos.1 + range as usize
		),
	)
}

/// Checks if the given postion is in the defined range
///
/// # Arguments:
///
/// * `pos` - Position to check, a Tuple of (x, y)
/// * `range` - The range to check, a Tuple of ((x0, x1), (y0, y1))
///
/// # Result
///
/// If the point is in the x-y range
fn is_in_range(pos: (usize, usize), range: ((usize, usize), (usize, usize))) -> bool {
	(pos.0 >= range.0.0 && pos.0 <= range.0.1) && (pos.1 >= range.1.0 && pos.1 <= range.1.1)
}

/// Get the distances X, Y and direct between two points
///
/// # Arguments:
///
/// * `p1` - Start point
/// * `p2` - End Point
/// * `offset` - Offset to add to the distance
///
/// # Result
///
/// A tuple with the three distances: (X, Y, Direct)
fn get_distances(p1: (usize, usize), p2: (usize, usize), offset: f32) -> (f32, f32, f32) {
	let direction_x = p2.0 as f32 - p1.0 as f32;
	let direction_y = p2.1 as f32 - p1.1 as f32;
	let distance = f32::sqrt((direction_x * direction_x) + (direction_y * direction_y)) + offset;

	(direction_x, direction_y, distance)
}

/// Returns a new point directing to the given end, maximum step_size apart from the start
///
/// # Arguments:
///
/// * `start` - The point from which the new one should be `step_size` away
/// * `direction` - The point in which the new point should direct to
/// * `step_size` - Radius around the start to create the new point
///
/// # Result
///
/// A tuple represents the new point, step_size apart from the start.
/// The third parameter represents the distance between the start and the direction point
fn get_new_position(start: (usize, usize), direction: (usize, usize), step_size: f32) -> ((usize, usize), f32) {
	let (direction_x, direction_y, distance) = get_distances(start, direction, 0.0);

	// If the direction-point is nearer than the new point, use the direction-point
	if step_size >= distance {
		return (direction, distance);
	}

	let angle = direction_y.atan2(direction_x);
	let new_x = (start.0 as f32 + (step_size * angle.cos())) as usize;
	let new_y = (start.1 as f32 + (step_size * angle.sin())) as usize;
	((new_x, new_y), distance)
}

/// Checks the area if between the two given points is an obstacle.
/// This is done by simply "draw" a line between and check the line-pixels on the area.
///
/// # Arguments:
///
/// * `area` - The area as a vector of u64 where every obstacle is u64::MAX
/// * `rows` - Number of rows, where rows x cols is the size of the area
/// * `node` - Node from where the line to check starts
/// * `new_pos` - A tuple represents the end point
///
/// # Result
///
/// Returns if there is an obstacle between the two points
fn is_collision_free(area: &[u64], rows: &usize, node: &Node, new_pos: (usize, usize)) -> bool {
	let p1 = node.pos;
	let p2 = (new_pos.0, new_pos.1);

	let dx = if p1.0 > p2.0 { p1.0 - p2.0 } else { p2.0 - p1.0 } as i64;
	let dy = if p1.1 > p2.1 { p1.1 - p2.1 } else { p2.1 - p1.1 } as i64;

	let sx = if p1.0 < p2.0 { 1 } else { -1 } as i64;
	let sy = if p1.1 < p2.1 { 1 } else { -1 } as i64;

	// Initialize error
	let mut err = if dx > dy { dx } else { -dy } / 2;
	let mut err2;

	let mut x = p1.0 as i64;
	let mut y = p1.1 as i64;
	loop {
		if x == p2.0 as i64 && y == p2.1 as i64 {
			break
		}

		if area[(y as usize * rows) + x as usize] == u64::MAX {
			return false;
		}

		// Store old error
		err2 = 2 * err;

		// Adjust error and start position
		if err2 > -dx {
			err -= dy;
			x += sx;
		}
		if err2 < dy {
			err += dx;
			y += sy;
		}
	}
	true
}

/// Based on all nodes which represents the tree, the path back from the end to the start is extracted and returned
///
/// # Arguments:
///
/// * `algorithm` - The name of the Algorithm (for benchmark logging)
/// * `end` - The End-Position as a tuple (x, y)
/// * `nodes` - All the nodes of the tree
///
/// # Result:
///
/// The Path from the end to the start as tuples of coordinates [(x, y)]
fn find_path(algorithm: &str, end: (usize, usize), nodes: &[Node]) -> Vec<(usize, usize)> {
	let benchmark = Instant::now();
	let mut result = vec![];

	// 1. Find the end in the tree, then go backwards
	let finish = nodes.iter().find(|&node| node.pos == end).unwrap_or(&nodes[0]);
	result.push(finish.pos);

	// 2. loop until the start is reached and color the area
	let mut parent = finish.parent;
	loop {
		let node = &nodes[parent];
		result.push(node.pos);
		if node.parent == 0 {
			break;
		}
		parent = node.parent;
	}
	result.push(nodes[0].pos);

	println!("{} Path-Calculation: {:.6?}", algorithm, benchmark.elapsed());
	println!("{} Path length: {}", algorithm, result.len());
	println!("{} Tree Edges: {}", algorithm, nodes.len());

	result
}


