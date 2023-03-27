pub use std::time::Instant;

use std::env;
use image::{ImageBuffer, ImageError, RgbImage};
use rand::distributions::{Distribution, Uniform};

mod grassfire;
mod a_star;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 7 {
		panic!("Usage: {} ROWS COLS START_ROW START_COL END_ROW END_COL [OBSTACLES]\n   ROWS, COLS: Size of the playfield\n   START_*: Position of the Robot\n   END_*: The Position to reach\n   OBSTACLES: (100) Number of walls/obstacles", args[0]);
	}

	let rows  = &args[1].parse::<usize>().unwrap_or_default();
	let cols  = &args[2].parse::<usize>().unwrap_or_default();
	let start = ( &args[3].parse::<usize>().unwrap_or_default() - 1, &args[4].parse::<usize>().unwrap_or_default() - 1 );
	let end   = ( &args[5].parse::<usize>().unwrap_or_default() - 1, &args[6].parse::<usize>().unwrap_or_default() - 1 );
	let obstacles = if args.len() >= 8 { args[7].parse::<usize>().unwrap_or(100) } else { 100 };

	assert!(start.0 < *rows, "Start-Position X {} is outside of the area {}", start.0, *rows);
	assert!(start.1 < *cols, "Start-Position Y {} is outside of the area {}", start.1, *rows);
	assert!(end.0   < *rows, "End-Position X {} is outside of the area {}", end.0, *rows);
	assert!(end.1   < *cols, "End-Position Y {} is outside of the area {}", end.1, *rows);

	// The Play-Field is a one-dimensional vector where all columns are just in line
	let px_dim = if *rows > 200 || *cols > 200 { 1 } else { 5 };
	let max_size = (rows / 10, cols / 10);
	let mut area = create_area(rows, cols, &obstacles, &max_size);
	let mut count = 0;
	while area[(start.1 * rows) + start.0] == u64::MAX || area[(end.1 * rows) + end.0] == u64::MAX {
		count += 1;
		println!("Area {} invalid: Start or End is inside an obstacle.", count);
		if count > 100 {
			println!("ERROR: Unable to create a valid area. Change the values...");
			return;
		}
		area = create_area(rows, cols, &obstacles, &max_size);
	}

	println!("Field Size: {}x{}", rows, cols);
	println!("Obstacles:  {} max {}x{}\n", obstacles, max_size.0, max_size.1);

	// Use Grassfire for the path: 4-Neighborhood, Not optimized
	{
		#[allow(clippy::redundant_clone)]
		let mut field = area.clone();
		let path = grassfire::v1(&mut field, rows, cols, start, end);
		let _ = export_image("grassfire_v1", &field, (rows, cols), start, end, &path, px_dim);
	}

	// Use Grassfire for the path: 4-Neighborhood, Optimized
	{
		#[allow(clippy::redundant_clone)]
		let mut field = area.clone();
		let path = grassfire::v2(&mut field, rows, cols, start, end);
		let _ = export_image("grassfire_v2", &field, (rows, cols), start, end, &path, px_dim);
	}

	// Use Grassfire for the path: 8-Neighborhood, Based on v2
	{
		#[allow(clippy::redundant_clone)]
		let mut field = area.clone();
		let path = grassfire::v3(&mut field, rows, cols, start, end);
		let _ = export_image("grassfire_v3", &field, (rows, cols), start, end, &path, px_dim);
	}

	// Use Grassfire for the path: 4-Neighborhood for calculation fut 8-Neighborhood for the path
	{
		#[allow(clippy::redundant_clone)]
		let mut field = area.clone();
		let path = grassfire::v4(&mut field, rows, cols, start, end);
		let _ = export_image("grassfire_v4", &field, (rows, cols), start, end, &path, px_dim);
	}

	// Use A* for the path
	{
		#[allow(clippy::redundant_clone)]
		let mut field = area.clone();
		let path = a_star::calculate(&mut field, rows, cols, start, end);
		let _ = export_image("a_star_calculate", &field, (rows, cols), start, end, &path, px_dim);
	}
}

