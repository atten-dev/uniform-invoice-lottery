
#[derive(PartialEq)]
#[derive(Debug)]
enum PrizeType {
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

struct WinningNumbers {
    special_prize: String,
    grand_prize: String,
    regular_prizes: [String; 3],
    additional_prize: String
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

fn check_ticket_all(winning_numbers: &WinningNumbers, ticket_number: &str) -> PrizeType
{
    PrizeType::NONE
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_last_digit_off() {
        assert_eq!(0, check_ticket("AB-87654320","AB-87654321"));
    }

    #[test]
    fn check_full_match() {
        assert_eq!(8, check_ticket("AB-87654321","AB-87654321"));
    }

    #[test]
    fn check_last_three() {
        assert_eq!(3, check_ticket("AB-87654321","AB-00000321"));
    }

    #[test]
    fn check_last_two_no_match() {
        assert_eq!(0, check_ticket("AB-87654321","AB-00000021"));
    }

    #[test]
    fn check_full_ticket() {
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
    }
}
