use crate::words::*;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Clue {
    Wrong(char),
    Elsewhere(char),
    Right(char),
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

    pub fn from_word(word: &'static str) -> Self {
        let chars: Vec<char> = word.chars().collect();
        Setter {
            chosen: word,
            chars,
        }
    }

    pub fn check(&self, word: &str) -> CheckResult {
        // Break the word into an array of chars so that we can index over it
        let word_chars: Vec<char> = word.chars().collect();
        let mut excluded_word: Vec<bool> = [false, false, false, false, false].into();
        let mut excluded_self = excluded_word.clone();

        // result defaults to all "grey"
        let mut result: Vec<Clue> = Vec::with_capacity(5);
        for i in 0..5 {
            result.push(Clue::Wrong(word_chars[i]));
        }

        // Record exact matches
        for i in 0..5 {
            if word_chars[i] == self.chars[i] {
                result[i] = Clue::Right(word_chars[i]);
                excluded_word[i] = true;
                excluded_self[i] = true;
            }
        }

        // Record "orange" matches
        for i in 0..5 {
            for j in MAP_OTHERS[i] {
                if !(excluded_word[i] || excluded_self[j]) && (self.chars[j] == word_chars[i]) {
                    result[i] = Clue::Elsewhere(word_chars[i]);
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
        Setter::from_word("abcce")
    }

    #[test]
    fn check_no_match() {
        let test = "fghij";
        let result = mock_setter().check(test);
        let mut n: usize = 0;
        for i in result {
            assert_eq!(i, Clue::Wrong(test.chars().nth(n).unwrap()));
            n += 1;
        }
    }

    #[test]
    fn check_1_match() {
        let result = mock_setter().check("fbhij");
        assert_eq!(
            result,
            [
                Clue::Wrong('f'),
                Clue::Right('b'),
                Clue::Wrong('h'),
                Clue::Wrong('i'),
                Clue::Wrong('j')
            ]
        );
    }

    #[test]
    fn check_2_match() {
        let result = mock_setter().check("fbhcj");
        assert_eq!(
            result,
            [
                Clue::Wrong('f'),
                Clue::Right('b'),
                Clue::Wrong('h'),
                Clue::Right('c'),
                Clue::Wrong('j')
            ]
        );
    }

    #[test]
    fn check_1_elsewhere() {
        let result = mock_setter().check("fgahj");
        assert_eq!(
            result,
            [
                Clue::Wrong('f'),
                Clue::Wrong('g'),
                Clue::Elsewhere('a'),
                Clue::Wrong('h'),
                Clue::Wrong('j')
            ]
        );
    }

    #[test]
    fn check_match_only_counted_once() {
        let result = mock_setter().check("bbbbb");
        assert_eq!(
            result,
            [
                Clue::Wrong('b'),
                Clue::Right('b'),
                Clue::Wrong('b'),
                Clue::Wrong('b'),
                Clue::Wrong('b')
            ]
        );
    }

    #[test]
    fn real_world() {
        let result = Setter::from_word("maybe").check("cable");
        assert_eq!(
            result,
            [
                Clue::Wrong('c'),
                Clue::Right('a'),
                Clue::Elsewhere('b'),
                Clue::Wrong('l'),
                Clue::Right('e')
            ]
        );
    }
}
