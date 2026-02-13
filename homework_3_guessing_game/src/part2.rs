use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let guess = (min + max) / 2;
        let temp = player.ask_to_compare(guess);
        if temp == 0 {
            return guess;
        } else if temp == 1 {
            return Self::guess_the_number(player, guess, max); //Google searched how to do recursion with Rust
        } else if temp == -1 {
            return Self::guess_the_number(player, min, guess);
        } else {
            return 0;
        }
    }
}
