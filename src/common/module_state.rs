use std::rc::Rc;

use ratatui::layout::Rect;
use serde::{Deserialize, Serialize, ser::SerializeStruct};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UISection {
    Search,
    Results,
}
