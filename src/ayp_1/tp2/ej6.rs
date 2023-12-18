use std::io;


/**
 * 6) Escribir un programa que lea un valor entero N, y luego N números. Los números se
 * van imprimiendo a medida que se leen. Por final se debe imprimir un mensaje diciendo
 * si la suma total de los N números es par o impar. Se pide realizar el ejercicio sin sumar
 * los números (Indicación: simule valores lógicos).
 */
pub(crate) fn ej6() {
  println!("Ingresar N:");
  let mut n = String::new();

  io::stdin().read_line(&mut n)
    .expect("Error al leer N");

  let n: u8 = n.trim().parse()
    .expect("n Debe ser un numero");

  let mut impar = false;
  for i in 0..n {
    println!("Ingrese numero {}/{}:", i+1, n);
    let mut num = String::new();

    io::stdin().read_line(&mut num)
      .expect("Error al leer numero");

    let num: i32 = num.trim().parse()
      .expect("Num tiene que ser un numero");

    if num % 2 != 0 {
      impar = !impar;
    }
  }

  println!("---------Respuesta---------");
  println!("Impar: {}", impar);

}