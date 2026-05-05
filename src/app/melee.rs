use crate::app::check_list::CheckboxEntry;

#[derive(Debug)]
pub struct StageEntry {
    pub bit: usize,
    pub checkbox: CheckboxEntry,
}

pub fn default_stocks() -> u8 {
    4
}

pub fn default_time() -> u8 {
    8
}

// Stage bitfield indices as defined from left to right in 32 bit space
pub fn default_stages() -> Vec<StageEntry> {
    vec![
        StageEntry {
            bit: 7,
            checkbox: CheckboxEntry::new("Battlefield", true),
        },
        StageEntry {
            bit: 11,
            checkbox: CheckboxEntry::new("Big Blue", false),
        },
        StageEntry {
            bit: 28,
            checkbox: CheckboxEntry::new("Brinstar", false),
        },
        StageEntry {
            bit: 16,
            checkbox: CheckboxEntry::new("Brinstar Depths", false),
        },
        StageEntry {
            bit: 25,
            checkbox: CheckboxEntry::new("Corneria", false),
        },
        StageEntry {
            bit: 5,
            checkbox: CheckboxEntry::new("Dream Land N64", true),
        },
        StageEntry {
            bit: 6,
            checkbox: CheckboxEntry::new("Final Destination", true),
        },
        StageEntry {
            bit: 8,
            checkbox: CheckboxEntry::new("Flat Zone", false),
        },
        StageEntry {
            bit: 26,
            checkbox: CheckboxEntry::new("Fountain of Dreams", true),
        },
        StageEntry {
            bit: 10,
            checkbox: CheckboxEntry::new("Fourside", false),
        },
        StageEntry {
            bit: 29,
            checkbox: CheckboxEntry::new("Great Bay", false),
        },
        StageEntry {
            bit: 14,
            checkbox: CheckboxEntry::new("Green Greens", false),
        },
        StageEntry {
            bit: 21,
            checkbox: CheckboxEntry::new("Icicle Mountain", false),
        },
        StageEntry {
            bit: 18,
            checkbox: CheckboxEntry::new("Jungle Japes", false),
        },
        StageEntry {
            bit: 30,
            checkbox: CheckboxEntry::new("Kongo Jungle", false),
        },
        StageEntry {
            bit: 3,
            checkbox: CheckboxEntry::new("Kongo Jungle N64", false),
        },
        StageEntry {
            bit: 20,
            checkbox: CheckboxEntry::new("Mushroom Kingdom", false),
        },
        StageEntry {
            bit: 9,
            checkbox: CheckboxEntry::new("Mushroom Kingdom II", false),
        },
        StageEntry {
            bit: 23,
            checkbox: CheckboxEntry::new("Mute City", false),
        },
        StageEntry {
            bit: 22,
            checkbox: CheckboxEntry::new("Onett", false),
        },
        StageEntry {
            bit: 12,
            checkbox: CheckboxEntry::new("Poke Floats", false),
        },
        StageEntry {
            bit: 24,
            checkbox: CheckboxEntry::new("Pokemon Stadium", true),
        },
        StageEntry {
            bit: 31,
            checkbox: CheckboxEntry::new("Princess Peach's Castle", false),
        },
        StageEntry {
            bit: 19,
            checkbox: CheckboxEntry::new("Rainbow Cruise", false),
        },
        StageEntry {
            bit: 17,
            checkbox: CheckboxEntry::new("Temple", false),
        },
        StageEntry {
            bit: 13,
            checkbox: CheckboxEntry::new("Venom", false),
        },
        StageEntry {
            bit: 15,
            checkbox: CheckboxEntry::new("Yoshi's Island", false),
        },
        StageEntry {
            bit: 4,
            checkbox: CheckboxEntry::new("Yoshi's Island N64", false),
        },
        StageEntry {
            bit: 27,
            checkbox: CheckboxEntry::new("Yoshi's Story", true),
        },
    ]
}

#[derive(Debug)]
pub struct ItemEntry {
    pub field: usize,
    pub bit: usize,
    pub checkbox: CheckboxEntry,
}

