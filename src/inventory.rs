use crate::actors::{self, *};
use chrono::*;
use sha256::digest;
use std::cmp;

#[derive(PartialEq, Clone)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
}

#[derive(PartialEq, Clone)]
pub struct InventoryItem {
    pub name: String,
    pub weight: i64,
    //pub item_type: ItemType,
    //pub consumable : Option<Consumable>,
    //pub equipable : Option<Equipable>,
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
}

// #[derive(PartialEq,Debug)]
// pub struct Consumable {
//     pub attribute: String,
//     pub restoration: f64,
// }

// #[derive(PartialEq, Debug, Clone)]
// pub enum EngagementType {
//     Melee,
//     Ranged,
//     Chant,
// }

#[derive(PartialEq, Clone)]
pub struct Weapon {
    // TODO: Add prefix / sufix Ideas for weapon stats and buffs: https://docs.google.com/spreadsheets/d/1RAJsvRAr0oMyn9X9nVzr5ZwHW8lVhq2PiNhLdk98Yd8/edit#gid=0
    pub name: String,
    pub is_melee: bool,
    pub is_ranged: bool,
    pub is_one_handed: bool,
    pub is_two_handed: bool,
    pub is_shield: bool,
    pub encumberence_base: i64,
    pub encumberence_buff: i64,

    // the follow stats to be determined by hash of weapon name
    pub damage_base: i64,
    pub damage_buff: i64,
    pub armor_base: i64,
    pub armor_buff: i64,
    pub speed_base: i64,
    pub speed_buff: i64,
    pub str_scaling: f64,
    pub dex_scaling: f64,
    pub int_scaling: f64,
    pub wis_scaling: f64,
    pub name_prefix: String,

    pub buffs: Vec<actors::Buff>,
}

pub fn create_random_1h_weapon(index: i64) -> Weapon {
    let mut weapon = match index {
        0 => create_named_weapon(&String::from("Cane"), true, false, true, false, false, 2),
        1 => create_named_weapon(
            &String::from("Stiletto"),
            true,
            false,
            true,
            false,
            false,
            1,
        ),
        2 => create_named_weapon(&String::from("Mace"), true, false, true, false, false, 3),
        3 => create_named_weapon(
            &String::from("Morning Star"),
            true,
            false,
            true,
            false,
            false,
            3,
        ),
        4 => create_named_weapon(&String::from("Club"), true, false, true, false, false, 3),
        5 => create_named_weapon(
            &String::from("Falchion"),
            true,
            false,
            true,
            false,
            false,
            4,
        ),
        6 => create_named_weapon(
            &String::from("Shamshir"),
            true,
            false,
            true,
            false,
            false,
            3,
        ),
        7 => create_named_weapon(&String::from("Kopis"), true, false, true, false, false, 3),
        8 => create_named_weapon(&String::from("Falcata"), true, false, true, false, false, 3),
        9 => create_named_weapon(
            &String::from("Katzbalger"),
            true,
            false,
            true,
            false,
            false,
            3,
        ),
        10 => create_named_weapon(
            &String::from("Scimitar"),
            true,
            false,
            true,
            false,
            false,
            3,
        ),
        11 => create_named_weapon(&String::from("Katana"), true, false, true, false, false, 3),
        12 => create_named_weapon(&String::from("Rapier"), true, false, true, false, false, 2),
        13 => create_named_weapon(&String::from("Shashka"), true, false, true, false, false, 3),
        14 => create_named_weapon(&String::from("Shotel"), true, false, true, false, false, 2),
        15 => create_named_weapon(
            &String::from("Wakizashi"),
            true,
            false,
            true,
            false,
            false,
            2,
        ),
        16 => create_named_weapon(&String::from("Parang"), true, false, true, false, false, 2),
        _ => create_named_weapon(
            &String::from("Pitchfork"),
            true,
            false,
            true,
            false,
            false,
            3,
        ),
    };

    return weapon;
}

pub fn create_random_1h_shield(index: i64) -> Weapon {
    let mut weapon = match index {
        0 | 1 => create_named_weapon(
            &String::from("Heater shield"),
            false,
            false,
            true,
            false,
            true,
            4,
        ),
        2 | 3 => create_named_weapon(
            &String::from("Kite shield"),
            false,
            false,
            true,
            false,
            true,
            4,
        ),
        4 | 5 => create_named_weapon(
            &String::from("Buckler shield"),
            false,
            false,
            true,
            false,
            true,
            2,
        ),
        6 | 7 => create_named_weapon(
            &String::from("Pavise shield"),
            false,
            false,
            true,
            false,
            true,
            6,
        ),
        8 | 9 => create_named_weapon(
            &String::from("Targe shield"),
            false,
            false,
            true,
            false,
            true,
            3,
        ),
        10 | 11 => create_named_weapon(
            &String::from("Rondache shield"),
            false,
            false,
            true,
            false,
            true,
            2,
        ),
        12 => create_named_weapon(
            &String::from("Dhal shield"),
            false,
            false,
            true,
            false,
            true,
            2,
        ),
        13 => create_named_weapon(
            &String::from("Aspis shield"),
            false,
            false,
            true,
            false,
            true,
            2,
        ),
        14 => create_named_weapon(
            &String::from("Scutum shield"),
            false,
            false,
            true,
            false,
            true,
            2,
        ),
        15 => create_named_weapon(
            &String::from("Chimalli shield"),
            false,
            false,
            true,
            false,
            true,
            2,
        ),
        16 => create_named_weapon(
            &String::from("Adarga shield"),
            false,
            false,
            true,
            false,
            true,
            2,
        ),
        _ => create_named_weapon(
            &String::from("Kettle Lid"),
            false,
            false,
            true,
            false,
            true,
            3,
        ),
    };

    return weapon;
}

