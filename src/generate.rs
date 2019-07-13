//! Generation of SREC records and files
use super::{checksum, Address, Address16, Address24, Address32, Count16, Count24, Data, Record};

fn make_record(t: u8, address: &impl Address, data: &Vec<u8>) -> String {
    assert!(t < 10);

    let mut bytes = vec![0x00];
    bytes.extend(address.to_be_bytes());
    bytes.extend(data);
    bytes[0] = (bytes.len() - 1 + 1) as u8;

    let bytes_str = bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join("");

    format!("S{}{}{:02X}", t, bytes_str, checksum(bytes))
}

impl Record {
    fn encode(&self) -> String {
        match self {
            Record::S0(s) => make_record(0, &Address16(0x0000), &s.bytes().collect::<Vec<_>>()),
            Record::S1(Data { address, data }) => make_record(1, address, data),
            Record::S2(Data { address, data }) => make_record(2, address, data),
            Record::S3(Data { address, data }) => make_record(3, address, data),
            Record::S5(Count16(c)) => make_record(5, &Address16(*c), &vec![]),
            Record::S6(Count24(c)) => make_record(6, &Address24(*c), &vec![]),
            Record::S7(address) => make_record(7, address, &vec![]),
            Record::S8(address) => make_record(8, address, &vec![]),
            Record::S9(address) => make_record(9, address, &vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_s0_empty_string_returns_empty_record() {
        let r = Record::S0("".to_string());

        let s = r.encode();

        assert_eq!(s, "S0030000FC");
    }

    #[test]
    fn encode_s0_simple_string_returns_correct_record() {
        let r = Record::S0("HDR".to_string());

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
}
