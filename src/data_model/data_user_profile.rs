use serde::{Deserialize, Serialize};
pub mod  content;
pub mod  background;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataUserProfile {
    background: Option<background::Background>,
    content: Option<content::Content>
}