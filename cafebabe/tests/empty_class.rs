use cafebabe::constant_pool::ConstantPoolEntry;
use cafebabe::{ClassFile, read_class_data};
use common::setup_logging;

mod common;

use std::io::BufReader;
use std::{fs::File, io::Read};

#[test]
fn reads_java8_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java8/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 52);
    validate_constant_pool(&class_file);
}
#[test]
fn reads_java11_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java11/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 55);
    validate_constant_pool(&class_file);
}

#[test]
fn reads_java17_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java17/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 61);
    validate_constant_pool(&class_file);
}

#[test]
fn reads_java21_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java21/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 65);
    validate_constant_pool(&class_file);
}

fn validate_constant_pool(class_file: &ClassFile) {
    let pool = &class_file.constant_pool;
    let items = &pool.items;
    assert_eq!(items.len(), 12);
    items.iter().for_each(|item| match item {
        ConstantPoolEntry::String(value) => assert_eq!(
            value.len() > 0,
            true,
            "Unexpected empty string in constant pool"
        ),
        ConstantPoolEntry::MethodRef(methodRef) => {
            assert_eq!(
                methodRef.class_ref < items.len() as u16,
                true,
                "Class reference outside of constant pool"
            );
            assert_eq!(
                methodRef.name_type_ref < items.len() as u16,
                true,
                "Name and type reference outside of constant pool"
            );
        }
        _ => (),
    })
}

fn reads_empty_class(path: &str) -> cafebabe::ClassFile {
    let file = File::open(path).expect("Can't open class file");
    let mut reader = BufReader::new(file);
    let mut data: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut data)
        .expect("Can't read class file into memory");

    let result = read_class_data(&data);

    assert_eq!(result.is_ok(), true);

    result.unwrap()
}
