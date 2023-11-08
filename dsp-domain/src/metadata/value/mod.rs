use serde::Serialize;

pub mod alternative_name;
pub mod description;
pub mod discipline;
pub mod iso_code;
pub mod keyword;
pub mod lang_text_data;
pub mod publication;
pub mod ref_data;
pub mod simple_text_data;
pub mod spatial_coverage;
pub mod temporal_coverage;
pub mod url;
pub mod version;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ID(String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct CreatedAt(pub u64);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct CreatedBy(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Shortcode(pub String);

impl Shortcode {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Name(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct TeaserText(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct HowToCite(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct StartDate(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct EndDate(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ContactPoint(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Title(pub String);
