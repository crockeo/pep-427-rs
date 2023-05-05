use csv::Reader;

use std::str::FromStr;

/// Used for parsing `... .dist-info/RECORD` files.
pub struct RecordFile {
    pub records: Vec<Record>,
}

impl FromStr for RecordFile {
    type Err = RecordFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reader = Reader::from_reader(s.as_bytes());
        let mut records = Vec::new();
        for record in reader.records() {
            let record = record?;
            records.push(record.try_into()?);
        }
        Ok(RecordFile { records })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RecordFileParseError {
    #[error(transparent)]
    CSVError(#[from] csv::Error),

    #[error("malformed digest")]
    MalformedDigest,

    #[error("malformed file size")]
    MalformedFileSize,
}

pub struct Record {
    pub filename: String,
    pub digest: Digest,
    pub file_size: usize,
}

impl TryFrom<csv::StringRecord> for Record {
    type Error = RecordFileParseError;

    fn try_from(value: csv::StringRecord) -> Result<Self, Self::Error> {
        let filename = (&value[0]).to_owned();
        let digest = Digest::from_str(&value[1])?;
        let Ok(file_size) = str::parse::<usize>(&value[2]) else {
	    return Err(RecordFileParseError::MalformedFileSize);
	};

        Ok(Self {
            filename,
            digest,
            file_size,
        })
    }
}

pub struct Digest {
    pub method: String,
    pub b64_digest: String,
}

impl FromStr for Digest {
    type Err = RecordFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((method, b64_digest)) = s.split_once("=") else {
	    return Err(RecordFileParseError::MalformedDigest);
	};
        Ok(Self {
            method: method.to_owned(),
            b64_digest: b64_digest.to_owned(),
        })
    }
}
