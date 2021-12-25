/* 
    Credit to @Strobetal on reddit!

    Not gonna lie, I used the subreddit a lot for help in this one!
    Since it was christmas eve as well I didn't have the time to 
    spend too much time on it! :(
*/
fn get_number(array: &[i32; 14]) -> String {
    let mut number = String::new();
    for num in array {
        number.push_str(&num.to_string());
    }
    return number;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut segments = Vec::new();
    for segment in input.split("inp w\n") {
        let mut seg = Vec::new();
        for line in segment.split("\n") {
            seg.push(line.to_string());
        }
        if seg.len() > 3 {
            segments.push(seg);
        }
    }
    let mut max = [0; 14];
    let mut min = [0; 14];
    let mut stack = Vec::new();
    for (mut i, code) in segments.iter().enumerate() {

        if code[3] == "div z 1" {

            let split = code[14].split(" ").collect::<Vec<&str>>();
            let num = split[2].to_string().parse::<i32>().unwrap();
            stack.push((i, num));

        } else if code[3] == "div z 26" {
            if let Some((mut j, x)) = stack.pop() {
                let split = code[4].split(" ").collect::<Vec<&str>>();
                let num = split[2].to_string().parse::<i32>().unwrap();
                let mut difference = x + num;
                if difference < 0 {
                    let i_temp = i;
                    let j_temp = j;
                    i = j_temp;
                    j = i_temp;
                    difference *= -1;
                }
                max[i] = 9;
                max[j] = 9 - difference;
                min[i] = 1 + difference;
                min[j] = 1;
            }
        }
    }
    println!("Part 1 Output: {}", get_number(&max));
    println!("Part 2 Output: {}", get_number(&min));
}