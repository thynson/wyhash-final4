#[inline(always)]
pub(crate) fn wy_rotate(x: u64) -> u64 {
    x.rotate_left(32)
}

///
/// Read first 8 bytes of a given slice as a u64 in little-endian order.
///
/// SAFETY: the caller must ensure that the byte slice is at least 8 bytes long,
/// otherwise it will cause undefined behavior.
///
#[inline(always)]
pub(crate) unsafe fn wy_read_8(input: &[u8]) -> u64 {
    unsafe { u64::from_le_bytes(*(input.as_ptr() as *const [u8; 8])) }
}

/// Read first two bytes of a given slice as a u64 in little-endian order.
///
/// SAFETY: the caller must ensure that the byte slice is at least 2 bytes long,
/// otherwise it will cause undefined behavior.
#[inline(always)]
unsafe fn wy_read_2(input: &[u8]) -> u64 {
    unsafe { u16::from_le_bytes(*(input.as_ptr() as *const [u8; 2])) as u64 }
}

///
/// Read a unsigned 64-bit integer from the tail of a byte slice in little-endian order.
///
/// Note that the caller must ensure that the byte slice is shorter or equals to 8 bytes,
/// otherwise it will panic due to unreachable assertion failure.
///
#[inline(always)]
pub(crate) fn wy_read_tail8(input: &[u8]) -> u64 {
    unsafe {
        match input.len() {
            0 => 0,
            1 => input[0] as u64,
            2 => wy_read_2(input),
            3 => wy_read_2(input) | ((input[2] as u64) << 16),
            4 => wy_read_4(input),
            5 => wy_read_4(input) | ((input[4] as u64) << 32),
            6 => wy_read_4(input) | (wy_read_2(&input[4..]) << 32),
            7 => wy_read_4(input) | (wy_read_2(&input[4..]) << 32) | ((input[6] as u64) << 48),
            8 => wy_read_8(input),
            _ => unreachable!(),
        }
    }
}

/// Read first four bytes of a given slice as a u64 in little-endian order.
///
/// SAFETY: the caller must ensure that the byte slice is at least 4 bytes long,
/// otherwise it will cause undefined behavior.
#[inline(always)]
pub(crate) unsafe fn wy_read_4(input: &[u8]) -> u64 {
    unsafe { u32::from_le_bytes(*(input.as_ptr() as *const [u8; 4])) as u64 }
}

///
/// Read a unsigned 64-bit integer from the tail of a byte slice in little-endian order.
/// This function is supposed to be used when handling the tail of a byte slice that the length
/// of it is greater than 0 and less than 4.
///
#[inline(always)]
pub(crate) unsafe fn wy_read_tail3(input: &[u8]) -> u64 {
    unsafe {
        let len = input.len();
        let input = input.as_ptr();
        (*input as u64) << 16 | (*input.add(len >> 1) as u64) << 8 | *input.add(len - 1) as u64
    }
}

#[inline(always)]
#[cold]
fn cold_path() {}

#[inline(always)]
pub(crate) fn likely(b: bool) -> bool {
    if !b {
        cold_path();
    }
    b
}

#[inline(always)]
pub(crate) fn unlikely(b: bool) -> bool {
    if b {
        cold_path();
    }
    b
}
