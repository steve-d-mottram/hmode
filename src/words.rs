pub type WdlWord = &'static [u8; 5];

pub static DEFAULT_START_WORD: WdlWord = b"tares";

// Import the static word lists generated by build.rs
include!(concat!(env!("OUT_DIR"), "/words-generated.rs"));

pub fn all_words(alt_words: bool) -> Vec<WdlWord> {
    if alt_words {
        ALT_WORDS.to_vec()
    } else {
        ALL_WORDS.to_vec()
    }
}

pub fn answer_words() -> Vec<WdlWord> {
    ALL_WORDS[0..ANSWER_WORDS_END].to_vec()
}

/// Validates that the provided word is in the list of all allowed
/// words, and returns a static reference to the word in the list.
/// This simplifies lifetime management for client code.
pub fn to_static_word(word: &str, answers_only: bool, alt_words: bool) -> Result<WdlWord, String> {
    if word.len() != 5 {
        return Err("Word must have 5 letters".into());
    }
    let list = if answers_only {
        answer_words()
    } else {
        all_words(alt_words)
    };
    // Copy the letters in word into a byte buffer
    let mut temp: [u8; 5] = [0; 5];
    word.bytes().zip(temp.iter_mut()).for_each(|(b, p)| *p = b);
    // Return the matching word in the static
    // probe word list, or an error if it's not in the list.
    if let Some(entry) = list.iter().find(|&&&w| w == temp) {
        Ok(entry)
    } else {
        Err(format!("The word '{}' is not in the list of valid words", word))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_static_word_finds_known_word() {
        assert_eq!(to_static_word("grass", false, false), Ok(b"grass"));
    }

    #[test]
    #[should_panic]
    fn to_static_word_checks_length() {
        to_static_word("wibble", false, false).unwrap();
    }

    #[test]
    #[should_panic]
    fn to_static_word_invalid_word() {
        to_static_word("xxxxx", false, false).unwrap();
    }

    #[test]
    #[should_panic]
    fn to_static_word_detects_probe_word_used_as_answer_word() {
        to_static_word("caber", true, false).unwrap();
    }
}
