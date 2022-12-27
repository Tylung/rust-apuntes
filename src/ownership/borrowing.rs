use std::io::stdin;

pub fn main() {
    let mut nombre = String::new();
    println!("Ingresa tu nombre");
    stdin().read_line(&mut nombre).expect("Fallo al leer stdin");
    let len = nombre.len();
    nombre.truncate(len - 1); // delete \n

    if nombre.is_empty() {
        println!("Nombre no provisto usando pepe");
        nombre = String::from("pepe");
    } 
    take_name(&nombre); 
    // el & significa que se presto o se hizo un borrow
    // nombre regresa a este scope y es valido
    modify_name(&mut nombre);
    println!("El nombre es: {}", nombre)
}

fn take_name(name: &String){
    println!("el nombre que me prestaron es: {name}");
}

fn modify_name(name: &mut String){
    name.push_str("Dev");
    println!("Nombre modificado: {}", name);
}