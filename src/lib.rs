//! This crate provides a structured interface to reading Python wheel metadata.
//! See [PyPA docs on wheels](https://packaging.python.org/en/latest/specifications/binary-distribution-format/)
//! for more information.

mod metadata_file;
mod record_file;
mod wheel_file;
mod wheel_name;

pub use metadata_file::MetadataFile;
pub use record_file::RecordFile;
pub use wheel_file::WheelFile;
pub use wheel_name::WheelName;
