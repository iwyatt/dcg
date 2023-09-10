// # DCG - "Dumb Computer Game"

// # TODO:
// ## FEATURES
// - implement basic inventory equipment and consumables
// - implement buff stack - includes equipment, passive, and duration effects
//      - implement equippable items on actors
//          - write the structs / create some armor by hand
//      - implement encumberence (And item capacity/volume) on actors
//      - implement post-fight loot process
// - implement serielization from json files
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
        "Prophet Gideon",
        "Maraj",
        "Jerubaal",
        "Gunther Boogle",
        "Xurich",
        "Ted Barry",
        "Richard Tooth",
    ];
    let enemy_index = rand::thread_rng().gen_range(0..enemies.len()); //update RNG range as I add functionality

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
        actors::Buff::update_buff_stack(player);
        clear_screen(player);

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
        actors::Buff::update_buff_stack(&mut npc);

        //clear screen
        clear_screen(player);
    }

    // ## conclude the encounter
    // TODO: add xp, gold, inventory, congratulate the player, level up, etc)
    resolve_encounter(player, &mut npc, &mut npc_encounter);

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    clear_screen(player);
}

fn clear_screen(player: &Actor) {
    println!("");
    println!("------------------------");
    println!(
        "HP: {} + {} / {} | MP: {} / {}",
        player.hp_current.base_value,
        player.hp_current.buff_value,
        player.hp_max.base_value,
        player.mp_current.base_value,
        player.mp_max.base_value
    );
    println!(
        "Encumberence: / {} / {}", //TODO: Implement Encumberence
        player.encumberence_current.base_value, player.encumberence_max.base_value
    );
    println!(
        "STR: {} + {} | DEX: {} + {} | CON: {} + {} | INT: {} + {} | WIS: {} + {} | CHA: {} + {}",
        player.str.base_value,
        player.str.buff_value,
        player.dex.base_value,
        player.dex.buff_value,
        player.con.base_value,
        player.con.buff_value,
        player.int.base_value,
        player.int.buff_value,
        player.wis.base_value,
        player.wis.buff_value,
        player.cha.base_value,
        player.cha.buff_value
    );
    println!("------------------------");

    println!(
        "Defense: {}",
        (player.defense.base_value + player.defense.buff_value).to_string()
    );
    //println!("Defense: {}", format!("{}", player.defense.base_value + player.defense.buff_value));

    // print inventory
    println!("~ ~ ~ ~INVENTORY~ ~ ~ ~");
    if player.inventory.len() <= 0 {
        println!(" NO INVENTORY !");
    } else {
        for i in player.inventory.iter() {
            println!("{}", i.name);
        }
        println!("~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~");
    }

    //print buffs
    println!("~ ~ ~ ~BUFFS~ ~ ~ ~");
    if player.buffs.len() <= 0 {
        println!(" NO BUFFS !");
    } else {
        for i in player.buffs.iter() {
            println!("{}", i.name);
        }
    }
    println!("~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~");
}

fn resolve_encounter(player: &mut Actor, npc: &mut Actor, npc_encounter: &mut NpcEncounter) {
    if matches!(npc_encounter.result, EncounterResult::PlayerVictory) {
        println!("{} has vanquished the {}!", player.name, npc.name);
        //TODO: Implement Looting
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
        "1" => actor_defend(npc),
        //"2" => println!("npc requests to parlay"),
        //"3" => println!("npc attempts to evade!"),
        "4" => actor_attack_target_melee(npc, player),
        "5" => actor_attack_target_ranged(npc, player),
        "6" => actor_chant(npc, player),
        //"7" =>
        "7" | "z" => Inventory::use_consumable(npc, &String::from("Health Potion")),
        "8" | "c" => Inventory::use_consumable(npc, &String::from("Mana Potion")),
        "0" | _ => println!("{} does nothing.", npc.name),
    }
}

