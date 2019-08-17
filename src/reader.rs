//! Parsing of SREC records and files
use crate::checksum::checksum_of;
use crate::record::*;
use std::str::{self, FromStr};

#[derive(Debug, PartialEq)]
struct RawRecord {
    t: u8,
    bytes: Vec<u8>,
}

/// Errors which may occur during reading
#[derive(Debug, PartialEq)]
pub enum Error {
    /// String did not have enough characters
    NotEnoughData,
    /// Next character was unexpected
    UnexpectedCharacter,
    /// Record byte count field was zero (must be >= 1)
    ByteCountZero,
    /// Record checksum did not match calculated checksum
    ChecksumMismatch,
}

// Using is_empty would ruin the consistency of checking if there are enough
// characters between 1 and 2 required
#[allow(clippy::len_zero)]
fn raw_record_from_str(s: &str) -> Result<RawRecord, Error> {
    // Read initial "S" character
    if s.len() < 1 {
        return Err(Error::NotEnoughData);
    }

    let (first_char, s) = s.split_at(1);

    if first_char != "S" {
        return Err(Error::UnexpectedCharacter);
    }

    // Read type field
    if s.len() < 1 {
        return Err(Error::NotEnoughData);
    }

    let (type_str, s) = s.split_at(1);

    let t = type_str
        .parse::<u8>()
        .map_err(|_| Error::UnexpectedCharacter)?;

    // Read byte count field
    if s.len() < 2 {
        return Err(Error::NotEnoughData);
    }

    let (byte_count_str, s) = s.split_at(2);

    let byte_count =
        usize::from_str_radix(byte_count_str, 16).map_err(|_| Error::UnexpectedCharacter)?;

    if byte_count == 0 {
        return Err(Error::ByteCountZero);
    }

    // Read payload bytes (including checksum)
    let mut bytes: Vec<u8> = Vec::with_capacity(byte_count);

    let mut s = s;
    for _ in 0..byte_count {
        if s.len() < 2 {
            return Err(Error::NotEnoughData);
        }

        let (byte_str, s2) = s.split_at(2);
        s = s2;

        bytes.push(u8::from_str_radix(byte_str, 16).map_err(|_| Error::UnexpectedCharacter)?);
    }

    let checksum = bytes.pop().unwrap();

    // TODO: Calculate checksum without having to essentially clone the bytes, maybe make
    // checksum_of take an iterator?
    let mut checksum_bytes = vec![byte_count as u8];
    checksum_bytes.extend(&bytes);
    let checksum_valid = checksum == checksum_of(&checksum_bytes);

    if checksum_valid {
        Ok(RawRecord { t, bytes })
    } else {
        Err(Error::ChecksumMismatch)
    }
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rr = raw_record_from_str(s)?;

        let r = match rr.t {
            0 => Record::S0(
                str::from_utf8(&rr.bytes[2..])
                    .expect("Invalid UTF-8 bytes in S0 data")
                    .trim_end_matches('\0')
                    .to_string(),
            ),
            1 => {
                if rr.bytes.len() < 2 {
                    return Err(Error::NotEnoughData);
                }

                let (address_bytes, data) = rr.bytes.split_at(2);

                let mut address = [0u8; 2];
                address.copy_from_slice(address_bytes);
                let address = u16::from_be_bytes(address);

                Record::S1(Data {
                    address: Address16(address),
                    data: data.to_vec(),
                })
            }
            2 => {
                if rr.bytes.len() < 3 {
                    return Err(Error::NotEnoughData);
                }

                let (address_bytes, data) = rr.bytes.split_at(3);

                let mut address = [0u8; 4];
                address[1..].copy_from_slice(address_bytes);
                let address = u32::from_be_bytes(address);

                Record::S2(Data {
                    address: Address24(address),
                    data: data.to_vec(),
                })
            }
            3 => {
                if rr.bytes.len() < 4 {
                    return Err(Error::NotEnoughData);
                }

                let (address_bytes, data) = rr.bytes.split_at(4);

                let mut address = [0u8; 4];
                address.copy_from_slice(address_bytes);
                let address = u32::from_be_bytes(address);

                Record::S3(Data {
                    address: Address32(address),
                    data: data.to_vec(),
                })
            }
            5 => {
                if rr.bytes.len() != 2 {
                    return Err(Error::NotEnoughData);
                }

                let mut count = [0u8; 2];
                count.copy_from_slice(&rr.bytes);
                let count = u16::from_be_bytes(count);

                Record::S5(Count16(count))
            }
            6 => {
                if rr.bytes.len() != 3 {
                    return Err(Error::NotEnoughData);
                }

                let mut count = [0u8; 4];
                count[1..].copy_from_slice(&rr.bytes);
                let count = u32::from_be_bytes(count);

                Record::S6(Count24(count))
            }
            7 => {
                if rr.bytes.len() != 4 {
                    return Err(Error::NotEnoughData);
                }

                let mut address = [0u8; 4];
                address.copy_from_slice(&rr.bytes);
                let address = u32::from_be_bytes(address);

                Record::S7(Address32(address))
            }
            8 => {
                if rr.bytes.len() != 3 {
                    return Err(Error::NotEnoughData);
                }

                let mut address = [0u8; 4];
                address[1..].copy_from_slice(&rr.bytes);
                let address = u32::from_be_bytes(address);

                Record::S8(Address24(address))
            }
            9 => {
                if rr.bytes.len() != 2 {
                    return Err(Error::NotEnoughData);
                }

                let mut address = [0u8; 2];
                address.copy_from_slice(&rr.bytes);
                let address = u16::from_be_bytes(address);

                Record::S9(Address16(address))
            }
            _ => return Err(Error::UnexpectedCharacter),
        };

        Ok(r)
    }
}

