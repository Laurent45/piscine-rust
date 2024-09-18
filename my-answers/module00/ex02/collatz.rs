fn collatz(start: u32) {
    println!("{start}");
    let mut n = start;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3*n + 1;
        }
        println!("{n}");
    }
}

fn main() {
    collatz(3);
}
