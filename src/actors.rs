use crate::inventory::*;
use std::io;
use chrono::*;
use rand::Rng;
use sha256::digest;
use std::cmp;

// Define Player, NPCS (enemies and friendlies), Objects, and respective enums
pub struct Character{
    pub name: String,
    pub char_type: CharType,
    pub hp_current: i64,
    pub hp_max: i64,
    pub mp_current: i64,
    pub mp_max: i64,
    pub str: i64,
    pub dex: i64,
    pub con: i64,
    pub int: i64,
    pub wis: i64,
    pub cha: i64,
    pub capacity_weight: i64,
    pub capacity_volume: i64,
    pub inventory: Inventory,
}

pub enum CharType{
    Player,
    Npc,
}

pub fn create_player(mut name: String) -> Character{
    // ## create the player's deity by getting the current system month
    // This will also determine the character's deity.
    let deity_index =  chrono::offset::Local::now().month();

    // salt the character name with the deityIndex and get the sha256 hash
    // the sha256 will determine the character's stats
    let digest_message = format!("{}{}",deity_index,name);
    let seed = digest(digest_message);
    
    // ## create the player's character
    let mut player = Character{ 
        name: String::from(&name),
        // TODO: add deity index, create struct with name of deity, and passive buff
        hp_current: 1,
        hp_max: 1,
        mp_current: 1,
        mp_max: 1,
        char_type: CharType::Player,
        capacity_volume: 1,
        capacity_weight: 1,
        //inventory:  Inventory { list: vec![InventoryItem::HealthPotion(String::from("Potion"), inventory::ItemType::Potion, 1)] },
        inventory: Inventory {items: vec![InventoryItem { name: String::from("Health Potion"), weight: 1, weapon: None, armor: None }]},

        // uses the sha256 hash to convert hexidecimal to int with range o to 16, then
        // .. adds 3 to provide 3 to 18 range
        str: get_int_from_seed(&seed, 0)+3,
        dex: get_int_from_seed(&seed, 1)+3,
        con: get_int_from_seed(&seed, 2)+3,
        int: get_int_from_seed(&seed, 3)+3,
        wis: get_int_from_seed(&seed, 4)+3,
        cha: get_int_from_seed(&seed, 5)+3,
    };

    //add mana potion mostly for example for my future self
    Inventory::new_consumable(&mut player, &"Mana Potion".to_string());

    // ## set initial hp/mp values
    player.hp_current = player.con;
    player.hp_max = player.con;
    player.mp_current = cmp::max(player.wis, player.int);
    player.mp_max = cmp::max(player.wis, player.int);

    // ## set inventory capacity and initial items
    player.capacity_volume = player.con;
    player.capacity_weight = player.str;

    // # Respond to the Chosen.
    println!("Very well, {}!", player.name);

    return player;
}

// function to convert char from sha256 hash to int for use in player stats
fn get_int_from_seed(seed: &String, input: usize) -> i64{
    let hex_char = seed.chars().nth(input).unwrap().to_string(); // UNWRAP UNWRAP!
    let i = i64::from_str_radix(& hex_char, 16).unwrap();
    return i;
}