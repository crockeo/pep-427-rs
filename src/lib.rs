//! This crate provides a structured interface to reading Python wheel metadata.
//! See [PyPA docs on wheels](https://packaging.python.org/en/latest/specifications/binary-distribution-format/)
//! for more information.

mod metadata_file;
mod record_file;
mod wheel_file;
mod wheel_name;

use std::io;
use std::io::Read;
use std::io::Seek;
use std::str::FromStr;

use zip::ZipArchive;

pub use metadata_file::MetadataFile;
pub use record_file::RecordFile;
pub use wheel_file::WheelFile;
pub use wheel_name::WheelName;

pub struct Wheel<R> {
    name: WheelName,
    archive: ZipArchive<R>,
}

impl<R: Read + Seek> Wheel<R> {
    pub fn open(name: &str, reader: R) -> Result<Wheel<R>, WheelError> {
        Ok(Self {
            name: WheelName::from_str(name)?,
            archive: ZipArchive::new(reader)?,
        })
    }

    pub fn metadata_file(&mut self) -> Result<MetadataFile, WheelError> {
        todo!("replace with wheel_file-like implementation when FromStr is ready")
    }

    pub fn record_file(&mut self) -> Result<RecordFile, WheelError> {
        todo!("replace with wheel_file-like implementation when FromStr is ready")
    }

    pub fn wheel_file(&mut self) -> Result<WheelFile, WheelError> {
        Ok(WheelFile::from_str(&self.dist_info_contents("WHEEL")?)?)
    }

    pub fn wheel_name(&self) -> &WheelName {
        &self.name
    }

    fn dist_info_contents(&mut self, filename: &str) -> Result<String, WheelError> {
        // TODO: maybe don't do this, use Path/PathBuf, and make sure this works on windows
        let filename = format!(
            "{}-{}.dist.info/{}",
            self.name.distribution, self.name.version, filename
        );
        let mut zip_file = self.archive.by_name(&filename)?;
        let mut contents = String::new();
        zip_file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum WheelError {
    #[error("encountered error while parsing wheel file")]
    WheelFileParseError(wheel_file::WheelFileParseError),

    #[error("encountered error while parsing wheel name")]
    WheelNameParseError(wheel_name::WheelNameParseError),

    #[error("encountered zip error")]
    ZipError(zip::result::ZipError),

    #[error("encountered IO error")]
    IOError(io::Error),
}

impl From<wheel_file::WheelFileParseError> for WheelError {
    fn from(value: wheel_file::WheelFileParseError) -> Self {
        Self::WheelFileParseError(value)
    }
}

impl From<wheel_name::WheelNameParseError> for WheelError {
    fn from(value: wheel_name::WheelNameParseError) -> Self {
        Self::WheelNameParseError(value)
    }
}

impl From<zip::result::ZipError> for WheelError {
    fn from(value: zip::result::ZipError) -> Self {
        Self::ZipError(value)
    }
}

impl From<io::Error> for WheelError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
