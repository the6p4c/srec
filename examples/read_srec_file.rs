use std::env;
use std::error;
use std::fs;

/// Reads every record from the file specified in the first command line
/// argument. See `examples/demo.mot` for an example SREC file.
fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let path = args
        .get(1)
        .expect("An SREC file must be provided as a command line argument");
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

    Ok(())
}
