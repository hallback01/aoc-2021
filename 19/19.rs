//implement basic linear algebra structs and functions
struct Matrix {
	values: [[f64; 4]; 4]
}

#[derive(Clone)]
struct Vector3 {
	x: f64,
	y: f64,
	z: f64
}

struct Quaternion {
	w: f64,
	x: f64,
	y: f64,
	z: f64
}

fn tuple_dot_product(a: &(f64,f64,f64,f64), b: &(f64,f64,f64,f64)) -> f64 {
	return a.0 * b.0 + a.1*b.1 + a.2*b.2 + a.3*b.3;
}

impl std::fmt::Display for Vector3 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "[{},{},{}]", self.x as f32, self.y as f32, self.z as f32);
	}
}

impl std::ops::Index<usize> for Matrix {
	type Output = [f64];
	fn index(&self, index: usize) -> &Self::Output {
		return &self.values[index];
	}
}

impl std::ops::IndexMut<usize> for Matrix {
	fn index_mut(&mut self, index: usize) -> &mut [f64] {
		return &mut self.values[index];
	}
}

impl std::ops::Mul for Matrix {
	type Output = Matrix;
	fn mul(self, other: Matrix) -> Matrix {
		let mut multiplied_matrix = Matrix::new();
		for row in 0..4 {
			for column in 0..4 {
				multiplied_matrix[row][column] = tuple_dot_product(&self.get_column(column), &other.get_row(row));
			}
		}
		return multiplied_matrix;
	}
}

impl Quaternion {
	fn euler(h: f64, p: f64, b: f64) -> Quaternion {
		let w =  f64::cos(h/2.0)*f64::cos(p/2.0)*f64::cos(b/2.0) + f64::sin(h/2.0)*f64::sin(p/2.0)*f64::sin(b/2.0);
		let x = -f64::cos(h/2.0)*f64::sin(p/2.0)*f64::cos(b/2.0) - f64::sin(h/2.0)*f64::cos(p/2.0)*f64::sin(b/2.0);
		let y =  f64::cos(h/2.0)*f64::sin(p/2.0)*f64::sin(b/2.0) - f64::sin(h/2.0)*f64::cos(p/2.0)*f64::cos(b/2.0);
		let z =  f64::sin(h/2.0)*f64::sin(p/2.0)*f64::cos(b/2.0) - f64::cos(h/2.0)*f64::cos(p/2.0)*f64::sin(b/2.0);
		return Quaternion {w, x, y, z}
	}
}

impl Vector3 {
	fn new(x: f64, y: f64, z: f64) -> Vector3 {
		return Vector3 {x, y, z};
	}
	fn zero() -> Vector3 {
		return Vector3 {x: 0.0, y: 0.0, z: 0.0};
	}
	fn matrix_multiply(vector: &Vector3, matrix: &Matrix) -> Vector3 {
		let x = vector.x * matrix[0][0] + vector.y*matrix[0][1] + vector.z*matrix[0][2] + matrix[3][0];
		let y = vector.x * matrix[1][0] + vector.y*matrix[1][1] + vector.z*matrix[1][2] + matrix[3][1];
		let z = vector.x * matrix[2][0] + vector.y*matrix[2][1] + vector.z*matrix[2][2] + matrix[3][2];
		return Vector3 {x, y, z}
	}
}

impl Matrix {
	fn new() -> Matrix {
		let mut matrix: [[f64; 4]; 4] = [[0.0; 4];4];
		matrix[0][0] = 1.0;
		matrix[1][1] = 1.0;
		matrix[2][2] = 1.0;
		matrix[3][3] = 1.0;

		return Matrix {
			values: matrix,
		};
	}
	fn trs(translation: &Vector3, rotation: &Quaternion, scale: &Vector3) -> Matrix {

		let mut return_matrix = Matrix::new();

		//set translation
		return_matrix[3][0] = translation.x;
		return_matrix[3][1] = translation.y;
		return_matrix[3][2] = translation.z;
		return_matrix[3][3] = 1.0;

		//set rotation
		return_matrix[0][0] = 1.0 - 2.0*rotation.y*rotation.y - 2.0*rotation.z*rotation.z;
		return_matrix[0][1] = 2.0*rotation.x*rotation.y + 2.0*rotation.w*rotation.z;
		return_matrix[0][2] = 2.0*rotation.x*rotation.z - 2.0*rotation.w*rotation.y;
		
		return_matrix[1][0] = 2.0*rotation.x*rotation.y - 2.0*rotation.w*rotation.z;
		return_matrix[1][1] = 1.0 - 2.0*rotation.x*rotation.x - 2.0*rotation.z*rotation.z;
		return_matrix[1][2] = 2.0*rotation.y*rotation.z + 2.0*rotation.w*rotation.x;
		
		return_matrix[2][0] = 2.0*rotation.x*rotation.z + 2.0*rotation.w*rotation.y;
		return_matrix[2][1] = 2.0*rotation.y*rotation.z - 2.0*rotation.w*rotation.x;
		return_matrix[2][2] = 1.0 - 2.0*rotation.x*rotation.x - 2.0*rotation.y*rotation.y;

		//scale
		let mut scale_matrix = Matrix::new();
		scale_matrix[0][0] = scale.x;
		scale_matrix[1][1] = scale.y;
		scale_matrix[2][2] = scale.z;
		scale_matrix[3][3] = 1.0;

		return return_matrix * scale_matrix;
	}

