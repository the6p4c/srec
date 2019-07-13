//! `srec`: parsing and generation of Motorola S-record (also known as SRECORD or SREC) files
#![deny(missing_docs)]

use std::num::Wrapping;

pub mod generate;
pub mod parse;

trait Address {
    fn to_be_bytes(&self) -> Vec<u8>;
}

#[derive(Debug, PartialEq)]
struct Address16(u16);

impl Address for Address16 {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

// TODO: Restrict the value to 24 bits
#[derive(Debug, PartialEq)]
struct Address24(u32);

impl Address for Address24 {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes()[1..].to_vec()
    }
}

#[derive(Debug, PartialEq)]
struct Address32(u32);

impl Address for Address32 {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

#[derive(Debug, PartialEq)]
struct Count16(u16);

// TODO: Restrict the value to 24 bits
#[derive(Debug, PartialEq)]
struct Count24(u32);

#[derive(Debug, PartialEq)]
struct Data<T> {
    address: T,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
enum Record {
    S0(String),
    S1(Data<Address16>),
    S2(Data<Address24>),
    S3(Data<Address32>),
    // S4 - reserved
    S5(Count16),
    S6(Count24),
    S7(Address32),
    S8(Address24),
    S9(Address16),
}

fn checksum_of(data: &Vec<u8>) -> u8 {
    !data.iter().map(|b| Wrapping(*b)).sum::<Wrapping<u8>>().0
}

#[derive(Debug, PartialEq)]
struct Block {
    address: u32,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
struct Image {
    blocks: Vec<Block>,
}

impl Image {
    fn new() -> Image {
        Image { blocks: vec![] }
    }

    fn add_data(&mut self, address: u32, data: &Vec<u8>) {
        self.blocks.push(Block {
            address: address,
            data: data.clone(),
        });
        self.blocks
            .sort_unstable_by(|a, b| a.address.cmp(&b.address));

        loop {
            let pair = {
                let blocks_first = self.blocks.iter().enumerate();
                let blocks_last = self.blocks.iter().enumerate().skip(1);
                let mut contiguous_pairs =
                    blocks_first
                        .zip(blocks_last)
                        .filter(|((_, first), (_, last))| {
                            let first_first_address_after =
                                first.address + (first.data.len() as u32);
                            let blocks_are_contiguous = first_first_address_after == last.address;

                            blocks_are_contiguous
                        });

                contiguous_pairs.next()
            };

            if let Some(((i_first, _first), (i_last, last))) = pair {
                let last_data = last.data.clone();
                self.blocks[i_first].data.extend(last_data);
                self.blocks.remove(i_last);
                continue;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address16_to_be_bytes() {
        let a = Address16(0x1234);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34]);
    }

    #[test]
    fn address24_to_be_bytes() {
        let a = Address24(0x123456);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34, 0x56]);
    }

