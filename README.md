# hmode
Hard-mode Wordle solver written in Rust.

## About
This is yet another Wordle solver, written more as an exercise in using Rust for a fun problem than for its value to others. Some of the key features include:

- Command line interface giving access to mutliple features
- A demo mode, where the user provides a word and the solver tries to guess it
- A stats calculation mode allowing the user to provide different starting words, with the solver analysing the mean number of guesses across the whole Wordle answer set using the given start word, and listing the words that required more than 6 guesses.
- TODO - an interactive play mode where hmode chooses a secret word and the user tries to solve it.
- TODO - an "assistant" mode, where the solver proposes guesses to a user playing the official Wordle game.
## Design

hmode is written to set and solve Wordle puzzles exclusively in "hard mode", where each guess must use all of the letters revealed by previous guesses. For humans playing this way, this makes it easy to get trapped in a position where part of the word is known, and the there are too few remaining guesses to allow the user to find the solution.
