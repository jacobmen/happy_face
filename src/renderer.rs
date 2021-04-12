use std::io;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::error::Error;
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
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use std::fs;

// use super::client::{Event};

enum Event<I> {
    Input(I),
    Tick,
}

pub fn render() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
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



    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Main block with round corners")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[0]);
            let block = Block::default()
                .title(vec![
                    Span::styled("With", Style::default().fg(Color::Yellow)),
                    Span::from(" background"),
                ])
                .style(Style::default().bg(Color::Green));
            f.render_widget(block, top_chunks[0]);

            let block = Block::default().title(Span::styled(
                "Styled title",
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ));
            f.render_widget(block, top_chunks[1]);

            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);
            let block = Block::default().title("With borders").borders(Borders::ALL);
            f.render_widget(block, bottom_chunks[0]);
            let block = Block::default()
                .title("With styled borders and doubled borders")
                .border_style(Style::default().fg(Color::Cyan))
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_type(BorderType::Double);
            f.render_widget(block, bottom_chunks[1]);
            })?;

            match rx.recv()? {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    _ => {}
                },
                Event::Tick => {}
            }
        }
        Ok(())
}