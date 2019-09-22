use d20;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::path::Path;
use rand::seq::SliceRandom;


#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Copy, Clone, Debug)]
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


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Monster
{
    pub name: String,
    pub ac: u8,
    pub hp: i32,
    pub max_hp: u32,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attack
{
    pub name: String,
    pub damage: String,
    pub bonus: u8,
    pub dtype: Damage_Type
}

impl Monster
{
    pub fn take_damage(&mut self, mut dmg: i32,tp: &Damage_Type)
    {
        if self.resist.contains(&tp)
        {
            dmg /= 2;
        }
        if self.hp - dmg <= 0
        {
            self.dead = true
        }
        else
        {
            self.hp -= dmg
        }
    }

    pub fn attack(&mut self, enemies: &mut Vec<&mut Monster>)
    {
        for e in enemies
        {
            if !e.dead
            {
                let atk = self.attacks.choose(&mut rand::thread_rng()).unwrap();
                if roll(&format!("d20 + {}", atk.bonus)) as u8 >= e.ac
                {
                    e.take_damage(d20::roll_dice(&atk.damage).unwrap().total, &atk.dtype);
                }
                break;
            }
        }
    }

    pub fn roll_init(&mut self)
    {
        self.initiative = roll(&format!("d20 + {}", self.mods[1])) as u8;
    }
}

#[derive(Debug)]
pub struct Team
{
    pub mobs: Vec<Monster>
}

impl Team
{
    pub fn is_defeated(&mut self) -> bool
    {
        for m in &self.mobs
        {
            if !m.dead
            {
                return false;
            }
        }
        true
    }
}

pub struct Arena<'a>
{
    pub Team1: Team,
    pub Team2: Team,
    mbi: Vec<&'a mut Monster>,
    pub iterations: u32,
}

impl Arena<'_>
{
    pub fn fight(&mut self)
    {
        let mut wins: [u32; 2] = [0,0];
        for _i in 0..self.iterations
        {
            //Check for Winner
            if self.Team1.is_defeated()
            {
                wins[1] +=1;
                println!("2 wins");
                self.reset_teams();
            }
            else if self.Team2.is_defeated()
            {
                wins[0] += 1;
                println!("1 wins");
                self.reset_teams();
            }
            //FIGHT!
            for m in &mut self.mbi
            {
                let mut target_team: u8 = 1;
                if m.team == 1
                {
                    target_team = 2;
                }
            }
        }
        self.eval(&wins)
    }

    fn begin<'a>(&'a mut self)
    {
        for m1 in &mut self.Team1.mobs
        {
            m1.roll_init();
            m1.team=1;
        }
        for m2 in &mut self.Team2.mobs
        {
            m2.roll_init();
            m2.team = 2;
        }
        println!("{:?}", self.Team1);
        self.mbi.append(&mut self.Team1.mobs.iter_mut().collect::<Vec<&mut Monster>>());
        self.mbi.append(&mut self.Team2.mobs.iter_mut().collect::<Vec<&mut Monster>>());
        self.mbi.sort_by(|a, b| a.initiative.cmp(&b.initiative))
    }

    fn eval(&mut self, wins: &[u32; 2])
    {
        println!("Team 1 wins: {}", wins[0]);
        println!("Team 2 wins: {}", wins[1]);
    }

    fn reset_teams(&mut self)
    {
        for m in self.Team1.mobs
        {
            m.hp = m.max_hp as i32;
        }
        for m in self.Team2.mobs
        {
            m.hp = m.max_hp as i32;
        }
    }
}

pub struct FightResult
{

}

pub fn roll(rl: &String) -> i32
{
    let r = d20::roll_dice(rl).unwrap();
    r.total
}

pub fn get_monster_from_json(path: &str) -> Monster
{
    let json_file_path = Path::new(path);
    let json_file = File::open(json_file_path).expect("Monsterfile not found");
    let mon: Monster = serde_json::from_reader(json_file).expect("error while reading json");
    mon
}
