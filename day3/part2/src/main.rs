use std::{any::Any, collections::HashSet};

const LINE_LENGTH: u32 = 141; // Including \n
const TOTAL_LINES: u32 = 140;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Type {
    Number(char),
    Void,
    Symbol,
    Gear,
}

#[derive(Eq, PartialEq, Hash)]
struct Number {
    value: u32,
    length: u8,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let tokens = parse_input(&input);
    println!("{:?}", inspect_engine(tokens));
}

fn inspect_engine(tokens: Vec<Type>) -> u64 {
    let mut result: u64 = 0;
    let lines = tokens.len() as u32 / LINE_LENGTH;
    for line in 0..lines {
        let line_start = (line * LINE_LENGTH) as usize;
        let line_end = line_start + LINE_LENGTH as usize;
        result += inspect_line(&tokens[line_start..line_end], line, &tokens);
    }
    result
}

fn inspect_line(line_tokens: &[Type], line: u32, tokens: &Vec<Type>) -> u64 {
    let mut line_result: u64 = 0;

    let mut i: u32 = 0;
    while i < LINE_LENGTH {
        if line_tokens[i as usize] == Type::Gear {
            let point = Point {
                x: i as u32,
                y: line,
            };
            
            line_result += check_neighbours(&point, tokens);
                
        }
        i += 1;
    }
        
    line_result
}


fn get_number(starting_point: Point, tokens: &[Type]) -> Number {
    let mut current_x = starting_point.x as usize;
    
    let mut number = Number {
        value: 0,
        length: 0,
    };

    loop {
        if current_x == 0 {
            break;
        } else {
            current_x -= 1;
        }
        let index = starting_point.y * LINE_LENGTH + current_x as u32;
        let current_token = tokens[index as usize];
        match current_token {
            Type::Number(_c) => {},
            _ => {
                current_x += 1;
                break;
            },
        }
    }

    loop {
        let index = starting_point.y * LINE_LENGTH + current_x as u32;
        let current_token = tokens[index as usize];
        match current_token {
            Type::Number(c) => {
                number.value = number.value * 10 + c.to_digit(10).unwrap();
                number.length += 1;
                current_x += 1;
            },
            _ => break,
        }
    }
    number
}

fn check_neighbours(point: &Point, tokens: &[Type]) -> u64 {
    let x_start = if point.x > 0 {
        point.x - 1
    } else {
        point.x
    };

    let x_end = if point.x < LINE_LENGTH {
        point.x + 2
    } else {
        point.x + 1
    };
    
    let y_start = if point.y > 0 {
        point.y - 1
    } else {
        point.y
    };

    let y_end = if point.y + 1 < TOTAL_LINES {
        point.y + 2
    } else {
        point.y + 1
    };
    let mut numbers: HashSet<Number> = HashSet::new();
    for x in x_start..x_end as u32 {
        for y in y_start..y_end {
            if check_point(Point {x, y}, tokens){
                numbers.insert(get_number(Point {x, y}, tokens));
            }
        }
    }
    let numbers = numbers.into_iter().collect::<Vec<Number>>();
    if numbers.len() == 2 {
        return (numbers[0].value * numbers[1].value) as u64;
    }
    0
}

fn check_point(point: Point, tokens: &[Type]) -> bool {
    let index = point.y * LINE_LENGTH + point.x;
    match tokens[index as usize] {
        Type::Number(_c) => true,
        _ => false,
    }
}


fn parse_input(input: &str) -> Vec<Type> {
    let mut tokens = vec![];
    for c in input.chars() {
        tokens.push(
            match c {
                '0'..='9' => Type::Number(c),
                '.' | '\n' => Type::Void,
                '*' => Type::Gear,
                _ => Type::Symbol,
            }
        );
    }
    tokens
}