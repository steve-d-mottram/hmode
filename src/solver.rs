use crate::setter::{CheckResult, Clue, Setter};
use crate::words::{all, answers, to_static_word, DEFAULT_START_WORD};

#[derive(Debug, Clone)]
pub struct Solver {
    words: Vec<[u8; 5]>,
    start_word: [u8; 5],
    probe_words: Vec<[u8; 5]>,
    guesses: u32,
    use_alt_words: bool,
}

impl Solver {
    pub fn new(alt_words: bool) -> Self {
        Solver {
            words: answers().to_vec(),
            start_word: DEFAULT_START_WORD,
            probe_words: all(alt_words).to_vec(),
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

    fn filter(list: &[[u8; 5]], clues: CheckResult) -> Vec<[u8; 5]> {
        let mut confirmed: [bool; 256] = [false; 256];
        for clue in &clues {
            match clue {
                Clue::Right(c) | Clue::Elsewhere(c) => confirmed[*c as usize] = true,
                Clue::Wrong(_) => {}
            }
        }

        // Apply position-specific filters to word list
        let mut result: Vec<[u8; 5]> = Vec::with_capacity(list.len());
        result.extend(list.iter().filter_map(|&word| {
            for (i, clue) in clues.into_iter().enumerate() {
                match clue {
                    Clue::Wrong(c) => {
                        // A letter can be marked as Wrong (gray) if the same letter is
                        // present in the word but already accounted for by a Right (green)
                        // or Elsewhere (orange) clue, so we can only eliminate words containing
                        // the letter if the letter doesn't exist elsewhere in the clues.
                        if word[i] == c || (word.contains(&c) && !confirmed[c as usize]) {
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
            Some(word)
        }));
        result
    }

    /// Count how many words in the list match the given clues (without allocating a filtered vector)
    fn count_matching(list: &[[u8; 5]], clues: CheckResult) -> usize {
        let mut confirmed: [bool; 256] = [false; 256];
        for clue in &clues {
            match clue {
                Clue::Right(c) | Clue::Elsewhere(c) => confirmed[*c as usize] = true,
                Clue::Wrong(_) => {}
            }
        }

        // Count words that pass all filters without allocating
        let mut count = 0;
        for &word in list {
            let mut matches = true;
            for (i, clue) in clues.into_iter().enumerate() {
                match clue {
                    Clue::Wrong(c) => {
                        if word[i] == c || (word.contains(&c) && !confirmed[c as usize]) {
                            matches = false;
                            break;
                        }
                    }
                    Clue::Right(c) => {
                        if word[i] != c {
                            matches = false;
                            break;
                        }
                    }
                    Clue::Elsewhere(c) => {
                        if !word.contains(&c) || word[i] == c {
                            matches = false;
                            break;
                        }
                    }
                }
            }
            if matches {
                count += 1;
            }
        }
        count
    }

    pub fn filter_self(&mut self, clues: CheckResult) {
        self.words = Self::filter(&self.words, clues);
        self.probe_words = Self::filter(&self.probe_words, clues);
    }

    pub fn guess(&mut self) -> [u8; 5] {
        // The exhaustive algorithm is too slow to select the first guess before the
        // answer word list has been pruned, so we have a pre-selected starting word
        if self.guesses == 0 {
            self.guesses += 1;
            return self.start_word;
        }
        assert!(!self.words.is_empty(), "Guess called with empty word list");
        assert!(
            !self.probe_words.is_empty(),
            "Guess called with empty probe word list"
        );
        if self.words.len() == 1 {
            return self.words[0];
        }
        let mut best_reduction = 0;
        let mut best_word: Option<[u8; 5]> = None;
        let start_len = self.words.len();
        
        // For each probe word, calculate total reduction in answer list size
        for probe in &self.probe_words {
            let mut total_diff = 0;
            
            // For each answer word, see how much this probe narrows down the list
            for word in &self.words {
                let setter = Setter::from_word(*word);
                let clues = setter.check(*probe);
                
                // Count matching words without allocating a filtered vector
                let matches = Solver::count_matching(&self.words, clues);
                if matches > 0 {
                    let diff = start_len - matches;
                    total_diff += diff;
                }
            }
            
            if total_diff > best_reduction {
                best_reduction = total_diff;
                best_word = Some(*probe);
            }
        }

        self.guesses += 1;
        let result = best_word.unwrap_or_else(|| {
            panic!(
                "No probe word was selected. words : {:?}, probe_words : {:?}",
                self.words, self.probe_words
            )
        });

        // Remove the guess word from the probe_words list as we should never
        // re-use a guess
        self.probe_words.retain(|w| *w != result);
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
                assert!(!word.contains(c));
            }
            assert_eq!(word[0], b'a');
            assert_ne!(word[4], b'e');
            assert!(word.contains(&b'e'));
        }
    }

    #[test]
    #[ignore] // This test is very slow. To run, use 'cargo test --ignored' or 'cargo test --include-ignored'
    fn test_some_words() {
        for &word in answers().into_iter().take(500) {
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
            if !solver.probe_words.contains(&b"crook") {
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
