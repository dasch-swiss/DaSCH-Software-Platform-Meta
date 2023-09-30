use serde::Serialize;

use crate::domain::model::value::Title;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Dataset {
    pub title: Title,
}
