fn path_amount(start: &str, end: &str, connected_caves: &Vec<(&str, &str)>, part1: bool) -> u32 {

	let mut amount = 0;

	let mut path_list: Vec<String> = Vec::new();
	path_list.push(String::from(format!("{},{}", start, end)));

	let mut to_remove: Vec<usize> = Vec::new();
	let mut to_insert: Vec<String> = Vec::new();

	while path_list.len() > 0 {

		for (i, path) in &mut path_list.iter().enumerate() {

			let end_cave = path_get_end_cave(&path);

			if end_cave == String::from("end") {
				amount += 1;
				to_remove.push(i);
			} else {

				to_remove.push(i);

				let caves = get_connected_caves(&end_cave, connected_caves);

				for cave in caves {

					if part1 {

						let is_used = path.contains(&cave);
						let repeateable = is_repeatable(&cave);

						if !repeateable && !is_used {
							to_insert.push(String::from(format!("{},{}", path, cave)));
						} else if repeateable {
							to_insert.push(String::from(format!("{},{}", path, cave)));
						}

					} else {

						if can_visit_again(&path, &cave) {
							to_insert.push(String::from(format!("{},{}", path, cave)));
						}
					}
				}
			}
		}
	
		for pos in to_remove.iter().rev() {
			path_list.remove(*pos);
		}
		to_remove.clear();

		for new_path in &to_insert {
			path_list.push(new_path.to_string());
		}
		to_remove.clear();
		to_insert.clear();
	}

	return amount;

}

fn is_repeatable(string: &String) -> bool {

	let mut is_valid = true;

	for chr in string.chars() {
		if chr.is_lowercase() {
			is_valid = false;
			break;
		}
	}

	return is_valid;

}

fn get_connected_caves(end_cave: &String, connected_caves: &Vec<(&str, &str)>) -> Vec<String> {

	let mut return_vec: Vec<String> = Vec::new();

	for (start, end) in connected_caves {
		if end_cave == start {
			return_vec.push(end.to_string());
		}

		if end_cave == end {
			return_vec.push(start.to_string());
		}
	}

	return return_vec;

}

fn path_get_end_cave(path: &String) -> String {
	let path_collection: Vec<&str> = path.split(",").collect();
	return String::from(*path_collection.last().unwrap());
}

fn can_visit_again(path: &String, destination_node: &String) -> bool {

	if destination_node == "start" {
		return false;
	}

	//I.E IS UPPERCASE or END NODE
	if is_repeatable(destination_node) || destination_node == "end" {
		return true;
	}

	if !path.contains(destination_node) {
		return true;
	} else {

		//check highest occurence
		let mut highest_occurence = 0;

		for node in path.split(",") {
			let current_occurence = path.matches(node).count();
			if !is_repeatable(&String::from(node)) {
				if current_occurence > highest_occurence {
					highest_occurence = current_occurence;
				}
			}
		}
		
		if highest_occurence < 2 {
			return true;
		} else {
			return false;
		}

	}

}

fn part1(connected_caves: &Vec<(&str, &str)>) {

	let mut paths = 0;

	for (start, end) in connected_caves {

		if start == &"start" {
			paths += path_amount(start, end, &connected_caves, true);
		}

		if end == &"start" {
			paths += path_amount(end, start, &connected_caves, true);
		}

	}

	println!("Part 1 Output: {}", paths);

}

fn part2(connected_caves: &Vec<(&str, &str)>) {

	let mut paths = 0;

	for (start, end) in connected_caves {

		if start == &"start" {
			paths += path_amount(start, end, &connected_caves, false);
		}

		if end == &"start" {
			paths += path_amount(end, start, &connected_caves, false);
		}

	}

	println!("Part 2 Output: {}", paths);

}

fn main() {

	match std::fs::read_to_string("input.txt") {

		Ok(input) => {

			let mut connected_caves = Vec::new();

			for line in input.split("\n") {
				let components: Vec<&str> = line.split("-").collect();
				connected_caves.push((components[0], components[1]));
			}

			part1(&connected_caves);
			part2(&connected_caves);

		},

		Err(e) => {
			println!("Error reading file: {}", e);
		},
		
	}

}