use std::fmt;
use crate::cards::Card;
use crate::cards::Rank;

pub struct Hand {
    cards: Vec<Card>
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::new()
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn is_busted(&self) -> bool {
        self.get_value() > 21
    }

    pub fn is_blackjack(&self) -> bool {
        self.get_value() == 21
    }

    pub fn is_done(&self) -> bool {
        self.is_busted() || self.is_blackjack()
    }

    /// Return the numeric value of the hand.
    pub fn get_value(&self) -> u8 {
        // *** IMPLEMENT THIS METHOD ***
        // Note that you can leverage the underlying rank.value() method of
        // the cards, but you'll need some special handling to decide when
        // to count an ace as 1 vs. 11
        0
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in self.cards.iter() {
            write!(f, "{}", card)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::cards::Suit;

    fn make_hand(cards: &[Card]) -> Hand {
        let mut hand = Hand::new();
        for &card in cards.iter() {
            hand.add_card(card);
        }
        hand
    }

    #[test]
    fn test_empty_hand() {
        let hand = make_hand(&[]);
        assert_eq!(hand.get_value(), 0);
    }

    #[test]
    fn test_one_card() {
        let hand = make_hand(&[
            Card { rank: Rank::Two, suit: Suit::Clubs }
        ]);
        assert_eq!(hand.get_value(), 2);
    }

    #[test]
    fn test_one_ace() {
        let hand = make_hand(&[
            Card { rank: Rank::Ace, suit: Suit::Spades }
        ]);
        assert_eq!(hand.get_value(), 11);
    }

    #[test]
    fn test_two_aces() {
        let hand = make_hand(&[
            Card { rank: Rank::Ace, suit: Suit::Spades },
            Card { rank: Rank::Ace, suit: Suit::Hearts },
        ]);
        assert_eq!(hand.get_value(), 12);
    }

    #[test]
    fn test_blackjack() {
        let hand = make_hand(&[
            Card { rank: Rank::Queen, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
        ]);
        assert_eq!(hand.get_value(), 21);
    }

    #[test]
    fn test_bust() {
        let hand = make_hand(&[
            Card { rank: Rank::Ten, suit: Suit::Diamonds },
            Card { rank: Rank::Seven, suit: Suit::Hearts },
            Card { rank: Rank::Ace, suit: Suit::Diamonds },
            Card { rank: Rank::Five, suit: Suit::Spades },
        ]);
        assert_eq!(hand.get_value(), 23);
    }
}
