mod basics {
    // pub mod args_println;
    // pub mod functions;
    // pub mod hola;
    // pub mod macro_rust;
    pub mod ownership;
    pub mod borrowing;
    // pub mod tipos_datos;
    // pub mod variables;
}
// creando modulo 📂 para basics, extrallendo el archivo a ejecutar
use basics::ownership::main as ownership;
use basics::borrowing::main as borrowing;

fn main() {
    ownership();
    borrowing();
}
