use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;

const ANSWER_WORDS_PATH: &str = "data/answer-words.txt";
const PROBE_WORDS_PATH: &str = "data/probe-words.txt";
const ALT_PROBE_WORDS_PATH: &str = "data/alt-probe-words.txt";

fn read_file(source: &str) -> Result<Vec<String>, String> {
    let l = fs::read_to_string(source)
        .map_err(|e| format!("Error reading file {}, {}", source, e))?
        .lines()
        .map(String::from)
        .collect();
    Ok(l)
}

fn list_to_static(words: Vec<String>) -> String {
    let mut text = String::with_capacity(words.len() * 10);
    for word in words {
        text.push_str(&format!("b\"{}\", ", word));
    }
    text
}

/// Reads in a file containing answer words and a file containing probe words.
/// The words in the two files are combined so that the answer words are at the
/// front of the resulting list, with the probe words following. The function
/// returns a Result containing the resulting vector, and the index of the start
/// of the probe words. If an error occurs, an Err(String) is returned with an
/// error message.
fn get_word_list(answer_file: &str, probe_file: &str) -> Result<(Vec<String>, usize), String> {
    // The probe words are placed in a BTreeSet as they may contain duplicates
    // from the Wordle answer list. This allows us to remove them more easily
    let mut answer_words: Vec<String> = read_file(answer_file)?;
    let answer_words_len = answer_words.len();
    let mut probe_words: BTreeSet<String> = read_file(probe_file)?.into_iter().collect();
    for word in &answer_words {
        _ = probe_words.remove(word)
    }
    answer_words.extend(probe_words);

    Ok((answer_words, answer_words_len))
}

fn main() -> Result<(), String> {
    let (normal_words, answer_words_len) = get_word_list(ANSWER_WORDS_PATH, PROBE_WORDS_PATH)?;
    let (alt_words, _) = get_word_list(ANSWER_WORDS_PATH, ALT_PROBE_WORDS_PATH)?;

    let out_dir = env::var_os("OUT_DIR").ok_or("Could not read environment variable 'OUT_DIR'")?;
    let dest_path = Path::new(&out_dir).join("words-generated.rs");
    fs::write(
        &dest_path,
        format!(
            r#"
static ALL_WORDS: &[WdlWord] = &[{} ];
static ALT_WORDS: &[WdlWord] = &[{} ];
static ANSWER_WORDS_END : usize = {};
    "#,
            list_to_static(normal_words),
            list_to_static(alt_words),
            answer_words_len
        ),
    )
    .map_err(|e| String::from(format!("{}", e)))?;
    println!("cargo:rerun-if-changed=data/");
    Ok(())
}
