use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Card {
    pub suit: String,
    pub value: char,
}

impl Card {
    pub fn new(s: String, val: char) -> Card {
        let suit = s.clone();
        let value = val;

        Card {
            suit,
            value,
        }
    }
}

pub fn generate_deck() -> HashMap<String, Vec<char>> {
    let mut deck = HashMap::new();
    let cards = vec!['A', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'J', 'Q', 'K'];

    deck.insert("Clubs".to_string(), cards.clone());
    deck.insert("Hearts".to_string(), cards.clone());
    deck.insert("Spades".to_string(), cards.clone());
    deck.insert("Diamonds".to_string(), cards.clone());

    return deck;
}

pub fn deal(mut deck: HashMap<String, Vec<char>>) -> Vec<Card> {
    let suits = ["Clubs", "Hearts", "Spades", "Diamonds"];
    let mut cards: Vec<Card> = vec![];

    for _i in 0..5 {
        let suit = suits.choose(&mut rand::thread_rng()).unwrap().to_string();
        let value = deck.get_mut(&suit).unwrap().choose(&mut rand::thread_rng()).unwrap().clone();
        cards.push(Card::new(suit, value));
    }

    return cards;
}
