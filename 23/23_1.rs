use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, PartialOrd)]
struct State {
    hallway: String,
    room_a: String,
    room_b: String,
    room_c: String,
    room_d: String,
    cost: usize
}

fn get_amphipod(amphipod: char) -> &'static str {
    match amphipod {
        'A' => "A",
        'B' => "B",
        'C' => "C",
        'D' => "D",
        _ => "."
    }
}

fn get_amphipod_cost(amphipod: char) -> usize {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => 0
    }
}

impl State {
    fn new(a: &str, b: &str, c: &str, d: &str) -> State {
        return State {
            hallway: "...........".to_string(),
            room_a: a.to_string(),
            room_b: b.to_string(),
            room_c: c.to_string(),
            room_d: d.to_string(),
            cost: 0
        };
    }

    fn make_copy(&self) -> State {
        return State {
            hallway: self.hallway.to_string(),
            room_a: self.room_a.to_string(),
            room_b: self.room_b.to_string(),
            room_c: self.room_c.to_string(),
            room_d: self.room_d.to_string(),
            cost: self.cost
        };
    }

    fn is_done(&self) -> bool {
        return self.room_a == "AA".to_string() && self.room_b == "BB".to_string() && self.room_c == "CC".to_string() && self.room_d == "DD".to_string();
    }

    fn is_occupied(&self, pos: usize, start: usize, end: usize) -> bool {
        let min = std::cmp::min(start, end);
        let max = std::cmp::max(start, end);
        let mut occupied = false;

        for (i,character) in self.hallway.chars().enumerate() {
            if i >= min && i <= max && i != pos {
                if character != '.' {
                    occupied = true;
                    break;
                }
            }
        }

        return occupied;
    }

