use core::panic;

use module00_ex07::strpcmp;

fn main() {
    if ftkit::ARGS.len() != 3 {
        panic!("Usage: cargo run -- <query> <pattern>");
    }

    let query = &ftkit::ARGS[1];
    let pattern = &ftkit::ARGS[2];

    if strpcmp(query.as_bytes(), pattern.as_bytes()) {
        println!("yes");
    } else {
        println!("no");
    }
}
