use clap::{Arg, ArgMatches, Command};
use owo_colors::OwoColorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;
use std::{io, time::Duration};

fn main() {
    let opts = obt_opciones();

    init_game(opts);
}

fn obt_opciones() -> ArgMatches {
    let opts = Command::new("guessing-game")
        // creamos las optiones de la app
        .version("0.1.0")
        .author("Zack @Tylung in Github")
        .about("Este es un juego de terminal hecho en rust! 🦀")
        .arg(
            Arg::new("clear")
                .short('c')
                .long("clear")
                .required(false)
                .num_args(0)
                .help("Limpia la terminal antes de empezar el juego"),
        )
        .arg(
            Arg::new("infinity")
                .short('i')
                .long("infinity")
                .required(false)
                .num_args(0)
                .help("Jugar de manera infinita hasta acertar el numero"),
        )
        .arg(
            Arg::new("range")
                .short('r')
                .long("range")
                .required(false)
                .num_args(2)
                .value_names(["INIT", "LIMIT"])
                .help("Provides a custom range for secret number"),
        )
        // argumento para limpiar la terminal
        .get_matches();

    return opts;
}

fn init_game(opts: ArgMatches) {

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

        let (secret_num, init, limit) = get_random_num(range);

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

        let (_secret_num, init, limit) = get_random_num(range);

		
        infinite_game(init, limit);

    }

    if opts.get_flag("infinity") && !opts.contains_id("range") {
        infinite_game(1, 10);
    }

	if !opts.get_flag("infinity") && !opts.contains_id("range") {
		let (secret_num, init, limit) = get_random_num(vec![1, 10]);

		game(secret_num, init, limit);
	}
	
}

fn clear_scr() {
    print!("{esc}[2J {esc}[1;1H", esc = 27 as char);
    // funciona en linux y windows!
    // limpia la pantalla y coloca el cursor en la primera columna y primera fila
}

fn game(num: u8, init: u8, limit: u8) -> bool {
    let mut num_secreto: u8 = num;

    if num == 0 {
        let res = get_random_num(vec![init, limit]);
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

    let num_elegido: u8 = match num_elegido.trim().parse() {
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

fn infinite_game(init_param: u8, limit_param: u8) {
    loop {
        let mut resp = String::new();
        clear_scr();

        let (num, init, limit) = get_random_num(vec![init_param, limit_param]);

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

fn get_random_num(nums: Vec<u8>) -> (u8, u8, u8) {
    let limit = nums[1];
    let init = nums[0];

    let num = rand::thread_rng().gen_range(init..=limit);

    (num, init, limit)
}
