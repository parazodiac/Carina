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

pub fn u64_to_cb_string(cb_id: u64) -> Result<String, Box<dyn Error>> {
    let mut cb_str = String::new();
    for i in 0..crate::CB_LENGTH {
        let offset = (crate::CB_LENGTH - i - 1) * 2;
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