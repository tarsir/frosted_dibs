use std::{collections::HashMap, option::Option};

use sea_orm::DatabaseConnection;
use tui::widgets::{ListState, TableState};

use crate::database::sqlite::TableInfoRow;

#[derive(Debug, Clone)]
pub enum UserPanels {
    Tables(ListState),
    Rows(TableState),
}

impl Default for UserPanels {
    fn default() -> Self {
        UserPanels::Tables(ListState::default())
    }
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub db_url: String,
    pub connection_info: DatabaseConnection,
    pub tables: Vec<String>,
    pub current_table: String,
    pub last_results: Vec<HashMap<String, Option<String>>>,
    pub current_columns: Vec<TableInfoRow>,
    pub table_state: TableState,
    pub list_state: ListState,
    pub panel: UserPanels,
}

impl State {
    pub fn switch_panel(&mut self) {
        self.panel = match &self.panel {
            UserPanels::Tables(_) => UserPanels::Rows(self.table_state.clone()),
            UserPanels::Rows(_) => UserPanels::Tables(self.list_state.clone()),
        };
    }

    pub fn next(&mut self) {
        let selected = match &self.panel {
            UserPanels::Tables(s) => s.selected(),
            UserPanels::Rows(s) => s.selected(),
        };
        let i = match selected {
            Some(i) => {
                if i >= self.last_results.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        match self.panel {
            UserPanels::Tables(ref mut s) => {
                self.list_state.select(Some(i));
                s.select(Some(i))
            }
            UserPanels::Rows(ref mut s) => {
                self.table_state.select(Some(i));
                s.select(Some(i))
            }
        };
    }

    pub fn prev(&mut self) {
        let selected = match &self.panel {
            UserPanels::Tables(s) => s.selected(),
            UserPanels::Rows(s) => s.selected(),
        };
        let i = match selected {
            Some(i) => {
                if i == 0 {
                    self.last_results.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        match self.panel {
            UserPanels::Tables(ref mut s) => {
                self.list_state.select(Some(i));
                s.select(Some(i))
            }
            UserPanels::Rows(ref mut s) => {
                self.table_state.select(Some(i));
                s.select(Some(i))
            }
        };
    }
}
