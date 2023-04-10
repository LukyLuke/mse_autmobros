use crate::Instant;
use std::collections::HashMap;

/// Use the A* algorithm to calculate the optimal way fro the start to the end.
///
/// Costs to go from one cell to an other:
/// - direct:   1
/// - diagonal: sqrt(2) = 1.4
///
/// Costs to go from a cell to the end:
/// - Manhattan-Distance
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
/// A Vector of tuples where each tuple represents a waypoint
pub fn calculate(area: &mut [u64], rows: &usize, cols: &usize, start:(usize, usize), end:(usize, usize)) -> Vec<(usize, usize)> {
	let benchmark = Instant::now();
	let cost_direct = 1.0;
	let cost_edge = 1.4;

	// Calculate the distance from the given position to the end
	// Manhattan: X-Distance + Y-Distance
	let calculate_distance = |field: (usize, usize)| {
		(field.0.max(end.0) - field.0.min(end.0)) + (field.1.max(end.1) - field.1.min(end.1))
	};

	// Calculate the cost to reach the given field
	let estimate_path_cost = |cost: f64, dist: usize| { (cost * 10.0) as u64 + (dist as u64 * 10) };

	// HashMap which holds all fields which can be processed in the next step
	// Key: tuple identifies the field (row, col)
	// Value: tuple identifies the (area-position, estimation)
	let mut fields_cache: HashMap<(usize, usize), (usize, u64)> = HashMap::new();
	fields_cache.insert(start, ((start.1 * rows) + start.0, 0));

	// All calculated fields have to be stored to go back to them in case a blocked path is calculated
	let mut calculated_fields: Vec<Option<Field>> = Vec::with_capacity(area.len());
	calculated_fields.resize_with(area.len(), || None);

	// Fill the start position
	calculated_fields[(start.1 * rows) + start.0] = Some(Field {
		prev: None,
		processed: false,
		cost: 0.0,
		estimate: estimate_path_cost(0.0, calculate_distance(start)),
		dist: calculate_distance(start),
	});

	// run as long as we have not reached the start field
	// or as long as we have neighbours to calculate
	loop {
		let mut changed = false;

		// Get the lowest calculated path length based on all known fields
		// If there is no field calculated yet, there can be no estimation
		let min_path_estimation = fields_cache.values()
			.map(|value| value.1)
			.min_by(|a, b| a.cmp(b))
			.unwrap_or_default();

		// Calculate all surrounding fields from the known fields which have the same minimal estimation
		let mut processed_fields: Vec<(usize, usize)> = vec![];
		let mut new_fields: Vec<(usize, usize, usize, u64)> = vec![];
		fields_cache.iter()
			.filter(|(_, values)| values.1 == min_path_estimation)
			.for_each(|(field, _)| {
				let current_field: &mut Field = calculated_fields[(field.1 * rows) + field.0].as_mut().unwrap();
				changed = true;
				current_field.processed = true;
				processed_fields.push(*field);

				let row_next = match field.0 + 1 { row if &row < rows => Some((field.0 + 1, field.1)),     _ => None };
				let row_prev = match field.0     { row if row > 0     => Some((field.0 - 1, field.1)),     _ => None };
				let col_next = match field.1 + 1 { col if &col < cols => Some((field.0,     field.1 + 1)), _ => None };
				let col_prev = match field.1     { col if col > 0     => Some((field.0,     field.1 - 1)), _ => None };
				let row_next_down = if row_next.is_some() && col_next.is_some() { Some(( field.0 + 1, field.1 + 1 )) } else { None };
				let row_next_up =   if row_prev.is_some() && col_next.is_some() { Some(( field.0 - 1, field.1 + 1 )) } else { None };
				let row_prev_down = if row_next.is_some() && col_prev.is_some() { Some(( field.0 + 1, field.1 - 1 )) } else { None };
				let row_prev_up =   if row_prev.is_some() && col_prev.is_some() { Some(( field.0 - 1, field.1 - 1 )) } else { None };

				// Add all new fields to the calculated fields area and to the processing list
				let current_cost = current_field.cost;
				let mut check_field = |f: Option<(usize, usize)>, cost: f64| {
					if let Some(fld) = f {
						let pos = (fld.1 * rows) + fld.0;
						let new_cost = current_cost + cost;

						let neigh = calculated_fields[pos].get_or_insert(Field {
							prev: Some(*field),
							processed: false,
							cost: new_cost,
							estimate: estimate_path_cost(new_cost, calculate_distance(fld)),
							dist: calculate_distance(fld),
						});

						// Check if this is a wall
						if area[pos] == u64::MAX {
							neigh.processed = true;
							neigh.estimate = u64::MAX;
							neigh.cost = f64::MAX;
						}
						// Check if this is a faster way based on the costs
						if neigh.cost > new_cost {
							neigh.prev = Some(*field);
							neigh.cost = new_cost;
							neigh.estimate = estimate_path_cost(new_cost, neigh.dist);
						}

						if area[pos] != u64::MAX {
							area[pos] = neigh.estimate / 10;
						}

						if !neigh.processed {
							new_fields.push(( fld.0, fld.1, pos, neigh.estimate ));
						}
					};
				};
				check_field(row_next_down, cost_edge);
				check_field(row_next_up,   cost_edge);
				check_field(row_prev_down, cost_edge);
				check_field(row_prev_up,   cost_edge);
				check_field(row_next, cost_direct);
				check_field(row_prev, cost_direct);
				check_field(col_next, cost_direct);
				check_field(col_prev, cost_direct);
			});

		// Clean up all processed fields and add new ones
		processed_fields.iter().for_each(|idx| { fields_cache.remove(idx); });
		new_fields.iter().for_each(|neigh| {
			fields_cache.insert((neigh.0, neigh.1), (neigh.2, neigh.3));
		});

		// This is not working in the while loop
		if !changed { println!("A*-Algorithm: No conneciton possible"); break; }
		if area[(end.1 * rows) + end.0] != 0 { break; }
	}
	println!("A*-Algorithm Calc: {:.6?}", benchmark.elapsed());

	find_path("A*-Algorithm", end, &calculated_fields, rows)
}

#[derive(Default, Debug)]
struct Field {
	pub prev: Option<(usize, usize)>,
	pub processed: bool,
	pub cost: f64,
	pub estimate: u64,
	pub dist: usize,
}

/// This is a generic function to finally findthe path from the start to the end
///
/// This funciton works with 4-Neighborhood and 8-Neighborhood because it checks always for a value -1 of the current
///
/// # Arguments:
///
/// * `algorithm` - Name of the algorithm for logging
/// * `start` - Start position as a tuple
/// * `area` - Calculated area
/// * `rows` - Number of rows
/// * `cols` - Number of columns
/// * `eight` - Check for 8-Neighborhood even if it was calculated in a 4-Neighborhood
///
/// # Result:
///
/// A Vector of tuples with usize values identifying the x/y position in the area
fn find_path(algorithm: &str, end: (usize, usize), area: &[Option<Field>], rows: &usize) -> Vec<(usize, usize)> {
	let benchmark = Instant::now();

	let mut last = end;
	let mut result = vec![];
	loop {
		result.push(last);
		let pos = (last.1 * rows) + last.0;
		match area[pos].as_ref() {
			Some(field) => {
				if let Some(prev) = field.prev {
					last = prev;
				} else {
					break;
				}
			},
			None => break,
		}
	}
	println!("{} Path-Calculation: {:.6?}", algorithm, benchmark.elapsed());
	println!("{} Path length: {}", algorithm, result.len());

	result
}


