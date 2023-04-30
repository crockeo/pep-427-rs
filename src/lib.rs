use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

pub struct WheelInfo {
    pub distribution: String,
    pub version: String,
    pub build_tag: Option<String>,
    pub python_tag: String,
    pub platform_tag: String,
}

#[derive(Debug)]
pub enum WheelInfoParseError {
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
            platform_tag: must_named_capture(&captures, "platform_tag"),
        })
    }
}

fn must_named_capture(captures: &regex::Captures, name: &str) -> String {
    captures.name(name).unwrap().as_str().to_owned()
}
