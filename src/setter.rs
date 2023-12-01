use crate::words::{answers, to_static_word};
use rand::distributions::{Distribution, Uniform};

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
    chosen: [u8; 5],
}

impl Setter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let w = answers();
        let range = Uniform::new(0usize, w.len());
        let mut rng = rand::thread_rng();
        let index = range.sample(&mut rng);
        Self::from_word(w[index])
    }

    pub fn from_word(word: [u8; 5]) -> Self {
        Setter { chosen: word }
    }

    pub fn from_str(word: &str) -> Result<Self, String> {
        let w = to_static_word(word, true, false)?;
        Ok(Self::from_word(w))
    }

    pub fn check(&self, word: [u8; 5]) -> CheckResult {
        let mut chosen_copy = self.chosen;
        let mut word_copy = word;

        // result defaults to all "grey"
        let mut result: Vec<Clue> = Vec::with_capacity(5);
        for c in word.iter().take(5) {
            result.push(Clue::Wrong(*c));
        }

        // Record exact matches

        self.chosen
            .iter()
            .zip(word.iter())
            .enumerate()
            .for_each(|(i, (&my, &other))| {
                if my == other {
                    result[i] = Clue::Right(my);
                    chosen_copy[i] = 0;
                    word_copy[i] = 255;
                }
            });

        // Record "orange" matches
        for i in 0..5 {
            if let Some(p) = chosen_copy.iter().position(|&c| word_copy[i] == c) {
                chosen_copy[p] = 0;
                word_copy[i] = 255;
                result[i] = Clue::Elsewhere(word[i]);
            }
        }
        [result[0], result[1], result[2], result[3], result[4]]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn mock_setter() -> Setter {
        Setter::from_word(*b"abcce")
    }

    #[test]
    fn elsewhere_must_not_override_wrong() {
        let result = Setter::from_word(*b"abccd").check(*b"fccgh");
        assert_eq!(
            result,
            [
                Clue::Wrong(b'f'),
                Clue::Elsewhere(b'c'),
                Clue::Right(b'c'),
                Clue::Wrong(b'g'),
                Clue::Wrong(b'h'),
            ]
        )
    }

    #[test]
    fn check_no_match() {
        let test = *b"fghij";
        let result = mock_setter().check(test);
        let mut n: usize = 0;
        for i in result {
            assert_eq!(i, Clue::Wrong(test[n]));
            n += 1;
        }
    }

    #[test]
    fn check_1_match() {
        let result = mock_setter().check(*b"fbhij");
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
        let result = mock_setter().check(*b"fbhcj");
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
        let result = mock_setter().check(*b"fgahj");
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
        let result = mock_setter().check(*b"bbbbb");
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
    fn check_exact_overides_elsewhere() {
        let result = Setter::from_word(*b"abcde").check(*b"hccij");
        assert_eq!(
            result,
            [
                Clue::Wrong(b'h'),
                Clue::Wrong(b'c'),
                Clue::Right(b'c'),
                Clue::Wrong(b'i'),
                Clue::Wrong(b'j')
            ]
        );
    }

    #[test]
    fn real_world() {
        let result = Setter::from_word(*b"maybe").check(*b"cable");
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
