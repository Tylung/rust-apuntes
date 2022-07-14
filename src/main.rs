mod helpers { 
    // pub mod variables;
    pub mod tipos_datos;
    // pub mod macro_rust;
    // pub mod hola;
    // pub mod args_println;
 }
// creando modulo 📂 para helpers, extrallendo el archivo a ejecutar
use helpers::tipos_datos::main as tipos;

fn main() {
    // println!("Hola, Mundo!");
    tipos();
}
