use crate::setter::{CheckResult, Clue, Setter};
use crate::words::{self, answer_words};

#[derive(Debug, Clone)]
pub struct Solver {
    words: Vec<&'static str>,
    probe_words: Vec<&'static str>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            words: answer_words().into_iter().collect(),
            probe_words: words::all_words().into_iter().collect(),
        }
    }

    fn filter(list: &mut Vec<&'static str>, clues: CheckResult) {
        // Split clues into separate collections
        let mut exacts: Vec<(usize, char)> = Vec::with_capacity(5);
        let mut wrongs: Vec<char> = Vec::with_capacity(5);
        let mut elsewheres: Vec<(usize, char)> = Vec::with_capacity(5);
        for (i, clue) in clues.into_iter().enumerate() {
            match clue {
                Clue::Right(c) => exacts.push((i, c)),
                Clue::Wrong(c) => wrongs.push(c),
                Clue::Elsewhere(c) => elsewheres.push((i, c)),
            }
        }

        // Apply position-specific filters to word list
        list.retain(|word| {
            for c in &wrongs {
                if word.contains(*c) {
                    return false;
                }
            }
            for &(i, c) in &exacts {
                if word.chars().nth(i) != Some(c) {
                    return false;
                }
            }
            for &(i, c) in &elsewheres {
                if (!word.contains(c)) || word.chars().nth(i) == Some(c) {
                    return false;
                }
            }
            true
        });
    }

    pub fn guess(&self) -> &'static str {
        let mut best_reduction = 0;
        let mut best_word: &'static str = self.probe_words[0];
        let start_len = self.words.len();
        for probe in &self.probe_words {
            let mut total_diff = 0;
            for word in &self.words {
                let mut cpy = self.words.clone();
                let setter = Setter::from_word(word);
                Solver::filter(&mut cpy, setter.check(probe));
                let diff = start_len - cpy.len();
                total_diff += diff;
            }
            if total_diff > best_reduction {
                best_reduction = total_diff;
                best_word = probe;
            }
        }
        best_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_handles_all_clues() {
        let original = Solver::new();
        let mut filtered = original.clone();
        Solver::filter(
            &mut filtered.words,
            [
                Clue::Right('a'),
                Clue::Wrong('b'),
                Clue::Wrong('c'),
                Clue::Wrong('d'),
                Clue::Elsewhere('e'),
            ],
        );
        assert!(filtered.words.len() > 0);
        assert!(original.words.len() > filtered.words.len());

        for word in filtered.words {
            for c in "bcd".chars() {
                assert!(!word.contains(c));
            }
            assert_eq!(word.chars().nth(0).unwrap(), 'a');
            assert_ne!(word.chars().nth(4).unwrap(), 'e');
            assert!(word.contains('e'));
        }
    }

    #[test]
    fn test_guess() {
        let s = Solver::new();
        assert_eq!(s.guess(), "wibbl");
    }
}
