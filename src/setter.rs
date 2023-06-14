use crate::words::*;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LetterStatus {
    Wrong,
    Elsewhere,
    Right,
}

const MAP_OTHERS: [[usize; 4]; 5] = [
    [1, 2, 3, 4],
    [0, 2, 3, 4],
    [0, 1, 3, 4],
    [0, 1, 2, 4],
    [0, 1, 2, 3],
];

pub type CheckResult = [LetterStatus; 5];

#[derive(Debug)]
pub struct Setter {
    chosen: &'static str,
    chars: Vec<char>,
}

impl Setter {
    pub fn new() -> Self {
        let w = answer_words();
        let range = Uniform::new(0usize, w.len());
        let mut rng = rand::thread_rng();
        let index = range.sample(&mut rng);
        Self::from_word(w[index])
    }

    fn from_word(word: &'static str) -> Self {
        let chars: Vec<char> = word.chars().collect();
        Setter {
            chosen: word,
            chars,
        }
    }

    pub fn check(&self, word: &str) -> CheckResult {
        // Break the word to be tested into a vec of Option<char>. This allows us to
        // remove characters from consideration if they are a "green" match, so that
        // they are not also picked up as an "orange" match
        let mut word_chars: Vec<Option<char>> = word.chars().map(|c| Some(c)).collect();

        // result defaults to all "grey"
        let mut result: Vec<LetterStatus> = Vec::from([LetterStatus::Wrong; 5]);

        // Record exact matches
        for i in 0..5 {
            if let Some(c) = word_chars[i] {
                if c == self.chars[i] {
                    result[i] = LetterStatus::Right;
                    word_chars[i] = None;
                }
            }
        }

        // Record "orange" matches
        for i in 0..5 {
            if let Some(c) = word_chars[i] {
                for j in MAP_OTHERS[i] {
                    if c == self.chars[j] {
                        result[i] = LetterStatus::Elsewhere;
                        word_chars[i] = None;
                    }
                }
            }
        }

        [result[0], result[1], result[2], result[3], result[4]]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn mock_setter() -> Setter {
        Setter::from_word("abcce")
    }

    #[test]
    fn check_no_match() {
        let result = mock_setter().check("fghij");
        for i in result {
            assert_eq!(i, LetterStatus::Wrong);
        }
    }

    #[test]
    fn check_1_match() {
        let result = mock_setter().check("fbhij");
        assert_eq!(
            result,
            [
                LetterStatus::Wrong,
                LetterStatus::Right,
                LetterStatus::Wrong,
                LetterStatus::Wrong,
                LetterStatus::Wrong
            ]
        );
    }

    #[test]
    fn check_2_match() {
        let result = mock_setter().check("fbhcj");
        assert_eq!(
            result,
            [
                LetterStatus::Wrong,
                LetterStatus::Right,
                LetterStatus::Wrong,
                LetterStatus::Right,
                LetterStatus::Wrong
            ]
        );
    }

    #[test]
    fn check_1_elsewhere() {
        let result = mock_setter().check("fgahj");
        assert_eq!(
            result,
            [
                LetterStatus::Wrong,
                LetterStatus::Wrong,
                LetterStatus::Elsewhere,
                LetterStatus::Wrong,
                LetterStatus::Wrong
            ]
        );
    }

    #[test]
    fn check_match_only_counted_once() {
        let result = mock_setter().check("bbbbb");
        assert_eq!(
            result,
            [
                LetterStatus::Wrong,
                LetterStatus::Right,
                LetterStatus::Wrong,
                LetterStatus::Wrong,
                LetterStatus::Wrong
            ]
        );
    }
}
