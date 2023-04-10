mod app_state;
mod database;
mod ui;
use app_state::State;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use database::sqlite::TableDataWithColumns;
use dotenvy::dotenv;
use sea_orm::Database;
use std::env;
use std::io;
use tui::backend::Backend;
use tui::widgets::{ListState, TableState};
use tui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // database setup
    dotenv().expect(".env file not found");
    let url = env::var("DATABASE_URL")?;
    let db = Database::connect(&url).await?;
    let tables = database::get_tables(db.clone()).await?;
    let table_name = tables
        .get(0)
        .expect("Dust and echoes...this database is empty.")
        .clone();
    let table_data: TableDataWithColumns =
        database::get_table_entries(db.clone(), &table_name).await?;
    let mut state = State {
        db_url: url,
        connection_info: db,
        tables,
        current_table: String::from(table_name),
        current_columns: table_data.columns,
        last_results: table_data.data,
        table_state: TableState::default(),
        list_state: ListState::default(),
        ..Default::default()
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut state);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        println!("{:?}", e);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut State) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => state.next(),
                KeyCode::Up => state.prev(),
                KeyCode::Char('p') => state.switch_panel(),
                KeyCode::Enter => (),
                _ => {}
            }
        }
    }
}
