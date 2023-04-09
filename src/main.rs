mod app_state;
mod database;
use app_state::{State, UserPanels};
use dotenvy::dotenv;
use std::env;
use std::{thread, io, time::Duration};
use sqlx::sqlite::SqlitePool;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use tui::Frame;
use tui::backend::Backend;
use tui::style::{Style, Modifier};
use tui::{backend::CrosstermBackend, Terminal};
use tui::widgets::{Widget, Block, Borders, ListItem, List, Table, Row, TableState, ListState};
use tui::layout::{Layout, Constraint, Direction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // database setup
    dotenv().expect(".env file not found");
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let tables = database::get_tables(&pool).await?;
    let table_name = tables[0].clone();
    let (rows, columns) = database::get_table_entries(&pool, &table_name).await?;
    let mut state = State {
        tables,
        current_table: String::from(table_name),
        current_columns: columns,
        last_results: rows,
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
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        println!("{:?}", e);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut State) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => state.next(),
                KeyCode::Up => state.prev(),
                KeyCode::Char('p') => state.switch_panel(),
                _ => {},
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &mut State) {
    let w = f.size().width;
    let h = f.size().height;
    let direction = if w > h {
        Direction::Horizontal
    } else {
        Direction::Vertical
    };

    let active_style = Style::default().bg(tui::style::Color::DarkGray).fg(tui::style::Color::White);
    let inactive_style = Style::default().bg(tui::style::Color::Reset).fg(tui::style::Color::White);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Max(h),
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(f.size());
    let top_chunks = Layout::default()
        .direction(direction)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(75),
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

    let info_block = Table::new(
        vec![
            Row::new(vec![w.to_string(), h.to_string()])
        ]
    )
        .header(
            Row::new(vec!["Width", "Height"])
            .style(Style::default().fg(tui::style::Color::Gray))
        )
        .widths(&[Constraint::Percentage(10), Constraint::Percentage(10)])
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(info_block, main_chunks[1]);

    let items: Vec<ListItem> = state.tables.clone().into_iter().map(ListItem::new).collect();
    let list = List::new(items)
        .block(Block::default().title("Tables").borders(Borders::ALL))
        .style(if let UserPanels::Tables(_) = state.panel { active_style } else { inactive_style })
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, top_chunks[0], &mut state.list_state);

    let column_names: Vec<String> = state
        .current_columns
        .iter()
        .map(|c| c.name.clone())
        .collect();
    let total_w: usize = column_names.iter().fold(0, |acc, x| acc + x.len());

    let column_widths: Vec<Constraint> = column_names
        .iter()
        .map(|n| {
            Constraint::Ratio(n.len() as u32, total_w as u32)
        })
        .collect();

    let rows: Vec<Row> = state.last_results.clone()
        .into_iter()
        .map(|r| {
            let vals: Vec<String> = column_names.iter().map(|col| {
                match r.get(col) {
                    Some(Some(v)) => String::from(v),
                    _ => "NULL".to_string(),
                }
            }).collect();
            Row::new(vals).style(Style::default().fg(tui::style::Color::White))
        })
        .collect();

    let column_names: Row = Row::new(column_names);

    let block_title = format!("Table preview: {}", &state.current_table);

    let table = Table::new(rows)
        .style(if let UserPanels::Rows(_) = state.panel { active_style } else { inactive_style })
        .widths(&column_widths)
        .column_spacing(1)
        .header(
            column_names
            .style(
                Style::default().fg(tui::style::Color::Yellow)
            )
            .bottom_margin(1)
        )
        .block(Block::default().title(block_title).borders(Borders::ALL))
        .highlight_style(
            Style::default().add_modifier(Modifier::BOLD)
        )
        .highlight_symbol(">>");
    f.render_stateful_widget(table, top_chunks[1], &mut state.table_state);
}
