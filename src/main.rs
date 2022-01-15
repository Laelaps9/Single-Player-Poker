use single_player_poker as poker;
use std::process;

fn main() {
    if let Err(e) = poker::run() {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
