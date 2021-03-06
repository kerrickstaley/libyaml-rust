// Allow unstable items until Rust hits 1.0
#![feature(io)]

extern crate yaml;

use yaml::error::YamlError;
use yaml::emitter::YamlEmitter;
use yaml::ffi::{YamlEncoding, YamlScalarStyle};

use std::error::Error;
use std::io;
use std::io::{Read, Write};

struct MockRW {
    _data: ()
}

impl MockRW {
    pub fn new() -> MockRW {
        MockRW { _data: () }
    }
}

impl Read for MockRW {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "",
            Some("mock reader".to_string())
        ))
    }
}

impl Write for MockRW {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "",
            Some("mock writer".to_string())
        ))
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "",
            Some("mock writer".to_string())
        ))
    }
}

#[test]
fn error_cause_test_read() {
    let mut mock_reader = MockRW::new();
    match yaml::parse_io_utf8(&mut mock_reader) {
        Ok(_) => panic!("Should return an error"),
        Err(e) => assert_eq!(e.cause().map(|ioe| format!("{}", ioe)), Some(" (mock reader)".to_string()))
    }
}

fn write_to_bad_stream() -> Result<(), YamlError> {
    let mut mock_writer = MockRW::new();
    let mut emitter = YamlEmitter::init(&mut mock_writer);
    try!(emitter.emit_stream(YamlEncoding::YamlUtf8Encoding, |stream|
        stream.emit_document(None, &[], true, |doc| {
            doc.emit_scalar_event(None, None, "a", true, false, YamlScalarStyle::YamlPlainScalarStyle)
        })
    ));
    emitter.flush()
}

#[test]
fn error_cause_test_write() {
    match write_to_bad_stream() {
        Ok(_) => panic!("Should return an error"),
        Err(e) => assert_eq!(e.cause().map(|ioe| format!("{}", ioe)), Some(" (mock writer)".to_string()))
    }
}
