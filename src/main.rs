mod ownership {
  pub mod intro;
  pub mod borrowing;
}

// creando modulo 📂 para basics, extrallendo el archivo a ejecutar
use ownership::intro::main as ownership;
use ownership::borrowing::main as borrowing;

fn main() {
    ownership();
    borrowing();
}
