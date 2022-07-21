use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if &args.len() > &1 {
        if &args[1] == "c" {
            clear_scr();
        }
    }
    println!("Adivina el numerust!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    // println!("\nEl numero secreto es {secret_number}\n");
    // generamos un numero al azar del 1 al 10
    println!("Del 1 al 10");
    print!("Ingresa el numero: ", );
    io::stdout().flush().unwrap();
    // evitamos que se vaya a otra linea el cursor

    let mut num_elegido = String::new();
    // la variable num_elegido es una nueva instancia del struct String, un nuevo string

    io::stdin()
        .read_line(&mut num_elegido)
        // leemos la linea del prompt
        .expect("Fallo al leer el prompt");
    
    let num_elegido: u32 = num_elegido.trim().parse().expect("Please type a number!");
    // shadow variable para convertirlo en numero y poder compararlo con el numero secreto

    println!("El numero que elegiste es {num_elegido}");

    match num_elegido.cmp( &secret_number) {
        Ordering::Less => println!("Muy bajo, el numero era {secret_number}"),
        Ordering::Greater => println!("Muy alto, el numero era {secret_number}"),
        Ordering::Equal => println!("Has Acertado :D el numero era {secret_number}")
    }

}

fn clear_scr(){
    if cfg!(windows) {
        std::process::Command::new("cls").status().unwrap();
    } else if cfg!(unix) {
        std::process::Command::new("clear").status().unwrap();
    }
}

