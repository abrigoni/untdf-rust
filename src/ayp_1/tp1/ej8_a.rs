use std::io;

const IMPUESTO: f32 = 0.1;

/**
 * El sueldo de una persona se calcula como la cantidad de horas
 * trabajadas por valor de la hora, menos 10% del sueldo bruto en aportes.
 */
pub(crate) fn tp1_ej8_a() {
  println!("Cargar valor por hora:");
  let mut valor_hora = String::new();
  
  io::stdin().read_line(&mut valor_hora)
    .expect("Error al leer valor por hora");

  let valor_hora: f32 = valor_hora.trim().parse()
    .expect("Ingresar un numero para valor por hora");

  let mut cantidad_horas = String::new();

  println!("Cargar cantidad de horas:");
  io::stdin().read_line(&mut cantidad_horas)
    .expect("Error al leer cantidad de horas");

  let cantidad_horas: u32 = cantidad_horas.trim().parse()
    .expect("Ingresar un numero para la cantidad de horas");

  let sueldo: f32  = cantidad_horas as f32 * valor_hora * (1.0 - IMPUESTO);

  let sueldo = format!("{:.2}", sueldo);

  println!("El sueldo es: {}", sueldo);
}