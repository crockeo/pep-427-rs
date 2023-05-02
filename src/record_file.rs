use std::str::FromStr;

/// Used for parsing `... .dist-info/RECORD` files.
pub struct RecordFile;

impl FromStr for RecordFile {
    type Err = RecordFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RecordFileParseError {}
