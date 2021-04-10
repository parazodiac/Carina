use std::error::Error;

pub fn cb_string_to_u64(cb_str: &[u8]) -> Result<u64, Box<dyn Error>> {
    let mut cb_id: u64 = 0;
    for (idx, nt) in cb_str.iter().rev().enumerate() {
        let offset = idx * 2;
        match nt {
            65 | 78 => (),              // A | N 00
            67 => cb_id |= 1 << offset, // C 01
            71 => cb_id |= 2 << offset, // G 10
            84 => cb_id |= 3 << offset, // T 11
            _ => panic!("unknown nucleotide {}", nt),
        };
    }

    Ok(cb_id)
}

pub fn u64_to_cb_string(cb_id: u64, cb_length: usize) -> Result<String, Box<dyn Error>> {
    let mut cb_str = String::new();
    for i in 0..cb_length {
        let offset = (cb_length - i - 1) * 2;
        let nt = (cb_id & (3 << offset)) >> offset;

        match nt {
            0 => cb_str += "A",
            1 => cb_str += "C",
            2 => cb_str += "G",
            3 => cb_str += "T",
            _ => unreachable!(),
        }
    }

    Ok(cb_str)
}

pub fn cb_string_to_u64_with_id(
    cb_str: &[u8],
    cb_length: usize,
    id: u8,
) -> Result<u64, Box<dyn Error>> {
    assert_eq!(cb_str.len(), cb_length);

    let mut cb_id: u64 = 0;
    for (idx, nt) in cb_str.iter().rev().enumerate() {
        let offset = idx * 2;
        match nt {
            65 | 78 => (),              // A | N 00
            67 => cb_id |= 1 << offset, // C 01
            71 => cb_id |= 2 << offset, // G 10
            84 => cb_id |= 3 << offset, // T 11
            _ => panic!("unknown nucleotide {}", nt),
        };
    }

    let id_offset = 62;
    match id {
        1 => cb_id |= 1 << id_offset,
        2 => cb_id |= 2 << id_offset,
        _ => unreachable!(),
    }

    Ok(cb_id)
}

pub fn u64_to_cb_string_with_id(cb_id: u64, cb_length: usize) -> Result<String, Box<dyn Error>> {
    let mut cb_str = String::new();
    for i in 0..cb_length {
        let offset = (cb_length - i - 1) * 2;
        let nt = (cb_id & (3 << offset)) >> offset;

        match nt {
            0 => cb_str += "A",
            1 => cb_str += "C",
            2 => cb_str += "G",
            3 => cb_str += "T",
            _ => unreachable!(),
        }
    }

    let id_offset = 62;
    let id_int = (cb_id & (3 << id_offset)) >> id_offset;
    match id_int {
        1 => cb_str += "-1",
        2 => cb_str += "-2",
        _ => unreachable!(),
    }

    Ok(cb_str)
}

#[cfg(test)]
mod tests {
    use crate::barcode::*;

    #[test]
    fn test_cb_string_to_u64() {
        let cb_id = cb_string_to_u64("A".repeat(16).as_bytes()).unwrap();
        assert_eq!(cb_id, 0);

        let cb_id = cb_string_to_u64("T".repeat(16).as_bytes()).unwrap();
        assert_eq!(cb_id, u32::MAX as u64);
    }

    #[test]
    fn test_u64_to_cb_string() {
        let cb_str = u64_to_cb_string(0, 16).unwrap();
        assert_eq!("A".repeat(16), cb_str);

        let cb_str = u64_to_cb_string(u32::MAX as u64, 16).unwrap();
        assert_eq!("T".repeat(16), cb_str);
    }

    #[test]
    fn test_cb_string_to_u64_with_id() {
        let cb_id = cb_string_to_u64_with_id("A".repeat(16).as_bytes(), 16, 2).unwrap();
        assert_eq!(cb_id, 9223372036854775808);

        let cb_id = cb_string_to_u64_with_id("T".repeat(16).as_bytes(), 16, 1).unwrap();
        assert_eq!(cb_id, 4611686022722355199);
    }

    #[test]
    fn test_u64_to_cb_string_with_id() {
        let cb_str = u64_to_cb_string_with_id(9223372036854775808, 16).unwrap();
        assert_eq!("A".repeat(16) + "-2", cb_str);

        let cb_str = u64_to_cb_string_with_id(4611686022722355199, 16).unwrap();
        assert_eq!("T".repeat(16) + "-1", cb_str);
    }
}