    fn fetch_next_moves(&self) -> Vec<State> {

        let mut moves = Vec::new();

        let valid_hallway_positions = [0, 1, 3, 5, 7, 9, 10];

        if self.room_a != "AA".to_string() {
            let pos1 = self.room_a.as_bytes()[0] as char;
            let pos2 = self.room_a.as_bytes()[1] as char;
            if !(pos1 == '.') {
                for position in valid_hallway_positions {
                    if !self.is_occupied(2, 2, position) {
                        //move is valid!!!
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod(pos1));
                        new_state.room_a.replace_range(0..1, ".");
                        new_state.cost += (1 + (position as i32 - 2 as i32).abs() as usize) * get_amphipod_cost(pos1);
                        moves.push(new_state);
                    }
                }
            } else if pos2 != 'A' && pos2 != '.' {
                for position in valid_hallway_positions {
                    if !self.is_occupied(2, 2, position) {
                        //move is valid!!!
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod(pos2));
                        new_state.room_a.replace_range(1..2, ".");
                        new_state.cost += (2 + (position as i32 - 2 as i32).abs() as usize) * get_amphipod_cost(pos2);
                        moves.push(new_state);
                    }
                }
            }
        }

        if self.room_b != "BB".to_string() {
            let pos1 = self.room_b.as_bytes()[0] as char;
            let pos2 = self.room_b.as_bytes()[1] as char;
            if !(pos1 == '.') {
                for position in valid_hallway_positions {
                    if !self.is_occupied(4, 4, position) {
                        //move is valid!!!
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod(pos1));
                        new_state.room_b.replace_range(0..1, ".");
                        new_state.cost += (1 + (position as i32 - 4 as i32).abs() as usize) * get_amphipod_cost(pos1);
                        moves.push(new_state);
                    }
                }
            } else if pos2 != 'B' && pos2 != '.' {
                for position in valid_hallway_positions {
                    if !self.is_occupied(4, 4, position) {
                        //move is valid!!!
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod(pos2));
                        new_state.room_b.replace_range(1..2, ".");
                        new_state.cost += (2 + (position as i32 - 4 as i32).abs() as usize) * get_amphipod_cost(pos2);
                        moves.push(new_state);
                    }
                }
            }
        }

        if self.room_c != "CC".to_string() {
            let pos1 = self.room_c.as_bytes()[0] as char;
            let pos2 = self.room_c.as_bytes()[1] as char;
            if !(pos1 == '.') {
                for position in valid_hallway_positions {
                    if !self.is_occupied(6, 6, position) {
                        //move is valid!!!
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod(pos1));
                        new_state.room_c.replace_range(0..1, ".");
                        new_state.cost += (1 + (position as i32 - 6 as i32).abs() as usize) * get_amphipod_cost(pos1);
                        moves.push(new_state);
                    }
                }
            } else if pos2 != 'C' && pos2 != '.' {
                for position in valid_hallway_positions {
                    if !self.is_occupied(6, 6, position) {
                        //move is valid!!!
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod(pos2));
                        new_state.room_c.replace_range(1..2, ".");
                        new_state.cost += (2 + (position as i32 - 6 as i32).abs() as usize) * get_amphipod_cost(pos2);
                        moves.push(new_state);
                    }
                }
            }
        }

        if self.room_d != "DD".to_string() {
            let pos1 = self.room_d.as_bytes()[0] as char;
            let pos2 = self.room_d.as_bytes()[1] as char;
            if !(pos1 == '.') {
                for position in valid_hallway_positions {
                    if !self.is_occupied(8, 8, position) {
                        //move is valid!!!
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod(pos1));
                        new_state.room_d.replace_range(0..1, ".");
                        new_state.cost += (1 + (position as i32 - 8 as i32).abs() as usize) * get_amphipod_cost(pos1);
                        moves.push(new_state);
                    }
                }
            } else if pos2 != 'D' && pos2 != '.' {
                for position in valid_hallway_positions {
                    if !self.is_occupied(8, 8, position) {
                        //move is valid!!!
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod(pos2));
                        new_state.room_d.replace_range(1..2, ".");
                        new_state.cost += (2 + (position as i32 - 8 as i32).abs() as usize) * get_amphipod_cost(pos2);
                        moves.push(new_state);
                    }
                }
            }
        }

        //check the hallway!
        for position in valid_hallway_positions {

            let amphipod = self.hallway.as_bytes()[position] as char;

            if amphipod == 'A' {
                if !self.is_occupied(position, 2, position) {
                    if self.room_a == "..".to_string() {
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod('.'));
                        new_state.room_a.replace_range(1..2, get_amphipod('A'));
                        new_state.cost += 2 + (position as i32 - 2 as i32).abs() as usize;
                        moves.push(new_state);
                    } else if self.room_a == ".A".to_string() {
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod('.'));
                        new_state.room_a.replace_range(0..1, get_amphipod('A'));
                        new_state.cost += 1 + (position as i32 - 2 as i32).abs() as usize;
                        moves.push(new_state);
                    }
                }
            }

            if amphipod == 'B' {
                if !self.is_occupied(position, 4, position) {
                    if self.room_b == "..".to_string() {
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod('.'));
                        new_state.room_b.replace_range(1..2, get_amphipod('B'));
                        new_state.cost += (2 + (position as i32 - 4 as i32).abs() as usize) * 10;
                        moves.push(new_state);
                    } else if self.room_b == ".B".to_string() {
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod('.'));
                        new_state.room_b.replace_range(0..1, get_amphipod('B'));
                        new_state.cost += (1 + (position as i32 - 4 as i32).abs() as usize) * 10;
                        moves.push(new_state);
                    }
                }
            }

            if amphipod =='C' {
                if !self.is_occupied(position, 6, position) {
                    if self.room_c == "..".to_string() {
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod('.'));
                        new_state.room_c.replace_range(1..2, get_amphipod('C'));
                        new_state.cost += (2 + (position as i32 - 6 as i32).abs() as usize) * 100;
                        moves.push(new_state);
                    } else if self.room_c == ".C".to_string() {
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod('.'));
                        new_state.room_c.replace_range(0..1, get_amphipod('C'));
                        new_state.cost += (1 + (position as i32 - 6 as i32).abs() as usize) * 100;
                        moves.push(new_state);
                    }
                }
            }

            if amphipod == 'D' {

                
                if !self.is_occupied(position, 8, position) {

                    if self.room_d == "..".to_string() {
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod('.'));
                        new_state.room_d.replace_range(1..2, get_amphipod('D'));
                        new_state.cost += (2 + (position as i32 - 8 as i32).abs() as usize) * 1000;
                        moves.push(new_state);
                    } else if self.room_d == ".D".to_string() {
                        let mut new_state = self.make_copy();
                        new_state.hallway.replace_range(position..position+1, get_amphipod('.'));
                        new_state.room_d.replace_range(0..1, get_amphipod('D'));
                        new_state.cost += (1 + (position as i32 - 8 as i32).abs() as usize) * 1000;
                        moves.push(new_state);
                    }
                }
            }

        }
        
        return moves;
    }
    
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.cost.cmp(&self.cost).reverse();
    }
}

fn part1(start_state: &State) {

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut hash: HashMap<State, bool> = HashMap::new();

    queue.push(start_state.make_copy());
    hash.insert(start_state.make_copy(), true);

    let mut cost = std::usize::MAX;

    while let Some(state) = queue.pop() {

        if state.is_done() {
            
            if state.cost < cost {
                cost = state.cost;
            }
        }

        let next_moves = state.fetch_next_moves();
        for new_state in next_moves {
            if !hash.contains_key(&new_state) {
                hash.insert(new_state.make_copy(), true);
                queue.push(new_state);
            } 
        }

    }

    println!("Part 1 Output: {}", cost);

}

fn main() {

    let start_state = State::new("DC", "BC", "DA", "AB");

    part1(&start_state);

}