use bit_vec::BitVec;

fn to_hex(val: &str, len: usize) -> String {
    let n: u32 = u32::from_str_radix(val, 2).unwrap();
    format!("0x{:01$x}", n, len)
}

pub struct Bit {
    pub pos: usize,
    pub state: bool,
}

pub fn generate(stages: Vec<Bit>) -> String {
    let mut stages_bitset = BitVec::from_elem(32, false);

    for entry in stages {
        stages_bitset.set(entry.pos, entry.state);
    }

    return to_hex(&stages_bitset.to_string(), 8);
}
