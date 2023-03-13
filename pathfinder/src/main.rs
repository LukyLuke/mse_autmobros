use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 7 {
		panic!("Usage: {} ROWS COLS START_X START_Y END_X END_Y\n   ROWS, COLS: Size of the playfield\n   START_*: Position of the Robot\n   END_*: The Position to reach", args[0]);
	}

	let rows  = &args[1].parse::<usize>().unwrap_or_default();
	let cols  = &args[2].parse::<usize>().unwrap_or_default();
	let start = ( &args[3].parse::<usize>().unwrap_or_default() - 1, &args[4].parse::<usize>().unwrap_or_default() - 1 );
	let end   = ( &args[5].parse::<usize>().unwrap_or_default() - 1, &args[6].parse::<usize>().unwrap_or_default() - 1 );

	assert!(start.0 <= rows - 1, "Start-Position X is outside of the area");
	assert!(start.1 <= cols - 1, "Start-Position Y is outside of the area");
	assert!(end.0 <= rows - 1, "End-Position X is outside of the area");
	assert!(end.1 <= cols - 1, "End-Position Y is outside of the area");

	// The Play-Field is a one-dimensional vector where all columns are just in line
	// TODO: Add some obstacles/walls
	let area:Vec<u64> = vec![0; *rows * *cols];

	// Use Grasfire for the path: start and end position are inverted
	let path = grasfire(area, rows, cols, start, end);
	println!("Grasfire-Path: {:?}", path);
}

/// Use the Grasfire algorithm to calculate the optimal way fro the start to the end.
///
/// TODO: Optimize...
///
/// # Arguments
///
/// * `area` - The play field as a one-dimensional vector
/// * `rows` - number of rows
/// * `cols` - number of cols
/// * `start` - start position (row, col)
/// * `end` - end position (row, col)
fn grasfire(mut area: Vec<u64>, rows: &usize, cols: &usize, start:(usize, usize), end:(usize, usize)) -> Vec<(usize, usize)> {
	// Fill the start position with 1
	let len = area.len();
	let mut value: usize = 1;
	area[(end.1 * rows) + end.0] = value as u64;

	// run as long as we have not reached the start field
	// but maximum as long as every field should be filled
	while area[(start.1 * rows) + start.0] == 0 && value < len {
		value += 1;
		for col_num in 0..*cols {
			let col_start = col_num * rows;
			let col_end = col_start + rows;

			for row_num in 0..*rows {
				if area[col_start .. col_end][row_num] > 0 {
					if row_num > 0 && area[col_start .. col_end][row_num-1] == 0 { area[col_start .. col_end][row_num-1] = value as u64; }
					if row_num+1 < *rows && area[col_start .. col_end][row_num+1] == 0 { area[col_start .. col_end][row_num+1] = value as u64; }

					// Check upper column
					if col_start > 0 && area[col_start-rows .. col_start][row_num] == 0 {
						area[col_start-rows .. col_start][row_num] = value as u64;
					}

					// Check lower column
					if col_end+1 < len && area[col_end .. col_end+rows][row_num] == 0 {
						area[col_end .. col_end+rows][row_num] = value as u64;
					}
				}
			}
		}
	}

	// After calculation, find a path from start to the end
	let mut row_num = start.0;
	let mut col_num = start.1;
	let mut result = vec![];
	while value < len {
		result.push(( row_num + 1, col_num + 1 ));

		// If the field is not filled or we reached 1, we found no- or the path
		let cell_value = area[ (col_num * rows) + row_num ];
		if cell_value == 0 || cell_value == 1 {
			break;
		}

		// Check for a lower value in the same column
		if row_num < *rows && area[ (col_num * rows) + row_num + 1 ] < cell_value {
			row_num += 1;
		} else if row_num > 0 && area[ (col_num * rows) + row_num - 1 ] < cell_value {
			row_num -= 1;
		}
		// Check for a lower value in the same row
		else if col_num < *cols && area[ ((col_num + 1) * rows) + row_num ] < cell_value {
			col_num += 1;
		} else if col_num > 0 && area[ ((col_num - 1) * rows) + row_num ] < cell_value {
			col_num -= 1;
		}
		// If there is no lower number found, there is no path...
		else { break; }
	}
	result
}
