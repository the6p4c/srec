//! Generation of SREC records and files
use crate::checksum::checksum_of;
use crate::record::*;

fn make_record(t: u8, address: &impl Address, data: &[u8]) -> String {
    assert!(t < 10, "invalid record type {}", t);

    let mut bytes = vec![0x00];
    bytes.extend(address.to_be_bytes());
    bytes.extend(data);
    bytes[0] = (bytes.len() - 1 + 1) as u8;

    let bytes_str = bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join("");

    format!("S{}{}{:02X}", t, bytes_str, checksum_of(&bytes))
}

impl Record {
    fn encode(&self) -> String {
        match self {
            Record::S0(s) => make_record(0, &Address16(0x0000), &s.bytes().collect::<Vec<_>>()),
            Record::S1(Data { address, data }) => make_record(1, address, data),
            Record::S2(Data { address, data }) => make_record(2, address, data),
            Record::S3(Data { address, data }) => make_record(3, address, data),
            Record::S5(Count16(c)) => make_record(5, &Address16(*c), &[]),
            Record::S6(Count24(c)) => make_record(6, &Address24(*c), &[]),
            Record::S7(address) => make_record(7, address, &[]),
            Record::S8(address) => make_record(8, address, &[]),
            Record::S9(address) => make_record(9, address, &[]),
        }
    }
}

/// Converts each provided record to a string, joining them with newlines ('\n')
/// to generate an LF terminated SREC file
///
/// Does not perform any validation on the provided records. The caller is
/// responsible for ensuring records do not contain duplicate/overlapping data
/// and that records are in the correct order.
///
/// # Examples
///
/// ```rust
/// let s = srec::writer::generate_srec_file(&[
///     srec::Record::S0("HDR".into()),
///     srec::Record::S1(srec::Data {
///         address: srec::Address16(0x1234),
///         data: vec![0x00, 0x01, 0x02, 0x03],
///     }),
///     srec::Record::S1(srec::Data {
///         address: srec::Address16(0x1238),
///         data: vec![0x04, 0x05, 0x06, 0x07],
///     }),
///     srec::Record::S9(srec::Address16(0x1234)),
/// ]);
///
/// assert_eq!(
///     s,
///     "S00600004844521B\nS107123400010203AC\nS10712380405060798\nS9031234B6\n"
/// );
/// ```
pub fn generate_srec_file(records: &[Record]) -> String {
    records
        .iter()
        .map(Record::encode)
        .map(|s| {
            let mut s2 = s.clone();
            s2.push('\n');
            s2
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_s0_empty_string_returns_empty_record() {
        let r = Record::S0("".into());

        let s = r.encode();

        assert_eq!(s, "S0030000FC");
    }

    #[test]
    fn encode_s0_simple_string_returns_correct_record() {
        let r = Record::S0("HDR".into());

        let s = r.encode();

        assert_eq!(s, "S00600004844521B");
    }

    #[test]
    fn encode_s1_empty_returns_empty_record() {
        let r = Record::S1(Data {
            address: Address16(0x1234),
            data: vec![],
        });

        let s = r.encode();

        assert_eq!(s, "S1031234B6");
    }

    #[test]
    fn encode_s1_with_data_returns_correct_record() {
        let r = Record::S1(Data {
            address: Address16(0x1234),
            data: vec![0x00, 0x01, 0x02, 0x03],
        });

        let s = r.encode();

        assert_eq!(s, "S107123400010203AC");
    }

    #[test]
    fn encode_s2_empty_returns_empty_record() {
        let r = Record::S2(Data {
            address: Address24(0x123456),
            data: vec![],
        });

        let s = r.encode();

        assert_eq!(s, "S2041234565F");
    }

    #[test]
    fn encode_s2_with_data_returns_correct_record() {
        let r = Record::S2(Data {
            address: Address24(0x123456),
            data: vec![0x00, 0x01, 0x02, 0x03],
        });

        let s = r.encode();

        assert_eq!(s, "S2081234560001020355");
    }

    #[test]
    fn encode_s3_empty_returns_empty_record() {
        let r = Record::S3(Data {
            address: Address32(0x12345678),
            data: vec![],
        });

        let s = r.encode();

        assert_eq!(s, "S30512345678E6");
    }

    #[test]
    fn encode_s3_with_data_returns_correct_record() {
        let r = Record::S3(Data {
            address: Address32(0x12345678),
            data: vec![0x00, 0x01, 0x02, 0x03],
        });

        let s = r.encode();

        assert_eq!(s, "S3091234567800010203DC");
    }

    #[test]
    fn encode_s5_returns_correct_record() {
        let r = Record::S5(Count16(0x1234));

        let s = r.encode();

        assert_eq!(s, "S5031234B6");
    }

    #[test]
    fn encode_s6_returns_correct_record() {
        let r = Record::S6(Count24(0x123456));

        let s = r.encode();

        assert_eq!(s, "S6041234565F");
    }

    #[test]
    fn encode_s7_returns_correct_record() {
        let r = Record::S7(Address32(0x12345678));

        let s = r.encode();

        assert_eq!(s, "S70512345678E6");
    }

    #[test]
    fn encode_s8_returns_correct_record() {
        let r = Record::S8(Address24(0x123456));

        let s = r.encode();

        assert_eq!(s, "S8041234565F");
    }

    #[test]
    fn encode_s9_returns_correct_record() {
        let r = Record::S9(Address16(0x1234));

        let s = r.encode();

        assert_eq!(s, "S9031234B6");
    }

    #[test]
    fn generate_srec_file_empty_list_returns_empty_string() {
        let r = [];

        let s = generate_srec_file(&r);

        assert_eq!(s, "");
    }

    #[test]
    fn generate_srec_file_one_record_returns_single_record_newline_terminated() {
        let r = [Record::S0("HDR".into())];

        let s = generate_srec_file(&r);

        assert_eq!(s, "S00600004844521B\n");
    }

    #[test]
    fn generate_srec_file_multiple_records_returns_all_records_joined_by_newline() {
        let r = [
            Record::S0("HDR".into()),
            Record::S1(Data {
                address: Address16(0x1234),
                data: vec![0x00, 0x01, 0x02, 0x03],
            }),
            Record::S1(Data {
                address: Address16(0x1238),
                data: vec![0x04, 0x05, 0x06, 0x07],
            }),
            Record::S9(Address16(0x1234)),
        ];

        let s = generate_srec_file(&r);

        assert_eq!(
            s,
            "S00600004844521B\nS107123400010203AC\nS10712380405060798\nS9031234B6\n"
        );
    }
}
