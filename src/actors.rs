use crate::inventory::*;
use chrono::*;
use sha256::digest;
use std::{cmp, ops::Index};

#[derive(Clone)]
pub struct Actor {
    pub name: String,
    pub buffs: Vec<Buff>,
    pub inventory: Vec<InventoryItem>,
    // I considered putting the following Attributes in to a vector, but found that it complicated the code without adding much value.
    // TODO: Revisit to see if I think putting the attributes in a vector would be better
    pub hp_current: Attribute,
    pub hp_max: Attribute,
    pub mp_current: Attribute,
    pub mp_max: Attribute,
    pub str: Attribute,
    pub dex: Attribute,
    pub con: Attribute,
    pub int: Attribute,
    pub wis: Attribute,
    pub cha: Attribute,
    pub encumberence_current: Attribute,
    pub encumberence_max: Attribute,
    pub defense: Attribute,
    pub offense: Attribute,
    pub slot_armor: Armor,
    pub slot_left_hand: Weapon,
    pub slot_right_hand: Weapon,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Attribute {
    pub attribute: AttributeName,
    pub base_value: i64,
    pub buff_value: i64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AttributeName {
    HPCurrent,
    HPMax,
    MPCurrent,
    MPMax,
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    EncumberenceCurrent,
    EncumberenceMax,
    EncumberenceValue,
    Defense,
    Offense,
    ArmorValue,
    Regeneration,
    WeaponBaseDamage,
}

#[derive(PartialEq, Clone)]
pub struct Buff {
    pub name: String,
    pub duration: i64,
    pub mod_attribute: Attribute,
    pub mod_flat: i64,
    pub mod_scale: f64,
}

// #[derive(Clone)]
// pub struct BuffStack {
//     pub effects: Vec<Buff>,
// }

#[derive(Clone)]
pub enum CharType {
    Player,
    Npc,
}

pub fn create_actor(mut name: String) -> Actor {
    let deity_index = chrono::offset::Local::now().month();
    let digest_message = format!("{}{}", deity_index, name);
    let seed = digest(digest_message);

    let mut actor = Actor {
        name: String::from(&name),
        hp_current: Attribute {
            attribute: AttributeName::HPCurrent,
            base_value: 1,
            buff_value: 0,
        },
        hp_max: Attribute {
            attribute: AttributeName::HPMax,
            base_value: 1,
            buff_value: 0,
        },
        mp_current: Attribute {
            attribute: AttributeName::MPCurrent,
            base_value: 1,
            buff_value: 0,
        },
        mp_max: Attribute {
            attribute: AttributeName::MPMax,
            base_value: 1,
            buff_value: 0,
        },
        str: Attribute {
            attribute: AttributeName::Strength,
            base_value: 1,
            buff_value: 0,
        },
        dex: Attribute {
            attribute: AttributeName::Dexterity,
            base_value: 1,
            buff_value: 0,
        },
        con: Attribute {
            attribute: AttributeName::Constitution,
            base_value: 1,
            buff_value: 0,
        },
        int: Attribute {
            attribute: AttributeName::Intelligence,
            base_value: 1,
            buff_value: 0,
        },
        wis: Attribute {
            attribute: AttributeName::Wisdom,
            base_value: 1,
            buff_value: 0,
        },
        cha: Attribute {
            attribute: AttributeName::Charisma,
            base_value: 1,
            buff_value: 0,
        },
        encumberence_current: Attribute {
            attribute: AttributeName::EncumberenceCurrent,
            base_value: 1,
            buff_value: 0,
        },
        encumberence_max: Attribute {
            attribute: AttributeName::EncumberenceMax,
            base_value: 1,
            buff_value: 0,
        },
        defense: Attribute {
            attribute: AttributeName::Defense,
            base_value: 0,
            buff_value: 0,
        },
        offense: Attribute {
            attribute: AttributeName::Defense,
            base_value: 0,
            buff_value: 0,
        },
        buffs: Vec::new(),
        inventory: vec![InventoryItem {
            name: String::from("Health Potion"),
            weight: 1,
            weapon: None,
            armor: None,
        }],
        slot_armor: create_random_armor(get_int_from_seed(&seed, 8)),

        slot_left_hand: create_random_1h_weapon(get_int_from_seed(&seed, 6)),
        slot_right_hand: create_random_1h_shield(get_int_from_seed(&seed, 7)),
    };

    actor.str.base_value = get_int_from_seed(&seed, 0) + 3;
    actor.dex.base_value = get_int_from_seed(&seed, 1) + 3;
    actor.con.base_value = get_int_from_seed(&seed, 2) + 3;
    actor.int.base_value = get_int_from_seed(&seed, 3) + 3;
    actor.wis.base_value = get_int_from_seed(&seed, 4) + 3;
    actor.cha.base_value = get_int_from_seed(&seed, 5) + 3;

    // actor's innate offensive and defensive power sans weapons/armor
    actor.defense.base_value = (actor.str.base_value + actor.dex.base_value) / 2;
    actor.offense.base_value = (actor.str.base_value + actor.dex.base_value) / 2;

    // hp/mp
    actor.hp_current.base_value = actor.con.base_value;
    actor.hp_max.base_value = actor.con.base_value;
    actor.mp_current.base_value = cmp::max(actor.wis.base_value, actor.int.base_value);
    actor.mp_max.base_value = cmp::max(actor.wis.base_value, actor.int.base_value);

    // ## set inventory capacity and initial items
    actor.encumberence_max.base_value = (actor.str.base_value + actor.con.base_value) / 2;

    //this seems awkward - shouldnt this be `actor.inventory.list.push("Health Potion")`?
    Inventory::new_consumable(&mut actor, &"Mana Potion".to_string());

    actor.buffs.push(Buff {
        name: String::from("Neophyte's Advantage"),
        duration: 1,
        mod_attribute: actor.defense,
        mod_flat: 1,
        mod_scale: 0.0,
    });

    return actor;
}

// function to convert char from sha256 hash to int for use in player stats
pub fn get_int_from_seed(seed: &String, input: usize) -> i64 {
    let hex_char = seed.chars().nth(input).unwrap().to_string(); // UNWRAP UNWRAP!
    let i = i64::from_str_radix(&hex_char, 16).unwrap();
    return i;
}

// fn create_buff(mut actor: Actor, buff_data: Buff) {
//     let buff = Buff {
//         name: String::from("Str"),
//         duration: 1,
//         mod_attribute: Attribute { attribute: attribute, base_value: (), buff_value: () },
//         mod_base: 1,
//         mod_scale: 0.1,
//     };

//     //add to actor buff stack
// }

pub trait Update {
    fn update_buff_stack(actor: &mut Actor) {}
}

impl Update for Buff {
    fn update_buff_stack(actor: &mut Actor) {
        // tick down each buff
        let mut remove_buff_indexes: Vec<usize> = Vec::new();
        for i in 0..actor.buffs.len() {
            actor.buffs[i].duration = actor.buffs[i].duration - 1;

            // if buff expired, add index of buff to vector to be removed outside of loop
            // else bad things happen if we try to remove buff via index while still inside the loop
            if actor.buffs[i].duration <= 0 {
                remove_buff_indexes.push(i);
                //
            }
        }

        // iterate through list of buff indexes that should be removed
        if remove_buff_indexes.len() > 0 {
            for i in 0..remove_buff_indexes.len() {
                actor.buffs.remove(i); //derefencing? Is this working?
            }
        }

        // recalculate buff stack effects
        // (super inefficient - should be incremental but probably within performance params for most systems)
        // TODO: Refactor for incremental additions/removals
        // set base buff values back to zero
        actor.offense.buff_value = 0;
        actor.defense.buff_value = 0;
        actor.hp_current.buff_value = 0;
        actor.hp_max.buff_value = 0;
        actor.str.buff_value = 0;
        actor.dex.buff_value = 0;
        actor.con.buff_value = 0;
        actor.int.buff_value = 0;
        actor.wis.buff_value = 0;
        actor.cha.buff_value = 0;

        for i in 0..actor.buffs.len() {
            match actor.buffs[i].mod_attribute.attribute {
                // TODO: Fill out all attributes match statement
                AttributeName::Strength => {
                    actor.str.buff_value = actor.str.buff_value + actor.buffs[i].mod_flat
                }
                AttributeName::Dexterity => {
                    actor.dex.buff_value = actor.dex.buff_value + actor.buffs[i].mod_flat
                }
                AttributeName::Constitution => {
                    actor.con.buff_value = actor.con.buff_value + actor.buffs[i].mod_flat
                }
                AttributeName::Intelligence => {
                    actor.int.buff_value = actor.int.buff_value + actor.buffs[i].mod_flat
                }
                AttributeName::Wisdom => {
                    actor.wis.buff_value = actor.wis.buff_value + actor.buffs[i].mod_flat
                }
                AttributeName::Charisma => {
                    actor.cha.buff_value = actor.cha.buff_value + actor.buffs[i].mod_flat
                }
                AttributeName::Offense => {
                    actor.offense.buff_value = actor.offense.buff_value + actor.buffs[i].mod_flat
                }
                AttributeName::Defense => {
                    actor.defense.buff_value = actor.defense.buff_value + actor.buffs[i].mod_flat
                }
                AttributeName::HPCurrent => {
                    actor.hp_current.buff_value =
                        actor.hp_current.buff_value + actor.buffs[i].mod_flat
                }
                AttributeName::HPMax => {
                    actor.hp_max.buff_value = actor.hp_max.buff_value + actor.buffs[i].mod_flat
                }
                _ => println!("*** BUFF NOT IMPLEMENTED *** "),
            }
        }
    }
    // for each buff in stack, tick, remove buffs that are expired; sum results; update buffstack attribute
}

// // Define Player, NPCS (enemies and friendlies), Objects, and respective enums
// pub struct Character{
//     pub name: String,
//     pub char_type: CharType,
//     pub hp_current: i64,
//     pub hp_max: i64,
//     pub mp_current: i64,
//     pub mp_max: i64,
//     pub str: i64,
//     pub dex: i64,
//     pub con: i64,
//     pub int: i64,
//     pub wis: i64,
//     pub cha: i64,
//     pub capacity_weight: i64,
//     pub capacity_volume: i64,
//     pub inventory: Inventory
// }

// pub fn create_player(mut name: String) -> Character{
//     // ## create the player's deity by getting the current system month
//     // This will also determine the character's deity.
//     let deity_index =  chrono::offset::Local::now().month();

//     // salt the character name with the deityIndex and get the sha256 hash
//     // the sha256 will determine the character's stats
//     let digest_message = format!("{}{}",deity_index,name);
//     let seed = digest(digest_message);

//     // ## create the player's character
//     let mut player = Character{
//         name: String::from(&name),
//         // TODO: add deity index, create struct with name of deity, and passive buff
//         hp_current: 1,
//         hp_max: 1,
//         mp_current: 1,
//         mp_max: 1,
//         char_type: CharType::Player,
//         capacity_volume: 1,
//         capacity_weight: 1,
//         //inventory:  Inventory { list: vec![InventoryItem::HealthPotion(String::from("Potion"), inventory::ItemType::Potion, 1)] },
//         inventory: Inventory {items: vec![InventoryItem { name: String::from("Health Potion"), weight: 1, weapon: None, armor: None }]},

//         // uses the sha256 hash to convert hexidecimal to int with range o to 16, then
//         // .. adds 3 to provide 3 to 18 range
//         str: get_int_from_seed(&seed, 0)+3,
//         dex: get_int_from_seed(&seed, 1)+3,
//         con: get_int_from_seed(&seed, 2)+3,
//         int: get_int_from_seed(&seed, 3)+3,
//         wis: get_int_from_seed(&seed, 4)+3,
//         cha: get_int_from_seed(&seed, 5)+3,
//     };

//     //add mana potion mostly for example for my future self
//     Inventory::new_consumable(&mut player, &"Mana Potion".to_string());

//     // ## set initial hp/mp values
//     player.hp_current = player.con;
//     player.hp_max = player.con;
//     player.mp_current = cmp::max(player.wis, player.int);
//     player.mp_max = cmp::max(player.wis, player.int);

//     // ## set inventory capacity and initial items
//     player.capacity_volume = player.con;
//     player.capacity_weight = player.str;

//     // # Respond to the Chosen.
//     println!("Very well, {}!", player.name);

//     return player;
// }
