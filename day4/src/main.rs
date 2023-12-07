#[derive(Debug)]
struct Scratchcard {
    winning_numbers: Vec<u32>,
    line_numbers: Vec<u32>,
    points: u32,
    matches: u32,
    index: u32,
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let mut scratchcards = parse_input(&input);
    part1(&mut scratchcards);
    //println!("Scratchcards: {:?}", scratchcards);
    part2(&scratchcards);
}

fn part2(scratchcards: &Vec<Scratchcard>) {
    let mut total_scratchcards = scratchcards.len() as u32;
    for (i, scratchcard) in scratchcards.into_iter().enumerate() {
        if scratchcard.matches > 0 {
            let copies_won = &scratchcards[i + 1..(i + scratchcard.matches as usize + 1)];
            total_scratchcards += scratchcard.matches;
            total_scratchcards += process_copies(copies_won, scratchcards);
        }
    }
    println!("Total scratchcards: {:?}", total_scratchcards);
}

fn process_copies(copies: &[Scratchcard], scratchcards: &Vec<Scratchcard>) -> u32 {
    let mut total_scratchcards = 0;
    for (i, scratchcard) in copies.into_iter().enumerate() {
        if scratchcard.matches > 0 {
            let copies_won = &scratchcards[scratchcard.index as usize + 1..(scratchcard.index + scratchcard.matches + 1) as usize];
            total_scratchcards += scratchcard.matches;
            total_scratchcards += process_copies(copies_won, scratchcards);
        }
    }
    total_scratchcards
}

fn part1(scratchcards: &mut Vec<Scratchcard>) {
    let mut points: u32 = 0;
    for scratchcard in scratchcards {
        for line_number in &scratchcard.line_numbers {
            if scratchcard.winning_numbers.contains(&line_number) {
                if scratchcard.points == 0 {
                    scratchcard.points = 1;
                } else {
                    scratchcard.points *= 2;
                }
                scratchcard.matches += 1;
            }
        }
        points += scratchcard.points;
    }
    println!("Total points: {:?}", points);
}

fn parse_input(input: &String) -> Vec<Scratchcard> {
    let mut scratchcards: Vec<Scratchcard> = Vec::new();
    for (i, line) in input.lines().into_iter().enumerate() {
        let split_line = line.split([':', '|']).collect::<Vec<&str>>();
        let winning_numbers = split_line[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let line_numbers = split_line[2]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<u32>()
            .unwrap())
            .collect::<Vec<u32>>();
        scratchcards.push(Scratchcard {
            winning_numbers,
            line_numbers,
            points: 0,
            matches: 0,
            index: i as u32,
        });
    }
    scratchcards
}