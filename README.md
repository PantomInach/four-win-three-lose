# four-win-three-lose
Simple Tic-Tac-Toe inspired game on a 4x4 field. If you have four in a row, you win. Otherwise, if you have three in a row, you lose. Just like the game name says.

## Get it running
Make sure you have [rust](https://www.rust-lang.org/) installed.

### Running in Project
Clone the repository and run `cargo run --release`.

### Installing to your System
Clone the repository, run `cargo install --path .`, and enjoy by starting the game with `four-win-three-lose`.

## Options
Per default, a GUI version is started. Optional you can also play in the terminal. See more with `four-win-three-lose --help`.
| Game Mode | Argument |
| --- | --- |
| Start GUI | `gui` |
| In Terminal: Human VS Human | `term-h-vs-h`|
| In Terminal: Computer VS Human | `term-c-vs-h`|
| In Terminal: Human VS Computer | `term-h-vs-c`|

# TODOs
- [ ] Make the computer more resilient against losing
- [ ] Make terminal and gui easier on the eyes
