#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub suit: String,
    pub rank: u8,
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
            _ => panic!("Invalid suit value: {}", (value  -1) / 13),
        };

        let rank: u8 = if let 0 = value % 13 {
            13
        } else {
            value % 13
        };

        Card { suit, rank, value }
    }

    pub fn get_card(&self) -> (String, String) {
        let rank = match self.rank {
            1 => "A".to_string(),
            2..=10 => self.rank.to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            _ => panic!("Invalid rank: {}", self.rank),
        };

        (rank, self.suit.clone())
    }
}

pub fn generate_deck() -> Vec<u8> {
    return (1..53).collect::<Vec<u8>>();
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
                rank: 1,
                value: 1,
            }
        );

        assert_eq!(
            Card::new(13),
            Card {
                suit: "Spades".to_string(),
                rank: 13,
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
                rank: 1,
                value: 14,
            }
        );

        assert_eq!(
            Card::new(26),
            Card {
                suit: "Hearts".to_string(),
                rank: 13,
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
                rank: 1,
                value: 27,
            }
        );

        assert_eq!(
            Card::new(39),
            Card {
                suit: "Diamonds".to_string(),
                rank: 13,
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
                rank: 1,
                value: 40,
            }
        );

        assert_eq!(
            Card::new(52),
            Card {
                suit: "Clubs".to_string(),
                rank: 13,
                value: 52,
            }
        );
    }
}
