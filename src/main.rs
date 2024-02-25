fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let n = 10;
    let result = fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}
