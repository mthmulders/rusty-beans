use std::result::Result;
use std::slice::Iter;
use std::str;

pub use crate::constant_pool::types::ConstantPoolEntry;
use crate::errors::ClassFileError;

use crate::constant_pool::types::{MethodRef, NameTypeDescriptor};
use crate::shared::to_u16;
use log::{debug, error};

pub mod types;

const TAG_STRING: usize = 1;
// const TAG_INTEGER: usize = 3;
// const TAG_FLOAT: usize = 4;
// const TAG_LONG: usize = 5;
// const TAG_DOUBLE: usize = 6;
const TAG_CLASS_REF: usize = 7;
// const TAG_STRING_REF: usize = 8;
// const TAG_FIELD_REF: usize = 9;
const TAG_METHOD_REF: usize = 10;
// const TAG_INTERFACE_METHOD_REF: usize = 11;
const TAG_NAME_TYPE_DESCRIPTOR: usize = 12;
// const TAG_METHOD_HANDLE: usize = 15;
// const TAG_METHOD_TYPE: usize = 16;
// const TAG_DYNAMIC: usize = 17;
// const TAG_INVOKE_DYNAMIC: usize = 18;
// const TAG_MODULE: usize = 19;
// const TAG_PACKAGE: usize = 20;

pub struct ConstantPool {
    items: Vec<ConstantPoolEntry>,
}

impl ConstantPool {
    pub fn get_entry(&self, index: u16) -> &ConstantPoolEntry {
        &self.items[(index - 1) as usize]
    }

    pub fn items(&self) -> Iter<ConstantPoolEntry> {
        self.items.iter()
    }

