use rut_lib::Rut;

fn main() {
    let stringifier_rut = "17951585-7";

    match Rut::from(stringifier_rut) {
        Ok(rut) => {
            println!("Number: {:#}", rut.number());
            println!("DV: {:#}", rut.dv());
            println!("RUT: {:#}", rut)
        }
        Err(error) => println!("Error: {:#}", error),
    }

    println!("Random RUT: {:#}", Rut::randomize())
}
