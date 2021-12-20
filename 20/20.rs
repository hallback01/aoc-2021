const OFFSET: usize = 300;

#[derive(Clone)]
struct Image {data: Vec<Vec<char>>,width: usize,height: usize,offset: usize,}

impl Image {
	fn enhance(&mut self, enchance_string: &String) {

		let mut new_data = vec![vec!['.'; self.width]; self.height];

		for y in 1..self.height-1 {
			for x in 1..self.width-1 {

				let mut sequence = String::new();
				sequence.push(self.data[x-1][y-1]);
				sequence.push(self.data[x][y-1]);
				sequence.push(self.data[x+1][y-1]);
				sequence.push(self.data[x-1][y]);
				sequence.push(self.data[x][y]);
				sequence.push(self.data[x+1][y]);
				sequence.push(self.data[x-1][y+1]);
				sequence.push(self.data[x][y+1]);
				sequence.push(self.data[x+1][y+1]);

				let mut binary = String::new();
				for num in sequence.chars() {
					if num == '#' {
						binary.push('1');
					} else {
						binary.push('0');
					}
				}
				let decimal = usize::from_str_radix(&binary, 2).unwrap();
				let replace_with = enchance_string.as_bytes()[decimal] as char;
				new_data[x][y] = replace_with;
			}
		}
		self.offset -= 1;
		self.data = new_data;
	}

	fn lit_pixel_count(&self) -> usize {
		let mut count = 0;
		for y in self.offset..self.height-self.offset {
			for x in self.offset..self.width-self.offset {
				if self.data[x][y] == '#' {
					count+=1;
				}
			}
		}
		return count;
	}
}

fn part1(input: &mut Image, enchancement_algoritm_string: &String) {
	let mut image = input.clone();
	image.enhance(enchancement_algoritm_string);
	image.enhance(enchancement_algoritm_string);
	println!("Part 1 Output: {}", image.lit_pixel_count());
}

fn part2(input: &mut Image, enchancement_algoritm_string: &String) {
	let mut image = input.clone();
	for _ in 0..50 {
		image.enhance(enchancement_algoritm_string);
	}
	println!("Part 2 Output: {}", image.lit_pixel_count());
}

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let input_split: Vec<&str> = input.split("\n").collect();
	let enchancement_algoritm_string = String::from(input_split[0]);
	let width = input_split[3].len() + OFFSET;
	let height = input_split.len() + OFFSET-2;
	let mut image = Image {data: vec![vec!['.'; width]; height], width, height, offset: OFFSET/2 - (width-OFFSET)/2};

	for y in 2..input_split.len() {
		for (x, pixel) in input_split[y].chars().enumerate() {

			image.data[x+OFFSET/2][y+OFFSET/2 - 2] = pixel;

		}
	}
	part1(&mut image, &enchancement_algoritm_string);
	part2(&mut image, &enchancement_algoritm_string);
}