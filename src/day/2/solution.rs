use std::fs;

pub fn solution() {
    let input = fs::read_to_string(String::from("./src/day/2/input")).unwrap();
    let lines = input.lines();
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;

    for line in lines {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        let command = split_line[0];
        let magnitude = split_line[1].parse::<i32>().unwrap();

        match command {
            "forward" => {
                horizontal = horizontal + magnitude;
            },
            "down" => {
                depth = depth + magnitude;
            },
            "up" => {
                depth = depth - magnitude;
            },
            _ => {}
        }
    }

    println!("Day Two Part One Solution: {}", horizontal * depth);
}

pub fn solution_2() {
    let input = fs::read_to_string(String::from("./src/day/2/input")).unwrap();
    let lines = input.lines();
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;

    for line in lines {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        let command = split_line[0];
        let magnitude = split_line[1].parse::<i32>().unwrap();

        match command {
            "forward" => {
                horizontal = horizontal + magnitude;
                depth = depth + (magnitude * aim);
            },
            "down" => {
                aim = aim + magnitude;
            },
            "up" => {
                aim = aim - magnitude;
            },
            _ => {}
        }
    }

    println!("Day Two Part Two Solution: {}", horizontal * depth);
}
