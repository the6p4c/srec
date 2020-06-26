//! Parsing (reading) and generation (writing) of [Motorola
//! S-record](https://en.wikipedia.org/wiki/SREC_\(file_format\)) (also known
//! as SRECORD or SREC) files
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![warn(clippy::cargo)]

mod checksum;
pub mod reader;
mod record;
pub mod writer;

pub use reader::{read_records, Error as ReaderError};
pub use record::*;
pub use writer::generate_srec_file;
