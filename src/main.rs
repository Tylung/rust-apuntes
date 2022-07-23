use std::io;
use std::io::Write;
use owo_colors::OwoColorize;
use rand::Rng;
use std::cmp::Ordering;
use clap::{Command, Arg, ArgMatches};

fn main() {
    let opts = obt_opciones();
    
    if opts.contains_id("clear") {
        clear_scr();
    }

    println!("{}", "Adivina el numerust!".bright_blue());

    let num_secreto: u8 = rand::thread_rng().gen_range(1..=10);
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
    
    let num_elegido: u8 = num_elegido.trim().parse().expect("Por favor ingresa un numero!");
    // shadow variable para convertirlo en numero y poder compararlo con el numero secreto

    println!("{} {}", "El numero que elegiste es".dimmed() , num_elegido.yellow() );

    match num_elegido.cmp( &num_secreto) {
        Ordering::Less => println!("{}, el numero era {}", "Muy bajo".red(), num_secreto.bright_red()),
        Ordering::Greater => println!("{}, el numero era {}", "Muy alto".red(), num_secreto.bright_red()),
        Ordering::Equal => println!("{} el numero era {}", "Has Acertado :D".green(), num_secreto.bright_green())
    }

}

fn clear_scr() {
    print!("{esc}[2J {esc}[1;1H", esc = 27 as char);
    // funciona en linux y windows! 
    // limpia la pantalla y coloca el cursor en la primera columna y primera fila
}


fn obt_opciones() -> ArgMatches {
    let opts = Command::new("guessing-game")
    // creamos las optiones de la app
        .version("0.0.1")
        .author("Zack @Tylung in Github")
        .about("Este es un juego de terminal hecho en rust! 🦀")
        .arg(
            Arg::new("clear")
                .short('c')
                .long("clear")
                .help("Limpia la terminal antes de empezar el juego")
        )
        // argumento para limpiar la terminal
        .get_matches();
    
    return opts;
}