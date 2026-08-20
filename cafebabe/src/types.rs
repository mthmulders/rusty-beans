use crate::constant_pool::ConstantPool;
use crate::constant_pool::types::Version;
use bitflags::bitflags;

bitflags! {
    /// Denote access permissions to and properties of this class or interface.
    pub struct AccessFlags: u16 {
        // Declared public; may be accessed from outside its package.
        const ACC_PUBLIC = 0x0001;
        // Declared final; no subclasses allowed.
        const ACC_FINAL = 0x0010;
        // Treat superclass methods specially when invoked by the `invokespecial` instruction.
        const ACC_SUPER = 0x0020;
        // Is an interface, not a class.
        const ACC_INTERFACE = 0x0200;
        // Declared abstract; must not be instantiated.
        const ACC_ABSTRACT = 0x0400;
        // Declared synthetic; not present in the source code.
        const ACC_SYNTHETIC = 0x1000;
        // Declared as an annotation type.
        const ACC_ANNOTATION = 0x2000;
        // Declared as an enum type.
        const ACC_ENUM = 0x4000;
    }
}

pub struct ClassDefinition {
    pub this_idx: usize,
    pub super_idx: usize,
    pub interface_names: Vec<String>,
}

fn interface_names(interfaces: Vec<usize>, constant_pool: &ConstantPool) -> Vec<String> {
    interfaces
        .iter()
        .map(|interface_name_idx| constant_pool.string_entry(*interface_name_idx as u16))
        .map(|result| result.expect("Failed to retrieve interface name from constant pool"))
        .cloned()
        .collect()
}

impl ClassDefinition {
    pub fn new(
        this_idx: usize,
        super_idx: usize,
        interfaces: Vec<usize>,
        constant_pool: &ConstantPool,
    ) -> ClassDefinition {
        ClassDefinition {
            this_idx,
            super_idx,
            interface_names: interface_names(interfaces, constant_pool),
        }
    }
}

pub struct ClassFile {
    pub version: Version,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub class: ClassDefinition,
}
