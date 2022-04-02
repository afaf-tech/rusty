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

struct Cacher<T>
where 
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where 
    T: Fn(u32) -> u32,
{   
    // constructor function
    fn new(calculation: T) -> Cacher<T> {
        Cacher{
            calculation,
            value: None,
        }
    }


    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v); // caching happens here
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32){
    let mut cached_result = Cacher::new(|num | -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let example_closure = |x  | x;

    // let s = example_closure(String::from("hello"));
    // let n  = example_closure(5);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cached_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            cached_result.value(intensity)
        );
    } else {
        if random_number == 3{
            println!("Take a break today! Remember to stay hydrated!");
        }else {
            println!(
                "Today, Run for {} minutes!",
                cached_result.value(intensity)
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
