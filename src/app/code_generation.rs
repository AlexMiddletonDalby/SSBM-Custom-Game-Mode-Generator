use bit_vec::BitVec;

const STAGE_CODE_TEMPLATE: &str = "C22668BC 00000009 # Stages
88EDAFA0 2C07000{1}
41820008 4082001C
3E208045 6231C370
3E00{2} 6210{3}
92110018 4800001C
3E208045 6231C370
3E000700 621000B0
92110018 48000004
60000000 00000000";

const ITEMS_CODE_TEMPLATE: &str = "C216E774 0000000B # items
3CE08048 80E79D30
54E7443E 2C070208
4082003C 88EDAFA0
2C07000{1} 40820030
3860000{2} 987F000B
7C8802A6 80640000
907F0020 80640004
907F0024 48000004
4E800021 FFFFFFFF
{3} C022A8C8
60000000 00000000";

fn to_hex(val: &str, len: usize) -> String {
    let n: u32 = u32::from_str_radix(val, 2).unwrap();
    format!("{:01$X}", n, len)
}

pub enum GameMode {
    Direct,
    Doubles,
}

impl GameMode {
    fn val(&self) -> u8 {
        match self {
            GameMode::Direct => 2,
            GameMode::Doubles => 3,
        }
    }
}

pub enum ItemFrequency {
    None,
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

impl ItemFrequency {
    fn val(&self) -> Result<u8, &str> {
        match self {
            ItemFrequency::None => Result::Err("Item code should be ommited entirely for 'None'"),
            ItemFrequency::VeryLow => Result::Ok(0),
            ItemFrequency::Low => Result::Ok(1),
            ItemFrequency::Medium => Result::Ok(2),
            ItemFrequency::High => Result::Ok(3),
            ItemFrequency::VeryHigh => Result::Ok(4),
        }
    }
}

pub struct Bit {
    pub pos: usize,
    pub state: bool,
}

pub fn generate(
    game_mode: GameMode,
    stages: Vec<Bit>,
    item_frequency: ItemFrequency,
    items: Vec<Bit>,
) -> String {
    let game_mode_val = game_mode.val();
    let mut stages_bitset = BitVec::from_elem(32, false);

    for entry in stages {
        stages_bitset.set(entry.pos, entry.state);
    }

    let mut stages1 = to_hex(&stages_bitset.to_string(), 8);
    let stages2 = stages1.split_off(4);

    let stage_code = STAGE_CODE_TEMPLATE
        .replace("{1}", &game_mode_val.to_string())
        .replace("{2}", &stages1)
        .replace("{3}", &stages2);

    let mut items_code = String::new();
    if let Result::Ok(item_frequency_val) = item_frequency.val() {
        let mut items_bitset = BitVec::from_elem(32, true);
        for entry in items {
            items_bitset.set(entry.pos, entry.state);
        }

        items_code = ITEMS_CODE_TEMPLATE
            .replace("{1}", &game_mode_val.to_string())
            .replace("{2}", &item_frequency_val.to_string())
            .replace("{3}", &to_hex(&items_bitset.to_string(), 8));
    }

    return stage_code + "\n" + &items_code;
}
