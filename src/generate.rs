//! Generation of SREC records and files
use crate::checksum::checksum_of;
use crate::image::{Block, Image};
use crate::record::*;
use std::convert::TryInto;

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

    format!("S{}{}{:02X}", t, bytes_str, checksum_of(&bytes))
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

enum AddressFormat {
    SmallestRequired,
    Address16,
    Address24,
    Address32,
}

fn image_records<'a>(
    i: &'a Image,
    address_format: AddressFormat,
) -> impl Iterator<Item = Record> + 'a {
    let max_address = i
        .blocks
        .iter()
        .flat_map(move |block| {
            block
                .data
                .chunks(32)
                .enumerate()
                .map(move |(i, chunk)| Block {
                    address: block.address + (i as u32) * 32,
                    data: chunk.to_vec(),
                })
        })
        .map(|block| block.address)
        .max();

    i.blocks
        .iter()
        .flat_map(move |block| {
            block
                .data
                .chunks(32)
                .enumerate()
                .map(move |(i, chunk)| Block {
                    address: block.address + (i as u32) * 32,
                    data: chunk.to_vec(),
                })
        })
        .map(move |block| match address_format {
            AddressFormat::Address16 => Record::S1(Data {
                address: Address16(block.address.try_into().unwrap()),
                data: block.data,
            }),
            AddressFormat::Address24 => {
                if block.address > 0xFFFFFF {
                    // TODO: This is awful, find a better solution
                    panic!();
                }

                Record::S2(Data {
                    address: Address24(block.address.try_into().unwrap()),
                    data: block.data,
                })
            }
            AddressFormat::Address32 => Record::S3(Data {
                address: Address32(block.address),
                data: block.data,
            }),
            AddressFormat::SmallestRequired => match max_address.unwrap() {
                0...0xFFFF => Record::S1(Data {
                    address: Address16(block.address.try_into().unwrap()),
                    data: block.data,
                }),
                0x10000...0xFFFFFF => Record::S2(Data {
                    address: Address24(block.address.try_into().unwrap()),
                    data: block.data,
                }),
                _ => Record::S3(Data {
                    address: Address32(block.address),
                    data: block.data,
                }),
            },
        })
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

    #[test]
    fn image_records_empty_image_returns_empty_iterator() {
        let i = Image::new();

        let mut rs = image_records(&i, AddressFormat::Address32);

        assert_eq!(rs.next(), None);
    }

    #[test]
    fn image_records_address_format_address_16_provided_16_uses_correct_record() {
        let mut i = Image::new();
        i.add_data(0x00001234, &vec![0x11, 0x22, 0x33, 0x44]);

        let mut rs = image_records(&i, AddressFormat::Address16);

        assert_eq!(
            rs.next(),
            Some(Record::S1(Data {
                address: Address16(0x1234),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(rs.next(), None);
    }

    #[test]
    #[should_panic]
    fn image_records_address_format_address_16_provided_24_panics() {
        let mut i = Image::new();
        i.add_data(0x123456, &vec![0x11, 0x22, 0x33, 0x44]);

        let rs = image_records(&i, AddressFormat::Address16);

        for _ in rs {}
    }

    #[test]
    #[should_panic]
    fn image_records_address_format_address_16_provided_32_panics() {
        let mut i = Image::new();
        i.add_data(0x12345678, &vec![0x11, 0x22, 0x33, 0x44]);

        let rs = image_records(&i, AddressFormat::Address16);

        for _ in rs {}
    }

    #[test]
    fn image_records_address_format_address_24_provided_16_uses_correct_record() {
        let mut i = Image::new();
        i.add_data(0x00001234, &vec![0x11, 0x22, 0x33, 0x44]);

        let mut rs = image_records(&i, AddressFormat::Address24);

        assert_eq!(
            rs.next(),
            Some(Record::S2(Data {
                address: Address24(0x1234),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(rs.next(), None);
    }

    #[test]
    fn image_records_address_format_address_24_provided_24_uses_correct_record() {
        let mut i = Image::new();
        i.add_data(0x123456, &vec![0x11, 0x22, 0x33, 0x44]);

        let mut rs = image_records(&i, AddressFormat::Address24);

        assert_eq!(
            rs.next(),
            Some(Record::S2(Data {
                address: Address24(0x123456),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(rs.next(), None);
    }

    #[test]
    #[should_panic]
    fn image_records_address_format_address_24_provided_32_panics() {
        let mut i = Image::new();
        i.add_data(0x12345678, &vec![0x11, 0x22, 0x33, 0x44]);

        let rs = image_records(&i, AddressFormat::Address24);

        for _ in rs.inspect(|x| println!("{:?}", x)) {}
    }

    #[test]
    fn image_records_address_format_address_32_provided_16_uses_correct_record() {
        let mut i = Image::new();
        i.add_data(0x00001234, &vec![0x11, 0x22, 0x33, 0x44]);

        let mut rs = image_records(&i, AddressFormat::Address32);

        assert_eq!(
            rs.next(),
            Some(Record::S3(Data {
                address: Address32(0x1234),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(rs.next(), None);
    }

    #[test]
    fn image_records_address_format_address_32_provided_24_uses_correct_record() {
        let mut i = Image::new();
        i.add_data(0x123456, &vec![0x11, 0x22, 0x33, 0x44]);

        let mut rs = image_records(&i, AddressFormat::Address32);

        assert_eq!(
            rs.next(),
            Some(Record::S3(Data {
                address: Address32(0x123456),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(rs.next(), None);
    }

    #[test]
    fn image_records_address_format_address_32_provided_32_uses_correct_record() {
        let mut i = Image::new();
        i.add_data(0x12345678, &vec![0x11, 0x22, 0x33, 0x44]);

        let mut rs = image_records(&i, AddressFormat::Address32);

        assert_eq!(
            rs.next(),
            Some(Record::S3(Data {
                address: Address32(0x12345678),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(rs.next(), None);
    }

    #[test]
    fn image_records_address_format_smallest_required_16_uses_correct_record() {
        let mut i = Image::new();
        i.add_data(0x00001234, &vec![0x11, 0x22, 0x33, 0x44]);

        let mut rs = image_records(&i, AddressFormat::SmallestRequired);

        assert_eq!(
            rs.next(),
            Some(Record::S1(Data {
                address: Address16(0x1234),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(rs.next(), None);
    }

    #[test]
    fn image_records_address_format_smallest_required_24_uses_correct_record() {
        let mut i = Image::new();
        i.add_data(0x00001234, &vec![0x11, 0x22, 0x33, 0x44]);
        i.add_data(0x00123456, &vec![0x11, 0x22, 0x33, 0x44]);

        let mut rs = image_records(&i, AddressFormat::SmallestRequired);

        assert_eq!(
            rs.next(),
            Some(Record::S2(Data {
                address: Address24(0x1234),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(
            rs.next(),
            Some(Record::S2(Data {
                address: Address24(0x123456),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(rs.next(), None);
    }

    #[test]
    fn image_records_address_format_smallest_required_32_uses_correct_record() {
        let mut i = Image::new();
        i.add_data(0x00001234, &vec![0x11, 0x22, 0x33, 0x44]);
        i.add_data(0x00123456, &vec![0x11, 0x22, 0x33, 0x44]);
        i.add_data(0x12345678, &vec![0x11, 0x22, 0x33, 0x44]);

        let mut rs = image_records(&i, AddressFormat::SmallestRequired);

        assert_eq!(
            rs.next(),
            Some(Record::S3(Data {
                address: Address32(0x1234),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(
            rs.next(),
            Some(Record::S3(Data {
                address: Address32(0x123456),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(
            rs.next(),
            Some(Record::S3(Data {
                address: Address32(0x12345678),
                data: vec![0x11, 0x22, 0x33, 0x44],
            }))
        );
        assert_eq!(rs.next(), None);
    }
}
