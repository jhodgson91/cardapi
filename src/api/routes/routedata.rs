use super::CardSelection;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct DrawData {
    pub(super) source: String,
    pub(super) selection: CardSelection,
}
