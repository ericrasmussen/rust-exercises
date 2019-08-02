use std::io;
use std::io::Write;

use crate::cards::Deck;
use crate::cards::Card;
use crate::hand::Hand;

pub struct Game {
    deck: Deck,
    dealer: Hand,
    player: Hand,
}

enum Winner {
    Dealer,
    Player,
    Tie
}

impl Game {

    pub fn new() -> Self {
        Self {
            deck: Deck::new(),
            dealer: Hand::new(),
            player: Hand::new(),
        }
    }

    pub fn play(&mut self) {
        self.deck.shuffle();
        loop {
            self.play_round();

            print!("(p)lay or (q)uit: ");
            io::stdout().flush().unwrap();
            let mut choice = String::new();
            io::stdin().read_line(&mut choice)
                .expect("read error");
            if choice.starts_with("q") {
                break;
            }
            println!();
        }
    }

    fn play_round(&mut self) {
        // dealer and player both start with two cards
        self.deal_card_to_player();
        self.deal_card_to_dealer();
        self.deal_card_to_player();
        self.deal_card_to_dealer();

        // display the initial hands
        self.dealer_status();
        self.player_status();

        // player takes as many cards as they wish,
        // followed by the dealer
        self.player_turn();
        self.dealer_turn();

        // determine the winner of the round
        let winner = self.determine_winner();
        Self::announce_winner(winner);

        // discard the hands
        self.player.clear();
        self.dealer.clear();
    }

    fn deal_card_to_player(&mut self) {
        let card = self.deal_card();
        self.player.add_card(card);
    }

    fn deal_card_to_dealer(&mut self) {
        let card = self.deal_card();
        self.dealer.add_card(card);
    }

    fn deal_card(&mut self) -> Card {
        match self.deck.deal_card() {
            Some(card) => card,
            None => {
                // all the cards have been dealt, so reset the
                // deck to the initial full, shuffled state
                self.deck.reset();
                self.deck.shuffle();
                match self.deck.deal_card() {
                    Some(card) => card,
                    None => Card::default(), // should never happen
                }
            }
        }
    }

    fn player_turn(&mut self) {
        loop {
            print!("(h)it or (s)tand: ");
            io::stdout().flush().unwrap();
            let mut choice = String::new();
            io::stdin().read_line(&mut choice)
                .expect("read error");
            println!();

            if choice.starts_with("h") {
                // hit
                self.deal_card_to_player();
                self.player_status();
                if self.player.is_done() {
                    break;
                }

            } else if choice.starts_with("s") {
                // stand
                break;
            }
        }
    }

    fn dealer_turn(&mut self) {
        // dealer wins automatically if the player busted
        if self.player.is_busted() {
            return;
        }

        loop {
            // dealer always stands on 17 or higher
            if (self.dealer.get_value() > self.player.get_value()) || (self.dealer.get_value() >= 17)  {
                break;
            }
            self.deal_card_to_dealer();
            self.dealer_status();
            if self.dealer.is_done() {
                break;
            }
        }
    }

    fn determine_winner(&self) -> Winner {
        let mut dealer_won = false;
        let mut player_won = false;
        if self.player.is_busted() {
            dealer_won = true;
        } else if self.dealer.is_busted() {
            player_won = true;
        } else if self.dealer.get_value() > self.player.get_value() {
            dealer_won = true;
        } else if self.dealer.get_value() < self.player.get_value() {
            player_won = true;
        }
        if dealer_won {
            Winner::Dealer
        } else if player_won {
            Winner::Player
        } else {
            Winner::Tie
        }
    }

    fn dealer_status(&self) {
        Self::status("dealer", &self.dealer);
    }

    fn player_status(&self) {
        Self::status("player", &self.player);
    }

    fn status(title: &str, hand: &Hand) {
        if hand.is_blackjack() {
            println!("{}: {} ({})", title, hand, "blackjack");
        } else if hand.is_busted() {
            println!("{}: {} ({})", title, hand, "busted");
        } else {
            println!("{}: {} ({})", title, hand, hand.get_value());
        }
    }

    fn announce_winner(winner: Winner) {
        match winner {
            Winner::Dealer => { println!("dealer wins!"); },
            Winner::Player => { println!("you win!"); },
            Winner::Tie => { println!("tie score!"); },
        }
    }
}
