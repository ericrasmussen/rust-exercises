use std::io;
use problem3::game_element::GameElement;
use problem3::game::Game;


// Console-based rock/paper/scissors game. Make sure to fix the broken
// code in `game_element.rs`.
fn main() {
    println!("Let's play rock paper scissors!");

    let mut game: Game = Game::new();

    loop {
        println!("Choose (r)ock, (p)aper, or (s)cissors: ");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        let choice: GameElement = match choice.parse() {
            Ok(c)  => c,
            Err(_) => {
                println!("Please enter (r)ock, (p)aper, or (s)cissors");
                continue
            },
        };

        // plays one round of the game and prints results
        game.play(choice);
        println!("Results: {:?}", game);
        println!("");

    }
}
