use crate::inventory::*;
use chrono::*;
use sha256::digest;
use std::cmp;

pub struct Actor {
    pub name: String,
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
    pub inventory: Inventory,
}

pub struct Attribute {
    pub attribute: Attributes,
    pub base_value: i64,
    pub buff_value: i64,
}

pub enum Attributes {
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
}

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
            attribute: Attributes::HPCurrent,
            base_value: 1,
            buff_value: 0,
        },
        hp_max: Attribute {
            attribute: Attributes::HPMax,
            base_value: 1,
            buff_value: 0,
        },
        mp_current: Attribute {
            attribute: Attributes::MPCurrent,
            base_value: 1,
            buff_value: 0,
        },
        mp_max: Attribute {
            attribute: Attributes::MPMax,
            base_value: 1,
            buff_value: 0,
        },
        str: Attribute {
            attribute: Attributes::Strength,
            base_value: 1,
            buff_value: 0,
        },
        dex: Attribute {
            attribute: Attributes::Dexterity,
            base_value: 1,
            buff_value: 0,
        },
        con: Attribute {
            attribute: Attributes::Constitution,
            base_value: 1,
            buff_value: 0,
        },
        int: Attribute {
            attribute: Attributes::Intelligence,
            base_value: 1,
            buff_value: 0,
        },
        wis: Attribute {
            attribute: Attributes::Wisdom,
            base_value: 1,
            buff_value: 0,
        },
        cha: Attribute {
            attribute: Attributes::Charisma,
            base_value: 1,
            buff_value: 0,
        },
        encumberence_current: Attribute {
            attribute: Attributes::EncumberenceCurrent,
            base_value: 1,
            buff_value: 0,
        },
        encumberence_max: Attribute {
            attribute: Attributes::EncumberenceMax,
            base_value: 1,
            buff_value: 0,
        },
        inventory: Inventory {
            items: vec![InventoryItem {
                name: String::from("Health Potion"),
                weight: 1,
                weapon: None,
                armor: None,
            }],
        },
    };

    // initialize actor values - should this be a seperate function?
    actor.str.base_value = get_int_from_seed(&seed, 0) + 3;
    actor.dex.base_value = get_int_from_seed(&seed, 1) + 3;
    actor.con.base_value = get_int_from_seed(&seed, 2) + 3;
    actor.int.base_value = get_int_from_seed(&seed, 3) + 3;
    actor.wis.base_value = get_int_from_seed(&seed, 4) + 3;
    actor.cha.base_value = get_int_from_seed(&seed, 5) + 3;

    // hp/mp
    actor.hp_current.base_value = actor.con.base_value;
    actor.hp_max.base_value = actor.con.base_value;
    actor.mp_current.base_value = cmp::max(actor.wis.base_value, actor.int.base_value);
    actor.mp_max.base_value = cmp::max(actor.wis.base_value, actor.int.base_value);

    // ## set inventory capacity and initial items
    actor.encumberence_max.base_value = (actor.str.base_value + actor.con.base_value) / 2;

    //this seems awkward - shouldnt this be `actor.inventory.list.push("Health Potion")`?
    Inventory::new_consumable(&mut actor, &"Mana Potion".to_string());

    return actor;
}

// function to convert char from sha256 hash to int for use in player stats
fn get_int_from_seed(seed: &String, input: usize) -> i64 {
    let hex_char = seed.chars().nth(input).unwrap().to_string(); // UNWRAP UNWRAP!
    let i = i64::from_str_radix(&hex_char, 16).unwrap();
    return i;
}

pub struct BuffStack {
    pub buffs: Vec<Buff>,
    pub sum_str: i64,
}

pub struct Buff {
    name: String,
    duration: i64,
    mod_attribute: String,
    mod_base: i64,
    mod_scale: f64,
}

fn create_buff(mut actor: Actor) {
    let buff = Buff {
        name: String::from("Str"),
        duration: 1,
        mod_attribute: String::from("Strength"),
        mod_base: 1,
        mod_scale: 0.1,
    };

    //add to actor buff stack
}

pub trait Update {
    fn update_buff_stack(&mut self) {}
}

impl Update for BuffStack {
    fn update_buff_stack(&mut self) {
        //initialize sum buff
        let mut sum_str = 0; //TODO: should initialize this as i64

        // tick down each buff
        for i in 0..self.buffs.len() {
            self.buffs[i].duration = self.buffs[i].duration - 1;

            // remove buff if expired
            if self.buffs[i].duration <= 0 {
                self.buffs.remove(i);
            } else {
                sum_str = sum_str + self.buffs[i].mod_base;
                //TODO: implement the scaling factor for player strength (and other attributes)
            }
        }

        //update the buff stack's sum of str
        self.sum_str = sum_str;
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