/// Reads records from a newline separated (either "\n" or "\r\n") string,
/// returning an iterator over them
///
/// Does not validate file consistency as a whole - data records may overlap and
/// start address records may be duplicated.
///
/// # Examples
///
/// ```rust
/// let mut records = srec::reader::read_records(
///     "S00600004844521B\nS107123400010203AC\nS10712380405060798\nS9031234B6\n"
/// );
///
/// for record in records {
///     println!("{:?}", record);
/// }
/// ```
pub fn read_records<'a>(s: &'a str) -> impl Iterator<Item = Result<Record, Error>> + 'a {
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Record>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_record_from_str_empty_str_returns_err_not_enough_data() {
        let s = "";

        let rr = raw_record_from_str(s);

        assert_eq!(rr, Err(Error::NotEnoughData));
    }

    #[test]
    fn raw_record_from_str_first_character_invalid_returns_err_unexpected_character() {
        let s = "D";

        let rr = raw_record_from_str(s);

        assert_eq!(rr, Err(Error::UnexpectedCharacter));
    }

    #[test]
    fn raw_record_from_str_no_type_value_returns_err_not_enough_data() {
        let s = "S";

        let rr = raw_record_from_str(s);

        assert_eq!(rr, Err(Error::NotEnoughData));
    }

    #[test]
    fn raw_record_from_str_invalid_type_value_returns_err_unexpected_character() {
        let s = "Sx";

        let rr = raw_record_from_str(s);

        assert_eq!(rr, Err(Error::UnexpectedCharacter));
    }

    #[test]
    fn raw_record_from_str_byte_count_zero_returns_err_byte_count_zero() {
        let s = "S100";

        let rr = raw_record_from_str(s);

        assert_eq!(rr, Err(Error::ByteCountZero));
    }

    #[test]
    fn raw_record_from_str_invalid_hex_character_returns_err_unexpected_character() {
        let s = "S104123400xx";

        let rr = raw_record_from_str(s);

        assert_eq!(rr, Err(Error::UnexpectedCharacter));
    }

    #[test]
    fn raw_record_from_str_byte_count_too_large_returns_err_not_enough_data() {
        let s = "S1100000FFEF";

        let rr = raw_record_from_str(s);

        assert_eq!(rr, Err(Error::NotEnoughData));
    }

    #[test]
    fn raw_record_from_str_valid_record_empty_returns_ok_correct_raw_record() {
        let s = "S101FE";

        let rr = raw_record_from_str(s);

        assert_eq!(
            rr,
            Ok(RawRecord {
                t: 1,
                bytes: vec![]
            })
        );
    }

    #[test]
    fn raw_record_from_str_valid_record_valid_checksum_returns_ok_correct_raw_record() {
        let s = "S1101234000102030405060708090A0B0C5B";

        let rr = raw_record_from_str(s);

        assert_eq!(
            rr,
            Ok(RawRecord {
                t: 1,
                bytes: vec![
                    0x12, 0x34, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                    0x0b, 0x0c
                ],
            })
        );
    }

    #[test]
    fn raw_record_from_str_valid_record_invalid_checksum_returns_ok_correct_raw_record() {
        let s = "S1101234000102030405060708090A0B0CFF";

        let rr = raw_record_from_str(s);

        assert_eq!(rr, Err(Error::ChecksumMismatch));
    }

    #[test]
    fn s0_empty_string_from_str_returns_correct_record() {
        let s = "S0030000FC";

        let r = s.parse::<Record>();

        assert_eq!(r, Ok(Record::S0("".to_string())));
    }

    #[test]
    fn s0_simple_string_from_str_returns_correct_record() {
        let s = "S00600004844521B";

        let r = s.parse::<Record>();

        assert_eq!(r, Ok(Record::S0("HDR".to_string())));
    }

    #[test]
    fn s0_null_terminated_string_from_str_returns_correct_record() {
        let s = "S009000048445200000018";

        let r = s.parse::<Record>();

        assert_eq!(r, Ok(Record::S0("HDR".to_string())));
    }

    #[test]
    fn s1_empty_from_str_returns_correct_record() {
        let s = "S1031234B6";

        let r = s.parse::<Record>();

        assert_eq!(
            r,
            Ok(Record::S1(Data {
                address: Address16(0x1234),
                data: vec![]
            }))
        );
    }

    #[test]
    fn s1_with_data_from_str_returns_correct_record() {
        let s = "S107123400010203AC";

        let r = s.parse::<Record>();

        assert_eq!(
            r,
            Ok(Record::S1(Data {
                address: Address16(0x1234),
                data: vec![0x00, 0x01, 0x02, 0x03]
            }))
        );
    }

    #[test]
    fn s1_invalid_from_str_returns_err_not_enough_data() {
        let s = "S10212EB";

        let r = s.parse::<Record>();

        assert_eq!(r, Err(Error::NotEnoughData));
    }

    #[test]
    fn s2_empty_from_str_returns_correct_record() {
        let s = "S2041234565F";

        let r = s.parse::<Record>();

        assert_eq!(
            r,
            Ok(Record::S2(Data {
                address: Address24(0x123456),
                data: vec![]
            }))
        );
    }

    #[test]
    fn s2_with_data_from_str_returns_correct_record() {
        let s = "S2081234560001020355";

        let r = s.parse::<Record>();

        assert_eq!(
            r,
            Ok(Record::S2(Data {
                address: Address24(0x123456),
                data: vec![0x00, 0x01, 0x02, 0x03]
            }))
        );
    }

    #[test]
    fn s2_invalid_from_str_returns_err_not_enough_data() {
        let s = "S2031234B6";

        let r = s.parse::<Record>();

        assert_eq!(r, Err(Error::NotEnoughData));
    }

    #[test]
    fn s3_empty_from_str_returns_correct_record() {
        let s = "S30512345678E6";

        let r = s.parse::<Record>();

        assert_eq!(
            r,
            Ok(Record::S3(Data {
                address: Address32(0x12345678),
                data: vec![]
            }))
        );
    }

    #[test]
    fn s3_with_data_from_str_returns_correct_record() {
        let s = "S3091234567800010203DC";

        let r = s.parse::<Record>();

        assert_eq!(
            r,
            Ok(Record::S3(Data {
                address: Address32(0x12345678),
                data: vec![0x00, 0x01, 0x02, 0x03]
            }))
        );
    }

    #[test]
    fn s3_invalid_from_str_returns_err_not_enough_data() {
        let s = "S3041234565F";

        let r = s.parse::<Record>();

        assert_eq!(r, Err(Error::NotEnoughData));
    }

    #[test]
    fn s5_returns_correct_record() {
        let s = "S5031234B6";

        let r = s.parse::<Record>();

        assert_eq!(r, Ok(Record::S5(Count16(0x1234))));
    }

    #[test]
    fn s5_invalid_from_str_returns_err_not_enough_data() {
        let s = "S50212EB";

        let r = s.parse::<Record>();

        assert_eq!(r, Err(Error::NotEnoughData));
    }

    #[test]
    fn s6_returns_correct_record() {
        let s = "S6041234565F";

        let r = s.parse::<Record>();

        assert_eq!(r, Ok(Record::S6(Count24(0x123456))));
    }

    #[test]
    fn s6_invalid_from_str_returns_err_not_enough_data() {
        let s = "S6031234B6";

        let r = s.parse::<Record>();

        assert_eq!(r, Err(Error::NotEnoughData));
    }

    #[test]
    fn s7_returns_correct_record() {
        let s = "S70512345678E6";

        let r = s.parse::<Record>();

        assert_eq!(r, Ok(Record::S7(Address32(0x12345678))));
    }

    #[test]
    fn s7_invalid_from_str_returns_err_not_enough_data() {
        let s = "S7041234565F";

        let r = s.parse::<Record>();

        assert_eq!(r, Err(Error::NotEnoughData));
    }

    #[test]
    fn s8_returns_correct_record() {
        let s = "S8041234565F";

        let r = s.parse::<Record>();

        assert_eq!(r, Ok(Record::S8(Address24(0x123456))));
    }

    #[test]
    fn s8_invalid_from_str_returns_err_not_enough_data() {
        let s = "S8031234B6";

        let r = s.parse::<Record>();

        assert_eq!(r, Err(Error::NotEnoughData));
    }

    #[test]
    fn s9_returns_correct_record() {
        let s = "S9031234B6";

        let r = s.parse::<Record>();

        assert_eq!(r, Ok(Record::S9(Address16(0x1234))));
    }

    #[test]
    fn s9_invalid_from_str_returns_err_not_enough_data() {
        let s = "S90212EB";

        let r = s.parse::<Record>();

        assert_eq!(r, Err(Error::NotEnoughData));
    }

    #[test]
    fn record_from_str_returns_err_unexpected_character_on_unknown_type() {
        let s = "S401FE";

        let r = s.parse::<Record>();

        assert_eq!(r, Err(Error::UnexpectedCharacter));
    }

    #[test]
    fn read_records_empty_string_returns_empty_iterator() {
        let s = "";

        let mut ri = read_records(s);

        assert_eq!(ri.next(), None);
    }

    #[test]
    fn read_records_one_line_returns_iterator_with_one_item() {
        let s = "S00600004844521B";

        let mut ri = read_records(s);

        assert_eq!(ri.next(), Some(Ok(Record::S0("HDR".to_string()))));
        assert_eq!(ri.next(), None);
    }

    #[test]
    fn read_records_one_line_with_trailing_newline_returns_iterator_with_one_item() {
        let s = "S00600004844521B\n";

        let mut ri = read_records(s);

        assert_eq!(ri.next(), Some(Ok(Record::S0("HDR".to_string()))));
        assert_eq!(ri.next(), None);
    }

    #[test]
    fn read_records_one_line_with_empty_line_returns_iterator_with_one_item() {
        let s = "S00600004844521B\n\n";

        let mut ri = read_records(s);

        assert_eq!(ri.next(), Some(Ok(Record::S0("HDR".to_string()))));
        assert_eq!(ri.next(), None);
    }

    #[test]
    fn read_records_multiple_lines_returns_iterator_containing_all() {
        let s = "S00600004844521B\nS107123400010203AC";

        let mut ri = read_records(s);

        assert_eq!(ri.next(), Some(Ok(Record::S0("HDR".to_string()))));
        assert_eq!(
            ri.next(),
            Some(Ok(Record::S1(Data {
                address: Address16(0x1234),
                data: vec![0x00, 0x01, 0x02, 0x03],
            })))
        );
        assert_eq!(ri.next(), None);
    }
}
