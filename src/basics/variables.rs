// Declarar variables en rust

pub fn main(){
    imp_num();
}


fn imp_num() {
    let numero = 64;
    println!("El numero es {numero}");
    // las variables se decalaran con la palabra reservada let ✍
    // numero = 1; ❌
    // Las variables en rust son inmutables por defecto por lo que no se puede modificar su valor
    let mut numero_mutable = 12;
    println!("El numero mutable es {numero_mutable}");
    numero_mutable = 89;
    // para decirle a rust que esa variable va a cambiar de valor agregamos la palabra reservada mut 🔐
    println!("El numero mutable es {numero_mutable}");

    // si no quieres volver mutable una variable porque no la vas a cambiar tanto puedes crear una nueva variable con el mismo nombre que sobreescriba el valor original
    let num_shadow = 12;
    println!("El num_shadow es {num_shadow}");
    let num_shadow = num_shadow + 88;
    println!("El num_shadow es {num_shadow}");
}

