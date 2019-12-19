pub(crate) const PATTERN: &str =
    r"^(?P<number>\d{1,2}(?:\.)?\d{3}(?:\.)?\d{3})(?:-)?(?P<dv>(?i)K|\d)$";

pub(crate) fn mod_eleven(number: u8) -> u8 {
    11 - (number % 11)
}

pub(crate) fn sum_product(number: u32) -> u8 {
    let reversed = number.to_string().chars().rev().collect::<String>();

    let multiply_start = 2;
    let multiply_ends = 7;
    let mut multiply_number = multiply_start;
    let mut multiply_accumulator = 0;

    for number in reversed.chars() {
        multiply_number = if multiply_number > multiply_ends {
            multiply_start
        } else {
            multiply_number
        };

        let single_number: u8 = number.to_string().parse().unwrap();
        multiply_accumulator += single_number * multiply_number;
        multiply_number += 1
    }

    multiply_accumulator
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
}
