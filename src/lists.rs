use crate::{
    setter::{CheckResult, Clue},
    words::{answers, probes, WdlWord},
};

use std::fmt;

#[derive(Clone)]
pub struct WordList {
    answers: Vec<WdlWord>,
    probes: Vec<WdlWord>,
}

impl WordList {
    pub fn new(alt: bool) -> Self {
        WordList {
            answers: answers(),
            probes: probes(alt),
        }
    }

    pub fn answer_iter(&self) -> impl Iterator<Item = WdlWord> + '_ {
        self.answers.iter().map(|&x| x)
    }

    pub fn all_iter(&self) -> impl Iterator<Item = WdlWord> + '_ {
        self.answers.iter().chain(self.probes.iter()).map(|&x| x)
    }

    fn filter(list: &[[u8; 5]], clues: CheckResult) -> Vec<[u8; 5]> {
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
        let mut result: Vec<[u8; 5]> = Vec::with_capacity(list.len());
        result.extend(list.iter().filter_map(|&word| {
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
            Some(word)
        }));
        result
    }

    pub fn filter_self(&mut self, clues: CheckResult) {
        self.answers = Self::filter(&self.answers, clues);
        self.probes = Self::filter(&self.probes, clues);
    }

    pub fn len(&self) -> usize {
        self.answers.len()
    }
}

impl fmt::Debug for WordList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WordList{{ answers: {}, probes: {} }}",
            self.answers.len(),
            self.probes.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_list() {
        let l: WordList = WordList::new(false);
        assert!(l.answers.len() > 2000);
        assert!(l.answers.len() < l.probes.len());
    }

    #[test]
    fn get_answer_iterator() {
        let l = WordList::new(false);
        let it = l.answer_iter();
        assert_eq!(it.count(), l.answers.len())
    }

    #[test]
    fn get_all_iterator() {
        let l = WordList::new(false);
        let it = l.all_iter();
        assert_eq!(it.count(), l.answers.len() + l.probes.len());
    }

    #[test]
    fn filter_handles_all_clues() {
        let original = answers();
        let original_len = original.len();
        let filtered = WordList::filter(
            &original,
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
    fn repeated_wrong_letter_does_not_eliminate_right_letters() {
        let clue = [
            Clue::Right(b'c'),
            Clue::Right(b'r'),
            Clue::Right(b'o'),
            Clue::Wrong(b'o'),
            Clue::Right(b'k'),
        ];

        let filtered = WordList::filter(&answers(), clue);
        for word in filtered {
            assert_eq!(word[0], b'c');
            assert_eq!(word[1], b'r');
            assert_eq!(word[2], b'o');
            assert_ne!(word[3], b'o');
            assert_eq!(word[4], b'k');
        }
    }
}
