use std::fs;

pub fn solution() {
    let input = fs::read_to_string(String::from("./src/day/1/input")).unwrap();
    let mut increases: usize = 0;
    let mut prev_line: Option<&str> = None;

    for line in input.lines() {
        if prev_line.is_none() {
            prev_line = Some(line);
        } else {
            let prev_depth: u32 = prev_line.unwrap().parse::<u32>().unwrap();
            let current_depth: u32 = line.parse::<u32>().unwrap();

            if current_depth > prev_depth {
                increases = increases + 1;
            }

            prev_line = Some(line);
        }
    }

    println!("Day One Part One Solution: {}", increases);
}

pub fn solution_2() {
    let input = fs::read_to_string(String::from("./src/day/1/input")).unwrap();
    let mut increases: usize = 0;
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = lines[0].parse::<u32>().unwrap() + lines[1].parse::<u32>().unwrap() + lines[2].parse::<u32>().unwrap();


    for i in 3..lines.len() {
        let new_sum = sum - lines[i - 3].parse::<u32>().unwrap() + lines[i].parse::<u32>().unwrap();

        if new_sum > sum {
            increases = increases + 1;
        }
        
        sum = new_sum;
    }

    println!("Day One Part Two Solution: {}", increases);
}