// Item bitfield indices as defined from left to right (split into 5 8-bit fields)
pub fn default_items() -> Vec<ItemEntry> {
    vec![
        ItemEntry {
            field: 3,
            bit: 5,
            checkbox: CheckboxEntry::new("Food", true),
        },
        ItemEntry {
            field: 4,
            bit: 6,
            checkbox: CheckboxEntry::new("Maxim Tomato", true),
        },
        ItemEntry {
            field: 4,
            bit: 7,
            checkbox: CheckboxEntry::new("Heart Container", true),
        },
        ItemEntry {
            field: 2,
            bit: 2,
            checkbox: CheckboxEntry::new("Warp Star", true),
        },
        ItemEntry {
            field: 3,
            bit: 7,
            checkbox: CheckboxEntry::new("Ray Gun", true),
        },
        ItemEntry {
            field: 3,
            bit: 2,
            checkbox: CheckboxEntry::new("Super Scope", true),
        },
        ItemEntry {
            field: 2,
            bit: 6,
            checkbox: CheckboxEntry::new("Fire Flower", true),
        },
        ItemEntry {
            field: 3,
            bit: 0,
            checkbox: CheckboxEntry::new("Lip's Stick", true),
        },
        ItemEntry {
            field: 3,
            bit: 1,
            checkbox: CheckboxEntry::new("Star Rod", true),
        },
        ItemEntry {
            field: 4,
            bit: 3,
            checkbox: CheckboxEntry::new("Beam Sword", true),
        },
        ItemEntry {
            field: 4,
            bit: 4,
            checkbox: CheckboxEntry::new("Home-Run Bat", true),
        },
        ItemEntry {
            field: 2,
            bit: 7,
            checkbox: CheckboxEntry::new("Fan", true),
        },
        ItemEntry {
            field: 2,
            bit: 3,
            checkbox: CheckboxEntry::new("Hammer", true),
        },
        ItemEntry {
            field: 4,
            bit: 1,
            checkbox: CheckboxEntry::new("Green Shell", true),
        },
        ItemEntry {
            field: 4,
            bit: 0,
            checkbox: CheckboxEntry::new("Red Shell", true),
        },
        ItemEntry {
            field: 3,
            bit: 3,
            checkbox: CheckboxEntry::new("Flipper", true),
        },
        ItemEntry {
            field: 3,
            bit: 6,
            checkbox: CheckboxEntry::new("Freezie", true),
        },
        ItemEntry {
            field: 5,
            bit: 0,
            checkbox: CheckboxEntry::new("Mr. Saturn", true),
        },
        ItemEntry {
            field: 1,
            bit: 5,
            checkbox: CheckboxEntry::new("Poke Ball", true),
        },
        ItemEntry {
            field: 5,
            bit: 1,
            checkbox: CheckboxEntry::new("Bob-omb", true),
        },
        ItemEntry {
            field: 3,
            bit: 4,
            checkbox: CheckboxEntry::new("Motion-Sensor Bomb", true),
        },
        ItemEntry {
            field: 2,
            bit: 5,
            checkbox: CheckboxEntry::new("Super Mushroom", true),
        },
        ItemEntry {
            field: 2,
            bit: 4,
            checkbox: CheckboxEntry::new("Poison Mushroom", true),
        },
        ItemEntry {
            field: 4,
            bit: 5,
            checkbox: CheckboxEntry::new("Starman", true),
        },
        ItemEntry {
            field: 4,
            bit: 2,
            checkbox: CheckboxEntry::new("Parasol", true),
        },
        ItemEntry {
            field: 2,
            bit: 1,
            checkbox: CheckboxEntry::new("Screw Attack", true),
        },
        ItemEntry {
            field: 1,
            bit: 7,
            checkbox: CheckboxEntry::new("Metal Box", true),
        },
        ItemEntry {
            field: 2,
            bit: 0,
            checkbox: CheckboxEntry::new("Bunny Hood", true),
        },
        ItemEntry {
            field: 1,
            bit: 6,
            checkbox: CheckboxEntry::new("Cloaking Device", true),
        },
        ItemEntry {
            field: 5,
            bit: 2,
            checkbox: CheckboxEntry::new("Barrel Cannon", true),
        },
        ItemEntry {
            field: 5,
            bit: 3,
            checkbox: CheckboxEntry::new("Party Ball", true),
        },
    ]
}
