use crate::Instant;

/// Use the Grassfire algorithm to calculate the optimal way fro the start to the end.
///
/// Version 1: go through the whole matrix multiple times (rows x cols times)
///            use the 4-Neighborhood
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
pub fn v1(area: &mut [u64], rows: &usize, cols: &usize, start:(usize, usize), end:(usize, usize)) -> Vec<(usize, usize)> {
	let benchmark = Instant::now();

	// Fill the start position with 1
	area[(end.1 * rows) + end.0] = 1;

	// Run as long as we have not reached the start field
	while area[(start.1 * rows) + start.0] == 0 {
		let mut changed = false;
		for row in 0..*rows {
			for col in 0..*cols {
				let pos = col * rows;
				let field_val = area[pos + row];
				if field_val < u64::MAX && field_val > 0 {
					// Check left and right row
					if row > 0 && area[pos + row - 1] == 0 {
						area[pos + row - 1] = field_val + 1;
						changed = true;
					}
					if row + 1 < *rows && area[pos + row + 1] == 0 {
						area[pos + row + 1] = field_val + 1;
						changed = true;
					}

					// Check upper and lower column
					if col > 0 && area[pos - rows + row] == 0 {
						area[pos - rows + row] = field_val + 1;
						changed = true;
					}
					if col + 1 < *cols && area[pos + rows + row] == 0 {
						area[pos + rows + row] = field_val + 1;
						changed = true;
					}
				}
			}
		}
		if !changed { println!("Grassfire-V1 Calc: No conneciton possible"); break; }
	}
	println!("Grassfire-V1 Calc: {:.6?}", benchmark.elapsed());

	find_path("Grassfire-V1", start, area, rows, cols, false)
}

/// Use the Grassfire algorithm to calculate the optimal way fro the start to the end.
///
/// Version 2: Hold the last processed fields in memory to not repeatedly go through the whole matrix
///            use the 4-Neighborhood
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
pub fn v2(area: &mut [u64], rows: &usize, cols: &usize, start:(usize, usize), end:(usize, usize)) -> Vec<(usize, usize)> {
	let benchmark = Instant::now();

	// Fill the start position with 1
	area[(end.1 * rows) + end.0] = 1;

	// Optimization: This vector holds the fields which has been processed in the last round
	// Based on these the neighbours can be calculated, starting on the end
	let mut last_fields: Vec<(usize, usize)> = vec![end];

	// run as long as we have not reached the start field
	// or as long as we have neighbours to calculate
	loop {
		let mut changed = false;

		// Calculate the fields to calculate as next
		let mut next_fields: Vec<(usize, usize)> = vec![];
		last_fields.iter()
			.for_each(|field| {
				let row_next = match field.0 + 1 { row if &row < rows => Some((field.0 + 1, field.1)),     _ => None };
				let row_prev = match field.0     { row if row > 0     => Some((field.0 - 1, field.1)),     _ => None };
				let col_next = match field.1 + 1 { col if &col < cols => Some((field.0,     field.1 + 1)), _ => None };
				let col_prev = match field.1     { col if col > 0     => Some((field.0,     field.1 - 1)), _ => None };

				let value = area[(field.1 * rows) + field.0] + 1;
				let mut check_field = |field: Option<(usize, usize)>| {
					if let Some(fld) = field {
						if area[(fld.1 * rows) + fld.0] == 0 {
							next_fields.push(fld);
							area[(fld.1 * rows) + fld.0] = value;
							changed = true;
						}
					};
				};
				check_field(row_next);
				check_field(row_prev);
				check_field(col_next);
				check_field(col_prev);
			});
		last_fields = next_fields.clone();

		// This is not working in the while loop
		if !changed { println!("Grassfire-V2 Calc: No conneciton possible"); break; }
		if area[(start.1 * rows) + start.0] != 0 || last_fields.is_empty() { break; }
	}
	println!("Grassfire-V2 Calc: {:.6?}", benchmark.elapsed());

	find_path("Grassfire-V2", start, area, rows, cols, false)
}

