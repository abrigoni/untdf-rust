use std::io;


/**
 * Escribir un programa que lea N (dato a leer) números enteros, e imprima:
 * a) la suma y el producto de los mismos.
 * b) El máximo y el mínimo valor leído.
 * c) La cantidad de los negativos pares. (NOTA: Puede no existir).
 */
pub(crate) fn ej5() {
  println!("Ingresar N:");
  let mut n = String::new();

  io::stdin().read_line(&mut n)
    .expect("Error al leer N");

  let n: u8 = n.trim().parse()
    .expect("n Debe ser un numero");

  let mut suma = 0;
  let mut prod = 1;
  let mut max = i32::MIN;
  let mut min = i32::MAX - 1;
  let mut cantidad_negativos_pares = 0;
  for i in 0..n {
    println!("Ingrese numero {}/{}:", i+1, n);
    let mut num = String::new();

    io::stdin().read_line(&mut num)
      .expect("Error al leer numero");

    let num: i32 = num.trim().parse()
      .expect("Num tiene que ser un numero");
    
    suma += num;
    prod *= num;

    if num < 0 && num % 2 == 0 {
      cantidad_negativos_pares += 1;
    } 
    
    if num >= max {
      max = num;
    }

    if (num <= min) {
      min = num;
    }

    
  }

  println!("---------Respuestas---------");
  println!("Suma: {}", suma);
  println!("Producto: {}", prod);
  println!("Max: {}", max);
  println!("Min: {}", min);
  println!("Cantidad de numeros negativos pares: {}", cantidad_negativos_pares);

}