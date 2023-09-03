// # DCG - "Dumb Computer Game"

// # TODO:
// ## FEATURES
// - implement basic inventory equipment and consumables
// - implement buff stack - includes equipment, passive, and duration effects
// - implement encounter actions (eg design parlay and spell systems) for both player and npc
//  - implement initiative roll for each round (?)
// - implement spell system
// - implement deity system
// - implement loop to listen for keyboard input rather than waiting for enter key
// - implement quest system (objectives, conditions, constraints)
// - implement npc ai scripts (combat, parlay, quest givers, merchants)
// - implement screen drawing / re-drawing
//   - consider using the color crate to stylize text: https://docs.rs/colored/latest/colored/
//   - consider ratatui crate for nice looking TUI and widgets
// - implement markdown writer/exporter to create story/world/journal of character
// - implement LLM generated content

// ## Optimization
// - move player creation from main() to its own section
// - ? move player and npc into encounter object, and pass only the encounter object for player actions
// - ? attach functions to inventory objects instead of being separate
// - ? consider moving object and function definitons to separate module(s)

// # import libraries
use std::char;
use std::cmp;
use std::io;

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
    let mut player = actors::create_actor(name);
    println!("Very well, {}!", player.name);

    // # print character attributes
    clear_screen(&player);

    // # NPC Encounter
    let mut kills = 0;
    while player.hp_current.base_value > 0 {
        println!("KILLS: {}", kills.to_string());
        npc_encounter(&mut player);
        kills = kills + 1;
    }
}

