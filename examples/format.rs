use rut_lib::{Format, Rut};

fn main() {
    let input = "179515857";
    let rut = Rut::from(input).unwrap();

    println!("Dots: {}", rut.to_format(Format::DOTS));
    println!("Dash: {}", rut.to_format(Format::DASH));
    println!("None: {}", rut.to_format(Format::NONE))
}
