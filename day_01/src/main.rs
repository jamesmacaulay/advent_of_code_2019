fn part_1(input: &'static str) -> u32 {
    input
        .lines()
        .fold(0, |sum, line| sum + (line.parse::<u32>().unwrap() / 3 - 2))
}

fn part_2(input: &'static str) -> u32 {
    input.lines().fold(0, |sum, line| {
        sum + required_fuel(line.parse::<u32>().unwrap())
    })
}

fn required_fuel(mass: u32) -> u32 {
    let mut result = 0;
    let mut this_mass = mass;
    loop {
        let fuel = (this_mass / 3).saturating_sub(2);
        if fuel > 0 {
            result += fuel;
            this_mass = fuel;
        } else {
            return result;
        }
    }
}

fn main() {
    let input: &'static str = include_str!("input");
    println!("I: {:?}", part_1(input));
    println!("II: {:?}", part_2(input));
}
