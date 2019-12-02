use std::fs;

fn main() {
    let filestring = fs::read_to_string("input/day02.txt")
        .expect("Unable to read file!");
    let lines = filestring.split(",");
    let program = lines.map(|line| line.trim().parse::<i32>().unwrap()).collect();
    
    println!("Output for Part 1: {}", run(&program, 12, 2));

    'outer: for noun in 0..100 {
        'inner: for verb in 0..100 {
            if run(&program, noun, verb) == 19690720 {
                println!("Noun, Verb for Part 2: {}, {}", noun, verb);
                break 'outer;
            }
        }
    }

}


fn run(initial: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut state = initial.to_vec();
    state[1] = noun;
    state[2] = verb;
    let mut pos = 0;
    loop {
        match state[pos] {
            99 => break,
            1 => {
                let writepos: usize = state[pos+3] as usize;
                state[writepos] =
                    state[state[pos+1] as usize] + state[state[pos+2] as usize];
            }
            2 => {
                let writepos: usize = state[pos+3] as usize;
                state[writepos] =
                    state[state[pos+1] as usize] * state[state[pos+2] as usize];
            }
            _ => panic!("Bad opcode!"),
        }
        pos += 4;
    }
    return state[0];
}

