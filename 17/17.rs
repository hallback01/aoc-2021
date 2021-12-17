struct Rectangle {
	x1: i32,
	y1: i32,
	x2: i32,
	y2: i32
}

struct Vector {
	x: i32,
	y: i32
}

impl Rectangle {
	fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Rectangle {
		return Rectangle {
			x1,
			y1,
			x2,
			y2
		};
	}
	fn contains_vector(&self, vector: &Vector) -> bool {
		if vector.x >= self.x1 && vector.x <= self.x2 {
			if vector.y >= self.y1 && vector.y <= self.y2 {
				return true;
			}
		}
		return false;
	}
}

impl Vector {
	fn new(x: i32, y: i32) -> Vector {
		return Vector {
			x,
			y
		};
	}
	fn add(&mut self, other: &Vector) {
		self.x += other.x;
		self.y += other.y;
	}
}

fn try_velocity(x: i32, y:i32) -> (i32, bool) {

	let mut position = Vector::new(0, 0);
	let mut velocity = Vector::new(x, y);
	let area = Rectangle::new(79, -176, 137, -117); //my speficic input: x1, y1, x2, y2

	let mut height = -100000;

	while !area.contains_vector(&position) {

		position.add(&velocity);
		if velocity.x > 0 {
			velocity.x -= 1;
		} else if velocity.x < 0 {
			velocity.x += 1;
		}
		velocity.y -= 1;

		if position.y > height {
			height = position.y;
		}

		if position.y < area.y1 || position.x > area.x2 {
			return (height, false);
		}
	}

	return (height, true);
}

fn simulate() {

	let range = 176; //highest value in the input data.

	let mut highest = 0;
	let mut count = 0;
	for y in -range..range {
		for x in -range..range {
			let (height, succeeded) = try_velocity(x, y);
			if succeeded {
				count+=1;
				if height > highest {
					highest = height;
				}
			}
		}
	}
	println!("Part 1 Output: {}", highest);
	println!("Part 2 Output: {}", count);
}

fn main() {
	simulate();
}