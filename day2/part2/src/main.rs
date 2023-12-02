use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut result = Vec::new();
    for line in input.lines() {
        let line_iter = line.split(":").collect::<Vec<_>>();
        let game_sets = line_iter[1];
        result.push(process_sets(game_sets));
    }
    println!("Result: {:?}", result.into_iter().sum::<u32>());
}

// Game possible with:  12 red, 13 green, and 14 blue in the bag


fn process_sets(sets_str: &str) -> u32 {
    let cubes = sets_str.split([',', ';']).collect::<Vec<_>>();
    let mut cubes_map: HashMap<String, u32> = HashMap::new();
    for cube in cubes {
        let cube_str = cube.trim().split_whitespace().collect::<Vec<_>>();
        let cubes_color = cube_str[1];
        let cubes_number = cube_str[0].parse::<u32>().unwrap();
        let entry = cubes_map.entry(cubes_color.to_string()).or_insert(cubes_number);
        if cubes_number > *entry {
            *entry = cubes_number;
        }
    }
    let min_red = *cubes_map.get("red").unwrap();
    let min_green = *cubes_map.get("green").unwrap();
    let min_blue = *cubes_map.get("blue").unwrap();
    let power = min_red * min_green * min_blue;
    power
}
