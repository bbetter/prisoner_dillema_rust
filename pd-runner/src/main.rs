use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::ops::AddAssign;
use pd;
use pd::{Action, Strategy};
use pd::Action::{Betray, Cooperate};
use libloading::{Library, library_filename, Symbol};

pub fn run(
    s1: &(impl Strategy + ?Sized),
    s2: &(impl Strategy+ ?Sized),
    round_count: i32
) -> (i32,i32) {
    let s1_name = s1.name();
    let s2_name = s2.name();

    let mut strat_points_1 = 0;
    let mut strat_points_2 = 0;

    let mut rounds : Vec<(Action, Action)> = vec![];

    for i in 0..round_count {
        let action1 = s1.go(rounds.as_slice());
        let action2 = s2.go(rounds.as_slice());
        rounds.push((action1.clone(), action2.clone()));

        match (action1.clone(), action2.clone()) {
            (Cooperate, Betray) => {
                strat_points_2 += 5;
            }
            (Betray, Cooperate) => {
                strat_points_1 += 5;
            }
            (Cooperate, Cooperate) => {
                strat_points_1 += 3;
                strat_points_2 += 3;
            }
            (Betray, Betray) => {
                strat_points_1 += 1;
                strat_points_2 += 1;
            }
        }
        println!("Round#{i}: {s1_name} vs {s2_name}.");
        println!("{:?} makes a move: {:?}", s1_name, action1);
        println!("{:?} makes a move: {:?}", s2_name, action2);
        println!("=========================================")
    }

    println!("Total Score {s1_name} - {s2_name} ({strat_points_1} - {strat_points_2})");
    return (strat_points_1, strat_points_2)
}

fn main() {

    // Iterate over the directory entries and print file names
    let paths = fs::read_dir("./target/debug/").unwrap();

    let mut strategies: Vec<Box<dyn Strategy>> = vec![];
    for entry in paths {
        let file = entry.unwrap();
        let mut file_name = file.file_name().to_string_lossy().to_string();
        if !file_name.ends_with(".dylib") {
            continue
        }

        let lib_name = file_name.replacen("lib", "",1)
            .replace("-","_")
            .replace(".dylib","");

        unsafe {
            let lib =Library::new(library_filename(format!("{lib_name}"))).unwrap();
            let get_strategy: Symbol<unsafe fn() -> *mut dyn Strategy> = lib.get(b"get_strategy").unwrap();

            let strategy = get_strategy();
            let boxed_strategy = Box::from_raw(strategy);
            strategies.push(boxed_strategy);
        }

        let mut pairs = Vec::new();
        for i in 0..strategies.len() {
            for j in i + 1..strategies.len() {
                let pair = (&strategies[i], &strategies[j]);
                pairs.push(pair);
            }
        }

        let mut table: HashMap<String, i32> = HashMap::new();
        for (s1, s2) in pairs {
            let (s1_score, s2_score) = run(s1.as_ref(), s2.as_ref(), 100);
            let s1_name = s1.name().clone();
            let s2_name = s2.name().clone();

            let mut score_updates = HashMap::new();
            score_updates.entry(s1_name).or_insert(0).add_assign(s1_score);
            score_updates.entry(s2_name).or_insert(0).add_assign(s2_score);

            // Update the scores in the main table after the loop
            for (name, score) in score_updates {
                match table.entry(name) {
                    Entry::Occupied(mut entry) => {
                        *entry.get_mut() += score;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(score);
                    }
                }
            }
        }

        println!("*************************************");
        println!("*************************************");
        println!("*************************************");
        for (player, score) in &table {
            println!("{}# {}", player, score);
        }
    }
}
