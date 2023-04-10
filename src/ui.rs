use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Row, Table},
    Frame,
};

use crate::app_state::{State, UserPanels};

pub fn ui<B: Backend>(f: &mut Frame<B>, state: &mut State) {
    let w = f.size().width;
    let h = f.size().height;
    let direction = if w > h {
        Direction::Horizontal
    } else {
        Direction::Vertical
    };

    let active_style = Style::default()
        .bg(tui::style::Color::DarkGray)
        .fg(tui::style::Color::White);
    let inactive_style = Style::default()
        .bg(tui::style::Color::Reset)
        .fg(tui::style::Color::White);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(h), Constraint::Length(4)].as_ref())
        .split(f.size());
    let top_chunks = Layout::default()
        .direction(direction)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(main_chunks[0]);

    let info_block = Table::new(vec![Row::new(vec![
        w.to_string(),
        h.to_string(),
        state.db_url.clone(),
    ])])
    .header(
        Row::new(vec!["Width", "Height", "Connection string"])
            .style(Style::default().fg(tui::style::Color::Gray)),
    )
    .widths(&[
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(40),
    ])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(info_block, main_chunks[1]);

    let items: Vec<ListItem> = state
        .tables
        .clone()
        .into_iter()
        .map(ListItem::new)
        .collect();
    let list = List::new(items)
        .block(Block::default().title("Tables").borders(Borders::ALL))
        .style(if let UserPanels::Tables(_) = state.panel {
            active_style
        } else {
            inactive_style
        })
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
        .map(|n| Constraint::Ratio(n.len() as u32, total_w as u32))
        .collect();

    let rows: Vec<Row> = state
        .last_results
        .clone()
        .into_iter()
        .map(|r| {
            let vals: Vec<String> = column_names
                .iter()
                .map(|col| match r.get(col) {
                    Some(Some(v)) => String::from(v),
                    _ => "NULL".to_string(),
                })
                .collect();
            Row::new(vals).style(Style::default().fg(tui::style::Color::White))
        })
        .collect();

    let column_names: Row = Row::new(column_names);

    let block_title = format!("Table preview: {}", &state.current_table);

    let table = Table::new(rows)
        .style(if let UserPanels::Rows(_) = state.panel {
            active_style
        } else {
            inactive_style
        })
        .widths(&column_widths)
        .column_spacing(1)
        .header(
            column_names
                .style(Style::default().fg(tui::style::Color::Yellow))
                .bottom_margin(1),
        )
        .block(Block::default().title(block_title).borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");
    f.render_stateful_widget(table, top_chunks[1], &mut state.table_state);
}
