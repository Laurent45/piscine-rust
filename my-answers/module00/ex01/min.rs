
fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    println!("Min between 3 and 5 should be 3 => min(3, 5) => {}", min(3, 5));
    println!("Min between 31 and 15 should be 15 => min(31, 15) => {}", min(31, 15));
    println!("Min between 5 and 5 should be 5 => min(5, 5) => {}", min(5, 5));
}

