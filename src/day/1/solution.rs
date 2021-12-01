use std::fs;

pub fn solution() {
    let input = fs::read_to_string(String::from("./src/day/1/input")).unwrap();
    let mut increases: usize = 0;
    let mut lines = input.lines();
    let mut prev_line: &str = lines.next().unwrap();

    for line in lines {
        let prev_depth: u32 = prev_line.parse::<u32>().unwrap();
        let current_depth: u32 = line.parse::<u32>().unwrap();

        if current_depth > prev_depth {
            increases = increases + 1;
        }

        prev_line = line;
    }

    println!("Day One Part One Solution: {}", increases);
    assert_eq!(1722usize, increases);
}

pub fn solution_2() {
    let input = fs::read_to_string(String::from("./src/day/1/input")).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = lines[0].parse::<u32>().unwrap() +
        lines[1].parse::<u32>().unwrap() +
        lines[2].parse::<u32>().unwrap();
    let mut increases: usize = 0;

    for i in 3..lines.len() {
        let next_sum = sum -
            lines[i - 3].parse::<u32>().unwrap() +
            lines[i].parse::<u32>().unwrap();

        if next_sum > sum {
            increases = increases + 1;
        }
        
        sum = next_sum;
    }

    println!("Day One Part Two Solution: {}", increases);
    assert_eq!(1748usize, increases);
}
