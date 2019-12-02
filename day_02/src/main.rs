fn part_1(input: &'static str) -> usize {
    let mut memory: Vec<usize> = input
        .split(",")
        .map(|token| token.parse::<usize>().unwrap())
        .collect();
    memory[1] = 12;
    memory[2] = 2;
    run(&mut memory);
    memory[0]
}

fn part_2(input: &'static str) -> usize {
    let immutable_memory: Vec<usize> = input
        .split(",")
        .map(|token| token.parse::<usize>().unwrap())
        .collect();
    for noun in 0..immutable_memory.len() {
        for verb in 0..immutable_memory.len() {
            let mut memory: Vec<usize> = immutable_memory.clone();
            memory[1] = noun;
            memory[2] = verb;
            run(&mut memory);
            if memory[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("valid noun and verb not found")
}

fn run(memory: &mut Vec<usize>) {
    let mut location: usize = 0;
    loop {
        let opcode = memory[location];
        match opcode {
            1 => {
                let arg_1 = memory[location + 1];
                let arg_2 = memory[location + 2];
                let ret = memory[location + 3];
                memory[ret] = memory[arg_1] + memory[arg_2];
            }
            2 => {
                let arg_1 = memory[location + 1];
                let arg_2 = memory[location + 2];
                let ret = memory[location + 3];
                memory[ret] = memory[arg_1] * memory[arg_2];
            }
            99 => {
                return;
            }
            _ => panic!("unexpected opcode {:?} at location {:?}", opcode, location),
        }
        location += 4;
    }
}

fn main() {
    let input: &'static str = include_str!("input");
    println!("I: {:?}", part_1(input));
    println!("II: {:?}", part_2(input));
}
