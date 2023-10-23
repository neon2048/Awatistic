use crate::errors::AwawaError;

const AWASCII: &str = "AWawJELYHOSIUMjelyhosiumPCNTpcntBDFGRbdfgr0123456789 .,!'()~_/;\n";

pub fn awascii(index: i32) -> Result<char, AwawaError> {
    let foo =
        usize::try_from(index).map_err(|_| return AwawaError::InvalidAwasciiCodeError(index))?;

    return AWASCII
        .chars()
        .nth(foo)
        .ok_or(AwawaError::InvalidAwasciiCodeError(index));
}

pub fn ord(c: char) -> Option<i32> {
    match AWASCII.find(c) {
        Some(idx) => Some(idx as i32),
        None => None,
    }
}
