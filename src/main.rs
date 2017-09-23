use std::io;

mod checker;

fn main() {
    println!("Enter ticket number: ");
    let mut ticket = String::new();

    io::stdin().read_line(&mut ticket)
        .expect("Failed to read ticket number");

    println!("Enter the winning number: ");
    let mut winning_number = String::new();
    io::stdin().read_line(&mut winning_number)
        .expect("Failed to read winning_number");

    //let win =

    //println("")
}
