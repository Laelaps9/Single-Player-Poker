use std::collections::HashMap;

pub fn generate_deck() -> HashMap<String, Vec<char>> {
    let mut deck = HashMap::new();
    let mut cards = vec!['A', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'J', 'Q', 'K'];

    deck.insert("Clubs".to_string(), cards.clone());
    deck.insert("Hearts".to_string(), cards.clone());
    deck.insert("Spades".to_string(), cards.clone());
    deck.insert("Diamonds".to_string(), cards.clone());

    return deck;
}
