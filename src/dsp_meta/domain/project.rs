use crate::domain::convert::project::{ExtractedProjectAttributes, ExtractedProjectBlocks};
use crate::domain::{
    AlternativeName, CreatedAt, CreatedBy, Description, Discipline, EndDate, HowToCite, Keyword,
    Name, Publication, Shortcode, StartDate, TeaserText, UrlValue,
};
use crate::errors::DspMetaError;

#[derive(Debug, Default, PartialEq)]
pub struct Project {
    pub created_at: CreatedAt,
    pub created_by: CreatedBy,
    pub shortcode: Shortcode,
    pub name: Name,
    pub alternative_names: Vec<AlternativeName>,
    pub teaser_text: TeaserText,
    pub description: Description,
    pub url: UrlValue,
    pub how_to_cite: HowToCite,
    pub start_date: StartDate,
    pub end_date: Option<EndDate>,
    pub keywords: Vec<Keyword>,
    pub disciplines: Vec<Discipline>,
    pub publications: Vec<Publication>,
}

impl TryFrom<&hcl::Block> for Project {
    type Error = DspMetaError;

    fn try_from(project_block: &hcl::Block) -> Result<Self, Self::Error> {
        if project_block.identifier.as_str() != "project" {
            return Err(DspMetaError::ParseProject(
                "Parse error: project block needs to be named 'project'.",
            ));
        }

        // extract the project attributes
        // created_at, created_by, shortcode, name, teaser_text, how_to_cite, start_date, end_date, datasets, funders, grants

        let attributes: Vec<&hcl::Attribute> = project_block.body.attributes().collect();

        let extracted_attributes = ExtractedProjectAttributes::try_from(attributes)?;

        let created_at = extracted_attributes.created_at.ok_or_else(|| {
            DspMetaError::ParseProject("Parse error: project needs to have a created_at value.")
        })?;

        let created_by = extracted_attributes.created_by.ok_or_else(|| {
            DspMetaError::ParseProject("Parse error: project needs to have a created_by value.")
        })?;

        let shortcode = extracted_attributes.shortcode.ok_or_else(|| {
            DspMetaError::ParseProject("Parse error: project needs to have a shortcode.")
        })?;

        let name = extracted_attributes.name.ok_or_else(|| {
            DspMetaError::ParseProject("Parse error: project needs to have a name.")
        })?;

        let teaser_text = extracted_attributes.teaser_text.ok_or_else(|| {
            DspMetaError::ParseProject("Parse error: project needs to have a teaser_text.")
        })?;

        let how_to_cite = extracted_attributes.how_to_cite.ok_or_else(|| {
            DspMetaError::ParseProject("Parse error: project needs to have a how_to_cite.")
        })?;

        let start_date = extracted_attributes.start_date.ok_or_else(|| {
            DspMetaError::ParseProject("Parse error: project needs to have a start_date.")
        })?;

        let end_date = extracted_attributes.end_date;

        // extract the project blocks
        // alternative_names, description, url, keywords, disciplines, publications)

        let blocks: Vec<&hcl::Block> = project_block.body.blocks().collect();
        let extracted_blocks = ExtractedProjectBlocks::try_from(blocks)?;

        let alternative_names = extracted_blocks.alternative_names;
        let description = extracted_blocks.description.ok_or_else(|| {
            DspMetaError::ParseProject("Parse error: project needs to have a description.")
        })?;
        let url = UrlValue::default();
        let keywords = vec![Keyword::default()];
        let disciplines = vec![Discipline::default()];
        let publications = vec![Publication::default()];

        let project = Project {
            created_at,
            created_by,
            shortcode,
            name,
            alternative_names,
            teaser_text,
            description,
            url,
            how_to_cite,
            start_date,
            end_date,
            keywords,
            disciplines,
            publications,
        };

        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use hcl::block;
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_convert_project_block() {
        let input_project_block = block!(
            project {
                created_at = 1630601274523025000u64 // FIXME: is there a more readable way to write an i64?
                created_by  = "dsp-metadata-gui"
                shortcode = "0803"
                name = "The German Family Panel (pairfam)"
                alternative_name "1" {
                    de = "Der deutsche Familienpanel (pairfam)"
                    en = "The German Family Panel (pairfam)"
                }
                teaser_text = "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
                description {
                    de = "Der deutsche Familienpanel (pairfam) ist eine multidisziplinäre, längsschnittliche Studie."
                    en = "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
                }
                how_to_cite = "Huinink, Johannes; Schröder, Carolin; Castiglioni, Laura; Feldhaus, Michael"
                start_date  = "2009-04-01"
                end_date    = "2012-03-31"
            }
        );
        let project = Project::try_from(&input_project_block).unwrap();
        dbg!(&project);
        assert_eq!(project.created_at, CreatedAt(1630601274523025000));
        assert_eq!(
            project.created_by,
            CreatedBy(String::from("dsp-metadata-gui"))
        );
        assert_eq!(project.shortcode, Shortcode(String::from("0803")));
        assert_eq!(
            project.name,
            Name(String::from("The German Family Panel (pairfam)"))
        );
        assert_eq!(
            project.teaser_text,
            TeaserText(String::from(
                "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
            ))
        );
        assert_eq!(
            project.how_to_cite,
            HowToCite(String::from(
                "Huinink, Johannes; Schröder, Carolin; Castiglioni, Laura; Feldhaus, Michael"
            ))
        );
        assert_eq!(project.start_date, StartDate(String::from("2009-04-01")));
        assert_eq!(project.end_date, Some(EndDate(String::from("2012-03-31"))));
    }
}
