use std::io;


const PORCENTAJE_APORTE_JUBILATORIO: f32 = 0.1;
const PORCENTAJE_APORTE_SOCIAL: f32 = 0.03;
const BONO_EXTRA: f32 = 0.8;

// Conocidos:
// ● El valor de la hora normal
// ● La cantidad de horas trabajadas en la quincena.
// Calcular el sueldo de un empleado, sabiendo que las
// primeras 85 horas son normales y las extras se pagan
// un 80% más de la hora normal. Además del sueldo completo
// se debe descontar un 10% para aporte jubilatorio y un 3%
// para la Obra Social. Los descuentos se efectúan sobre horas
// normales únicamente.
// NOTA: Se supone que el empleado trabaja como mínimo 85
// horas.
pub(crate) fn tp1_ej8_h() {
  println!("Ingresar valor hora normal:");
  let mut valor_hora_normal = String::new();

  io::stdin().read_line(&mut valor_hora_normal)
    .expect("Error al leer valor de hora normal");

  let valor_hora_normal: f32 = valor_hora_normal.trim().parse()
    .expect("Valor de hora normal debe ser un numero");

  let valor_hora_extra = valor_hora_normal + valor_hora_normal * BONO_EXTRA;
    
  println!("Ingresar cantidad de horas");
  let mut cantidad_horas = String::new();

  io::stdin().read_line(&mut cantidad_horas)
    .expect("Error al leer cantidad de horas");

  let cantidad_horas: f32 = cantidad_horas.trim().parse()
    .expect("Cantidad de horas debe ser un numero");

  let horas_extra = cantidad_horas - 85.0;

  let salario_base_bruto = valor_hora_normal * 85.0;

  let salario_extra = horas_extra * valor_hora_extra;

  let aporte_jubilatorio = salario_base_bruto * PORCENTAJE_APORTE_JUBILATORIO;

  let aporte_social = salario_base_bruto * PORCENTAJE_APORTE_SOCIAL;

  let salario =  salario_base_bruto - aporte_jubilatorio - aporte_social + salario_extra;

  println!("Salario base bruto = ${}", salario_base_bruto);
  println!("Aporte jubilatorio = ${}", aporte_jubilatorio);
  println!("Aporte social = ${}", aporte_social);
  println!("Extra = ${}", salario_extra);
  println!("Salario neto= ${}", salario);

}