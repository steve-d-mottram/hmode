mod setter;
mod solver;
mod words;

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
        if self.1.len() > 0 {
            write!(f, "\nOutliers")?;
            for outlier in &self.1 {
                write!(f, "{} : {}", outlier.0, outlier.1)?;
            }
        }
        write!(f, "\n")
    }
}

fn heartbeat() {
    print!(".");
    std::io::stdout().flush().unwrap();
}

fn stats_for_start_word(start_word: &str) -> Result<Stats, String> {
    let mut total_guesses: u32 = 0;
    let mut outliers: Vec<Outlier> = Vec::new();
    for word in words::answer_words() {
        let mut solver = solver::Solver::new().with_start_word(start_word)?;
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
                        std::str::from_utf8(guess).unwrap().into(),
                        solver.guesses(),
                    ));
                }
                break;
            }
            solver.filter_self(result);
        }
    }
    Ok(Stats(
        total_guesses as f32 / words::answer_words().len() as f32,
        outliers,
    ))
}

fn demo(target: &str) -> Result<(), String> {
    let setter = setter::Setter::from_str(target)?;
    let mut solver = solver::Solver::new();
    loop {
        let guess = solver.guess();
        let result = setter.check(guess);

        if let [Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_)] =
            result
        {
            println!(
                "solved : {}",
                std::str::from_utf8(guess).map_err(|e| e.to_string())?
            );
            break;
        }
        println!(
            "Guessing : {}",
            std::str::from_utf8(guess).map_err(|e| e.to_string())?
        );
        solver.filter_self(result);
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli {
        Cli {
            start_word: Some(s),
            ..
        } => {
            println!(
                "Calculating statistics for start word \"{}\". This may take some time.",
                s
            );
            println!("{}", stats_for_start_word(&s.as_str())?);
            Ok(())
        }

        Cli { demo: Some(d), .. } => Ok(demo(d.as_str())?),
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
