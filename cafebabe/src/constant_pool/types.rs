#[derive(Debug, PartialEq)]
pub enum ConstantPoolEntry {
    /// An entry holding a `String`.
    String(String),
    /// An entry holding an `int`.
    Integer(),
    /// An entry holding a `float`.
    Float(),
    /// An entry holding a `long`.
    Long(),
    /// An entry holding a `double`.
    Double(),
    /// An entry holding a reference to a class.
    Class(),
    /// An entry holding a reference to a [`String`] entry.
    StringRef(),
    /// An entry holding a reference to a field in a class.
    FieldRef(),
    /// An entry holding a reference to a method in a class.
    MethodRef(MethodRef),
    /// An entry holding a reference to a method in an interface.
    InterfaceMethodRef(),
    /// An entry describing a name and a type.
    NameTypeDescriptor(),
    /// An entry holding a method handle.
    MethodHandle(),
    /// An entry holding a type description of a method.
    MethodType(),
    /// An entry holding a dynamically computed constant (produced by invocation of a bootstrap method).
    Dynamic(),
    /// An entry holding a bootstrap method that can be used by the `invokedynamic` instruction.
    InvokeDynamic(),
    /// An entry holding a JPMS module identification.
    Module(),
    /// An entry holding a package identification.
    Package(),

    /// Placeholder for empty or unimplemented constant pool entries.
    Empty(),
}

#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}

#[derive(Debug, PartialEq)]
pub struct MethodRef {
    class_ref: usize,
    name_and_type_desc: usize,
}
