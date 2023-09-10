use crate::actors::{self, *};
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
    // pub enagement_type: EngagementType,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub buffs: Vec<actors::Buff>,
}

#[derive(PartialEq, Clone)]
pub struct Armor {
    pub name: String,
    pub armor_value: Attribute,
    pub encumberence_value: Attribute,
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
            println!("No {} in inventory!", item_name);
            return;
        } else {
            // remove from inventory
            println!("{}: *GULP!*", character.name);
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
