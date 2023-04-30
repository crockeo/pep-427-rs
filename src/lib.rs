use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

/// Structured version of the
/// [information contained in wheel names](https://packaging.python.org/en/latest/specifications/binary-distribution-format/#file-name-convention).
/// This does not perform any validation of each component
/// (e.g. `version` is not necessarily a valid [PEP-0440](https://peps.python.org/pep-0440/) version).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WheelInfo {
    pub distribution: String,
    pub version: String,
    pub build_tag: Option<String>,
    pub python_tag: String,
    pub abi_tag: String,
    pub platform_tag: String,
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum WheelInfoParseError {
    #[error("regex could not match wheel name")]
    FailedMatch,
}

impl FromStr for WheelInfo {
    type Err = WheelInfoParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref WHEEL_INFO_RE: Regex = Regex::new(concat!(
                "^",
                "(?P<distribution>.+)",
                "-(?P<version>.+)",
                "(-(?P<build_tag>.+))?",
                "-(?P<python_tag>.+)",
                "-(?P<abi_tag>.+)",
                "-(?P<platform_tag>.+)",
                "\\.whl",
                "$",
            ))
            .expect("Failed to parse wheel name regex");
        }

        let Some(captures) = WHEEL_INFO_RE.captures(s) else {
	    return Err(WheelInfoParseError::FailedMatch);
	};

        Ok(Self {
            distribution: must_named_capture(&captures, "distribution"),
            version: must_named_capture(&captures, "version"),
            build_tag: captures
                .name("build_tag")
                .map(|m| m.as_str())
                .map(str::to_owned),
            python_tag: must_named_capture(&captures, "python_tag"),
            abi_tag: must_named_capture(&captures, "abi_tag"),
            platform_tag: must_named_capture(&captures, "platform_tag"),
        })
    }
}

fn must_named_capture(captures: &regex::Captures, name: &str) -> String {
    captures.name(name).unwrap().as_str().to_owned()
}

#[cfg(test)]
mod wheel_info_tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn from_str_simple() -> Result<(), WheelInfoParseError> {
        let wheel_info = WheelInfo::from_str("requests-2.29.0-py3-none-any.whl")?;
        assert_eq!(
            wheel_info,
            WheelInfo {
                distribution: "requests".to_string(),
                version: "2.29.0".to_string(),
                build_tag: None,
		python_tag: "py3".to_string(),
		abi_tag: "none".to_string(),
                platform_tag: "any".to_string(),
            },
        );
        Ok(())
    }

    #[test]
    fn from_str_build_tag() -> Result<(), WheelInfoParseError> {
        let wheel_info = WheelInfo::from_str("requests-2.29.0-1-py3-none-any.whl")?;
        assert_eq!(
            wheel_info,
            WheelInfo {
                distribution: "requests".to_string(),
                version: "2.29.0".to_string(),
		build_tag: Some("1".to_string()),
		python_tag: "py3".to_string(),
		abi_tag: "none".to_string(),
                platform_tag: "any".to_string(),
            },
        );
        Ok(())
    }

    #[test]
    fn from_str_underscore_name() -> Result<(), WheelInfoParseError> {
	let wheel_info = WheelInfo::from_str("charset_normalizer-3.1.0-py3-none-any.whl")?;
        assert_eq!(
            wheel_info,
            WheelInfo {
                distribution: "charset_normalizer".to_string(),
                version: "3.1.0".to_string(),
                build_tag: None,
		python_tag: "py3".to_string(),
		abi_tag: "none".to_string(),
                platform_tag: "any".to_string(),
            },
        );
	Ok(())
    }

    #[test]
    fn from_str_not_wheel() -> Result<(), WheelInfoParseError> {
	let wheel_info = WheelInfo::from_str("charset-normalizer-3.1.0.tar.gz");
	assert_eq!(
	    wheel_info,
	    Err(WheelInfoParseError::FailedMatch),
	);
	Ok(())
    }

    #[test]
    fn from_str_kekab() -> Result<(), WheelInfoParseError> {
	// Wheel name `distribution` field is not allowed to have a dash in it.
	// This is an invalid wheel name because the distribution
	// `charset-normalizer` has not been noramlized to
	// `charset_normalizer`.
	let wheel_info = WheelInfo::from_str("charset-normalizer-3.1.0-py3-none-any.whl");
        assert_eq!(
	    wheel_info,
	    Err(WheelInfoParseError::FailedMatch),
        );
	Ok(())
    }
}
