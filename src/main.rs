use single_player_poker as poker;

fn main() {
    let mut deck = poker::generate_deck();

    let cards = poker::deal(deck);

    for card in cards {
        println!("{:?}", card);
    }
}
