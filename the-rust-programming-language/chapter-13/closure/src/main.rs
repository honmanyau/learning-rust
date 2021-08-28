use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expansive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");

    thread::sleep(Duration::from_secs(2));

    return intensity;
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expansive_result = simulated_expansive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expansive_result);
        println!("Next, do {} situps!", expansive_result);
    } else {
        if random_number == 3 {
            println!("Take a braek today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expansive_result);
        }
    }
}
