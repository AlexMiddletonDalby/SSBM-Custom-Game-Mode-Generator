use crate::app::stages::StageData;
use bit_vec::BitVec;

fn to_hex(val: &str, len: usize) -> String {
    let n: u32 = u32::from_str_radix(val, 2).unwrap();
    format!("0x{:01$x}", n, len)
}

pub fn generate(stages: Vec<StageData>) -> String {
    let mut stages_bitset = BitVec::from_elem(32, false);

    for entry in stages {
        stages_bitset.set(entry.bit, entry.checked);
    }

    return to_hex(&stages_bitset.to_string(), 8);
}
