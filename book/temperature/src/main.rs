fn main() {
    let fahrenheit = 100.0;
    println!("to celsius: {}", to_celsius(fahrenheit));
    let celsius = 100.0;
    println!("to fahrenheit: {}", to_fahrenheit(celsius));
}

const RATE: f32 = 1.8;
const VALUE: f32 = 32.0;

fn to_celsius(f: f32) -> f32 {
    (f - VALUE) / RATE
}

fn to_fahrenheit(c: f32) -> f32 {
    c * RATE + VALUE
}
