use std::io;
use std::io::Write;

fn main() {
    println!("Adivina el numerust!");

    print!("Ingresa el numero: ", );
    io::stdout().flush().unwrap();

    let mut num_elegido = String::new();

    io::stdin()
        .read_line(&mut num_elegido)
        .expect("Fallo al leer el prompt");
    //     let mut guess = String::new();

    // declaramos la variable par ael numero como una nueva instancia del numero
    println!("El numero que elegiste es {}", num_elegido);


}
