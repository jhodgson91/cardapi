use super::cards::CardSelection;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct DrawData {
    pub(super) destination: String,
    pub(super) selection: CardSelection,
}
