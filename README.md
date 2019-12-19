# Rut Library 🇨🇱
[![Build Status](https://travis-ci.org/erikjara/rut-lib.svg?branch=master)](https://travis-ci.org/erikjara/rut-lib.svg)
[![codecov](https://codecov.io/gh/erikjara/rut-lib/branch/master/graph/badge.svg)](https://codecov.io/gh/erikjara/rut-lib)
[![Crates.io](https://img.shields.io/crates/v/rut-lib.svg)](https://crates.io/crates/rut-lib)
[![Documentation](https://docs.rs/rut-lib/badge.svg)](https://docs.rs/rut-lib)

A Rust library for parsing, format and validate a Chilean ID (RUT)

# Usage
This crate is [on crates.io](https://crates.io/crates/rut-lib) and can be used by adding `rut-lib` to your dependencies in your project's Cargo.toml.

```toml
[dependencies]
rut-lib = "0.1.2"
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

## License
This project is licensed under the terms of the [MIT License](LICENSE).