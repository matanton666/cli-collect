use crate::color_str::ColoredStr;
use indexmap::IndexMap;
use std::{io::{Error, Result}};

use inquire::{
    Text, 
    validator::{StringValidator, Validation},
    Select,
    error::InquireError, ui::{Color, RenderConfig}
};
pub struct Table {
    columns: IndexMap<String, Vec<ColoredStr>>, // map of column name and values
    max_len: IndexMap<String, usize>, // map of string length of column
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        Table { columns: IndexMap::new(), max_len: IndexMap::new() }
    }

    fn get_headers(&self) -> String {
        let mut headers = String::new();
        for (key, value) in &self.columns {
            headers.push_str(format!("{:<1$} ", key, self.max_len[key]+2).as_str());
        }
        headers.push('\n');
        headers
    }

    fn get_values(&self) -> Result<Vec<String>> {
        // get values
        if let Some(first) = self.columns.first() {
            let mut rows: Vec<String> = Vec::new();
            for i in 0..first.1.len() {
                let mut row = String::new();
                for (key, value) in &self.columns {
                    row.push_str(format!("{:<1$} ", value[i as usize].as_colored_string(), self.max_len[key]+2).as_str());
                }
                rows.push(row);
            }
            Ok(rows)
        }
        else {
            Err(Error::new(std::io::ErrorKind::Other, "no values"))
        }
    }

    pub fn get(&self, index: u32) -> IndexMap<String, String> {
        let mut out: IndexMap<String, String> = IndexMap::new();

        // insert all values from index to the new map 
        for (name, values) in self.columns.iter() {
            out.insert(name.clone(), values[index as usize].get_content().clone());
        }
        out
    }

    // returns the index of the line 
    pub fn print_interactive(&self) -> i32 {
        let headers = self.get_headers();
        println!("{}", "-".repeat(headers.len()));

        let rows = self.get_values();
        match rows {
            Ok(rows) => {
                let mut ans = Select::new(&headers, rows);
                // * customize the renderer
                ans.render_config.prompt_prefix.content = "";
                ans.render_config.highlighted_option_prefix.style.bg = Some(Color::DarkRed);
                ans = ans.with_help_message("↑↓ to move, enter to select, type to filter, Esc to exit");
                let res = ans.raw_prompt();
                
                
                
                match res {
                    Ok(choice) => choice.index as i32,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }

    pub fn print_as_table(&self) {
        // get heders
        print!("{}", self.get_headers());

        // get values
        for i in self.get_values().unwrap_or_default() { // REF: handle error
            println!("{}", i);
        }
    }


    pub fn insert(&mut self, key: &str, value: &ColoredStr) {
        // create if new column
        if !self.columns.contains_key(key) { 
            self.columns.insert(key.to_string(), Vec::new());
            self.max_len.insert(key.to_string(), key.len());
        }
        // push and update max
        self.columns.get_mut(key).unwrap().push(value.clone());
        if self.max_len[key] < value.get_content().len() {
            self.max_len.insert(key.to_string(), value.get_content().len());
        }
    }
}
