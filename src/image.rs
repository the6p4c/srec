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
        let new_block_range = address..(address + data.len() as u32);

        for block in &self.blocks {
            let block_range = block.address..(block.address + block.data.len() as u32);

            for addr in block_range {
                if new_block_range.contains(&addr) {
                    panic!(
                        "New block (at {:#x}, length {:#x}) overlaps with existing block (at {:#x}, length {:#x})",
                        address, data.len(),
                        block.address, block.data.len()
                    );
                }
            }
        }

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
    fn new_returns_empty_image() {
        let i = Image::new();

        assert_eq!(i.blocks, vec![]);
    }

    #[test]
    fn add_data_allocates_new_block() {
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
    fn add_data_non_contiguous_after_allocates_new_block() {
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
    fn add_data_non_contiguous_before_allocates_new_block() {
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
    fn add_data_contiguous_after_merges_blocks() {
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
    fn add_data_contiguous_before_merges_blocks() {
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
    fn add_data_contiguous_middle_merges_blocks() {
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

    #[test]
    #[should_panic]
    fn add_data_overlapping_after_panics() {
        let mut i = Image::new();

        i.add_data(0x00000000, &vec![0x11, 0x22, 0x33, 0x44]);
        i.add_data(0x00000003, &vec![0x55, 0x66, 0x77, 0x88]);
    }

    #[test]
    #[should_panic]
    fn add_data_overlapping_before_panics() {
        let mut i = Image::new();

        i.add_data(0x00000003, &vec![0x55, 0x66, 0x77, 0x88]);
        i.add_data(0x00000000, &vec![0x11, 0x22, 0x33, 0x44]);
    }
}
