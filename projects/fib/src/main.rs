fn main() {
    let res = fib(6);
    println!("fib(6) = {res}");
}

fn fib(n: u32) -> u32 {
    if n == 1 {
        1
    } else if n == 0 {
        0
    } else {
        fib(n - 2) + fib(n - 1)
    }
}
