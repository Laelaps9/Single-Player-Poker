use single_player_poker as poker;

fn main() {
    let mut deck = poker::generate_deck();
    let cards = poker::deal(&mut deck);

    println!("Your cards:");
    for (i, card) in cards.iter().enumerate() {
        println!("{}) {}", i, card);
    }


}
