use std::io;

const IMPUESTO: f32 = 0.05;
const COMISION: f32 = 0.1;

/**
 * El valor de un automóvil para el comprador se calcula como:
 * el costo de ese auto más 5% de impuestos, más 10% de ganancia
 * del vendedor. Realice un algoritmo que lo este valor, a partir
 * de un precio ingresado por el usuario.
 */
pub(crate) fn tp1_ej8_d() {
  println!("Ingrese valor del auto");
  let mut precio = String::new();

  io::stdin().read_line(&mut precio)
    .expect("Error al leer precio");

  let precio: f32 = precio.trim().parse()
    .expect("El precio tiene que ser un numero");

  let precio_final = precio * (1.0 + IMPUESTO + COMISION);


  println!("Precio final: {:.2}", precio_final);

}