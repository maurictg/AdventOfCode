use std::fs;

fn main() {
    let masses: Vec<f32> = fs::read_to_string("input.txt").unwrap().lines().map(|x|x.parse().unwrap()).collect();
    
    // Part 1
    let fuel: f32 = masses.iter().map(|f|fuel_for_mass(*f)).sum();
    println!("Part 1: {}", fuel);

    // Part 2
    let fuel: f32 = masses.iter().map(|m|fuel_for_mass(*m)).map(|f|f + fuel_for_fuel(f)).sum();
    println!("Part 2: {}", fuel);
}

fn fuel_for_mass(mass: f32) -> f32 {
    (mass / 3.0).floor() - 2.0
}

fn fuel_for_fuel(fuel_mass: f32) -> f32 {
    let fuel = fuel_for_mass(fuel_mass);
    if fuel <= 0.0 { 0.0 } else { fuel + fuel_for_fuel(fuel) }
}