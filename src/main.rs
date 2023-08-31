// # DCG - "Dumb Computer Game"

// # TODO:
// ## FEATURES
// - implement basic inventory equipment and consumables
// - implement buff stack - includes equipment, passive, and duration effects
// - implement encounter actions (eg design parlay and spell systems) for both player and npc
// - implement spell system
// - implement deity system
// - implement loop to listen for keyboard input rather than waiting for enter key
// - implement quest system (objectives, conditions, constraints)
// - implement npc ai scripts (combat, parlay, quest givers, merchants)
// - implement screen drawing / re-drawing
//   - consider using the color crate to stylize text: https://docs.rs/colored/latest/colored/
// - implement markdown writer/exporter to create story/world/journal of character
// - implement LLM generated content

// ## Optimization
// - move player creation from main() to its own section
// - ? move player and npc into encounter object, and pass only the encounter object for player actions
// - ? attach functions to inventory objects instead of being separate
// - ? consider moving object and function definitons to separate module(s)

// # import libraries
use std::char;
use std::io;
use std::cmp;

// NOTE: Uses sverd library whose authors want to only distribute pre-compiled binaries
use rand::Rng;
use sha256::digest;

//load dcg modules
mod inventory;
use inventory::*;

mod actors;
use actors::*;

// program start
fn main() {

     // # clear the terminal but maybe only in linux
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    //create player
    // # Player Creation
    println!("Enter the name of the Chosen.");

    // ## Input Name
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    name = String::from(name.trim());
    let mut player = actors::create_player(name);
    
    // # print character attributes
    clear_screen(&player);

    // # NPC Encounter
    let mut kills = 0;
    while player.hp_current > 0 {
        println!("KILLS: {}",kills.to_string());
        npc_encounter(&mut player);
        kills = kills + 1;
    }
}

// # NPC Encounter
fn npc_encounter(player: &mut Character){
    // Initialize The Fight

    // ## create an enemy to fight
    let mut npc = Character{
        name: String::from("Big Bad Scary Monster"),
        char_type: CharType::Npc,
        hp_current: 1,
        hp_max: 1,
        mp_current: 1,
        mp_max: 1,
        str: rand::thread_rng().gen_range(1..=16) +2,
        dex: rand::thread_rng().gen_range(1..=16) +2,
        con: rand::thread_rng().gen_range(1..=16) +2,
        int: rand::thread_rng().gen_range(1..=16) +2,
        wis: rand::thread_rng().gen_range(1..=16) +2,
        cha: rand::thread_rng().gen_range(1..=16) +2,
        capacity_volume: 1,
        capacity_weight: 1,
        inventory: Inventory {items: vec![InventoryItem { name: String::from("Health Potion"), weight: 1, weapon: None, armor: None }]}
    };

    // ### set initial hp/mp values
    npc.hp_current = player.con;
    npc.hp_max = player.con;
    npc.mp_current = cmp::max(npc.wis, npc.int);
    npc.mp_max = cmp::max(npc.wis, npc.int);

    let mut npc_encounter = NpcEncounter{
        turns: 0,
        result: EncounterResult::NoResult,
    };

    // ## announce the npc!
    println!("An {} has appeared!",npc.name);

    // ## begin the encounter loop
    while matches!(&mut npc_encounter.result,EncounterResult::NoResult){

        // ### player turn
        player_turn(player, &mut npc);
            
        // check if encounter result conditions have been met
        if player.hp_current <= 0 {
            npc_encounter.result = EncounterResult::PlayerDeath;
            break;
        };

        if npc.hp_current <= 0 {
            npc_encounter.result = EncounterResult::PlayerVictory;
            break;
        };

        // incrment encounter turns
        npc_encounter.turns = npc_encounter.turns + 1;

        // ### npc turn
        npc_turn(player, &mut npc);

        // check if encounter result conditions have been met
        if player.hp_current <= 0 {
            npc_encounter.result = EncounterResult::PlayerDeath;
            break;
        };

        if npc.hp_current <= 0 {
            npc_encounter.result = EncounterResult::PlayerVictory;
            break;
        };

        // incrment encounter turns
        npc_encounter.turns = npc_encounter.turns + 1;

        //clear screen
        clear_screen(player);
    }

    // ## conclude the encounter
    // TODO: add xp, gold, inventory, congratulate the player, level up, etc)
    resolve_encounter(player,&mut npc,&mut npc_encounter);
}