/// Creates the area and adds random created obstacles
///
/// The Area is a 1-Dimensonal vector with rows*cols fields
/// A new row begins on each `rows` values.
/// This means the second column starts at position `1 * rows`,
/// the third on `2 * rows`, ...
///
/// Obstacles have the value `u64::MAX = 18446744073709551615`
/// all other fields are initialized with `0`
///
/// # Arguments:
///
/// * `rows` - Number of rows for the area
/// * `cols` - Number of columns for the area
/// * `obstacles` - Number of obstacles
/// * `max_size` - A Tuple which holds the max size for an obstacle
///
/// # Result
///
/// A vector which represents the whole area including obstacles
fn create_area(rows: &usize, cols: &usize, obstacles: &usize, max_size: &(usize, usize)) -> Vec<u64> {
	let benchmark = Instant::now();
	let max = *rows * *cols;
	let mut area:Vec<u64> = vec![0; max];

	if max_size.0 > 0 && max_size.1 > 0 {
		// Let the noise the max value
		let wall = u64::MAX;

		// Create some random noise based on the given values
		let range_x = Uniform::from(0..*rows);
		let range_y = Uniform::from(0..*cols);
		let size_x = Uniform::from(0..max_size.0);
		let size_y = Uniform::from(0..max_size.1);
		let mut rng = rand::thread_rng();
		for _ in 0..*obstacles {
			let x = range_x.sample(&mut rng);
			let y = range_y.sample(&mut rng);
			let sx = size_x.sample(&mut rng);
			let sy = size_y.sample(&mut rng);

			for row in x..(x + sx) {
				if row+1 > *rows {
					break;
				}
				for col in y..(y + sy) {
					if col+1 > *cols {
						break;
					}
					area[(col * rows) + row] = wall;
				}
			}
		}
	}
	println!("Create Area: {:.6?}", benchmark.elapsed());

	area
}

/// Creates an image of the area with the path, the start and end position.
///
/// TODO: Make this faster somehow...
///
/// # Arguments:
///
/// * `algorith` - Name of the algorithm used for the area and path - used for the filename
/// * `area` - The area processed by the algorithm - A one dimensional list where on each rows entries a new column begins
/// * `area_size` - Tuple represents the rows and columns of the area
/// * `start` - Tuple with the start point (row, col)
/// * `end` - Tuple with the end point (row, col)
/// * `path` - List of tuples where the robot should drive on
/// * `field_size` - Size of one field in px
///
/// # Result:
///
/// Error from the image creation
fn export_image(algorithm: &str, area: &[u64], area_size: (&usize, &usize), start: (usize, usize), end: (usize, usize), path: &[(usize, usize)], field_size: usize) -> Result<(), ImageError> {
	let mut img: RgbImage = ImageBuffer::new((area_size.0 * field_size) as u32, (area_size.1 * field_size) as u32);
	let max_value = area.iter()
		.filter(|val| val < &&u64::MAX)
		.max()
		.unwrap_or(&1);

	// Fill the area
	for row in 0..*area_size.0 {
		for col in 0..*area_size.1 {
			let val = area[(col * area_size.0) + row];
			let color = match val {
				u64::MAX => [0, 0, 80],
				0 => [255, 255, 255],
				_ => {
					let red = (val * 255 / max_value) as u8;
					let blue = 0;
					let green = 28;
					[red, blue, green]
				}
			};

			let c_row = row * field_size;
			let c_col = col * field_size;
			for x in c_row..(c_row + field_size) {
				for y in c_col..(c_col + field_size) {
					let px = img.get_pixel_mut(x as u32, y as u32);
					*px = image::Rgb(color);
				}
			}
		}
	}

	// Draw the path
	for p in path {
		let color = [192, 243, 173];
		let c_row = (p.0) * field_size;
		let c_col = (p.1) * field_size;
		for x in c_row..(c_row + field_size) {
			for y in c_col..(c_col + field_size) {
				let px = img.get_pixel_mut(x as u32, y as u32);
				*px = image::Rgb(color);
			}
		}
	}

	// Draw start
	let color = [0, 221, 255];
	let c_row = (start.0) * field_size;
	let c_col = (start.1) * field_size;
	for x in c_row..(c_row + field_size) {
		for y in c_col..(c_col + field_size) {
			let px = img.get_pixel_mut(x as u32, y as u32);
			*px = image::Rgb(color);
		}
	}

	// Draw the end
	let color = [0, 255, 47];
	let c_row = (end.0) * field_size;
	let c_col = (end.1) * field_size;
	for x in c_row..(c_row + field_size) {
		for y in c_col..(c_col + field_size) {
			let px = img.get_pixel_mut(x as u32, y as u32);
			*px = image::Rgb(color);
		}
	}

	let file_name = format!("{}.png", algorithm);
	img.save(file_name.clone())?;
	println!("Saved: {}\n", file_name);
	Ok(())
}
