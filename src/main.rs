use std::io;
use std::io::Write;
use owo_colors::OwoColorize;
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
    println!("{}", "Adivina el numerust!".bright_blue());

    let secret_number = rand::thread_rng().gen_range(1..=10);
    // println!("\nEl numero secreto es {secret_number}\n");
    // generamos un numero al azar del 1 al 10
    println!("{}", "Del 1 al 10".bright_cyan());
    print!("{}","Ingresa el numero: ".bright_purple() );
    io::stdout().flush().unwrap();
    // evitamos que se vaya a otra linea el cursor

    let mut num_elegido = String::new();
    // la variable num_elegido es una nueva instancia del struct String, un nuevo string

    io::stdin()
        .read_line(&mut num_elegido)
        // leemos la linea del prompt
        .expect("Fallo al leer el prompt" );
    
    let num_elegido: u32 = num_elegido.trim().parse().expect("Please type a number!");
    // shadow variable para convertirlo en numero y poder compararlo con el numero secreto

    println!("{} {}", "El numero que elegiste es".blue().dimmed() , num_elegido.yellow() );

    match num_elegido.cmp( &secret_number) {
        Ordering::Less => println!("{}, el numero era {}", "Muy bajo".red(), secret_number.bright_red()),
        Ordering::Greater => println!("{}, el numero era {}", "Muy alto".red(), secret_number.bright_red()),
        Ordering::Equal => println!("{} el numero era {}", "Has Acertado :D".green(), secret_number.bright_green())
    }

}

fn clear_scr() {
    if cfg!(windows) {
        print!("{esc}[2J {esc}[1;1H", esc = 27 as char);
    } else if cfg!(unix) {
        std::process::Command::new("clear").status().unwrap();
    }
}