    #[test]
    fn address32_to_be_bytes() {
        let a = Address32(0x12345678);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn checksum_of_returns_correct_value() {
        // All sourced from the Wikipedia SREC article
        // https://en.wikipedia.org/wiki/SREC_(file_format)
        assert_eq!(
            checksum_of(&vec![
                0x13, 0x7a, 0xf0, 0x0a, 0x0a, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00
            ]),
            0x61
        );

        assert_eq!(
            checksum_of(&vec![
                0x0f, 0x00, 0x00, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00,
                0x00
            ]),
            0x3C
        );

        assert_eq!(
            checksum_of(&vec![
                0x1f, 0x00, 0x00, 0x7c, 0x08, 0x02, 0xa6, 0x90, 0x01, 0x00, 0x04, 0x94, 0x21, 0xff,
                0xf0, 0x7c, 0x6c, 0x1b, 0x78, 0x7c, 0x8c, 0x23, 0x78, 0x3c, 0x60, 0x00, 0x00, 0x38,
                0x63, 0x00, 0x00
            ]),
            0x26
        );

        assert_eq!(
            checksum_of(&vec![
                0x1f, 0x00, 0x1c, 0x4b, 0xff, 0xff, 0xe5, 0x39, 0x80, 0x00, 0x00, 0x7d, 0x83, 0x63,
                0x78, 0x80, 0x01, 0x00, 0x14, 0x38, 0x21, 0x00, 0x10, 0x7c, 0x08, 0x03, 0xa6, 0x4e,
                0x80, 0x00, 0x20
            ]),
            0xE9
        );

        assert_eq!(
            checksum_of(&vec![
                0x11, 0x00, 0x38, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64,
                0x2e, 0x0a, 0x00
            ]),
            0x42
        );

        assert_eq!(checksum_of(&vec![0x03, 0x00, 0x03]), 0xF9);

        assert_eq!(checksum_of(&vec![0x03, 0x00, 0x00]), 0xFC);
    }

    #[test]
    fn image_add_data_allocates_new_block() {
        let mut i = Image::new();

        i.add_data(0x00000000, &vec![0x11, 0x22, 0x33, 0x44]);

        assert_eq!(
            i.blocks,
            vec![Block {
                address: 0x00000000,
                data: vec![0x11, 0x22, 0x33, 0x44]
            }]
        );
    }

    #[test]
    fn image_add_data_non_contiguous_after_allocates_new_block() {
        let mut i = Image::new();

        i.add_data(0x00000000, &vec![0x11, 0x22, 0x33, 0x44]);
        i.add_data(0x00000005, &vec![0x66, 0x77, 0x88, 0x99]);

        assert_eq!(
            i.blocks,
            vec![
                Block {
                    address: 0x00000000,
                    data: vec![0x11, 0x22, 0x33, 0x44]
                },
                Block {
                    address: 0x00000005,
                    data: vec![0x66, 0x77, 0x88, 0x99],
                }
            ]
        );
    }

    #[test]
    fn image_add_data_non_contiguous_before_allocates_new_block() {
        let mut i = Image::new();

        i.add_data(0x00000005, &vec![0x66, 0x77, 0x88, 0x99]);
        i.add_data(0x00000000, &vec![0x11, 0x22, 0x33, 0x44]);

        assert_eq!(
            i.blocks,
            vec![
                Block {
                    address: 0x00000000,
                    data: vec![0x11, 0x22, 0x33, 0x44]
                },
                Block {
                    address: 0x00000005,
                    data: vec![0x66, 0x77, 0x88, 0x99],
                }
            ]
        );
    }

    #[test]
    fn image_add_data_contiguous_after_merged() {
        let mut i = Image::new();

        i.add_data(0x00000000, &vec![0x11, 0x22, 0x33, 0x44]);
        i.add_data(0x00000004, &vec![0x55, 0x66, 0x77, 0x88]);

        assert_eq!(
            i.blocks,
            vec![Block {
                address: 0x00000000,
                data: vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]
            }]
        );
    }

    #[test]
    fn image_add_data_contiguous_before_merged() {
        let mut i = Image::new();

        i.add_data(0x00000004, &vec![0x55, 0x66, 0x77, 0x88]);
        i.add_data(0x00000000, &vec![0x11, 0x22, 0x33, 0x44]);

        assert_eq!(
            i.blocks,
            vec![Block {
                address: 0x00000000,
                data: vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]
            }]
        );
    }

    #[test]
    fn image_add_data_contiguous_middle_merged() {
        let mut i = Image::new();

        i.add_data(0x00000000, &vec![0x11, 0x22, 0x33, 0x44]);
        i.add_data(0x00000005, &vec![0x66, 0x77, 0x88, 0x99]);
        i.add_data(0x00000004, &vec![0x55]);

        assert_eq!(
            i.blocks,
            vec![Block {
                address: 0x00000000,
                data: vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99]
            }]
        );
    }
}
