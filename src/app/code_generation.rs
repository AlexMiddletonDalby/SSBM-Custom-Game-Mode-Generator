use crate::app::melee;
use bit_vec::BitVec;

const RULES_NO_TIME_LIMIT_NO_ITEMS: &str = "C200C160 0000000D
3CE08048 80E79D30
54E7443E 2C070208
4082004C 88EDAFA0
2C07000{MODE} 40820040
80E10104 3D008016
6108E94C 7C074000
4082002C 3D008048
61080530 38E0{STOCKS}
98E80062 98E80086
98E800AA 98E800CE
3CE03001 60E7864C
90E80000 2C040000
60000000 00000000";

const RULES_NO_TIME_LIMIT_ITEMS: &str = "C200C160 00000013
3CE08048 80E79D30
54E7443E 2C070208
4082007C 88EDAFA0
2C07000{MODE} 40820070
80E10104 3D008016
6108E94C 7C074000
4082005C 3D008048
61080530 38E0{STOCKS}
98E80062 98E80086
98E800AA 98E800CE
3CE03001 60E7864C
90E80000 38E0000{ITEM_FREQ}
98E8000B 38E000{ITEM_BF1}
98E80023 38E000{ITEM_BF2}
98E80024 38E000{ITEM_BF3}
98E80025 38E000{ITEM_BF4}
98E80026 38E000{ITEM_BF5}
98E80027 2C040000
60000000 00000000";

const RULES_TIME_LIMIT_NO_ITEMS: &str = "C200C160 0000000C
3CE08048 80E79D30
54E7443E 2C070208
40820048 88EDAFA0
2C07000{MODE} 4082003C
80E10104 3D008016
6108E94C 7C074000
40820028 3D008048
61080530 38E0{STOCKS}
98E80062 98E80086
98E800AA 98E800CE
38E0{TIME} 90E80010
2C040000 00000000";

const RULES_TIME_LIMIT_ITEMS: &str = "C200C160 00000012
3CE08048 80E79D30
54E7443E 2C070208
40820078 88EDAFA0
2C07000{MODE} 4082006C
80E10104 3D008016
6108E94C 7C074000
40820058 3D008048
61080530 38E0{STOCKS}
98E80062 98E80086
98E800AA 98E800CE
38E0{TIME} 90E80010
38E0000{ITEM_FREQ} 98E8000B
38E000{ITEM_BF1} 98E80023
38E000{ITEM_BF2} 98E80024
38E000{ITEM_BF3} 98E80025
38E000{ITEM_BF4} 98E80026
38E000{ITEM_BF5} 98E80027
2C040000 00000000";

const STAGES: &str = "C22668BC 00000009
88EDAFA0 2C07000{MODE}
41820008 4082001C
3E208045 6231C370
3E00{STAGES_BF1} 6210{STAGES_BF2}
92110018 4800001C
3E208045 6231C370
3E000700 621000B0
92110018 48000004
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
    pub field: usize,
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
            field: 0,
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
        STAGES
            .replace("{MODE}", &game_mode_val)
            .replace("{STAGES_BF1}", &stages1)
            .replace("{STAGES_BF2}", &stages2),
    );
}

fn choose_base_code(has_time_limit: bool, has_items: bool) -> String {
    if has_time_limit {
        if has_items {
            return RULES_TIME_LIMIT_ITEMS.to_string();
        }

        return RULES_TIME_LIMIT_NO_ITEMS.to_string();
    }

    if has_items {
        return RULES_NO_TIME_LIMIT_ITEMS.to_string();
    }

    return RULES_NO_TIME_LIMIT_NO_ITEMS.to_string();
}

fn build_rules_code(
    game_mode: &GameMode,
    stocks: u8,
    time_limit: Option<u8>,
    item_frequency: ItemFrequency,
    items: Vec<Bit>,
) -> String {
    let mut rules = choose_base_code(time_limit.is_some(), item_frequency.val().is_ok())
        .replace("{MODE}", &game_mode.val().to_string())
        .replace("{STOCKS}", &format!("{:01$X}", stocks, 4));

    if let Some(limit) = time_limit {
        rules = rules.replace("{TIME}", &format!("{:01$X}", limit as u16 * 60, 4));
    }

    if let Ok(freq) = item_frequency.val() {
        let mut items_bitset1 = BitVec::from_elem(8, true);
        let mut items_bitset2 = BitVec::from_elem(8, true);
        let mut items_bitset3 = BitVec::from_elem(8, true);
        let mut items_bitset4 = BitVec::from_elem(8, true);
        let mut items_bitset5 = BitVec::from_elem(8, true);

        for entry in items.iter().filter(|item| item.field == 1) {
            items_bitset1.set(entry.pos, entry.state);
        }
        for entry in items.iter().filter(|item| item.field == 2) {
            items_bitset2.set(entry.pos, entry.state);
        }
        for entry in items.iter().filter(|item| item.field == 3) {
            items_bitset3.set(entry.pos, entry.state);
        }
        for entry in items.iter().filter(|item| item.field == 4) {
            items_bitset4.set(entry.pos, entry.state);
        }
        for entry in items.iter().filter(|item| item.field == 5) {
            items_bitset5.set(entry.pos, entry.state);
        }

        rules = rules
            .replace("{ITEM_FREQ}", &freq.to_string())
            .replace("{ITEM_BF1}", &to_hex(&items_bitset1.to_string(), 2))
            .replace("{ITEM_BF2}", &to_hex(&items_bitset2.to_string(), 2))
            .replace("{ITEM_BF3}", &to_hex(&items_bitset3.to_string(), 2))
            .replace("{ITEM_BF4}", &to_hex(&items_bitset4.to_string(), 2))
            .replace("{ITEM_BF5}", &to_hex(&items_bitset5.to_string(), 2));
    }

    return rules;
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

    let rules = build_rules_code(&game_mode, stocks, time_limit, item_frequency, items);
    code.push_str("\n");
    code.push_str(&rules);

    if let Some(stages) = build_stages_code(&stages, &game_mode) {
        code.push_str("\n");
        code.push_str(&stages);
    }

    return code;
}
