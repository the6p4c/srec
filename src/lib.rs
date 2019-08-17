//! Parsing (reading) and generation (writing) of [Motorola
//! S-record](https://en.wikipedia.org/wiki/SREC_\(file_format\)) (also known
//! as SRECORD or SREC) files
#![deny(missing_docs)]

mod checksum;
pub mod reader;
mod record;
pub mod writer;

pub use record::*;
