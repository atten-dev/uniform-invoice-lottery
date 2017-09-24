
#[derive(PartialEq)]
#[derive(Debug)]
pub enum PrizeType {
    SPECIAL,
    GRAND,
    FIRST,
    SECOND,
    THIRD,
    FOURTH,
    FIFTH,
    SIXTH,
    ADDITIONAL,
    NONE
}

#[derive(Debug)]
pub struct WinningNumbers {
    pub special_prize: String,
    pub grand_prize: String,
    pub regular_prizes: [String; 3],
    pub additional_prize: String
}

fn check_ticket(winning_number: &str, ticket_number: &str) -> usize {
    let mut max_length = 0;

    for length in (3..9).rev() {
        let winning_reverse: String = winning_number.chars().rev().collect();
        let ticket_reverse: String = ticket_number.chars().rev().collect();
        if winning_reverse[0..length] == ticket_reverse[0..length] {
            max_length = length;
            break;
        }
    }

    max_length
}

fn convert_number_to_prize(number: usize) -> PrizeType {
    match number {
        8 => PrizeType::FIRST,
        7 => PrizeType::SECOND,
        6 => PrizeType::THIRD,
        5 => PrizeType::FOURTH,
        4 => PrizeType::FIFTH,
        3 => PrizeType::SIXTH,
        _ => PrizeType::NONE,
    }
}

fn check_ticket_regular(regular_prizes: &[String; 3], ticket_number: &str) -> PrizeType {
    let regular1 = check_ticket(&regular_prizes[0], ticket_number);
    let regular2 = check_ticket(&regular_prizes[1], ticket_number);
    let regular3 = check_ticket(&regular_prizes[2], ticket_number);

    let results = [regular1, regular2, regular3];
    let best_result = results.iter().max();

    match best_result {
        Some(number) => convert_number_to_prize(*number),
        None => PrizeType::NONE,
    }
}

pub fn check_ticket_all(winning_numbers: &WinningNumbers, ticket_number: &str) -> PrizeType
{
    let special_rev = winning_numbers.special_prize.chars().rev();
    let grand_rev = winning_numbers.grand_prize.chars().rev();
    let ticket_rev : String = ticket_number.chars().rev().collect();

    let regular_prize_result = check_ticket_regular(&winning_numbers.regular_prizes, ticket_number);

    if ticket_rev.chars().take(8).eq(special_rev) {
        PrizeType::SPECIAL
    }
    else if ticket_rev.chars().take(8).eq(grand_rev) {
        PrizeType::GRAND
    }
    else if regular_prize_result != PrizeType::NONE {
        regular_prize_result
    }
    else if ticket_rev.chars().take(3).eq(winning_numbers.additional_prize.chars().rev()) {
        PrizeType::ADDITIONAL
    }
    else
    {
        PrizeType::NONE
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_digit_off() {
        assert_eq!(0, check_ticket("AB-87654320","AB-87654321"));
    }

    #[test]
    fn test_full_match() {
        assert_eq!(8, check_ticket("AB-87654321","AB-87654321"));
    }

    #[test]
    fn test_last_three() {
        assert_eq!(3, check_ticket("AB-87654321","AB-00000321"));
    }

    #[test]
    fn test_last_two_no_match() {
        assert_eq!(0, check_ticket("AB-87654321","AB-00000021"));
    }
/////////////////////// Convert number to prize
    #[test]
    fn test_convert_number_to_prize() {
        assert_eq!(PrizeType::FIRST, convert_number_to_prize(8));
        assert_eq!(PrizeType::SECOND, convert_number_to_prize(7));
        assert_eq!(PrizeType::THIRD, convert_number_to_prize(6));
        assert_eq!(PrizeType::FOURTH, convert_number_to_prize(5));
        assert_eq!(PrizeType::FIFTH, convert_number_to_prize(4));
        assert_eq!(PrizeType::SIXTH, convert_number_to_prize(3));
        assert_eq!(PrizeType::NONE, convert_number_to_prize(2));
        assert_eq!(PrizeType::NONE, convert_number_to_prize(1));
        assert_eq!(PrizeType::NONE, convert_number_to_prize(0));
        assert_eq!(PrizeType::NONE, convert_number_to_prize(9));
    }
/////////////////////// check_ticket_regular
    #[test]
    fn test_check_regular() {
        let regular_numbers = [String::from("98765432"),
                               String::from("87654321"),
                               String::from("76543210")];

        assert_eq!(PrizeType::FIRST, check_ticket_regular(&regular_numbers,  "98765432"));
        assert_eq!(PrizeType::FIRST, check_ticket_regular(&regular_numbers,  "87654321"));
        assert_eq!(PrizeType::FIRST, check_ticket_regular(&regular_numbers,  "76543210"));
        assert_eq!(PrizeType::SECOND, check_ticket_regular(&regular_numbers, "08765432"));
        assert_eq!(PrizeType::THIRD, check_ticket_regular(&regular_numbers,  "00765432"));
        assert_eq!(PrizeType::FOURTH, check_ticket_regular(&regular_numbers, "00065432"));
        assert_eq!(PrizeType::FIFTH, check_ticket_regular(&regular_numbers,  "00005432"));
        assert_eq!(PrizeType::SIXTH, check_ticket_regular(&regular_numbers,  "00000432"));
        assert_eq!(PrizeType::NONE, check_ticket_regular(&regular_numbers,   "00000032"));

        // Double check that smaller, non-first prizes are checked on the other winning numbers
        assert_eq!(PrizeType::FIFTH, check_ticket_regular(&regular_numbers,  "00004321"));
        assert_eq!(PrizeType::SIXTH, check_ticket_regular(&regular_numbers,  "00000210"));
    }

///////////////////// check_ticket_all
    #[test]
    fn test_full_ticket() {
        let winning_numbers = WinningNumbers {
            special_prize: String::from("01234567"),
            grand_prize: String::from("12345678"),
            regular_prizes: [String::from("98765432"),
                             String::from("87654321"),
                             String::from("76543210")],
            additional_prize: String::from("765")
        };

        assert_eq!(PrizeType::NONE, check_ticket_all(&winning_numbers, "AZ-39842818"));
        assert_eq!(PrizeType::SPECIAL, check_ticket_all(&winning_numbers, "AD-01234567"));
        assert_eq!(PrizeType::GRAND, check_ticket_all(&winning_numbers, "AD-12345678"));
        assert_eq!(PrizeType::FIRST, check_ticket_all(&winning_numbers, "AD-98765432"));
        assert_eq!(PrizeType::SIXTH, check_ticket_all(&winning_numbers, "BB-00000210"));
        assert_eq!(PrizeType::ADDITIONAL, check_ticket_all(&winning_numbers, "ZZ-00000765"));
    }

    #[test]
    fn test_largest_prize_returned() {
        let winning_numbers = WinningNumbers {
            special_prize: String::from("98765432"),
            grand_prize: String::from("87654321"),
            regular_prizes: [String::from("08765432"), // Almost same as special
                             String::from("07654321"), // Almost same as grand
                             String::from("76543210")],
            additional_prize: String::from("321") //Same as last 3 of both grand and reguar
        };

        assert_eq!(PrizeType::SPECIAL, check_ticket_all(&winning_numbers, "AZ-98765432"));
        assert_eq!(PrizeType::GRAND, check_ticket_all(&winning_numbers, "AD-87654321"));
        assert_eq!(PrizeType::FOURTH, check_ticket_all(&winning_numbers, "AF-00054321"));
    }
}
