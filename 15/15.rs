use std::collections::HashSet;

struct Grid {
	grid: Vec<Vec<u8>>,
	width: usize,
	height: usize
}

impl Grid {

	fn new(w: usize, h: usize) -> Grid {
		return Grid {
			grid: vec![vec![0u8; w]; h],
			width: w,
			height: h
		};
	}

	fn set_value(&mut self, x: usize, y: usize, value: u8) -> bool {
		if x < self.width && y < self.height {
			self.grid[x][y] = value;
			return true;
		} else {
			return false;
		}
	}

	fn get_value(&self, x: usize, y: usize) -> Option<u8> {
		if x < self.width && y < self.height {
			return Some(self.grid[x][y]);
		} else {
			return None;
		}
	}

}

fn get_heuristic(_x1: i32, _y1: i32, _x2: i32, _y2: i32) -> usize {
	//let _dx: i32 = (_x1 - _x2).abs();
	//let _dy: i32 = (_y1 - _y2).abs();
	//return (dx + dy) as usize;
	return 0; //eliminate the heurstic because A* doesn't guarantee the shortest paht... Now it is dijkstra's instead.
}

fn push_new_node(g: usize, x: i32, y: i32, grid: &Grid, already_added: &mut HashSet<(i32, i32)>, queue: &mut Vec<(usize, usize, usize, usize)>) {

	if x >= 0 && y >= 0 {
		if !already_added.contains(&(x,y)) {

			if let Some(cost) = grid.get_value(x as usize, y as usize) {
				let new_g = g + cost as usize;
				let new_h = get_heuristic(x, y, grid.width as i32 - 1, grid.height as i32 - 1);

				queue.push((x as usize, y as usize, new_g, new_g + new_h));
	
				already_added.insert((x,y));
			}
		}
	}
}

fn path_traversal(grid: &Grid) -> usize {

	let mut queue: Vec<(usize, usize, usize, usize)> = Vec::new();

	let g_value = 0;
	let h_value = get_heuristic(0, 0, grid.width as i32 - 1, grid.height as i32 - 1);

	queue.push((0, 0, g_value, g_value + h_value));

	let mut already_added: HashSet<(i32, i32)> = HashSet::new();

	already_added.insert((0, 0));

	loop {
		if let Some((x, y, g, _f)) = queue.pop() {
			//found end node
			if x == grid.width - 1 && y == grid.height - 1 {
				return g;
			} else {
				push_new_node(g, x as i32 + 1, y as i32, &grid, &mut already_added, &mut queue);
				push_new_node(g, x as i32 - 1, y as i32, &grid, &mut already_added, &mut queue);
				push_new_node(g, x as i32, y as i32 + 1, &grid, &mut already_added, &mut queue);
				push_new_node(g, x as i32, y as i32 - 1, &grid, &mut already_added, &mut queue);

				//sort list.
				queue.sort_by_key(|k| k.3);
				queue.reverse();
			}
		} else {
			println!("Could not find a path..");
			return std::usize::MAX;
		}
	} 

}

fn part1() {

	let input = std::fs::read_to_string("input.txt").unwrap();

	let width = input.find("\n").unwrap();
	let height = input.len() / width;

	let mut grid = Grid::new(width, height);

	let mut x = 0;
	let mut y = 0;
	for line in input.split("\n") {
		for chr in line.chars() {
			grid.set_value(x, y, chr as u8 - 0x30);
			x += 1;
		}
		x = 0;
		y += 1;
	}

	println!("Part 1 Output: {}", path_traversal(&grid));
}

fn part2() {

	let input = std::fs::read_to_string("input.txt").unwrap();

	let width = input.find("\n").unwrap();
	let height = input.len() / width;

	let mut grid = Grid::new(width * 5, height * 5);

	let mut x = 0;
	let mut y = 0;

	let width_inc = width - 1;
	let height_inc = height - 1;

	for line in input.split("\n") {
		for chr in line.chars() {

			let mut start_y = chr as u8 - 0x30;
			let mut start_x = chr as u8 - 0x30;
			for yy in 0..5 {

				for xx in 0..5 {
					grid.set_value(x + xx + width_inc * xx, y + yy + height_inc * yy, start_x);
					start_x += 1; if start_x > 9 {start_x = 1; }
				}
				start_y += 1; if start_y > 9 { start_y = 1; }
				start_x = start_y;
			}
			
			x += 1;
		}
		x = 0;
		y += 1;
	}

	println!("Part 2 Output: {}", path_traversal(&grid));
}

fn main() {

	part1();
	part2();
	
}