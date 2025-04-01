use cafebabe::constant_pool::{ConstantPool, ConstantPoolEntry};
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
    validate_constant_pool(class_file);
}
#[test]
fn reads_java11_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java11/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 55);
    validate_constant_pool(class_file);
}

#[test]
fn reads_java17_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java17/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 61);
    validate_constant_pool(class_file);
}

#[test]
fn reads_java21_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java21/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 65);
    validate_constant_pool(class_file);
}

fn validate_constant_pool(class_file: ClassFile) {
    let pool = class_file.constant_pool;

    assert_eq!(pool.len(), 12);

    for item in pool.items() {
        match item {
            ConstantPoolEntry::String(value) => {
                assert_eq!(
                    value.len() > 0,
                    true,
                    "Unexpected empty string in constant pool"
                )
            }
            ConstantPoolEntry::MethodRef(method_ref) => {
                let class_idx = pool
                    .class_ref_entry(method_ref.class_ref as usize - 1)
                    .unwrap();
                assert_string_class_name(&pool, class_idx);
                // TODO assert items[method_ref.name_type_ref] is indeed a ref to a name and type
                assert_eq!(
                    method_ref.name_type_ref < pool.len(),
                    true,
                    "Name and type reference outside of constant pool"
                );
            }
            ConstantPoolEntry::ClassRef(class_ref) => {
                assert_string_class_name(&pool, *class_ref);
            }
            ConstantPoolEntry::NameTypeDescriptor(name_type_descriptor) => {
                assert_string_method_name(&pool, name_type_descriptor.name_ref);
                assert_type_descriptor(&pool, name_type_descriptor.type_descriptor_ref);
            }
            _ => (),
        }
    }
}

fn assert_type_descriptor(pool: &ConstantPool, idx: u16) -> () {
    let type_descriptor = pool.string_entry(idx - 1).unwrap();
    assert_eq!(
        type_descriptor.contains("("),
        true,
        "Method descriptor points to string in unexpected format"
    );
    assert_eq!(
        type_descriptor.contains(")"),
        true,
        "Method descriptor points to string in unexpected format"
    );
}

fn assert_string_method_name(pool: &ConstantPool, idx: u16) -> () {
    let method_name = pool.string_entry(idx - 1).unwrap();
    let is_constructor = "<init>".eq(method_name);

    assert_eq!(
        is_constructor, true,
        "Method name ref points to string in unexpected format"
    );
}
fn assert_string_class_name(pool: &ConstantPool, idx: u16) -> () {
    let class_name = pool.string_entry(idx - 1).unwrap();
    assert_eq!(
        class_name.contains('/'),
        true,
        "Class ref points to string in unexpected format"
    );
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
