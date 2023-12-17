use std::io;

const SALARIO_BASE: f32 = 750.0;
const COMISION: f32 = 0.2;
const BONO: f32 = 10.0;

/**
 * Una agencia de autos paga a sus vendedores un salario de
 * $ 750 más una comisión del 20% sobre el precio de cada
 * vehículo vendido por esa persona, más $ 10 por cada vehículo 
 * vendido. Suponga que la agencia vende un único tipo de vehículo.
 * Calcular cuánto gana el vendedor en ese mes.
 * a) Qué datos necesita conocer para calcular ese sueldo.
 * b) Realice un algoritmo que lo calcule.
 */
pub(crate) fn tp1_ej8_f() {
  println!("Ingresar precio del vehiculo");
  let mut precio_vehiculo = String::new();

  io::stdin().read_line(&mut precio_vehiculo)
    .expect("Error al leer el precio del vehiculo");

  let precio_vehiculo: f32 = precio_vehiculo.trim().parse()
    .expect("El precio del vehiculo tiene que ser un numero");

  println!("Ingresar cantidad de vehiculos vendidos");
  let mut cantidad_vehiculos = String::new();

  io::stdin().read_line(&mut cantidad_vehiculos)
    .expect("Error al leer cantidad de vehiculos");

  let cantidad_vehiculos: u16 = cantidad_vehiculos.trim().parse()
    .expect("Cantidad de vehiculos tiene que ser un numero");

  let comisiones = COMISION * precio_vehiculo * cantidad_vehiculos as f32;
  let bonos = BONO * cantidad_vehiculos as f32;
  let salario = SALARIO_BASE + comisiones + bonos;

  println!("Salario: {}", salario);
}