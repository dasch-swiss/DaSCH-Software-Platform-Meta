use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{
    CreatedAt, CreatedBy, Datasets, EndDate, Funders, Grants, HowToCite, Name, Shortcode,
    StartDate, TeaserText, ID,
};

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Project {
    id: ID,
    created_at: CreatedAt,
    created_by: CreatedBy,
    shortcode: Shortcode,
    name: Name,
    teaser_text: TeaserText,
    how_to_cite: HowToCite,
    start_date: StartDate,
    end_date: Option<EndDate>,
    datasets: Datasets,
    funders: Funders,
    grants: Option<Grants>,
}

impl Project {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ID,
        created_at: CreatedAt,
        created_by: CreatedBy,
        shortcode: Shortcode,
        name: Name,
        teaser_text: TeaserText,
        how_to_cite: HowToCite,
        start_date: StartDate,
        end_date: Option<EndDate>,
        datasets: Datasets,
        funders: Funders,
        grants: Option<Grants>,
    ) -> Self {
        Self {
            id,
            created_at,
            created_by,
            shortcode,
            name,
            teaser_text,
            how_to_cite,
            start_date,
            end_date,
            datasets,
            funders,
            grants,
        }
    }
    pub fn id(&self) -> &ID {
        &self.id
    }
    pub fn created_at(&self) -> &CreatedAt {
        &self.created_at
    }
    pub fn created_by(&self) -> &CreatedBy {
        &self.created_by
    }
    pub fn shortcode(&self) -> &Shortcode {
        &self.shortcode
    }
    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn teaser_text(&self) -> &TeaserText {
        &self.teaser_text
    }
    pub fn how_to_cite(&self) -> &HowToCite {
        &self.how_to_cite
    }
    pub fn start_date(&self) -> &StartDate {
        &self.start_date
    }
    pub fn end_date(&self) -> &Option<EndDate> {
        &self.end_date
    }
}
