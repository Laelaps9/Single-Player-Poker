use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use single_player_poker as poker;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap
    },
    Terminal,
};
use std::io;

// Types of events
enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Screen {
    Welcome,
    Game,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // stdin won't be printed and input isn't buffered
    enable_raw_mode().expect("can run in raw mode");

    // mpsc channel to communicate between input handler and renderer
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    // input loop
    thread::spawn(move || {
       let mut last_tick = Instant::now();
       loop {
           let timeout = tick_rate
               .checked_sub(last_tick.elapsed())
               .unwrap_or_else(|| Duration::from_secs(0));

           if event::poll(timeout).expect("poll works") {
               if let CEvent::Key(key) = event::read().expect("can read events") {
                   tx.send(Event::Input(key)).expect("can send events");
               }
           }

           if last_tick.elapsed() >= tick_rate {
               if let Ok(_) = tx.send(Event::Tick) {
                   last_tick = Instant::now();
               }
           }
       }
    });

    // Set up and clear TUI terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut active_screen = Screen::Welcome;
    let mut game_active = false;
    let mut deck: Vec<u8> = poker::generate_deck();
    let mut hand: Vec<poker::Card> = vec![];
    let mut to_change: Vec<usize> = vec![];
    let mut discarded: Vec<u8> = vec![];
    let mut score = 0;
    let mut points = 0;
    let _poker_hand: &str;
    
    // Stateful list where cards will be stored
    let mut hand_list_state = ListState::default();
    hand_list_state.select(Some(0));

    // Render loop
    loop {
        // Terminal is separated vertically into 3 sections
        // header, body, and footer
        terminal.draw(|rect| {
            let size = rect.size();
            let constraints = match active_screen {
                Screen::Welcome => vec![Constraint::Min(20)],
                Screen::Game => {
                    vec![
                        Constraint::Length(4),
                        Constraint::Min(4),
                        Constraint::Length(5),
                        Constraint::Length(9),
                    ]
                }
            };

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(constraints.as_ref())
                .split(size);

            match active_screen {
                Screen::Welcome => {
                    rect.render_widget(render_welcome(), chunks[0]);
                },
                Screen::Game => {
                    let help = render_help();
                    let score = render_score(score);

                    let poker_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(40), Constraint::Percentage(60)].as_ref(),
                        )
                        .split(chunks[1]);
                    let game = render_game(&mut hand, &to_change);
                    let ascii_card = render_ascii_card(
                        &hand[hand_list_state.selected().unwrap()].rank,
                        &hand[hand_list_state.selected().unwrap()].suit,
                        );
                    rect.render_stateful_widget(game, poker_chunks[0], &mut hand_list_state);
                    rect.render_widget(ascii_card, poker_chunks[1]);

                    if !game_active {
                        let (message, _poker_hand) = render_message(&points);
                        rect.render_widget(message, chunks[2]);
                    }

                    rect.render_widget(score, chunks[0]);
                    rect.render_widget(help, chunks[3]);
                },
            }
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Down => {
                    if let Some(selected) = hand_list_state.selected() {
                        hand_list_state.select(Some((selected + 1) % 5))
                    }
                },
                KeyCode::Up => {
                    if let Some(selected) = hand_list_state.selected() {
                        if selected > 0 {
                            hand_list_state.select(Some(selected - 1));
                        } else {
                            hand_list_state.select(Some(4));
                        }
                    }
                },
                KeyCode::Enter => {
                    if !game_active {
                        active_screen = Screen::Game;
                        game_active = true;
                        hand = poker::deal(&mut deck)
                    } else {
                        if to_change.len() > 0 {
                            discarded = poker::change_cards(&mut deck, &mut hand, &to_change);
                            to_change.clear();
                        }
                        points = poker::check_hand(&hand);
                        score += points;
                        game_active = false;
                        poker::reset_deck(&mut deck, &mut hand, &mut discarded);

                    }
                },
                KeyCode::Char(' ') => {
                    if game_active {
                        let selection = hand_list_state.selected().unwrap();

                        if to_change.contains(&selection) {
                            to_change.retain(|i| i != &selection);
                        } else {
                            if to_change.len() == 3 {
                                to_change.pop();
                            }
                            to_change.push(selection);
                        }
                    }
                }
                _ => {},
            }
            Event::Tick => {},
        }
    }

    Ok(())
}

