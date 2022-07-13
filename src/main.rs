mod helpers { 
    // pub mod variables;
    pub mod tipos_datos;
    // pub mod hola;
    // pub mod args_println;
 }
// creando modulo 📂 para helpers, extrallendo el archivo a ejecutar
use helpers::tipos_datos::main as tipos;

fn main() {
    // println!("Hola, Mundo!");
    tipos();
}
