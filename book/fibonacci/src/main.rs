use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("failed to read line");
    let n: i32 = n.trim().parse().expect("type a number!");

    println!("fibonacchi num is {}", fibonacci(n));
}

fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-2) + fibonacci(n-1),
    }
}
