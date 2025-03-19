#[derive(Debug)]
pub enum ClassFileError {
    Unknown,

    InvalidMagicNumber,
    MajorVersionTooLow,
    InvalidMinorVersion,
    MajorVersionTooHigh,
    UnknownConstantPoolEntryTag,
    InvalidConstantPoolContent,
}
