mod helpers { 
    // pub mod variables;
    // pub mod tipos_datos;
    pub mod functions;
    // pub mod macro_rust;
    // pub mod hola;
    // pub mod args_println;
 }
// creando modulo 📂 para helpers, extrallendo el archivo a ejecutar
use helpers::functions::main as func;

fn main() {
    func();
}
