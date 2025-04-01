use crate::constant_pool::ConstantPool;
use crate::constant_pool::types::Version;

pub struct AccessFlags {}

pub struct ClassDefinition {
    pub name_idx: usize,
    pub super_idx: usize,
}

pub struct ClassFile {
    pub version: Version,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub class: ClassDefinition,
}
