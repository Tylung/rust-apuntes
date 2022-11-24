fn otra_funcion() {
    println!("Esta es otra función diferente a la main");
}

fn suma(a: u8, b: u8) -> u8 {
    // tipo de retorno
    a + b
    // el return implicito siempre es la ultima linea sin ;
}

fn resta(a: u8, b: u8) -> u8 {
    println!("función resta");
    let num1 = a;
    let num2: u8 = b;

    // return num1 + num2;
    // o usar la palabra return si se quiere poner el ;
    num1 + num2 // la convencion es retornar así
}

fn variables_declaration() {
    let y = {
        // asignando valor como una funcion o statement
        let x = 3;
        x + 1
    };

    println!("El valor de y es: {y}");
}

pub fn main() {
    println!("Esta es la función main");
    otra_funcion();
    println!("El resultado de la suma de 2 y 3 es: {}", suma(2, 3));
    println!("El resultado de la resta de 2 y 3 es: {}", resta(2, 3));

    variables_declaration();
}
