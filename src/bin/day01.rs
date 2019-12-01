use std::fs;

fn main() {
    let filestring = fs::read_to_string("input/day01.txt")
        .expect("Unable to read file!");
    let lines = filestring.split("\n");
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for line in lines {
        match line.trim().parse::<i32>() {
            Ok(v) => {
                sum_part1 += fuel_requirement(v);
                sum_part2 += fuel_recursive(v);
            },
            Err(_e) => (),
        }
    }
    println!("The sum for Part 1 is: {}", sum_part1);
    println!("The sum for Part 2 is: {}", sum_part2);
}


fn fuel_requirement(mass: i32) -> i32 {
    let req = (mass / 3) - 2;
    if req < 0 {
        return 0;
    } else {
        return req;
    }
}

fn fuel_recursive(mass: i32) -> i32 {
    let mut req = 0;
    let mut additional = (mass / 3) - 2;
    while additional > 0 {
        req += additional;
        additional = (additional / 3) - 2;
    }
    return req;
}

