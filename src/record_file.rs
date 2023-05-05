use csv::ReaderBuilder;

use std::str::FromStr;

/// Used for parsing `... .dist-info/RECORD` files.
#[derive(Debug, Eq, PartialEq)]
pub struct RecordFile {
    pub records: Vec<Record>,
}

impl FromStr for RecordFile {
    type Err = RecordFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reader = ReaderBuilder::default()
            .has_headers(false)
            .from_reader(s.as_bytes());

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

#[derive(Debug, Eq, PartialEq)]
pub struct Record {
    pub filename: String,
    pub digest: Option<Digest>,
    pub file_size: Option<usize>,
}

impl TryFrom<csv::StringRecord> for Record {
    type Error = RecordFileParseError;

    fn try_from(value: csv::StringRecord) -> Result<Self, Self::Error> {
        let filename = (&value[0]).to_owned();

        let digest = if value[1].is_empty() {
            None
        } else {
            Some(Digest::from_str(&value[1])?)
        };

        let file_size = if value[2].is_empty() {
            None
        } else {
            let Ok(file_size) = str::parse::<usize>(&value[2]) else {
		return Err(RecordFileParseError::MalformedFileSize);
	    };
            Some(file_size)
        };

        Ok(Self {
            filename,
            digest,
            file_size,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_packaging_example_record() -> Result<(), RecordFileParseError> {
        let record_file_text = concat!(
            "file.py,sha256=AVTFPZpEKzuHr7OvQZmhaU3LvwKz06AJw8mT\\_pNh2yI,3144\n",
            "distribution-1.0.dist-info/RECORD,,\n",
        );
        let record_file = RecordFile::from_str(record_file_text)?;

        assert_eq!(
            record_file,
            RecordFile {
                records: vec![
                    Record {
                        filename: "file.py".to_string(),
                        digest: Some(Digest {
                            method: "sha256".to_string(),
                            b64_digest: "AVTFPZpEKzuHr7OvQZmhaU3LvwKz06AJw8mT\\_pNh2yI".to_string(),
                        }),
                        file_size: Some(3144),
                    },
                    Record {
                        filename: "distribution-1.0.dist-info/RECORD".to_string(),
                        digest: None,
                        file_size: None,
                    },
                ],
            },
        );

        Ok(())
    }
}
