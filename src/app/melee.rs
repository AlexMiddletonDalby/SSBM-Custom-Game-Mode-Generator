use crate::app::check_list::CheckboxEntry;

#[derive(Debug)]
pub struct Entry {
    pub bit: usize,
    pub checkbox: CheckboxEntry,
}

pub fn default_stages() -> Vec<Entry> {
    vec![
        Entry {
            bit: 0,
            checkbox: CheckboxEntry::new("Battlefield", true),
        },
        Entry {
            bit: 1,
            checkbox: CheckboxEntry::new("Big Blue", false),
        },
        Entry {
            bit: 2,
            checkbox: CheckboxEntry::new("Brinstar", false),
        },
        Entry {
            bit: 3,
            checkbox: CheckboxEntry::new("Brinstar Depths", false),
        },
        Entry {
            bit: 4,
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
            bit: 7,
            checkbox: CheckboxEntry::new("Flat Zone", false),
        },
        Entry {
            bit: 8,
            checkbox: CheckboxEntry::new("Fountain of Dreams", true),
        },
        Entry {
            bit: 9,
            checkbox: CheckboxEntry::new("Fourside", false),
        },
        Entry {
            bit: 10,
            checkbox: CheckboxEntry::new("Great Bay", false),
        },
        Entry {
            bit: 11,
            checkbox: CheckboxEntry::new("Green Greens", false),
        },
        Entry {
            bit: 12,
            checkbox: CheckboxEntry::new("Icicle Mountain", false),
        },
        Entry {
            bit: 13,
            checkbox: CheckboxEntry::new("Jungle Japes", false),
        },
        Entry {
            bit: 14,
            checkbox: CheckboxEntry::new("Kongo Jungle", false),
        },
        Entry {
            bit: 15,
            checkbox: CheckboxEntry::new("Kongo Jungle N64", false),
        },
        Entry {
            bit: 16,
            checkbox: CheckboxEntry::new("Mushroom Kingdom", false),
        },
        Entry {
            bit: 17,
            checkbox: CheckboxEntry::new("Mushroom Kingdom II", false),
        },
        Entry {
            bit: 18,
            checkbox: CheckboxEntry::new("Mute City", false),
        },
        Entry {
            bit: 19,
            checkbox: CheckboxEntry::new("Onett", false),
        },
        Entry {
            bit: 20,
            checkbox: CheckboxEntry::new("Poke Floats", false),
        },
        Entry {
            bit: 21,
            checkbox: CheckboxEntry::new("Pokemon Stadium", true),
        },
        Entry {
            bit: 22,
            checkbox: CheckboxEntry::new("Princess Peach's Castle", false),
        },
        Entry {
            bit: 23,
            checkbox: CheckboxEntry::new("Rainbow Cruise", false),
        },
        Entry {
            bit: 24,
            checkbox: CheckboxEntry::new("Temple", false),
        },
        Entry {
            bit: 25,
            checkbox: CheckboxEntry::new("Venom", false),
        },
        Entry {
            bit: 26,
            checkbox: CheckboxEntry::new("Yoshi's Island", false),
        },
        Entry {
            bit: 27,
            checkbox: CheckboxEntry::new("Yoshi's Island N64", false),
        },
        Entry {
            bit: 28,
            checkbox: CheckboxEntry::new("Yoshi's Story", true),
        },
    ]
}

pub fn default_items() -> Vec<Entry> {
    vec![
        Entry {
            bit: 0,
            checkbox: CheckboxEntry::new("Food", false),
        },
        Entry {
            bit: 1,
            checkbox: CheckboxEntry::new("Maxim Tomato", false),
        },
        Entry {
            bit: 2,
            checkbox: CheckboxEntry::new("Heart Container", false),
        },
        Entry {
            bit: 3,
            checkbox: CheckboxEntry::new("Warp Star", false),
        },
        Entry {
            bit: 4,
            checkbox: CheckboxEntry::new("Ray Gun", false),
        },
        Entry {
            bit: 5,
            checkbox: CheckboxEntry::new("Super Scope", false),
        },
        Entry {
            bit: 6,
            checkbox: CheckboxEntry::new("Fire Flower", false),
        },
        Entry {
            bit: 7,
            checkbox: CheckboxEntry::new("Lip's Stick", false),
        },
        Entry {
            bit: 8,
            checkbox: CheckboxEntry::new("Star Rod", false),
        },
        Entry {
            bit: 9,
            checkbox: CheckboxEntry::new("Beam Sword", false),
        },
        Entry {
            bit: 10,
            checkbox: CheckboxEntry::new("Home-Run Bat", false),
        },
        Entry {
            bit: 11,
            checkbox: CheckboxEntry::new("Fan", false),
        },
        Entry {
            bit: 12,
            checkbox: CheckboxEntry::new("Hammer", false),
        },
        Entry {
            bit: 13,
            checkbox: CheckboxEntry::new("Green Shell", false),
        },
        Entry {
            bit: 14,
            checkbox: CheckboxEntry::new("Red Shell", false),
        },
        Entry {
            bit: 15,
            checkbox: CheckboxEntry::new("Flipper", false),
        },
        Entry {
            bit: 16,
            checkbox: CheckboxEntry::new("Freezie", false),
        },
        Entry {
            bit: 17,
            checkbox: CheckboxEntry::new("Mr. Saturn", false),
        },
        Entry {
            bit: 18,
            checkbox: CheckboxEntry::new("Poke Ball", false),
        },
        Entry {
            bit: 19,
            checkbox: CheckboxEntry::new("Bob-omb", false),
        },
        Entry {
            bit: 20,
            checkbox: CheckboxEntry::new("Motion-Sensor Bomb", false),
        },
        Entry {
            bit: 21,
            checkbox: CheckboxEntry::new("Super Mushroom", false),
        },
        Entry {
            bit: 22,
            checkbox: CheckboxEntry::new("Poison Mushroom", false),
        },
        Entry {
            bit: 23,
            checkbox: CheckboxEntry::new("Starman", false),
        },
        Entry {
            bit: 24,
            checkbox: CheckboxEntry::new("Parasol", false),
        },
        Entry {
            bit: 25,
            checkbox: CheckboxEntry::new("Screw Attack", false),
        },
        Entry {
            bit: 26,
            checkbox: CheckboxEntry::new("Metal Box", false),
        },
        Entry {
            bit: 27,
            checkbox: CheckboxEntry::new("Bunny Hood", false),
        },
        Entry {
            bit: 28,
            checkbox: CheckboxEntry::new("Cloaking Device", false),
        },
        Entry {
            bit: 29,
            checkbox: CheckboxEntry::new("Barrel Cannon", false),
        },
        Entry {
            bit: 30,
            checkbox: CheckboxEntry::new("Party Ball", false),
        },
    ]
}
