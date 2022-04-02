use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let random_number = 32;
    generate_workout(simulated_intensity, random_number);
}

fn generate_workout(intensity: u32, random_number: u32){
    let expensive_result = |num | -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // let example_closure = |x  | x;

    // let s = example_closure(String::from("hello"));
    // let n  = example_closure(5);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result(intensity)
        );
    } else {
        if random_number == 3{
            println!("Take a break today! Remember to stay hydrated!");
        }else {
            println!(
                "Today, Run for {} minutes!",
                expensive_result(intensity)
            );
        }
    }
}

fn generate_workout_non_closure(intensity: u32, random_number: u32){
    // weakness:  not always used but still always be called
    let expensive_result = simulated_expensive_calculation(intensity); 
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3{
            println!("Take a break today! Remember to stay hydrated!");
        }else {
            println!(
                "Today, Run for {} minutes!",
                expensive_result
            );
        }
    }
}
