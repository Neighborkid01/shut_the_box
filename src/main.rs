use std::{io, io::Write};
use std::collections::BTreeSet;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let all_nums = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut board = all_nums.clone();
    let mut possible_combinations: Vec::<BTreeSet<i32>>;
    let mut score = 0;
    let mut turn = 0;
    let mut dice_sum;
    let mut input: String;
    let mut selected_index: usize;
    let mut selected_combination: BTreeSet::<i32>;
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    let sum_combinations = [
        /*01*/ vec![ BTreeSet::from([1]) ],
        /*02*/ vec![ BTreeSet::from([2]) ],
        /*03*/ vec![ BTreeSet::from([1, 2]), BTreeSet::from([3]) ],
        /*04*/ vec![ BTreeSet::from([1, 3]), BTreeSet::from([4]) ],
        /*05*/ vec![ BTreeSet::from([1, 4]), BTreeSet::from([2, 3]), BTreeSet::from([5]) ],
        /*06*/ vec![ BTreeSet::from([1, 2, 3]), BTreeSet::from([1, 5]), BTreeSet::from([2, 4]), BTreeSet::from([6]) ],
        /*07*/ vec![ BTreeSet::from([1, 2, 4]), BTreeSet::from([1, 6]), BTreeSet::from([2, 5]), BTreeSet::from([3, 4]), BTreeSet::from([7]) ],
        /*08*/ vec![ BTreeSet::from([1, 2, 5]), BTreeSet::from([1, 3, 4]), BTreeSet::from([1, 7]), BTreeSet::from([2, 6]), BTreeSet::from([3, 5]), BTreeSet::from([8]) ],
        /*09*/ vec![ BTreeSet::from([1, 2, 6]), BTreeSet::from([1, 3, 5]), BTreeSet::from([2, 3, 4]), BTreeSet::from([1, 8]), BTreeSet::from([2, 7]), BTreeSet::from([3, 6]), BTreeSet::from([4, 5]), BTreeSet::from([9]) ],
        /*10*/ vec![ BTreeSet::from([1, 2, 3, 4]), BTreeSet::from([1, 2, 7]), BTreeSet::from([1, 3, 6]), BTreeSet::from([1, 4, 5]), BTreeSet::from([2, 3, 5]), BTreeSet::from([1, 9]), BTreeSet::from([2, 8]), BTreeSet::from([3, 7]), BTreeSet::from([4, 6]), BTreeSet::from([10]) ],
        /*11*/ vec![ BTreeSet::from([1, 2, 8]), BTreeSet::from([1, 3, 7]), BTreeSet::from([1, 4, 6]), BTreeSet::from([2, 3, 6]), BTreeSet::from([2, 4, 5]), BTreeSet::from([2, 9]), BTreeSet::from([3, 8]), BTreeSet::from([4, 7]), BTreeSet::from([5, 6]), BTreeSet::from([11]) ],
        /*12*/ vec![ BTreeSet::from([1, 2, 9]), BTreeSet::from([1, 3, 8]), BTreeSet::from([1, 4, 7]), BTreeSet::from([1, 5, 6]), BTreeSet::from([2, 3, 7]), BTreeSet::from([2, 4, 6]), BTreeSet::from([3, 4, 5]), BTreeSet::from([3, 9]), BTreeSet::from([4, 8]), BTreeSet::from([5, 7]), BTreeSet::from([12]) ],
    ];

    clear_screen();

    loop {
        if score == 45 {
            println!("You win!");
            break;
        }

        turn += 1;
        dice_sum = die.sample(&mut rng);
        if score < 39 { dice_sum += die.sample(&mut rng); }
        println!("Turn: {}\tScore: {:0>2}\t|{}|", turn, score, board.to_string());
        println!("Rolled sum: {}\n", dice_sum);

        possible_combinations = sum_combinations[dice_sum - 1].clone();
        possible_combinations.retain(|combination| board.is_superset(combination));
        if possible_combinations.len() == 0 {
            println!("No possible combinations. You lose.");
            break;
        }

        println!("Select which tiles to remove:");
        for (i, combination) in possible_combinations.iter().rev().enumerate() {
            println!("{:0>2}. {}", i + 1, combination.to_string());
        }

        // Ask user to select which combination
        print!("> ");
        let _ = io::stdout().flush();
        input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");
        selected_index = input.trim().parse().unwrap();
        selected_combination = possible_combinations[possible_combinations.len() - selected_index].clone();

        score += dice_sum;
        board = board.difference(&selected_combination).cloned().collect();
        clear_screen();
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for BTreeSet<i32> {
    fn to_string(&self) -> String {
        self.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ")
    }
}

