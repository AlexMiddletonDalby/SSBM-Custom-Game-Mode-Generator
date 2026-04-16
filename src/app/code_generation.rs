use bit_vec::BitVec;

const METADATA_TEMPLATE: &str = "${1} [{2}]";

const WATERMARK: &str = "#Generated with ssbm-custom-game-mode-generator | AlexMD =)";

const STOCKS_CODE_TEMPLATE: &str = "C216E91C 0000000F
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

const TIME_LIMIT_TEMPLATE: &str = "C216E750 00000007
3CE08048 80E79D30
54E7443E 2C070208
40820020 886DAFA0
2C03000{1} 40820014
3A40{2} 3E808048
62940540 92540000
3C808017 00000000";

const NO_TIME_LIMIT_TEMPLATE: &str = "C216E750 00000008
3CE08048 80E79D30
54E7443E 2C070208
40820024 886DAFA0
2C03000{1} 40820018
3E808048 62940530
3E403001 6252864C
92540000 3C808017
60000000 00000000";

const STAGE_CODE_TEMPLATE: &str = "C22668BC 00000009
88EDAFA0 2C07000{1}
41820008 4082001C
3E208045 6231C370
3E00{2} 6210{3}
92110018 4800001C
3E208045 6231C370
3E000700 621000B0
92110018 48000004
60000000 00000000";

const ITEMS_CODE_TEMPLATE: &str = "C216E774 0000000B
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

pub struct Bit {
    pub pos: usize,
    pub state: bool,
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
    let mut metadata_code = METADATA_TEMPLATE
        .replace("{1}", &metadata.name)
        .replace("{2}", &metadata.author);
    if let Some(description) = metadata.description {
        metadata_code.push_str("\n");
        metadata_code.push_str("*");
        metadata_code.push_str(&description);
    }

    let game_mode_val = game_mode.val().to_string();
    let mut stages_bitset = BitVec::from_elem(32, false);

    let stocks_val = format!("{:01$X}", stocks, 4);
    let stocks_code = STOCKS_CODE_TEMPLATE
        .replace("{1}", &game_mode_val)
        .replace("{2}", &stocks_val);

    let time_limit_code = match time_limit {
        Some(limit) => TIME_LIMIT_TEMPLATE
            .replace("{1}", &game_mode_val)
            .replace("{2}", &format!("{:01$X}", limit as u16 * 60, 4)),
        None => NO_TIME_LIMIT_TEMPLATE.replace("{1}", &game_mode_val),
    };

    for entry in stages {
        stages_bitset.set(entry.pos, entry.state);
    }

    let mut stages1 = to_hex(&stages_bitset.to_string(), 8);
    let stages2 = stages1.split_off(4);

    let stage_code = STAGE_CODE_TEMPLATE
        .replace("{1}", &game_mode_val)
        .replace("{2}", &stages1)
        .replace("{3}", &stages2);

    let mut items_code = String::new();
    if let Ok(item_frequency_val) = item_frequency.val() {
        let mut items_bitset = BitVec::from_elem(32, true);
        for entry in items {
            items_bitset.set(entry.pos, entry.state);
        }

        items_code = ITEMS_CODE_TEMPLATE
            .replace("{1}", &game_mode_val)
            .replace("{2}", &item_frequency_val.to_string())
            .replace("{3}", &to_hex(&items_bitset.to_string(), 8));
    }

    return metadata_code
        + "\n"
        + WATERMARK
        + "\n"
        + &stocks_code
        + "\n"
        + &time_limit_code
        + "\n"
        + &stage_code
        + "\n"
        + &items_code;
}