fn render_ascii_card<'a>(rank: &String, suit: &String) -> Paragraph<'a> {
    let suit_symbol = match &suit[..] {
        "Spades" => "♠",
        "Hearts" => "♥",
        "Diamonds" => "♦",
        "Clubs" => "♣",
        _ => panic!("Error"),
    };

    let top;
    let mid;
    let bot;

    if rank == "10" {
        top = format!("│{rank}               │");
        bot = format!("│               {rank}│");
    } else {
        top = format!("│{rank}                │");
        bot = format!("│                {rank}│");
    }

    mid = format!("│        {suit_symbol}        │");

    let card = Paragraph::new(vec![
        Spans::from(vec![Span::raw("╭─────────────────╮")]),
        Spans::from(vec![Span::raw(top)]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw(mid)]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw(bot)]),
        Spans::from(vec![Span::raw("╰─────────────────╯")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain)
    );

    card
}

fn render_game<'a>(hand: &mut Vec<poker::Card>,
    to_change: &Vec<usize>) -> List<'a> {

    // Game block
    let game = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain);

    let mut strings: Vec<String> = vec![];
    for i in 0..5 {
        let mut string = hand[i].get_card();
        if to_change.contains(&i) {
            string.push('*');
        }
        strings.push(string);
    }

    let cards: Vec<_> = strings
        .iter()
        .map(|card| {
            ListItem::new(Spans::from(vec![Span::styled(
                        card.clone(),
                        Style::default(),
            )]))
        })
        .collect();

    let list = List::new(cards).block(game).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    list
}

fn render_help<'a>() -> Paragraph<'a> {
    let help = Paragraph::new(vec![
        Spans::from(vec![Span::raw("You are dealt 5 cards.")]),
        Spans::from(vec![Span::raw("Use the up/down arrow keys to move between cards")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Up to 3 cards can be changed.")]),
        Spans::from(vec![Span::raw("Press 'space' to select/deselct a card.")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("When done, press enter to get your new cards and score")]),
    ])
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Help")
            .border_type(BorderType::Rounded),
    );

    help
}

fn render_message<'a>(points: &i32) -> (Paragraph<'a>, &str) {
    let poker_hand = match points {
        1 => "Pair!",
        3 => "Two pair!",
        5 => "Three of a kind!",
        10 => "Straight!",
        20 => "Four of a kind!",
        _ => "Nothing!"
    };

    let points_added = format!("+{}", points.to_string());

    let message = Paragraph::new(vec![
        Spans::from(vec![Span::styled(
            poker_hand,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        )]),
        Spans::from(vec![Span::styled(
            points_added,
            Style::default().fg(Color::Red)
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press enter to be dealt again")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
    );

    (message, poker_hand)
}

fn render_score<'a>(s: i32) -> Paragraph<'a> {
    let score = Paragraph::new(vec![
        Spans::from(vec![Span::raw("Score")]),
        Spans::from(vec![Span::styled(
            s.to_string(),
            Style::default().fg(Color::Red),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded),
    );

    score
}

fn render_welcome<'a>() -> Paragraph<'a> {
    let welcome = Paragraph::new(vec![
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Single Player Poker",
            Style::default().fg(Color::Cyan),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("╭─────────────────╮")]),
        Spans::from(vec![Span::raw("│A                │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│        ♠        │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                 │")]),
        Spans::from(vec![Span::raw("│                A│")]),
        Spans::from(vec![Span::raw("╰─────────────────╯")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "♠ Press Enter to play ♠",
            Style::default()
                .bg(Color::LightGreen)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    );

    welcome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_nothing() {
        let (_par, poker_hand) = render_message(&0);
        assert_eq!("Nothing!", poker_hand);
    }

    #[test]
    fn display_pair() {
        let (_par, poker_hand) = render_message(&1);
        assert_eq!("Pair!", poker_hand);
    }

    #[test]
    fn display_two_pair() {
        let (_par, poker_hand) = render_message(&3);
        assert_eq!("Two pair!", poker_hand);
    }

    #[test]
    fn display_three_of_a_kind() {
        let (_par, poker_hand) = render_message(&5);
        assert_eq!("Three of a kind!", poker_hand);
    }

    #[test]
    fn display_straight() {
        let (_par, poker_hand) = render_message(&10);
        assert_eq!("Straight!", poker_hand);
    }

    #[test]
    fn display_four_of_a_kind() {
        let (_par, poker_hand) = render_message(&20);
        assert_eq!("Four of a kind!", poker_hand);
    }
}
