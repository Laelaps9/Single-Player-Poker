use std::process;

mod game;
mod ui;

fn main() {
    if let Err(e) = ui::run() {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
