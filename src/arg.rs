use clap::{Arg, ArgMatches, Command};

pub fn get_opts() -> ArgMatches {
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
        .get_matches();

    return opts;
}
