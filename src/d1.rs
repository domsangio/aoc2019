fn get_fuel_from_module(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn get_recursive_fuel_from_module(mass: i32) -> i32 {
    let next_mass = get_fuel_from_module(mass);
    if next_mass > 8 {
        return next_mass + get_recursive_fuel_from_module(next_mass)
    }
    next_mass
} 

fn d1a(input: &str) -> i32 {
    input.lines().map(|line| line.parse::<i32>().unwrap()).map(get_fuel_from_module).sum::<i32>()
}

fn d1b(input: &str) -> i32 {
    input.lines().map(|line| line.parse::<i32>().unwrap()).map(get_recursive_fuel_from_module).sum::<i32>()
}

pub fn runner(input: &str) {
    let d1a_result = d1a(input);
    println!("d1a: {}", d1a_result);
    println!("d1b: {}", d1b(input));
}