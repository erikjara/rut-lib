use rut_lib::Rut;

fn main() {
    let number = 24136773;

    match Rut::from_number(number) {
        Ok(rut) => {
            println!("Number: {:#}", rut.number());
            println!("DV: {:#}", rut.dv());
            println!("RUT: {:#}", rut)
        }
        Err(error) => println!("Error: {:#}", error),
    }
}
