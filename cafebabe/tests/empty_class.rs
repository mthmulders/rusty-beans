use cafebabe::constant_pool::{ConstantPool, ConstantPoolEntry};
use cafebabe::{AccessFlags, ClassFile, read_class_data};
use common::setup_logging;

mod common;

use std::io::BufReader;
use std::{fs::File, io::Read};

static ACCESS_FLAGS: AccessFlags = AccessFlags::ACC_PUBLIC;

#[test]
fn reads_java8_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java8/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 52);
    validate_constant_pool(&class_file);
    validate_access_flags(&class_file);
}
#[test]
fn reads_java11_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java11/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 55);
    validate_constant_pool(&class_file);
    validate_access_flags(&class_file);
}

#[test]
fn reads_java17_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java17/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 61);
    validate_constant_pool(&class_file);
    validate_access_flags(&class_file);
}

#[test]
fn reads_java21_empty_class() {
    setup_logging();
    let class_file = reads_empty_class("res/java21/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 65);
    validate_constant_pool(&class_file);
    validate_access_flags(&class_file);
}

fn validate_access_flags(class_file: &ClassFile) {
    let is_public = ACCESS_FLAGS.contains(AccessFlags::ACC_PUBLIC);
    assert_eq!(
        ACCESS_FLAGS.contains(AccessFlags::ACC_PUBLIC),
        class_file.access_flags.contains(AccessFlags::ACC_PUBLIC),
        "Expect class to be {}",
        if is_public { "public" } else { "non-public" }
    );
}

fn validate_constant_pool(class_file: &ClassFile) {
    let pool = &class_file.constant_pool;

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

                let name_type_ref_idx = pool
                    .name_type_entry(method_ref.name_type_ref as usize - 1)
                    .unwrap();
                assert_string_method_name(&pool, name_type_ref_idx.name_ref);
                assert_type_descriptor(&pool, name_type_ref_idx.type_descriptor_ref);
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

fn reads_empty_class(path: &str) -> ClassFile {
    let file = File::open(path).expect("Can't open class file");
    let mut reader = BufReader::new(file);
    let mut data: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut data)
        .expect("Can't read class file into memory");

    read_class_data(&data).expect("Can't parse class file")
}
