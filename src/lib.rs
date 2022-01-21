use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

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
            _ => panic!("Error"),
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
            _ => panic!("Error"),
        };

        (rank, self.suit.clone())
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
    let mut suits = HashMap::new();
    let mut ranks = HashMap::new();
    let mut rank_keys: Vec<u8> = vec![];
    let mut rank_values: Vec<i32> = vec![];

    for card in hand {
        let suit_counter = suits.entry(&card.suit).or_insert(0);
        let rank_counter = ranks.entry(&card.rank).or_insert(0);

        *suit_counter += 1;
        *rank_counter += 1;
    }

    let ranks_iter = ranks.into_iter();

    for rank in ranks_iter {
        // rank_keys represents the ranks themselves
        // in a vec
        rank_keys.push(*rank.0);

        // rank_values represents how many times
        // each rank repeated
        rank_values.push(rank.1);
    }

    // Four of a kind -> +20 points
    if rank_values.contains(&4) {
        return 20;
    }

    // Full house

    // Flush
    let suit_keys: Vec<&String> = suits.into_keys().collect();
    let flush_found = suit_keys.len() == 1;

    // Straight
    let mut straight_found = false;
    if rank_keys.len() == 5 {
        rank_keys.sort();
        straight_found = straight(&rank_keys[..]);

        // If not found and there's an ace in the hand
        // check again counting the ace as its high value
        if !straight_found && rank_keys.contains(&1) {
            rank_keys.push(14);
            straight_found = straight(&rank_keys[1..]);
        }
    }

    // Match to find straight flush, just flush or just straight
    match (flush_found, straight_found) {
        (false, false) => {},
        (true, false) => return 15, // Flush -> +15 points
        (false, true) => return 10, // Straight -> +10 points
        (true, true) => {
            if rank_keys[rank_keys.len() - 1] == 14 {
                return 40; // Royal Flush -> +40 points
            }

            return 30; // Straight Flush -> +30 points
        }
    }

    // Three of a kind -> +5 points
    if rank_values.contains(&3) {
        return 5;
    }

    let mut count_pairs = 0;
    for r in rank_values {
        if r == 2 {
            count_pairs += 1;
        }
    }

    // Pairs
    let score = match count_pairs {
        1 => 1, // -> Pair -> +1 point
        2 => 3, // -> Two pair -> +3 points
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

pub fn straight(hand: &[u8]) -> bool {
    let mut past = hand[0] - 1;

    for card in hand {
        if past == card - 1 {
            past = *card;
        } else {
            return false;
        }
    }

    return true;
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
    fn hand_straight() {
        // First straight starts with ace and ends in 5
        let card1 = Card::new(1);
        let card2 = Card::new(15);
        let card3 = Card::new(29);
        let card4 = Card::new(43);
        let card5 = Card::new(44);
        let hand = vec![card1, card4, card2, card5, card3];

        assert_eq!(10, check_hand(&hand));
        
        // Second straight starts with 10 and ends in A
        let card1 = Card::new(23);
        let card2 = Card::new(24);
        let card3 = Card::new(25);
        let card4 = Card::new(26);
        let card5 = Card::new(1);
        let hand2 = vec![card5, card2, card1, card4, card3];

        assert_eq!(10, check_hand(&hand2));

        // Third hand doesn't have a straight
        let card1 = Card::new(2);
        let card2 = Card::new(3);
        let card3 = Card::new(4);
        let card4 = Card::new(5);
        let card5 = Card::new(5);
        let hand3 = vec![card5, card4, card2, card1, card3];

        assert_ne!(10, check_hand(&hand3));

        // Fourth hand doesn't have a straight
        let card1 = Card::new(11);
        let card2 = Card::new(12);
        let card3 = Card::new(13);
        let card4 = Card::new(14);
        let card5 = Card::new(15);
        let hand4 = vec![card5, card4, card2, card1, card3];

        assert_ne!(10, check_hand(&hand4));
    }

    #[test]
    fn hand_flush() {
        // All with the spades suit
        let card1 = Card::new(1);
        let card2 = Card::new(2);
        let card3 = Card::new(5);
        let card4 = Card::new(10);
        let card5 = Card::new(13);
        let hand = vec![card5, card4, card2, card1, card3];
    
        assert_eq!(15, check_hand(&hand));
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

    #[test]
    fn hand_straight_flush() {
        let card1 = Card::new(16);
        let card2 = Card::new(17);
        let card3 = Card::new(18);
        let card4 = Card::new(19);
        let card5 = Card::new(20);
        let hand = vec![card5, card4, card2, card1, card3];

        assert_eq!(30, check_hand(&hand));
    }

    #[test]
    fn hand_royal_flush() {
        let card1 = Card::new(40);
        let card2 = Card::new(49);
        let card3 = Card::new(50);
        let card4 = Card::new(51);
        let card5 = Card::new(52);
        let hand = vec![card5, card4, card2, card1, card3];

        assert_eq!(40, check_hand(&hand));

        let card1 = Card::new(27);
        let card2 = Card::new(49);
        let card3 = Card::new(50);
        let card4 = Card::new(51);
        let card5 = Card::new(52);
        let hand2 = vec![card5, card4, card2, card1, card3];

        assert_ne!(40, check_hand(&hand2));
    }
}
