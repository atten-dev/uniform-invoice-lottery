use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;

mod checker;
use checker::WinningNumbers;
use checker::PrizeType;
use checker::check_ticket_all;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 10 {
        println!("Invalid arguments. Correct usage:");
        print!("<exec> SpecialPrize, GrandPrize, RegularPrize1, RegularPrize2, RegularPrize3, ");
        println!("AdditionalPrize1 AdditionalPrize2 AdditionalPrize3 <invoices file name>");
        process::exit(1);
    }

    for i in 1..6 {
        if args[i].len() != 8 {
            println!("Argument {} not 8 digits.", i);
            process::exit(1);
        }
    }
    if args[6].len() != 3 && args[7].len() != 3 && args[8].len() != 3 {
        println!("AdditionalPrize should be 3 digits!");
        process::exit(1);
    }
    let winning_numbers = WinningNumbers {
        special_prize: args[1].clone(),
        grand_prize: args[2].clone(),
        regular_prizes: [ args[3].clone(), args[4].clone(), args[5].clone() ],
        additional_prizes: [ args[6].clone(), args[7].clone(), args[8].clone() ],
    };
    println!("{:?}", winning_numbers);

    let mut f = File::open(args[9].clone()).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File read failed");

    let tickets : Vec<&str> = contents.split("\n").map(|t| t.trim()).collect();

    let mut total = 0;
    let mut winners = 0;
    for ticket in tickets {
        if ticket.len() == 11 || ticket.len() == 8 {
            let result = check_ticket_all(&winning_numbers, ticket);
            //println!("{} {:?}", ticket, result)
            match result {
                PrizeType::SPECIAL => println!("{} won the Specia Prize!", ticket),
                PrizeType::GRAND => println!("{} won the Grand Prize!", ticket),
                PrizeType::FIRST => println!("{} won the First Prize!", ticket),
                PrizeType::SECOND => println!("{} won the Second Prize!", ticket),
                PrizeType::THIRD => println!("{} won the Third Prize!", ticket),
                PrizeType::FOURTH => println!("{} won the Fourth Prize!", ticket),
                PrizeType::FIFTH => println!("{} won the Fifth Prize!", ticket),
                PrizeType::SIXTH => println!("{} won the Sixth Prize!", ticket),
                PrizeType::ADDITIONAL => println!("{} won the Additional Prize!", ticket),
                PrizeType::NONE => ()
            }
            if result != PrizeType::NONE {
                winners+=1;
            }
            total+=1;
        }
        else {
            if ticket.len() != 0
            {
                println!("Ticket {} is malformed. Exiting.", ticket);
                process::exit(1);
            }
        }
    }

    println!("Checked {} valid tickets and found {} winners.", total, winners);
}
