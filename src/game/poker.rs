use rand::seq::SliceRandom;
use rand::thread_rng;
use single_player_poker::Card;
use std::collections::HashMap;
use std::process;

pub fn change_cards(deck: &mut Vec<u8>, hand: &mut Vec<Card>, to_change: &Vec<usize>) -> Vec<u8> {
    let mut discarded: Vec<u8> = vec![];

    // Removed cards are sent to the discarded pile
    // New cards are popped from the deck
    for i in to_change {
        discarded.push(hand.remove(*i).value);
        let new_card = deck.pop().unwrap_or_else(|| {
            eprintln!("Problem extracting card from deck");
            process::exit(1);
        });
        hand.insert(*i, Card::new(new_card));
    }

    return discarded;
}

pub fn check_hand(hand: &Vec<Card>) -> i32 {
    let mut suits = HashMap::new();
    let mut ranks = HashMap::new();
    let mut rank_keys: Vec<u8> = vec![];
    let mut ranks_count: Vec<i32> = vec![];

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

        // ranks_count represents how many times
        // each rank repeated
        ranks_count.push(rank.1);
    }

    // Four of a kind
    if ranks_count.contains(&4) {
        return 20;
    }

    // Full house

    // Four of a kind is the only other way
    // to have no more than 2 different ranks
    if ranks_count.len() == 2 {
        return 18;
    }

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
        (false, false) => {}
        (true, false) => return 15, // Flush
        (false, true) => return 10, // Straight
        (true, true) => {
            if rank_keys[rank_keys.len() - 1] == 14 {
                return 40; // Royal Flush
            }

            return 30; // Straight Flush
        }
    }

    // Three of a kind
    if ranks_count.contains(&3) {
        return 5;
    }

    let mut count_pairs = 0;
    for r in ranks_count {
        if r == 2 {
            count_pairs += 1;
        }
    }

    // Pairs
    match count_pairs {
        1 => return 1, // Pair
        2 => return 3, // Two pair
        _ => return 0, // Nothing
    };
}

pub fn deal(deck: &mut Vec<u8>) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];
    let mut rng = thread_rng();

    deck.shuffle(&mut rng);

    for _i in 0..5 {
        let card_val = deck.pop().unwrap_or_else(|| {
            eprintln!("Problem extracting card from deck");
            process::exit(1);
        });
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

#[cfg(test)]
mod tests {
    use super::*;

    // Functions tests
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

    // Hand combinations tests
    #[test]
    fn hand_nothing() {
        let card1 = Card::new(10);
        let card2 = Card::new(8);
        let card3 = Card::new(42);
        let card4 = Card::new(17);
        let card5 = Card::new(26);
        let hand = vec![card1, card2, card3, card4, card5];

        assert_eq!(0, check_hand(&hand));
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
        // All have the spades suit
        let card1 = Card::new(1);
        let card2 = Card::new(2);
        let card3 = Card::new(5);
        let card4 = Card::new(10);
        let card5 = Card::new(13);
        let hand = vec![card5, card4, card2, card1, card3];

        assert_eq!(15, check_hand(&hand));
    }

    #[test]
    fn hand_full_house() {
        // All with the spades suit
        let card1 = Card::new(1); // A of spades
        let card2 = Card::new(14); // A of hearts
        let card3 = Card::new(27); // A of diamonds
        let card4 = Card::new(5); // 5 of spades
        let card5 = Card::new(44); // 5 of clubs
        let hand = vec![card5, card4, card2, card1, card3];

        assert_eq!(18, check_hand(&hand));

        let card1 = Card::new(1); // A of spades
        let card2 = Card::new(14); // A of hearts
        let card3 = Card::new(27); // A of diamonds
        let card4 = Card::new(40); // A of clubs
        let card5 = Card::new(44); // 5 of clubs
        let hand = vec![card5, card4, card2, card1, card3];

        assert_ne!(18, check_hand(&hand)); // Four of a kind is returned
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
        let card1 = Card::new(16); // 3 of hearts
        let card2 = Card::new(17); // 4 of hearts
        let card3 = Card::new(18); // 5 of heats
        let card4 = Card::new(19); // 6 of hearts
        let card5 = Card::new(20); // 7 of hearts
        let hand = vec![card5, card4, card2, card1, card3];

        assert_eq!(30, check_hand(&hand));
    }

    #[test]
    fn hand_royal_flush() {
        let card1 = Card::new(40); // A of clubs
        let card2 = Card::new(49); // 10 of clubs
        let card3 = Card::new(50); // J of clubs
        let card4 = Card::new(51); // Q of clubs
        let card5 = Card::new(52); // K of clubs
        let hand = vec![card5, card4, card2, card1, card3];

        assert_eq!(40, check_hand(&hand));

        let card1 = Card::new(27); // A of diamonds
        let card2 = Card::new(49);
        let card3 = Card::new(50);
        let card4 = Card::new(51);
        let card5 = Card::new(52);
        let hand2 = vec![card5, card4, card2, card1, card3];

        assert_ne!(40, check_hand(&hand2));
    }
}
