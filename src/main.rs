mod monsterlib;



fn main()
{
    println!("Hello, world!");
    let f: &str = "src/goblin.json";
    let goblin = monsterlib::get_monster_from_json(&f);
    let t1 = vec![goblin.clone(), goblin.clone()];
    let t2 = vec![goblin.clone(), goblin.clone()];
    let mut arena = monsterlib::Arena::new(t1, t2, 1000);
    arena.fight();
}
