use bit_vec::BitVec;

const STAGE_CODE_TEMPLATE: &str = "C22668BC 00000009 //Stages
88EDAFA0 2C07000{1}
41820008 4082001C
3E208045 6231C370
3E00{2} 6210{3}
92110018 4800001C
3E208045 6231C370
3E000700 621000B0
92110018 48000004
60000000 00000000";

fn to_hex(val: &str, len: usize) -> String {
    let n: u32 = u32::from_str_radix(val, 2).unwrap();
    format!("{:01$x}", n, len)
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

    let mut stages1 = to_hex(&stages_bitset.to_string(), 8);
    let stages2 = stages1.split_off(4);

    return STAGE_CODE_TEMPLATE
        .replace("{1}", "2")
        .replace("{2}", &stages1)
        .replace("{3}", &stages2);
}
