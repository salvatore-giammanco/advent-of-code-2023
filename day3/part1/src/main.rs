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
}

struct Number {
    value: u32,
    starting_point: Point,
    length: u8,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let tokens = parse_input(&input);
    println!("{:?}", inspect_engine(tokens));
}

fn inspect_engine(tokens: Vec<Type>) -> u32 {
    let mut result: u32 = 0;
    let lines = tokens.len() as u32 / LINE_LENGTH;
    for line in 0..lines {
        let line_start = (line * LINE_LENGTH) as usize;
        let line_end = line_start + LINE_LENGTH as usize;
        result += inspect_line(&tokens[line_start..line_end], line,  &tokens);
    }
    result
}

fn inspect_line(line_tokens: &[Type], line: u32, tokens: &Vec<Type>) -> u32 {
    let mut line_result: u32 = 0;

    let mut i: u32 = 0;
    while i < LINE_LENGTH {
        match line_tokens[i as usize] {
            Type::Number(_c) => {
                let point = Point {
                    x: i as u32,
                    y: line,
                };
                let number = get_number(point, line_tokens);
                if check_neighbours(&number, tokens) {
                    line_result += number.value;
                }
                i += (number.length) as u32;
            },
            _ => {
                i += 1;
            },
        }
    }
    line_result
}


fn get_number(starting_point: Point, line_tokens: &[Type]) -> Number {
    let mut current_x = starting_point.x as usize;
    
    let mut number = Number {
        value: 0,
        starting_point,
        length: 0,
    };
    
    loop {
        let current_token = line_tokens[current_x];
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

fn check_neighbours(number: &Number, tokens: &[Type]) -> bool {
    let x_start = if number.starting_point.x > 0 {
        number.starting_point.x - 1
    } else {
        number.starting_point.x
    };

    let x_end = if (number.starting_point.x + number.length as u32) < LINE_LENGTH {
        number.starting_point.x + number.length as u32 + 1 
    } else {
        number.starting_point.x + number.length as u32
    };
    
    let y_start = if number.starting_point.y > 0 {
        number.starting_point.y - 1
    } else {
        number.starting_point.y
    };

    let y_end = if number.starting_point.y + 1 < TOTAL_LINES {
        number.starting_point.y + 2
    } else {
        number.starting_point.y + 1
    };
    for x in x_start..x_end as u32 {
        for y in y_start..y_end {

            if check_point(Point {x, y}, tokens){
                return true;
            }
        }
    }
    false
}

fn check_point(point: Point, tokens: &[Type]) -> bool {
    let index = point.y * LINE_LENGTH + point.x;
    tokens[index as usize] == Type::Symbol
}


fn parse_input(input: &str) -> Vec<Type> {
    let mut tokens = vec![];
    for c in input.chars() {
        tokens.push(
            match c {
                '0'..='9' => Type::Number(c),
                '.' | '\n' => Type::Void,
                _ => Type::Symbol,
            }
        );
    }
    tokens
}