fn actor_attack_target_melee(actor: &mut Actor, target: &mut Actor) {
    // TODO: Implement check to make sure actor has a melee weapon
    println!("{} attacks {} with melee!", actor.name, target.name);

    // 1. Add up actor attack value total
    // TODO: add in weapon value here when equipment system is implemented
    let attack_value = actor.str.base_value + actor.str.buff_value;

    // 2. get unmitigated damage by subtracting damage mitigation from the attack value
    let mut unmitigated_damage = attack_value
        - (target.defense.base_value
            + target.defense.buff_value
            + target.slot_armor.armor_value.base_value
            + target.slot_armor.armor_value.buff_value);

    // 3. assumng unmitigated damage is a negative int, apply unmitigated damage to current bonus hp, if there is any
    if unmitigated_damage > 0 && target.hp_current.buff_value > 0 {
        // set up value to subtract from hp buff
        let hp_buff_dmg = unmitigated_damage;

        // set the new value of the unmitigated dmg
        // eg reduce the quantity of unmitigated damage by the amount of current hp buff (prior to damage being applied)
        unmitigated_damage = unmitigated_damage - target.hp_current.buff_value;

        // set the new value of the current hp buff
        target.hp_current.buff_value = target.hp_current.buff_value - hp_buff_dmg;

        //carry over any remaining unmitigated damage
        if target.hp_current.buff_value < 0 {
            unmitigated_damage = target.hp_current.buff_value * -1;

            // set buff value to 0 since it is depleted
            target.hp_current.buff_value = 0;
        }
    }

    // 4. if there is still unmitigated damage remaining, subtract from current hp base
    if unmitigated_damage > 0 {
        target.hp_current.base_value = target.hp_current.base_value - unmitigated_damage;
        println!(
            "{} takes {} damage!",
            target.name,
            unmitigated_damage.to_string()
        );
    } else {
        println!("No damage! Weak!");
    }
}

fn actor_attack_target_ranged(actor: &mut Actor, target: &mut Actor) {
    // TODO: Implement check to make sure actor has a ranged weapon
    println!("{} attacks {} with ranged!", actor.name, target.name);

    // 1. Add up actor attack value total
    // TODO: add in weapon value here when equipment system is implemented
    let attack_value = actor.dex.base_value + actor.dex.buff_value;

    // 2. get unmitigated damage by subtracting damage mitigation from the attack value
    let mut unmitigated_damage = attack_value
        - (target.defense.base_value
            + target.defense.buff_value
            + target.slot_armor.armor_value.base_value
            + target.slot_armor.armor_value.buff_value);

    // 3. assumng unmitigated damage is a negative int, apply unmitigated damage to current bonus hp, if there is any
    if unmitigated_damage > 0 && target.hp_current.buff_value > 0 {
        // set up value to subtract from hp buff
        let hp_buff_dmg = unmitigated_damage;

        // set the new value of the unmitigated dmg
        // eg reduce the quantity of unmitigated damage by the amount of current hp buff (prior to damage being applied)
        unmitigated_damage = unmitigated_damage - target.hp_current.buff_value;

        // set the new value of the current hp buff
        target.hp_current.buff_value = target.hp_current.buff_value - hp_buff_dmg;

        //carry over any remaining unmitigated damage
        if target.hp_current.buff_value < 0 {
            unmitigated_damage = target.hp_current.buff_value * -1;

            // set buff value to 0 since it is depleted
            target.hp_current.buff_value = 0;
        }
    }

    // 4. if there is still unmitigated damage remaining, subtract from current hp base
    if unmitigated_damage > 0 {
        target.hp_current.base_value = target.hp_current.base_value - unmitigated_damage;
        println!(
            "{} takes {} damage!",
            target.name,
            unmitigated_damage.to_string()
        );
    } else {
        println!("No damage! Weak!");
    }
}

// fn npc_action_attack_ranged(player: &mut Actor, npc: &mut Actor) {
//     println!("{} attacks {} with ranged!", npc.name, player.name);
//     player.hp_current.base_value = player.hp_current.base_value - npc.dex.base_value;
// }

