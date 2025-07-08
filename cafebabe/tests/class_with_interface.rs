use cafebabe::ClassFile;
use common::read_class_from_path;
use common::setup_logging;

mod common;

#[test]
fn reads_java8_class_with_interface() {
    setup_logging();
    let class_file = read_class_from_path("res/java8/examples/ClassWithInterface.class");
    assert_eq!(class_file.version.major, 52);
    validate_interfaces(&class_file);
}

#[test]
fn reads_java11_class_with_interface() {
    setup_logging();
    let class_file = read_class_from_path("res/java11/examples/ClassWithInterface.class");
    assert_eq!(class_file.version.major, 55);
    validate_interfaces(&class_file);
}

#[test]
fn reads_java17_class_with_interface() {
    setup_logging();
    let class_file = read_class_from_path("res/java17/examples/ClassWithInterface.class");
    assert_eq!(class_file.version.major, 61);
    validate_interfaces(&class_file);
}

#[test]
fn reads_java21_class_with_interface() {
    setup_logging();
    let class_file = read_class_from_path("res/java21/examples/ClassWithInterface.class");
    assert_eq!(class_file.version.major, 65);
    validate_interfaces(&class_file);
}

fn validate_interfaces(class_file: &ClassFile) {
    let interfaces = &class_file.class.interfaces;
    assert_eq!(interfaces.len(), 1);
    let interface_name_idx = interfaces[0] as u16;
    let interface_name = class_file.constant_pool.string_entry(interface_name_idx);
    assert_eq!(interface_name.unwrap(), "java/io/Serializable");
}
