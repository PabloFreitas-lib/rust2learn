fn fib(n: u32) -> u32 {
    if n <= 2 {
        // The base case.
        return 1;
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 20;
    // around 48 this function will crash - attempt to add with overflow
    println!("fib({n}) = {}", fib(n));
}
