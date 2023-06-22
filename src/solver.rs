use crate::setter::{CheckResult, Clue, Setter};
use crate::words::{self, answer_words};

#[derive(Debug, Clone)]
pub struct Solver {
    words: Vec<&'static [u8; 5]>,
    probe_words: Vec<&'static [u8; 5]>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            words: answer_words(),
            probe_words: words::all_words(),
        }
    }

    fn filter(list: Vec<&'static [u8; 5]>, clues: CheckResult) -> Vec<&'static [u8; 5]> {
        // Split clues into separate collections
        let mut exacts: Vec<(usize, u8)> = Vec::with_capacity(5);
        let mut wrongs: Vec<u8> = Vec::with_capacity(5);
        let mut elsewheres: Vec<(usize, u8)> = Vec::with_capacity(5);
        for (i, clue) in clues.into_iter().enumerate() {
            match clue {
                Clue::Right(c) => exacts.push((i, c)),
                Clue::Wrong(c) => wrongs.push(c),
                Clue::Elsewhere(c) => elsewheres.push((i, c)),
            }
        }

        #[inline]
        fn fast_contains(word: &'static [u8; 5], c: u8) -> bool {
            word[0] == c || word[1] == c || word[2] == c || word[3] == c || word[4] == c
        }

        // Apply position-specific filters to word list
        let result = list
            .into_iter()
            .filter(|word| {
                for &c in &wrongs {
                    if fast_contains(word, c) {
                        return false;
                    }
                }
                for &(i, c) in &exacts {
                    if word[i] != c {
                        return false;
                    }
                }
                for &(i, c) in &elsewheres {
                    if (!fast_contains(word, c)) || word[i] == c {
                        return false;
                    }
                }
                true
            })
            .collect();
        result
    }

    pub fn guess(&self) -> &'static [u8; 5] {
        let mut best_reduction = 0;
        let mut best_word: &'static [u8; 5] = self.probe_words[0];
        let start_len = self.words.len();
        for probe in &self.probe_words {
            let mut total_diff = 0;
            for word in &self.words {
                let mut cpy = self.words.clone();
                let setter = Setter::from_word(word);
                cpy = Solver::filter(cpy, setter.check(probe));
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
        let original_len = original.words.len();
        let mut filtered = Solver::filter(
            original.words,
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
    fn test_guess() {
        let s = Solver::new();
        assert_eq!(s.guess(), b"wibbl");
    }
}
