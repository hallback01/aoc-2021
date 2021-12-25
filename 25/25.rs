//First time solving every puzzle in advent of code! Was fun! 
//I wanna thank Eric Wastl for such a fun challenge!

#[derive(PartialEq, Clone)]
enum Region {Empty,East,South}
enum Difference {Same, Different}

struct Map {
    regions: Vec<Vec<Region>>,
    width: usize,
    height: usize
}

impl Map {
    fn new(input_location: &str) -> Map {
        let mut h: usize = 0;
        let mut w: usize = 0;
        let input = std::fs::read_to_string(input_location).unwrap();

        let mut regions = Vec::new();

        for lines in input.split("\n") {
            w = lines.len();
            h += 1;
            let mut region_line = Vec::new();
            for character in lines.chars() {
                match character {
                    '>' => region_line.push(Region::East),
                    'v' => region_line.push(Region::South),
                    _ => region_line.push(Region::Empty)
                }
            }
            regions.push(region_line);
        }

        return Map {
            regions: regions,
            width: w,
            height: h
        }
    }

    fn step(&mut self) -> Difference {

        let mut is_same = true;

        let mut already_moved_east = std::collections::HashMap::<(usize, usize), bool>::new();
        let mut already_moved_south = std::collections::HashMap::<(usize, usize), bool>::new();

        let mut compare_to = self.regions.clone();

        //first make the east regions move
        for y in 0..self.height {
            for x in 0..self.width {

                if !already_moved_east.contains_key(&(y,x)) {
                    if self.regions[y][x] == Region::East {
                        if x == self.width-1 {
                            if compare_to[y][0] == Region::Empty {
                                self.regions[y][0] = Region::East;
                                self.regions[y][x] = Region::Empty;
                                already_moved_east.insert((y, 0), true);
                                is_same = false;
                            }
                        } else {
                            if compare_to[y][x+1] == Region::Empty {
                                self.regions[y][x+1] = Region::East;
                                self.regions[y][x] = Region::Empty;
                                already_moved_east.insert((y, x+1), true);
                                is_same = false;
                            }
                        }
                    }
                }
            }
        }

        compare_to = self.regions.clone();

        //the the south
        for y in (0..self.height).rev() {
            for x in 0..self.width {

                if !already_moved_south.contains_key(&(y,x)) {
                    if compare_to[y][x] == Region::South {
                        if y == self.height-1 {
                            if self.regions[0][x] == Region::Empty {
                                self.regions[0][x] = Region::South;
                                self.regions[y][x] = Region::Empty;
                                already_moved_south.insert((0, x), true);
                                is_same = false;
                            }
                        } else {
                            if compare_to[y+1][x] == Region::Empty {
                                self.regions[y+1][x] = Region::South;
                                self.regions[y][x] = Region::Empty;
                                already_moved_south.insert((y+1, x), true);
                                is_same = false;
                            }
                        }
                    }
                }
            }
        }

        if is_same {
            return Difference::Same;
        } else {
            return Difference::Different;
        }
    }
}

fn main() {

    let mut map = Map::new("input.txt");

    let mut steps = 0;
    
    loop {
        match map.step() {
            Difference::Same => {
                steps += 1;
                println!("Final Output: {} 😊!!", steps);
                break;
            }

            Difference::Different => {
                steps += 1;
            }
        }
    }

}