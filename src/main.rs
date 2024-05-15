use core::str;
use std::io::{self, Write};
use std::collections::BTreeSet;
use crossterm::queue;
use crossterm::style::{ResetColor, SetForegroundColor};
use rand::distributions::{Distribution, Uniform};
use std::fs::OpenOptions;
use crossterm::{
    execute, cursor, terminal,
    style::{Color, Print},
};

mod optimal_choices;
use optimal_choices::OptimalChoices;

fn main() -> io::Result<()> {
    let all_nums = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut board = all_nums.clone();
    let mut board_states: Vec<String> = vec![];
    let mut possible_combinations: Vec::<BTreeSet<i32>>;
    let mut score = 0;
    let mut turn = 0;
    let mut stop = false;
    let mut dice_sum;
    let mut stdout = io::stdout();
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
    let optimal_choices = OptimalChoices::new();
    let mut show_optimal_choices = false;
    let mut x: usize;
    let mut y: usize;
    let mut option_string: String;

    initial_clear_screen(&mut stdout)?;

    while !stop {
        loop {
            if score == 45 {
                println!("You win!");
                break;
            }

            turn += 1;
            dice_sum = die.sample(&mut rng);
            if score < 39 { dice_sum += die.sample(&mut rng); }
            println!("Turn: {}\tScore: {:0>2}\t|{}|", turn, score, board.to_string(", "));
            println!("Rolled sum: {}\n", dice_sum);

            possible_combinations = sum_combinations[dice_sum - 1].clone();
            possible_combinations.retain(|combination| board.is_superset(combination));
            if possible_combinations.len() == 0 {
                println!("No possible combinations. You lose.");
                break;
            }

            let optimal_choice = optimal_choices.optimal_choice(&board, dice_sum);
            println!("Select which tiles to remove:");
            for (i, combination) in possible_combinations.iter().rev().enumerate() {
                y = i / 3;
                x = i % 3;
                if show_optimal_choices && optimal_choice == *combination {
                    queue!(stdout, SetForegroundColor(Color::Green))?;
                }

                option_string = format!("  {:0>2}. {}", i + 1, combination.to_string(", "));
                option_string = format!("{: <15}", option_string);
                execute!(
                    stdout,
                    cursor::MoveTo((x as u16) * 15, (y as u16) + 4),
                    Print(option_string),
                    ResetColor,
                )?;
            }
            println!("\n");

            selected_index = 0;
            while selected_index == 0 || selected_index > possible_combinations.len() {
                input = get_input("> ");
                selected_index = input.trim().parse().unwrap_or(0);
                if selected_index == 123456789 {
                    show_optimal_choices = true;
                }
            }
            selected_combination = possible_combinations[possible_combinations.len() - selected_index].clone();

            score += dice_sum;
            board = board.difference(&selected_combination).cloned().collect();
            board_states.push(all_nums.difference(&board).cloned().collect::<BTreeSet<_>>().to_string(""));
            clear_screen(&mut stdout)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("game_log.csv")
            .unwrap();

        if let Err(e) = writeln!(file, "{}", board_states.join(",")) {
            eprintln!("Couldn't write to file: {}", e);
        }

        println!("\nPlay again? (Y/n)");
        input = get_input("> ");
        match input.trim() {
            "n" | "N" | "no" | "No" => stop = true,
            _ => {
                board = all_nums.clone();
                board_states = vec![];
                score = 0;
                turn = 0;
                clear_screen(&mut stdout)?;
            }
        }
    }

    Ok(())
}

fn initial_clear_screen(stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
    )?;
    Ok(())
}

fn clear_screen(stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::FromCursorUp),
        cursor::MoveTo(0, 0),
    )?;
    Ok(())
}

fn get_input(text: &str) -> String {
    let mut input = String::new();
    print!("{}", text);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Error reading input");
    input
}

trait ToString {
    fn to_string(&self, separator: &str) -> String;
}

impl ToString for BTreeSet<i32> {
    fn to_string(&self, separator: &str) -> String {
        self.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(separator)
    }
}
