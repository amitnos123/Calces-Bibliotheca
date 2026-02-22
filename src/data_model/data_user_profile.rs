use serde::{Deserialize, Serialize};
pub mod  content;
pub mod  background;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataUserProfile {
    pub background: Option<background::Background>,
    pub content: Option<content::Content>
}