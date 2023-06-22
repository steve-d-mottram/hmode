mod setter;
mod solver;
mod words;

fn main() {
    let solver = solver::Solver::new();
    let word: &str = std::str::from_utf8(solver.guess()).unwrap();
    println!("solver guessed {}", word);
}
