

pub fn main(){
    // existen bastantes tipos de datos en rust
    /*
    1. Numeros enteros
    2. Numeros de coma flotante
    3. Booleanos
    4. Characters o Caracteres
    */

    enteros();
    flotantes();

}


fn enteros() {
    // Los numeros enteros se declaran de la siguiente manera
    let num: i8 = 64;
    // el i32 significa que es un numero entero de 32 bits, y la i significa que puede tener signo, puede ser negativo o positivo
    // exiten varios tipos de numeros con signo o signed 
    /*
    i8 entero con 8 bits con signo  
    i16 entero con 16 bits con signo 
    i32 entero con 32 bits con signo 
    i64 entero con 64 bits con signo 
    i128 entero con 128 bits con signo  
    */
    // si no le especificas que tipo de numero es asumira que es i32
    let num_32 = 21;

    println!("El numero es {}", num);
    println!("El num_32 es: {}", num_32);
    // tambien existen los numeros sin signo o unsigned 
}

fn flotantes() {
    // los numeros de coma flotante son los numeros con decimales
}