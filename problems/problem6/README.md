# Problem 6

`problem6` is a (simplified) "Blackjack" card game. There are a variety
of structs and enums representing relevant concepts, like Cards, Decks,
Hands, etc.

Note: The author of this exercise is not a card player, and apologizes
in advance for making up his own rules.

## game.rs

This module contains the gameplay logic, such as handling multiple rounds
and turns, prompting the user to hit or stand, determining the winner of
each round, and so forth.  For purposes of this exercise, you can leave
it as-is, unless of course you want to make your own improvements.

## hand.rs

This module holds the Hand struct and related methods.  Some of the behavior
is specific to Blackjack, which is the main reason this code is separate
from the more generic items in the `cards` module.

The follow method is missing an implementation:

* `pub fn get_value(&self) -> u8`

This method should add up and return the value of the cards in the hand.
Cards 2-10 are worth their face value.  Kings, queens, and jacks are
worth 10. An Ace is counted as 11, _unless_ doing so would bring the total
over 21, in which case it should be counted as 1.

## cards.rs

This module defines the `Deck` struct, which contains `Card` structs.  A card
is composed of `Rank` and `Suit` enums.

The following implementations are missing or broken:

* `impl fmt::Display for Suit`
* `impl fmt::Display for Rank`
* `fn load(&mut self)`
* `pub fn shuffle(&mut self)`

See the code for more information.

Optional exercise: Write a test for the `shuffle` method.  How do you define randomness?

## testing

You can run the tests with:

```
cargo test --package problem6
```

Once you've implemented the missing functions and all the tests are passing,
you can play the game by running:

```
cargo run --package problem6
```
