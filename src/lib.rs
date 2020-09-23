/*!
A Rust library for parsing, format and validate a Chilean ID (RUT)

# Usage
This crate is [on crates.io](https://crates.io/crates/rut-lib) and can be used by adding `rut_lib` to your dependencies in your project's Cargo.toml.

```toml
[dependencies]
rut-lib = "1.0.0"
```

If you're using Rust 2015, then you'll also need to add it to your crate root:

```rust
extern crate rut_lib;
```

# Examples

## Parsing from String
A easy way to validate and create a `Rut` using the `from` method, this returns a `Result<Rut, Error>`

The input must be a valid `String` format, a few examples:
- `17.951.585-7`
- `17951585-7`
- `179515857`

```rust
use rut_lib::Rut;

let stringifier_rut = "17951585-7";

match Rut::from(stringifier_rut) {
    Ok(rut) => {
        println!("Number: {:#}", rut.number());
        println!("DV: {:#}", rut.dv());
        println!("RUT: {:#}", rut)
    },
    Err(error) => println!("Error: {:#}", error)
}
```
Output

```bash
Number: 17951585
DV: 7
RUT: 17951585-7
```

#### Error behaviour
<details><summary>Error::InvalidFormat</summary>
<p>

```rust
use rut_lib::Rut;

let stringifier_rut = "17,951,585-7";

match Rut::from(stringifier_rut) {
    Ok(rut) => println!("RUT: {:#}", rut),
    Err(error) => println!("Error: {:#}", error)
}
```

Output

```bash
Error: The input format is invalid
```
</p>
</details>

<details><summary>Error::InvalidDV</summary>
<p>

```rust
use rut_lib::Rut;

let stringifier_rut = "17951585K";

match Rut::from(stringifier_rut) {
    Ok(rut) => println!("RUT: {:#}", rut),
    Err(error) => println!("Error: {:#}", error)
}
```

Output

```bash
Error: Invalid DV, must be 7, instead K.
```
</p>
</details>

## Parsing from Number
Create a `Rut` using the `from_number` method (If you don't have a `DV`), this returns a `Result<Rut, Error>`

The input must be a `number` (`u32`) and stay in a range from `1_000_000` to `99_999_999`

```rust
use rut_lib::Rut;

let number = 24136773;

match Rut::from_number(number) {
    Ok(rut) => {
        println!("Number: {:#}", rut.number());
        println!("DV: {:#}", rut.dv());
        println!("RUT: {:#}", rut)
    },
    Err(error) => println!("Error: {:#}", error)
}
```

Output

```bash
Number: 24136773
DV: 8
RUT: 24136773-8
```

#### Error behaviour
<details><summary>Error::OutOfRange</summary>
<p>

```rust
use rut_lib::Rut;

let number = 999_999;

match Rut::from_number(number) {
    Ok(rut) => {
        println!("RUT: {:#}", rut)
    },
    Err(error) => println!("Error: {:#}", error)
}
```

Output

```bash
Error: The input number must be between 1.000.000 to 99.999.999
```
</p>
</details>

## Randomize Rut
Generate a randomize rut from scratch for testing use

Example:
```rust
use rut_lib::Rut;

let rut = Rut::randomize();

println!("Number: {:#}", rut.number());
println!("DV: {:#}", rut.dv());
println!("RUT: {:#}", rut);
```

Output

```bash
Number: 56606059
DV: 0
RUT: 56606059-0
```

## Prettify Format
The `to_format` method receive a `Format` (`enum`) as input to returns a Prettify `Rut`
```rust
use rut_lib::{Rut, Format};

let input = "179515857";
let rut = Rut::from(input).unwrap();

println!("Dots: {}", rut.to_format(Format::DOTS));
println!("Dash: {}", rut.to_format(Format::DASH));
println!("None: {}", rut.to_format(Format::NONE));
```

Output

```bash
Dots: 17.951.585-7
Dash: 17951585-7
None: 179515857
```
*/

mod error;
mod range;
mod utils;

use core::fmt;
use error::Error;
use num_format::{Locale, ToFormattedString};
use range::{random_number, Range};
use regex::Regex;
use utils::{mod_eleven, sum_product, PATTERN};

#[derive(Debug)]
pub struct Rut {
    number: u32,
    dv: char,
}

#[derive(Copy, Clone)]
pub enum Format {
    DOTS,
    DASH,
    NONE,
}

impl fmt::Display for Rut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_format(Format::DASH))
    }
}

impl Rut {
    /// Create a `Rut` from a input String
    /// This is useful when you want to parse to String
    /// or check if is a valid input Rut.
    ///
    /// The Input must be a valid RUT format.
    pub fn from(input: &str) -> Result<Rut, Error> {
        match Rut::extract_from(input) {
            Ok(unverified_rut) => Rut::check_dv(unverified_rut),
            Err(error) => Err(error),
        }
    }

