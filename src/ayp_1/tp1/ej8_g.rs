use std::{io, process};

// Leer un número entero de hasta 4 dígitos,
// imprimirlo y mostrar en pantalla:
// ● Cantidad de unidades.
// ● Cantidad de decenas.
// ● Cantidad de centenas.
// ● Cantidad de miles.
pub(crate) fn tp1_ej8_g() {
  println!("Ingresar numero");
  let mut num = String::new();

  io::stdin().read_line(&mut num)
    .expect("Error al leer el numero");

  let num = num.trim();

  if num.len() > 4 {
    eprintln!("La longitud debe ser hasta 4");
    process::exit(1);
  }

  let num: u32 = num.parse()
    .expect("La variable debe ser un numero");

  println!("Número ingresado: {}", num);

  let miles = num / 1000 % 10;
  let centenas = num / 100 % 10;
  let decenas = num / 10 % 10;
  let unidades = num % 10;

  println!("Cantidad de miles: {}", miles);
  println!("Cantidad de centenas: {}", centenas);
  println!("Cantidad de decenas: {}", decenas);
  println!("Cantidad de unidades: {}", unidades);
}