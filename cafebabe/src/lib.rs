pub mod constant_pool;
mod errors;
mod shared;
mod types;
mod version;

use std::result::Result;

use crate::shared::to_u16;
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

fn read_access_flags(
    data: &[u8],
    start_idx: usize,
) -> Result<(AccessFlags, usize), ClassFileError> {
    let access_flags = to_u16(data, start_idx, start_idx + 1);
    match AccessFlags::from_bits(access_flags) {
        Some(access_flags) => Ok((access_flags, start_idx + 2)),
        None => Err(ClassFileError::InvalidAccessFlags),
    }
}

fn read_class_definition(
    data: &[u8],
    start_idx: usize,
) -> Result<(ClassDefinition, usize), ClassFileError> {
    let class_definition = ClassDefinition {
        this_idx: u16::from_be_bytes([data[start_idx], data[start_idx + 1]]) as usize,
        super_idx: u16::from_be_bytes([data[start_idx + 2], data[start_idx + 3]]) as usize,
    };
    Ok((class_definition, start_idx + 4))
}

pub fn read_class_data(data: &[u8]) -> Result<ClassFile, ClassFileError> {
    read_magic_number(data)?;
    let constant_pool = constant_pool::read_constant_pool(data)?;
    let access_flags = read_access_flags(data, constant_pool.1)?;
    let class_definition = read_class_definition(data, access_flags.1)?;

    Ok(ClassFile {
        version: version::read_version(data)?,
        constant_pool: constant_pool.0,
        access_flags: access_flags.0,
        class: class_definition.0,
    })
}
