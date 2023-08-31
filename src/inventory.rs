use crate::actors::*;
use std::cmp;

#[derive(PartialEq,Debug)]
pub struct Inventory{
    pub items: Vec<InventoryItem>
}

#[derive(PartialEq,Debug)]
pub struct InventoryItem {
    pub name : String,
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

#[derive(PartialEq,Debug)]
pub struct Weapon {
    pub enagement_type: EngagementType,
    pub str_factor: f64
}

#[derive(PartialEq,Debug)]
pub enum EngagementType{
    Melee,
    Ranged, 
    Chant 
}

#[derive(PartialEq,Debug)]
pub struct Armor{
    pub name: String,
    pub base: i64,
    pub str_bonus: f64
}

pub trait Use{
    fn use_consumable(character: & mut Character, item_name: & String){}
    fn new_consumable(character: & mut Character, item_name: & String){}
}

impl Use for Inventory {
    fn new_consumable(character: & mut Character, item_name: & String){
        //check if encumberence too high

        match item_name.to_string().as_str(){
            "Health Potion" => {
                let new_item = InventoryItem { name: String::from("Health Potion"), weight: 1, weapon: None, armor: None };
                character.inventory.items.push(new_item);
            },

            "Mana Potion" => {
                let new_item = InventoryItem { name: String::from("Mana Potion"), weight: 1, weapon: None, armor: None };
                character.inventory.items.push(new_item);
            },
            _ => println!("No Item Created")
        }
    }

    fn use_consumable(character: & mut Character, item_name: & String){
        // get item index
        //println!("{:?}", item_name);
        //println!("{:?}", character.inventory.items.iter());
        let item_index = character.inventory.items.iter().position(|item| String::from(&item.name) == String::from(item_name));

        if item_index == None{
            println!("No {} in inventory!", item_name);    
            return;
        }
        
        else {
            
            // remove from inventory
            println!("{}: *GULP!*",character.name);
            character.inventory.items.remove(item_index.unwrap());
        }

        // do the effect
        match item_name.to_string().as_str() {
            "Health Potion" => {character.hp_current = cmp::min(character.hp_current + ((character.str + character.con)/2), character.hp_max)},
            "Mana Potion" => {character.mp_current = cmp::min(character.mp_max, character.mp_current + (character.mp_max /2 ))},
            _ => println!(""),
        };
    }
}