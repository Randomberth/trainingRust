use std::io::{stdout, Write};
use termion::cursor;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "Ingrese su nombre: ").unwrap();
    stdout.flush().unwrap();

    // Coloca el cursor en la misma línea, después del prompt
    write!(stdout, "{}", cursor::Goto(18, 1)).unwrap();

    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Fallo de lectura");

    writeln!(stdout, "\nHola, {}!", name.trim());
}
