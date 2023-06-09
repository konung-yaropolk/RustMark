fn main() {
    let mut n = 1u32;
    let repeats = 100000000u32;
    let mut pi = 1f64;
    while n < repeats {
        pi = pi * 2.0 * n as f64 / (2*n-1) as f64 * 2.0 * n as f64 /(2*n+1) as f64;
        n += 1;
    }
    pi=pi*2.0;
    println!("Pi = {} iterations done: {}\n", pi, n);
    return ();
}