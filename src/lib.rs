//! `srec`: parsing and generation of Motorola S-record (also known as SRECORD or SREC) files
#![deny(missing_docs)]

mod checksum;
pub mod reader;
mod record;
pub mod writer;
