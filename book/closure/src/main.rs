use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simurated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simurated_user_specified_value, simulated_random_number);
}

struct Cacher<T, U, W>
where
    T: Fn(U) -> W,
{
    calculation: T,
    value: HashMap<U, W>,
}

impl<T, U, W> Cacher<T, U, W>
where
    T: Fn(U) -> W,
    U: std::cmp::Eq + std::hash::Hash + Copy,
    W: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, W> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> W {
        match self.value.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let key = arg.clone();
                let v = (self.calculation)(arg);
                self.value.insert(key, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

#[test]
fn call_with_multiple_types() {
    let mut c1 = Cacher::new(|a| a);

    let v1 = c1.value(1);
    assert_eq!(v1, 1);

    let mut c2 = Cacher::new(|s: &str| -> usize { s.len() });
    let v2: usize = From::from(c2.value("rust"));

    assert_eq!(v2, 4);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));

        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
