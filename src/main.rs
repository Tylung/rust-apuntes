mod basics {
    // pub mod variables;
    // pub mod tipos_dato;
    // pub mod functions;
    pub mod ownership;
    // pub mod macro_rust;
    // pub mod hola;
    // pub mod args_println;
}
// creando modulo 📂 para helpers, extrallendo el archivo a ejecutar
use basics::ownership::main as ownership;

fn main() {
    ownership();
}
