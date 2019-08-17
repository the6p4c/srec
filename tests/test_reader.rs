#[test]
fn test_read_lf() {
    let s = "S00600004844521B\nS107123400010203AC\nS10712380405060798\nS9031234B6\n";

    let mut records = srec::reader::read_records(&s);

    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S0("HDR".to_string())))
    );
    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S1(srec::Data {
            address: srec::Address16(0x1234),
            data: vec![0x00, 0x01, 0x02, 0x03],
        })))
    );
    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S1(srec::Data {
            address: srec::Address16(0x1238),
            data: vec![0x04, 0x05, 0x06, 0x07],
        })))
    );
    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S9(srec::Address16(0x1234))))
    );
    assert_eq!(records.next(), None);
}

#[test]
fn test_read_crlf() {
    let s = "S00600004844521B\r\nS107123400010203AC\r\nS10712380405060798\r\nS9031234B6\r\n";

    let mut records = srec::reader::read_records(&s);

    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S0("HDR".to_string())))
    );
    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S1(srec::Data {
            address: srec::Address16(0x1234),
            data: vec![0x00, 0x01, 0x02, 0x03],
        })))
    );
    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S1(srec::Data {
            address: srec::Address16(0x1238),
            data: vec![0x04, 0x05, 0x06, 0x07],
        })))
    );
    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S9(srec::Address16(0x1234))))
    );
    assert_eq!(records.next(), None);
}

#[test]
fn test_read_lf_with_err() {
    let s = "S00600004844521B\nS107123400010203AC\nS10712380405060798\nS9031234B4\n";

    let mut records = srec::reader::read_records(&s);

    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S0("HDR".to_string())))
    );
    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S1(srec::Data {
            address: srec::Address16(0x1234),
            data: vec![0x00, 0x01, 0x02, 0x03],
        })))
    );
    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S1(srec::Data {
            address: srec::Address16(0x1238),
            data: vec![0x04, 0x05, 0x06, 0x07],
        })))
    );
    assert_eq!(
        records.next(),
        Some(Err(srec::reader::Error::ChecksumMismatch))
    );
    assert_eq!(records.next(), None);
}

#[test]
fn test_read_crlf_with_err() {
    let s = "S00600004844521B\r\nS107123400010203AC\r\nS10712380405060798\r\nS9031234B4\r\n";

    let mut records = srec::reader::read_records(&s);

    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S0("HDR".to_string())))
    );
    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S1(srec::Data {
            address: srec::Address16(0x1234),
            data: vec![0x00, 0x01, 0x02, 0x03],
        })))
    );
    assert_eq!(
        records.next(),
        Some(Ok(srec::Record::S1(srec::Data {
            address: srec::Address16(0x1238),
            data: vec![0x04, 0x05, 0x06, 0x07],
        })))
    );
    assert_eq!(
        records.next(),
        Some(Err(srec::reader::Error::ChecksumMismatch))
    );
    assert_eq!(records.next(), None);
}
