use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub suit: String,
    pub rank: String,
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

        let tmp: u8 = value % 13;
        let rank = match tmp {
            1 => "A".to_string(),
            2..=10 => tmp.to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            0 => "K".to_string(),
            _ => panic!("Error"),
        };

        Card { suit, rank, value }
    }

    pub fn get_card(&self) -> String {
        return format!("{} of {}", self.rank, self.suit);
    }
}

pub fn change_cards(deck: &mut Vec<u8>, hand: &mut Vec<Card>, to_change: &Vec<usize>) -> Vec<u8> {
    let mut discarded: Vec<u8> = vec![];

    // Removed cards are sent to the discarded pile
    // New cards are popped from the deck
    for i in to_change {
        discarded.push(hand.remove(*i).value);
        hand.insert(*i, Card::new(deck.pop().unwrap()));
    }

    return discarded;
}

pub fn check_hand(hand: &Vec<Card>) -> i32 {
    //let mut suits = HashMap::new();
    let mut ranks = HashMap::new();

    for card in hand {
        //let counter = groups.entry(&card.suit).or_insert(0);
        let counter = ranks.entry(&card.value % 13).or_insert(0);
        *counter += 1;
    }

    let values: Vec<i32> = ranks.into_values().collect();

    // Four of a kind
    if values.contains(&4) {
        return 20;
    }

    // Straight

    // Three of a kind
    if values.contains(&3) {
        return 5;
    }

    let mut count_pairs = 0;
    for v in values {
        if v == 2 {
            count_pairs += 1;
        }
    }

    // Two pair
    let score = match count_pairs {
        1 => 1,
        2 => 3,
        _ => 0,
    };

    return score;
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

pub fn reset_deck(deck: &mut Vec<u8>, hand: &mut Vec<Card>, discarded: &mut Vec<u8>) {
    deck.append(discarded);

    for card in hand {
        deck.push(card.value);
    }

    discarded.clear();

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
                rank: "A".to_string(),
                value: 1,
            }
        );

        assert_eq!(
            Card::new(13),
            Card {
                suit: "Spades".to_string(),
                rank: "K".to_string(),
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
                rank: "A".to_string(),
                value: 14,
            }
        );

        assert_eq!(
            Card::new(26),
            Card {
                suit: "Hearts".to_string(),
                rank: "K".to_string(),
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
                rank: "A".to_string(),
                value: 27,
            }
        );

        assert_eq!(
            Card::new(39),
            Card {
                suit: "Diamonds".to_string(),
                rank: "K".to_string(),
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
                rank: "A".to_string(),
                value: 40,
            }
        );

        assert_eq!(
            Card::new(52),
            Card {
                suit: "Clubs".to_string(),
                rank: "K".to_string(),
                value: 52,
            }
        );
    }

    #[test]
    fn test_change() {
        let mut deck = generate_deck();
        let mut hand = deal(&mut deck);
        let hand_copy = hand.clone();
        let to_change: Vec<usize> = vec![0, 1, 4];
        let _discarded = change_cards(&mut deck, &mut hand, &to_change);

        // Deck has 5 cards removed from the dealing and
        // 3 more after changing cards
        assert_eq!(44, deck.len());

        // hand cards should be different after changing
        assert_ne!(hand_copy, hand);
    }

    #[test]
    fn test_deal() {
        let mut deck = generate_deck();
        let hand = deal(&mut deck);

        // Hands always contain 5 random cards
        assert_eq!(5, hand.len());

        // Hand cards are removed from the deck to avoid
        // duplicates in case of changing cards
        assert_eq!(47, deck.len());
    }

    #[test]
    fn test_reset() {
        let mut deck = generate_deck();
        let mut hand = deal(&mut deck);
        let to_change: Vec<usize> = vec![1, 2, 3];
        let mut discarded = change_cards(&mut deck, &mut hand, &to_change);

        reset_deck(&mut deck, &mut hand, &mut discarded);
        deck.sort();

        let deck2 = generate_deck();

        // After reset, deck should contain the same 
        // values it had when created
        assert_eq!(deck2, deck);

        // The discarded pile is cleared
        assert_eq!(0, discarded.len());
    }

    #[test]
    fn hand_pair() {
        let ace_one = Card::new(1); // Ace of spades
        let ace_two = Card::new(14); // Ace of hearts
        let card3 = Card::new(4);
        let card4 = Card::new(18);
        let card5 = Card::new(45);
        let hand = vec![ace_one, card3, card4, ace_two, card5];

        // A pair returns 1 point
        assert_eq!(1, check_hand(&hand));
    }

    #[test]
    fn hand_two_pair() {
        let k_one = Card::new(13); // K of spades
        let k_two = Card::new(26); // K of hearts
        let q_one = Card::new(51); // Q of clubs
        let q_two = Card::new(25); // Q of hearts
        let card5 = Card::new(2);
        let hand = vec![k_one, q_one, q_two, k_two, card5];

        // Two pairs return 3 points
        assert_eq!(3, check_hand(&hand));
    }

    #[test]
    fn hand_three_of_a_kind() {
        let five_one = Card::new(5); // 5 of spades
        let five_two = Card::new(31); // 5 of diamonds
        let five_three = Card::new(44); // 5 of clubs
        let card4 = Card::new(25);
        let card5 = Card::new(47);
        let hand = vec![five_one, card4, five_two, card5, five_three];

        // Three of a kind return 5 points
        assert_eq!(5, check_hand(&hand));
    }

    #[test]
    fn hand_four_of_a_kind() {
        let j_one = Card::new(11); // J of spades
        let j_two = Card::new(24); // J of hears
        let j_three = Card::new(37); // J of diamonds
        let j_four = Card::new(50); // J of clubs
        let card5 = Card::new(4);
        let hand = vec![j_one, j_two, j_three, card5, j_four];

        // Four of a kind return 20 points
        assert_eq!(20, check_hand(&hand));
    }
}
