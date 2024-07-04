use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Bookmark {
    pub name: String,
    pub url: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl Bookmark {
    pub fn new(
        name: String,
        url: Option<String>,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Self {
        Bookmark {
            name,
            url,
            description,
            tags,
        }
    }
}
