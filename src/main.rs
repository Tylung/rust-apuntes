mod ownership {
    pub mod slice;
}

// creando modulo 📂 para basics, extrallendo el archivo a ejecutar
use ownership::slice::main as slice;

fn main() {
    slice();
}
