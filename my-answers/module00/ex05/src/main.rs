use core::panic;


fn is_leap_year(year: u32) -> bool {
    if year == 0 {
        panic!("[is_leap_year] year parameter cannot be equal to zero");
    }

    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    match (is_leap_year(year), month) {
        (true, 2) => 29,
        (false, 2) => 28,
        (_, 1 | 3 | 5 | 7 | 8 | 10 | 12) => 31,
        (_, 4 | 6 | 9 | 11) => 30,
        (_, _) => panic!("[num_days_in_month] month [{month}] is invalid, should be between 1-12")
    }
}

fn str_month(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => panic!("{month} is not a valid month")
    }
}

fn main() {
    // Fist day of month always equal to value between 0-6
    // 0: sunday - 6: saturday
    let mut first_day = 1;
    const FRIDAY_DAY: u32 = 5;

    for year in 1.. {

        for month in 1..=12 {
            let thirteenth_day = (first_day + 12) % 7;
            if thirteenth_day == FRIDAY_DAY {
                    println!("Friday, {} 13, {year}", str_month(month));
                }
            first_day = (first_day + num_days_in_month(year, month)) % 7
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::{is_leap_year, num_days_in_month};



    #[test]
    fn test_leap_year_divisible_by_4() {
        assert!(is_leap_year(2024), "2024 should be a leap year");
    }

    #[test]
    fn test_non_leap_year_divisible_by_100() {
        assert!(!is_leap_year(1900), "1900 should not be a leap year");
    }

    #[test]
    fn test_leap_year_divisible_by_400() {
        assert!(is_leap_year(2000), "2000 should be a leap year");
    }

    #[test]
    fn test_non_leap_year_not_divisible_by_4() {
        assert!(!is_leap_year(2023), "2023 should not be a leap year");
    }

    #[test]
    fn test_non_leap_year_divisible_by_100_but_not_400() {
        assert!(!is_leap_year(2100), "2100 should not be a leap year");
    }

    #[test]
    fn test_leap_year_edge_case_4000() {
        assert!(is_leap_year(4000), "4000 should be a leap year");
    }

    #[test]
    fn should_be_a_leap_year_1600() {
        assert!(is_leap_year(1600), "1600 is a leap year");
    } 

    #[test]
    fn should_not_be_a_leap_year_1500() {
        assert!(!is_leap_year(1500), "1500 is not a leap year");
    } 

    #[test]
    fn should_be_a_leap_year_2004() {
        assert!(is_leap_year(2004), "2004 is a leap year");
    } 

    #[test]
    fn should_not_be_a_leap_year_2003() {
        assert!(!is_leap_year(2003), "2003 is not a leap year");
    } 

    #[test]
    fn test_february_in_leap_year() {
        assert_eq!(num_days_in_month(2020, 2), 29);
    }

    #[test]
    fn test_february_in_non_leap_year() {
        assert_eq!(num_days_in_month(2021, 2), 28);
    }

    #[test]
    fn test_days_in_january() {
        assert_eq!(num_days_in_month(2021, 1), 31);
    }

    #[test]
    fn test_days_in_april() {
        assert_eq!(num_days_in_month(2021, 4), 30);
    }

    #[test]
    #[should_panic(expected = "[num_days_in_month] month [0] is invalid, should be between 1-12")]
    fn test_invalid_month_low() {
        num_days_in_month(2021, 0);
    }

    #[test]
    #[should_panic(expected = "[num_days_in_month] month [13] is invalid, should be between 1-12")]
    fn test_invalid_month_high() {
        num_days_in_month(2021, 13);
    }
}
