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
pub use record_file::{RecordEntry, RecordFile};
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
        Ok(MetadataFile::from_str(
            &self.dist_info_contents("METADATA")?,
        )?)
    }

    pub fn record_file(&mut self) -> Result<RecordFile, WheelError> {
        Ok(RecordFile::from_str(&self.dist_info_contents("RECORD")?)?)
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
    #[error(transparent)]
    MetadataFileParseError(#[from] metadata_file::MetadataFileParseError),

    #[error(transparent)]
    RecordFileParseError(#[from] record_file::RecordFileParseError),

    #[error(transparent)]
    WheelFileParseError(#[from] wheel_file::WheelFileParseError),

    #[error(transparent)]
    WheelNameParseError(#[from] wheel_name::WheelNameParseError),

    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),

    #[error(transparent)]
    IOError(#[from] io::Error),
}
