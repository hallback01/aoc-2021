fn convert_to_binary(input: String) -> String {

	let mut output = String::new();
	for character in input.chars() {
		if character == '0' {
			output.push_str("0000");
		} else if character == '1' {
			output.push_str("0001");
		} else if character == '2' {
			output.push_str("0010");
		} else if character == '3' {
			output.push_str("0011");
		} else if character == '4' {
			output.push_str("0100");
		} else if character == '5' {
			output.push_str("0101");
		} else if character == '6' {
			output.push_str("0110");
		} else if character == '7' {
			output.push_str("0111");
		} else if character == '8' {
			output.push_str("1000");
		} else if character == '9' {
			output.push_str("1001");
		} else if character == 'A' {
			output.push_str("1010");
		} else if character == 'B' {
			output.push_str("1011");
		} else if character == 'C' {
			output.push_str("1100");
		} else if character == 'D' {
			output.push_str("1101");
		} else if character == 'E' {
			output.push_str("1110");
		} else if character == 'F' {
			output.push_str("1111");
		}
	}
	return output;
}

fn parse(input: &String, position: usize, input_score: usize) -> (usize, usize, usize) {

	let mut read_position = position;
	let mut score = input_score; 

	let mut value = 0;

	let version = usize::from_str_radix(&input[read_position..read_position+3], 2).unwrap(); read_position+=3;
	let packet_type = usize::from_str_radix(&input[read_position..read_position+3], 2).unwrap(); read_position+=3;

	score += version;

	match packet_type {

		4 => {

			let mut literal_number = String::new();

			loop {
				let prefix = usize::from_str_radix(&input[read_position..read_position+1], 2).unwrap(); read_position+=1;
				literal_number.push_str(&input[read_position..read_position+4]); read_position+=4;

				if prefix == 0 {
					break;
				}
			}

			let number = usize::from_str_radix(&literal_number, 2).unwrap();
			value = number;
		},

		_ => {

			let length_type_id = usize::from_str_radix(&input[read_position..read_position+1], 2).unwrap(); read_position+=1;

			let mut values: Vec<usize> = Vec::new();

			if length_type_id == 0 {

				let length = usize::from_str_radix(&input[read_position..read_position+15], 2).unwrap(); read_position+=15;

				let read_offset = read_position + length;

				while read_position != read_offset {
					let (new_pos, new_score, value) = parse(&input, read_position.clone(), score);
					score = new_score;
					read_position = new_pos;
					values.push(value);
				}


			} else {

				let length = usize::from_str_radix(&input[read_position..read_position+11], 2).unwrap(); read_position+=11;

				for _ in 0..length {
					let (new_pos, new_score, value) = parse(&input, read_position, score);
					score = new_score;
					read_position = new_pos;
					values.push(value);
				}

			}

			match packet_type {

				0 => {
					for sum in values {
						value += sum;
					}
				},

				1 => {
					value = 1;
					for product in values {
						value *= product;
					}
				},

				2 => {

					if let Some(min) = values.iter().min() {
						value = *min;
					}

				},

				3 => {
					if let Some(min) = values.iter().max() {
						value = *min;
					}
				},

				5 => {

					if values[0] > values[1] {
						value = 1;
					} else {
						value = 0;
					}

				},

				6 => {
					if values[0] < values[1] {
						value = 1;
					} else {
						value = 0;
					}
				},

				7 => {
					if values[0] == values[1] {
						value = 1;
					} else {
						value = 0;
					}
				},

				_ => {

				},

			}

		},

	}
	return (read_position, score, value);
}

fn part1(input: &String) {

	let (_, score, _) = parse(&input, 0, 0);

	println!("Part 1 Output: {}", score);

}

fn part2(input: &String) {

	let (_, _, value) = parse(&input, 0, 0);

	println!("Part 2 Output: {}", value);

}


fn main() {

	let input = std::fs::read_to_string("input.txt").unwrap();
	let binary_input = convert_to_binary(input);

	part1(&binary_input);
	part2(&binary_input);

}