	fn get_row(&self, row: usize) -> (f64,f64,f64,f64) {
		return (self.values[row][0], self.values[row][1], self.values[row][2], self.values[row][3]);
	}
	fn get_column(&self, column: usize) -> (f64,f64,f64,f64) {
		return (self.values[0][column], self.values[1][column], self.values[2][column], self.values[3][column]);
	}
}

#[derive(Clone)]
struct Scanner {
	beacons: Vec<Vector3>,
	rotation: Vector3,
	position: Vector3
}

impl Scanner {
	fn new() -> Scanner {
		return Scanner {
			beacons: Vec::new(),
			rotation: Vector3::zero(),
			position: Vector3::zero()
		}
	}
}

fn find_rotation_position(complete_scanners: &Vec<Scanner>, scanner: &Scanner) -> Option<(Vector3, Vector3)> {
	let mut occurences: std::collections::HashMap<String, (Vector3, Vector3, usize)> = std::collections::HashMap::new();
	for complete_scanner in complete_scanners.iter() {
		occurences.clear();
		for i in 0..complete_scanner.beacons.len() {
			for j in 0..scanner.beacons.len() {
				let origin = &complete_scanner.beacons[i];
				let check = &scanner.beacons[j];
				for x in 0..4 {
					for y in 0..4 {
						for z in 0..4 {
			
							let x_angle = x as f64 * ((std::f64::consts::PI * 2.0) / 4.0);
							let y_angle = y as f64 * ((std::f64::consts::PI * 2.0) / 4.0);
							let z_angle = z as f64 * ((std::f64::consts::PI * 2.0) / 4.0);
							
							let matrix = Matrix::trs(&Vector3::zero(), &Quaternion::euler(x_angle, y_angle, z_angle), &Vector3::new(1.0, 1.0, 1.0));
							let new_vec = Vector3::matrix_multiply(&check, &matrix);
			
							let delta = Vector3::new(origin.x - new_vec.x, origin.y - new_vec.y, origin.z - new_vec.z);

							match occurences.get_mut(&delta.to_string()) {
								Some((_, _, amount)) => {
									*amount += 1;

									if *amount >= 12 {
										let trs = Matrix::trs(&complete_scanner.position, &Quaternion::euler(complete_scanner.rotation.x, complete_scanner.rotation.y, complete_scanner.rotation.z), &Vector3::new(1.0, 1.0, 1.0));
										let transformed_to_origin = Vector3::matrix_multiply(&delta, &trs);
										return Some((transformed_to_origin.clone(), Vector3::new(x_angle,y_angle,z_angle).clone()));
									}
								}
								None => {
									occurences.insert(delta.to_string(), (delta, Vector3::new(x_angle,y_angle,z_angle), 1));
								}
							}
						}
					}
				}
			}
		}
	}
	return None;

}

fn get_largest_manhattan_distance(positions: &Vec<Vector3>) -> usize {

	let mut highest: f64 = 0.0;
	for scanner1 in positions.iter() {
		for scanner2 in positions.iter() {
			let manhattan_distance = (scanner1.x - scanner2.x).abs() + (scanner1.y - scanner2.y).abs() + (scanner1.z - scanner2.z).abs();
			if manhattan_distance > highest {
				highest = manhattan_distance.round();
			}
		}
	}
	return highest as usize;
}

fn calculate(scanners: &mut Vec<Scanner>) {

	let mut complete_scanners: Vec<Scanner> = Vec::new();
	complete_scanners.push(scanners[0].clone());

	let mut unique_beacons: std::collections::HashSet<String> = std::collections::HashSet::new();

	let mut redo = Vec::new();

	for i in 0..scanners.len() {
		redo.push(i);
	}

	let mut positions = Vec::new();

	while redo.len() > 0 {
		let redo_copy = redo.clone();
		redo.clear();
		for i in redo_copy {
			if let Some((position, rotation)) = find_rotation_position(&complete_scanners, &scanners[i]) {

				positions.push(position.clone());

				for vector in scanners[i].beacons.iter_mut() {
					*vector = Vector3::matrix_multiply(&vector, &Matrix::trs(&position, &Quaternion::euler(rotation.x, rotation.y, rotation.z), &Vector3::new(1.0, 1.0, 1.0)));
					unique_beacons.insert(vector.to_string());
				}
				complete_scanners.push(scanners[i].clone());
			} else {
				redo.push(i);
			}
		}
	}

	println!("Part 1 Output: {}", unique_beacons.len());
	println!("Part 2 Output: {}", get_largest_manhattan_distance(&positions));

}

fn main() {

	//load in all the scanners and beacons.
	let input = std::fs::read_to_string("input.txt").unwrap();
	let mut scanner = Scanner::new();
	let mut scanners: Vec<Scanner> = Vec::new();
	for line in input.split("\n") {

		if line.contains("scanner") {
			//new scanner
			if line != "--- scanner 0 ---" {
				scanners.push(scanner);
				scanner = Scanner::new();
			}
		} else if line != "" {
			//vector
			let mut nums: Vec<f64> = Vec::new();
			for vec_num in line.split(",") {
				nums.push(vec_num.parse::<f64>().unwrap());
			}
			scanner.beacons.push(Vector3::new(nums[0], nums[1], nums[2]));
		}
	}
	scanners.push(scanner);

	calculate(&mut scanners);
}