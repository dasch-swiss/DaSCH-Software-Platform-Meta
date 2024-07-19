use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use valico::json_schema;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub project: Project,
    pub datasets: NonEmpty<Dataset>,
    pub persons: Option<Vec<Person>>,
    pub organizations: Option<Vec<Organization>>,
    pub grants: Option<Vec<Grant>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: String,
    #[serde(rename = "__createdBy")]
    pub created_by: String,
    pub shortcode: String,
    pub name: String,
    pub description: Text,
    pub start_date: Date,
    pub teaser_text: String,
    pub datasets: NonEmpty<String>,
    pub keywords: NonEmpty<Text>,
    pub disciplines: NonEmpty<TextOrUrl>,
    pub temporal_coverage: NonEmpty<TextOrUrl>,
    pub spatial_coverage: NonEmpty<Url>,
    pub funders: NonEmpty<String>,
    pub url: Url,
    pub secondary_url: Option<Url>,
    pub data_management_plan: Option<DataManagementPlan>,
    pub end_date: Option<Date>,
    pub contact_point: Option<String>,
    pub how_to_cite: String,
    pub publications: Option<Vec<String>>,
    pub grants: Option<Vec<String>>,
    pub alternative_names: Option<Vec<Text>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: String,
    #[serde(rename = "__createdBy")]
    pub created_by: String,
    pub title: String,
    pub access_conditions: String, // "enum": [ "open", "restricted", "closed" ]
    pub how_to_cite: String,
    pub status: String, // "enum": [ "In planning", "Ongoing", "On hold", "Finished" ]
    pub abstracts: NonEmpty<TextOrUrl>,
    pub type_of_data: NonEmpty<String>, // "enum": [ "XML", "Text", "Image", "Video", "Audio" ]
    pub licenses: NonEmpty<License>,
    pub languages: NonEmpty<Text>,
    pub attributions: NonEmpty<Attribution>, // non-empty
    pub alternative_titles: Option<Vec<Text>>,
    pub date_published: Option<Date>,
    pub date_created: Option<Date>,
    pub date_modified: Option<Date>,
    pub distribution: Option<Url>,
    pub urls: Option<Vec<Url>>,
    pub additional: Option<Vec<TextOrUrl>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: String,
    #[serde(rename = "__createdBy")]
    pub created_by: String,
    pub job_titles: NonEmpty<String>,
    pub given_names: NonEmpty<String>,
    pub family_names: NonEmpty<String>,
    pub affiliation: NonEmpty<String>,
    pub address: Option<Address>,
    pub email: Option<String>,
    pub secondary_email: Option<String>,
    pub authority_refs: Option<Vec<Url>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: String,
    #[serde(rename = "__createdBy")]
    pub created_by: String,
    pub name: String,
    pub url: Option<Url>,
    pub address: Option<Address>,
    pub email: Option<String>,
    pub alternative_names: Option<Vec<Text>>,
    pub authority_refs: Option<Vec<Url>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Grant {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: String,
    #[serde(rename = "__createdBy")]
    pub created_by: String,
    pub funders: NonEmpty<String>,
    pub number: Option<String>,
    pub name: Option<String>,
    pub url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Text(HashMap<String, String>);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Date(String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub r#type: String,
    pub url: String,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: String,
    pub postal_code: String,
    pub locality: String,
    pub country: String,
    pub canton: Option<String>,
    pub additional: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataManagementPlan {
    pub available: bool,
    pub url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Attribution {
    pub agent: String,
    pub roles: NonEmpty<String>, // non-empty
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub license: Url,
    pub date: Date,
    pub details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TextOrUrl {
    Text(Text),
    Url(Url),
}

#[test]
fn test_deserialization() {
    let paths = vec![
        "/Users/christian/git/dasch/dsp-meta/data/json/_bilddatenbank.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/roud.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/limc.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/posepi.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/olympic.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/dasch.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/mfmps.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/waldaucinema.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/operativetv.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/beol.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/_dssl.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/mls.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/societesavoie.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/biz.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/_rosetta.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/hdm.json",
    ].into_iter().map(|s| Path::new(s));
    let mut success: usize = 0;
    let mut error: usize = 0;

    for path in paths {
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            println!("Checking {}:", path.to_str().get_or_insert(""));
            let contents = fs::read_to_string(path)
                .expect("Should have been able to read the file");
            let metadata = serde_json::from_str::<Metadata>(&*contents);
            match metadata {
                Ok(_data) => {
                    success = success + 1;
                    println!("SUCCESS\n") // println!("DATA:\n {:?}\n", data),
                }
                Err(err) => {
                    error = error + 1;
                    println!("ERROR:\n {:?}\n", err)
                }
            };
        }
    }
    println!("Success: {}, Error: {}, Total: {}", success, error, success + error)
}

#[test]
fn test_jsonschema() {
    verify_all_json_files_in_directory_jsonschema("/Users/christian/git/dasch/dsp-meta/data/json/");
    assert!(true)
}

fn verify_all_json_files_in_directory_jsonschema(directory: &str) {
    let paths = fs::read_dir(directory).unwrap();
    let mut success: usize = 0;
    let mut error: usize = 0;
    let json_schema: Value = serde_json::from_reader(File::open("/Users/christian/git/dasch/dsp-meta/docs/domain_model/schema-metadata.json").unwrap()).unwrap();
    let mut scope = json_schema::Scope::new();
    let schema = scope.compile_and_return(json_schema, false).unwrap();
    let mut valid: Vec<String> = Vec::new();
    let mut invalid: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file = (*path.to_str().get_or_insert("")).to_string();
            println!("Checking {}:", file);
            let contents = fs::read_to_string(&path)
                .expect("Should have been able to read the file");
            let metadata = serde_json::from_str::<Value>(&*contents).expect("parsed data as json");
            let result = schema.validate(&metadata).is_valid();
            let filename = file["/Users/christian/git/dasch/dsp-meta/data/json/".len()..].to_string();
            match result {
                true => {
                    success = success + 1;
                    valid.push(filename);
                    println!("VALID\n") // println!("DATA:\n {:?}\n", data),
                }
                false => {
                    error = error + 1;
                    invalid.push(filename);
                    println!("INVALID\n") // println!("DATA:\n {:?}\n", data),
                }
            };
        }
    }
    println!("Success: {}, Error: {}, Total: {}", success, error, success + error);
    println!();

    println!("VALID files:\n{}", valid.join("\n"));
    println!();

    println!("INVALID files:\n{}", invalid.join("\n"));
}