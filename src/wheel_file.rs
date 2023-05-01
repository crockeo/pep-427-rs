use std::str;
use std::str::FromStr;

/// Used for parsing `... .dist-info/WHEEL` files.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WheelFile {
    pub wheel_version: String,
    pub generator: String,
    pub root_is_purelib: bool,
    pub tags: Vec<String>,
    pub build: Option<usize>,
}

impl FromStr for WheelFile {
    type Err = WheelFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use WheelFileParseError::*;

        let mut wheel_version = None;
        let mut generator = None;
        let mut root_is_purelib = None;
        let mut tags = Vec::new();
        let mut build = None;

        for line in s.lines() {
            if let Some(line) = line.strip_prefix("Wheel-Version: ") {
                if wheel_version.is_some() {
                    return Err(DuplicateField("wheel_version"));
                }
                wheel_version = Some(line.to_owned());
            }

            if let Some(line) = line.strip_prefix("Generator: ") {
                if generator.is_some() {
                    return Err(DuplicateField("generator"));
                }
                generator = Some(line.to_owned());
            }

            if let Some(line) = line.strip_prefix("Root-Is-Purelib: ") {
                if root_is_purelib.is_some() {
                    return Err(DuplicateField("root_is_purelib"));
                }
                root_is_purelib = Some(
                    str::parse::<bool>(line)
                        .map_err(|e| InvalidFieldValue("root_is_purelib", e.to_string()))?,
                );
            }

            if let Some(line) = line.strip_prefix("Tag: ") {
                tags.push(line.to_owned());
            }

            if let Some(line) = line.strip_prefix("Build: ") {
                if build.is_some() {
                    return Err(DuplicateField("build"));
                }
                build = Some(
                    str::parse::<usize>(line)
                        .map_err(|e| InvalidFieldValue("build", e.to_string()))?,
                );
            }
        }

        Ok(WheelFile {
            wheel_version: wheel_version.ok_or(MissingField("wheel_version"))?,
            generator: generator.ok_or(MissingField("generator"))?,
            root_is_purelib: root_is_purelib.ok_or(MissingField("root_is_purelib"))?,
            tags,
            build,
        })
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum WheelFileParseError {
    #[error("there is at least one duplicate field")]
    DuplicateField(&'static str),

    #[error("a field has an invalid value")]
    InvalidFieldValue(&'static str, String),

    #[error("there is a missing field")]
    MissingField(&'static str),
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_from_str_simple() -> Result<(), WheelFileParseError> {
        let wheel_file_contents = fs::read_to_string("fixtures/simple_WHEEL.txt").unwrap();
        let wheel_file = WheelFile::from_str(&wheel_file_contents)?;
        assert_eq!(
            wheel_file,
            WheelFile {
                wheel_version: "1.0".to_owned(),
                generator: "bdist_wheel 1.0".to_owned(),
                root_is_purelib: true,
                tags: vec!["py2-none-any".to_owned(), "py3-none-any".to_owned()],
                build: Some(1),
            },
        );
        Ok(())
    }
}
