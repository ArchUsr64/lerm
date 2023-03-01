/// Reads pbm files from the given file_path
/// Return a tuple with following parameters of the image `(width, height, vector with pixel values)`
/// Comments are also not supported
pub fn parse_pbm(file_data: &str) -> Option<(u32, u32, Vec<u8>)> {
	let mut metadata = file_data.chars();
	if metadata.next_chunk::<2>().ok()?.iter().collect::<String>() != *"P1" {
		return None;
	}
	metadata.next()?;
	let mut dimensions = Vec::<u32>::with_capacity(2);
	let mut pixel_data = Vec::new();
	for (line_number, line) in file_data.lines().enumerate() {
		if line_number == 0 || line.trim_start().starts_with('#') {
			continue;
		}
		if dimensions.is_empty() {
			dimensions = line
				.split_whitespace()
				.map(|i| i.parse().unwrap())
				.collect();
			assert!(dimensions.len() == 2 && dimensions[0] > 0 && dimensions[1] > 0);
			pixel_data.reserve(dimensions.iter().product::<u32>() as usize);
		} else {
			let mut buffer = line
				.chars()
				.map(|i| i.to_digit(2).unwrap() as u8)
				.map(|i| if i == 1 { 0 } else { 255 })
				.collect();
			pixel_data.append(&mut buffer);
		}
	}
	assert!(dimensions.iter().product::<u32>() == pixel_data.len() as u32);
	let (width, height) = (dimensions[0], dimensions[1]);
	// Flip the image vertically
	let mut pixel_data_flipped = Vec::with_capacity(pixel_data.len());
	for i in (0..height).rev() {
		for j in 0..width {
			pixel_data_flipped.push(pixel_data[(width * i + j) as usize]);
		}
	}
	Some((width, height, pixel_data_flipped))
}
