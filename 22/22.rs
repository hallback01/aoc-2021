use std::collections::HashMap;

fn check_values(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> bool {
    return !(x1 > 50 || x1 < -50 || x2 > 50 || x2 < -50 || y1 > 50 || y1 < -50 || y2 > 50 || y2 < -50 || z1 > 50 || z1 < -50 || z2 > 50 || z2 < -50);
}

fn part1(reboot_steps: &Vec<(bool, i64, i64, i64, i64, i64, i64)>) {
    let mut cubes: HashMap<(i64, i64, i64), bool> = HashMap::new();
    for (on, x1, x2, y1, y2, z1, z2) in reboot_steps {
        if check_values(*x1, *x2, *y1, *y2, *z1, *z2) {
            for x in *x1..*x2+1 {
                for y in *y1..*y2+1 {
                    for z in *z1..*z2+1 {
                        if *on {
                            cubes.insert((x,y,z), true);
                        } else {
                            cubes.remove(&(x,y,z));
                        }
    
                    }
                }
            }
        }
    }
    println!("Part 1 Output: {}", cubes.len());
}

struct Cube {
    on: bool,
    x: i64,
    y: i64,
    z: i64,
    x2: i64,
    y2: i64,
    z2: i64,
}

impl Cube {
    fn new(on: bool, x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> Cube {
        return Cube {
            on: on,
            x: x1,
            y: y1,
            z: z1,
            x2: x2,
            y2: y2,
            z2: z2,
        }
    }

    fn get_volume(&self) -> i128 {
        return ((self.x2 - self.x + 1) * (self.y2 - self.y + 1) * (self.z2 - self.z + 1)) as i128;
    }

    fn intersect(&self, other: &Cube) -> bool {
        return (other.x2 >= self.x && other.x <= self.x2) && (other.y2 >= self.y && other.y <= self.y2) && (other.z2 >= self.z && other.z <= self.z2);
    }

    fn create_exclusion(&self, other: &Cube) -> Cube {
        return Cube {
            on: !self.on,
            x2: std::cmp::min(self.x2, other.x2),
            y2: std::cmp::min(self.y2, other.y2),
            z2: std::cmp::min(self.z2, other.z2),
            x: std::cmp::max(self.x, other.x),
            y: std::cmp::max(self.y, other.y),
            z: std::cmp::max(self.z, other.z),
        };
    }
}

fn part2(reboot_steps: &Vec<(bool, i64, i64, i64, i64, i64, i64)>) {

    let mut cubes: Vec<Cube> = Vec::new();
    
    for (on, x1, x2, y1, y2, z1, z2) in reboot_steps {

        let new_cube = Cube::new(*on, *x1, *x2, *y1, *y2, *z1, *z2);
        let mut to_add = vec![];
        for cube in cubes.iter() {
            if cube.intersect(&new_cube) {
                to_add.push(cube.create_exclusion(&new_cube));
            }
        }
        if *on {
            cubes.push(new_cube);
        }
        for cube in to_add {
            cubes.push(cube);
        }
    }

    let mut total_volume = 0;
    for cube in cubes {
        total_volume += cube.get_volume() * (if cube.on {1} else {-1});
    }

    println!("Part 2 Output: {}", total_volume);

}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut reboot_steps = Vec::new();
    for line in input.split("\n") {
        let x_where = line.find("x").unwrap() as usize;
        let y_where = line.find(",y").unwrap() as usize;
        let z_where = line.find(",z").unwrap() as usize;
        let mut on = true;
        if line.contains("off") {
            on = false;
        }
        let x_range = &line[x_where+2..y_where].split("..").collect::<Vec<&str>>();
        let y_range = &line[y_where+3..z_where].split("..").collect::<Vec<&str>>();
        let z_range = &line[z_where+3..line.len()].split("..").collect::<Vec<&str>>();

        let x1 = x_range[0].parse::<i64>().unwrap();
        let x2 = x_range[1].parse::<i64>().unwrap();
        let y1 = y_range[0].parse::<i64>().unwrap();
        let y2 = y_range[1].parse::<i64>().unwrap();
        let z1 = z_range[0].parse::<i64>().unwrap();
        let z2 = z_range[1].parse::<i64>().unwrap();
        reboot_steps.push((on, x1, x2, y1, y2, z1, z2));
    }

    part1(&reboot_steps);
    part2(&reboot_steps);
}