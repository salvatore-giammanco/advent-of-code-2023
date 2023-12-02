use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(process_game(line));
    }
    println!("Result: {:?}", result.into_iter().sum::<u32>());
}

// Game possible with:  12 red, 13 green, and 14 blue in the bag

fn process_game(line: &str) -> u32 {
    let line_iter = line.split(":").collect::<Vec<_>>();
    let game_sets = line_iter[1];
    if process_sets(game_sets) {
        let game_str = line_iter[0];
        let game_number = game_str.split_whitespace().collect::<Vec<_>>()[1].parse::<u32>().unwrap();
        return game_number;
    }
    0
}

fn process_sets(sets_str: &str) -> bool {
    let sets = sets_str.split(';').collect::<Vec<_>>();
    for set in sets {
        if !process_set(set) {
            return false;
        }
    }
    true
}

fn process_set(set: &str) -> bool {
    let cubes = set.split([',']).collect::<Vec<_>>();
    let mut cubes_map: HashMap<String, u32> = HashMap::new();
    for cube in cubes {
        let cube_str = cube.trim().split_whitespace().collect::<Vec<_>>();
        let cubes_color = cube_str[1];
        let cubes_number = cube_str[0].parse::<u32>().unwrap();
        *cubes_map.entry(cubes_color.to_string()).or_insert(0) += cubes_number;
    }
    let red_cubes = *cubes_map.entry("red".to_string()).or_insert(0) <= 12;
    let green_cubes = *cubes_map.entry("green".to_string()).or_insert(0) <= 13;
    let blue_cubes = *cubes_map.entry("blue".to_string()).or_insert(0) <= 14;

    red_cubes & green_cubes & blue_cubes
}