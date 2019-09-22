mod monsterlib;



fn main()
{
    println!("Hello, world!");
    let f: &str = "src\\goblin.json";
    let goblin = monsterlib::get_monster_from_json(&f);
    let mut t1 = monsterlib::Team
    {
        mobs: vec![goblin.clone(), goblin.clone()]
    };
    let mut t2 = monsterlib::Team
    {
        mobs: vec![goblin.clone(), goblin.clone()]
    };
    let mut arena = monsterlib::Arena
    {
        Team1: t1,
        Team2: t2,
        iterations: 1000
    };
    arena.fight();
}
