pub(crate) const PATTERN: &str =
    r"^(?P<number>\d{1,2}(?:\.)?\d{3}(?:\.)?\d{3})(?:-)?(?P<dv>(?i)K|\d)$";

#[derive(Clone, Copy)]
enum Range {
    Init = 2,
    Limit = 7,
}

pub(crate) fn mod_eleven(number: u8) -> u8 {
    11 - (number % 11)
}

fn reversed_numbers(number: u32) -> Vec<u8> {
    number
        .to_string()
        .chars()
        .rev()
        .map(|char| char.to_digit(10))
        .map(Option::unwrap)
        .map(|num| num as u8)
        .collect()
}

fn limit_range(number: u8) -> u8 {
    if number > (Range::Limit as u8) {
        Range::Init as u8
    } else {
        number
    }
}

pub(crate) fn sum_product(number: u32) -> u8 {
    let mut total = 0;
    let mut range = Range::Init as u8;
    let numbers = reversed_numbers(number);

    for number in numbers {
        total += number * range;
        range = limit_range(range + 1);
    }

    total
}

#[cfg(test)]
mod utils_test {
    use crate::utils::*;

    #[test]
    fn test_sum_product() {
        assert_eq!(sum_product(17951585), 169)
    }

    #[test]
    fn test_mod_eleven() {
        assert_eq!(mod_eleven(169), 7)
    }

    #[test]
    fn test_reversed_numbers() {
        assert_eq!(reversed_numbers(123), vec![3, 2, 1])
    }

    #[test]
    fn test_limit_range() {
        assert_eq!(limit_range(8), 2);
        assert_eq!(limit_range(2), 2);
    }
}
