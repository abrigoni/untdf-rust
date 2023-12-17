use std::io;

const IMPUESTO_1_PAGO: f32 = 0.05;
const IMPUESTO_3_PAGOS: f32 = 0.04;


/**
 * El valor de un televisor es X pesos si se lo paga de contado.
 * Si se paga con tarjeta en un sólo pago, se le incrementa un 5%.
 * Si se lo paga en 3 cuotas se le incrementa un 4% sobre lo que va adeudando.
 * Dado el precio del televisor, dar como respuesta los diferentes precios
 * de acuerdo a cada forma de pago.
 */
pub(crate) fn tp1_ej8_e() {
  println!("Ingresar precio TV");
  
  let mut precio_tv = String::new();

  io::stdin().read_line(&mut precio_tv)
    .expect("Error al leer precio TV");

  let precio_tv: f32 = precio_tv.trim().parse()
    .expect("El precio tiene que ser un numero");

  let precio_1_pago = precio_tv + (precio_tv * IMPUESTO_1_PAGO);

  let mut precio_3_pagos: f32 = 0.0;
  let cuota: f32 = precio_tv / 3.0;

  for _ in 0..3 {
    // interes sobre capital adeudado en el mes
    let interes = (precio_tv - precio_3_pagos) * IMPUESTO_3_PAGOS;
    precio_3_pagos += cuota + interes;
  }

  println!("Precio TV 1 pago: ${}", precio_1_pago);
  println!("Precio TV 3 pagos: ${}", precio_3_pagos);
}