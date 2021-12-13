use std::collections::HashSet;

fn part1(dots_ref: &HashSet<(i32, i32)>, first_fold: &(&str, i32)) {
	let mut dots = dots_ref.clone();
	dots = fold(dots, &first_fold);

	println!("Part 1 Output: {}", dots.len());
}

fn part2(dots_ref: &HashSet<(i32, i32)>, folds: &Vec<(&str, i32)>) {

	let mut dots = dots_ref.clone();

	for fold_inst in folds {
		dots = fold(dots, &fold_inst);
	}

	println!("Part 2 Output: {}", print_letters(&dots));

}

fn print_letters(dots: &HashSet<(i32, i32)>) -> String {

	let mut width = 0;
	let mut height = 0;

	let mut ret_string = String::from("\n");

	for (x,y) in dots {
		if x > &width {
			width = *x + 1;
		}
		if y > &height {
			height = *y + 1;
		}
	}

	let mut paper: Vec<String> = Vec::new();

	for i in 0..width*height {
		let x = i % width;
		let y = i / width;

		if dots.contains(&(x,y)) {
			paper.push("#".to_string());
		} else {
			paper.push(".".to_string());
		}
	}
	
	for y in 0..height {

		for x in 0..width {
			ret_string.push_str(&paper[(width * y + x) as usize]);
		}
		ret_string.push_str("\n");
	}

	return ret_string;

}

fn fold(dots: HashSet<(i32,i32)>, instruction: &(&str, i32)) -> HashSet<(i32, i32)> {

	let mut new_dots = HashSet::new();

	match &instruction.0 {

		&"x" => {

			for (x,y) in dots {

				if x > instruction.1 {
					let dist_from_fold = x - instruction.1;

					new_dots.insert((instruction.1 - dist_from_fold, y));

				} else if x < instruction.1 {
					new_dots.insert((x,y));
				}

			}

		},

		&"y" => {

			for (x, y) in dots {

				if y > instruction.1 {
					
					let dist_from_fold = y - instruction.1;

					new_dots.insert((x, instruction.1 - dist_from_fold));

				} else if y < instruction.1 {
					new_dots.insert((x,y));
				}
			}

		},

		_ => {

		},

	}

	return new_dots;

}

fn main() {

	match std::fs::read_to_string("input.txt") {

		Ok(input) => {

			let mut load_coordinate = true;

			let mut coordinates = HashSet::new();
			let mut instructions = Vec::new();

			for line in input.split("\n") {

				if load_coordinate {

					if line.len() == 0 {
						load_coordinate = false;
					} else {
						let dot = line.split(",").collect::<Vec<&str>>();
						coordinates.insert((dot[0].parse::<i32>().unwrap(), dot[1].parse::<i32>().unwrap()));
					}
				} else {
					let reduced_line = &line[11..line.len()];
					let instruction = reduced_line.split("=").collect::<Vec<&str>>();
					instructions.push((instruction[0], instruction[1].parse::<i32>().unwrap()));
				}

			}

			part1(&coordinates, &instructions[0]);
			part2(&coordinates, &instructions);

		},

		Err(_e) => {
			println!("Could not read file.");
		},

	}

}