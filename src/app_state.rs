use std::option::Option;

#[derive(Debug, Default, Clone)]
pub struct State {
    pub tables: Vec<String>,
    pub current_table: String,
    pub last_results: Option<String>,
}
