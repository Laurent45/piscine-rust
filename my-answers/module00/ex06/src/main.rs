use std::cmp::Ordering;

fn main() {
    let random_num = ftkit::random_number(0..=100);

    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");

    loop {
        let user_num = ftkit::read_number();
        match user_num.cmp(&random_num) {
            Ordering::Less => println!("This student might not be as smart as I was told. This answer is obviously too weak."),
            Ordering::Greater => println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
            Ordering::Equal => {
                println!("That is right! The secret was indeed the number {user_num}, which you have brilliantly discovered!");
                break;
            }
        }
    }
}
