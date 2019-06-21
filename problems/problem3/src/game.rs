use rand::prelude::*;

use std::cmp::Ordering;

use crate::game_element::*;


/// A structure that tracks wins/losses/ties for someone playing our game.
#[derive(Debug)]
pub struct Game {

    player_wins: u32,
    player_losses: u32,
    player_ties: u32,

}


impl Game {

    /// Constructs a new `Game` instance.
    pub fn new() -> Game {
        Game {
            player_wins: 0,
            player_losses: 0,
            player_ties: 0,
        }
    }

    /// Generates a random `GameElement`.
    fn generate_choice(&self) -> GameElement {
        let mut rng = rand::thread_rng();

        rng.gen()
    }

    /// Takes a player's choice and plays one round of the game.
    pub fn play(&mut self, player: GameElement) {
        let computer_choice = self.generate_choice();

        println!("");
        print!("I choose {}. ", computer_choice);

        match player.cmp(&computer_choice) {
            Ordering::Less    => {
                self.player_losses += 1;
                println!("I win!");
            },
            Ordering::Greater => {
                self.player_wins += 1;
                println!("I can't believe I lost!");
            },
            Ordering::Equal   => {
                self.player_ties += 1;
                println!("No winner. Try again.")
            }
        }

    }
}
