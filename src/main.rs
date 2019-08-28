mod monsterlib;



fn main()
{
    println!("Hello, world!");
    let f: &str = "src\\goblin.json";
    println!("{}", &f);
    let goblin = monsterlib::get_monster_from_json(&f);
    monsterlib::roll(&String::from("1d6+2"));
}
