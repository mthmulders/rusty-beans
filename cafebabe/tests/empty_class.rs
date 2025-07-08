use cafebabe::constant_pool::{ConstantPool, ConstantPoolEntry};
use cafebabe::{AccessFlags, ClassFile};
use common::read_class_from_path;
use common::setup_logging;
use common::validate_class_name;

mod common;

static ACCESS_FLAGS: AccessFlags = AccessFlags::ACC_PUBLIC;

#[test]
fn reads_java8_empty_class() {
    setup_logging();
    let class_file = read_class_from_path("res/java8/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 52);
    validate_constant_pool(&class_file);
    validate_access_flags(&class_file);
    validate_class_name(&class_file, "examples/EmptyClass");
    validate_super_class_name(&class_file);
    validate_no_interfaces(&class_file);
}

#[test]
fn reads_java11_empty_class() {
    setup_logging();
    let class_file = read_class_from_path("res/java11/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 55);
    validate_constant_pool(&class_file);
    validate_access_flags(&class_file);
    validate_class_name(&class_file, "examples/EmptyClass");
    validate_super_class_name(&class_file);
    validate_no_interfaces(&class_file);
}

#[test]
fn reads_java17_empty_class() {
    setup_logging();
    let class_file = read_class_from_path("res/java17/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 61);
    validate_constant_pool(&class_file);
    validate_access_flags(&class_file);
    validate_class_name(&class_file, "examples/EmptyClass");
    validate_super_class_name(&class_file);
    validate_no_interfaces(&class_file);
}

#[test]
fn reads_java21_empty_class() {
    setup_logging();
    let class_file = read_class_from_path("res/java21/examples/EmptyClass.class");
    assert_eq!(class_file.version.major, 65);
    validate_constant_pool(&class_file);
    validate_access_flags(&class_file);
    validate_class_name(&class_file, "examples/EmptyClass");
    validate_super_class_name(&class_file);
    validate_no_interfaces(&class_file);
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
                let class_idx = pool.class_ref_entry(method_ref.class_ref as usize).unwrap();
                assert_string_class_name(&pool, class_idx);

                let name_type_ref_idx = pool
                    .name_type_entry(method_ref.name_type_ref as usize)
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

fn validate_access_flags(class_file: &ClassFile) {
    let is_public = ACCESS_FLAGS.contains(AccessFlags::ACC_PUBLIC);
    assert_eq!(
        ACCESS_FLAGS.contains(AccessFlags::ACC_PUBLIC),
        class_file.access_flags.contains(AccessFlags::ACC_PUBLIC),
        "Expect class to be {}",
        if is_public { "public" } else { "non-public" }
    );
}

fn validate_super_class_name(class_file: &ClassFile) {
    let class_ref_idx = class_file.class.super_idx;
    let class_name_idx = class_file
        .constant_pool
        .class_ref_entry(class_ref_idx)
        .unwrap();
    let class_name = class_file
        .constant_pool
        .string_entry(class_name_idx)
        .unwrap();
    assert_eq!(
        class_name, "java/lang/Object",
        "Expect super class to be java/lang/Object"
    );
}

fn validate_no_interfaces(class_file: &ClassFile) {
    let interfaces = &class_file.class.interfaces;
    assert_eq!(interfaces.len(), 0);
}

fn assert_type_descriptor(pool: &ConstantPool, idx: u16) -> () {
    let type_descriptor = pool.string_entry(idx).unwrap();
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
    let method_name = pool.string_entry(idx).unwrap();
    let is_constructor = "<init>".eq(method_name);

    assert_eq!(
        is_constructor, true,
        "Method name ref points to string in unexpected format"
    );
}

fn assert_string_class_name(pool: &ConstantPool, idx: u16) -> () {
    let class_name = pool.string_entry(idx).unwrap();
    assert_eq!(
        class_name.contains('/'),
        true,
        "Class ref points to string in unexpected format"
    );
}