    pub fn len(&self) -> u16 {
        self.items.len() as u16
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn string_entry(&self, index: u16) -> Result<&String, ClassFileError> {
        match self.get_entry(index) {
            ConstantPoolEntry::String(value) => Ok(value),
            other => {
                error!("Expected String at index {:?}, found {:?}", index, other);
                Err(ClassFileError::UnexpectedConstantPoolType)
            }
        }
    }

    pub fn class_ref_entry(&self, index: usize) -> Result<u16, ClassFileError> {
        match self.get_entry(index as u16) {
            ConstantPoolEntry::ClassRef(value) => Ok(*value),
            other => {
                error!("Expected Class at index {:?}, found {:?}", index, other);
                Err(ClassFileError::UnexpectedConstantPoolType)
            }
        }
    }

    pub fn name_type_entry(&self, index: usize) -> Result<NameTypeDescriptor, ClassFileError> {
        match self.get_entry(index as u16) {
            ConstantPoolEntry::NameTypeDescriptor(value) => Ok(*value),
            other => {
                error!(
                    "Expected NameTypeDescriptor at index {:?}, found {:?}",
                    index, other
                );
                Err(ClassFileError::UnexpectedConstantPoolType)
            }
        }
    }
}

fn read_constant_pool_entry_class_ref(
    data: &[u8],
    from_idx: usize,
) -> Result<(ConstantPoolEntry, usize), ClassFileError> {
    let class_ref = to_u16(data, from_idx, from_idx + 1);
    debug!("found class ref; class_ref={class_ref}");

    Ok((ConstantPoolEntry::ClassRef(class_ref), from_idx + 2))
}

fn read_constant_pool_entry_name_type_descriptor(
    data: &[u8],
    from_idx: usize,
) -> Result<(ConstantPoolEntry, usize), ClassFileError> {
    let name_ref = to_u16(data, from_idx, from_idx + 1);
    let type_descriptor_ref = to_u16(data, from_idx + 2, from_idx + 3);
    debug!(
        "found name and type descriptor; class_name_ref={name_ref}, type_descriptor_ref={type_descriptor_ref}"
    );

    let descriptor = NameTypeDescriptor {
        name_ref,
        type_descriptor_ref,
    };
    Ok((
        ConstantPoolEntry::NameTypeDescriptor(descriptor),
        from_idx + 4,
    ))
}

fn read_constant_pool_entry_method_ref(
    data: &[u8],
    from_idx: usize,
) -> Result<(ConstantPoolEntry, usize), ClassFileError> {
    let class_ref = to_u16(data, from_idx, from_idx + 1);
    let name_type_ref = to_u16(data, from_idx + 2, from_idx + 3);
    debug!("found method ref; class_ref={class_ref}, name_type_ref={name_type_ref}");

    let method_ref = MethodRef {
        class_ref,
        name_type_ref,
    };
    Ok((ConstantPoolEntry::MethodRef(method_ref), from_idx + 4))
}

// fn read_constant_pool_entry_float(
//     data: &[u8],
//     from_idx: usize,
// ) -> Result<(ConstantPoolEntry, usize), ClassFileError> {
//     // 4 bytes with a 32-bit single-precision IEEE 754 floating-point number
//     let bytes: [u8; 4] = data[from_idx..(from_idx + 4)]
//         .try_into()
//         .expect("incorrect length of slice");
//     let value = f32::from_be_bytes(bytes);
//     debug!("found float; value={value}");
//
//     Ok((ConstantPoolEntry::Empty {}, from_idx + 4))
// }

fn read_constant_pool_entry_string(
    data: &[u8],
    from_idx: usize,
) -> Result<(ConstantPoolEntry, usize), ClassFileError> {
    let size = to_u16(data, from_idx, from_idx + 1);
    let content = &data[from_idx + 2..from_idx + 2 + usize::from(size)];
    match str::from_utf8(content) {
        Ok(value) => {
            debug!("found string; value={value}");
            Ok((
                ConstantPoolEntry::String(value.to_string()),
                from_idx + 2 + usize::from(size),
            ))
        }
        Err(error) => {
            dbg!(error);
            Err(ClassFileError::InvalidConstantPoolContent)
        }
    }
}

fn read_constant_pool_entry(
    data: &[u8],
    from_idx: usize,
) -> Result<(ConstantPoolEntry, usize), ClassFileError> {
    let tag = usize::from(data[from_idx]);
    // debug!("next constant pool entry; tag={tag}, from_idx={from_idx}");
    match tag {
        TAG_STRING => read_constant_pool_entry_string(data, from_idx + 1),
        // TAG_INTEGER => Ok((ConstantPoolEntry {}, from_idx + 4)),
        // TAG_FLOAT => read_constant_pool_entry_float(data, from_idx + 1),
        // TAG_LONG => Ok((ConstantPoolEntry {}, from_idx + 8)),
        // TAG_DOUBLE => Ok((ConstantPoolEntry {}, from_idx + 8)),
        TAG_CLASS_REF => read_constant_pool_entry_class_ref(data, from_idx + 1),
        // TAG_STRING_REF => Ok((ConstantPoolEntry {}, from_idx + 2)),
        // TAG_FIELD_REF => Ok((ConstantPoolEntry {}, from_idx + 4)),
        TAG_METHOD_REF => read_constant_pool_entry_method_ref(data, from_idx + 1),
        TAG_NAME_TYPE_DESCRIPTOR => {
            read_constant_pool_entry_name_type_descriptor(data, from_idx + 1)
        }
        _ => {
            debug!("unknown constant pool entry; tag={tag}");
            Err(ClassFileError::UnknownConstantPoolEntryTag)
        }
    }
}

pub fn read_constant_pool(data: &[u8]) -> Result<(ConstantPool, usize), ClassFileError> {
    let pool_size = u16::from_be_bytes([data[8], data[9]]);
    debug!("start reading constant pool; expected_size={pool_size}");

    let mut items: Vec<ConstantPoolEntry> = Vec::with_capacity(usize::from(pool_size - 1));
    let mut from_idx = 10;

    for _ in 1..pool_size {
        // nah, nasty one: reading a variable number of bytes here
        // what is going to be the starting point for the next item?!
        let item = read_constant_pool_entry(data, from_idx)?;
        items.push(item.0);
        from_idx = item.1;
    }

    Ok((ConstantPool { items }, from_idx))
}
