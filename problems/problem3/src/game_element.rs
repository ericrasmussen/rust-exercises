use rand::distributions::Distribution;
use rand::distributions::Standard;
use std::cmp::Ordering;
use simple_error::SimpleError;
use std::str::FromStr;
use std::fmt;

/// The base type for representing Rock, Paper, and Scissors, which are all the
/// possible choices in our game.
#[derive(PartialEq, Eq)]
pub enum GameElement {
    Rock,
    Paper,
    Scissors,
}

/// Follows the standard ordering rules for rock paper scissors, where
/// rock < paper < scissors < rock.
///
/// # Examples
///
/// ```
/// use problem3::game_element::GameElement::*;
/// let comparison = Rock < Paper;
/// assert_eq!(comparison, true);
///```
impl Ord for GameElement {

    fn cmp(&self, other: &Self) -> Ordering {

        use GameElement::*;
        use Ordering::*;

        if self == other {
            return Ordering::Equal;
        }

        match (self, other) {
            (Rock, Paper)     => Less,
            (Paper, Scissors) => Less,
            (Scissors, Rock)  => Less,
            _ => Ordering::Greater,
        }
    }
}

impl PartialOrd for GameElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Allows callers to randomly generate game choices.
impl Distribution<GameElement> for Standard {

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GameElement {

        // randomly chooses 1, 2, or 3
        let n: u32 = rng.gen_range(1, 4);

        match n {
            1 => GameElement::Rock,
            2 => GameElement::Paper,
            _ => GameElement::Scissors,
        }

    }
}

/// Console-friendly string representation of each element.
impl fmt::Display for GameElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable_str = match self {
            GameElement::Rock     => "Rock".to_string(),
            GameElement::Paper    => "Paper".to_string(),
            GameElement::Scissors => "Scissors".to_string(),
        };
        write!(f, "{}", printable_str)
    }
}

/// For our game parser we'll accept any string that starts with r, p, or s
/// and convert it into Rock, Paper, or Scissors, respectively
impl FromStr for GameElement {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("r") {
            Ok(GameElement::Rock)
        } else if s.starts_with("p") {
            Ok(GameElement::Paper)
        } else if s.starts_with("s") {
            Ok(GameElement::Scissors)
        } else {
            Err(SimpleError::new("Choice must start with r, p, or s"))
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ordering() {
        use GameElement::*;

        assert!(Rock < Paper && Paper < Scissors && Scissors < Rock);
    }

    // add additional tests to make sure we can parse game elements from
    // strings and also display them

}
