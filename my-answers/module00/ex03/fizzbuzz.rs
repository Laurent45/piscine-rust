fn main() {
    for i in 1..=100 {
        match i {
            i if i % 3 == 0 => println!("fizz"),
            i if i % 5 == 0 => println!("buzz"),
            i if i % 3 == 0 && i % 5 == 0 => println!("fizzbuzz"),
            i if (i - 3) % 11 == 0 => println!("FIZZ"),
            i if (i - 5) % 11 == 0 => println!("BUZZ"),
            _ => println!("{i}")
        }
    }
}
