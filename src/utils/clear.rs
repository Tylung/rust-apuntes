

pub fn clear_scr() {
    print!("{esc}[2J {esc}[1;1H", esc = 27 as char);
    // funciona en linux y windows!
    // limpia la pantalla y coloca el cursor en la primera columna y primera fila
}