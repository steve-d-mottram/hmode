mod setter;
mod solver;
mod words;

use setter::Clue;


fn main() {
    let mut solver = solver::Solver::new();
    let mut setter = setter::Setter::new();
    let mut guess; 
    loop {
        guess = solver.guess();
        println!("Guess : {}", std::str::from_utf8(guess).unwrap());
        let result = setter.check(guess);
        if let [Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_)] = result {
            break;
        }
        solver.filter_self(result);
    }  
    println!("solved  {}", std::str::from_utf8(guess).unwrap());
}
