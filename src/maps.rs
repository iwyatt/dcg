use crate::actors::*;
use crate::inventory::*;

#[derive(Clone)]
pub struct Campaign {
    pub id: i64,
    pub name: String,
    pub hash: String,
    pub quests: Vec<Quest>,
    pub maps: Vec<Map>,
}

#[derive(Clone)]
pub struct Quest {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct Map {
    pub id: i64,
    pub name: String,
    pub locations: Vec<Location>,
}

#[derive(Clone)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub encounters: Vec<Encounter>
}

#[derive(Clone)]
pub struct Encounter {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub actors: Vec<Option<Actor>>,
    pub puzzles: Vec<Option<Puzzle>>,
    pub reward_items: Vec<Option<InventoryItem>>,
    pub reward_xp: i64,
    pub reward_gold: i64,
    pub failure_effect: EncounterFailureEffect,
    pub success_effect: EncounterSuccessEffect,
    pub next_encounters: Vec<NextEncounter>
    //surprise
    //retreat-able
}

#[derive(Clone)]
pub struct Puzzle {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub solutions: Vec<PuzzleSolution>,
    pub num_attempts: i64,
}

#[derive(Clone)]
pub struct PuzzleSolution
{
    pub id: i64,
    pub name: String,
    pub description: String,
    pub attempt_description: String,
    pub success_description: String,
    pub failure_description: String,
    pub check_str: i64,
    pub check_dex: i64,
    pub check_con: i64,
    pub check_int: i64,
    pub check_wis: i64,
    pub check_cha: i64,
    pub check_inventory: Vec<InventoryItem>
}

#[derive(Clone, Copy)]
pub enum EncounterSuccessEffect{
    NoEffect,
}

#[derive(Clone, Copy)]
pub enum EncounterFailureEffect{
    LoseHP,
    LoseMP,
    LoseXP,
    LoseGold,
    LoseItem,
    LoseEquipment,
    ApplyBadBuff,
    NoEffect,
}

#[derive(Clone, Copy)]
pub struct NextEncounter{
    pub id: i64,
    pub condition: NextEncounterCondition,
    pub next_encounter_id: i64,
}

#[derive(Clone, Copy)]
pub enum NextEncounterCondition{
    EncounterSuccess,
    EncounterFailed,
    PuzzleSuccess,
    PuzzleFailed,
    CombatSuccess,
    ActorBarterSuccess,
    ActorBarterFail,
    FindTrapsSuccess,
    FindTrapsFail,
    FindTrapsSprung,
}

pub fn create_tutorial_campaign() -> Campaign {
    let campaign = Campaign{
        id: 0,
        name: String::from("Tutorial Campaign"),
        hash: String::from(""),
        quests: vec![ Quest {id: 0, name: String::from("Rat Bastard")}],
        maps: vec![ Map {
            id: 0,
            name: String::from("Wyatt's End Village"),
            locations: vec![ Location {
                id: 0,
                name: String::from("Wyatt's House"),
                encounters: vec![
                    Encounter {
                        id: 0,
                        name: String::from("Wyatt's House"),
                        description: String::from("Outside of Wyatt's house is a giant rat!"),
                        actors: vec![Some(create_actor(String::from("Giant Rat")))],
                        puzzles: Vec::new(),
                        reward_items: Vec::new(),
                        reward_xp: 10,
                        reward_gold: 0,
                        failure_effect: EncounterFailureEffect::NoEffect,
                        success_effect: EncounterSuccessEffect::NoEffect,
                        next_encounters: vec![
                            NextEncounter{
                                id: 0,
                                condition: NextEncounterCondition::EncounterSuccess,
                                next_encounter_id: 1
                            }
                        ],  
                    },
                    Encounter {
                        id: 1,
                        name: String::from("Inside Wyatt's House"),
                        description: String::from("After dispatching the beast, you enter Wyatt's house. There you find yet another foul manifestation to dispatch!"),
                        actors: vec![Some(create_actor(String::from("Foul Manifestation")))],
                        puzzles: Vec::new(),
                        reward_items: Vec::new(),
                        reward_xp: 10,
                        reward_gold: 0,
                        failure_effect: EncounterFailureEffect::NoEffect,
                        success_effect: EncounterSuccessEffect::NoEffect,
                        next_encounters: vec![
                            NextEncounter{
                                id: 0,
                                condition: NextEncounterCondition::EncounterSuccess,
                                next_encounter_id: 2
                            }
                        ],
                    },
                    ]
        }],
        }],
    };
    return campaign;
}