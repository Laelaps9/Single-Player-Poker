use std::error::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: String,
    pub value: u8,
}

impl Card {
    pub fn new(v: u8) -> Card {
        let value = v;

        let suit = match (value - 1) / 13 {
            0 => "Spades".to_string(),
            1 => "Hearts".to_string(),
            2 => "Diamonds".to_string(),
            3 => "Clubs".to_string(),
            _ => panic!("Error"),
        };

        Card { suit, value }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tmp = self.value % 13;
        let mut _r = String::new();
        let rank = match tmp {
            1 => "A",
            2..=10 => {
                _r = tmp.to_string();
                _r.as_str()
            }
            11 => "J",
            12 => "Q",
            0 => "K",
            _ => panic!("Error"),
        };

        write!(f, "{} of {}", rank, self.suit)
    }
}


pub fn deal(deck: &mut Vec<u8>) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];
    let mut rng = thread_rng();

    deck.shuffle(&mut rng);

    for _i in 0..5 {
        let card_val = deck.pop().unwrap();

        cards.push(Card::new(card_val));
    }

    return cards;
}

pub fn generate_deck() -> Vec<u8> {
    return (1..53).collect::<Vec<u8>>();
}


pub fn run() -> Result<(), Box<dyn Error>> {
    let mut deck = generate_deck();
    let cards = deal(&mut deck);

    println!("Your cards:");
    for (i, card) in cards.iter().enumerate() {
        println!("{}) {}", i, card);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests to check if the correct suit is given to the 
    // first and last card of each
    #[test]
    fn aces() {
        assert_eq!(
            Card::new(1),
            Card {
                suit: "Spades".to_string(),
                value: 1,
            }
        );
        
        assert_eq!(
            Card::new(13),
            Card {
                suit: "Spades".to_string(),
                value: 13,
            }
        );
    }

    #[test]
    fn hearts() {
        assert_eq!(
            Card::new(14),
            Card {
                suit: "Hearts".to_string(),
                value: 14,
            }
        );
        
        assert_eq!(
            Card::new(26),
            Card {
                suit: "Hearts".to_string(),
                value: 26,
            }
        );
    }

    #[test]
    fn diamonds() {
        assert_eq!(
            Card::new(27),
            Card {
                suit: "Diamonds".to_string(),
                value: 27,
            }
        );
        
        assert_eq!(
            Card::new(39),
            Card {
                suit: "Diamonds".to_string(),
                value: 39,
            }
        );
    }

    #[test]
    fn clubs() {
        assert_eq!(
            Card::new(40),
            Card {
                suit: "Clubs".to_string(),
                value: 40,
            }
        );
        
        assert_eq!(
            Card::new(52),
            Card {
                suit: "Clubs".to_string(),
                value: 52,
            }
        );
    }
}
