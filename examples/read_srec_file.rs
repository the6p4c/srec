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

    let records = srec::reader::read_records(&s);

    for record in records {
        println!("{:x?}", record?);
    }

    Ok(())
}
