use core::fmt;
use num_format::{Locale, ToFormattedString};
use rand::prelude::*;

pub enum Range {
    MIN = 1_000_000,
    MAX = 99_999_999,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_u32().to_formatted_string(&Locale::es_CL))
    }
}

impl Range {
    pub fn to_u32(&self) -> u32 {
        match self {
            Range::MIN => Range::MIN as u32,
            Range::MAX => Range::MAX as u32,
        }
    }
}

fn random_range(min: u32, max: u32) -> u32 {
    let mut rand = rand::thread_rng();
    rand.gen_range(min, max)
}

#[inline(always)]
pub(crate) fn random_number() -> u32 {
    random_range(Range::MIN as u32, Range::MAX as u32)
}

#[cfg(test)]
mod range_test {
    use crate::range::*;

    #[test]
    fn format() {
        assert_eq!(Range::MIN.to_string(), "1.000.000");
        assert_eq!(Range::MAX.to_string(), "99.999.999")
    }
}