/// Use the Grassfire algorithm to calculate the optimal way fro the start to the end.
///
/// Version 3: Hold the last processed fields in memory, use the 8-Neighborhood
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
pub fn v3(area: &mut [u64], rows: &usize, cols: &usize, start:(usize, usize), end:(usize, usize)) -> Vec<(usize, usize)> {
	let benchmark = Instant::now();

	// Fill the start position with 1
	area[(end.1 * rows) + end.0] = 1;

	// Optimization: This vector holds the fields which has been processed in the last round
	// Based on these the neighbours can be calculated, starting on the end
	let mut last_fields: Vec<(usize, usize)> = vec![end];

	// run as long as we have not reached the start field
	// or as long as we have neighbours to calculate
	loop {
		let mut changed = false;

		// Calculate the fields to calculate as next
		let mut next_fields: Vec<(usize, usize)> = vec![];
		last_fields.iter()
			.for_each(|field| {
				let row_next = match field.0 + 1 { row if &row < rows => Some((field.0 + 1, field.1)),     _ => None };
				let row_prev = match field.0     { row if row > 0     => Some((field.0 - 1, field.1)),     _ => None };
				let col_next = match field.1 + 1 { col if &col < cols => Some((field.0,     field.1 + 1)), _ => None };
				let col_prev = match field.1     { col if col > 0     => Some((field.0,     field.1 - 1)), _ => None };
				let row_next_down = if row_next.is_some() && col_next.is_some() { Some(( field.0 + 1, field.1 + 1 )) } else { None };
				let row_next_up =   if row_prev.is_some() && col_next.is_some() { Some(( field.0 - 1, field.1 + 1 )) } else { None };
				let row_prev_down = if row_next.is_some() && col_prev.is_some() { Some(( field.0 + 1, field.1 - 1 )) } else { None };
				let row_prev_up =   if row_prev.is_some() && col_prev.is_some() { Some(( field.0 - 1, field.1 - 1 )) } else { None };

				let value = area[(field.1 * rows) + field.0] + 1;
				let mut check_field = |field: Option<(usize, usize)>| {
					if let Some(fld) = field {
						if area[(fld.1 * rows) + fld.0] == 0 {
							next_fields.push(fld);
							area[(fld.1 * rows) + fld.0] = value;
							changed = true;
						}
					};
				};
				check_field(row_next_down);
				check_field(row_next_up);
				check_field(row_prev_down);
				check_field(row_prev_up);
				check_field(row_next);
				check_field(row_prev);
				check_field(col_next);
				check_field(col_prev);
			});
		last_fields = next_fields.clone();

		// This is not working in the while loop
		if !changed { println!("Grassfire-V3 Calc: No conneciton possible"); break; }
		if area[(start.1 * rows) + start.0] != 0 || last_fields.is_empty() { break; }
	}
	println!("Grassfire-V3 Calc: {:.6?}", benchmark.elapsed());

	find_path("Grassfire-V3", start, area, rows, cols, false)
}


