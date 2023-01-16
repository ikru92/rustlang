mod guess_game;
use rand::prelude::*;

use guess_game::Guess;

fn main() {
    let guess = Guess {
        random_number: thread_rng().gen_range(0..100)
    };
    guess.GuessGame();
}
