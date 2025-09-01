use crate::text_box::InteractiveTextBox;

#[derive(Default)]
pub struct Search {
    pub search_box: InteractiveTextBox,
    pub parsed_tags: Vec<String>,
    pub parsed_search_text: Option<String>,
}

impl Search {
    pub fn new() -> Self {
        Self {
            search_box: InteractiveTextBox::with_title("Search".to_string()),
            parsed_tags: Vec::new(),
            parsed_search_text: None,
        }
    }
}