pub fn create_named_weapon(
    name: &String,
    is_melee: bool,
    is_ranged: bool,
    is_one_handed: bool,
    is_two_handed: bool,
    is_shield: bool,
    encumberence_base: i64,
) -> Weapon {
    let seed = digest(name);

    let mut weapon = Weapon {
        name: String::from(name),
        is_melee: is_melee,
        is_ranged: is_ranged,
        is_one_handed: is_one_handed,
        is_two_handed: is_two_handed,
        is_shield: is_shield,
        encumberence_base: encumberence_base,
        encumberence_buff: 0,

        damage_base: if is_melee {
            actors::get_int_from_seed(&seed, 0)
        } else {
            0
        },
        damage_buff: 0,
        armor_base: if is_shield {
            actors::get_int_from_seed(&seed, 7)
        } else {
            0
        },
        armor_buff: 0,
        speed_base: actors::get_int_from_seed(&seed, 2),
        speed_buff: 0,
        str_scaling: (100 / (actors::get_int_from_seed(&seed, 3) + 4) / 100) as f64,
        dex_scaling: (100 / (actors::get_int_from_seed(&seed, 4) + 4) / 100) as f64,
        int_scaling: (100 / (actors::get_int_from_seed(&seed, 5) + 4) / 100) as f64,
        wis_scaling: (100 / (actors::get_int_from_seed(&seed, 6) + 4) / 100) as f64,
        name_prefix: String::from(""),

        buffs: Vec::new(),
    };

    if weapon.is_two_handed {
        weapon.damage_base = (weapon.damage_base as f64 * 1.25) as i64;
        weapon.speed_base = (weapon.speed_base as f64 * 0.50) as i64;
        weapon.encumberence_base = (weapon.encumberence_base as f64 * 1.5) as i64;
    };

    weapon.name_prefix = match weapon.damage_base {
        1 => String::from("Begger's"),
        2 => String::from("Slave's"),
        3 => String::from("Serf's"),
        4 => String::from("Vagabond's"),
        5 => String::from("Peasant's"),
        6 => String::from("Farmer's"),
        7 => String::from("Yeoman's"),
        8 => String::from("Knight's"),
        9 => String::from("Champion's"),
        10 => String::from("Thane's"),
        11 => String::from("Castellan's"),
        12 => String::from("Duke's"),
        13 => String::from("Prince's"),
        14 => String::from("Queen's"),
        15 => String::from("Tsarina's"),
        16.. => String::from("Demi-God's"),
        _ => String::from("Jester's"),
    };

    return weapon;
}

#[derive(PartialEq, Clone)]
pub struct Armor {
    pub name: String,
    pub armor_base: i64,
    pub armor_buff: i64,
    pub encumberence_base: i64,
    pub encumberence_buff: i64,
    pub buffs: Vec<actors::Buff>,
}

pub trait Use {
    fn use_consumable(character: &mut Actor, item_name: &String) {}
    fn new_consumable(character: &mut Actor, item_name: &String) {}
}

impl Use for Inventory {
    fn new_consumable(character: &mut Actor, item_name: &String) {
        //check if encumberence too high

        match item_name.to_string().as_str() {
            "Health Potion" => {
                let new_item = InventoryItem {
                    name: String::from("Health Potion"),
                    weight: 1,
                    weapon: None,
                    armor: None,
                };
                character.inventory.push(new_item);
            }

            "Mana Potion" => {
                let new_item = InventoryItem {
                    name: String::from("Mana Potion"),
                    weight: 1,
                    weapon: None,
                    armor: None,
                };
                character.inventory.push(new_item);
            }
            _ => println!("No Item Created"),
        }
    }

    fn use_consumable(character: &mut Actor, item_name: &String) {
        // get item index
        //println!("{:?}", item_name);
        //println!("{:?}", character.inventory.items.iter());
        let item_index = character
            .inventory
            .iter()
            .position(|item| String::from(&item.name) == String::from(item_name));

        if item_index == None {
            println!("{} has no {} in inventory!", character.name, item_name);
            return;
        } else {
            // remove from inventory
            println!("{} drinks a potion: *GULP!*", character.name);
            character.inventory.remove(item_index.unwrap());
        }

        // do the effect
        match item_name.to_string().as_str() {
            "Health Potion" => {
                character.hp_current.base_value = cmp::min(
                    character.hp_current.base_value
                        + ((character.str.base_value + character.con.base_value) / 2),
                    character.hp_max.base_value,
                )
            }
            "Mana Potion" => {
                character.mp_current.base_value = cmp::min(
                    character.mp_max.base_value,
                    character.mp_current.base_value + (character.mp_max.base_value / 2),
                )
            }
            _ => println!(""),
        };
    }
}
