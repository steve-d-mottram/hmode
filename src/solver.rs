use crate::setter::{CheckResult, Clue, Setter};
use crate::words::*;
use std::collections::{HashMap, HashSet};

struct Index {
    words: Vec<Option<WdlWord>>,
    words_by_byte_posn: HashMap<u8, HashMap<Option<usize>, HashSet<usize>>>,
    size: usize,
}

#[derive(Debug, Clone)]
pub struct Solver {
    words: Vec<&'static [u8; 5]>,
    probe_words: Vec<&'static [u8; 5]>,
    guesses: u32,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            words: answer_words(),
            probe_words: all_words(),
            guesses: 0,
        }
    }

    pub fn guesses(&self) -> u32 {
        self.guesses
    }

    fn filter(list: &Vec<&'static [u8; 5]>, clues: CheckResult) -> Vec<&'static [u8; 5]> {
        // Split clues into separate collections
        let mut exacts: Vec<(usize, u8)> = Vec::with_capacity(5);
        let mut wrongs: Vec<u8> = Vec::with_capacity(5);
        let mut elsewheres: Vec<(usize, u8)> = Vec::with_capacity(5);
        let mut confirmed_letters: Vec<u8> = Vec::with_capacity(5);
        for (i, clue) in clues.into_iter().enumerate() {
            match clue {
                Clue::Right(c) => {
                    exacts.push((i, c));
                    confirmed_letters.push(c);
                }
                Clue::Wrong(c) => wrongs.push(c),
                Clue::Elsewhere(c) => {
                    elsewheres.push((i, c));
                    confirmed_letters.push(c);
                }
            }
        }

        // Apply position-specific filters to word list
        let result: Vec<&'static [u8; 5]> = list
            .iter()
            .filter_map(|&word| {
                for &c in &wrongs {
                    // We can get a Wrong(c) if the letter c also occurs elsewhere in the word and
                    // has already been accounted for in exact matches or elsewheres
                    if word.contains(&c) && !confirmed_letters.contains(&c) {
                        return None;
                    }
                }
                for &(i, c) in &exacts {
                    if word[i] != c {
                        return None;
                    }
                }
                for &(i, c) in &elsewheres {
                    if (!word.contains(&c)) || word[i] == c {
                        return None;
                    }
                }
                Some(word)
            })
            .collect();
        result
    }

    pub fn filter_self(&mut self, clues: CheckResult) {
        self.words = Self::filter(&self.words, clues);
        self.probe_words = Self::filter(&self.probe_words, clues);
    }

    pub fn guess(&mut self) -> &'static [u8; 5] {
        // The exhaustive algorithm is too slow to select the first guess before the
        // answer word list has been pruned, so we have a pre-selected starting word,
        // chosen by running the algorithm on the full word list for 18 or so
        // painful minutes!
        if self.guesses == 0 {
            self.guesses += 1;
            return b"irate";
        }
        assert!(self.words.len() > 0, "Guess called with empty word list");
        assert!(
            self.probe_words.len() > 0,
            "Guess called with empty probe word list"
        );
        if self.words.len() == 1 {
            return self.words[0];
        }
        let mut best_reduction = 0;
        let mut best_word: Option<&'static [u8; 5]> = None;
        let start_len = self.words.len();
        for probe in &self.probe_words {
            let mut total_diff = 0;
            for word in &self.words {
                let mut cpy = self.words.clone();
                let setter = Setter::from_word(word);
                cpy = Solver::filter(&cpy, setter.check(probe));
                if cpy.len() > 0 {
                    let diff = start_len - cpy.len();
                    total_diff += diff;
                }
            }
            if total_diff > best_reduction {
                best_reduction = total_diff;
                best_word = Some(probe);
            }
        }

        self.guesses += 1;
        let result = best_word.expect(
            format!(
                "No probe word was selected. words : {:?}, probe_words : {:?}",
                self.words, self.probe_words
            )
            .as_str(),
        );

        // Remove the guess word from the probe_words list as we should never
        // re-use a guessi
        self.probe_words.retain(|w| *w != result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::setter;
    use crate::solver;

    use super::*;

    #[test]
    fn filter_handles_all_clues() {
        let original = Solver::new();
        let original_len = original.words.len();
        let mut filtered = Solver::filter(
            &original.words,
            [
                Clue::Right(b'a'),
                Clue::Wrong(b'b'),
                Clue::Wrong(b'c'),
                Clue::Wrong(b'd'),
                Clue::Elsewhere(b'e'),
            ],
        );
        assert!(filtered.len() > 0);
        assert!(original_len > filtered.len());

        for word in filtered {
            for c in b"bcd" {
                assert!(!word.contains(c));
            }
            assert_eq!(word[0], b'a');
            assert_ne!(word[4], b'e');
            assert!(word.contains(&b'e'));
        }
    }

    #[test]
    fn test_all_words() {
        for word in answer_words() {
            println!("Testing : {}", std::str::from_utf8(word).unwrap());
            let mut solver = Solver::new();
            let setter = Setter::from_word(word);
            let mut guess;
            loop {
                guess = solver.guess();
                println!(
                    "Word : {}, Guess : {}",
                    std::str::from_utf8(word).unwrap(),
                    std::str::from_utf8(guess).unwrap()
                );
                let result = setter.check(guess);
                if let [Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_), Clue::Right(_)] =
                    result
                {
                    break;
                }
                solver.filter_self(result);
            }

            assert_eq!(guess, word);
        }
    }

    #[test]
    fn repeated_wrong_letter_does_not_eliminate_right_letters() {
        let setter = Setter::from_word(b"crook");
        let mut solver = Solver::new();
        let mut guess = b"xxxxx";
        let mut clues = [Clue::Wrong(b'x'); 5];
        while solver.probe_words.contains(&b"crook") {
            guess = solver.guess();
            clues = setter.check(guess);
            println!(
                "Guess = {}, Clue = {:?}",
                std::str::from_utf8(guess).unwrap(),
                clues
            );
            solver.filter_self(clues);
        }
        assert_eq!(
            clues,
            [
                Clue::Right(b'c'),
                Clue::Right(b'r'),
                Clue::Right(b'o'),
                Clue::Right(b'o'),
                Clue::Right(b'k')
            ]
        );
        assert_eq!(guess, b"crook");
    }
}