fn clear_screen(player: &Character){
	println!("");
    println!("------------------------");
    println!("HP: {} / {} | MP: {} / {}", player.hp_current, player.hp_max, player.mp_current, player.mp_max);
    println!("Inventory Space: (current) / {} | Encumberence : (current) / {}", player.capacity_volume, player.capacity_weight);
    println!("STR: {} | DEX: {} | CON: {} | INT: {} | WIS: {} | CHA: {}", player.str, player.dex, player.con, player.int, player.wis, player.cha);
    println!("------------------------");

    println!("~ ~ ~ ~INVENTORY~ ~ ~ ~");
    if player.inventory.items.len() <= 0 {
        println!(" NO INVENTORY !");
    }

    else {
        for i in player.inventory.items.iter(){
            println!("{}",i.name);
        };
        println!("~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~");
    }
}

fn resolve_encounter(player: &mut Character, npc: &mut Character, npc_encounter: &mut NpcEncounter){
    if matches!(npc_encounter.result, EncounterResult::PlayerVictory){
        println!("{} has vanquished the {}!",player.name, npc.name);
    };

    if matches!(npc_encounter.result, EncounterResult::PlayerDeath){
        println!("{} was felled by {}!",player.name, npc.name);
    };
}

// ## NPC Turn
fn npc_turn(player: &mut Character, npc: &mut Character) {
    // use simple random number to determine npc action
    // maybe swap this out for more intelligent ai at some point
    let npc_action = rand::thread_rng().gen_range(4..=7); //update RNG range as I add functionality
    match npc_action.to_string().as_str() {
        //"1" => println!("npc defends!"),
        //"2" => println!("npc requests to parlay"),
        //"3" => println!("npc attempts to evade!"),
        "4" => npc_action_attack_melee(player, npc),
        "5" => npc_action_attack_ranged(player, npc),
        "6" => player_action_chant(npc, player),
        //"7" => 
        "7" | "z" => Inventory::use_consumable(npc, &String::from("Health Potion")),
        "8" | "c" => Inventory::use_consumable(npc, &String::from("Mana Potion")),
        "0" | _ => println!("{} does nothing.",npc.name),
    }
}

fn npc_action_attack_melee(player: &mut Character, npc: &mut Character){
    println!("{} attacks {} with melee!", npc.name, player.name);
    player.hp_current = player.hp_current - npc.str;
}

fn npc_action_attack_ranged(player: &mut Character, npc: &mut Character){
    println!("{} attacks {} with ranged!", npc.name, player.name);
    player.hp_current = player.hp_current - npc.dex;
}

// ## Player Turn
fn player_turn(player: &mut Character, npc: &mut Character) {
    // input player action
    let mut player_input = String::new();
    
    // describe available actions - TODO: Needs updated as we implement more functions
    println!("Command?");
    println!("q - melee attack | e - ranged attack");
    println!("z - use health pot | c - use mana pot");
	println!("r - chant mantra");
	
    // get input
    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read line");

    let player_input: String = match player_input.trim().parse() {
        Ok(num) => num,
        Err(_) => String::from("error"),
    };
    
    // process player input
    // "1" | "x" defend | constitution
    // "2" | "p" parlay
    // "3" | "v" evade
    // "4" | "q" attack melee
    // "5" | "e" attack ranged
    // "6" | "r" chant mantra
    // "7" | "z" quaff healing potion
    // "8" | "c" quaff mana potion
    // "9" | "f" use item
    // "0" | "n" nothing / wait

    match player_input.as_str() {
        "0" | "n" => player_action_nothing(&player, &npc),
        "1" | "d" => player_action_defend(&player, &npc),
        "2" | "p" => player_action_talk(&player, &npc),
        "4" | "q" => player_action_attack_melee(player, npc),
        "5" | "e" => player_action_attack_ranged(player, npc),
        "6" | "r" => player_action_chant(player, npc),
        "7" | "z" => Inventory::use_consumable(player, &String::from("Health Potion")),
        "8" | "c" => Inventory::use_consumable(player, &String::from("Mana Potion")),
        _ => println!("3"),
    }
}

