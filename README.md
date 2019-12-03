srec
====
A Rust crate for parsing/reading and generating/writing [Motorola S-record](https://en.wikipedia.org/wiki/SREC_\(file_format\)) (also known as SRECORD or SREC) files.

[View documentation on docs.rs](https://docs.rs/srec/)

# Examples
## Reading
See [`examples/read_srec_file.rs`](/examples/read_srec_file.rs)

```rust
let s = fs::read_to_string(path)?;

let records = srec::reader::read_records(&s);

for record in records {
	println!("{:x?}", record?);
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
