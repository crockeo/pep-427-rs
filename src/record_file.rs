use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};
use std::ops::Deref;
use std::str::FromStr;

/// Used for parsing `.dist-info/RECORD` files.
///
/// <https://www.python.org/dev/peps/pep-0376/#record>
///
/// Internally, this is a [Vec] of [RecordEntry].
///
/// # Example
/// ```
/// use std::str::FromStr;
/// use pep_427::{RecordFile, RecordEntry};
///
/// let record = RecordFile::from_str(r#"
/// tqdm/cli.py,sha256=x_c8nmc4Huc-lKEsAXj78ZiyqSJ9hJ71j7vltY67icw,10509
/// tqdm-4.62.3.dist-info/RECORD,,
/// "#).unwrap();
/// assert_eq!(record.to_vec(), vec![
///     RecordEntry {
///         path: "tqdm/cli.py".to_string(),
///         hash: Some("sha256=x_c8nmc4Huc-lKEsAXj78ZiyqSJ9hJ71j7vltY67icw".to_string()),
///         size: Some(10509)
///     },
///     RecordEntry {
///         path: "tqdm-4.62.3.dist-info/RECORD".to_string(),
///         hash: None,
///         size: None,
///     },
/// ]);
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RecordFile(Vec<RecordEntry>);

impl RecordFile {
    /// Read a `.dist-info/RECORD` file
    pub fn read(record: &mut impl Read) -> Result<Self, RecordFileParseError> {
        let entries: Vec<RecordEntry> = csv::ReaderBuilder::new()
            .has_headers(false)
            .escape(Some(b'"'))
            .from_reader(record)
            .deserialize()
            .map(|entry| {
                let entry: RecordEntry = entry?;
                Ok(RecordEntry {
                    // selenium 4.1.0 uses absolute paths for some reason
                    path: entry.path.trim_start_matches('/').to_string(),
                    ..entry
                })
            })
            .collect::<Result<_, RecordFileParseError>>()?;
        Ok(Self(entries))
    }
}

impl FromStr for RecordFile {
    type Err = RecordFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::read(&mut Cursor::new(s))
    }
}

impl Deref for RecordFile {
    type Target = Vec<RecordEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for RecordFile {
    type Item = RecordEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// A line in a `.dist-info/RECORD` file
#[derive(Debug, Clone, Deserialize, Serialize, PartialOrd, PartialEq, Ord, Eq)]
pub struct RecordEntry {
    pub path: String,
    pub hash: Option<String>,
    pub size: Option<usize>,
}

pub type RecordFileParseError = csv::Error;

#[cfg(test)]
mod test {
    use crate::RecordFile;
    use indoc::indoc;
    use std::str::FromStr;

    #[test]
    fn record_with_absolute_paths() {
        let record: &str = indoc! {"
            /selenium/__init__.py,sha256=l8nEsTP4D2dZVula_p4ZuCe8AGnxOq7MxMeAWNvR0Qc,811
            /selenium/common/exceptions.py,sha256=oZx2PS-g1gYLqJA_oqzE4Rq4ngplqlwwRBZDofiqni0,9309
            selenium-4.1.0.dist-info/METADATA,sha256=jqvBEwtJJ2zh6CljTfTXmpF1aiFs-gvOVikxGbVyX40,6468
            selenium-4.1.0.dist-info/RECORD,,
        "};

        let entries = RecordFile::from_str(record).unwrap();
        let expected = [
            "selenium/__init__.py",
            "selenium/common/exceptions.py",
            "selenium-4.1.0.dist-info/METADATA",
            "selenium-4.1.0.dist-info/RECORD",
        ]
        .map(ToString::to_string)
        .to_vec();
        let actual = entries
            .into_iter()
            .map(|entry| entry.path)
            .collect::<Vec<String>>();
        assert_eq!(expected, actual);
    }
}
