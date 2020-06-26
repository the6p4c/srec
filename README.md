srec
====
[![crates.io badge](https://img.shields.io/crates/v/srec)](https://crates.io/crates/srec)

A Rust crate for parsing/reading and generating/writing [Motorola S-record](https://en.wikipedia.org/wiki/SREC_\(file_format\)) (also known as SRECORD or SREC) files.

[View documentation on docs.rs](https://docs.rs/srec/)

# Examples
## Reading
See [`examples/read_srec_file.rs`](/examples/read_srec_file.rs)

```rust
let s = fs::read_to_string(path)?;

let records = srec::read_records(&s);

for record in records {
    match record {
        Ok(record) => match record {
            srec::Record::S0(s) => println!("S0 header: \"{}\"", s),
            srec::Record::S1(data) => println!(
                "S1 data w/ 16-bit address: addr = {:#06X}, data = {:02X?}",
                u32::from(data.address),
                data.data
            ),
            srec::Record::S2(data) => println!(
                "S2 data w/ 24-bit address: addr = {:#08X}, data = {:02X?}",
                u32::from(data.address),
                data.data
            ),
            srec::Record::S3(data) => println!(
                "S3 data w/ 32-bit address: addr = {:#010X}, data = {:02X?}",
                u32::from(data.address),
                data.data
            ),
            srec::Record::S5(count) => {
                println!("S5 16-bit record count: count = {:#06X}", u32::from(count))
            }
            srec::Record::S6(count) => {
                println!("S6 24-bit record count: count = {:#08X}", u32::from(count))
            }
            srec::Record::S7(addr) => {
                println!("S7 32-bit start address: addr = {:#010X}", u32::from(addr))
            }
            srec::Record::S8(addr) => {
                println!("S8 24-bit start address: addr = {:#08X}", u32::from(addr))
            }
            srec::Record::S9(addr) => {
                println!("S9 16-bit start address: addr = {:#06X}", u32::from(addr))
            }
        },
        Err(err) => println!("error reading record: {}", err),
    }
}
```

## Writing
See [`examples/write_srec_file.rs`](/examples/write_srec_file.rs)

```rust
let records = [
    srec::Record::S0("HDR".into()),
    srec::Record::S1(srec::Data {
        address: srec::Address16(0x1234),
        data: vec![0x00, 0x01, 0x02, 0x03],
    }),
    srec::Record::S1(srec::Data {
        address: srec::Address16(0x1238),
        data: vec![0x04, 0x05, 0x06, 0x07],
    }),
    srec::Record::S9(srec::Address16(0x1234)),
];

let s = srec::writer::generate_srec_file(&records);

let mut file = fs::File::create(path)?;
file.write_all(&s.into_bytes())?;
```
