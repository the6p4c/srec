use std::error;
use std::fs;
use std::io::Write;

/// Generates an SREC file, and creates/overwrites out.mot with the generated
/// string
fn main() -> Result<(), Box<dyn error::Error>> {
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

    let s = srec::generate_srec_file(&records);

    let mut file = fs::File::create("out.mot")?;
    file.write_all(&s.into_bytes())?;

    Ok(())
}
