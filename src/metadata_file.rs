use std::str::FromStr;

/// Used for parsing `... .dist-info/METADATA` files.
pub struct MetadataFile {}

impl FromStr for MetadataFile {
    type Err = MetadataFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MetadataFileParseError {}
