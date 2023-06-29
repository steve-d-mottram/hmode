# hmode
Hard-mode Wordle solver written in Rust.

## About
This is yet another Wordle solver, written more as an exercise in using Rust for a fun problem than for its value
 to others. Some of the key features include:

- Command line interface giving access to mutliple features
- A demo mode, where the user provides a word and the solver tries to guess it
- A stats calculation mode allowing the user to provide different starting words, with the solver analysing
 the mean number of guesses across the whole Wordle answer set using the given start word, and listing the words that required more than 6 guesses.
- TODO - an interactive play mode where hmode chooses a secret word and the user tries to solve it.
- TODO - an "assistant" mode, where the solver proposes guesses to a user playing the official Wordle game.

## Design

hmode is written to set and solve Wordle puzzles exclusively in "hard mode", where each guess must use all of the
letters revealed by previous guesses. For humans playing this way, this makes it easy to get trapped in a position
where part of the word is known, and the there are too few remaining guesses to allow the user to find the solution
e.g. if the user has found -OUND, following guesses could be HOUND, WOUND, SOUND, BOUND, FOUND etc.

It turns out though that a suitable algorithm for hard mode is potentially simpler than for normal mode. This app
starts with a list of all allowed Wordle guesses (over 12,000), sorted so that the first ~2300 are the allowed 
answer words. There are two key components to hmode: the Setter, which knows the target word and provides Clues when
asked to score a Guess, and the Solver, which uses the Clues to refine its Guesses until it finds the target word. In
an online Wordle game, Wordle is the Setter, and the human player is the Solver. hmode has both components, which can
be combined together in different ways for different uses.

There are three stages that are used repeatedly in the solving algorithm:

- Guessing a word
- Scoring the Guess to generate Clues
- Filtering the list to eliminate words that don't match the Clues

These stages are repeated until the Clue indicates that the correct word has been found.

## Guessing
This algorithm turns out to be surprisingly simple, if computationally demanding.
The process involves finding the word that, if applied to all possible remaining answer words to generate Clues,
on average eliminates the largest proportion of the remaining word list. This requires A.len()^2 x F.len() comparisons, where
A is the list of allowed answers, and F is the full Wordle word list. This equates to approximately 63E10 calculations for
the first guess before filtering.

on a 2023 Intel i5 laptop, this is infeasibly slow for generating the first guess. However, since the first guess is made
from a position of no information, and the the algorithm converges very rapidly, hmode uses a pre-selected word for the first
guess, and then applies the algorithm from the second guess onwards, when the word list is much shorter after filtering using
the first Clue.

Although it is not computationally efficient to search for the very best starting word for this algorithm on the hardware 
available, hmode includes a mode that allows a proposed starting word to be tested for efficiency over the whole set 
of allowed Wordle answers, calculating the mean number of guesses to solve, and a list of outliers that could not be solved
in 6 or fewer guesses. This analysis takes about 6 minutes on the hardware available to the author.

## Performance
The current version of hmode uses the starting word "tares" and solves all Wordle answers in fewer than 6 guesses, with a
a mean of 2.94 guesses per word.




