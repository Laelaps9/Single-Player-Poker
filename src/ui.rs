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
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
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

// Allows for use in the Tabs component 
// to highlight current tab
impl From<Screen> for usize {
    fn from(input: Screen) -> usize {
        match input {
            Screen::Welcome => 0,
            Screen::Game => 1,
        }
    }
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

    let menu_titles = vec!["Home", "Poker", "Tutorial"];
    let mut active_menu_item = Screen::Welcome;
    let mut game_active = false;
    let mut deck: Vec<u8> = poker::generate_deck();
    let mut hand: Vec<poker::Card> = vec![];
    let mut to_change: Vec<usize> = vec![];
    let mut discarded: Vec<u8> = vec![];
    let mut score = 0;
    
    // Stateful list where cards will be stored
    let mut hand_list_state = ListState::default();
    hand_list_state.select(Some(0));

    // Render loop
    loop {
        // Terminal is separated vertically into 3 sections
        // header, body, and footer
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let footer = Paragraph::new("Single Player Poker")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Help")
                        .border_type(BorderType::Plain),
                );

            // Tabs no longer needed
            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);

            if game_active {
                let poker_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [Constraint::Percentage(20), Constraint::Percentage(90)].as_ref(),
                    )
                    .split(chunks[1]);
                let (score, game) = render_game(&hand_list_state, &mut hand, &to_change, &score);
                rect.render_widget(score, poker_chunks[0]);
                rect.render_stateful_widget(game, poker_chunks[1], &mut hand_list_state);

            } else {
                match active_menu_item {
                    Screen::Welcome => {
                        rect.render_widget(render_welcome(), chunks[1]);
                    }
                    Screen::Game => {},                }
            }

            rect.render_widget(footer, chunks[2]);
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
                        game_active = true;
                        hand = poker::deal(&mut deck)
                    } else {
                        if to_change.len() > 0 {
                            discarded = poker::change_cards(&mut deck, &mut hand, &to_change);
                            to_change.clear();
                        }
                        score += poker::check_hand(&hand);
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

fn render_welcome<'a>() -> Paragraph<'a> {
    let welcome = Paragraph::new(vec![
        Spans::from(vec![Span::raw("Welcome")]),
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

fn render_game<'a>(hand_list_state: &ListState,
    hand: &mut Vec<poker::Card>,
    to_change: &Vec<usize>,
    score: &i32) -> (Paragraph<'a>, List<'a>) {
    let score = Paragraph::new(vec![
        Spans::from(vec![Span::raw("Score")]),
        Spans::from(vec![Span::styled(
            score.to_string(),
            Style::default().fg(Color::Red),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain),
    );

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

    let selected_card = hand
        .get(
            hand_list_state
                .selected()
                .expect("always a card selected"),
        )
        .expect("exists")
        .clone();

    let list = List::new(cards).block(game).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    (score, list)
}
