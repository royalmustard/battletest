mod monsterlib;
use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();


    if args.len() >= 3
    {
        let thread_type = &args[1];
        let iterations = args[2].parse::<usize>().unwrap();
        let mut threads: usize = 0;
        if args.len() >= 4
        {
            threads = args[3].parse::<usize>().unwrap();
        }
        let f: &str = "/home/synapsis/Desktop/Projects/battletest/src/goblin.json";
        let goblin = monsterlib::get_monster_from_json(&f);
        let t1 = vec![goblin.clone(), goblin.clone()];
        let t2 = vec![goblin.clone(), goblin.clone()];

        match thread_type.as_str()
        {
            "single" => {monsterlib::fight_multithreaded(t1, t2, iterations as u32, 1)},
            "multi" =>   monsterlib::fight_multithreaded(t1, t2, iterations as u32, threads),
            _ => return
        }
    }







}
