use single_player_poker as poker;
use std::process;

mod ui;

fn main() {
    //if let Err(e) = poker::run() {
    if let Err(e) = ui::run() {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
