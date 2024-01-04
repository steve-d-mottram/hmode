mod setter;
mod solver;
mod words;
mod lists;

use clap::Parser;
use setter::Clue;
use std::io::Write;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    /// Calculates the average number of solving steps
    /// and list words that take more than 6 steps to solve,
    /// given a specific starting word. This may take several minutes
    /// to run on typical desktop hardware.
    start_word: Option<String>,
    /// Performs a demo of the solver, where the provided word is the solution
    #[arg(short, long)]
    demo: Option<String>,
    /// Prints the complete list of recognised Words
    #[arg(long)]
    list_words: bool,
    /// Uses a shorter alternative word list, instead of the very obscure Wordle list of valid words
    #[arg(short, long)]
    alt_words: bool,
}

struct Outlier(String, u32);
struct Stats(f32, Vec<Outlier>);

impl std::fmt::Display for Stats {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "\nAverage solving steps : {}\n", self.0)?;
        if !self.1.is_empty() {
            write!(f, "\nOutliers\n")?;
            for outlier in &self.1 {
                writeln!(f, "{} : {}", outlier.0, outlier.1)?;
            }
        }
        writeln!(f)
    }
}

fn heartbeat() {
    print!(".");
    std::io::stdout().flush().unwrap();
}

fn stats_for_start_word(start_word: &str, alt_words: bool) -> Result<Stats, String> {
    let mut total_guesses: u32 = 0;
    let mut outliers: Vec<Outlier> = Vec::new();
    for word in words::answers() {
        let mut solver = solver::Solver::new(alt_words).with_start_word(start_word)?;
        let setter = setter::Setter::from_word(word);
        let mut guess;
        loop {
            guess = solver.guess();
            let result = setter.check(guess);
            if let [Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_)] =
                result
            {
                heartbeat();
                total_guesses += solver.guesses();
                if solver.guesses() > 6 {
                    outliers.push(Outlier(
                        std::str::from_utf8(&guess).unwrap().into(),
                        solver.guesses(),
                    ));
                }
                break;
            }
            solver.filter_self(result);
        }
    }
    Ok(Stats(
        total_guesses as f32 / words::answers().len() as f32,
        outliers,
    ))
}

fn demo(target: &str, alt_words: bool) -> Result<(), String> {
    let setter = setter::Setter::from_str(target)?;
    let mut solver = solver::Solver::new(alt_words);
    loop {
        let guess = solver.guess();
        let result = setter.check(guess);
        solver.filter_self(result);

        if let [Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_)] =
            result
        {
            println!(
                "solved : {}",
                std::str::from_utf8(&guess).map_err(|e| e.to_string())?
            );
            break;
        }
        println!(
            "Guessing : {}, {}",
            std::str::from_utf8(&guess).map_err(|e| e.to_string())?,
            solver.remaining()
        );
    }
    Ok(())
}

fn list_all_words(alt_words: bool) {
    for i in words::all(alt_words) {
        println!("{}", std::str::from_utf8(&i).expect("Invalid utf8"));
    }
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli {
        Cli {
            start_word: Some(s),
            ..
        } => {
            println!("Calculating statistics for start word \"{s}\". This may take some time.");
            println!("{}", stats_for_start_word(s.as_str(), cli.alt_words)?);
            Ok(())
        }
        Cli { demo: Some(d), .. } => Ok(demo(d.as_str(), cli.alt_words)?),
        Cli { .. } if cli.list_words => {
            list_all_words(cli.alt_words);
            Ok(())
        }

        _ => Err("Invalid parameters. Try 'hmode --help'".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_display() {
        let outliers: Vec<Outlier> =
            [Outlier("table".into(), 7), Outlier("fable".into(), 7)].into();
        let stats = Stats(3.14, outliers);
        println!("{}", stats);
    }
}
