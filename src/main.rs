mod setter;
mod solver;
mod words;

fn main() {
    let solver = solver::Solver::new();
    println!("solver guessed {}", solver.guess());
}
