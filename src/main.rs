mod setter;
mod words;

fn main() {
    for w in words::answer_words() {
        println!("Hello, {}", w);
    }
    println!("Setter : {:?}", setter::Setter::new());
}
