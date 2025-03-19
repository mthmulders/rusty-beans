pub struct ClassFile {
    pub version: Version,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub class: ClassDefinition,
}

#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}

pub struct ConstantPool {
    pub items: Vec<ConstantPoolEntry>,
}

pub struct ConstantPoolEntry {}

pub struct AccessFlags {}

pub struct ClassDefinition {
    pub name_idx: usize,
    pub super_idx: usize,
}
