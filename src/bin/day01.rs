fn calculate_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn main() {
    println!("Advent of Code - Day 1");
    
    let test_mass = 12;
    let fuel = calculate_fuel(test_mass);
    
    println!("Fuel required for mass {}: {}", test_mass, fuel);
    
    // Test a few more values
    for mass in [14, 1969, 100756] {
        println!("Fuel for mass {}: {}", mass, calculate_fuel(mass));
    }
}

