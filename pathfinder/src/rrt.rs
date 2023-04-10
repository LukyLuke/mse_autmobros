use crate::{Instant, thread_rng, Rng};

const MAX_NODES: usize = 16383;
const END_POSITION: i32 = 5; // radius around the end to catch the end position

/// Use the Probabilistic Random Tree Algorithm to find a path.
///
/// Version 1:
///
/// * Fixed Distance
/// * Random direction
/// * Random Node
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
#[allow(clippy::type_complexity)]
pub fn v1(area: &mut [u64], rows: &usize, cols: &usize, start:(usize, usize), end:(usize, usize)) -> (Vec<(usize, usize)>, Vec<(usize, usize, usize, usize)>) {
	let benchmark = Instant::now();
	let mut rng = thread_rng();
	let mut found_end = false;

	// Configuration
	let max_distance = 10.0;
	let max_nodes = usize::min(MAX_NODES, (area.len() as f32 / max_distance) as usize);
	let mut nodes: Vec<Node> = Vec::with_capacity(max_nodes);

	// Range in which the algorithm will connect to the end
	// this is `+/- END_POSITION` of the end itself
	let finish_range: (usize, usize, usize, usize) = (
		i32::max(0, end.0 as i32 - END_POSITION) as usize,
		end.0 + END_POSITION as usize,
		i32::max(0, end.1 as i32 - END_POSITION) as usize,
		end.1 + END_POSITION as usize,
	);

	// Initialize with the start node
	nodes.push(Node {
		pos: start,
		parent: 0,
	});

	loop {
		// Loop until we have filled the whole nodes vector
		if nodes.len() == max_nodes { break; }

		// 1. Get a random point on the area where to drive to
		let random_sample = (rng.gen_range(0..*rows), rng.gen_range(0..*cols));

		// 2. Find the nearest collision free Node
		let mut last_distance = f32::MAX;
		let mut new_node = Node {
			pos: (0, 0),
			parent: 0,
		};

		let mut replaced_with_end = false;
		nodes.iter().enumerate().for_each(|(key, node)| {
			// The new node to check
			let new_pos = get_new_position(node.pos, random_sample, max_distance);
			let new_distance = new_pos.2;

			// Check if the new node may be a valid one and update it
			if last_distance > new_distance && is_collision_free(area, rows, node, new_pos) {
				last_distance = new_pos.2;

				// Check if the new point is in the finishing area
				new_node.parent = key;
				new_node.pos = if (new_pos.0 >= finish_range.0 && new_pos.0 <= finish_range.1) && (new_pos.1 >= finish_range.2 && new_pos.1 <= finish_range.3) {
					replaced_with_end = true;
					end
				} else {
					replaced_with_end = false;
					(new_pos.0, new_pos.1)
				};
			}
		});

		if last_distance < f32::MAX {
			nodes.push(new_node.clone());
			if !found_end && replaced_with_end {
				found_end = true;
				println!("RRT-V1 End Reached: {:.6?}", benchmark.elapsed());
				println!("RRT-V1 End Reached: {:?}", nodes.len());
			}
		}
	}

	if !found_end { println!("RRT-V1 Calc: No conneciton found"); }
	println!("RRT-V1 Calc: {:.6?}", benchmark.elapsed());

	// For the return value we need only the position as a tuple (x0, y0, x1, y1) to draw the tree/network
	let result_nodes = nodes.iter().map(|node| (node.pos.0, node.pos.1, nodes[node.parent].pos.0, nodes[node.parent].pos.1)).collect();

	// Return the tuple of the path-vector and tree-vector
	(find_path("RRT-V1", end, area, rows, cols, &nodes), result_nodes)
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
fn get_new_position(start: (usize, usize), direction: (usize, usize), step_size: f32) -> (usize, usize, f32) {
	let direction_x = direction.0 as f32 - start.0 as f32;
	let direction_y = direction.1 as f32 - start.1 as f32;
	let distance = f32::sqrt( (direction_x * direction_x) as f32  +  (direction_y * direction_y) as f32 );

	let max_step_size = if step_size < distance { step_size } else { distance } as f32;

	let angle = direction_y.atan2(direction_x);
	let new_x = (start.0 as f32 + (max_step_size * angle.cos())) as usize;
	let new_y = (start.1 as f32 + (max_step_size * angle.sin())) as usize;

	(new_x, new_y, distance)
}

/// Checks the area if between the two given points is am obstacle
///
/// # Arguments:
///
/// * `area` - The area as a vector of u64 where every obstacle is u64::MAX
/// * `rows` - Number of rows, where rows x cols is the size of the area
/// * `node` - Node from where the line to check starts
/// * `new_pos` - A tuple represents the end point incl. the distance to it
///
/// # Result
///
/// Returns if there is an obstacle between the two points
fn is_collision_free(area: &[u64], rows: &usize, node: &Node, new_pos: (usize, usize, f32)) -> bool {
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

#[derive(Debug, Clone)]
struct Node {
	pos: (usize, usize),
	parent: usize,
}


fn find_path(algorithm: &str, end: (usize, usize), _area: &[u64], _rows: &usize, _cols: &usize, nodes: &[Node]) -> Vec<(usize, usize)> {
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


