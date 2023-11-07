use serde::Serialize;

use crate::metadata::value::publication::Publication;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SimpleTextData(pub String);

impl SimpleTextData {
    pub fn into_simple_text(self) -> Publication {
        Publication::SimpleText(self)
    }
}
