use std::{thread, io, time::Duration};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use tui::Frame;
use tui::backend::Backend;
use tui::style::{Style, Modifier};
use tui::{backend::CrosstermBackend, Terminal};
use tui::widgets::{Widget, Block, Borders, ListItem, List};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), io::Error>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run_app(&mut terminal)?;

    thread::sleep(Duration::from_millis(500));

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(75),
            ]
            .as_ref(),
        )
        .split(f.size());
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

    let items: Vec<ListItem> = vec!("Item 1", "Item 2").into_iter().map(ListItem::new).collect();
    let list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(tui::style::Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::SLOW_BLINK))
        .highlight_symbol(">");

    f.render_widget(list, left_chunks[0]);
    let block = Block::default().title("Preview").borders(Borders::ALL);
    f.render_widget(block, main_chunks[1]);
}
