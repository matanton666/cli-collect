
use crate::utils::{DEF_COLOR, color_string};
use colored::ColoredString;

#[derive(Clone)]
/// implementation of a colored string so the string itself can be changed
pub struct ColoredStr {
    content: String,
    color: String,
}

impl ColoredStr {
    pub fn new(input_string: String)  -> Self {
        ColoredStr { content: input_string, color: DEF_COLOR.to_string() }
    }
    
    pub fn new_colored(input_string: String, input_color: String) -> Self {
        ColoredStr { content: input_string, color: input_color }
    }

    pub fn as_colored_string(&self) -> ColoredString {
        color_string(&self.content, &self.color)
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn set_content(&mut self, new_content: String) {
        self.content = new_content;
    }
    
    pub fn append(&mut self, new_content: &str) {
        self.content.push_str(new_content);
    }

    pub fn clear(&mut self) {
        self.set_content("".to_string());
    }

    pub fn set_color(&mut self, input_color: String) {
        // TODO: check on color to make sure it is compatible
        self.color = input_color;
    }
}