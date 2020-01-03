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

pub struct AttackResult
{
    pub to_hit: u8,
    pub dtype: DamageType,
    pub damage: i32
}

impl Monster
{
    pub fn take_attack(&mut self, ar: AttackResult)
    {
        if ar.to_hit >= self.ac
        {
            self.take_damage(ar.damage, &ar.dtype)
        }
    }

    fn take_damage(&mut self, mut dmg: i32,tp: &DamageType)
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

    pub fn attack(&mut self, adv: i8) -> AttackResult
    {
        let atk = self.attacks.choose(&mut rand::thread_rng()).unwrap();
        //Check Advantage
        let droll = match adv{
            1 => *vec![roll(&format!("1d20")), roll(&format!("1d20"))].iter().max().unwrap(),
            -1 => *vec![roll(&format!("1d20")), roll(&format!("1d20"))].iter().min().unwrap(),
            _ => roll(&format!("1d20"))
        };
        //Check Crit
        let damage : i32 = match droll{
            20 => roll(&atk.damage) + roll(&atk.damage),
            _ => roll(&atk.damage)
        };
        AttackResult
        {
            to_hit: droll as u8+atk.bonus,
            damage: damage,
            dtype: atk.dtype
        }
    }

    pub fn roll_init(&mut self)
    {
        self.initiative = roll(&format!("1d20 + {}", self.mods[1])) as u8;
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
        let a: Arena = Arena{
            mbi: vec![t1, t2].concat(),
            iterations: iterations
        };
        a
    }

    pub fn fight(&mut self)
    {
        self.begin();
        let mut wins: [u32; 2] = [0,0];
        for _i in 0..self.iterations
        {
            loop
            {
                //Check for Winner
                if self.mbi.iter().filter(|m| m.team == 1 && !m.dead).count() <1
                {
                    //Team 1 lost
                    wins[1] +=1;
                    self.reset();
                    break;
                }
                else if self.mbi.iter().filter(|m| m.team == 2 && !m.dead).count() <1
                {
                    //Team 2 lost
                    wins[0] += 1;
                    self.reset();
                    break;
                }
                //FIGHT!

                for i in 0..self.mbi.len()
                {
                    if !self.mbi[i].dead
                    {
                        let res = self.mbi[i].attack(0);
                        let team = self.mbi[i].team;
                        let e= self.mbi.iter_mut().filter(|mo| mo.team != team && !mo.dead).choose(&mut rand::thread_rng());
                        match e{
                            Some(e) => e.take_attack(res),
                            _ => break
                        }
                    }
                }
        }
        }
        self.eval(&wins)
    }

    pub fn begin(&mut self)
    {
        self.mbi.iter_mut().for_each(|m| m.roll_init());
        self.mbi.sort_by(|a, b| b.initiative.cmp(&a.initiative));
        //Dbg
        self.mbi.iter().for_each(|m| println!("{}|{}", m.team, m.initiative));
        println!("-----------------")
    }


    fn reset(&mut self)
    {
        for m in &mut self.mbi
        {
            m.hp = m.max_hp as i32;
        }
        self.begin();
    }

    fn eval(&mut self,wins: &[u32; 2])
    {
        println!("Team 1 wins: {}", wins[0]);
        println!("Team 2 wins: {}", wins[1]);
    }
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
