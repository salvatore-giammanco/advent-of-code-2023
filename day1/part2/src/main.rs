fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut nums = Vec::new();
    for line in input.lines() {
        let first_num = find_first_number(line);
        let last_num = find_last_number(line);
        
        println!("{:?}, {:?}", first_num, last_num); 
        let num = first_num.unwrap() * 10 + last_num.unwrap();
        nums.push(num);
    }
    println!("{:?}", nums.iter().map(|&i| i as u32).sum::<u32>());
}


struct Number {
    name: String,
    value: u8,
}

fn extract_spelled_number(input: &str, starts: bool) -> Option<u32> {
    let mut first_number = None;
    let numbers = vec![
        Number {
            name: "zero".to_string(),
            value: 0,
        },
        Number {
            name: "one".to_string(),
            value: 1,
        },
        Number {
            name: "two".to_string(),
            value: 2,
        },
        Number {
            name: "three".to_string(),
            value: 3,
        },
        Number {
            name: "four".to_string(),
            value: 4,
        },
        Number {
            name: "five".to_string(),
            value: 5,
        },
        Number {
            name: "six".to_string(),
            value: 6,
        },
        Number {
            name: "seven".to_string(),
            value: 7,
        },
        Number {
            name: "eight".to_string(),
            value: 8,
        },
        Number {
            name: "nine".to_string(),
            value: 9,
        },
    ];
    
    for number in numbers {
        if starts == true {
            if input.starts_with(&number.name) {
                first_number = Some(number.value as u32);
                break;
            }
        } else {
            if input.ends_with(&number.name) {
                first_number = Some(number.value as u32);
                break;
            }
        }
    }
    first_number
}

fn find_first_number(input: &str) -> Option<u32> {
    let mut first_number = None;
    let chars = input.as_bytes();
    for i in 0..input.len() {
        if chars[i].is_ascii_digit() {
            first_number = Some(chars[i] as u32 - 48);
            break;
        } else {
            let remaining = input.len() - i;
            let offset = if remaining < 5 {
                remaining
            } else {
                5   
            };

            first_number = extract_spelled_number(&input[i..i+offset], true);
            if first_number.is_some() {
                break;
            }
        }
    }
    first_number
}

fn find_last_number(input: &str) -> Option<u32> {
    let mut first_number = None;
    let chars = input.as_bytes();
    for i in (0..input.len()).rev() {
        if chars[i].is_ascii_digit() {
            first_number = Some(chars[i] as u32 - 48);
            break;
        } else {
            let remaining = 0 + i;
            let offset = if remaining < 5 {
                remaining
            } else {
                5   
            };

            first_number = extract_spelled_number(&input[i-offset+1..i+1], false);
            if first_number.is_some() {
                break;
            }
        }
    }
    first_number
}