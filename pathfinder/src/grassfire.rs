pub(crate) use std::time::Instant;

/// Use the Grassfire algorithm to calculate the optimal way fro the start to the end.
///
/// Version 1: go through the whole matrix multiple times (rows x cols times)
///
/// # Arguments
///
/// * `area` - The play field as a one-dimensional vector
/// * `rows` - number of rows
/// * `cols` - number of cols
/// * `start` - start position (row, col)
/// * `end` - end position (row, col)
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

	find_path(start, area, rows, cols)
}

/// Use the Grassfire algorithm to calculate the optimal way fro the start to the end.
///
/// Version 2: Hold the last processed fields in memory to not repeatedly go through the whole matrix
///
/// # Arguments
///
/// * `area` - The play field as a one-dimensional vector
/// * `rows` - number of rows
/// * `cols` - number of cols
/// * `start` - start position (row, col)
/// * `end` - end position (row, col)
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
				let row_next = match field.0 + 1 { row if &row < rows => Some(field.0 + 1), _ => None };
				let row_prev = match field.0     { row if row > 0     => Some(field.0 - 1), _ => None };
				let col_next = match field.1 + 1 { col if &col < cols => Some(field.1 + 1), _ => None };
				let col_prev = match field.1     { col if col > 0     => Some(field.1 - 1), _ => None };
				let value = area[(field.1 * rows) + field.0] + 1;

				if let Some(row) = row_next {
					let check_field = (row, field.1);
					if area[(field.1 * rows) + row] == 0 {
						next_fields.push(check_field);
						area[(field.1 * rows) + row] = value;
						changed = true;
					}
				};
				if let Some(row) = row_prev {
					let check_field = (row, field.1);
					if area[(field.1 * rows) + row] == 0 {
						next_fields.push(check_field);
						area[(field.1 * rows) + row] = value;
						changed = true;
					}
				};

				if let Some(col) = col_next {
					let check_field = (field.0, col);
					if area[(col * rows) + field.0] == 0 {
						next_fields.push(check_field);
						area[(col * rows) + field.0] = value;
						changed = true;
					}
				};
				if let Some(col) = col_prev {
					let check_field = (field.0, col);
					if area[(col * rows) + field.0] == 0 {
						next_fields.push(check_field);
						area[(col * rows) + field.0] = value;
						changed = true;
					}
				};
			});
		last_fields = next_fields.clone();

		// This is not working in the while loop
		if !changed { println!("Grassfire-V2 Calc: No conneciton possible"); break; }
		if area[(start.1 * rows) + start.0] != 0 || last_fields.is_empty() { break; }
	}
	println!("Grassfire-V2 Calc: {:.6?}", benchmark.elapsed());

	find_path(start, area, rows, cols)
}

/// This is a generic function to finally findthe path from the start to the end
///
/// # Arguments:
///
/// * `start` - Start position as a tuple
/// * `area` - Calculated area
/// * `rows` - Number of rows
/// * `cols` - Number of columns
///
/// # Result:
///
/// A Vector of tuples with usize values identifying the x/y position in the area
fn find_path(start: (usize, usize), area: &mut [u64], rows: &usize, cols: &usize) -> Vec<(usize, usize)> {
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

		// Check for a lower value in the same column
		let next_value = cell_value - 1;
		if row_num < *rows && area[ (col_num * rows) + row_num + 1 ] == next_value {
			row_num += 1;
		} else if row_num > 0 && area[ (col_num * rows) + row_num - 1 ] == next_value {
			row_num -= 1;
		}
		// Check for a lower value in the same row
		else if col_num < *cols && area[ ((col_num + 1) * rows) + row_num ] == next_value {
			col_num += 1;
		} else if col_num > 0 && area[ ((col_num - 1) * rows) + row_num ] == next_value {
			col_num -= 1;
		}
		// If there is no lower number found, there is no path...
		else { break; }
	}
	println!("Path-Calculation: {:.6?}", benchmark.elapsed());

	result
}

