use crate::app::melee;
use bit_vec::BitVec;

const STOCKS_CODE: &str = "C216E91C 0000000F
3CE08048 80E79D30
54E7443E 2C070208
4082005C 886DAFA0
2C03000{1} 40820050
3860{2} 3C808045
6084310E 98640000
3C808045 60843F9E
98640000 3C808045
60844E2E 98640000
3C808045 60845CBE
98640000 3E808048
62940530 3E403001
6252864C 92540000
48000004 80010024
60000000 00000000";

const TIME_LIMIT_CODE: &str = "C216E750 00000007
3CE08048 80E79D30
54E7443E 2C070208
40820020 886DAFA0
2C03000{1} 40820014
3A40{2} 3E808048
62940540 92540000
3C808017 00000000";

const NO_TIME_LIMIT_CODE: &str = "C216E750 00000008
3CE08048 80E79D30
54E7443E 2C070208
40820024 886DAFA0
2C03000{1} 40820018
3E808048 62940530
3E403001 6252864C
92540000 3C808017
60000000 00000000";

const STAGES_CODE: &str = "C22668BC 00000009
88EDAFA0 2C07000{1}
41820008 4082001C
3E208045 6231C370
3E00{2} 6210{3}
92110018 4800001C
3E208045 6231C370
3E000700 621000B0
92110018 48000004
60000000 00000000";

const ITEMS_CODE: &str = "C216E774 0000000B
3CE08048 80E79D30
54E7443E 2C070208
4082003C 88EDAFA0
2C07000{1} 40820030
3860000{2} 987F000B
7C8802A6 80640000
907F0020 80640004
907F0024 48000004
4E800021 {3}
{4} C022A8C8
60000000 00000000";

pub struct Metadata {
    pub name: String,
    pub author: String,
    pub description: Option<String>,
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

#[derive(PartialEq)]
pub struct Bit {
    pub pos: usize,
    pub state: bool,
}

fn to_hex(val: &str, len: usize) -> String {
    let n: u32 = u32::from_str_radix(val, 2).unwrap();
    format!("{:01$X}", n, len)
}

fn default_stages() -> Vec<Bit> {
    return melee::default_stages()
        .iter()
        .enumerate()
        .map(|(index, entry)| Bit {
            pos: melee::default_stages()[index].bit,
            state: entry.checkbox.checked,
        })
        .collect();
}

fn build_metadata_code(metadata: &Metadata) -> String {
    const METADATA_CODE: &str = "${1} [{2}]";

    let mut code = METADATA_CODE
        .replace("{1}", &metadata.name)
        .replace("{2}", &metadata.author);

    if let Some(description) = &metadata.description {
        code.push_str("\n");
        code.push_str("*");
        code.push_str(&description);
    }

    return code;
}

fn build_stocks_code(stocks: u8, game_mode: &GameMode) -> Option<String> {
    if stocks == melee::default_stocks() {
        return None;
    }

    let game_mode_val = game_mode.val().to_string();
    return Some(
        STOCKS_CODE
            .replace("{1}", &game_mode_val)
            .replace("{2}", &format!("{:01$X}", stocks, 4)),
    );
}

fn build_time_limit_code(time_limit: Option<u8>, game_mode: &GameMode) -> Option<String> {
    if time_limit.unwrap_or(0) == melee::default_time() {
        return None;
    }

    let game_mode_val = game_mode.val().to_string();
    return Some(match time_limit {
        Some(limit) => TIME_LIMIT_CODE
            .replace("{1}", &game_mode_val)
            .replace("{2}", &format!("{:01$X}", limit as u16 * 60, 4)),
        None => NO_TIME_LIMIT_CODE.replace("{1}", &game_mode_val),
    });
}

fn build_stages_code(stages: &Vec<Bit>, game_mode: &GameMode) -> Option<String> {
    if stages == &default_stages() {
        return None;
    }

    let game_mode_val = game_mode.val().to_string();
    let mut stages_bitset = BitVec::from_elem(32, false);
    for entry in stages {
        stages_bitset.set(entry.pos, entry.state);
    }

    let mut stages1 = to_hex(&stages_bitset.to_string(), 8);
    let stages2 = stages1.split_off(4);

    return Some(
        STAGES_CODE
            .replace("{1}", &game_mode_val)
            .replace("{2}", &stages1)
            .replace("{3}", &stages2),
    );
}

fn build_items_code(
    item_frequency: &ItemFrequency,
    items: &Vec<Bit>,
    game_mode: &GameMode,
) -> Option<String> {
    if item_frequency.val().is_err() {
        return None;
    }

    let game_mode_val = game_mode.val().to_string();
    let item_frequency_val = item_frequency.val().unwrap();

    let mut items_bitset1 = BitVec::from_elem(32, true);
    let mut items_bitset2 = BitVec::from_elem(32, true);
    for entry in items {
        if entry.pos < 32 {
            items_bitset1.set(entry.pos, entry.state);
        } else {
            items_bitset2.set(entry.pos - 32, entry.state);
        }
    }

    return Some(
        ITEMS_CODE
            .replace("{1}", &game_mode_val)
            .replace("{2}", &item_frequency_val.to_string())
            .replace("{3}", &to_hex(&items_bitset1.to_string(), 8))
            .replace("{4}", &to_hex(&items_bitset2.to_string(), 8)),
    );
}

pub fn can_be_generated(
    stocks: u8,
    time_limit: Option<u8>,
    stages: Vec<Bit>,
    item_frequency: ItemFrequency,
) -> bool {
    let stocks_default = stocks == melee::default_stocks();
    let time_default = time_limit.unwrap_or(0) == melee::default_time();
    let stages_default = stages == default_stages();
    let items_default = item_frequency.val().is_err();

    return !stocks_default || !time_default || !stages_default || !items_default;
}

pub fn generate(
    metadata: Metadata,
    game_mode: GameMode,
    stocks: u8,
    time_limit: Option<u8>,
    stages: Vec<Bit>,
    item_frequency: ItemFrequency,
    items: Vec<Bit>,
) -> String {
    let mut code = build_metadata_code(&metadata);
    code.push_str("\n");
    code.push_str("#Generated with ssbm-custom-game-mode-generator | by AlexMD");
    if let Some(stocks_code) = build_stocks_code(stocks, &game_mode) {
        code += "\n";
        code += &stocks_code;
    }
    if let Some(time_limit_code) = build_time_limit_code(time_limit, &game_mode) {
        code += "\n";
        code += &time_limit_code;
    }
    if let Some(stages_code) = build_stages_code(&stages, &game_mode) {
        code += "\n";
        code += &stages_code;
    }
    if let Some(items_code) = build_items_code(&item_frequency, &items, &game_mode) {
        code += "\n";
        code += &items_code;
    }

    return code;
}
