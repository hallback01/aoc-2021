use std::collections::HashMap;

fn construct_pairs(input: &String) -> HashMap<String, u64> {
	
	let mut map = HashMap::new();

	for n in 0..input.len() - 1 {

		let first: u8 = input.as_bytes()[n];
		let second: u8 = input.as_bytes()[n+1];

		let pair = String::from(format!("{}{}", first as char, second as char));

		match map.get_mut(&pair) {

			Some(amount) => {
				*amount += 1;
			},

			None => {
				map.insert(pair, 1);
			}
		}
	}

	return map;
}

fn expand_pairs(mut pairs: HashMap<String, u64>, insertions:  &HashMap<String, String>) -> HashMap<String, u64> {

	let mut to_add = Vec::new();

	for (pair, value) in insertions {

		match pairs.get_mut(pair) {

			Some(amount) => {

				if *amount > 0 {

					//create two new pairs
					let pair1 = (String::from(format!("{}{}", &pair[0..1], value)), amount.clone());
					let pair2 = (String::from(format!("{}{}", value, &pair[1..2])), amount.clone());

					to_add.push(pair1);
					to_add.push(pair2);

					*amount = 0;
				}
			},

			None => {}
		}
	}

	for (key, amount) in to_add {
		match pairs.get_mut(&key) {
			Some(amount1) => {

				*amount1 += amount;

			}

			None => {
				pairs.insert(key.clone(), amount);
			}
		}
	}

	return pairs;
}

fn get_low_high(pairs: &HashMap<String, u64>) -> (u64, u64) {

	let mut map: HashMap<String, u64> = HashMap::new();

	for (pair, amount) in pairs {

		let first_char = String::from(&pair[0..1]);
		let second_char = String::from(&pair[1..2]);

		match map.get_mut(&first_char) {

			Some(value) => {
				*value += *amount as u64;
			}
			None => {
				map.insert(first_char.clone(), *amount as u64);
			}
		}

		match map.get_mut(&second_char) {

			Some(value) => {
				*value += *amount as u64;
			}
			None => {
				map.insert(second_char.clone(), *amount as u64);
			}
		}
	}

	let mut largest = 0;
	let mut smallest =  std::u64::MAX;

	for (_, amount) in map {
		
		let new_amount = amount / 2 + amount % 2;

		if new_amount > largest {
			largest = new_amount;
		}

		if new_amount < smallest {
			smallest = new_amount;
		}

	}

	return (smallest, largest);

}

fn part1(input: &String, insertions: &HashMap<String, String>) {

	let mut pairs = construct_pairs(&input);

	for _ in 0..10 {
		pairs = expand_pairs(pairs, &insertions);
	}

	let (lowest, highest) = get_low_high(&pairs);

	println!("Part 1 Output: {}", highest - lowest);

}

fn part2(input: &String, insertions: &HashMap<String, String>) {

	let mut pairs = construct_pairs(&input);

	for _ in 0..40 {
		pairs = expand_pairs(pairs, &insertions);
	}

	let (lowest, highest) = get_low_high(&pairs);

	println!("Part 2 Output: {}", highest - lowest);

}

fn main() {

	let input = std::fs::read_to_string("input.txt").unwrap();

	let mut input_elements = String::from("");

	let mut insertions: HashMap<String, String> = HashMap::new();

	for line in input.split("\n") {

		if line.contains("->") {
			let split_line: Vec<&str> = line.split(" -> ").collect();
			insertions.insert(String::from(split_line[0]), String::from(split_line[1]));
		} else if !line.eq("") {
			input_elements.push_str(line);
		}
	}
	
	part1(&input_elements, &insertions);
	part2(&input_elements, &insertions);
}