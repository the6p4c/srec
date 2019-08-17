#[test]
fn test_write_read() {
    let records = vec![
        srec::Record::S0("HDR".to_string()),
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

    let records2 = srec::reader::read_records(&srec::writer::write_records(&records))
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    assert_eq!(records, records2);
}

#[test]
fn test_read_write() {
    let s = "S00600004844521B\nS107123400010203AC\nS10712380405060798\nS9031234B6\n";

    let s2 = srec::writer::write_records(
        &srec::reader::read_records(&s)
            .map(Result::unwrap)
            .collect::<Vec<_>>(),
    );

    assert_eq!(s, s2);
}
