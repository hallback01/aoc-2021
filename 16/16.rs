fn convert_to_binary(input: String) -> String {
	let mut output = String::new();
	for character in input.chars() {
		let mut binary = String::from(format!("{:b}", usize::from_str_radix(&String::from(character), 16).unwrap()));
		while binary.len() < 4 {
			binary.insert(0, '0');
		}
		output.push_str(&binary);
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
			value = usize::from_str_radix(&literal_number, 2).unwrap();
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
				0 => value = values.into_iter().sum(),
				1 => value = values.into_iter().product(),
				2 => value = values.into_iter().min().unwrap(),
				3 => value = values.into_iter().max().unwrap(),
				5 => value = (values[0] > values[1]) as usize,
				6 => value = (values[0] < values[1]) as usize,
				7 => value = (values[0] == values[1]) as usize,
				_ => {},
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
	let binary_input = convert_to_binary(std::fs::read_to_string("input.txt").unwrap());
	part1(&binary_input);
	part2(&binary_input);
}