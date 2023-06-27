mod setter;
mod solver;
mod words;

use setter::Clue;
use std::io::Write;

fn main() {
    let mut total_guesses: u32 = 0;
    let mut outliers: Vec<(&str, u32)> = Vec::new();
    for word in words::answer_words() {
        let mut solver = solver::Solver::new();
        let mut setter = setter::Setter::from_word(word);
        let mut guess;
        loop {
            guess = solver.guess();
            let result = setter.check(guess);
            if let [Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_)] =
                result
            {
                print!(".");
                std::io::stdout().flush().unwrap();
                total_guesses += solver.guesses();
                if solver.guesses() > 6 {
                    outliers.push((std::str::from_utf8(guess).unwrap(), solver.guesses()));
                }
                break;
            }
            solver.filter_self(result);
        }
    }
    println!(
        "\nMean guesses : {}",
        total_guesses as f32 / words::answer_words().len() as f32
    );
    println!("\nOutliers");
    for (word, count) in outliers {
        println!("{} : {}", word, count);
    }
}
