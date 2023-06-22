use crate::words::*;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Clue {
    Wrong(u8),
    Elsewhere(u8),
    Right(u8),
}

const MAP_OTHERS: [[usize; 4]; 5] = [
    [1, 2, 3, 4],
    [0, 2, 3, 4],
    [0, 1, 3, 4],
    [0, 1, 2, 4],
    [0, 1, 2, 3],
];

pub type CheckResult = [Clue; 5];

#[derive(Debug)]
pub struct Setter {
    chosen: &'static [u8; 5],
}

impl Setter {
    pub fn new() -> Self {
        let w = answer_words();
        let range = Uniform::new(0usize, w.len());
        let mut rng = rand::thread_rng();
        let index = range.sample(&mut rng);
        Self::from_word(w[index])
    }

    pub fn from_word(word: &'static [u8; 5]) -> Self {
        Setter { chosen: word }
    }

    pub fn check(&self, word: &[u8; 5]) -> CheckResult {
        // Break the word into an array of chars so that we can index over it
        //        let word_chars: Vec<u8> = word.chars().collect();
        let mut excluded_word: Vec<bool> = [false, false, false, false, false].into();
        let mut excluded_self = excluded_word.clone();

        // result defaults to all "grey"
        let mut result: Vec<Clue> = Vec::with_capacity(5);
        for i in 0..5 {
            result.push(Clue::Wrong(word[i]));
        }

        // Record exact matches
        for i in 0..5 {
            if word[i] == self.chosen[i] {
                result[i] = Clue::Right(word[i]);
                excluded_word[i] = true;
                excluded_self[i] = true;
            }
        }

        // Record "orange" matches
        for i in 0..5 {
            for j in MAP_OTHERS[i] {
                if !(excluded_word[i] || excluded_self[j]) && (self.chosen[j] == word[i]) {
                    result[i] = Clue::Elsewhere(word[i]);
                    excluded_word[i] = true;
                    excluded_self[j] = true;
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
        Setter::from_word(b"abcce")
    }

    #[test]
    fn check_no_match() {
        let test = b"fghij";
        let result = mock_setter().check(test);
        let mut n: usize = 0;
        for i in result {
            assert_eq!(i, Clue::Wrong(test[n]));
            n += 1;
        }
    }

    #[test]
    fn check_1_match() {
        let result = mock_setter().check(b"fbhij");
        assert_eq!(
            result,
            [
                Clue::Wrong(b'f'),
                Clue::Right(b'b'),
                Clue::Wrong(b'h'),
                Clue::Wrong(b'i'),
                Clue::Wrong(b'j')
            ]
        );
    }

    #[test]
    fn check_2_match() {
        let result = mock_setter().check(b"fbhcj");
        assert_eq!(
            result,
            [
                Clue::Wrong(b'f'),
                Clue::Right(b'b'),
                Clue::Wrong(b'h'),
                Clue::Right(b'c'),
                Clue::Wrong(b'j')
            ]
        );
    }

    #[test]
    fn check_1_elsewhere() {
        let result = mock_setter().check(b"fgahj");
        assert_eq!(
            result,
            [
                Clue::Wrong(b'f'),
                Clue::Wrong(b'g'),
                Clue::Elsewhere(b'a'),
                Clue::Wrong(b'h'),
                Clue::Wrong(b'j')
            ]
        );
    }

    #[test]
    fn check_match_only_counted_once() {
        let result = mock_setter().check(b"bbbbb");
        assert_eq!(
            result,
            [
                Clue::Wrong(b'b'),
                Clue::Right(b'b'),
                Clue::Wrong(b'b'),
                Clue::Wrong(b'b'),
                Clue::Wrong(b'b')
            ]
        );
    }

    #[test]
    fn real_world() {
        let result = Setter::from_word(b"maybe").check(b"cable");
        assert_eq!(
            result,
            [
                Clue::Wrong(b'c'),
                Clue::Right(b'a'),
                Clue::Elsewhere(b'b'),
                Clue::Wrong(b'l'),
                Clue::Right(b'e')
            ]
        );
    }
}
