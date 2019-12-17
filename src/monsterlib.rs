use d20;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use rand::seq::SliceRandom;
use rand::seq::IteratorRandom;

#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum DamageType
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
    pub resist: Vec<DamageType>,
    pub weakness: Vec<DamageType>,
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
    pub dtype: DamageType
}

impl Monster
{
    pub fn take_damage(&mut self, mut dmg: i32,tp: &DamageType)
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

    pub fn attack(&mut self, e:&mut Monster)
    {

        let atk = self.attacks.choose(&mut rand::thread_rng()).unwrap();
        if roll(&format!("d20 + {}", atk.bonus)) as u8 >= e.ac
        {
            e.take_damage(d20::roll_dice(&atk.damage).unwrap().total, &atk.dtype);
        }
    }

    pub fn roll_init(&mut self)
    {
        self.initiative = roll(&format!("d20 + {}", self.mods[1])) as u8;
    }
}



pub struct Arena
{
    mbi: Vec<Monster>,
    pub iterations: u32,
}

impl Arena
{
    pub fn new(mut t1: Vec<Monster>, mut t2: Vec<Monster>, iterations: u32) -> Arena
    {
        t1.iter_mut().for_each(|m| m.team=1);
        t2.iter_mut().for_each(|m| m.team=2);
        let mut a: Arena = Arena{
            mbi: vec![t1, t2].concat(),
            iterations: iterations
        };
        a.begin();
        a
    }

    pub fn fight(&mut self)
    {
        self.begin();
        let mut wins: [u32; 2] = [0,0];
        for _i in 0..self.iterations
        {
            //Check for Winner
            if !(self.mbi.iter().filter(|m| m.team == 1 && !m.dead).count() > 0)
            {
                //Team 1 lost
                wins[1] +=1;
                self.reset();
            }
            else if !(self.mbi.iter().filter(|m| m.team == 2 && !m.dead).count() > 0)
            {
                //Team 2 lost
                wins[0] += 1;
                self.reset();
            }
            //FIGHT!
            for i in 0..self.mbi.len()
            {
                let team = self.mbi[i].team;
                let e:&mut Monster= &mut self.mbi.iter_mut().filter(|mo| mo.team != team).choose(&mut rand::thread_rng()).unwrap();
                self.mbi[i].attack(&mut e);
            }

        }
        self.eval(&wins)
    }

    pub fn begin(&mut self)
    {
        self.mbi.iter_mut().for_each(|m| m.roll_init());
        self.mbi.sort_by(|a, b| a.initiative.cmp(&b.initiative))
    }

    fn eval(&mut self, wins: &[u32; 2])
    {
        println!("Team 1 wins: {}", wins[0]);
        println!("Team 2 wins: {}", wins[1]);
    }

    fn reset(&mut self)
    {
        for m in &mut self.mbi
        {
            m.hp = m.max_hp as i32;
        }
        self.begin();
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
