use clap::ArgMatches;
use owo_colors::OwoColorize;
use std::cmp::Ordering;
use std::io::Write;
use std::{io, time::Duration};

use crate::utils::clear::clear_scr;
use crate::utils::random::get_random_num;

pub fn init_game(opts: ArgMatches) {
    if opts.get_flag("clear") {
        clear_scr();
    }

    if opts.contains_id("range") && !opts.get_flag("infinity") {
        let range_raw: Vec<_> = opts
            .get_many::<String>("range")
            .expect("Error get range")
            .collect();
        let range = range_raw
            .into_iter()
            .map(|n| n.parse().expect("Error converting to u8"))
            .collect();

        // let (secret_num, init, limit) = get_random_num(range).expect("Test err");

        let (secret_num, init, limit) = match get_random_num(range) {
            Ok(result) => result,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        game(secret_num, init, limit);
    }

    if opts.get_flag("infinity") && opts.contains_id("range") {
        let range_raw: Vec<_> = opts
            .get_many::<String>("range")
            .expect("Error get range")
            .collect();
        let range = range_raw
            .into_iter()
            .map(|n| n.parse().expect("Error converting to u8"))
            .collect();

        let (_secret_num, init, limit) = match get_random_num(range) {
            Ok(result) => result,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        infinite_game(init, limit);
    }

    if opts.get_flag("infinity") && !opts.contains_id("range") {
        infinite_game(1, 10);
    }

    if !opts.get_flag("infinity") && !opts.contains_id("range") {
        let (secret_num, init, limit) = get_random_num(vec![1, 10]).expect("Error Empty Array");

        game(secret_num, init, limit);
    }
}

pub fn game(num: u32, init: u32, limit: u32) -> bool {
    let mut num_secreto: u32 = num;

    if num == 0 {
        let res = get_random_num(vec![init, limit]).expect("Error in get_random_num");
        num_secreto = res.0
    }

    println!("{}", "Adivina el numerust!".bright_blue());

    let mut is_winner: bool = false;
    // generamos un numero al azar de 2 numeros
    println!("Del {init} al {limit}");
    print!("{}", "Ingresa el numero: ".bright_purple());
    io::stdout().flush().expect("Error al flush stdout");

    let mut num_elegido = String::new();
    // la variable num_elegido es una nueva instancia del struct String, un nuevo string

    io::stdin()
        .read_line(&mut num_elegido)
        // leemos la linea del prompt
        .expect("Fallo al leer el prompt");

    let num_elegido: u32 = match num_elegido.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "No ingresaste un numero valido".red());
            return is_winner;
        }
    };
    // shadow variable para convertirlo en numero y poder compararlo con el numero secreto
    // y utilizamos la expresion match para que cuando no reciba un numero simpremente continue el programa

    println!(
        "{} {}",
        "El numero que elegiste es".dimmed(),
        num_elegido.yellow()
    );

    match num_elegido.cmp(&num_secreto) {
        Ordering::Less => println!(
            "{}, el numero era {}",
            "Muy bajo".red(),
            num_secreto.bright_red()
        ),
        Ordering::Greater => println!(
            "{}, el numero era {}",
            "Muy alto".red(),
            num_secreto.bright_red()
        ),
        Ordering::Equal => {
            println!(
                "{} el numero era {}",
                "Has Acertado :D".green(),
                num_secreto.bright_green()
            );
            is_winner = true
        }
    }

    is_winner
}

pub fn infinite_game(init_param: u32, limit_param: u32) {
    loop {
        let mut resp = String::new();
        clear_scr();

        let (num, init, limit) = get_random_num(vec![init_param, limit_param]).expect("EmptyError");

        let is_winner = game(num, init, limit);

        if is_winner {
            return;
        }

        print!(
            "{}{}",
            "\nDesea jugar otravez?".bright_green(),
            "(s/N) ".cyan().dimmed()
        );
        io::stdout().flush().expect("Error al flush stdout");

        io::stdin()
            .read_line(&mut resp)
            .expect("No es una entrada valida");

        resp = resp.trim().to_ascii_lowercase();

        if resp.chars().count() == 1 && resp.contains("s") {
            println!("{}", "\nCreando nuevo juego...".bright_cyan());
            io::stdout().flush().expect("Error en flush stdout");
            std::thread::sleep(Duration::from_millis(200));
        } else {
            break;
        }
    }
}
