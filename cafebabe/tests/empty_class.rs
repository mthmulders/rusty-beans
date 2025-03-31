use cafebabe::read_class_data;
use cafebabe::types::ClassFile;
use common::setup_logging;

mod common;

use std::io::BufReader;
use std::{fs::File, io::Read};

#[test]
fn reads_java8_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java8/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 52);
}
#[test]
fn reads_java11_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java11/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 55);
}

#[test]
fn reads_java17_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java17/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 61);
}

#[test]
fn reads_java21_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java21/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 65);
}

fn reads_empty_class(path: &str) -> ClassFile {
    let file = File::open(path).expect("Can't open class file");
    let mut reader = BufReader::new(file);
    let mut data: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut data)
        .expect("Can't read class file into memory");

    let result = read_class_data(&data);

    assert_eq!(result.is_ok(), true);

    return result.unwrap();
}
