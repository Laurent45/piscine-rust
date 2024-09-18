fn print_bytes(s: &str) {
    for byte in s.bytes() {
        println!("{byte}");
    }
}

fn main() {
    print_bytes("Déjà Vu\n");
}
