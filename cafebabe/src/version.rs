use std::result::Result;

use crate::constant_pool::types::Version;
use crate::errors::ClassFileError;

pub fn read_version(data: &[u8]) -> Result<Version, ClassFileError> {
    let minor = u16::from_be_bytes([data[4], data[5]]);
    let major = u16::from_be_bytes([data[6], data[7]]);

    match major {
        0..0x2d => {
            dbg!(major);
            Err(ClassFileError::MajorVersionTooLow)
        }
        0x2d..=0x37 => Ok(Version { major, minor }),
        0x38..0x43 => match minor {
            0 | 65535 => Ok(Version { major, minor }),
            _ => {
                dbg!(minor);
                Err(ClassFileError::InvalidMinorVersion)
            }
        },
        _ => {
            dbg!(major);
            Err(ClassFileError::MajorVersionTooHigh)
        }
    }
}
