#![feature(const_raw_ptr_deref)]

pub const fn four_cc(cc: &[u8; 4]) -> u32 {
    let v = unsafe { *(cc.as_ptr() as *const u32) };
    #[cfg(target_endian="little")] {
        v.to_be()
    }
    #[cfg(target_endian="big")] {
        v
    }
}

#[cfg(test)]
mod tests {
    use crate::four_cc;
    // from https://gist.github.com/rust-play/730673365471818b5bbbc876e65ffe40
    #[test]
    fn test_a_thing() {
        assert_eq!(0x68656963, four_cc(b"heic"));
        assert_eq!(1918990199, four_cc(b"raww"));
    }
}