// player input action: do nothing
fn player_action_nothing(player: &Character, npc: &Character){
    println!("{} does nothing.", player.name.as_str());
}

// player input action: talk to NPC
fn player_action_talk(player: &Character, npc: &Character){
    // TODO: NPC CHA check
}

// player input action: defend
fn player_action_defend(player: &Character, npc: &Character){
    // TODO: player action defend (+1 all attriutes for 1 round)
}

// player input action: attack w/ melee weapon
fn player_action_attack_melee(player: &Character, npc: &mut Character){
    println!("{} attacks {} with melee!", player.name.trim(), npc.name);
    npc.hp_current = npc.hp_current - player.str;
}

// player input acton: ranged attack
fn player_action_attack_ranged(player: &mut Character, npc: &mut Character){
    println!("{} attacks {} with ranged!", player.name, npc.name);
    npc.hp_current = npc.hp_current - player.dex;
}

// player input action: chant / pray
fn player_action_chant(player: &mut Character, npc: &mut Character){
    
    println!("{} chants <deity>'s mantra", player.name);

    let mantra_chance = rand::thread_rng().gen_range(1..=18); //TODO: plus bonus for mantra book equipped

    // TODO: mantra effects and mana cost
    if mantra_chance <= (player.wis + player.int) / 2 {
        match mantra_chance{
            20 => {println!("BY FIRE BE PURGED!"); npc.hp_current = 0},
            19 => {println!("BE BORN AGAIN!"); player.hp_current = player.hp_max; player.mp_current = player.mp_max;},
            18 => {println!("I SHALL SMITE THINE ENEMIES!"); npc.hp_current = npc.hp_current / 2},
            17 => {println!("BE HEALED!"); player.hp_current = cmp::min(player.hp_max,player.hp_current + (player.hp_max / 2))},
            16 => {println!("BE REJUVINATED!"); player.mp_current = player.mp_max},
            15 => {println!("BE AT PEACE!"); player.mp_current = cmp::min(player.mp_max, player.mp_current + (player.mp_max /2 ))},
            //6 - 14 - do other stuff / buffs TODO: Implement after buff system implemented
            5 => println!("You have a renewed sense of confdence,"),
            4 => println!("You have a sense of peace"),
            3 => println!("You have a sense of calm."),
            2 => println!("You complete the recitation of the mantra."),
            1 => println!("You feel a sense of existential angst."),
            _ => println!("stuff happens")
        }
    }
    else {
        println!("Mantra was interrupted.");
    }
}


// Define NPC_Encounter
struct NpcEncounter{
    turns: u32,
    result: EncounterResult
}

enum EncounterResult{
    PlayerVictory,
    PlayerDeath,
    PlayerFlee,
    NPCFlee,
    NoResult
}


// Gods: Anya [AHN-yuh]
// Bane [BAYN]
// Callia [KAH-lee-uh]
// Dax [DAX]
// Elora [EH-loh-ruh]
// Freya [FREE-uh]
// Ira [EYE-ruh]
// Kai [KY]
// Lyra [LY-ruh]
// Mira [MEE-ruh]
// Nova [NOH-vuh]
// Pax [PAX]
// Zara [ZAH-ruh]
