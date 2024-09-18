use std::io;

fn main() {
    let mut input = String::new();

    println!("type a positive number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u64 = input.trim().parse().expect("Please type a number!");
    let result = fib(input);
    println!("fib({input}): {result}");

}

fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}