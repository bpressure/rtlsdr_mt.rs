use std::fmt;

const NO_VALID_EEPROM_HEADER: i32 = -13;
const STRING_VALUE_TOO_LONG: i32 = -14;
const STRING_DESCRIPTOR_INVALID: i32 = -15;
const STRING__DESCRIPTOR_TOO_LONG: i32 = -16;

#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum RtlSdrError {
    Io,
    InvalidParam,
    Access,
    NoDevice,
    NotFound,
    Busy,
    Timeout,
    Overflow,
    Pipe,
    Interrupted,
    NoMem,
    NotSupported,
    NoValidEEPROMHeader,
    StringValueTooLong,
    StringDescriptorInvalid,
    StringDescriptorTooLong,
    NoDeviceFound,
    Unknown,
}

pub fn check_err(e: i32) -> Result<(), RtlSdrError> {
    match e {
        0 => Ok(()),
        _ => Err(by_err_msg(e))
    }
}

pub fn by_err_msg(e: i32) -> RtlSdrError {
    match e {
        -1 => RtlSdrError::Io,
        -2 => RtlSdrError::InvalidParam,
        -3 => RtlSdrError::Access,
        -4 => RtlSdrError::NoDevice,
        -5 => RtlSdrError::NotFound,
        -6 => RtlSdrError::Busy,
        -7 => RtlSdrError::Timeout,
        -8 => RtlSdrError::Overflow,
        -9 => RtlSdrError::Pipe,
        -10 => RtlSdrError::Interrupted,
        -11 => RtlSdrError::NoMem,
        -12 => RtlSdrError::NotSupported,
        NO_VALID_EEPROM_HEADER => RtlSdrError::NoValidEEPROMHeader,
        STRING_VALUE_TOO_LONG => RtlSdrError::StringValueTooLong,
        STRING_DESCRIPTOR_INVALID => RtlSdrError::StringDescriptorInvalid,
        STRING__DESCRIPTOR_TOO_LONG => RtlSdrError::StringDescriptorTooLong,
        _ => RtlSdrError::Unknown,
    }
}

impl std::error::Error for RtlSdrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl fmt::Display for RtlSdrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}