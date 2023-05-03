use std::str::FromStr;

/// Used for parsing `... .dist-info/METADATA` files.
pub struct MetadataFile {
    pub metadata_version: String,
    pub name: String,
    pub version: String,
    // TODO: dynamic https://packaging.python.org/en/latest/specifications/core-metadata/#dynamic-multiple-use
    pub platform: String,
    pub supported_platform: String,
    pub summary: String,
    // TODO: this one is going to need some special treatment
    // https://packaging.python.org/en/latest/specifications/core-metadata/#description
    pub description: String,
    pub description_content_type: String,
    pub keywords: Vec<String>,
    pub home_page: String,
    pub author: String,
    pub author_email: Vec<String>,
    pub maintainer: String,
    pub maintainer_email: Vec<String>,
    pub license: String,
    pub classifier: Vec<String>,
    // TODO: https://packaging.python.org/en/latest/specifications/core-metadata/#requires-dist-multiple-use
    pub requires_dist: (),
    pub requires_python: String,
    pub requires_external: Vec<String>,
    pub project_url: ProjectURL,
    // This is probably going to need some smarts https://packaging.python.org/en/latest/specifications/core-metadata/#provides-extra-multiple-use
    pub provides_extra: Vec<String>,
    // Intentionally omitting fields which are marked as rarely used.
    // https://packaging.python.org/en/latest/specifications/core-metadata/#rarely-used-fields
}

pub struct ProjectURL {
    pub label: String,
    pub url: String,
}

impl FromStr for MetadataFile {
    type Err = MetadataFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MetadataFileParseError {}
