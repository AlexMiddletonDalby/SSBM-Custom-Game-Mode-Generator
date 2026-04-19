use crate::app::check_list::CheckboxEntry;

#[derive(Debug)]
pub struct Entry {
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
pub fn default_stages() -> Vec<Entry> {
    vec![
        Entry {
            bit: 7,
            checkbox: CheckboxEntry::new("Battlefield", true),
        },
        Entry {
            bit: 11,
            checkbox: CheckboxEntry::new("Big Blue", false),
        },
        Entry {
            bit: 28,
            checkbox: CheckboxEntry::new("Brinstar", false),
        },
        Entry {
            bit: 16,
            checkbox: CheckboxEntry::new("Brinstar Depths", false),
        },
        Entry {
            bit: 25,
            checkbox: CheckboxEntry::new("Corneria", false),
        },
        Entry {
            bit: 5,
            checkbox: CheckboxEntry::new("Dream Land N64", true),
        },
        Entry {
            bit: 6,
            checkbox: CheckboxEntry::new("Final Destination", true),
        },
        Entry {
            bit: 8,
            checkbox: CheckboxEntry::new("Flat Zone", false),
        },
        Entry {
            bit: 26,
            checkbox: CheckboxEntry::new("Fountain of Dreams", true),
        },
        Entry {
            bit: 10,
            checkbox: CheckboxEntry::new("Fourside", false),
        },
        Entry {
            bit: 29,
            checkbox: CheckboxEntry::new("Great Bay", false),
        },
        Entry {
            bit: 14,
            checkbox: CheckboxEntry::new("Green Greens", false),
        },
        Entry {
            bit: 21,
            checkbox: CheckboxEntry::new("Icicle Mountain", false),
        },
        Entry {
            bit: 18,
            checkbox: CheckboxEntry::new("Jungle Japes", false),
        },
        Entry {
            bit: 30,
            checkbox: CheckboxEntry::new("Kongo Jungle", false),
        },
        Entry {
            bit: 3,
            checkbox: CheckboxEntry::new("Kongo Jungle N64", false),
        },
        Entry {
            bit: 20,
            checkbox: CheckboxEntry::new("Mushroom Kingdom", false),
        },
        Entry {
            bit: 9,
            checkbox: CheckboxEntry::new("Mushroom Kingdom II", false),
        },
        Entry {
            bit: 23,
            checkbox: CheckboxEntry::new("Mute City", false),
        },
        Entry {
            bit: 22,
            checkbox: CheckboxEntry::new("Onett", false),
        },
        Entry {
            bit: 12,
            checkbox: CheckboxEntry::new("Poke Floats", false),
        },
        Entry {
            bit: 24,
            checkbox: CheckboxEntry::new("Pokemon Stadium", true),
        },
        Entry {
            bit: 31,
            checkbox: CheckboxEntry::new("Princess Peach's Castle", false),
        },
        Entry {
            bit: 19,
            checkbox: CheckboxEntry::new("Rainbow Cruise", false),
        },
        Entry {
            bit: 17,
            checkbox: CheckboxEntry::new("Temple", false),
        },
        Entry {
            bit: 13,
            checkbox: CheckboxEntry::new("Venom", false),
        },
        Entry {
            bit: 15,
            checkbox: CheckboxEntry::new("Yoshi's Island", false),
        },
        Entry {
            bit: 4,
            checkbox: CheckboxEntry::new("Yoshi's Island N64", false),
        },
        Entry {
            bit: 27,
            checkbox: CheckboxEntry::new("Yoshi's Story", true),
        },
    ]
}

// Item bitfield indices as defined from left to right in 64 bit space (as there are more than 32 items)
pub fn default_items() -> Vec<Entry> {
    vec![
        Entry {
            bit: 45,
            checkbox: CheckboxEntry::new("Food", true),
        },
        Entry {
            bit: 54,
            checkbox: CheckboxEntry::new("Maxim Tomato", true),
        },
        Entry {
            bit: 55,
            checkbox: CheckboxEntry::new("Heart Container", true),
        },
        Entry {
            bit: 34,
            checkbox: CheckboxEntry::new("Warp Star", true),
        },
        Entry {
            bit: 47,
            checkbox: CheckboxEntry::new("Ray Gun", true),
        },
        Entry {
            bit: 42,
            checkbox: CheckboxEntry::new("Super Scope", true),
        },
        Entry {
            bit: 38,
            checkbox: CheckboxEntry::new("Fire Flower", true),
        },
        Entry {
            bit: 40,
            checkbox: CheckboxEntry::new("Lip's Stick", true),
        },
        Entry {
            bit: 41,
            checkbox: CheckboxEntry::new("Star Rod", true),
        },
        Entry {
            bit: 51,
            checkbox: CheckboxEntry::new("Beam Sword", true),
        },
        Entry {
            bit: 52,
            checkbox: CheckboxEntry::new("Home-Run Bat", true),
        },
        Entry {
            bit: 39,
            checkbox: CheckboxEntry::new("Fan", true),
        },
        Entry {
            bit: 35,
            checkbox: CheckboxEntry::new("Hammer", true),
        },
        Entry {
            bit: 49,
            checkbox: CheckboxEntry::new("Green Shell", true),
        },
        Entry {
            bit: 48,
            checkbox: CheckboxEntry::new("Red Shell", true),
        },
        Entry {
            bit: 43,
            checkbox: CheckboxEntry::new("Flipper", true),
        },
        Entry {
            bit: 46,
            checkbox: CheckboxEntry::new("Freezie", true),
        },
        Entry {
            bit: 56,
            checkbox: CheckboxEntry::new("Mr. Saturn", true),
        },
        Entry {
            bit: 29,
            checkbox: CheckboxEntry::new("Poke Ball", true),
        },
        Entry {
            bit: 57,
            checkbox: CheckboxEntry::new("Bob-omb", true),
        },
        Entry {
            bit: 44,
            checkbox: CheckboxEntry::new("Motion-Sensor Bomb", true),
        },
        Entry {
            bit: 37,
            checkbox: CheckboxEntry::new("Super Mushroom", true),
        },
        Entry {
            bit: 36,
            checkbox: CheckboxEntry::new("Poison Mushroom", true),
        },
        Entry {
            bit: 53,
            checkbox: CheckboxEntry::new("Starman", true),
        },
        Entry {
            bit: 50,
            checkbox: CheckboxEntry::new("Parasol", true),
        },
        Entry {
            bit: 33,
            checkbox: CheckboxEntry::new("Screw Attack", true),
        },
        Entry {
            bit: 31,
            checkbox: CheckboxEntry::new("Metal Box", true),
        },
        Entry {
            bit: 32,
            checkbox: CheckboxEntry::new("Bunny Hood", true),
        },
        Entry {
            bit: 30,
            checkbox: CheckboxEntry::new("Cloaking Device", true),
        },
        Entry {
            bit: 58,
            checkbox: CheckboxEntry::new("Barrel Cannon", true),
        },
        Entry {
            bit: 59,
            checkbox: CheckboxEntry::new("Party Ball", true),
        },
    ]
}
