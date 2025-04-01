pub mod constant_pool;
mod errors;
mod types;
mod version;

use std::result::Result;

use errors::ClassFileError;
pub use types::{AccessFlags, ClassDefinition, ClassFile};

const CAFEBABE: u32 = u32::from_be_bytes([0xca, 0xfe, 0xba, 0xbe]);

fn read_magic_number(data: &[u8]) -> Result<(), ClassFileError> {
    let bytes: [u8; 4] = data[0..4].try_into().expect("slice with incorrect length");
    let magic_number = u32::from_be_bytes(bytes);

    match magic_number {
        CAFEBABE => Ok(()),
        _ => {
            dbg!(data[0..4].to_vec());
            Err(ClassFileError::InvalidMagicNumber)
        }
    }
}

pub fn read_class_data(data: &[u8]) -> Result<ClassFile, ClassFileError> {
    let _ = read_magic_number(data);
    let constant_pool = constant_pool::read_constant_pool(data)?;

    Ok(ClassFile {
        version: version::read_version(data)?,
        constant_pool: constant_pool.0,
        access_flags: AccessFlags {},
        class: ClassDefinition {
            name_idx: 0,
            super_idx: 0,
        },
    })
}
