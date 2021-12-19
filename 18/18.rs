fn add_lines(line1: &String, line2: &String) -> String {
	return reduce(&format!("[{},{}]", line1, line2));
}

fn split(line: &String) -> Option<(String, usize)> {

	let mut new_string = line.clone();
	for (i, _) in line.chars().enumerate() {
		if i < line.len() - 1 {
			if new_string.as_bytes()[i] >= '0' as u8 && new_string.as_bytes()[i] <= '9' as u8 && new_string.as_bytes()[i+1] >= '0' as u8 && new_string.as_bytes()[i+1] <= '9' as u8 {
				let number: f32 = line[i..i+2].parse().unwrap();
				let right_number = (number / 2.0).ceil() as i32;
				let left_number = (number / 2.0).floor() as i32;
				new_string.replace_range(i..i+2, "");
				let insert = format!("[{},{}]", left_number, right_number);
				new_string.insert_str(i, &insert[0..insert.len()]);
				return Some((new_string, i));
			}
		}
	}
	return None;
}

fn contains_numeral(string: &String) -> Option<(usize, usize)> {
	for (i, character) in string.chars().enumerate() {
		if character >= '0' && character <= '9' {
			let mut range = 1;
			if string.as_bytes()[i+1] >= '0' as u8 && string.as_bytes()[i+1] <= '9' as u8 {
				range = 2;
			}
			return Some((i, range));
		}
	}
	return None;
}

fn contains_numeral_reverse(string: &String) -> Option<(usize,usize)> {

	let mut iterator = string.chars().rev().enumerate().peekable();

	while let Some((i, character)) = iterator.next() {

		if character >= '0' && character <= '9' { 

			let mut range = 1;
			if let Some((_, peekchar)) = iterator.peek() {
				if peekchar >= &'0' && peekchar <= &'9' {
					range = 2;
				}
			}
			return Some((i, range));
		}
	}
	return None;
}

fn explode(line: &String) -> Option<(String, usize)> {

	let mut pair_count = 0;
	let mut new_string = line.clone();
	for (i, character) in line.chars().enumerate() {

		if character == '[' {
			pair_count += 1;
		}
		if character == ']' {
			pair_count -= 1;
		}

		if pair_count > 4 {

			if !(character == '[' || character == ',') { // !(character == '[' || character == ',')
				
				let pair_end = &line[i..line.len()].find("]").unwrap() + i;
				let pair_string = &line[i..pair_end];
				if !pair_string.contains("[") { // !pair_string.contains("[") && pair_string.len() >= 5

					new_string.replace_range(i-1..pair_end+1, "");
					new_string.insert(i-1, '0');
					let numbers = &pair_string[0..pair_string.len()];
					let nums: Vec<&str> = numbers.split(",").collect();
					let left_number: i32 = nums[0].parse().unwrap();
					let right_number: i32 = nums[1].parse().unwrap();

					if &line[i-2..i-1] == "," { //on the right side
						
						let remain = String::from(&new_string[0..i-2]);
						if let Some((pos, range)) = contains_numeral_reverse(&String::from(remain)) {
							let mut new_pos = i - pos;

							if range == 2 {
								new_pos -= 1;
							}
							let current_left_number: i32 = line[new_pos-3..new_pos-3+range].parse().unwrap();
							new_string.replace_range(new_pos-3..new_pos-3+range, "");
							let insert = format!("{}", left_number + current_left_number);
							new_string.insert_str(new_pos-3, &insert[0..insert.len()]);
						}

						//now find the next 
						let remaining = &new_string[i+2..new_string.len()];

						if let Some((position, width)) = contains_numeral(&String::from(remaining)) {
							let pos = position + i + 2;
							let current_right_number: i32 = new_string[pos..pos+width].parse().unwrap();
							new_string.replace_range(pos..pos+width, "");
							let right_insert = format!("{}", current_right_number + right_number);
							new_string.insert_str(pos, &right_insert[0..right_insert.len()]);
						}
					} else { //on the left side

						let remain = String::from(&new_string[i+1..new_string.len()]);

						if let Some((pos, width)) = contains_numeral(&remain) {
							let new_pos = i + 1 + pos;
							let current_right_number: i32 = new_string[new_pos..new_pos+width].parse().unwrap();
							new_string.replace_range(new_pos..new_pos+width, "");
							let insert_right = format!("{}", current_right_number + right_number);
							new_string.insert_str(new_pos, &insert_right[0..insert_right.len()]);
						}

						//now find the previous number (if any) and add to that.
						let left_side_line = &new_string[0..i-1];
						if let Some((position,range)) = contains_numeral_reverse(&String::from(left_side_line)) {

							let mut new_pos = i - position - 2;

							if range == 2 {
								new_pos -= 1;
							}
							let current_left_number: i32 = new_string[new_pos..new_pos+range].parse().unwrap();
							new_string.replace_range(new_pos..new_pos+range, "");				
							let insert_left = format!("{}", current_left_number+left_number);
							new_string.insert_str(new_pos, &insert_left[0..insert_left.len()]);
						}
					}
					return Some((new_string, i));
				}
			}
		}
	}
	return None;

}

fn reduce(line: &String) -> String {

	let mut new_line = line.clone();
	loop {
		let mut break_loop = 0;
		if let Some((exploded_line, _)) = explode(&new_line) {
			new_line = exploded_line;
		} else {
			break_loop += 1;
			if let Some((splitted_line, _)) = split(&new_line) {
				new_line = splitted_line;
			} else {
				break_loop +=1;
			}
		}
	
		if break_loop == 2 {
			break;
		}
	}
	return new_line;
}

fn magnitude(string: &String) -> i32 {

	let mut value: i32 = 0;
	if string.len() == 5 {
		value += &string[1..2].parse::<i32>().unwrap() * 3;
		value += &string[3..4].parse::<i32>().unwrap() * 2;
	} else if string.len() < 5 {
		value += string.parse::<i32>().unwrap();
	} else {
		let sub = &string[1..string.len()-1];
		let mut depth = 0;
		for (i,character) in sub.chars().enumerate() {
			if character == '[' {
				depth += 1;
			}
			if character == ']' {
				depth -= 1;
			}
			if character == ',' && depth == 0 {
				value += magnitude(&sub[0..i].to_string()) * 3;
				value += magnitude(&sub[i+1..sub.len()].to_string()) * 2;
			}
		}
	}
	return value;
}

fn part1(input: &Vec<&str>) {
	let mut added = String::from(input[0]);
	for i in 1..input.len() {
		added = add_lines(&added, &String::from(input[i]));
	}
	println!("Part 1 Output: {}", magnitude(&added));
}

fn part2(input: &Vec<&str>) {
	let mut largest = 0;
	for a in input {
		for b in input {
			let added = add_lines(&a.to_string(), &b.to_string());
			let magni = magnitude(&added);
			if magni > largest {
				largest = magni;
			}
		}
	}
	println!("Part 2 Output: {}", largest);
}

fn main() {

	let input = std::fs::read_to_string("input.txt").unwrap();

	let mut snailfish_number_lines = Vec::new();

	for line in input.split("\n") {
		snailfish_number_lines.push(line);
	}

	part1(&snailfish_number_lines);
	part2(&snailfish_number_lines);
}