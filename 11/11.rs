const WIDTH:usize = 10;
const HEIGHT:usize = 10;

fn check_already_flashed(x: i32, y: i32,  already_flashed: &mut Vec<(usize, usize)>) -> bool {
    if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
        return already_flashed.contains(&(x as usize, y as usize));
    }
    return true;
}

fn simulate(grid: &mut [[u32; WIDTH]; HEIGHT]) -> u32 {
    let mut flashes = 0;

    let mut already_flashed: Vec<(usize, usize)> = Vec::new();
    
    let mut to_check: Vec<(usize, usize)> = Vec::new();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            grid[x][y] += 1;

            if grid[x][y] > 9 {
                to_check.push((x,y));
            }
        }
    }

    while to_check.len() > 0 {

        let (x, y) = to_check.pop().unwrap();

        if !already_flashed.contains(&(x,y)) {

            if grid[x][y] > 9 {
                flashes+=1;

                grid[x][y] = 0;
                
                let xx = x as i32;
                let yy = y as i32;

                if !check_already_flashed(xx+1, yy, &mut already_flashed) {
                    grid[x+1][y] += 1;
                    to_check.push((x+1,y));
                }

                if !check_already_flashed(xx-1, yy, &mut already_flashed) {
                    grid[x-1][y] += 1;
                    to_check.push((x-1,y));
                }

                if !check_already_flashed(xx, yy+1, &mut already_flashed) {
                    grid[x][y+1] += 1;
                    to_check.push((x,y+1));
                }

                if !check_already_flashed(xx, yy-1, &mut already_flashed) {
                    grid[x][y-1] += 1;
                    to_check.push((x,y-1));
                }

                //DIAGONAL
                if !check_already_flashed(xx+1, yy+1, &mut already_flashed) {
                    grid[x+1][y+1] += 1;
                    to_check.push((x+1,y+1));
                }

                if !check_already_flashed(xx-1, yy+1, &mut already_flashed) {
                    grid[x-1][y+1] += 1;
                    to_check.push((x-1,y+1));
                }

                if !check_already_flashed(xx+1, yy-1, &mut already_flashed) {
                    grid[x+1][y-1] += 1;
                    to_check.push((x+1,y-1));
                }

                if !check_already_flashed(xx-1, yy-1, &mut already_flashed) {
                    grid[x-1][y-1] += 1;
                    to_check.push((x-1,y-1));
                }
                already_flashed.push((x,y));
            }

        }

    }

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if grid[x][y] > 9 {
                grid[x][y] = 0;
            }
        }
    }

    return flashes;
}

fn part1(mut grid: [[u32; WIDTH]; HEIGHT]) {

    let mut flashes = 0;

    for _n in 0..100 {
        flashes += simulate(&mut grid);
    }

    println!("Part 1 Output: {}", flashes);

}

fn part2(mut grid: [[u32; WIDTH]; HEIGHT]) {

    let mut loop_count = 0;

    loop {
        loop_count+=1;
        if simulate(&mut grid) == 100 {
            break;
        }
    }

    println!("Part 2 Output: {}", loop_count);

}

fn main() {

    let filename = "input.txt";

    match std::fs::read_to_string(filename) {

        Ok(input) => {
            
            let mut grid = [[0u32; WIDTH]; HEIGHT];

            let mut x = 0;
            let mut y = 0;

            for line in input.split('\n') {
                for value in line.chars() {
                
                    match value.to_digit(10) {
                        Some(val) => grid[x][y] = val,
                        None => {},
                    }

                    x+=1;
                }
                y+=1;
                x=0;
            }
            
            part1(grid);
            part2(grid);

        },
        Err(_e) => {
            println!("Could not open the file '{}'. Does it exist?", filename);
        },
    };
}