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

    let result100 = map.pp()
        .accuracy(100.0)
        .calculate(); 

    let result98 = map.pp()
        .accuracy(98.0)
        .calculate(); 

    let result95 = map.pp()
        .accuracy(95.0)
        .calculate(); 

    println!("{}", result100.pp());
    println!("{}", result98.pp());
    println!("{}", result95.pp());
}