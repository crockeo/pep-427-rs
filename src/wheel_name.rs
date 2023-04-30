use std::str::FromStr;

use lazy_static::lazy_static;
use pep440_rs::Version;
use regex::Regex;

lazy_static! {
    static ref NAME_RE: Regex = Regex::new(r#"^[\w\d._]*$"#).unwrap();
    static ref BUILD_TAG_RE: Regex = Regex::new(r#"^(?P<number>\d)+(?P<remainder>.*)$"#).unwrap();
}

/// Used for parsing `*.whl` file names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WheelName {
    pub distribution: String,
    pub version: Version,
    pub build_tag: Option<BuildTag>,
    pub python_tag: String,
    pub abi_tag: String,
    pub platform_tag: String,
}

impl FromStr for WheelName {
    type Err = WheelNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(filename) = s.strip_suffix(".whl") else {
	    return Err(WheelNameParseError::NotAWheel)
	};

        let parts = filename.split('-').collect::<Vec<&str>>();
        if parts.len() != 5 && parts.len() != 6 {
            return Err(WheelNameParseError::PartMismatch);
        }

        let distribution = parts[0].to_owned();
        if distribution.contains("__") || !NAME_RE.is_match(&distribution) {
            return Err(WheelNameParseError::InvalidDistributionName(distribution));
        }
        let distribution = distribution.to_lowercase().replace(['_', '.'], "-");

        let version = match Version::from_str(parts[1]) {
            Err(reason) => return Err(WheelNameParseError::InvalidVersion(reason)),
            Ok(version) => version,
        };

        let (build_tag, index_offset) = if parts.len() == 6 {
            let build_tag = BuildTag::from_str(parts[2])?;
            (Some(build_tag), 1)
        } else {
            (None, 0)
        };

        let python_tag = parts[2 + index_offset].to_owned();
        let abi_tag = parts[3 + index_offset].to_owned();
        let platform_tag = parts[4 + index_offset].to_owned();
        Ok(Self {
            distribution,
            version,
            build_tag,
            python_tag,
            abi_tag,
            platform_tag,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildTag {
    pub number: usize,
    pub remainder: Option<String>,
}

impl FromStr for BuildTag {
    type Err = WheelNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = BUILD_TAG_RE.captures(s) else {
	    return Err(WheelNameParseError::InvalidBuildTag(s.to_owned()));
	};

        let Ok(number) = captures.name("number").unwrap().as_str().parse::<usize>() else {
	    return Err(WheelNameParseError::InvalidBuildTag(s.to_owned()));
	};
        let remainder = {
            let raw_remainder = captures.name("remainder").unwrap().as_str();
            if raw_remainder.is_empty() {
                None
            } else {
                Some(raw_remainder.to_owned())
            }
        };

        Ok(BuildTag { number, remainder })
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum WheelNameParseError {
    #[error("provided file name does not end with a .whl")]
    NotAWheel,

    #[error("wheel has an unexpected number of parts")]
    PartMismatch,

    #[error("invalid distribution name")]
    InvalidDistributionName(String),

    #[error("invalid PEP440 version")]
    InvalidVersion(String),

    #[error("invalid build tag")]
    InvalidBuildTag(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_str_simple() -> Result<(), WheelNameParseError> {
        let wheel_name = WheelName::from_str("requests-2.29.0-py3-none-any.whl")?;
        assert_eq!(
            wheel_name,
            WheelName {
                distribution: "requests".to_string(),
                version: Version::from_str("2.29.0").unwrap(),
                build_tag: None,
                python_tag: "py3".to_string(),
                abi_tag: "none".to_string(),
                platform_tag: "any".to_string(),
            },
        );
        Ok(())
    }

    #[test]
    fn test_from_str_build_tag() -> Result<(), WheelNameParseError> {
        let wheel_name = WheelName::from_str("requests-2.29.0-1-py3-none-any.whl")?;
        assert_eq!(
            wheel_name,
            WheelName {
                distribution: "requests".to_string(),
                version: Version::from_str("2.29.0").unwrap(),
                build_tag: Some(BuildTag {
                    number: 1,
                    remainder: None,
                }),
                python_tag: "py3".to_string(),
                abi_tag: "none".to_string(),
                platform_tag: "any".to_string(),
            },
        );
        Ok(())
    }

    #[test]
    fn test_from_str_build_tag_trailing_content() -> Result<(), WheelNameParseError> {
        let wheel_name = WheelName::from_str("requests-2.29.0-1asdf-py3-none-any.whl")?;
        assert_eq!(
            wheel_name,
            WheelName {
                distribution: "requests".to_string(),
                version: Version::from_str("2.29.0").unwrap(),
                build_tag: Some(BuildTag {
                    number: 1,
                    remainder: Some("asdf".to_string()),
                }),
                python_tag: "py3".to_string(),
                abi_tag: "none".to_string(),
                platform_tag: "any".to_string(),
            },
        );
        Ok(())
    }

    #[test]
    fn test_from_str_multiple_platforms() -> Result<(), WheelNameParseError> {
        let wheel_name = WheelName::from_str("charset_normalizer-3.0.1-cp37-cp37m-manylinux_2_5_i686.manylinux1_i686.manylinux_2_17_i686.manylinux2014_i686.whl")?;
        assert_eq!(
            wheel_name,
            WheelName {
                distribution: "charset-normalizer".to_string(),
                version: Version::from_str("3.0.1").unwrap(),
                build_tag: None,
                python_tag: "cp37".to_string(),
                abi_tag: "cp37m".to_string(),
                platform_tag:
                    "manylinux_2_5_i686.manylinux1_i686.manylinux_2_17_i686.manylinux2014_i686"
                        .to_string(),
            },
        );
        Ok(())
    }

    #[test]
    fn test_from_str_underscore_name() -> Result<(), WheelNameParseError> {
        let wheel_name = WheelName::from_str("charset_normalizer-3.1.0-py3-none-any.whl")?;
        assert_eq!(
            wheel_name,
            WheelName {
                distribution: "charset-normalizer".to_string(),
                version: Version::from_str("3.1.0").unwrap(),
                build_tag: None,
                python_tag: "py3".to_string(),
                abi_tag: "none".to_string(),
                platform_tag: "any".to_string(),
            },
        );
        Ok(())
    }

    #[test]
    fn test_from_str_not_wheel() -> Result<(), WheelNameParseError> {
        let wheel_name = WheelName::from_str("charset-normalizer-3.1.0.tar.gz");
        assert_eq!(wheel_name, Err(WheelNameParseError::NotAWheel),);
        Ok(())
    }

    #[test]
    fn test_from_str_kekab() -> Result<(), WheelNameParseError> {
        // Wheel name `distribution` field is not allowed to have a dash in it.
        let wheel_name = WheelName::from_str("charset-normalizer-3.1.0-py3-none-any.whl");
        assert_eq!(
            wheel_name,
            Err(WheelNameParseError::InvalidVersion(
                "Version `normalizer` doesn't match PEP 440 rules".to_string(),
            )),
        );
        Ok(())
    }
}