// ## Player Turn
fn player_turn(player: &mut Actor, npc: &mut Actor) {
    // input player action
    let mut player_input = String::new();

    // describe available actions - TODO: Needs updated as we implement more functions
    println!("Command?");
    println!("q - melee attack | e - ranged attack");
    println!("z - use health pot | c - use mana pot");
    println!("r - chant mantra | d - defend");

    // get input
    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read line");

    let mut player_input: String = match player_input.trim().parse() {
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
        "1" | "d" => actor_defend(player),
        "2" | "p" => player_action_talk(&player, &npc),
        "4" | "q" => actor_attack_target_melee(player, npc),
        "5" | "e" => actor_attack_target_ranged(player, npc),
        "6" | "r" => actor_chant(player, npc),
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
fn actor_defend(player: &mut Actor) {
    // TODO: player action defend (+1 all attriutes for 1 round)
    player.buffs.push(Buff {
        name: String::from("Defensive Stance"),
        duration: 2,
        mod_attribute: player.defense,
        mod_flat: 3 + player.str.base_value / 2,
        mod_scale: 0.1,
    });
}

// player input action: attack w/ melee weapon
// fn player_action_attack_melee(player: &Actor, npc: &mut Actor) {
//     println!("{} attacks {} with melee!", player.name.trim(), npc.name);
//     npc.hp_current.base_value = npc.hp_current.base_value - player.str.base_value;
// }

// player input acton: ranged attack
// fn player_action_attack_ranged(player: &mut Actor, npc: &mut Actor) {
//     println!("{} attacks {} with ranged!", player.name, npc.name);
//     npc.hp_current.base_value = npc.hp_current.base_value - player.dex.base_value;
// }

// player input action: chant / pray
// TODO: rename function and parameters to Actor/Target respectively
fn actor_chant(actor: &mut Actor, target: &mut Actor) {
    println!("{} chants <deity>'s mantra", actor.name);

    let mantra_chance =
        rand::thread_rng().gen_range(1..=18) + (actor.int.buff_value + actor.wis.buff_value); //TODO: plus bonus for mantra book equipped

    // TODO: mantra effects and mana cost
    if mantra_chance
        <= (actor.wis.base_value
            + actor.int.base_value
            + actor.wis.buff_value
            + actor.int.buff_value)
            / 2
    {
        match mantra_chance {
            20 => {
                println!("BY FIRE BE PURGED!");
                target.hp_current.base_value = 0
            }
            19 => {
                println!("BE BORN AGAIN!");
                actor.hp_current.base_value = actor.hp_max.base_value;
                actor.mp_current.base_value = actor.mp_max.base_value;
            }
            18 => {
                println!("I SHALL SMITE THINE ENEMIES!");
                target.hp_current.base_value = target.hp_current.base_value / 2
            }
            17 => {
                println!("BE HEALED!");
                actor.hp_current.base_value = cmp::min(
                    actor.hp_max.base_value,
                    actor.hp_current.base_value + (actor.hp_max.base_value / 2),
                )
            }
            16 => {
                println!("BE REJUVINATED!");
                actor.mp_current.base_value = actor.mp_max.base_value
            }
            15 => {
                println!("BE AT PEACE!");
                actor.mp_current.base_value = cmp::min(
                    actor.mp_max.base_value,
                    actor.mp_current.base_value + (actor.mp_max.base_value / 2),
                )
            }
            14 => {
                println!("Resilience!");
                actor.buffs.push(Buff {
                    name: String::from("Resilience of the Pangolin"),
                    duration: 3,
                    mod_attribute: actor.defense,
                    mod_flat: 9,
                    mod_scale: 0.1,
                })
            }
            13 => {
                println!("Zen!");
                actor.buffs.push(Buff {
                    name: String::from("Zen of Gaia"),
                    duration: 3,
                    mod_attribute: actor.mp_current,
                    mod_flat: 18,
                    mod_scale: 0.1,
                })
            }
            12 => {
                println!("Longevity!");
                actor.buffs.push(Buff {
                    name: String::from("Longevity of the Treant"),
                    duration: 3,
                    mod_attribute: actor.hp_current,
                    mod_flat: 18,
                    mod_scale: 0.1,
                })
            }
            11 => {
                println!("Charming. :)");
                actor.buffs.push(Buff {
                    name: String::from("Charisma of the Dolphin"),
                    duration: 6,
                    mod_attribute: actor.cha,
                    mod_flat: 2,
                    mod_scale: 0.1,
                })
            }
            10 => {
                println!("Smart. 8)");
                actor.buffs.push(Buff {
                    name: String::from("Intelligence of the Raven"),
                    duration: 6,
                    mod_attribute: actor.int,
                    mod_flat: 2,
                    mod_scale: 0.1,
                })
            }
            9 => {
                println!("Precocious. :P");
                actor.buffs.push(Buff {
                    name: String::from("Wisdom of the Owl"),
                    duration: 6,
                    mod_attribute: actor.wis,
                    mod_flat: 2,
                    mod_scale: 0.1,
                })
            }
            8 => {
                println!("Fit.");
                actor.buffs.push(Buff {
                    name: String::from("Constitution of the Water Bear"),
                    duration: 6,
                    mod_attribute: actor.con,
                    mod_flat: 2,
                    mod_scale: 0.1,
                })
            }
            7 => {
                println!("Agile!");
                actor.buffs.push(Buff {
                    name: String::from("Dexterity of the Tiger"),
                    duration: 6,
                    mod_attribute: actor.dex,
                    mod_flat: 2,
                    mod_scale: 0.1,
                })
            }
            6 => {
                println!("Beefy!");
                actor.buffs.push(Buff {
                    name: String::from("Strength of The Ox"),
                    duration: 6,
                    mod_attribute: actor.str,
                    mod_flat: 2,
                    mod_scale: 0.1,
                })
            }
            5 => println!("{} has a renewed sense of confdence.", actor.name),
            4 => println!("{} has a sense of peace.", actor.name),
            3 => println!("{} has a sense of calm.", actor.name),
            2 => println!("{} completes the recitation of the mantra.", actor.name),
            1 => println!("{} feels a sense of existential angst.", actor.name),
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