// # NPC Encounter
fn npc_encounter(player: &mut Actor) {
    // Initialize The Fight

    // ## create an enemy to fight

    // TODO: temporary list of strings - will move to list_actors.json eventually
    let enemies = vec![
        "Scary Monster",
        "Dung Toothed Pup",
        "The Pulverizer",
        "Captain Kickass",
        "Super Fred",
        "Gerbil Slayer",
        "Molotov Jester",
    ];
    let enemy_index = rand::thread_rng().gen_range(0..=enemies.len()); //update RNG range as I add functionality
                                                                       //

    let mut npc = actors::create_actor(enemies[enemy_index].to_string());

    let mut npc_encounter = NpcEncounter {
        turns: 0,
        result: EncounterResult::NoResult,
    };

    // ## announce the npc!
    println!("An {} has appeared!", npc.name);

    // ## begin the encounter loop
    while matches!(&mut npc_encounter.result, EncounterResult::NoResult) {
        // ### player turn
        player_turn(player, &mut npc);

        // check if encounter result conditions have been met
        if player.hp_current.base_value <= 0 {
            npc_encounter.result = EncounterResult::PlayerDeath;
            break;
        };

        if npc.hp_current.base_value <= 0 {
            npc_encounter.result = EncounterResult::PlayerVictory;
            break;
        };

        // incrment encounter turns
        npc_encounter.turns = npc_encounter.turns + 1;

        // ### npc turn
        npc_turn(player, &mut npc);

        // check if encounter result conditions have been met
        if player.hp_current.base_value <= 0 {
            npc_encounter.result = EncounterResult::PlayerDeath;
            break;
        };

        if npc.hp_current.base_value <= 0 {
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
    resolve_encounter(player, &mut npc, &mut npc_encounter);
}

fn clear_screen(player: &Actor) {
    println!("");
    println!("------------------------");
    println!(
        "HP: {} / {} | MP: {} / {}",
        player.hp_current.base_value,
        player.hp_max.base_value,
        player.mp_current.base_value,
        player.mp_max.base_value
    );
    println!(
        "Inventory Space: (current) / {} | Encumberence : (current) / {}",
        player.encumberence_current.base_value, player.encumberence_max.base_value
    );
    println!(
        "STR: {} | DEX: {} | CON: {} | INT: {} | WIS: {} | CHA: {}",
        player.str.base_value,
        player.dex.base_value,
        player.con.base_value,
        player.int.base_value,
        player.wis.base_value,
        player.cha.base_value
    );
    println!("------------------------");

    println!("~ ~ ~ ~INVENTORY~ ~ ~ ~");
    if player.inventory.items.len() <= 0 {
        println!(" NO INVENTORY !");
    } else {
        for i in player.inventory.items.iter() {
            println!("{}", i.name);
        }
        println!("~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~");
    }
}

fn resolve_encounter(player: &mut Actor, npc: &mut Actor, npc_encounter: &mut NpcEncounter) {
    if matches!(npc_encounter.result, EncounterResult::PlayerVictory) {
        println!("{} has vanquished the {}!", player.name, npc.name);
    };

    if matches!(npc_encounter.result, EncounterResult::PlayerDeath) {
        println!("{} was felled by {}!", player.name, npc.name);
    };
}

// ## NPC Turn
fn npc_turn(player: &mut Actor, npc: &mut Actor) {
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
        "0" | _ => println!("{} does nothing.", npc.name),
    }
}

fn npc_action_attack_melee(player: &mut Actor, npc: &mut Actor) {
    println!("{} attacks {} with melee!", npc.name, player.name);
    player.hp_current.base_value = player.hp_current.base_value - npc.str.base_value;
}

fn npc_action_attack_ranged(player: &mut Actor, npc: &mut Actor) {
    println!("{} attacks {} with ranged!", npc.name, player.name);
    player.hp_current.base_value = player.hp_current.base_value - npc.dex.base_value;
}

// ## Player Turn
fn player_turn(player: &mut Actor, npc: &mut Actor) {
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
fn player_action_nothing(player: &Actor, npc: &Actor) {
    println!("{} does nothing.", player.name.as_str());
}

// player input action: talk to NPC
fn player_action_talk(player: &Actor, npc: &Actor) {
    // TODO: NPC CHA check
}

// player input action: defend
fn player_action_defend(player: &Actor, npc: &Actor) {
    // TODO: player action defend (+1 all attriutes for 1 round)
}

// player input action: attack w/ melee weapon
fn player_action_attack_melee(player: &Actor, npc: &mut Actor) {
    println!("{} attacks {} with melee!", player.name.trim(), npc.name);
    npc.hp_current.base_value = npc.hp_current.base_value - player.str.base_value;
}

// player input acton: ranged attack
fn player_action_attack_ranged(player: &mut Actor, npc: &mut Actor) {
    println!("{} attacks {} with ranged!", player.name, npc.name);
    npc.hp_current.base_value = npc.hp_current.base_value - player.dex.base_value;
}

// player input action: chant / pray
fn player_action_chant(player: &mut Actor, npc: &mut Actor) {
    println!("{} chants <deity>'s mantra", player.name);

    let mantra_chance = rand::thread_rng().gen_range(1..=18); //TODO: plus bonus for mantra book equipped

    // TODO: mantra effects and mana cost
    if mantra_chance <= (player.wis.base_value + player.int.base_value) / 2 {
        match mantra_chance {
            20 => {
                println!("BY FIRE BE PURGED!");
                npc.hp_current.base_value = 0
            }
            19 => {
                println!("BE BORN AGAIN!");
                player.hp_current.base_value = player.hp_max.base_value;
                player.mp_current.base_value = player.mp_max.base_value;
            }
            18 => {
                println!("I SHALL SMITE THINE ENEMIES!");
                npc.hp_current.base_value = npc.hp_current.base_value / 2
            }
            17 => {
                println!("BE HEALED!");
                player.hp_current.base_value = cmp::min(
                    player.hp_max.base_value,
                    player.hp_current.base_value + (player.hp_max.base_value / 2),
                )
            }
            16 => {
                println!("BE REJUVINATED!");
                player.mp_current.base_value = player.mp_max.base_value
            }
            15 => {
                println!("BE AT PEACE!");
                player.mp_current.base_value = cmp::min(
                    player.mp_max.base_value,
                    player.mp_current.base_value + (player.mp_max.base_value / 2),
                )
            }
            //6 - 14 - do other stuff / buffs TODO: Implement after buff system implemented
            5 => println!("You have a renewed sense of confdence,"),
            4 => println!("You have a sense of peace"),
            3 => println!("You have a sense of calm."),
            2 => println!("You complete the recitation of the mantra."),
            1 => println!("You feel a sense of existential angst."),
            _ => println!("stuff happens"),
        }
    } else {
        println!("Mantra was interrupted.");
    }
}

// Define NPC_Encounter
struct NpcEncounter {
    turns: u32,
    result: EncounterResult,
}

enum EncounterResult {
    PlayerVictory,
    PlayerDeath,
    PlayerFlee,
    NPCFlee,
    NoResult,
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