    /// Create a `Rut` from a input Number
    ///
    /// The input must be between in a Range value.
    pub fn from_number(number: u32) -> Result<Rut, Error> {
        let min = Range::MIN.to_u32();
        let max = Range::MAX.to_u32();
        let range = min..max;
        if range.contains(&number) {
            let dv = Rut::generate_dv(number);
            Ok(Rut { number, dv })
        } else {
            Err(Error::OutOfRange)
        }
    }

    /// Generate a Rut from scratch with a random number.
    pub fn randomize() -> Rut {
        let number = random_number();
        let dv = Rut::generate_dv(number);

        Rut { number, dv }
    }

    /// Take a `Rut` and Prettify the output to String
    /// This use the `Format` enum as input.
    pub fn to_format(&self, format: Format) -> String {
        match format {
            Format::DOTS => format!(
                "{}-{}",
                self.number.to_formatted_string(&Locale::es_CL),
                self.dv
            ),
            Format::DASH => format!("{}-{}", self.number, self.dv),
            Format::NONE => format!("{}{}", self.number, self.dv),
        }
    }

    /// Return the number output
    pub fn number(&self) -> &u32 {
        &self.number
    }

    /// Return the DV output
    pub fn dv(&self) -> &char {
        &self.dv
    }

    fn check_dv(unsigned_rut: Rut) -> Result<Rut, Error> {
        let signed_rut = Rut::from_number(unsigned_rut.number).unwrap();
        if unsigned_rut.dv != signed_rut.dv {
            Err(Error::InvalidDV {
                must_be: signed_rut.dv,
                instead: unsigned_rut.dv,
            })
        } else {
            Ok(signed_rut)
        }
    }

    fn extract_from(input: &str) -> Result<Rut, Error> {
        let regex = Regex::new(PATTERN).unwrap();
        if regex.is_match(input) {
            let captures = regex.captures(input).unwrap();
            let number: u32 = captures["number"].replace(".", "").parse().unwrap();
            let dv = captures["dv"].to_uppercase().chars().next().unwrap();
            Ok(Rut { number, dv })
        } else {
            Err(Error::InvalidFormat)
        }
    }

    fn generate_dv(number: u32) -> char {
        let total_number = sum_product(number);
        let dv = mod_eleven(total_number);

        match dv {
            10 => 'K',
            11 => '0',
            _ => format!("{}", dv).chars().next().unwrap(),
        }
    }
}

#[cfg(test)]
mod rut_test {
    use super::*;

    #[test]
    fn from() {
        let valid_rut = ["17951585-7", "5.665.328-7", "241367738"];
        for rut in valid_rut.iter() {
            assert!(Rut::from(rut).is_ok())
        }
    }

    #[test]
    fn from_number() {
        let valid_rut = [(17951585, '7'), (12621806, '0'), (24136773, '8')];

        for (number, dv) in valid_rut.iter() {
            let rut = Rut::from_number(*number).unwrap();
            assert_eq!(&rut.number, number);
            assert_eq!(rut.dv(), dv);
        }
    }

    #[test]
    fn to_format() {
        let rut_tuple = [
            (
                "17951585-7",
                Rut::from_number(17951585).unwrap(),
                Format::DASH,
            ),
            (
                "5.665.328-7",
                Rut::from_number(5665328).unwrap(),
                Format::DOTS,
            ),
            (
                "241367738",
                Rut::from_number(24136773).unwrap(),
                Format::NONE,
            ),
        ];

        for (input, rut, format) in rut_tuple.iter() {
            assert_eq!(rut.to_format(*format), *input)
        }
    }

    #[test]
    fn randomize() {
        let rut = Rut::randomize();
        assert_eq!(rut.to_string(), rut.to_string())
    }

    #[test]
    fn wrong_dv() {
        assert_eq!(
            Rut::from("17951585-K").unwrap_err().to_string(),
            Error::InvalidDV {
                must_be: '7',
                instead: 'K'
            }
            .to_string()
        )
    }

    #[test]
    fn invalid_format() {
        assert_eq!(
            Rut::from("17.951,585-7").unwrap_err().to_string(),
            Error::InvalidFormat.to_string()
        )
    }

    #[test]
    fn out_of_range() {
        assert_eq!(
            Rut::from_number(999999).unwrap_err().to_string(),
            Error::OutOfRange.to_string()
        );
        assert_eq!(
            Rut::from_number(100000000).unwrap_err().to_string(),
            Error::OutOfRange.to_string()
        );
    }
}
