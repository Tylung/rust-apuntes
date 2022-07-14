macro_rules! di_hola {
    // `()` indica que el macro no acepta argumentos.
    () => {
        // El macro se expandirá al contenido de este bloque.
        let number: u8 = 255;
        println!("Hola numero {}!", number);
    };
}

pub fn main() {
    // Esta llamada se expandirá a `println!("¡Hola!");`
    di_hola!();
}