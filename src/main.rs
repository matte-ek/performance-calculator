use std::env;
use rosu_pp::{Beatmap, BeatmapExt};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("err: no map argument");
        return;
    }

    let file = format!("cache/{}.osu", args[1]);

    let map = match Beatmap::from_path(file) {
        Ok(map) => map,
        Err(_) => return println!("err: failed to load map."),
    };

    if args.len() <= 3 {
        let mods: u32 = if args.len() == 3 {
            args[2].parse().unwrap()
        } else {
            0
        };

        let result100 = map.pp()
            .accuracy(100.0)
            .mods(mods)
            .calculate(); 

        let result98 = map.pp()
            .accuracy(98.0)
            .mods(mods)
            .calculate(); 

        let result95 = map.pp()
            .accuracy(95.0)
            .mods(mods)
            .calculate(); 

        println!("{}", result100.pp());
        println!("{}", result98.pp());
        println!("{}", result95.pp());
    }
    else if args.len() <= 8 { // <map-id> <n300> <n100> <n50> <nmiss> <max-combo> <[mods]>
        let mods: u32 = if args.len() == 8 {
            args[7].parse().unwrap()
        } else {
            0
        };

        let result = map.pp()
            .n300(args[2].parse().unwrap())
            .n100(args[3].parse().unwrap())
            .n50(args[4].parse().unwrap())
            .n_misses(args[5].parse().unwrap())
            .combo(args[6].parse().unwrap())
            .mods(mods)
            .calculate();

        let result_max = map.pp()
            .n300(args[2].parse().unwrap())
            .n100(args[3].parse().unwrap())
            .n50(args[4].parse().unwrap())
            .n_misses(0)
            .mods(mods)
            .calculate();

        println!("{}", result.pp());
        println!("{}", result_max.pp());
    }
}