/// Use the Grassfire algorithm to calculate the optimal way fro the start to the end.
///
/// Version 4: Hold the last processed fields in memory to not repeatedly go through the whole matrix
///            use the 4-Neighborhood for calculation but for the path finding the 8-Neighborhood
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
pub fn v4(area: &mut [u64], rows: &usize, cols: &usize, start:(usize, usize), end:(usize, usize)) -> Vec<(usize, usize)> {
	let benchmark = Instant::now();

	// Fill the start position with 1
	area[(end.1 * rows) + end.0] = 1;

	// Optimization: This vector holds the fields which has been processed in the last round
	// Based on these the neighbours can be calculated, starting on the end
	let mut last_fields: Vec<(usize, usize)> = vec![end];

	// run as long as we have not reached the start field
	// or as long as we have neighbours to calculate
	loop {
		let mut changed = false;

		// Calculate the fields to calculate as next
		let mut next_fields: Vec<(usize, usize)> = vec![];
		last_fields.iter()
			.for_each(|field| {
				let row_next = match field.0 + 1 { row if &row < rows => Some((field.0 + 1, field.1)),     _ => None };
				let row_prev = match field.0     { row if row > 0     => Some((field.0 - 1, field.1)),     _ => None };
				let col_next = match field.1 + 1 { col if &col < cols => Some((field.0,     field.1 + 1)), _ => None };
				let col_prev = match field.1     { col if col > 0     => Some((field.0,     field.1 - 1)), _ => None };

				let value = area[(field.1 * rows) + field.0] + 1;
				let mut check_field = |field: Option<(usize, usize)>| {
					if let Some(fld) = field {
						if area[(fld.1 * rows) + fld.0] == 0 {
							next_fields.push(fld);
							area[(fld.1 * rows) + fld.0] = value;
							changed = true;
						}
					};
				};
				check_field(row_next);
				check_field(row_prev);
				check_field(col_next);
				check_field(col_prev);
			});
		last_fields = next_fields.clone();

		// This is not working in the while loop
		if !changed { println!("Grassfire-V2 Calc: No conneciton possible"); break; }
		if area[(start.1 * rows) + start.0] != 0 || last_fields.is_empty() { break; }
	}
	println!("Grassfire-V4 Calc: {:.6?}", benchmark.elapsed());

	find_path("Grassfire-V4", start, area, rows, cols, true)
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
pub fn find_path(algorithm: &str, start: (usize, usize), area: &mut [u64], rows: &usize, cols: &usize, eight: bool) -> Vec<(usize, usize)> {
	let benchmark = Instant::now();

	let mut row_num = start.0;
	let mut col_num = start.1;
	let mut result = vec![];

	loop {
		result.push((row_num, col_num));

		// Break if the field:
		//   was never calculated (0)
		//   or the end is reached (1)
		//   or we are in a obstacle (MAX)
		let cell_value = area[ (col_num * rows) + row_num ];
		if cell_value == 0 || cell_value == 1 || cell_value == u64::MAX {
			break;
		}

		// Get all eight neighbours of the current field
		let row_next = match row_num + 1 { row if &row < rows => Some(( row_num + 1, col_num )),     _ => None };
		let row_prev = match row_num     { row if row > 0     => Some(( row_num - 1, col_num )),     _ => None };
		let col_next = match col_num + 1 { col if &col < cols => Some(( row_num,     col_num + 1 )), _ => None };
		let col_prev = match col_num     { col if col > 0     => Some(( row_num,     col_num - 1 )), _ => None };
		let row_next_down = if row_next.is_some() && col_next.is_some() { Some(( row_num + 1, col_num + 1 )) } else { None };
		let row_next_up =   if row_prev.is_some() && col_next.is_some() { Some(( row_num - 1, col_num + 1 )) } else { None };
		let row_prev_down = if row_next.is_some() && col_prev.is_some() { Some(( row_num + 1, col_num - 1 )) } else { None };
		let row_prev_up =   if row_prev.is_some() && col_prev.is_some() { Some(( row_num - 1, col_num - 1 )) } else { None };

		// Closure to check a value for a lower value than an other one
		let mut check_neighbor = |field: Option<(usize, usize)>| {
			if let Some(fld) = field {
				let fld_val = area[(fld.1 * rows) + fld.0];
				if fld_val > 0 && fld_val < cell_value {
					row_num = fld.0;
					col_num = fld.1;
					return true;
				}
			};
			false
		};

		match eight {
			// Try the edge-neighbors before the direct connected (up/down/left/right)
			true => if !check_neighbor(row_prev_up) &&
				!check_neighbor(row_prev_down) &&
				!check_neighbor(row_next_up) &&
				!check_neighbor(row_next_down) &&
				!check_neighbor(col_prev) &&
				!check_neighbor(col_next) &&
				!check_neighbor(row_prev) &&
				!check_neighbor(row_next) {
				break;
			},

			// Try the direct connected neighbors (up/down/left/right) before the edges
			false => if !check_neighbor(col_prev) &&
			 !check_neighbor(col_next) &&
			 !check_neighbor(row_prev) &&
			 !check_neighbor(row_next) &&
			 !check_neighbor(row_prev_up) &&
			 !check_neighbor(row_prev_down) &&
			 !check_neighbor(row_next_up) &&
			 !check_neighbor(row_next_down) {
				break;
			}
		}
	}
	println!("{} Path-Calculation: {:.6?}", algorithm, benchmark.elapsed());
	println!("{} Path length: {}", algorithm, result.len());

	result
}

