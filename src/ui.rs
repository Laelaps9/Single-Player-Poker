use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
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

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Poker,
    Tutorial,
}

// Allows for use in the Tabs component 
// to highlight current tab
impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Poker => 1,
            MenuItem::Tutorial => 2,
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
    let mut active_menu_item = MenuItem::Home;

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

            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::Poker => {},
                MenuItem::Tutorial => {},
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
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('p') => active_menu_item = MenuItem::Poker,
                KeyCode::Char('t') => active_menu_item = MenuItem::Tutorial,
                _ => {},
            }
            Event::Tick => {},
        }
    }

    Ok(())
}

fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Single Player Poker",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'h' for help")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Game")
            .border_type(BorderType::Plain),
    );

    home
}
