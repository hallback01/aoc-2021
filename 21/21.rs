struct DeterministicDice {
	current_value: u64,
	total_rolls: u64
}

impl DeterministicDice {
	fn new() -> DeterministicDice {
		return DeterministicDice {current_value: 0, total_rolls: 0}
	}
	fn roll(&mut self) -> u64 {
		self.current_value += 1;
		if self.current_value > 100 {
			self.current_value = 1;
		}
		self.total_rolls+=1;
		return self.current_value;
	}
	fn get_total_rolls(&self) -> u64 {
		return self.total_rolls;
	}
}

#[derive(Clone)]
struct Player {
	position: u64,
	score: u64
}

impl Player {
	fn new(start_position: u64) -> Player {
		return Player {position: start_position, score: 0};
	}
	fn step(&mut self, steps: u64) {
		self.position += steps;
		while self.position > 10 {
			self.position -= 10;
		}
		self.score += self.position;
	}
}

fn quantum_solve(rolls: &std::collections::HashMap<u64, usize>, turn: bool, p1: &Player, p2: &Player) -> (usize,usize) {
	if p1.score >= 21 {
		return (1, 0);
	}
	if p2.score >= 21 {
		return (0, 1);
	}
	let mut total_wins_1 = 0;
	let mut total_wins_2 = 0;
	for (roll, amount) in rolls.iter() {
		if turn {
			let mut temp_player = p2.clone();
			temp_player.step(*roll);
			let (result_wins_1, result_wins_2) = quantum_solve(&rolls, false, &p1, &temp_player);
			total_wins_1 += result_wins_1 * amount;
			total_wins_2 += result_wins_2 * amount;
		} else {
			let mut temp_player = p1.clone();
			temp_player.step(*roll);
			let (result_wins_1, result_wins_2) = quantum_solve(&rolls, true, &temp_player, &p2);
			total_wins_1 += result_wins_1 * amount;
			total_wins_2 += result_wins_2 * amount;
		}
	}
	return (total_wins_1, total_wins_2);
}

fn dirac_dice_game(p1s: u64, p2s: u64) -> usize {
	let mut rolls = std::collections::HashMap::new();
	for x in 1..4 {
		for y in 1..4 {
			for z in 1..4 {
				if let Some(amount) = rolls.get_mut(&(x+y+z)) {
					*amount += 1;
				} else {
					rolls.insert(x+y+z, 1);
				}
			}
		}
	}
	let (r1, r2) = quantum_solve(&rolls, false, &Player::new(p1s), &Player::new(p2s));
	return if r1 > r2 {r1} else {r2};
}

fn practise_game(p1s: u64, p2s: u64) -> u64 {
	let mut det_dice = DeterministicDice::new();
	let mut player1 = Player::new(p1s);
	let mut player2 = Player::new(p2s);
	loop {
		player1.step(det_dice.roll() + det_dice.roll() + det_dice.roll());
		if player1.score >= 1000 {
			return player2.score * det_dice.get_total_rolls();
		}
		player2.step(det_dice.roll() + det_dice.roll() + det_dice.roll());
		if player2.score >= 1000 {
			return player1.score * det_dice.get_total_rolls();
		}
	}
}

fn main() {
	println!("Part 1 Output: {}", practise_game(4, 2));
	println!("Part 2 Output: {}", dirac_dice_game(4, 2));
}