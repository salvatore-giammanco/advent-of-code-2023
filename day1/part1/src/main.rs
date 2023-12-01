fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut nums = Vec::new();
    for line in input.lines() {
        let line_nums = find_numbers(line);
        let num = line_nums.first().unwrap() * 10 + line_nums.last().unwrap();
        nums.push(num);
    }
    println!("{:?}", nums.iter().sum::<u32>());
}


fn find_numbers(line: &str) -> Vec<u32> {
    let mut line_nums = Vec::new();
    let chars = line.chars();

    for char in chars {
        if char.is_ascii_digit() {
            line_nums.push(char.to_digit(10).unwrap());
        }
    }
    line_nums
}