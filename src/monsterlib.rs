use d20;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::path::Path;


#[derive(Serialize, Deserialize)]
pub enum Damage_Type
{
    ACID,
    BLUDGEOONING,
    COLD,
    FIRE,
    FORCE,
    LIGHTNING,
    NECROTIC,
    PIERCING,
    POISON,
    PSYCHIC,
    RADIANT,
    SLASHING,
    THUNDER
}


#[derive(Serialize, Deserialize)]
pub struct Monster
{
    pub name: String,
    pub ac: u8,
    pub hp: i32,
    pub stats: [u8; 6],
    pub mods: [i8; 6],
    pub attacks: Vec<Attack>,
    pub cr: f32,
    pub resist: Vec<Damage_Type>,
    pub weakness: Vec<Damage_Type>,
    pub dead: bool,
    pub initiative: u8,
    pub team: u8
}

#[derive(Serialize, Deserialize)]
pub struct Attack
{
    pub name: String,
    pub damage: String,
    pub bonus: u8,
    pub dtype: Damage_Type
}


pub struct Arena
{

}

pub struct FightResult
{

}

pub fn roll(rl: &String) -> i32
{
    let mut r = d20::roll_dice(rl).unwrap();
    r.total
}

pub fn get_monster_from_json(path: &str) -> Monster
{
    let json_file_path = Path::new(path);
    let json_file = File::open(json_file_path).expect("Monsterfile not found");
    let mon: Monster = serde_json::from_reader(json_file).expect("error while reading json");
    mon
}
