// Ownership es una feature unica de Rust
// y tiene profundas implicaciones en el lenguaje

// Que es la ownership?
/*
Ownership es un grupo de reglas que
dictan el manejo de la memoria en Rust.
todos los programas deben de tener una
forma de manejar la memoria
*/

// Reglas del ownership
// 1. Cada valor en Rust tiene un dueño u owner.
// 2. solo puede haber un dueño a la vez.
// 3. Cuando el dueño sale del scope, el valor es eliminado o dropeado.
pub fn main() {
    let s: String = String::from("hello"); // s comes into scope

    takes_ownership(s); // El valor de s es tomado por la funcion take_ownership
                        // ... y el valor s no es mas valido en este scope
                        // println!("La variable es {}", s) -> Error

    let x: i32 = 5; // x llega al scope

    makes_copy(x);
    // aqui x sale del scope pero como una copia, a diferencia de s.
    // pero nada especial pasa x sigue siendo valido
    println!("El numero es {}", x);
}

fn takes_ownership(some_string: String) {
    // some_string entra al scope
    println!("El string es: {}", some_string);
} // Aquí some_string sale del scope y la funcion drop es llamada
  // Y la memoria es liberada

fn makes_copy(some_integer: i32) {
    // some_integer entra al scope
    println!("La copia del numero es: {}", some_integer);
} // Sale some_integer del scope, nada especial pasa
