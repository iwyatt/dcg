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
    pub encounters: Vec<Encounter>,
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
    pub next_encounters: Vec<NextEncounter>, //surprise
                                             //retreat-able
}

#[derive(Clone)]
pub struct Puzzle {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub num_attempts: i64,
    pub solutions: Vec<PuzzleSolution>,
}

#[derive(Clone)]
pub struct PuzzleSolution {
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
    pub check_inventory: Vec<InventoryItem>,
}

#[derive(Clone, Copy)]
pub enum EncounterSuccessEffect {
    NoEffect,
    GainItem,
}

#[derive(Clone, Copy)]
pub enum EncounterFailureEffect {
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
pub struct NextEncounter {
    pub id: i64,
    pub condition: NextEncounterCondition,
    pub next_encounter_id: i64,
}

#[derive(Clone, Copy)]
pub enum NextEncounterCondition {
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
                    Encounter {
                        id: 1,
                        name: String::from("Wyatt's Footlocker"),
                        description: String::from("After dispatching the beast, you see a footlocker inside the house."),
                        actors: Vec::new(),
                        puzzles: vec![Some(Puzzle{
                                id: 0,
                                name: String::from("Locked Footlocker"),
                                description: String::from("The footlocker is locked with a built-in lock and latch. It is very secure."),
                                num_attempts: 2,
                                solutions: vec![
                                    PuzzleSolution {
                                    id: 0,
                                    name: String::from("Use a key."),
                                    description: String::from("While there is no guarantee you have the correct key, there is little harm in trying."),
                                    attempt_description: String::from("You try opening the lock with a key."),
                                    success_description: String::from("You insert a key into the lock. Although it settles into the lock with a satisfying click, it doesn't turn. Then you try turning the key counter-clock-wise and the footlocker pops open!"),
                                    failure_description: String::from("While fussing with the lock, it rattles and jingles. You wonder if you broke it, but it does not open."),
                                    check_str: 6,
                                    check_dex: 2,
                                    check_con: 6,
                                    check_int: 4,
                                    check_wis: 2,
                                    check_cha: 0,
                                    check_inventory: vec![
                                        InventoryItem{
                                            name: String::from("Wyatt's Key"),
                                            weight: 1,
                                            weapon: None,
                                            armor: None,
                                        }],
                                    },
                                    PuzzleSolution {
                                        id: 1,
                                        name: String::from("Break the footlocker open."),
                                        description: String::from("No exposed weakpoint on the lock. Internal hinges. Well-joined corners and flush lid. Solid oak construction. This will not be easy to break open."),
                                        attempt_description: String::from("You lift the heavy footlocker over your head and give it solid toss using body weight to add velocity. It thuds loudly and again when it hits the ground."),
                                        success_description: String::from("You retrieve and examine the footlocker. There is a thin crack with just enough separation in the wood for you to pry on. After several minutes and a finger blister, the chest pops open!"),
                                        failure_description: String::from("You retrieve and examine the footlocker. It is as solid as it ever was but is sligthly dented. You attempt to break the chest where it is dented, but without any meaningful process after several minutes."),
                                        check_str: 15,
                                        check_dex: 10,
                                        check_con: 8,
                                        check_int: 6,
                                        check_wis: 2,
                                        check_cha: 0,
                                        check_inventory: Vec::new(),
                                    }
                                    ],
                                })],
                        reward_items: Vec::new(), //TODO: add potion as reward
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
