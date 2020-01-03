mod monsterlib;



fn main()
{

    let f: &str = "src/goblin.json";
    let goblin = monsterlib::get_monster_from_json(&f);

    for _ in 0..1
    {
        let t1 = vec![goblin.clone(), goblin.clone()];
        let t2 = vec![goblin.clone(), goblin.clone()];
        let mut arena = monsterlib::Arena::new(t1, t2, 5);
        arena.fight();
    }


}
