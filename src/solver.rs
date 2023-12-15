use std::ops::Deref;

use crate::setter::{CheckResult, Clue, Setter};
use crate::words::{tagged, to_static_word, Tagged, DEFAULT_START_WORD};

#[derive(Debug, Clone)]
pub struct Solver {
    words: Vec<Tagged>,
    start_word: [u8; 5],
    guesses: u32,
    use_alt_words: bool,
}

impl Solver {
    pub fn new(alt_words: bool) -> Self {
        Solver {
            words: tagged(),
            start_word: DEFAULT_START_WORD,
            guesses: 0,
            use_alt_words: alt_words,
        }
    }

    pub fn with_start_word(mut self, word: &str) -> Result<Self, String> {
        self.start_word = to_static_word(word, false, self.use_alt_words)?;
        Ok(self)
    }

    pub fn guesses(&self) -> u32 {
        self.guesses
    }

    pub fn remaining(&self) -> usize {
        self.words.len()
    }

    fn filter(list: &[Tagged], clues: CheckResult) -> Vec<Tagged> {
        let mut confirmed_letters: Vec<u8> = Vec::with_capacity(5);
        for clue in &clues {
            match clue {
                Clue::Right(c) | Clue::Elsewhere(c) => {
                    confirmed_letters.push(*c);
                }
                Clue::Wrong(_) => {}
            }
        }

        // Apply position-specific filters to word list
        let mut result: Vec<Tagged> = Vec::with_capacity(list.len());
        result.extend(list.iter().filter_map(|&tag| {
            let word = tag.unwrap();
            for (i, clue) in clues.into_iter().enumerate() {
                match clue {
                    Clue::Wrong(c) => {
                        // A letter can be marked as Wrong (gray) if the same letter is
                        // present in the word but already accounted for by a Right (green)
                        // or Elsewhere (orange) clue, so we can only eliminate words containing
                        // the letter if the letter doesn't exist elsewhere in the clues.
                        if word[i] == c || (word.contains(&c) && !confirmed_letters.contains(&c)) {
                            return None;
                        }
                    }
                    Clue::Right(c) => {
                        if word[i] != c {
                            return None;
                        }
                    }
                    Clue::Elsewhere(c) => {
                        if (!word.contains(&c)) || word[i] == c {
                            return None;
                        }
                    }
                }
            }
            Some(tag)
        }));
        result
    }

    pub fn filter_self(&mut self, clues: CheckResult) {
        self.words = Self::filter(&self.words, clues);
        // self.probe_words = Self::filter(&self.probe_words, clues);
    }

    pub fn guess(&mut self) -> [u8; 5] {
        // The exhaustive algorithm is too slow to select the first guess before the
        // answer word list has been pruned, so we have a pre-selected starting word
        if self.guesses == 0 {
            self.guesses += 1;
            return self.start_word;
        }
        assert!(!self.words.is_empty(), "Guess called with empty word list");
        if self.words.len() == 1 {
            return self.words[0].unwrap();
        }
        let mut best_reduction = 0;
        let mut best_word: Option<[u8; 5]> = None;
        let start_len = self.words.len();
        self.words.iter().for_each(|&probe| {
            let probe_word = probe.unwrap();
            let mut total_diff = 0;
            for word in self
                .words
                .iter()
                .take_while(|&tag| tag.is_answer())
                .map(|&tag| tag.unwrap())
            {
                let setter = Setter::from_word(word);
                let filtered = Solver::filter(&self.words, setter.check(probe_word));
                if !filtered.is_empty() {
                    let diff = start_len - filtered.len();
                    total_diff += diff;
                }
            }
            if total_diff > best_reduction {
                best_reduction = total_diff;
                best_word = Some(probe_word);
            }
        });

        self.guesses += 1;
        let result = best_word.unwrap_or_else(|| {
            panic!("No probe word was selected.");
        });

        // Remove the guess word from the probe_words list as we should never
        // re-use a guess
        //self.probe_words.retain(|w| *w != result);
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn filter_handles_all_clues() {
        let original = Solver::new(false);
        let original_len = original.words.len();
        let filtered = Solver::filter(
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
                assert!(!word.unwrap().contains(c));
            }
            assert_eq!(word.unwrap()[0], b'a');
            assert_ne!(word.unwrap()[4], b'e');
            assert!(word.unwrap().contains(&b'e'));
        }
    }

    #[test]
    #[ignore] // This test is very slow. To run, use 'cargo test --ignored' or 'cargo test --include-ignored'
    fn test_some_words() {
        for word in tagged().into_iter().map(|tag| tag.unwrap()).take(500) {
            println!("Testing : {}", std::str::from_utf8(&word).unwrap());
            let mut solver = Solver::new(false);
            let setter = Setter::from_word(word);
            let mut guess;
            loop {
                guess = solver.guess();
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
        let setter = Setter::from_word(*b"crook");
        let mut solver = Solver::new(false);
        //        let mut guess = b"xxxxx";
        let (guess, clues) = loop {
            let guess = solver.guess();
            let clues = setter.check(guess);
            {
                solver.filter_self(clues);
            }
            if !solver.words.contains(&Tagged::Answer(*b"crook")) {
                break (guess, clues);
            }
        };
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
        assert_eq!(guess, *b"crook");
    }

    #[test]
    fn start_word() {
        let solver = Solver::new(false).with_start_word("winch").unwrap();
        assert_eq!(std::str::from_utf8(&solver.start_word).unwrap(), "winch");
    }

    #[test]
    #[should_panic]
    fn start_word_too_long() {
        let _solver = Solver::new(false)
            .with_start_word("too-long")
            .expect("Should panic with word too long");
    }
}
